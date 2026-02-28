+++
title = "Text"
weight = 4
+++

Text is a special element in Ply. Unlike containers that you build with `ui.element()`,
text is created with its own `ui.text()` call and configured through a dedicated closure.

## Creating text

```rust
ui.text("Hello, Ply!", |t| t
  .font_size(24)
  .color(0xFFFFFF)
);
```

The first argument is the string, the second is a closure that configures the text.

## Configuration

Here's everything you can set on a text element:

| Method                 | What it does                             | Default      |
|------------------------|------------------------------------------|--------------|
| `.font_size(24)`       | Size in pixels                           | `0`          |
| `.color(0xFFFFFF)`     | Text color (same formats as backgrounds) | transparent  |
| `.font(&HEADING_FONT)` | Which font to use                        | default font |
| `.letter_spacing(2)`   | Extra space between characters in px     | `0`          |
| `.line_height(28)`     | Height of each line in pixels            | auto         |
| `.wrap_mode(Words)`    | How text wraps                           | `Words`      |
| `.alignment(Left)`     | Horizontal text alignment                | `Left`       |

## Font size and color

{% example(id="text_color_demo", height=350) %}
ui.text("[[label1]]", |t| t
    .font_size([[font_size1]])
    .color([[color1]])
);

ui.text("[[label2]]", |t| t
    .font_size([[font_size2]])
    .color([[color2]])
);
---
label1=Big and red
n.font_size1=32
c.color1=0xB91414
label2=Small and gold
n.font_size2=12
c.color2=0xFFC32C
{% end %}

Color accepts the same formats as `.background_color()`: hex integers,
RGB float tuples, or RGBA float tuples.

## Fonts

Define your fonts as `static FontAsset` constants. Use `FontAsset::Path` for file paths or `FontAsset::Bytes` for embedded fonts:

```rust
static BODY_FONT: FontAsset = FontAsset::Path("assets/fonts/inter.ttf");
static CODE_FONT: FontAsset = FontAsset::Path("assets/fonts/firacode.ttf");

// Or embed the font bytes directly
static EMBEDDED: FontAsset = FontAsset::Bytes {
    file_name: "inter.ttf",
    data: include_bytes!("../assets/fonts/inter.ttf"),
};
```

The default font is the one you pass to `Ply::new`. Use `.font()` to pick a different font:

```rust
ui.text("Monospaced", |t| t.font(&CODE_FONT).font_size(16).color(0xE8E0DC));
```

Fonts are loaded automatically and cached. Unused fonts are evicted after 60 frames.

## Wrapping

Text wraps by default. Three wrap modes are available:

| Mode                | Behavior                              |
|---------------------|---------------------------------------|
| `WrapMode::Words`   | Wraps at word boundaries (default)    |
| `WrapMode::Newline` | Only wraps at `\n` newline characters |
| `WrapMode::None`    | Never wraps, will overflow the parent |


{% example(id="text_wrap_demo", height=350) %}
ui.element()
    .width(fixed!([[width]]))
    .children(|ui| {
        ui.text("This text will wrap at word boundaries when it runs out of space.", |t| t
            .font_size(14)
            .color(0xE8E0DC)
            .wrap_mode([[wrap_mode]])
        );
    });
---
n.width=200
e.wrap_mode=Words;Newline;None
{% end %}

## Alignment

Text alignment controls where text sits within its available width:

| Value     | Effect                 |
|-----------|------------------------|
| `Left`    | Left-aligned (default) |
| `CenterX` | Centered               |
| `Right`   | Right-aligned          |

{% example(id="text_align_demo", height=300) %}
ui.text("Aligned heading", |t| t
    .font_size([[font_size]])
    .color(0xFFFFFF)
    .alignment([[alignment]])
);
---
e.alignment=Left;CenterX;Right
n.font_size=24
{% end %}

## Letter spacing and line height

Fine-tune text with `.letter_spacing()` and `.line_height()`:

```rust
ui.text("Spaced out", |t| t
  .font_size(16)
  .color(0xE8E0DC)
  .letter_spacing(4)  // 4px extra between characters
  .line_height(28)    // 28px per line
);
```

Both values are in pixels. A `line_height` of `0` lets the engine use the font's
natural line height.

## Accessibility

Mark text as accessible so screen readers can announce it:

```rust
ui.text("Welcome back, Alice", |t| t
  .font_size(18)
  .color(0xE8E0DC)
  .accessible()
);
```

Without `.accessible()`, the text is visual only. Maybe you don't
want screen readers announcing every decorative label.

## Next steps

→ [IDs & State](/docs/ids-and-state/)
