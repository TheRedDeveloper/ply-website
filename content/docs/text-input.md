+++
title = "Text Input"
weight = 9
+++

Ply has a full built-in text input system with cursor movement, selection, undo/redo,
multiline editing, password mode, and click-to-position.

## Basic text input

Add `.text_input()` to an element. The element needs an explicit `.id()` so
the engine can persist input state across frames:

<!-- The font_size, text_color, and placeholder should be editable -->
```rust
ui.element()
    .id("username")
    .width(fixed!(300.0))
    .height(fixed!(36.0))
    .background_color(0x262220)
    .corner_radius(6.0)
    .text_input(|t| t
        .placeholder("Enter username")
        .font_size(14)
        .text_color(0xE8E0DC)
        .placeholder_color(0x6E6560)
        .cursor_color(0xFFC32C)
        .selection_color((0.27, 0.51, 0.71, 0.5))
    )
    .empty();
```
<!-- TODO: Embedded WASM demo — basic text input -->

Click to focus, type to enter text. The cursor blinks, selection highlights,
and all standard keyboard shortcuts work.

## Configuration

| Method                     | What it does                              | Default                 |
|----------------------------|-------------------------------------------|-------------------------|
| `.placeholder("text")`     | Ghost text when empty                     | none                    |
| `.font_size(14)`           | Text size in pixels                       | `0`                     |
| `.font(&FONT_ASSET)`       | Which font to use                         | default font            |
| `.text_color(0xFFFFFF)`    | Input text color                          | white                   |
| `.placeholder_color(0x99)` | Placeholder text color                    | gray                    |
| `.cursor_color(0xFFC32C)`  | Cursor line color                         | white                   |
| `.selection_color(c)`      | Selection highlight color                 | semi-transparent blue   |
| `.max_length(100)`         | Maximum character count                   | unlimited               |
| `.password(true)`          | Show dots instead of characters           | `false`                 |
| `.multiline(true)`         | Enable multiline editing                  | `false`                 |
| `.no_styles_movement()`    | Cursor skips style markup (text-styling)  | `false`                 |

## Reading text

```rust
let username = ply.get_text_value("username");
```

or use callbacks to react to changes:

```rust
.text_input(|t| t
    .font_size(14)
    .on_changed(|text| {
        println!("Text changed: {}", text);
    })
    .on_submit(|text| {
        println!("Submitted: {}", text);
    })
)
```

`.on_changed()` fires after every keystroke. `.on_submit()` fires when Enter is
pressed (in single-line mode). If you call either method more than once, the
last callback wins.


## Multiline

Enable with `.multiline(true)`:

```rust
ui.element()
    .id("editor")
    .width(grow!())
    .height(fixed!(200.0))
    .background_color(0x262220)
    .corner_radius(8.0)
    .text_input(|t| t
        .multiline(true)
        .font_size(14)
        .text_color(0xE8E0DC)
    )
    .empty();
```
<!-- TODO: Embedded WASM demo — multiline text editor -->

In multiline mode:
- Enter inserts a newline (instead of triggering submit)
- Up/Down arrows navigate between lines (it remembers the x position)
- The input scrolls vertically when content overflows

## Password mode

```rust
.text_input(|t| t
    .password(true)
    .font_size(14)
    .placeholder("Enter password")
)
```
<!-- TODO: Embedded WASM demo — password input -->

Characters are displayed as dots. Copy/cut is disabled.

## Programmatic control

Manipulate text input state by ID:

```rust
ply.set_text_value("editor", "Hello world");

ply.set_cursor_pos("editor", 5);

let pos = ply.get_cursor_pos("editor");

ply.set_selection("editor", 0, 5);  // select "Hello"

if let Some((start, end)) = ply.get_selection_range("editor") {
    // active selection from start to end
}
```

## Keyboard operations

All standard text editing shortcuts work out of the box:

| Shortcut              | Action                   |
|-----------------------|--------------------------|
| Arrow keys            | Move cursor              |
| Shift + arrows        | Extend selection         |
| Ctrl/Cmd + Left/Right | Move by word             |
| Home / End            | Start/end of line        |
| Ctrl/Cmd + A          | Select all               |
| Ctrl/Cmd + Z          | Undo                     |
| Ctrl/Cmd + Y          | Redo                     |
| Double-click          | Select word              |

## Login form example

```rust
ui.element()
    .width(fixed!(320.0))
    .height(fit!())
    .background_color(0x2E2A28)
    .corner_radius(12.0)
    .layout(|l| l.direction(TopToBottom).padding(24).gap(16))
    .children(|ui| {
        ui.text("Sign In", |t| t.font_size(20).color(0xFFFFFF));

        ui.element()
            .id("email")
            .width(grow!())
            .height(fixed!(36.0))
            .background_color(0x262220)
            .corner_radius(6.0)
            .text_input(|t| t
                .placeholder("Email")
                .font_size(14)
                .text_color(0xE8E0DC)
                .placeholder_color(0x6E6560)
            )
            .empty();

        ui.element()
            .id("password")
            .width(grow!())
            .height(fixed!(36.0))
            .background_color(0x262220)
            .corner_radius(6.0)
            .text_input(|t| t
                .password(true)
                .placeholder("Password")
                .font_size(14)
                .text_color(0xE8E0DC)
                .placeholder_color(0x6E6560)
            )
            .empty();

        button(ui, "Sign In", |_| {
            // ply.get_text_value("email"), ply.get_text_value("password")
        });
    });
```
<!-- TODO: Embedded WASM demo — login form -->

## Next steps

→ [Text Styling](/docs/text-styling/)
