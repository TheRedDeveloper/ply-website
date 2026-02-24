+++
title = "Rotation"
weight = 12
+++

Ply offers two rotation systems. Visual rotation renders the element
and its children to an offscreen buffer then draws it back rotated,
layout is unaffected. Shape rotation rotates the element's own
geometry at the vertex level and adjusts its bounding box, children and
text are unaffected.

## Visual rotation

Rotate an element and everything inside it, purely visual:

<!-- editable degrees -->
```rust
ui.element()
    .width(fixed!(120.0))
    .height(fixed!(80.0))
    .background_color(0xFFC32C)
    .corner_radius(8.0)
    .rotate_visual(|r| r.degrees(15.0))
    .children(|ui| {
        ui.text("Tilted!", |t| t.font_size(14).color(0x181515));
    });
```
<!-- TODO: Embedded WASM demo — visual rotation -->

The layout stays axis-aligned, surrounding elements don't see the
rotation.

### Pivot

By default, rotation is around the center (0.5, 0.5). Change it with
`.pivot()`:

```rust
.rotate_visual(|r| r
    .degrees(30.0)
    .pivot(0.0, 0.0)
)
```

Pivot coordinates are normalized: (0, 0) = top-left, (1, 1) = bottom-right.

### Flipping

Mirror horizontally or vertically:

```rust
.rotate_visual(|r| r.flip_x())

.rotate_visual(|r| r.flip_y())

.rotate_visual(|r| r.degrees(45.0).flip_x())
```

### Combining with shaders

Visual rotation shares the same offscreen render target as `.shader()`.
When you use both, there's no extra GPU cost:

```rust
ui.element()
    .shader(&FOIL, |s| s.uniform("u_time", get_time() as f32))
    .rotate_visual(|r| r.degrees(5.0))
    .children(|ui| {
        ui.text("Shiny + tilted", |t| t.font_size(16).color(0xFFFFFF));
    });
```

### Reference

| Method          | What it does              | Default    |
|-----------------|---------------------------|------------|
| `.degrees(f32)` | Rotation angle in degrees | 0          |
| `.radians(f32)` | Rotation angle in radians | 0          |
| `.pivot(x, y)`  | Rotation center (0.0–1.0) | (0.5, 0.5) |
| `.flip_x()`     | Mirror horizontally       | false      |
| `.flip_y()`     | Mirror vertically         | false      |

## Shape rotation

Rotates the element's own geometry (background, border, image) at the
vertex level. The bounding box adjusts to fit the rotated shape (AABB):

<!-- editable degrees -->
```rust
ui.element()
    .width(fixed!(80.0))
    .height(fixed!(80.0))
    .background_color(0xFF654D)
    .rotate_shape(|r| r.degrees(45.0))
    .empty();
```
<!-- TODO: Embedded WASM demo — shape rotation diamond -->

What shape rotation does not affect:
- Children
- Text
- Shaders

There is no pivot, because pivots actually only move elements
and `.rotate_shape()` stays within the layout.

### Reference

| Method          | What it does              | Default |
|-----------------|---------------------------|---------|
| `.degrees(f32)` | Rotation angle in degrees | 0       |
| `.radians(f32)` | Rotation angle in radians | 0       |
| `.flip_x()`     | Mirror horizontally       | false   |
| `.flip_y()`     | Mirror vertically         | false   |

## Next steps

→ [Accessibility](/docs/accessibility/)
