use crate::interpreter::ast::*;
use crate::interpreter::error::InterpreterError;
use ply_engine::layout::{LayoutDirection, Padding, Sizing};
use ply_engine::prelude::*;

/// Execute a parsed program against a live Ui handle.
pub fn exec_program(program: &Program, ui: &mut Ui<'_, ()>) -> Result<(), InterpreterError> {
    exec_statements(&program.statements, ui)
}

fn exec_statements(stmts: &[Statement], ui: &mut Ui<'_, ()>) -> Result<(), InterpreterError> {
    for stmt in stmts {
        match stmt {
            Statement::Expr(expr) => exec_expr_stmt(expr, ui)?,
        }
    }
    Ok(())
}

/// Execute an expression used as a statement. Must be either a `ui.element()...` chain
/// or a `ui.text(...)` call.
fn exec_expr_stmt(expr: &Expr, ui: &mut Ui<'_, ()>) -> Result<(), InterpreterError> {
    let (root, chain) = flatten_chain(expr)?;

    if root != "ui" {
        return Err(InterpreterError::eval(format!(
            "Expected `ui.element()` or `ui.text()`, but got `{root}`"
        )));
    }

    if chain.is_empty() {
        return Err(InterpreterError::eval(
            "Expected a method call on `ui`, like `ui.element()` or `ui.text()`",
        ));
    }

    let first_method = &chain[0].name;
    match first_method.as_str() {
        "element" => exec_element_chain(&chain, ui),
        "text" => exec_text_call(&chain, ui),
        other => Err(InterpreterError::eval(format!(
            "Unknown method `ui.{other}()`. Use `ui.element()` or `ui.text()`."
        ))),
    }
}

// ── Method chain flattening ──────────────────────────────────────────

struct FlatCall {
    name: String,
    args: Vec<Expr>,
}

/// Flatten nested MethodCall AST nodes into a linear list: (root_ident, [calls...])
fn flatten_chain(expr: &Expr) -> Result<(String, Vec<FlatCall>), InterpreterError> {
    let mut calls = Vec::new();
    let mut current = expr;

    loop {
        match current {
            Expr::MethodCall {
                receiver,
                method,
                args,
            } => {
                calls.push(FlatCall {
                    name: method.clone(),
                    args: args.clone(),
                });
                current = receiver;
            }
            Expr::Ident(name) => {
                calls.reverse();
                return Ok((name.clone(), calls));
            }
            _ => {
                return Err(InterpreterError::eval(
                    "Expected a method chain starting with `ui`",
                ));
            }
        }
    }
}

// ── Element chain execution ─────────────────────────────────────────

fn exec_element_chain(
    chain: &[FlatCall],
    ui: &mut Ui<'_, ()>,
) -> Result<(), InterpreterError> {
    // chain[0] is "element" — skip it
    if chain.is_empty() || chain[0].name != "element" {
        return Err(InterpreterError::eval("Expected `ui.element()`"));
    }

    let mut builder = ui.element();

    for (_i, call) in chain.iter().enumerate().skip(1) {
        match call.name.as_str() {
            "width" => {
                expect_args(&call.name, &call.args, 1)?;
                let sizing = eval_sizing(&call.args[0])?;
                builder = builder.width(sizing);
            }
            "height" => {
                expect_args(&call.name, &call.args, 1)?;
                let sizing = eval_sizing(&call.args[0])?;
                builder = builder.height(sizing);
            }
            "background_color" => {
                expect_args(&call.name, &call.args, 1)?;
                let color = eval_color(&call.args[0])?;
                builder = builder.background_color(color);
            }
            "corner_radius" => {
                expect_args(&call.name, &call.args, 1)?;
                let cr = eval_corner_radius(&call.args[0])?;
                builder = builder.corner_radius(cr);
            }
            "id" => {
                expect_args(&call.name, &call.args, 1)?;
                let s = eval_string(&call.args[0])?;
                builder = builder.id((&*s, 0u32));
            }
            "aspect_ratio" => {
                expect_args(&call.name, &call.args, 1)?;
                let f = eval_f32(&call.args[0])?;
                builder = builder.aspect_ratio(f);
            }
            "preserve_focus" => {
                builder = builder.preserve_focus();
            }
            "layout" => {
                expect_args(&call.name, &call.args, 1)?;
                let (_params, body) = expect_closure(&call.args[0])?;
                let layout_calls = flatten_closure_chain(&body)?;
                builder = builder.layout(|l| {
                    apply_layout_calls(l, &layout_calls);
                    l
                });
            }
            "floating" => {
                expect_args(&call.name, &call.args, 1)?;
                let (_params, body) = expect_closure(&call.args[0])?;
                let float_calls = flatten_closure_chain(&body)?;
                builder = builder.floating(|f| {
                    apply_floating_calls(f, &float_calls);
                    f
                });
            }
            "border" => {
                expect_args(&call.name, &call.args, 1)?;
                let (_params, body) = expect_closure(&call.args[0])?;
                let border_calls = flatten_closure_chain(&body)?;
                builder = builder.border(|b| {
                    apply_border_calls(b, &border_calls);
                    b
                });
            }
            "overflow" => {
                expect_args(&call.name, &call.args, 1)?;
                let (_params, body) = expect_closure(&call.args[0])?;
                let overflow_calls = flatten_closure_chain(&body)?;
                builder = builder.overflow(|o| {
                    apply_overflow_calls(o, &overflow_calls);
                    o
                });
            }
            "children" => {
                expect_args(&call.name, &call.args, 1)?;
                let (_params, body) = expect_closure(&call.args[0])?;
                let stmts = expect_block_stmts(&body)?;
                // We need to collect errors from inside the closure
                let mut inner_err: Option<InterpreterError> = None;
                builder.children(|child_ui| {
                    if let Err(e) = exec_statements(stmts, child_ui) {
                        inner_err = Some(e);
                    }
                });
                if let Some(e) = inner_err {
                    return Err(e);
                }
                return Ok(());
            }
            "empty" => {
                builder.empty();
                return Ok(());
            }

            // Denied methods with helpful messages
            "on_press" | "on_hover" | "on_release" | "on_focus" | "on_unfocus" => {
                return Err(InterpreterError::unsupported(
                    "Event handlers aren't available in the demo. Check out the interactivity docs!",
                ));
            }
            "text_input" => {
                return Err(InterpreterError::unsupported(
                    "Text input isn't available in the demo. Check out the text input docs!",
                ));
            }
            "effect" | "shader" => {
                return Err(InterpreterError::unsupported(
                    "Shaders aren't available in the demo. Check out the shaders docs!",
                ));
            }
            "image" => {
                return Err(InterpreterError::unsupported(
                    "Images aren't available in the demo. Check out the images docs!",
                ));
            }
            "rotate_visual" | "rotate_shape" => {
                return Err(InterpreterError::unsupported(
                    "Rotation isn't available in the demo yet!",
                ));
            }
            "accessibility" => {
                return Err(InterpreterError::unsupported(
                    "Accessibility isn't available in the demo. Check out the accessibility docs!",
                ));
            }

            other => {
                return Err(InterpreterError::eval(format!(
                    "Unknown element method `.{other}()`. Check the docs for available methods!"
                )));
            }
        }
    }

    Err(InterpreterError::eval(
        "Element chain must end with .children() or .empty()",
    ))
}

// ── Text call execution ──────────────────────────────────────────────

fn exec_text_call(
    chain: &[FlatCall],
    ui: &mut Ui<'_, ()>,
) -> Result<(), InterpreterError> {
    // chain[0] is "text"
    let call = &chain[0];
    if call.args.len() != 2 {
        return Err(InterpreterError::eval(
            "ui.text() takes 2 arguments: a string and a config closure, like ui.text(\"Hello\", |t| t.font_size(24))",
        ));
    }

    let text = eval_string(&call.args[0])?;
    let (_params, body) = expect_closure(&call.args[1])?;
    let text_calls = flatten_closure_chain(&body)?;

    // Check for unsupported text methods before rendering
    for tc in &text_calls {
        if tc.name == "accessible" {
            return Err(InterpreterError::unsupported(
                "Accessible text isn't available in the demo. Check out the accessibility docs!",
            ));
        }
    }

    ui.text(&text, |t| {
        apply_text_config_calls(t, &text_calls);
        t
    });

    if chain.len() > 1 {
        return Err(InterpreterError::eval(
            "ui.text() doesn't support further method chaining",
        ));
    }

    Ok(())
}

// ── Builder closure helpers ─────────────────────────────────────────

/// Flatten the body of a closure like `|l| l.direction(TopToBottom).gap(8)` into method calls.
/// The body can be a chain starting from the closure param, or a block containing statements.
fn flatten_closure_chain(body: &Expr) -> Result<Vec<FlatCall>, InterpreterError> {
    match body {
        Expr::MethodCall { .. } => {
            let (_, calls) = flatten_chain(body)?;
            Ok(calls)
        }
        Expr::Ident(_) => {
            // Just the bare param with no method calls: `|l| l`
            Ok(Vec::new())
        }
        _ => Err(InterpreterError::eval(
            "Expected a method chain in builder closure, like |l| l.gap(8).direction(TopToBottom)",
        )),
    }
}

fn expect_block_stmts(body: &Expr) -> Result<&[Statement], InterpreterError> {
    match body {
        Expr::Block { statements } => Ok(statements),
        _ => Err(InterpreterError::eval(
            "Expected a block `{ ... }` in .children() closure",
        )),
    }
}

fn expect_closure(expr: &Expr) -> Result<(&[String], &Expr), InterpreterError> {
    match expr {
        Expr::Closure { params, body } => Ok((params, body)),
        _ => Err(InterpreterError::eval("Expected a closure like |x| ...")),
    }
}

fn expect_args(method: &str, args: &[Expr], expected: usize) -> Result<(), InterpreterError> {
    if args.len() != expected {
        Err(InterpreterError::eval(format!(
            ".{method}() expects {expected} argument(s), got {}",
            args.len()
        )))
    } else {
        Ok(())
    }
}

// ── LayoutBuilder ───────────────────────────────────────────────────

fn apply_layout_calls(
    l: &mut ply_engine::layout::LayoutBuilder,
    calls: &[FlatCall],
) {
    for call in calls {
        match call.name.as_str() {
            "direction" => {
                if let Ok(dir) = eval_direction(&call.args[0]) {
                    l.direction(dir);
                }
            }
            "padding" => {
                if let Ok(pad) = eval_padding(&call.args[0]) {
                    l.padding(pad);
                }
            }
            "gap" => {
                if let Ok(g) = eval_u16(&call.args[0]) {
                    l.gap(g);
                }
            }
            "align" => {
                if call.args.len() >= 2 {
                    if let (Ok(ax), Ok(ay)) =
                        (eval_align_x(&call.args[0]), eval_align_y(&call.args[1]))
                    {
                        l.align(ax, ay);
                    }
                }
            }
            _ => {} // silently ignore unknown layout methods
        }
    }
}

// ── FloatingBuilder ─────────────────────────────────────────────────

fn apply_floating_calls(
    f: &mut ply_engine::elements::FloatingBuilder,
    calls: &[FlatCall],
) {
    for call in calls {
        match call.name.as_str() {
            "attach_parent" => {
                f.attach_parent();
            }
            "attach_root" => {
                f.attach_root();
            }
            "attach_id" => {
                if let Ok(s) = eval_string(&call.args[0]) {
                    f.attach_id((&*s, 0u32));
                }
            }
            "offset" => {
                if call.args.len() >= 2 {
                    if let (Ok(x), Ok(y)) =
                        (eval_f32(&call.args[0]), eval_f32(&call.args[1]))
                    {
                        f.offset(x, y);
                    }
                }
            }
            "z_index" => {
                if let Ok(z) = eval_i16(&call.args[0]) {
                    f.z_index(z);
                }
            }
            "anchor" => {
                if call.args.len() >= 2 {
                    if let (Ok((ex, ey)), Ok((tx, ty))) =
                        (eval_align_tuple(&call.args[0]), eval_align_tuple(&call.args[1]))
                    {
                        f.anchor((ex, ey), (tx, ty));
                    }
                }
            }
            "clip_by_parent" => {
                f.clip_by_parent();
            }
            "passthrough" => {
                f.passthrough();
            }
            _ => {}
        }
    }
}

// ── BorderBuilder ───────────────────────────────────────────────────

fn apply_border_calls(
    b: &mut ply_engine::elements::BorderBuilder,
    calls: &[FlatCall],
) {
    for call in calls {
        match call.name.as_str() {
            "color" => {
                if let Ok(c) = eval_color(&call.args[0]) {
                    b.color(c);
                }
            }
            "all" => {
                if let Ok(w) = eval_u16(&call.args[0]) {
                    b.all(w);
                }
            }
            "left" => {
                if let Ok(w) = eval_u16(&call.args[0]) {
                    b.left(w);
                }
            }
            "right" => {
                if let Ok(w) = eval_u16(&call.args[0]) {
                    b.right(w);
                }
            }
            "top" => {
                if let Ok(w) = eval_u16(&call.args[0]) {
                    b.top(w);
                }
            }
            "bottom" => {
                if let Ok(w) = eval_u16(&call.args[0]) {
                    b.bottom(w);
                }
            }
            "between_children" => {
                if let Ok(w) = eval_u16(&call.args[0]) {
                    b.between_children(w);
                }
            }
            _ => {}
        }
    }
}

// ── OverflowBuilder ────────────────────────────────────────────────

fn apply_overflow_calls(
    o: &mut ply_engine::elements::OverflowBuilder,
    calls: &[FlatCall],
) {
    for call in calls {
        match call.name.as_str() {
            "clip" => { o.clip(); }
            "clip_x" => { o.clip_x(); }
            "clip_y" => { o.clip_y(); }
            "scroll" => { o.scroll(); }
            "scroll_x" => { o.scroll_x(); }
            "scroll_y" => { o.scroll_y(); }
            _ => {}
        }
    }
}

// ── TextConfig ──────────────────────────────────────────────────────

fn apply_text_config_calls(
    t: &mut ply_engine::text::TextConfig,
    calls: &[FlatCall],
) {
    for call in calls {
        match call.name.as_str() {
            "color" => {
                if let Ok(c) = eval_color(&call.args[0]) {
                    t.color(c);
                }
            }
            "font_size" => {
                if let Ok(s) = eval_u16(&call.args[0]) {
                    t.font_size(s);
                }
            }
            "letter_spacing" => {
                if let Ok(s) = eval_u16(&call.args[0]) {
                    t.letter_spacing(s);
                }
            }
            "line_height" => {
                if let Ok(h) = eval_u16(&call.args[0]) {
                    t.line_height(h);
                }
            }
            "wrap_mode" => {
                if let Ok(m) = eval_wrap_mode(&call.args[0]) {
                    t.wrap_mode(m);
                }
            }
            "alignment" => {
                if let Ok(a) = eval_align_x(&call.args[0]) {
                    t.alignment(a);
                }
            }
            _ => {}
        }
    }
}

// ── Value evaluation helpers ─────────────────────────────────────────

fn eval_sizing(expr: &Expr) -> Result<Sizing, InterpreterError> {
    match expr {
        Expr::MacroCall { name, args } => match name.as_str() {
            "grow" => match args.len() {
                0 => Ok(Sizing::Grow(0.0, f32::MAX)),
                1 => Ok(Sizing::Grow(eval_f32(&args[0])?, f32::MAX)),
                2 => Ok(Sizing::Grow(eval_f32(&args[0])?, eval_f32(&args[1])?)),
                _ => Err(InterpreterError::eval("grow!() takes 0, 1, or 2 arguments")),
            },
            "fit" => match args.len() {
                0 => Ok(Sizing::Fit(0.0, f32::MAX)),
                1 => Ok(Sizing::Fit(eval_f32(&args[0])?, f32::MAX)),
                2 => Ok(Sizing::Fit(eval_f32(&args[0])?, eval_f32(&args[1])?)),
                _ => Err(InterpreterError::eval("fit!() takes 0, 1, or 2 arguments")),
            },
            "fixed" => {
                if args.len() != 1 {
                    return Err(InterpreterError::eval("fixed!() takes exactly 1 argument"));
                }
                Ok(Sizing::Fixed(eval_f32(&args[0])?))
            }
            "percent" => {
                if args.len() != 1 {
                    return Err(InterpreterError::eval("percent!() takes exactly 1 argument"));
                }
                Ok(Sizing::Percent(eval_f32(&args[0])?))
            }
            other => Err(InterpreterError::eval(format!(
                "Unknown sizing macro `{other}!()`. Use grow!(), fit!(), fixed!(), or percent!()"
            ))),
        },
        _ => Err(InterpreterError::eval(
            "Expected a sizing macro like grow!(), fit!(), fixed!(), or percent!()",
        )),
    }
}

fn eval_color(expr: &Expr) -> Result<Color, InterpreterError> {
    match expr {
        Expr::IntLit(n) => Ok(Color::from(*n as i32)),
        Expr::Tuple(elems) => match elems.len() {
            3 => {
                let r = eval_f32(&elems[0])?;
                let g = eval_f32(&elems[1])?;
                let b = eval_f32(&elems[2])?;
                Ok(Color::rgb(r, g, b))
            }
            4 => {
                let r = eval_f32(&elems[0])?;
                let g = eval_f32(&elems[1])?;
                let b = eval_f32(&elems[2])?;
                let a = eval_f32(&elems[3])?;
                Ok(Color::rgba(r, g, b, a))
            }
            n => Err(InterpreterError::eval(format!(
                "Color tuple must have 3 or 4 elements, got {n}"
            ))),
        },
        _ => Err(InterpreterError::eval(
            "Expected a color: hex integer like 0xFF3366 or a tuple like (255.0, 100.0, 50.0)",
        )),
    }
}

fn eval_corner_radius(
    expr: &Expr,
) -> Result<ply_engine::layout::CornerRadius, InterpreterError> {
    match expr {
        Expr::FloatLit(f) => Ok(ply_engine::layout::CornerRadius::from(*f as f32)),
        Expr::IntLit(n) => Ok(ply_engine::layout::CornerRadius::from(*n as f32)),
        Expr::Tuple(elems) if elems.len() == 4 => {
            let tl = eval_f32(&elems[0])?;
            let tr = eval_f32(&elems[1])?;
            let bl = eval_f32(&elems[2])?;
            let br = eval_f32(&elems[3])?;
            Ok(ply_engine::layout::CornerRadius::from((tl, tr, bl, br)))
        }
        _ => Err(InterpreterError::eval(
            "Expected a corner radius: a number like 12.0 or a tuple (tl, tr, bl, br)",
        )),
    }
}

fn eval_padding(expr: &Expr) -> Result<Padding, InterpreterError> {
    match expr {
        Expr::IntLit(n) => {
            let v = *n as u16;
            Ok(Padding::all(v))
        }
        Expr::Tuple(elems) if elems.len() == 4 => {
            let top = eval_u16(&elems[0])?;
            let right = eval_u16(&elems[1])?;
            let bottom = eval_u16(&elems[2])?;
            let left = eval_u16(&elems[3])?;
            Ok(Padding::from((top, right, bottom, left)))
        }
        _ => Err(InterpreterError::eval(
            "Expected padding: a number like 16 or a tuple (top, right, bottom, left)",
        )),
    }
}

fn eval_direction(expr: &Expr) -> Result<LayoutDirection, InterpreterError> {
    match expr {
        Expr::Ident(name) => match name.as_str() {
            "TopToBottom" => Ok(LayoutDirection::TopToBottom),
            "LeftToRight" => Ok(LayoutDirection::LeftToRight),
            other => Err(InterpreterError::eval(format!(
                "Unknown direction `{other}`. Use TopToBottom or LeftToRight"
            ))),
        },
        _ => Err(InterpreterError::eval(
            "Expected a direction: TopToBottom or LeftToRight",
        )),
    }
}

fn eval_align_x(expr: &Expr) -> Result<AlignX, InterpreterError> {
    match expr {
        Expr::Ident(name) => match name.as_str() {
            "Left" => Ok(AlignX::Left),
            "Right" => Ok(AlignX::Right),
            "CenterX" => Ok(AlignX::CenterX),
            other => Err(InterpreterError::eval(format!(
                "Unknown X alignment `{other}`. Use Left, Right, or CenterX"
            ))),
        },
        _ => Err(InterpreterError::eval(
            "Expected an X alignment: Left, Right, or CenterX",
        )),
    }
}

fn eval_align_y(expr: &Expr) -> Result<AlignY, InterpreterError> {
    match expr {
        Expr::Ident(name) => match name.as_str() {
            "Top" => Ok(AlignY::Top),
            "Bottom" => Ok(AlignY::Bottom),
            "CenterY" => Ok(AlignY::CenterY),
            other => Err(InterpreterError::eval(format!(
                "Unknown Y alignment `{other}`. Use Top, Bottom, or CenterY"
            ))),
        },
        _ => Err(InterpreterError::eval(
            "Expected a Y alignment: Top, Bottom, or CenterY",
        )),
    }
}

/// Evaluate a tuple expression `(AlignX, AlignY)` for anchor calls.
fn eval_align_tuple(expr: &Expr) -> Result<(AlignX, AlignY), InterpreterError> {
    match expr {
        Expr::Tuple(elems) if elems.len() == 2 => {
            let x = eval_align_x(&elems[0])?;
            let y = eval_align_y(&elems[1])?;
            Ok((x, y))
        }
        _ => Err(InterpreterError::eval(
            "Expected an alignment tuple like (CenterX, Top)",
        )),
    }
}

fn eval_wrap_mode(expr: &Expr) -> Result<ply_engine::text::WrapMode, InterpreterError> {
    match expr {
        Expr::Ident(name) => match name.as_str() {
            "Words" => Ok(ply_engine::text::WrapMode::Words),
            "Newline" => Ok(ply_engine::text::WrapMode::Newline),
            "None" => Ok(ply_engine::text::WrapMode::None),
            other => Err(InterpreterError::eval(format!(
                "Unknown wrap mode `{other}`. Use Words, Newline, or None"
            ))),
        },
        _ => Err(InterpreterError::eval(
            "Expected a wrap mode: Words, Newline, or None",
        )),
    }
}

fn eval_f32(expr: &Expr) -> Result<f32, InterpreterError> {
    match expr {
        Expr::FloatLit(f) => Ok(*f as f32),
        Expr::IntLit(n) => Ok(*n as f32),
        _ => Err(InterpreterError::eval("Expected a number")),
    }
}

fn eval_u16(expr: &Expr) -> Result<u16, InterpreterError> {
    match expr {
        Expr::IntLit(n) => {
            if *n < 0 || *n > u16::MAX as i64 {
                return Err(InterpreterError::eval(format!(
                    "Value {n} is out of range for u16 (0..65535)"
                )));
            }
            Ok(*n as u16)
        }
        _ => Err(InterpreterError::eval("Expected an integer")),
    }
}

fn eval_i16(expr: &Expr) -> Result<i16, InterpreterError> {
    match expr {
        Expr::IntLit(n) => {
            if *n < i16::MIN as i64 || *n > i16::MAX as i64 {
                return Err(InterpreterError::eval(format!(
                    "Value {n} is out of range for i16"
                )));
            }
            Ok(*n as i16)
        }
        _ => Err(InterpreterError::eval("Expected an integer")),
    }
}

fn eval_string(expr: &Expr) -> Result<String, InterpreterError> {
    match expr {
        Expr::StringLit(s) => Ok(s.clone()),
        _ => Err(InterpreterError::eval("Expected a string")),
    }
}
