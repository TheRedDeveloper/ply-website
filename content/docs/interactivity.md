+++
title = "Interactivity"
weight = 6
+++

Make your elements respond to clicks, hovers, and keyboard focus.
Ply gives you two levels of control: inline state queries inside
a children closure, and callback-based events on the element builder.

## Hover

### Inline

Check `ui.hovered()` inside a `.children()` closure to react to the pointer
being over the current element:

<!-- The bg colors should be editable -->
```rust
ui.element().width(fit!()).height(fixed!(40.0))
    .children(|ui| {
        let bg = if ui.hovered() { 0x3A3533 } else { 0x262220 };

        ui.element().width(fit!()).height(grow!())
            .background_color(bg)
            .corner_radius(8.0)
            .layout(|l| l.padding((0, 16, 0, 16)).align(CenterX, CenterY))
            .children(|ui| {
                ui.text("Hover me", |t| t.font_size(14).color(0xE8E0DC));
            });
    });
```
<!-- TODO: Embedded WASM demo — hover highlight -->

### Callback

`ui.on_hover()` fires every frame the pointer is over the element. It gives you
the element's `Id` and a `PointerData` with the pointer position:

```rust
ui.element().width(grow!()).height(fixed!(80.0))
    .background_color(0x262220)
    .children(|ui| {
        ui.on_hover(|id, pointer| {
            println!("Hovering {:?} at ({}, {})", id, pointer.position.x, pointer.position.y);
        });

        ui.text("Move your pointer around", |t| t.font_size(14).color(0xE8E0DC));
    });
```

## Press & release

### Inline

`ui.pressed()` returns true while the pointer is held down on the element inside a `.children()` closure:

<!-- The bg colors should be editable -->
```rust
ui.element().width(fit!()).height(fixed!(40.0))
    .children(|ui| {
        let bg = if ui.pressed() {
            0xFF654D
        } else if ui.hovered() {
            0x3A3533
        } else {
            0x262220
        };

        ui.element().width(fit!()).height(grow!())
            .background_color(bg)
            .corner_radius(8.0)
            .layout(|l| l.padding((0, 16, 0, 16)).align(CenterX, CenterY))
            .children(|ui| {
                ui.text("Press me", |t| t.font_size(14).color(0xE8E0DC));
            });
    });
```
<!-- TODO: Embedded WASM demo — press highlight -->

### Callbacks

`.on_press()` fires once when the pointer goes down. `.on_release()` fires once
when it comes back up:

```rust
ui.element().id("delete_btn").width(fit!()).height(fixed!(36.0))
    .on_press(|id| {
        println!("Pressed: {:?}", id);
    })
    .on_release(|id| {
        println!("Released: {:?}", id);
    })
    .children(|ui| {
        let bg = if ui.pressed() { 0xB91414 } else { 0xFF654D };

        ui.element()
            .width(fit!())
            .height(grow!())
            .background_color(bg)
            .corner_radius(6.0)
            .layout(|l| l.padding((0, 12, 0, 12)).align(CenterX, CenterY))
            .children(|ui| {
                ui.text("Delete", |t| t.font_size(14).color(0xFFFFFF));
            });
    });
```
<!-- TODO: Embedded WASM demo — press/release callbacks with visual feedback -->
<!-- Little console for the wasm to print to -->

If you click a child element, all its ancestors with `.on_press()`
callbacks will also fire.

## Focus

### Inline

`ui.focused()` returns true when the element has keyboard focus:

```rust
ui.element().id("input_field").width(fixed!(200.0)).height(fixed!(36.0))
    .corner_radius(6.0)
    .children(|ui| {
        let (bg, border_color) = if ui.focused() {
            (0x3A3533, 0xFFC32C)
        } else {
            (0x262220, 0x4A4440)
        };

        ui.element()
            .width(grow!())
            .height(grow!())
            .background_color(bg)
            .corner_radius(6.0)
            .border(|b| b.all(1).color(border_color))
            .layout(|l| l.padding(8).align(Left, CenterY))
            .children(|ui| {
                ui.text("Focused field", |t| t.font_size(14).color(0xE8E0DC));
            });
    });
```

### Callbacks

`.on_focus()` fires when the element gains focus. `.on_unfocus()` fires when it
loses focus:

```rust
ui.element()
    .id("search")
    .on_focus(|id| {
        println!("Search gained focus: {:?}", id);
    })
    .on_unfocus(|id| {
        println!("Search lost focus: {:?}", id);
    })
    .children(|ui| {
        // ...
    });
```

Focus moves via Tab / Shift+Tab, arrow keys, or programmatically with
`ply.set_focus("search")`.

## Pointer queries

Query pointer state by ID from anywhere:

```rust
// Is the pointer over this element?
if ply.pointer_over("tooltip_trigger") {
    // show tooltip
}

// Is this element being held down?
if ply.is_pressed("submit") {
    // show active state
}

// All elements under the pointer, z-sorted
let hovered = ply.pointer_over_ids();
```

## Preserve focus

Toolbar buttons that modify a text input (bold, italic, etc.) shouldn't steal
focus from the input. Use `.preserve_focus()`:

```rust
fn toolbar_button(ui: &mut Ui, label: &str, action: impl FnMut(Id) + 'static) {
    ui.element().width(fixed!(32.0)).height(fixed!(32.0))
        .preserve_focus()
        .on_press(action)
        .children(|ui| {
            let bg = if ui.pressed() { 0x4A4440 } else { 0x3A3533 };

            ui.element()
                .width(grow!())
                .height(grow!())
                .background_color(bg)
                .corner_radius(4.0)
                .layout(|l| l.align(CenterX, CenterY))
                .children(|ui| {
                    ui.text(label, |t| t.font_size(14).color(0xE8E0DC));
                });
        });
}
```

Without `.preserve_focus()`, clicking the button would unfocus the text input.
With it, the text input keeps focus and the button's `on_press` still fires.

## Building a button

Putting it all together:

<!-- The bg colors, sizing, corner_radius, padding, font_size, and text color should be editable -->
```rust
fn button(ui: &mut Ui, label: &str, on_click: impl FnMut(Id) + 'static) {
    ui.element()
        .width(fit!())
        .height(fixed!(36.0))
        .corner_radius(6.0)
        .on_press(on_click)
        .children(|ui| {
            let bg = if ui.pressed() {
                0xB91414
            } else if ui.hovered() {
                0xFF654D
            } else {
                0x3A3533
            };

            ui.element().width(fit!()).height(grow!())
                .background_color(bg)
                .corner_radius(6.0)
                .layout(|l| l.padding((0, 16, 0, 16)).align(CenterX, CenterY))
                .children(|ui| {
                    ui.text(label, |t| t.font_size(14).color(0xFFFFFF));
                });
        });
}
```

Use it:

```rust
ui.element()
    .width(grow!())
    .height(grow!())
    .layout(|l| l.direction(LeftToRight).gap(8).padding(16).align(Left, Top))
    .children(|ui| {
        button(ui, "Save", |_| { println!("Saved!"); });
        button(ui, "Cancel", |_| { println!("Cancelled!"); });
        button(ui, "Delete", |_| { println!("Deleted!"); });
    });
```
<!-- TODO: Embedded WASM demo — row of buttons with hover/press states -->
<!-- Little console for the wasm to print to -->

## Next steps

→ [Floating Elements](/docs/floating-elements/)
