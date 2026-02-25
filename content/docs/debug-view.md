+++
title = "Debug View"
weight = 3
+++

One line of code gives you Chrome DevTools-style element inspection — built right
into the engine. No other Rust UI library has this.

## Turning it on

```rust
ply.set_debug_mode(true);
```
<!-- You would be able to click on the above "true" to toggle it below --->

That's it. A full element inspector appears on top of your UI, showing every element
in your tree, its sizing, layout, colors, borders, and more.

<!-- TODO: Embedded WASM demo showing debug mode toggled on/off -->

## What you get

### Element tree

A scrollable, indented tree of every element in your UI. Containers have +/- buttons
to collapse or expand their children. Leaf elements are shown with dots.

- **Hover** a row to highlight that element on screen
- **Click** a row to select it and open the detail panel

### Detail panel

When you select an element, the detail panel shows everything about it:

| Section      | What it shows                                               |
|--------------|-------------------------------------------------------------|
| **Layout**   | bounding box, sizing, padding, child gap, alignment         |
| **Color**    | background color                                            |
| **Shape**    | corner radius, shape rotation                               |
| **Text**     | size, ID, line height, spacing, wrap mode, alignment, color |
| **Border**   | widths, color, between-children                             |
| **Floating** | offset, anchor, attachment, z-index                         |
| **Overflow** | clip/scroll per axis                                        |
| **Image**    | source info                                                 |
| **Effects**  | shader infos and visual rotation                            |
| **Input**    | input information                                           |

## Common debugging patterns

### "Why is my element the wrong size?"

Select the element in the tree. The detail panel shows its sizing type.
You'll immediately see if it's being clamped, if the parent is too small, or if a sibling is eating the available space.

<!-- All the sizings in the below should be interactive like in the Elements & Layout -->
```rust
ui.element()
    .width(fit!())
    .height(grow!())
    .background_color(0x1E1B1B)
    .layout(|l| l.direction(LeftToRight))
    .children(|ui| {
        // Sidebar
        ui.element()
            .width(percent!(0.25))
            .height(grow!())
            .background_color(0x262220)
            .layout(|l| l.direction(TopToBottom).gap(8).padding(12))
            .children(|ui| {
                ui.text("Sidebar", |t| t.font_size(16).color(0xFFC32C));
                ui.text("Home", |t| t.font_size(14).color(0xE8E0DC));
                ui.text("Settings", |t| t.font_size(14).color(0xE8E0DC));
            });

        // Content
        ui.element()
            .width(grow!()).height(grow!())
            .layout(|l| l.align(CenterX, CenterY))
            .children(|ui| {
                // This text is crunched into two lines
                ui.text("Why is everything crushed?", |t| t.font_size(24).color(0xFFFFFF));
            });
    });
```
<!-- TODO: Embedded WASM demo — a UI with a sizing bug, debug mode reveals it -->

The parent uses `fit!()`, so it sizes itself to the only thing it knows the size of "Why is everything crushed?" which is 320 px wide. Then the sidebar asks for 25 % of that: 320 × 0.25 = 80 px, so it crushes the text onto two lines. Do you know how to fix this? Edit the sizings above.

### "Where is my element?"

Hover over rows in the tree. Each row highlights the corresponding element's bounding box.
If an element exists in the tree but has no visible highlight, check its dimensions.
If an element is highlighted but invisible, check the clip settings.

<!-- All the "fixed!(41.0)" should be interactive and the "clip" should be a dropdown -->
```rust
ui.element()
    .width(fit!())
    .height(fixed!(41.0))
    .background_color(0x2E2A28)
    .corner_radius(8.0)
    .overflow(|o| o.clip())
    .layout(|l| l.direction(TopToBottom).padding(8).gap(8))
    .children(|ui| {
        for name in ["Alice", "Bob", "Charlie", "Diana", "Eve"] {
            ui.text(name, |t| t.font_size(30).color(0xE8E0DC));
        }
    });
```
<!-- TODO: Embedded WASM demo — a UI with a clipped element -->

All five text elements appear in the element tree. Hovering each one highlights its bounding box. The parent's `.overflow(|o| o.clip())` hides anything that overflows. Do you know how to fix this? Edit the code above.

## Works everywhere

The debug view runs on every platform Ply supports, desktop, web, and mobile.
It's built with Ply elements itself (it uses the engine to inspect the engine).

You can ship it in development builds and toggle it with a keybind:

```rust
if is_key_pressed(KeyCode::F12) {
    let current = ply.is_debug_mode();
    ply.set_debug_mode(!current);
}
```

## Next steps

→ [Text](/docs/text/)
