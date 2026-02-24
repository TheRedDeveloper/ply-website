+++
title = "Images & Custom Rendering"
weight = 8
+++

<!-- TODO: Maybe rename to Images, Vectors & Custom Rendering, so people can find info on how to do vecto stuff quicker -->  

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

```rust
ui.element()
    .width(fixed!(200.0))
    .height(fixed!(200.0))
    .image(&LOGO)
    .empty();
```
<!-- TODO: Embedded WASM demo — image from asset -->

The image scales to fill the element. The texture is loaded and cached
automatically.

### Vector graphics

Ply supports procedural vector graphics via the `tinyvg` feature.
You can use TinyVG files in GraphicAsset just like PNGs:

```rust
static ICON: GraphicAsset = GraphicAsset::Path("assets/images/icon.tvg");
```

To use TinyVG, enable the feature in your `Cargo.toml`:

```toml
ply = { version = "1.0", features = ["tinyvg"] }
```

TinyVG assets are resolution-independent and scale seamlessly to any size, the
engine rasterizes them on-demand.
Everything is cached so that when nothing changes, no computation needs to be done.

To convert your existing `.svg` files into the ultra-compact `.tvg` format, use the [official TinyVG tools](https://tinyvg.tech/).
This gives you asset files that are blazingly fast to render with a fraction of
the file size compared to PNG or SVG.

Here is a cool tiger, just 27kB (the SVG is 100kB):

<!-- Width and height should be editable -->
```rust
static TIGER: GraphicAsset = GraphicAsset::Bytes {
    file_name: "tiger.tvg",
    data: include_bytes!("../assets/images/tiger.tvg"),
};

ui.element()
    .width(fixed!(300.0))
    .height(fixed!(200.0))
    .image(&TIGER)
    .empty();
```
<!-- TODO: Embedded WASM demo — procedural TinyVG image with editable width and height -->

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

Draw arbitrary macroquad geometry into a texture, then use it as an element's image.

```rust
let chart = render_to_texture(400.0, 200.0, || {
    clear_background(BLANK);
    draw_line(10.0, 180.0, 390.0, 20.0, 2.0, GREEN);
    draw_circle(200.0, 100.0, 30.0, RED);
});

let mut ui = ply.begin();

ui.element()
    .width(fixed!(400.0))
    .height(fixed!(200.0))
    .corner_radius(8.0)
    .image(chart)
    .empty();
```
<!-- TODO: Embedded WASM demo — custom render_to_texture chart -->

`render_to_texture` uses MSAA (Antialiasing) and linear filtering, just like the rest of Ply.

## Procedual TinyVG

With the `tinyvg` feature, you can display procedural vector graphics that
rasterize at the element's layout size each frame:

```rust
// Decode from embedded bytes
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
<!-- TODO: actually try this example -->

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
