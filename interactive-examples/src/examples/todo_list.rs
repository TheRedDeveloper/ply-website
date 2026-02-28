use ply_engine::prelude::*;
use std::cell::RefCell;

static CHECK_ICON: GraphicAsset = GraphicAsset::Bytes {
  file_name: "check.tvg",
  data: include_bytes!("../../assets/images/check.tvg"),
};

static CLOSE_ICON: GraphicAsset = GraphicAsset::Bytes {
  file_name: "close.tvg",
  data: include_bytes!("../../assets/images/close.tvg"),
};

struct TodoItem {
  text: String,
  done: bool,
}

thread_local! {
  static TODOS: RefCell<Vec<TodoItem>> = RefCell::new(vec![
    TodoItem { text: "Learn Ply engine basics".into(), done: true },
    TodoItem { text: "Build your own app".into(), done: false },
  ]);
  static SUBMIT_PENDING: RefCell<bool> = RefCell::new(false);
}

fn add_todo(ui: &mut Ui<'_, ()>) {
  let text = ui.get_text_value("todo_input").to_string();
  if !text.trim().is_empty() {
    TODOS.with(|todos| {
      todos.borrow_mut().push(TodoItem {
        text: text.trim().to_string(),
        done: false,
      });
    });
    ui.set_text_value("todo_input", "");
  }
}

pub fn run(ui: &mut Ui<'_, ()>) {
  // Root
  ui.element()
    .width(grow!())
    .height(grow!())
    .background_color(0x1E1B1B)
    .layout(|l| l.align(CenterX, Top).padding(24_u16))
    .children(|ui| {
      // Card
      ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(0x262220)
        .corner_radius(12.0)
        .layout(|l| l.direction(TopToBottom).padding(20).gap(16))
        .children(|ui| {
          // Title
          ui.text("Todo List", |t| t.font_size(22).color(0xE8E0DC));

          // Input row
          ui.element()
            .width(grow!())
            .height(fixed!(40.0))
            .layout(|l| l.gap(8))
            .children(|ui| {
              // Input container
              ui.element()
                .width(grow!())
                .height(grow!())
                .background_color(0x3A3533)
                .corner_radius(8.0)
                .layout(|l| l.padding((0, 12, 0, 12)))
                .children(|ui| {
                  // Text input
                  ui.element()
                    .id("todo_input")
                    .width(grow!())
                    .height(grow!())
                    .text_input(|t| t
                      .placeholder("What needs to be done?")
                      .font_size(14)
                      .text_color(0xE8E0DC)
                      .placeholder_color(0x6E6560)
                      .cursor_color(0xFF654D)
                      .selection_color((255u8, 101u8, 77u8, 51u8))
                      .on_submit(|_| {
                        SUBMIT_PENDING.with(|f| *f.borrow_mut() = true);
                      })
                    )
                    .empty();
                });

              // Add button
              ui.element()
                .id("add_btn")
                .width(fixed!(40.0))
                .height(fixed!(40.0))
                .children(|ui| {
                  let bg: u32 = if ui.pressed() {
                    0xB91414
                  } else if ui.hovered() {
                    0x3A3533
                  } else {
                    0x4A4440
                  };
                  ui.element()
                    .width(grow!())
                    .height(grow!())
                    .background_color(bg)
                    .corner_radius(8.0)
                    .layout(|l| l.align(CenterX, CenterY))
                    .children(|ui| {
                      ui.text("+", |t| t.font_size(20).color(0xE8E0DC));
                    });
                });
            });

          // Handle add (button click or Enter key)
          let submit = SUBMIT_PENDING.with(|f| {
            let val = *f.borrow();
            *f.borrow_mut() = false;
            val
          });
          if ui.is_pressed("add_btn") || submit {
            add_todo(ui);
          }

          // Scrollable todo list
          ui.element()
            .width(grow!())
            .height(grow!())
            .overflow(|o| o.scroll_y())
            .layout(|l| l.direction(TopToBottom).gap(4_u16))
            .children(|ui| {
              TODOS.with(|todos| {
                let items: Vec<_> = todos.borrow().iter().enumerate()
                  .map(|(i, item)| (i as u32, item.done, item.text.clone()))
                  .collect();
                for (idx, done, text) in items {
                  todo_row(ui, idx, done, &text);
                }
              });
            });

          // Footer
          TODOS.with(|todos| {
            let total = todos.borrow().len();
            let remaining = todos.borrow().iter().filter(|t| !t.done).count();
            let word = if total == 1 { "item" } else { "items" };
            let label = format!("{} / {} {} left", remaining, total, word);
            ui.text(&label, |t| t.font_size(12).color(0x9E9590));
          });
        });
    });
}

fn todo_row(ui: &mut Ui<'_, ()>, idx: u32, done: bool, text: &str) {
  ui.element()
    .width(grow!())
    .height(fit!())
    .children(|ui| {
      let row_bg: u32 = if ui.hovered() { 0x302C2A } else { 0x262220 };

      ui.element()
        .width(grow!())
        .height(fit!())
        .background_color(row_bg)
        .corner_radius(8.0)
        .layout(|l| l.gap(10_u16).padding((8_u16, 10, 8, 10)).align(Left, CenterY))
        .children(|ui| {
          // Checkbox
          checkbox(ui, idx, done);

          // Text
          let text_color: u32 = if done { 0x6E6560 } else { 0xE8E0DC };
          ui.text(text, |t| t.font_size(14).color(text_color));

          // Spacer
          ui.element().width(grow!()).height(fixed!(1.0)).empty();

          // Delete
          delete_button(ui, idx);
        });
    });
}

fn checkbox(ui: &mut Ui<'_, ()>, idx: u32, done: bool) {
  let idx_copy = idx;
  ui.element()
    .id(("todo_check", idx))
    .width(fixed!(22.0))
    .height(fixed!(22.0))
    .on_press(move |_, _| {
      TODOS.with(|todos| {
        let mut t = todos.borrow_mut();
        if let Some(item) = t.get_mut(idx_copy as usize) {
          item.done = !item.done;
        }
      });
    })
    .children(|ui| {
      let (bg, border_col): (u32, u32) = if done {
        (0x6ECB63, 0x6ECB63)
      } else if ui.hovered() {
        (0x3A3533, 0xFF654D)
      } else {
        (0x3A3533, 0x4A4440)
      };
      ui.element()
        .width(grow!())
        .height(grow!())
        .background_color(bg)
        .corner_radius(11.0)
        .border(|b| b.all(2).color(border_col))
        .children(|ui| {
          if done {
            ui.element()
              .width(grow!())
              .height(grow!())
              .image(&CHECK_ICON)
              .empty();
          }
        });
    });
}

fn delete_button(ui: &mut Ui<'_, ()>, idx: u32) {
  let idx_copy = idx;
  ui.element()
    .id(("todo_del", idx))
    .width(fixed!(22.0))
    .height(fixed!(22.0))
    .on_press(move |_, _| {
      TODOS.with(|todos| {
        let mut t = todos.borrow_mut();
        if (idx_copy as usize) < t.len() {
          t.remove(idx_copy as usize);
        }
      });
    })
    .image(&CLOSE_ICON)
    .empty();
}
