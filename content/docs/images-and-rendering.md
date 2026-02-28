+++
title = "Images, Vectors & Custom Rendering"
weight = 8
+++

Display images from files, embedded bytes, or custom draw calls.

## Static assets

Define image assets as `static GraphicAsset` constants:

```rust
static LOGO: GraphicAsset = GraphicAsset::Path("assets/images/logo.png");
static ICON: GraphicAsset = GraphicAsset::Bytes {
    file_name: "icon.tvg",
    data: include_bytes!("../assets/images/icon.tvg"),
};
```

Display them with `.image()`:
{% example(id="image_demo", height=360) %}
ui.element()
    .width(fixed!([[width]]))
    .height(fixed!([[height]]))
    .image(&LOGO)
    .empty();
---
n.width=200
n.height=200
{% end %}

The image scales to fill the element. The texture is loaded and cached
automatically.

### Vector graphics

Ply supports vector graphics via the `tinyvg` feature.
You can use TinyVG files in GraphicAsset just like PNGs:

```rust
static ICON: GraphicAsset = GraphicAsset::Path("assets/images/icon.tvg");
```

To use TinyVG, enable the feature in your `Cargo.toml`:

```toml
ply-engine = { version = "1.0", features = ["tinyvg"] }
```

Or add it with the CLI:

```bash
plyx add tinyvg
```

TinyVG assets are resolution-independent and scale seamlessly to any size, the
engine rasterizes them on-demand.
Everything is cached so that when nothing changes, no computation needs to be done.

To convert your existing `.svg` files into the ultra-compact `.tvg` format, use the [official TinyVG tools](https://tinyvg.tech/).
This gives you asset files that are blazingly fast to render with a fraction of
the file size compared to PNG or SVG.

Here is a cool tiger, just 27kB (the SVG is 100kB):

{% example(id="tiger_demo", height=470) %}
static TIGER: GraphicAsset = GraphicAsset::Bytes {
    file_name: "tiger.tvg",
    data: include_bytes!("tiger.tvg"),
};

ui.element()
    .width(fixed!([[width]]))
    .height(fixed!([[height]]))
    .image(&TIGER)
    .empty();
---
n.width=200
n.height=200
{% end %}

## Texture2D

Pass any macroquad `Texture2D` directly:

```rust
let tex = load_texture("assets/photo.png").await.unwrap();
tex.set_filter(FilterMode::Linear);

ui.element()
    .width(fixed!(300.0))
    .height(fixed!(200.0))
    .image(tex)
    .empty();
```

This is useful when you load textures yourself or generate them at runtime.
You can also use Ply's TextureManager to cache your textures.

## render_to_texture

Draw arbitrary macroquad geometry into a texture,

```rust
let chart = render_to_texture(w, h, || {
    clear_background(BLACK);
    let data = [0.2, 0.5, 0.3, 0.8, 0.6, 0.9, 0.4, 0.7];
    let step = w / (data.len() - 1) as f32;
    for i in 0..data.len() - 1 {
        let x1 = step * i as f32;
        let y1 = h - data[i] as f32 * h;
        let x2 = step * (i + 1) as f32;
        let y2 = h - data[i + 1] as f32 * h;
        draw_line(x1, y1, x2, y2, 2.0, GREEN);
    }
    for (i, &val) in data.iter().enumerate() {
        let x = step * i as f32;
        let y = h - val as f32 * h;
        draw_circle(x, y, 4.0, RED);
    }
});
```

then use it as an element's image:
{% example(id="chart_demo", height=530) %}
let mut ui = ply.begin();

ui.element()
    .width(fixed!([[width]]))
    .height(fixed!([[height]]))
    .image(chart)
    .empty();
---
n.width=400
n.height=200
{% end %}

`render_to_texture` uses MSAA (Antialiasing) and linear filtering, just like the rest of Ply.

## Procedual TinyVG

With the `tinyvg` feature, you can display procedural vector graphics that
rasterize at the element's layout size each frame:

```rust
let tvg_bytes = include_bytes!("../assets/icon.tvg");
let image = tinyvg::Decoder::new(std::io::Cursor::new(tvg_bytes))
    .decode()
    .unwrap();

image.commands.push(tinyvg::format::Command::FillPolygon {
    polygon: vec![
        tinyvg::format::Point { x: 50.0, y: 10.0 },
        tinyvg::format::Point { x: 90.0, y: 190.0 },
        tinyvg::format::Point { x: 10.0, y: 190.0 },
    ],
    fill_style: tinyvg::format::Style::FlatColor(0),
    outline: None,
});

ui.element()
    .width(fixed!(64.0))
    .height(fixed!(64.0))
    .image(image)
    .empty();
```

TinyVG images are resolution-independent. The engine uses lyon tessellation to
render filled paths at whatever size the layout gives the element.

## ImageSource

`.image()` accepts anything that implements `Into<ImageSource>`:

| Type                     | What it does                                    |
|--------------------------|-------------------------------------------------|
| `&'static GraphicAsset`  | File path or embedded bytes, auto-cached        |
| `Texture2D`              | Pre-existing GPU texture                        |
| `tinyvg::format::Image`  | Procedural vector (requires `tinyvg` feature)   |

## Next steps

→ [Text Input](/docs/text-input/)
