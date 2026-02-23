+++
title = "Floating Elements"
weight = 7
+++

Floating elements break out of normal layout flow. They position themselves
relative to a parent, another element, or the root. This is perfect for tooltips, dropdowns, modals, and badges.

## Basic floating

Add `.floating()` to make an element float relative to where it would normally be:

```rust
MISSING EXAMPLE
```
<!-- TODO: Embedded WASM demo — floating badge -->

The floating element doesn't affect the layout of its siblings.

## Anchoring

Anchor points control where the floating element attaches. Each anchor is a pair
of `(AlignX, AlignY)` — one for the element, one for the parent:

```rust
.floating(|f| f.anchor(
    (CenterX, Top),    // element's attachment point
    (CenterX, Bottom)  // parent's attachment point
))
```

This places the element's top-center at the parent's bottom-center.

### Badge example

<!-- The anchor points should be editable -->
```rust
ui.element().width(grow!()).height(grow!()).layout(|l| l.align(CenterX, CenterY))
    .children(|ui| {
        ui.element()
            .width(fixed!(280.0))
            .height(fixed!(160.0))
            .background_color(0x2E2A28)
            .corner_radius(12.0)
            .layout(|l| l.direction(TopToBottom).padding(16).gap(8))
            .children(|ui| {
                ui.text("Notification Card", |t| t.font_size(18).color(0xFFFFFF));
                ui.text("You have 3 new messages", |t| t.font_size(14).color(0x9E9590));

                // Badge at top-right
                ui.element()
                    .width(fixed!(24.0))
                    .height(fixed!(24.0))
                    .background_color(0xB91414)
                    .corner_radius(12.0)
                    .floating(|f| f
                        .attach_parent()
                        .anchor((CenterX, CenterY), (Right, Top))
                    )
                    .layout(|l| l.align(CenterX, CenterY))
                    .children(|ui| {
                        ui.text("3", |t| t.font_size(12).color(0xFFFFFF));
                    });
            });
    });
```
<!-- TODO: Embedded WASM demo — tooltip on hover -->

### Tooltip example

<!-- The offset and anchor points should be editable -->
```rust
ui.element()
    .width(fit!())
    .height(fixed!(36.0))
    .background_color(0x3A3533)
    .corner_radius(6.0)
    .layout(|l| l.padding((0, 12, 0, 12)).align(CenterX, CenterY))
    .children(|ui| {
        ui.text("Hover for tooltip", |t| t.font_size(14).color(0xE8E0DC));

        if ui.hovered() {
            ui.element()
                .width(fit!())
                .height(fit!())
                .background_color(0x1E1B1B)
                .corner_radius(4.0)
                .floating(|f| f
                    .attach_parent()
                    .anchor((CenterX, Top), (CenterX, Bottom))
                    .offset(0.0, 4.0)
                )
                .layout(|l| l.padding(8))
                .children(|ui| {
                    ui.text("Extra information here", |t| t.font_size(12).color(0x9E9590));
                });
        }
    });
```
<!-- TODO: Embedded WASM demo — tooltip on hover -->

### Anchor examples

| Element point        | Parent point        | Result                       |
|----------------------|---------------------|------------------------------|
| `(CenterX, Top)`     | `(CenterX, Bottom)` | Tooltip below, centered      |
| `(CenterX, Bottom)`  | `(CenterX, Top)`    | Tooltip above, centered      |
| `(Left, Top)`        | `(Left, Bottom)`    | Dropdown below, left-aligned |
| `(Left, CenterY)`    | `(Right, CenterY)`  | Popover to the right         |
| `(CenterX, CenterY)` | `(Right, Top)`      | Badge at top-right corner    |
| ...                  | ...                 | ...                          |

## Attach targets

By default, floating elements attach to their parent. Use `.attach_root()` for fullscreen
overlays or `.attach_id(id)` to anchor to any element:

```rust
// Attach to root — fullscreen modal backdrop
.floating(|f| f.attach_root().z_index(100))

// Attach to another element by ID
.floating(|f| f.attach_id("target_element"))
```

### Modal example

```rust
// Modal backdrop — fullscreen, attached to root
ui.element()
    .width(grow!())
    .height(grow!())
    .background_color((0.0, 0.0, 0.0, 0.5))
    .floating(|f| f.attach_root().z_index(100))
    .layout(|l| l.align(CenterX, CenterY))
    .children(|ui| {
        // Modal card
        ui.element()
            .width(fixed!(400.0))
            .height(fit!())
            .background_color(0x2E2A28)
            .corner_radius(12.0)
            .layout(|l| l.direction(TopToBottom).padding(24).gap(16))
            .children(|ui| {
                ui.text("Delete item?", |t| t.font_size(20).color(0xFFFFFF));
                ui.text("This action cannot be undone.", |t| t.font_size(14).color(0x9E9590));

                ui.element()
                    .width(grow!())
                    .height(fit!())
                    .layout(|l| l.direction(LeftToRight).gap(8).align(Right, CenterY))
                    .children(|ui| {
                        button(ui, "Cancel", |_| {});
                        button(ui, "Delete", |_| {});
                    });
            });
    });
```
<!-- TODO: Embedded WASM demo — modal dialog -->

## Z-index

Control stacking order with `.z_index()`. Higher values render on top:

```rust
// Dropdown: above normal content
.floating(|f| f.attach_parent().z_index(10))

// Modal: above everything
.floating(|f| f.attach_root().z_index(100))

// Toast notification: above modals
.floating(|f| f.attach_root().z_index(200))
```

## Clipping

`.clip_by_parent()` clips the floating element to its parent's bounds.

```rust
MISSING EXAMPLE
```

## Pointer passthrough

`.passthrough()` makes clicks pass through the floating element to whatever is
below it. Good for visual overlays that shouldn't block interaction:

```rust
// Non-interactive overlay
ui.element()
    .width(grow!())
    .height(fixed!(40.0))
    .background_color((1.0, 1.0, 1.0, 0.1))
    .floating(|f| f.attach_root().passthrough())
    .children(|ui| {
        ui.text("v0.5.0-dev", |t| t.font_size(12).color(0x9E9590));
    });
```

## Explicit dimensions

Force a floating element's size with `.dimensions()`:

```rust
.floating(|f| f
    .attach_parent()
    .anchor((Left, Top), (Left, Bottom))
    .dimensions(Dimensions::new(200.0, 300.0))
)
```

This overrides the element's normal sizing. Useful for dropdown menus that need
a fixed width regardless of content.

## Next steps

→ [Images & Custom Rendering](/docs/images-and-rendering/)

