+++
title = "Elements & Layout"
weight = 2
+++

Everything in Ply is an element. This single building block
handles sizing, layout, color, borders, overflow, and nesting.

## The element builder

Create an element with `ui.element()`, configure it with chained methods, then
finalize with `.children()` or `.empty()`:

```rust
ui.element()
    .width(grow!())
    .height(fixed!(200.0))
    .background_color(0x262220)
    .children(|ui| {
        ui.text("Hello!", |t| t.font_size(24).color(0xFFFFFF));
    });
```

## Sizing

Every element has a width and height, set with four sizing macros:

| Macro           | Behavior                 |
|-----------------|--------------------------|
| `grow!()`       | Fill all available space |
| `fit!()`        | Shrink to fit content    |
| `fixed!(200.0)` | Exactly 200 pixels       |
| `percent!(0.5)` | 50% of parent's size     |

`grow!` and `fit!` accept optional min/max bounds:
- `fit!(100.0)`: fit content, but don't shrink below 100px
- `grow!(100.0, 500.0)`: grow within 100-500px range

<!-- The width, height, background_color, corner_radius, direction, gap, padding, and alignment should all be editable -->
```rust
ui.element()
    .width(grow!())
    .height(fixed!(200.0))
    .background_color(0x262220)
    .corner_radius(8.0)
    .layout(|l| l
        .direction(LeftToRight)
        .gap(12)
        .padding(16)
        .align(CenterX, CenterY)
    )
    .children(|ui| {
        ui.text("A", |t| t.font_size(24).color(0xFFFFFF));
        ui.text("B", |t| t.font_size(24).color(0xFFFFFF));
        ui.text("C", |t| t.font_size(24).color(0xFFFFFF));
    });
```
<!-- TODO: Embedded WASM demo — interactive element with editable sizing, layout, and color properties -->

## Layout

The `.layout()` closure configures how children are arranged:

```rust
.layout(|l| l
    .direction(LeftToRight)   // or TopToBottom (default)
    .gap(16)                  // pixels between children
    .padding(12)              // inner padding on all sides
    .align(CenterX, CenterY)  // child alignment
)
```

### Direction

| Value         | Behavior                             |
|---------------|--------------------------------------|
| `LeftToRight` | Children flow horizontally (default) |
| `TopToBottom` | Children stack vertically            |

### Alignment

Alignment has two axes:

| X axis    | Y axis    |
|-----------|-----------|
| `Left`    | `Top`     |
| `CenterX` | `CenterY` |
| `Right`   | `Bottom`  |

The alignment positions children within the available space.

### Padding

Padding adds space inside the element, between its border and its children:

```rust
// Same on all sides
.layout(|l| l.padding(16))

// Per-side: (top, right, bottom, left), CSS order
.layout(|l| l.padding((12, 16, 12, 16)))
```

## Background color

```rust
.background_color(0xFF654D)              // hex integer
.background_color((1.0, 0.4, 0.3))       // RGB floats
.background_color((1.0, 0.4, 0.3, 1.0))  // RGBA floats
```

## Corner radius

Round the corners with a single value or per-corner tuple:

```rust
// All corners: 12px
.corner_radius(12.0)

// Per-corner: (top-left, top-right, bottom-right, bottom-left), CSS order
.corner_radius((12.0, 12.0, 0.0, 0.0))
```

## Borders

Configure per-side border widths, color, and child dividers:

```rust
.border(|b| b
    .all(2)                  // 2px on all sides
    .color(0x4A4440)         // border color
    .between_children(1)     // 1px dividers between children
)
```

Individual sides: `.left(2)`, `.right(2)`, `.top(2)`, `.bottom(2)`.

## Overflow

By default, children can overflow their parent. Use `.overflow()` to control clipping
and scrolling:

```rust
// Clip both axes (no scrolling)
.overflow(|o| o.clip())

// Clip horizontal only
.overflow(|o| o.clip_x())

// Scroll vertically (implies clip on that axis)
.overflow(|o| o.scroll_y())

// Scroll both axes
.overflow(|o| o.scroll())
```

Clipping hides any content that extends beyond the element's bounds. Scrolling
enables drag/wheel interaction so the user can pan through the overflow.

## Aspect ratio

Lock an element to a specific aspect ratio:

```rust
.aspect_ratio(16.0 / 9.0)
```

The engine fills in whichever dimension is unset. If both height and width are explicitly sized, aspect ratio has no effect.

## Passing Ui into functions

Pass Ui to functions for modular UI:

<!-- The bg colors, sizing, corner_radius, padding, alignment, font_size, and text color should be editable -->
```rust
fn nav_item(ui: &mut Ui, label: &str, active: bool) {
    let bg = if active { 0x3A3533 } else { 0x262220 };
    ui.element()
        .width(grow!())
        .height(fixed!(36.0))
        .background_color(bg)
        .corner_radius(6.0)
        .layout(|l| l.padding(8).align(Left, CenterY))
        .children(|ui| {
            ui.text(label, |t| t.font_size(14).color(0xE8E0DC));
        });
}
```

Then call it wherever you need it:

<!-- The width, background_color, direction, gap, padding, labels, and active states should be editable, there should be a "-" next to the nav_item calls so you can remove them and a "+" below the nav_item calls so you can add new ones -->
```rust
ui.element()
    .width(fixed!(200.0))
    .height(grow!())
    .background_color(0x181515)
    .layout(|l| l.direction(TopToBottom).gap(4).padding(8))
    .children(|ui| {
        nav_item(ui, "Home", true);
        nav_item(ui, "Settings", false);
        nav_item(ui, "About", false);
    });
```

<!-- TODO: Embedded WASM demo — sidebar with reusable nav_item function -->

## Next steps

→ [Debug View](/docs/debug-view/)
