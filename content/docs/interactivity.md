+++
title = "Interactivity"
weight = 6
+++

Make your elements respond to clicks, hovers, and keyboard focus.
Ply gives you two levels of control: inline state queries inside
a children closure, and callback-based events on the element builder.

## Inline

Check `ui.pressed()`, `ui.hovered()` and `ui.focused()` inside a `.children()` closure,
we explained this in the last section:

<!-- The bg colors should be editable -->
```rust
ui.element().width(fit!()).height(fixed!(40.0))
    .children(|ui| {
        let bg = if ui.pressed() {
            0xFF654D
        } else if ui.hovered() || ui.focused() {
            0x3A3533
        } else {
            0x262220
        };

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

## Callback

| Method          | Fires                                        |
|-----------------|----------------------------------------------|
| `.on_hover()`   | every frame the pointer is over the element. |
| `.on_press()`   | once when the pointer goes down.             |
| `.on_release()` | once when the pointer comes back up.         |
| `.on_focus()`   | once when the element gets focused.          |
| `.on_unfocus()` | once when the element gets unfocused.        |

It gives you the element's `Id` and (for the pointer ones) a `PointerData` with the pointer position:

```rust
ui.element().width(grow!()).height(fixed!(80.0))
    .background_color(0x262220)
    .on_hover(|id, pointer| {
        println!("Hovering {:?} at ({}, {})", id, pointer.position.x, pointer.position.y);
    })
    .on_press(|id, pointer| {
        println!("Pressed {:?} at ({}, {})", id, pointer.position.x, pointer.position.y);
    })
    .on_release(|id, pointer| {
        println!("Released {:?} at ({}, {})", id, pointer.position.x, pointer.position.y);
    })
    .on_focus(|id| {
        println!("Focused {:?}", id);
    })
    .on_unfocus(|id| {
        println!("Unfocused {:?}", id);
    })
    .children(|ui| {
        ui.text("Move your pointer around", |t| t.font_size(14).color(0xE8E0DC));
    });
```
<!-- TODO: Embedded WASM demo — reacts to cursor and tabbing -->

## Pointer queries

As mentioned in [Interactivity](/docs/interactivity) you can query state by ID from anywhere:

```rust
// All elements under the pointer, z-sorted
let hovered = ply.pointer_over_ids();

let mut ui = ply.begin();

if ui.is_pressed("submit") {
    // add active state "submit" element
} else {
    // add inactive state "submit" element
}

if ui.pointer_over("tooltip_trigger") {
    // show tooltip
}
```

## Preserve focus

Toolbar buttons that modify a text input shouldn't steal
focus from the input. Use `.preserve_focus()`:

```rust
ui.element().width(fixed!(32.0)).height(fixed!(32.0))
    .preserve_focus()
    .on_press(TODO)
    .children(|ui| {
        ui.text(Make red, |t| t.font_size(14).color(0xFFFFFF));
    });
```
<!-- TODO: Interactive WASM example – Text box with make red button --->

Without `.preserve_focus()`, clicking the button would unfocus the text input,
then the {color=red| wouldn't get applied.

## Building a button

Putting it all together:

<!-- The bg colors, sizing, corner_radius, padding, font_size, and text color should be editable -->
<!-- TODO: Make a better button -->
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
            } else if ui.hovered() || ui.focused() {
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
