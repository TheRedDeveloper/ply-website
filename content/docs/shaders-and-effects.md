+++
title = "Shaders & Effects"
weight = 11
+++

Ply lets you apply fragment shaders to individual elements or groups of
elements. Write standard GLSL ES 3.00, pass uniforms, and get
per-element post-processing without boilerplate.

## Per-element effects

Use `.effect()` to apply a shader to a single element. The shader
receives the element's rendered content as `sampler2D Texture`:

```rust
const TINT: ShaderAsset = ShaderAsset::Path("shaders/tint.frag");

ui.element()
    .width(fixed!(200.0))
    .height(fixed!(100.0))
    .background_color(0xE8E0DC)
    .effect(&TINT, |s| s
        .uniform("u_time", get_time() as f32)
        .uniform("u_tint", [1.0f32, 0.3, 0.2, 0.5])
    )
    .empty();
```
<!-- TODO: Embedded WASM demo — per-element effect -->

Every shader automatically gets two uniforms:
- `u_resolution`: element width and height in pixels (`vec2`)
- `u_position`: element position in screen space (`vec2`)

## Group shaders

Use `.shader()` to capture an element and all its children into an
offscreen buffer, then apply the shader as post-processing:

```rust
ui.element()
    .shader(&FOIL, |s| s
        .uniform("u_time", get_time() as f32)
        .uniform("u_speed", 1.5f32)
    )
    .children(|ui| {
        ui.text("Shiny card", |t| t.font_size(18).color(0xFFFFFF));
        ui.element()
            .width(grow!())
            .height(fixed!(60.0))
            .background_color(0x3A3533)
            .empty();
    });
```

Multiple `.shader()` calls nest, the first is innermost (closest to the content), later ones wrap previous:

```rust
ui.element()
    .shader(&BLUR_SHADER, |s| s.uniform("u_radius", 4.0f32))
    .shader(&COLOR_GRADE, |s| s.uniform("u_contrast", 1.2f32))
    .children(|ui| { /* ... */ });
```

## ShaderAsset

Three ways to provide shader source:

| Variant                                       | Use case           |
|-----------------------------------------------|--------------------|
| `ShaderAsset::Path("shaders/fx.frag")`        | File on disk       |
| `ShaderAsset::Source { file_name, fragment }` | Embedded in binary |
| `ShaderAsset::Stored("name")`                 | Runtime-updateable |

### Path

```rust
const WAVE: ShaderAsset = ShaderAsset::Path("shaders/wave.frag");
```

Reads the file each time the material is created.

### Source

```rust
const WAVE: ShaderAsset = ShaderAsset::Source {
    file_name: "wave",
    fragment: include_str!("../shaders/wave.frag"),
};
```

Baked into the binary. No file fetch at runtime.

### Stored (live-editable)

```rust
use ply_engine::renderer::set_shader_source;

// Update the source at any time:
set_shader_source("live_shader", &editor_text);

// Reference it:
const LIVE: ShaderAsset = ShaderAsset::Stored("live_shader");

ui.element()
    .effect(&LIVE, |s| s.uniform("u_time", get_time() as f32))
    .empty();
```

When the source changes, the old compiled material is evicted
automatically. The next render pass recompiles.

## Uniform types

`.uniform()` accepts anything that implements `Into<ShaderUniformValue>`:

| Rust type       | GLSL type |
|-----------------|-----------|
| `f32`           | `float`   |
| `[f32; 2]`      | `vec2`    |
| `[f32; 3]`      | `vec3`    |
| `[f32; 4]`      | `vec4`    |
| `i32`           | `int`     |
| `[[f32; 4]; 4]` | `mat4`    |

## Built-in shaders

Enable with the `built-in-shaders` feature:

```toml
[dependencies]
ply-engine = { version = "1.0", features = ["built-in-shaders"] }
```

Or add it with the CLI:

```bash
plyx add built-in-shaders
```

Built-in shaders are automatically included in the prelude.

### FOIL

```rust
.effect(&FOIL, |s| s
    .uniform("u_time", get_time() as f32)
    .uniform("u_speed", 1.0f32)
    .uniform("u_intensity", 0.3f32)
)
```

### HOLOGRAPHIC

```rust
.effect(&HOLOGRAPHIC, |s| s
    .uniform("u_time", get_time() as f32)
    .uniform("u_speed", 1.0f32)
    .uniform("u_saturation", 0.7f32)
)
```

### DISSOLVE

```rust
.effect(&DISSOLVE, |s| s
    .uniform("u_threshold", progress)
    .uniform("u_edge_color", [1.0f32, 0.5, 0.0, 1.0])
    .uniform("u_edge_width", 0.05f32)
    .uniform("u_seed", 42.0f32)
)
```

### GLOW

```rust
.effect(&GLOW, |s| s
    .uniform("u_glow_color", [0.0f32, 0.5, 1.0, 1.0])
    .uniform("u_glow_radius", 0.05f32)
    .uniform("u_glow_intensity", 1.0f32)
)
```

### CRT

```rust
.effect(&CRT, |s| s
    .uniform("u_line_count", 100.0f32)
    .uniform("u_intensity", 0.3f32)
    .uniform("u_time", get_time() as f32)
)
```

### GRADIENT_LINEAR

```rust
.effect(&GRADIENT_LINEAR, |s| s
    .uniform("u_color_a", [0.73f32, 0.08, 0.08, 1.0])
    .uniform("u_color_b", [1.0f32, 0.76, 0.17, 1.0])
    .uniform("u_angle", 0.0f32)
)
```

### GRADIENT_RADIAL

```rust
.effect(&GRADIENT_RADIAL, |s| s
    .uniform("u_color_a", [1.0f32, 0.4, 0.3, 1.0])
    .uniform("u_color_b", [0.15f32, 0.13, 0.12, 1.0])
    .uniform("u_center", [0.5f32, 0.5])
    .uniform("u_radius", 0.5f32)
)
```

### GRADIENT_CONIC

```rust
.effect(&GRADIENT_CONIC, |s| s
    .uniform("u_color_a", [1.0f32, 0.76, 0.17, 1.0])
    .uniform("u_color_b", [0.73f32, 0.08, 0.08, 1.0])
    .uniform("u_center", [0.5f32, 0.5])
    .uniform("u_offset", 0.0f32)
    .uniform("u_hardness", 0.0f32)
)
```

## Writing a shader

Shaders are GLSL ES 3.00 fragment shaders. The shaded content is
available as `sampler2D Texture` and the UV coordinates come from the
bounding box:

```glsl
#version 300 es
precision highp float;

in vec2 uv;
out vec4 fragColor;

uniform sampler2D Texture;
uniform vec2 u_resolution;
uniform vec2 u_position;
uniform float u_time;

void main() {
    vec4 color = texture(Texture, uv);
    // Tint red based on time
    color.r = mix(color.r, 1.0, sin(u_time) * 0.5 + 0.5);
    fragColor = color;
}
```

## Shader build pipeline

For larger projects, use the `shader-build` feature to compile `.frag`
and `.slang` files via a `build.rs` script:

```toml
[build-dependencies]
ply-engine = { version = "1.0", features = ["shader-build"] }
```

```rust
use ply_engine::shader_build::ShaderBuild;

fn main() {
    ShaderBuild::new()
        .source_dir("shaders/")
        .output_dir("assets/build/shaders/")
        .build();
}
```

Or initialize it with the CLI:

```bash
plyx add shader-pipeline
```

This compiles all `.hlsl` and `.slang` files to GLSL ES 3.00, with
hash-based incremental rebuilds. Files go through SPIR-V cross-compilation.

Custom file types can be handled too:

```rust
ShaderBuild::new()
    .override_file_type_handler(".wgsl", |file_path, output_dir| {
        // Custom compilation logic
        vec!["shaders/includes/**/*.wgsl".to_string()]
    })
    .build();
```

## Next steps

→ [Rotation](/docs/rotation/)
