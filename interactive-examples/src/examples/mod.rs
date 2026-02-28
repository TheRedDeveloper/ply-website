use ply_engine::prelude::*;

mod shader_playground;
mod todo_list;
mod snake;

/// A parsed parameter: (key, value) pair, preserving order and duplicates.
pub type Params = Vec<(String, String)>;

/// Parse the raw `key=value\n` string from JS into ordered params.
pub fn parse_params(raw: &str) -> Params {
    let mut params = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            params.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    params
}

/// Get the first value for a given key.
fn get<'a>(params: &'a Params, name: &str) -> Option<&'a str> {
    params
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

/// Get all values for a given key (for multi-line params like multiple `text` entries).
fn get_all<'a>(params: &'a Params, name: &str) -> Vec<&'a str> {
    params
        .iter()
        .filter(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
        .collect()
}

/// Dispatch to the correct compiled example function.
pub fn run(id: &str, params: &Params, ui: &mut Ui<'_, ()>) {
    match id {
        "text_preview" => text_preview(ui, params),
        "layout_card" => layout_card(ui, params),
        "basic_float" => basic_float(ui, params),
        "passthrough_overlay" => passthrough_overlay(ui, params),
        "notification_badge" => notification_badge(ui, params),
        "sidebar_nav" => sidebar_nav(ui, params),
        "hover_highlight" => hover_highlight(ui, params),
        "button_row" => button_row(ui, params),
        "tooltip_demo" => tooltip_demo(ui, params),
        "modal_dialog" => modal_dialog(ui, params),
        "basic_input" => basic_input(ui, params),
        "multiline_editor" => multiline_editor(ui, params),
        "password_input" => password_input(ui, params),
        "login_form" => login_form(ui, params),
        "preserve_focus_demo" => preserve_focus_demo(ui, params),
        "visual_rotation" => visual_rotation(ui, params),
        "shape_rotation" => shape_rotation(ui, params),
        "text_color_demo" => text_color_demo(ui, params),
        "text_wrap_demo" => text_wrap_demo(ui, params),
        "text_align_demo" => text_align_demo(ui, params),
        "indexed_nav_demo" => indexed_nav_demo(ui, params),
        "hover_press_demo" => hover_press_demo(ui, params),
        "sizing_bug_demo" => sizing_bug_demo(ui, params),
        "clipped_list_demo" => clipped_list_demo(ui, params),
        "debug_mode_demo" => debug_mode_demo(ui, params),
        "explicit_id_demo" => explicit_id_demo(ui, params),
        "chart_demo" => chart_demo(ui, params),
        "image_demo" => image_demo(ui, params),
        "tiger_demo" => tiger_demo(ui, params),
        "tint_shader_demo" => tint_shader_demo(ui, params),
        "shader_playground" => shader_playground::run(ui),
        "todo_list" => todo_list::run(ui),
        "snake" => snake::run(ui),
        _ => unknown(ui, id),
    }
}

// ---------------------------------------------------------------------------
// Example: text_preview
// Renders one or more styled text lines. Covers nearly all text-styling.md demos.
//
// Params:
//   text      — styled text string (repeatable for multiple lines)
//   font_size — font size in px (default: 24)
// ---------------------------------------------------------------------------

fn text_preview(ui: &mut Ui<'_, ()>, params: &Params) {
    let default_font_size: u16 = get(params, "font_size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(24)
        .min(500);

    // Collect text params: "text" for single-line, "text1","text2",... for multi-line
    let mut texts: Vec<(&str, u16)> = Vec::new();
    if let Some(t) = get(params, "text") {
        texts.push((t, default_font_size));
    }
    for i in 1..=10 {
        let key = format!("text{i}");
        if let Some(t) = get(params, &key) {
            let fs_key = format!("font_size{i}");
            let fs = get(params, &fs_key)
                .and_then(|s| s.parse().ok())
                .unwrap_or(default_font_size)
                .min(500);
            texts.push((t, fs));
        }
    }
    if texts.is_empty() {
        return;
    }

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| {
            l.direction(LayoutDirection::TopToBottom)
                .align(AlignX::CenterX, AlignY::CenterY)
                .gap(4)
                .padding(8_u16)
        })
        .children(|ui| {
            for (text, fs) in &texts {
                ui.text(text, |t| t.font_size(*fs).color(0xCDD6F4));
            }
        });
}

// ---------------------------------------------------------------------------
// Fallback for unknown example IDs
// ---------------------------------------------------------------------------

fn unknown(ui: &mut Ui<'_, ()>, id: &str) {
    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.text(
                &format!("Unknown example: {id}"),
                |t| t.font_size(14).color(0xFF6B6B),
            );
        });
}

// ---------------------------------------------------------------------------
// Example: layout_card
// Interactive element from elements-and-layout.md with configurable layout.
//
// Params:
//   bg         — background color hex (default: 0x262220)
//   direction  — LeftToRight or TopToBottom (default: LeftToRight)
//   gap        — gap in px (default: 12)
//   padding    — padding in px (default: 16)
// ---------------------------------------------------------------------------

fn layout_card(ui: &mut Ui<'_, ()>, params: &Params) {
    let bg = parse_hex_color(get(params, "bg").unwrap_or("0x262220"));
    let gap: u16 = get(params, "gap")
        .and_then(|s| s.parse().ok())
        .unwrap_or(12);
    let padding = parse_padding(get(params, "padding").unwrap_or("16"));
    let dir = match get(params, "direction").unwrap_or("LeftToRight") {
        "TopToBottom" => LayoutDirection::TopToBottom,
        _ => LayoutDirection::LeftToRight,
    };
    let align_x = match get(params, "align_x").unwrap_or("CenterX") {
        "Left" => AlignX::Left,
        "Right" => AlignX::Right,
        _ => AlignX::CenterX,
    };
    let align_y = match get(params, "align_y").unwrap_or("CenterY") {
        "Top" => AlignY::Top,
        "Bottom" => AlignY::Bottom,
        _ => AlignY::CenterY,
    };

    ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(bg)
        .layout(|l| l.direction(dir).gap(gap).padding(padding).align(align_x, align_y))
        .children(|ui| {
            ui.text("A", |t| t.font_size(24).color(0xFFFFFF));
            ui.text("B", |t| t.font_size(24).color(0xFFFFFF));
            ui.text("C", |t| t.font_size(24).color(0xFFFFFF));
        });
}

// ---------------------------------------------------------------------------
// Example: basic_float
// From floating-elements.md — element with a floating badge.
// ---------------------------------------------------------------------------

fn basic_float(ui: &mut Ui<'_, ()>, _params: &Params) {
    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(200.0))
                .height(fixed!(100.0))
                .background_color(0x2E2A28_u32)
                .children(|ui| {
                    ui.text("I'm the parent", |t| t.font_size(14).color(0xE8E0DC));

                    ui.element()
                        .width(fixed!(80.0))
                        .height(fixed!(30.0))
                        .background_color(0xB91414_u32)
                        .floating(|f| f.attach_parent())
                        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
                        .children(|ui| {
                            ui.text("Float!", |t| t.font_size(12).color(0xFFFFFF));
                        });
                });
        });
}

// ---------------------------------------------------------------------------
// Example: passthrough_overlay
// From floating-elements.md — floating banner with passthrough.
// ---------------------------------------------------------------------------

fn passthrough_overlay(ui: &mut Ui<'_, ()>, _params: &Params) {
    ui.element()
        .width(grow!())
        .height(grow!())
        .children(|ui| {
            ui.element()
                .width(grow!())
                .height(grow!())
                .background_color((255u8, 255u8, 255u8, 26u8))
                .floating(|f| f.attach_root().passthrough())
                .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
                .children(|ui| {
                    ui.text("v0.5.0-dev", |t| t.font_size(12).color(0x9E9590));
                });
        });
}

// ---------------------------------------------------------------------------
// Example: notification_badge
// From floating-elements.md — card with a floating red badge.
// ---------------------------------------------------------------------------

fn notification_badge(ui: &mut Ui<'_, ()>, params: &Params) {
    let count = get(params, "count").unwrap_or("3");
    let badge_color = parse_hex_color(get(params, "badge_color").unwrap_or("0xB91414"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(280.0))
                .height(fixed!(80.0))
                .background_color(0x2E2A28_u32)
                .corner_radius(12.0)
                .layout(|l| l.padding(16_u16).align(AlignX::Left, AlignY::CenterY))
                .children(|ui| {
                    ui.text("Messages", |t| t.font_size(16).color(0xFFFFFF));

                    ui.element()
                        .width(fixed!(24.0))
                        .height(fixed!(24.0))
                        .background_color(badge_color)
                        .corner_radius(12.0)
                        .floating(|f| {
                            f.attach_parent()
                                .anchor((AlignX::CenterX, AlignY::CenterY), (AlignX::Right, AlignY::Top))
                        })
                        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
                        .children(|ui| {
                            ui.text(count, |t| t.font_size(12).color(0xFFFFFF));
                        });
                });
        });
}

// ---------------------------------------------------------------------------
// Example: sidebar_nav
// From elements-and-layout.md — sidebar with nav items.
// ---------------------------------------------------------------------------

fn sidebar_nav(ui: &mut Ui<'_, ()>, params: &Params) {
    let bg = parse_hex_color(get(params, "bg").unwrap_or("0x181515"));
    let items_str = get(params, "items").unwrap_or("Home|Settings|About");

    ui.element()
        .width(fixed!(200.0))
        .height(grow!())
        .background_color(bg)
        .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(4_u16).padding(8_u16))
        .children(|ui| {
            for (i, label) in items_str.split('|').enumerate() {
                nav_item(ui, label, i == 0);
            }
        });
}

fn nav_item(ui: &mut Ui<'_, ()>, label: &str, active: bool) {
    let bg: u32 = if active { 0x3A3533 } else { 0x262220 };
    ui.element()
        .width(grow!())
        .height(fixed!(36.0))
        .background_color(bg)
        .corner_radius(6.0)
        .layout(|l| l.padding(8_u16).align(AlignX::Left, AlignY::CenterY))
        .children(|ui| {
            ui.text(label, |t| t.font_size(14).color(0xE8E0DC));
        });
}

// ---------------------------------------------------------------------------
// Example: hover_highlight
// From interactivity.md — box that changes color on hover/press/focus.
// ---------------------------------------------------------------------------

fn hover_highlight(ui: &mut Ui<'_, ()>, params: &Params) {
    let press_color = parse_hex_color(get(params, "press_color").unwrap_or("0xFF654D"));
    let hover_color = parse_hex_color(get(params, "hover_color").unwrap_or("0x3A3533"));
    let default_color = parse_hex_color(get(params, "default_color").unwrap_or("0x262220"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fit!())
                .height(fixed!(40.0))
                .children(|ui| {
                    let bg = if ui.pressed() {
                        press_color
                    } else if ui.hovered() || ui.focused() {
                        hover_color
                    } else {
                        default_color
                    };

                    ui.element()
                        .width(fit!())
                        .height(grow!())
                        .background_color(bg)
                        .corner_radius(8.0)
                        .layout(|l| {
                            l.padding((0_u16, 16_u16, 0_u16, 16_u16))
                                .align(AlignX::CenterX, AlignY::CenterY)
                        })
                        .children(|ui| {
                            ui.text("Hover me", |t| t.font_size(14).color(0xE8E0DC));
                        });
                });
        });
}

// ---------------------------------------------------------------------------
// Example: button_row
// From interactivity.md — row of buttons with hover/press visual states.
// ---------------------------------------------------------------------------

fn button_row(ui: &mut Ui<'_, ()>, _params: &Params) {
    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| {
            l.direction(LayoutDirection::LeftToRight)
                .gap(8_u16)
                .padding(16_u16)
                .align(AlignX::Left, AlignY::Top)
        })
        .children(|ui| {
            demo_button_log(ui, "Save", "Saved!");
            demo_button_log(ui, "Cancel", "Cancelled!");
            demo_button_log(ui, "Delete", "Deleted!");
        });
}

fn demo_button_log(ui: &mut Ui<'_, ()>, label: &str, log_msg: &str) {
    let msg = log_msg.to_string();
    ui.element()
        .width(fit!())
        .height(fixed!(36.0))
        .corner_radius(6.0)
        .on_press(move |_, _| {
            crate::log_to_console(&msg);
        })
        .children(|ui| {
            let bg = if ui.pressed() {
                0xB91414
            } else if ui.hovered() || ui.focused() {
                0xFF654D
            } else {
                0x3A3533
            };

            ui.element()
                .width(fit!())
                .height(grow!())
                .background_color(bg)
                .corner_radius(6.0)
                .layout(|l| {
                    l.padding((0_u16, 16_u16, 0_u16, 16_u16))
                        .align(AlignX::CenterX, AlignY::CenterY)
                })
                .children(|ui| {
                    ui.text(label, |t| t.font_size(14).color(0xFFFFFF));
                });
        });
}

fn demo_button(ui: &mut Ui<'_, ()>, label: &str) {
    ui.element()
        .width(fit!())
        .height(fixed!(36.0))
        .corner_radius(6.0)
        .children(|ui| {
            let bg = if ui.pressed() {
                0xB91414
            } else if ui.hovered() || ui.focused() {
                0xFF654D
            } else {
                0x3A3533
            };

            ui.element()
                .width(fit!())
                .height(grow!())
                .background_color(bg)
                .corner_radius(6.0)
                .layout(|l| {
                    l.padding((0_u16, 16_u16, 0_u16, 16_u16))
                        .align(AlignX::CenterX, AlignY::CenterY)
                })
                .children(|ui| {
                    ui.text(label, |t| t.font_size(14).color(0xFFFFFF));
                });
        });
}

// ---------------------------------------------------------------------------
// Example: tooltip_demo
// From floating-elements.md — hover an element to see a floating tooltip.
// ---------------------------------------------------------------------------

fn tooltip_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let label = get(params, "label").unwrap_or("Hover me for info");
    let tooltip = get(params, "tooltip").unwrap_or("Extra information here");

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fit!())
                .height(fit!())
                .background_color(0x2E2A28_u32)
                .corner_radius(8.0)
                .layout(|l| l.padding(12_u16).align(AlignX::CenterX, AlignY::CenterY))
                .children(|ui| {
                    ui.text(label, |t| t.font_size(14).color(0xE8E0DC));

                    if ui.hovered() {
                        ui.element()
                            .width(fit!())
                            .height(fit!())
                            .background_color(0x1E1B1B_u32)
                            .corner_radius(4.0)
                            .floating(|f| {
                                f.attach_parent()
                                    .anchor(
                                        (AlignX::CenterX, AlignY::Top),
                                        (AlignX::CenterX, AlignY::Bottom),
                                    )
                                    .offset(0.0, 4.0)
                            })
                            .layout(|l| l.padding(8_u16))
                            .children(|ui| {
                                ui.text(tooltip, |t| {
                                    t.font_size(12).color(0x9E9590)
                                });
                            });
                    }
                });
        });
}

// ---------------------------------------------------------------------------
// Example: modal_dialog
// From floating-elements.md — full-screen overlay with modal dialog.
// ---------------------------------------------------------------------------

fn modal_dialog(ui: &mut Ui<'_, ()>, _params: &Params) {
    // Background content
    ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(0x181515_u32)
        .layout(|l| l.padding(16_u16).align(AlignX::Left, AlignY::Top))
        .children(|ui| {
            ui.text("Background content", |t| t.font_size(14).color(0x6E6560));

            // Overlay: semi-transparent fullscreen
            ui.element()
                .width(grow!())
                .height(grow!())
                .background_color((0u8, 0u8, 0u8, 128u8))
                .floating(|f| f.attach_root())
                .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
                .children(|ui| {
                    // Modal card
                    ui.element()
                        .width(fit!())
                        .height(fit!())
                        .background_color(0x2E2A28_u32)
                        .corner_radius(12.0)
                        .layout(|l| {
                            l.direction(LayoutDirection::TopToBottom)
                                .padding(24_u16)
                                .gap(16_u16)
                        })
                        .children(|ui| {
                            ui.text("Delete item?", |t| t.font_size(20).color(0xFFFFFF));
                            ui.text("This cannot be undone.", |t| {
                                t.font_size(14).color(0x9E9590)
                            });

                            ui.element()
                                .width(grow!())
                                .height(fit!())
                                .layout(|l| {
                                    l.direction(LayoutDirection::LeftToRight)
                                        .gap(8_u16)
                                        .align(AlignX::Right, AlignY::CenterY)
                                })
                                .children(|ui| {
                                    demo_button(ui, "Cancel");
                                    demo_button(ui, "Delete");
                                });
                        });
                });
        });
}

// ---------------------------------------------------------------------------
// Example: basic_input
// From text-input.md — single-line text input with placeholder.
// ---------------------------------------------------------------------------

fn basic_input(ui: &mut Ui<'_, ()>, params: &Params) {
    let placeholder = get(params, "placeholder").unwrap_or("Enter username");
    let font_size: u16 = get(params, "font_size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(14);
    let text_color = parse_hex_color(get(params, "text_color").unwrap_or("0xE8E0DC"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .id("username")
                .width(fixed!(300.0))
                .height(fixed!(36.0))
                .background_color(0x262220_u32)
                .corner_radius(6.0)
                .text_input(|t| {
                    t.placeholder(placeholder)
                        .font_size(font_size)
                        .text_color(text_color)
                        .placeholder_color(0x6E6560)
                        .cursor_color(0xFFC32C)
                        .selection_color((69u8, 130u8, 181u8, 128u8))
                })
                .empty();
        });
}

// ---------------------------------------------------------------------------
// Example: multiline_editor
// From text-input.md — multiline text editor.
// ---------------------------------------------------------------------------

fn multiline_editor(ui: &mut Ui<'_, ()>, params: &Params) {
    let font_size: u16 = get(params, "font_size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(14);
    let bg = parse_hex_color(get(params, "bg").unwrap_or("0x262220"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.padding(8_u16))
        .children(|ui| {
            ui.element()
                .id("editor")
                .width(grow!())
                .height(grow!())
                .background_color(bg)
                .corner_radius(8.0)
                .text_input(|t| {
                    t.multiline(true)
                        .font_size(font_size)
                        .text_color(0xE8E0DC)
                        .cursor_color(0xFFC32C)
                        .selection_color((69u8, 130u8, 181u8, 128u8))
                })
                .empty();
        });
}

// ---------------------------------------------------------------------------
// Example: password_input
// From text-input.md — password mode text input.
// ---------------------------------------------------------------------------

fn password_input(ui: &mut Ui<'_, ()>, params: &Params) {
    let placeholder = get(params, "placeholder").unwrap_or("Enter password");

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .id("password")
                .width(fixed!(300.0))
                .height(fixed!(36.0))
                .background_color(0x262220_u32)
                .corner_radius(6.0)
                .text_input(|t| {
                    t.password(true)
                        .placeholder(placeholder)
                        .font_size(14)
                        .text_color(0xE8E0DC)
                        .placeholder_color(0x6E6560)
                        .cursor_color(0xFFC32C)
                })
                .empty();
        });
}

// ---------------------------------------------------------------------------
// Example: login_form
// From text-input.md — email + password + sign in button.
// ---------------------------------------------------------------------------

fn login_form(ui: &mut Ui<'_, ()>, _params: &Params) {
    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(320.0))
                .height(fit!())
                .background_color(0x1E1B1B_u32)
                .corner_radius(12.0)
                .layout(|l| {
                    l.direction(LayoutDirection::TopToBottom)
                        .padding(24_u16)
                        .gap(12_u16)
                })
                .children(|ui| {
                    ui.text("Sign In", |t| t.font_size(20).color(0xFFFFFF));

                    // Email input
                    ui.element()
                        .id("email")
                        .width(grow!())
                        .height(fixed!(36.0))
                        .background_color(0x262220_u32)
                        .corner_radius(6.0)
                        .text_input(|t| {
                            t.placeholder("Email")
                                .font_size(14)
                                .text_color(0xE8E0DC)
                                .placeholder_color(0x6E6560)
                                .cursor_color(0xFFC32C)
                        })
                        .empty();

                    // Password input
                    ui.element()
                        .id("login_password")
                        .width(grow!())
                        .height(fixed!(36.0))
                        .background_color(0x262220_u32)
                        .corner_radius(6.0)
                        .text_input(|t| {
                            t.password(true)
                                .placeholder("Password")
                                .font_size(14)
                                .text_color(0xE8E0DC)
                                .placeholder_color(0x6E6560)
                                .cursor_color(0xFFC32C)
                        })
                        .empty();

                    // Sign In button
                    demo_button(ui, "Sign In");
                });
        });
}

// ---------------------------------------------------------------------------
// Example: preserve_focus_demo
// From interactivity.md — text input with a "Make red" button that keeps
// focus on the input using .preserve_focus().
// ---------------------------------------------------------------------------

fn preserve_focus_demo(ui: &mut Ui<'_, ()>, _params: &Params) {
    // Check if "Make red" was pressed this frame and apply formatting
    if ui.is_pressed("make_red") {
        let raw = ui.get_text_value("styled_input").to_string();
        if let Some((start, end)) = ui.get_selection_range("styled_input") {
            if start != end {
                let (vis_lo, vis_hi) = if start < end { (start, end) } else { (end, start) };
                // Convert visual cursor positions to raw char positions
                let raw_lo = styling::cursor_to_raw(&raw, vis_lo);
                let raw_hi = styling::cursor_to_raw(&raw, vis_hi);
                // Convert raw char positions to byte offsets
                let chars: Vec<(usize, char)> = raw.char_indices().collect();
                let lo_byte = chars.get(raw_lo).map(|c| c.0).unwrap_or(raw.len());
                let hi_byte = chars.get(raw_hi).map(|c| c.0).unwrap_or(raw.len());
                let before = &raw[..lo_byte];
                let selected = &raw[lo_byte..hi_byte];
                let after = &raw[hi_byte..];
                let new_text = format!("{before}{{color=red|{selected}}}{after}");
                // visual cursor offset: the {color=red| header is invisible, } adds 1
                ui.set_text_value("styled_input", &new_text);
                ui.set_cursor_pos("styled_input", vis_hi);
            }
        }
    }

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| {
            l.direction(LayoutDirection::LeftToRight)
                .gap(8_u16)
                .padding(12_u16)
                .align(AlignX::Left, AlignY::CenterY)
        })
        .children(|ui| {
            // Text input
            ui.element()
                .id("styled_input")
                .width(fixed!(220.0))
                .height(fixed!(36.0))
                .background_color(0x262220_u32)
                .corner_radius(6.0)
                .text_input(|t| {
                    t.font_size(14)
                        .text_color(0xE8E0DC)
                        .placeholder("Type here...")
                        .placeholder_color(0x6E6560)
                        .cursor_color(0xFFC32C)
                })
                .empty();

            // "Make red" toolbar button with preserve_focus
            ui.element()
                .id("make_red")
                .width(fit!())
                .height(fixed!(36.0))
                .corner_radius(6.0)
                .preserve_focus()
                .children(|ui| {
                    let bg = if ui.pressed() {
                        0xB91414
                    } else if ui.hovered() || ui.focused() {
                        0xFF654D
                    } else {
                        0x3A3533
                    };
                    ui.element()
                        .width(fit!())
                        .height(grow!())
                        .background_color(bg)
                        .corner_radius(6.0)
                        .layout(|l| {
                            l.padding((0_u16, 12_u16, 0_u16, 12_u16))
                                .align(AlignX::CenterX, AlignY::CenterY)
                        })
                        .children(|ui| {
                            ui.text("Make red", |t| t.font_size(14).color(0xFFFFFF));
                        });
                });
        });
}

// ---------------------------------------------------------------------------
// Example: visual_rotation
// From rotation.md — visually rotate an element (children rotate too).
// ---------------------------------------------------------------------------

fn visual_rotation(ui: &mut Ui<'_, ()>, params: &Params) {
    let degrees: f32 = get(params, "degrees")
        .and_then(|s| s.parse().ok())
        .unwrap_or(15.0);
    let bg = parse_hex_color(get(params, "bg").unwrap_or("0xFFC32C"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(120.0))
                .height(fixed!(80.0))
                .background_color(bg)
                .corner_radius(8.0)
                .rotate_visual(|r| r.degrees(degrees))
                .children(|ui| {
                    ui.text("Tilted!", |t| t.font_size(14).color(0xFFFFFF));
                });
        });
}

// ---------------------------------------------------------------------------
// Example: shape_rotation
// From rotation.md — rotate element geometry (AABB adjusts).
// ---------------------------------------------------------------------------

fn shape_rotation(ui: &mut Ui<'_, ()>, params: &Params) {
    let degrees: f32 = get(params, "degrees")
        .and_then(|s| s.parse().ok())
        .unwrap_or(45.0);
    let bg = parse_hex_color(get(params, "bg").unwrap_or("0xFF654D"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(80.0))
                .height(fixed!(80.0))
                .background_color(bg)
                .rotate_shape(|r| r.degrees(degrees))
                .empty();
        });
}

fn text_color_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let label1 = get(params, "label1").unwrap_or("Big and red");
    let font_size1: u16 = get(params, "font_size1")
        .and_then(|s| s.parse().ok())
        .unwrap_or(32)
        .min(500);
    let color1 = parse_hex_color(get(params, "color1").unwrap_or("0xB91414"));
    let label2 = get(params, "label2").unwrap_or("Small and gold");
    let font_size2: u16 = get(params, "font_size2")
        .and_then(|s| s.parse().ok())
        .unwrap_or(12)
        .min(500);
    let color2 = parse_hex_color(get(params, "color2").unwrap_or("0xFFC32C"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.direction(LayoutDirection::TopToBottom).padding(16).gap(8))
        .children(|ui| {
            ui.text(label1, |t| {
                t.font_size(font_size1).color(color1)
            });
            ui.text(label2, |t| {
                t.font_size(font_size2).color(color2)
            });
        });
}

fn text_wrap_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let width: f32 = get(params, "width")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200.0);
    let wrap_mode = match get(params, "wrap_mode").unwrap_or("Words") {
        "Newline" => WrapMode::Newline,
        "None" => WrapMode::None,
        _ => WrapMode::Words,
    };

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(width))
                .background_color(0x3A3330)
                .layout(|l| l.padding(12))
                .children(|ui| {
                    ui.text(
                        "This text will wrap at word boundaries when it runs out of space.",
                        |t| {
                            t.font_size(14).color(0xE8E0DC).wrap_mode(wrap_mode)
                        },
                    );
                });
        });
}

fn text_align_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let alignment = match get(params, "alignment").unwrap_or("CenterX") {
        "Left" => AlignX::Left,
        "Right" => AlignX::Right,
        _ => AlignX::CenterX,
    };
    let font_size: u16 = get(params, "font_size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(24)
        .min(500);

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(280.0))
                .background_color(0x3A3330)
                .layout(|l| l.padding(12))
                .children(|ui| {
                    ui.text("Aligned heading", |t| {
                        t.font_size(font_size).color(0xFFFFFF).alignment(alignment)
                    });
                });
        });
}

fn indexed_nav_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let active: usize = get(params, "active")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let items_raw = get(params, "items").unwrap_or("Home|Settings|Profile|About");
    let items: Vec<&str> = items_raw.split('|').collect();

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(200.0))
                .height(grow!())
                .background_color(0x181515)
                .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(4).padding(8))
                .children(|ui| {
                    for (i, label) in items.iter().enumerate() {
                        let bg = if i == active { 0x3A3533 } else { 0x262220 };
                        ui.element()
                            .id(("nav_item", i as u32))
                            .width(grow!())
                            .height(fixed!(36.0))
                            .background_color(bg)
                            .corner_radius(6.0)
                            .layout(|l| l.padding(8).align(AlignX::Left, AlignY::CenterY))
                            .children(|ui| {
                                ui.text(label, |t| t.font_size(14).color(0xE8E0DC));
                            });
                    }
                });
        });
}

fn hover_press_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let press_color = parse_hex_color(get(params, "press_color").unwrap_or("0xFF654D"));
    let hover_color = parse_hex_color(get(params, "hover_color").unwrap_or("0x3A3533"));
    let default_color = parse_hex_color(get(params, "default_color").unwrap_or("0x262220"));

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fit!())
                .height(fixed!(40.0))
                .corner_radius(8.0)
                .children(|ui| {
                    let bg = if ui.pressed() {
                        press_color
                    } else if ui.hovered() {
                        hover_color
                    } else {
                        default_color
                    };

                    ui.element()
                        .width(fit!())
                        .height(grow!())
                        .background_color(bg)
                        .corner_radius(8.0)
                        .layout(|l| l.padding((0, 16, 0, 16)).align(AlignX::CenterX, AlignY::CenterY))
                        .children(|ui| {
                            ui.text("Hover me", |t| t.font_size(14).color(0xE8E0DC));
                        });
                });
        });
}

fn sizing_bug_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    ui.set_debug_mode(true);
    let use_grow = get(params, "parent_width").unwrap_or("fit!") == "grow!";
    let parent_w = if use_grow { grow!() } else { fit!() };

    ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(0x1E1B1B)
        .layout(|l| l.direction(LayoutDirection::LeftToRight))
        .children(|ui| {
            ui.element()
                .width(parent_w)
                .height(grow!())
                .background_color(0x1E1B1B)
                .layout(|l| l.direction(LayoutDirection::LeftToRight))
                .children(|ui| {
                    // Sidebar
                    ui.element()
                        .width(percent!(0.25))
                        .height(grow!())
                        .background_color(0x262220)
                        .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(8).padding(12))
                        .children(|ui| {
                            ui.text("Sidebar", |t| t.font_size(16).color(0xFFC32C));
                            ui.text("Home", |t| t.font_size(14).color(0xE8E0DC));
                            ui.text("Settings", |t| t.font_size(14).color(0xE8E0DC));
                        });

                    // Content
                    ui.element()
                        .width(grow!())
                        .height(grow!())
                        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
                        .children(|ui| {
                            ui.text("Why is everything crushed?", |t| t.font_size(24).color(0xFFFFFF));
                        });
                });
        });
}

fn clipped_list_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    ui.set_debug_mode(true);
    let height: f32 = get(params, "height")
        .and_then(|s| s.parse().ok())
        .unwrap_or(41.0);
    let overflow_mode = get(params, "overflow").unwrap_or("clip");

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            let el = ui.element()
                .width(fit!())
                .height(fixed!(height))
                .background_color(0x2E2A28)
                .corner_radius(8.0)
                .layout(|l| l.direction(LayoutDirection::TopToBottom).padding(8).gap(8));
            let el = match overflow_mode {
                "scroll" => el.overflow(|o| o.scroll()),
                "clip" => el.overflow(|o| o.clip()),
                _ => el,
            };
            el.children(|ui| {
                for name in ["Alice", "Bob", "Charlie", "Diana", "Eve"] {
                    ui.text(name, |t| t.font_size(30).color(0xE8E0DC));
                }
            });
        });
}

fn debug_mode_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let debug = get(params, "debug").unwrap_or("true") == "true";
    ui.set_debug_mode(debug);
    // Build a rich layout to explore in the debug inspector.
    ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(0x1E1B1B)
        .layout(|l| l.direction(LayoutDirection::LeftToRight))
        .children(|ui| {
            // Sidebar
            ui.element()
                .id("sidebar")
                .width(fixed!(140.0))
                .height(grow!())
                .background_color(0x262220)
                .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(4).padding(8))
                .children(|ui| {
                    ui.text("Dashboard", |t| t.font_size(16).color(0xFFC32C));
                    for (i, label) in ["Home", "Stats", "Settings"].iter().enumerate() {
                        let bg = if i == 0 { 0x3A3533 } else { 0x262220 };
                        ui.element()
                            .id(("nav", i as u32))
                            .width(grow!())
                            .height(fixed!(28.0))
                            .background_color(bg)
                            .corner_radius(4.0)
                            .layout(|l| l.padding(6).align(AlignX::Left, AlignY::CenterY))
                            .children(|ui| {
                                ui.text(label, |t| t.font_size(12).color(0xE8E0DC));
                            });
                    }
                });

            // Main content
            ui.element()
                .width(grow!())
                .height(grow!())
                .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(8).padding(12))
                .children(|ui| {
                    // Header
                    ui.text("Welcome back", |t| t.font_size(18).color(0xFFFFFF));

                    // Cards row
                    ui.element()
                        .width(grow!())
                        .layout(|l| l.direction(LayoutDirection::LeftToRight).gap(8))
                        .children(|ui| {
                            for (label, value, color) in [
                                ("Users", "1,204", 0xFF654D_u32),
                                ("Revenue", "$48k", 0xFFC32C),
                                ("Growth", "+12%", 0x4CAF50),
                            ] {
                                ui.element()
                                    .width(grow!())
                                    .background_color(0x2E2A28)
                                    .corner_radius(8.0)
                                    .layout(|l| l.direction(LayoutDirection::TopToBottom).padding(10).gap(4))
                                    .children(|ui| {
                                        ui.text(label, |t| t.font_size(10).color(0x999999));
                                        ui.text(value, |t| t.font_size(20).color(color));
                                    });
                            }
                        });

                    // Table-like area
                    ui.element()
                        .width(grow!())
                        .height(grow!())
                        .background_color(0x2E2A28)
                        .corner_radius(8.0)
                        .layout(|l| l.direction(LayoutDirection::TopToBottom).padding(10).gap(6))
                        .children(|ui| {
                            ui.text("Recent activity", |t| t.font_size(12).color(0x999999));
                            for name in ["Alice joined", "Bob purchased", "Carol signed up"] {
                                ui.element()
                                    .width(grow!())
                                    .height(fixed!(24.0))
                                    .background_color(0x3A3533)
                                    .corner_radius(4.0)
                                    .layout(|l| l.padding(6).align(AlignX::Left, AlignY::CenterY))
                                    .children(|ui| {
                                        ui.text(name, |t| t.font_size(11).color(0xE8E0DC));
                                    });
                            }
                        });
                });
        });
}

fn explicit_id_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let id_label = get(params, "id_label").unwrap_or("sidebar");

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.direction(LayoutDirection::LeftToRight).gap(12).padding(12))
        .children(|ui| {
            let sidebar_id = ui.element()
                .id("sidebar")
                .width(fixed!(160.0))
                .height(grow!())
                .background_color(0x181515)
                .layout(|l| l.direction(LayoutDirection::TopToBottom).gap(4).padding(8))
                .children(|ui| {
                    ui.text("Navigation", |t| t.font_size(14).color(0xFFC32C));
                    for label in ["Home", "Settings", "Profile"] {
                        ui.element()
                            .width(grow!())
                            .height(fixed!(28.0))
                            .background_color(0x262220)
                            .corner_radius(4.0)
                            .layout(|l| l.padding(6).align(AlignX::Left, AlignY::CenterY))
                            .children(|ui| {
                                ui.text(label, |t| t.font_size(12).color(0xE8E0DC));
                            });
                    }
                });

            if let Some(bbox) = ui.bounding_box(sidebar_id) {
                crate::log_to_console(&format!(
                    "\"{}\" bounding box: x={:.0}, y={:.0}, w={:.0}, h={:.0}",
                    id_label, bbox.x, bbox.y, bbox.width, bbox.height
                ));
            }
        });
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn parse_hex_color(s: &str) -> u32 {
    let s = s.trim().trim_start_matches("0x").trim_start_matches('#');
    u32::from_str_radix(s, 16).unwrap_or(0x262220)
}

fn parse_padding(s: &str) -> (u16, u16, u16, u16) {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<u16> = inner.split(',').filter_map(|p| p.trim().parse().ok()).collect();
        match parts.len() {
            4 => (parts[0], parts[1], parts[2], parts[3]),
            2 => (parts[0], parts[1], parts[0], parts[1]),
            1 => (parts[0], parts[0], parts[0], parts[0]),
            _ => (16, 16, 16, 16),
        }
    } else {
        let v: u16 = s.parse().unwrap_or(16);
        (v, v, v, v)
    }
}

// ---------------------------------------------------------------------------
// Example: chart_demo
// Draws a simple chart using render_to_texture with macroquad draw calls.
// ---------------------------------------------------------------------------

fn chart_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let w: f32 = get(params, "width")
        .and_then(|s| s.parse().ok())
        .unwrap_or(400.0);
    let h: f32 = get(params, "height")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200.0);

    let chart = render_to_texture(w, h, || {
        clear_background(MacroquadColor::new(0.0, 0.0, 0.0, 0.0));
        let data = [0.2, 0.5, 0.3, 0.8, 0.6, 0.9, 0.4, 0.7];
        let step = w / (data.len() - 1) as f32;
        for i in 0..data.len() - 1 {
            let x1 = step * i as f32;
            let y1 = h - data[i] as f32 * h;
            let x2 = step * (i + 1) as f32;
            let y2 = h - data[i + 1] as f32 * h;
            draw_line(
                x1, y1, x2, y2, 2.0,
                GREEN,
            );
        }
        for (i, &val) in data.iter().enumerate() {
            let x = step * i as f32;
            let y = h - val as f32 * h;
            draw_circle(x, y, 4.0, RED);
        }
    });

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(w))
                .height(fixed!(h))
                .image(chart)
                .empty();
        });
}

// ---------------------------------------------------------------------------
// Example: image_demo
// Displays a PNG image from a file path.
// ---------------------------------------------------------------------------

static DEMO_IMAGE: GraphicAsset = GraphicAsset::Bytes {
    file_name: "logo.png",
    data: include_bytes!("../../assets/images/logo.png"),
};

fn image_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let w: f32 = get(params, "width")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200.0);
    let h: f32 = get(params, "height")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200.0);

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(w))
                .height(fixed!(h))
                .corner_radius(8.0)
                .image(&DEMO_IMAGE)
                .empty();
        });
}

// ---------------------------------------------------------------------------
// Example: tiger_demo
// Displays a TinyVG vector graphic with editable dimensions.
// ---------------------------------------------------------------------------

static TIGER: GraphicAsset = GraphicAsset::Bytes {
    file_name: "tiger.tvg",
    data: include_bytes!("../../assets/images/tiger.tvg"),
};

fn tiger_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let w: f32 = get(params, "width")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200.0);
    let h: f32 = get(params, "height")
        .and_then(|s| s.parse().ok())
        .unwrap_or(200.0);

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(w))
                .height(fixed!(h))
                .image(&TIGER)
                .empty();
        });
}

// ---------------------------------------------------------------------------
// Example: tint_shader_demo
// Applies a pulsing tint shader to an element.
// ---------------------------------------------------------------------------

static TINT_SHADER: ShaderAsset = ShaderAsset::Source {
    file_name: "tint.frag",
    fragment: "#version 100
precision highp float;

varying lowp vec2 uv;

uniform sampler2D Texture;
uniform float u_time;
uniform vec4 u_tint;

void main() {
    vec4 col = texture2D(Texture, uv);
    float strength = u_tint.a * (0.5 + 0.5 * sin(u_time * 3.0));
    gl_FragColor = mix(col, vec4(u_tint.rgb, col.a), strength);
}
",
};

fn tint_shader_demo(ui: &mut Ui<'_, ()>, params: &Params) {
    let tint_r: f32 = get(params, "tint_r")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let tint_g: f32 = get(params, "tint_g")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.3);
    let tint_b: f32 = get(params, "tint_b")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.2);

    let time = get_time() as f32;

    ui.element()
        .width(grow!())
        .height(grow!())
        .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
        .children(|ui| {
            ui.element()
                .width(fixed!(200.0))
                .height(fixed!(100.0))
                .background_color(0xE8E0DC)
                .corner_radius(8.0)
                .effect(&TINT_SHADER, |s| {
                    s.uniform("u_time", time)
                        .uniform("u_tint", [tint_r, tint_g, tint_b, 0.5f32]);
                })
                .layout(|l| l.align(AlignX::CenterX, AlignY::CenterY))
                .children(|ui| {
                    ui.text("Tinted element", |t| t.font_size(16).color(0x1E1B1B));
                });
        });
}
