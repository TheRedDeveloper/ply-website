pub mod ast;
pub mod error;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod tokens;

use ply_engine::prelude::*;

pub use error::InterpreterError;

/// Interpret a string of Ply UI code, calling real Ply API methods on the given Ui.
pub fn interpret(code: &str, ui: &mut Ui<'_, '_>) -> Result<(), InterpreterError> {
    let tokens = lexer::lex(code)?;
    let program = parser::parse(tokens)?;
    eval::exec_program(&program, ui)
}

/// Render an interpreter error as styled text in the Ply canvas.
pub fn render_error(ui: &mut Ui<'_, '_>, err: &InterpreterError) {
    ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(0x2D1117)
        .layout(|l| {
            l.direction(LayoutDirection::TopToBottom)
                .padding(16_u16)
                .gap(4)
                .align(AlignX::CenterX, AlignY::CenterY)
        })
        .children(|ui| {
            ui.text(&err.to_string(), |t| t.font_size(14).color(0xFF6B6B));
        });
}
