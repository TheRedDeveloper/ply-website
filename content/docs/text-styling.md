+++
title = "Text Styling"
weight = 10
+++

Ply has a built-in rich text system that lets you color, animate, and
transform individual characters.

**Requires the `text-styling` feature flag:**

```toml
[dependencies]
ply-engine = { version = "1.0", features = ["text-styling"] }
```

## Syntax

Wrap styled text in `{tag|content}`. The tag goes before the pipe, the
content after:

<!-- TODO: editable string -->
```rust
ui.text("{color=red|This text is red}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

Use underscores to chain parameters:

<!-- TODO: editable string -->
```rust
ui.text("{wave_a=0.5_f=1.0|Wavy text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

Tags can nest. Inner tags override outer ones if they conflict:

<!-- editable string -->
```rust
ui.text("{color=#FFC32C|Hello {color=#FF654D|world}!}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

**Escaping:** Use `\` to insert literal `{`, `}`, `|`, or `\`:

<!-- editable string -->
```rust
ui.text(r"Price: \{5\}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

## Properties

Static attributes on the wrapped text. These are optimized in rendering.

### color

Sets the text fill color. Accepts hex (`#RRGGBB`), RGB tuples
(`(r,g,b)`), or named colors.

<!-- TODO: editable strings -->
```rust
ui.text("{color=#FF654D|Salmon text}", |t| t.font_size(24));

ui.text("{color=cyan|Cyan text}", |t| t.font_size(24));

ui.text("{color=(255,195,44)|Yellow text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

Named colors: `white`, `black`, `lightgray`, `darkgray`, `red`, `orange`, `yellow`,
`lime`, `green`, `cyan`, `lightblue`, `blue`, `purple`, `magenta`, `brown`, `pink` (case
insensitive).

### opacity

Makes text semi-transparent:

<!-- TODO: editable string -->
```rust
ui.text("{opacity=0.5|Ghost text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

Properties combine, nest opacity inside a color tag:

<!-- TODO: editable string -->
```rust
ui.text("{color=red|{opacity=0.5|Faded red}}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

## Effects

Per-character visual effects that create movement, shadows, or gradients.

### wave

Vertical sine wave:

<!-- TODO: editable string -->
```rust
ui.text("{wave|Bouncing text}", |t| t.font_size(18));
```
<!-- TODO: small wasm preview -->


| Parameter | What it does                       | Default |
|-----------|------------------------------------|---------|
| `w`       | Wavelength in characters           | 3       |
| `f`       | Frequency (cycles/sec)             | 0.5     |
| `s`       | Speed (chars/sec) — overrides `f`  | —       |
| `a`       | Amplitude (ratio of font size)     | 0.3     |
| `p`       | Phase offset (0–1)                 | 0       |
| `r`       | Direction rotation in degrees      | 0       |

<!-- TODO: editable string -->
```rust
ui.text("{wave_w=5_a=0.6_f=0.2_r=45|Diagonal wave}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

### pulse

Characters grow and shrink in a wave:

<!-- TODO: editable string -->
```rust
ui.text("{pulse|Pulsing text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

| Parameter | What it does                       | Default |
|-----------|------------------------------------|---------|
| `w`       | Wavelength in characters           | 2       |
| `f`       | Frequency (cycles/sec)             | 0.6     |
| `s`       | Speed (chars/sec) — overrides `f`  | —       |
| `a`       | Scale amplitude                    | 0.15    |
| `p`       | Phase offset (0–1)                 | 0       |

### swing

Pendulum rotation per character:

<!-- TODO: editable string -->
```rust
ui.text("{swing_a=12|Swinging text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

| Parameter | What it does                       | Default |
|-----------|------------------------------------|---------|
| `w`       | Wavelength in characters           | 3       |
| `f`       | Frequency (swings/sec)             | 0.5     |
| `s`       | Speed (chars/sec) — overrides `f`  | —       |
| `a`       | Amplitude in degrees               | 8       |
| `p`       | Phase offset (0–1)                 | 0       |

### jitter

Random character displacement:

<!-- TODO: editable string -->
```rust
ui.text("{jitter_radii=0.2,0.2|Shaky text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

| Parameter  | What it does                             | Default   |
|------------|------------------------------------------|-----------|
| `radii`    | Horizontal,vertical offset (font ratio)  | 0.5,0.5   |
| `rotation` | Rotation of the jitter ellipse (degrees) | 0         |

### gradient

Cycling color gradient across characters:

<!-- TODO: editable string -->
```rust
ui.text("{gradient|Rainbow text}", |t| t.font_size(24));
```
<!-- TODO: Embedded WASM demo — gradient text -->

The default is a full rainbow. Custom stops use `position:color` pairs:

```rust
ui.text("{gradient_stops=0:#FF0000,5:#FFC32C_speed=2|Fire text}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

| Parameter | What it does                    | Default |
|-----------|---------------------------------|---------|
| `stops`   | Comma-separated `pos:color`     | rainbow |
| `speed`   | Scroll speed (chars/sec)        | 1       |

### shadow

Draws a duplicate behind each character:

<!-- TODO: editable string -->
```rust
ui.text("{shadow_color=#000000_offset=-0.1,0.1|Shadowed}", |t| t.font_size(24));
```

| Parameter | What it does            | Default   |
|-----------|-------------------------|-----------|
| `color`   | Shadow color            | black     |
| `offset`  | X,Y offset (font ratio) | -0.3,0.3 |
| `scale`   | Shadow size multiplier  | 1.0       |

### transform

Static per-character transform:

<!-- TODO: editable string -->
```rust
ui.text("{transform_translate=0,0.5|Shifted down}", |t| t.font_size(24));
ui.text("{transform_scale=1.5|Giant text}", |t| t.font_size(24));
ui.text("{transform_rotate=15|Tilted}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->

| Parameter   | What it does                    | Default |
|-------------|---------------------------------|---------|
| `translate` | X,Y offset (font size ratio)   | 0,0     |
| `scale`     | X,Y size multiplier             | 1.0     |
| `rotate`    | Rotation in degrees             | 0       |

### hide

Prevents rendering entirely. Useful for reserving space or with
animations:

<!-- TODO: editable string -->
```rust
ui.text("{hide|invisible}", |t| t.font_size(18));
```
<!-- TODO: small wasm preview -->

## Animations

Time-based transitions tracked by a unique `id`. Every animation needs
either `in` (appear) or `out` (disappear).

### type

Typewriter reveal:

<!-- TODO: editable string -->
```rust
ui.text("{type_in_id=intro_speed=12|Hello, world!}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->
<!-- TODO: restart button -->

| Parameter | What it does                       | Default |
|-----------|------------------------------------|---------|
| `in`/`out`| Direction (required)               | —       |
| `id`      | Unique identifier (required)       | —       |
| `speed`   | Characters per second              | 8       |
| `delay`   | Delay before starting (seconds)    | 0       |
| `cursor`  | Character to show as cursor        | none    |

Show a blinking cursor while typing:

<!-- TODO: editable string -->
```rust
ui.text("{type_in_id=msg_speed=10_cursor=\\||Loading...}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->
<!-- TODO: restart button -->

### fade

Opacity transition, character by character:

<!-- TODO: editable string -->
```rust
ui.text("{fade_in_id=appear_speed=5|Fading in}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->
<!-- TODO: restart button -->

| Parameter | What it does                       | Default |
|-----------|------------------------------------|---------|
| `in`/`out`| Direction (required)               | —       |
| `id`      | Unique identifier (required)       | —       |
| `speed`   | Characters per second              | 3       |
| `trail`   | Gradient length in characters      | 3       |
| `delay`   | Delay before starting (seconds)    | 0       |

### scale

Pop-in or pop-out by scaling each character:

<!-- TODO: editable string -->
```rust
ui.text("{scale_in_id=pop_speed=4_trail=5|Popping in!}", |t| t.font_size(24));
```
<!-- TODO: small wasm preview -->
<!-- TODO: restart button -->

| Parameter | What it does                       | Default |
|-----------|------------------------------------|---------|
| `in`/`out`| Direction (required)               | —       |
| `id`      | Unique identifier (required)       | —       |
| `speed`   | Characters per second              | 3       |
| `trail`   | Gradient length in characters      | 3       |
| `delay`   | Delay before starting (seconds)    | 0       |

## Combining styles

Stack tags to combine effects. Effects compose: transforms accumulate,
colors override:

<!-- TODO: editable string -->
```rust
ui.text(
    "{type_in_id=hero_speed=15|{wave_a=0.4|{gradient|Welcome to Ply!}}}",
    |t| t.font_size(24)
);
```
<!-- TODO: small wasm preview -->
<!-- TODO: restart button -->

<!-- TODO: editable string -->
```rust
ui.text(
    "{color=#B91414|{pulse_a=0.2|{shadow_color=#000000|Alert!}}}",
    |t| t.font_size(24)
);
```
<!-- TODO: small wasm preview -->
<!-- TODO: restart button -->

## Styled text input

The text styling syntax works inside text inputs too. When `text-styling`
is enabled, you can add styles and have the user interact with and see it rendered live.

Use `.no_styles_movement()` so the cursor skips over style tag boundaries, this is useful when you are highlighting code:

```rust
ui.element()
    .id("styled_editor")
    .width(grow!())
    .height(fixed!(200.0))
    .background_color(0x1A1A28)
    .corner_radius(6.0)
    .text_input(|t| t
        .multiline(true)
        .font_size(14)
        .text_color(0xDDDDDD)
        .no_styles_movement()
    )
    .empty();
```

### Live highlighting

Build a highlighter that converts plain text to styled text, then apply
it on every frame. Use the `styling_cursor` module:

```rust
use ply_engine::text_input::styling_cursor;

fn highlight(plain: &str) -> String {
    plain.split(' ').map(|word| {
        if word.starts_with('#') {
            format!("{{color=#FFC32C|{}}}", styling_cursor::escape_str(word))
        } else {
            styling_cursor::escape_str(word)
        }
    }).collect::<Vec<_>>().join(" ")
}
```

Apply each frame:

```rust
let raw = ply.get_text_value("styled_editor").to_string();
if !raw.is_empty() {
    let plain = styling_cursor::strip_styling(&raw);
    let highlighted = highlight(&plain);

    if raw != highlighted {
        let cursor = ply.get_cursor_pos("styled_editor");
        let content_pos = styling_cursor::visual_to_content_pos(&raw, cursor);
        ply.set_text_value("styled_editor", &highlighted);
        let new_cursor = styling_cursor::content_to_visual_pos(&highlighted, content_pos);
        ply.set_cursor_pos("styled_editor", new_cursor);
    }
}
```

### styling_cursor functions

| Function                                         | What it does                                          |
|--------------------------------------------------|-------------------------------------------------------|
| `escape_str(s)`                                  | Escapes all style delimiters in a string              |
| `strip_styling(s)`                               | Removes all style tags, returning plain content       |
| `visual_len(s)`                                  | Length in visual (cursor) positions                   |
| `visual_to_content_pos(s, pos, skip_structural)` | Converts visual cursor pos to content character index |
| `content_to_visual_pos(s, pos, skip_structural)` | Converts content character index to visual cursor pos |

<!-- TODO: Add skip_structural argument instead of two separate methods -->

## Processing order

When multiple tags are active on the same text, they are processed in
this order:

1. hide
2. type (animation)
3. fade (animation)
4. scale (animation)
5. transform
6. wave
7. pulse
8. swing
9. jitter
10. gradient
11. opacity
12. color
13. shadow

Later entries override earlier ones if they affect the same property (color, opacity).

## Next steps

→ [Shaders & Effects](/docs/shaders-and-effects/)
