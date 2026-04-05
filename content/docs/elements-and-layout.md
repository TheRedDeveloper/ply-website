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
- `fit!(100.0)`/`fit!(min: 100.0)`: fit content, but don't shrink below 100px
- `grow!(100.0, 500.0)`/`grow!(min: 100.0, max: 500.0)`: grow within 100-500px range

`grow!` supports a third weight parameter for proportional sizing among siblings. `grow!(100.0, 500.0, 2.0)`/`grow!(min: 100.0, max: 500.0, weight: 2.0)`: grow within 100-500px, and take twice as much space as siblings with weight 1.0

{% example(id="layout_card", height=500) %}
ui.element().width(grow!()).height(grow!())
  .background_color([[bg]])
  .layout(|l| l
    .direction([[direction]])
    .gap([[gap]])
    .padding([[padding]])
    .align([[align_x]], [[align_y]])
  )
  .children(|ui| {
    ui.text("A", |t| t.font_size(24));
    ui.text("B", |t| t.font_size(24));
    ui.text("C", |t| t.font_size(24));
  });
---
c.bg = 0x262220
e.direction = LeftToRight ; TopToBottom
n.gap = 12
padding = 16
e.align_x = CenterX ; Left ; Right
e.align_y = CenterY ; Top ; Bottom
{% end %}

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

Configure per-side border widths, color, child dividers, and border position:

{% example(id="borders_demo", height=600) %}
ui.element().width(fit!()).height(fit!())
  .background_color(0x262220)
  .corner_radius(12.0)
  .border(|b| b
    .left([[left]])
    .right([[right]])
    .top([[top]])
    .bottom([[bottom]])
    .between_children([[between_children]])
    .color([[border_color]])
    .position([[position]])
  )
  .layout(|l| l.padding(12).gap(20))
  .children(|ui| {
    ui.text("A", |t| t.font_size(24));
    ui.text("B", |t| t.font_size(24));
    ui.text("C", |t| t.font_size(24));
  });
---
n.left = 2
n.right = 2
n.top = 2
n.bottom = 2
n.between_children = 1
c.border_color = 0x4A4440
e.position = Outside ; Middle ; Inside
{% end %}

`.all(2)` = `.left(2).right(2).top(2).bottom(2)`. `.between_children()` is separate.

### Border position

Choose where the border is drawn:

| Value     | Behavior                                  |
|-----------|-------------------------------------------|
| `Outside` | Border is fully outside (default)         |
| `Middle`  | Border is centered on the edge            |
| `Inside`  | Border is fully inside the element bounds |

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

// Disable mouse drag-scrolling, keep touch + wheel
.overflow(|o| o.scroll_y().no_drag_scroll())

// Enable a scrollbar
.overflow(|o| o.scroll_y().scrollbar(|s| {
  s.width(4.0)
   .corner_radius(2.0)
   .thumb_color(0xFFFFFF)
   .track_color(0x141414)
   .min_thumb_size(18.0)
   .hide_after_frames(60)
}))
```

Clipping hides any content that extends beyond the element's bounds. Scrolling
enables drag/wheel interaction so the user can pan through the overflow.

## Aspect ratio

Lock an element to a specific aspect ratio:

```rust
.aspect_ratio(16.0 / 9.0)

.contain(16.0 / 9.0)

.cover(16.0 / 9.0)
```

With `aspect_ratio()`, the engine fills in whichever dimension is unset. If both height and width are explicitly sized, aspect ratio will use contain behavior. `contain()` and `cover()` work like in CSS.

## Passing Ui into functions

Pass Ui to functions for modular UI:

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

{% example(id="sidebar_nav", height=495, base_height=300, height_per_item=65, list_param="items") %}
ui.element()
  .width(fixed!(200.0))
  .height(grow!())
  .background_color([[bg]])
  .layout(|l| l.direction(TopToBottom).gap(4).padding(8))
  .children(|ui| {
[[items:    nav_item(ui, "$", false);]]
  });
---
c.bg = 0x181515
l.items = Home|Settings|About
{% end %}

## Next steps

→ [Debug View](/docs/debug-view/)
