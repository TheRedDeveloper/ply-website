+++
title = "Floating Elements"
weight = 7
+++

Floating elements break out of normal layout flow. They position
themselves relative to a parent, another element, or the root. Use them
for tooltips, dropdowns, modals, and badges.

## Basic floating

Add `.floating()` with an attach target to pull an element out of the
normal layout flow. You always need to say what the element floats
relative to:

```rust
ui.element().width(fixed!(200.0)).height(fixed!(100.0))
  .background_color(0x2E2A28)
  .children(|ui| {
    ui.text("I'm the parent", |t| t.font_size(14).color(0xE8E0DC));

    ui.element()
      .width(fixed!(80.0))
      .height(fixed!(30.0))
      .background_color(0xB91414)
      .floating(|f| f.attach_parent())
      .children(|ui| {
        ui.text("Float!", |t| t.font_size(12).color(0xFFFFFF));
      });
  });
```
{{ demo(id="basic_float", height=160) }}

## Attach targets

The three attach targets are:

- `.attach_parent()`: float relative to the parent element
- `.attach_root()`: float relative to the entire window
- `.attach_id("some_id")`: float relative to any element by ID

Without an attach target, `.floating(|f| f)` does nothing.

## Anchoring

By default, the element's top-left corner sits at the parent's
top-left corner. Use `.anchor()` to change that. The first tuple is the
element's attachment point, the second is the parent's:

```rust
.floating(|f| f.anchor(
    (CenterX, Top),    // element's attachment point
    (CenterX, Bottom)  // parent's attachment point
))
```

This places the element's top-center at the parent's bottom-center,
which is how you'd position a tooltip below something.

Common anchor combos:

| Element point        | Parent point        | Use case                     |
|----------------------|---------------------|------------------------------|
| `(CenterX, Top)`     | `(CenterX, Bottom)` | Tooltip below, centered      |
| `(CenterX, Bottom)`  | `(CenterX, Top)`    | Tooltip above, centered      |
| `(Left, Top)`        | `(Left, Bottom)`    | Dropdown below, left-aligned |
| `(Left, CenterY)`    | `(Right, CenterY)`  | Popover to the right         |
| `(CenterX, CenterY)` | `(Right, Top)`      | Badge at top-right corner    |

## Offset

Add spacing between the floating element and its anchor point:

```rust
.floating(|f| f
    .anchor((CenterX, Top), (CenterX, Bottom))
    .offset(0.0, 8.0)  // 8px gap below the parent
)
```

## Z-index

Control stacking order with `.z_index()`. Higher values render on top:

```rust
.floating(|f| f.attach_parent().z_index(10))    // dropdown
.floating(|f| f.attach_root().z_index(100))     // modal
.floating(|f| f.attach_root().z_index(200))     // toast notification
```

## Clipping

`.clip_by_parent()` clips the floating element to its parent's bounds.
This is useful for elements that should not overflow outside a
container:

```rust
.floating(|f| f.attach_parent().clip_by_parent())
```

## Pointer passthrough

`.passthrough()` lets clicks pass through the floating element to
whatever is underneath. Use this for visual overlays that shouldn't
block interaction:

```rust
ui.element()
  .width(grow!())
  .height(grow!())
  .background_color((1.0, 1.0, 1.0, 0.1))
  .floating(|f| f.attach_root().passthrough())
  .children(|ui| {
    ui.text("v0.5.0-dev", |t| t.font_size(12).color(0x9E9590));
  });
```
{{ demo(id="passthrough_overlay", height=40) }}

## Examples

### Tooltip on hover

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
{% example(id="tooltip_demo", height=600) %}
ui.element()
  .width(fit!()).height(fit!())
  .background_color(0x2E2A28)
  .corner_radius(8.0)
  .children(|ui| {
    ui.text("[[label]]", |t| t.font_size(14).color(0xE8E0DC));

    if ui.hovered() {
      ui.element()
        .floating(|f| f
          .attach_parent()
          .anchor((CenterX, Top), (CenterX, Bottom))
          .offset(0.0, 4.0)
        )
        .children(|ui| {
          ui.text("[[tooltip]]", |t| t.font_size(12).color(0x9E9590));
        });
    }
  });
---
label = Hover me for info
tooltip = Extra information here
{% end %}

### Notification badge

```rust
ui.element()
  .width(fixed!(280.0))
  .height(fixed!(80.0))
  .background_color(0x2E2A28)
  .corner_radius(12.0)
  .layout(|l| l.padding(16).align(Left, CenterY))
  .children(|ui| {
    ui.text("Messages", |t| t.font_size(16).color(0xFFFFFF));

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
```
{% example(id="notification_badge", height=600) %}
ui.element()
  .width(fixed!(280.0)).height(fixed!(80.0))
  .background_color(0x2E2A28)
  .corner_radius(12.0)
  .children(|ui| {
    ui.text("Messages", |t| t.font_size(16).color(0xFFFFFF));

    ui.element()
      .width(fixed!(24.0)).height(fixed!(24.0))
      .background_color([[badge_color]])
      .corner_radius(12.0)
      .floating(|f| f
        .attach_parent()
        .anchor((CenterX, CenterY), (Right, Top))
      )
      .children(|ui| {
        ui.text("[[count]]", |t| t.font_size(12).color(0xFFFFFF));
      });
  });
---
c.badge_color = 0xB91414
count = 3
{% end %}

### Modal dialog

```rust
if show_modal {
  ui.element()
    .width(grow!())
    .height(grow!())
    .background_color((0.0, 0.0, 0.0, 0.5))
    .floating(|f| f.attach_root().z_index(100))
    .layout(|l| l.align(CenterX, CenterY))
    .children(|ui| {
      ui.element()
        .width(fixed!(400.0))
        .height(fit!())
        .background_color(0x2E2A28)
        .corner_radius(12.0)
        .layout(|l| l.direction(TopToBottom).padding(24).gap(16))
        .children(|ui| {
          ui.text("Delete item?", |t| t.font_size(20).color(0xFFFFFF));
          ui.text("This cannot be undone.", |t| t.font_size(14).color(0x9E9590));

          ui.element()
            .width(grow!())
            .height(fit!())
            .layout(|l| l.direction(LeftToRight).gap(8).align(Right, CenterY))
            .children(|ui| {
              // Let's reuse the same button helper from before
              button(ui, "Cancel", |_| {});
              button(ui, "Delete", |_| {});
            });
        });
    });
}
```
{{ demo(id="modal_dialog", height=200) }}

## Next steps

-> [Images, Vectors & Custom Rendering](/docs/images-and-rendering/)

