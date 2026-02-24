+++
title = "Getting Started"
weight = 1
+++

Install the Ply CLI and scaffold your first project in under a minute.

## Install plyx

[plyx](https://github.com/TheRedDeveloper/plyx) is Ply's CLI companion. It scaffolds projects, builds for web and mobile, and manages fonts.

```bash
cargo install plyx
```

## Create a project

Run `plyx init`. The interactive TUI lets you pick a name, select a font from Google Fonts, and choose feature flags:

```bash
plyx init
```

You'll see a selection of features you can enable:

| Feature              | What it adds                                                  |
|----------------------|---------------------------------------------------------------|
| **Built-in shaders** | Pre-made visual effects                                       |
| **Text styling**     | Rich text with inline colors, sizes, shadows, and animations  |
| **TinyVG**           | Vector graphics support                                       |
| **Shader pipeline**  | Custom shader compilation with SPIR-V Cross (adds `build.rs`) |

Don't worry about choosing wrong. You can add features later with `plyx add`.

## Your first app

After `plyx init`, your project is ready to run:

<div class="project-view">
   <div class="pv-tree">
      <div class="pv-item">
         <div class="pv-entry">Cargo.toml</div>
      </div>
      <div class="pv-item">
         <div class="pv-entry pv-dir"><span>assets</span></div>
         <div class="pv-children">
            <div class="pv-item">
               <div class="pv-entry pv-dir"><span>fonts</span></div>
               <div class="pv-children">
                  <div class="pv-item">
                     <div class="pv-entry">MyFont.ttf</div>
                  </div>
               </div>
            </div>
         </div>
      </div>
      <div class="pv-item">
         <div class="pv-entry pv-dir"><span>src</span></div>
         <div class="pv-children">
            <div class="pv-item">
               <div class="pv-entry pv-active">main.rs</div>
            </div>
         </div>
      </div>
   </div>
</div>

```rust
use ply_engine::prelude::*;

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Hello Ply!".to_owned(),
            window_width: 800,
            window_height: 600,
            high_dpi: true,
            sample_count: 4,
            platform: miniquad::conf::Platform {
                webgl_version: miniquad::conf::WebGLVersion::WebGL2,
                ..Default::default()
            },
            ..Default::default()
        },
        draw_call_vertex_capacity: 100000,
        draw_call_index_capacity: 100000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/MyFont.ttf");
    let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;

    loop {
        clear_background(MacroquadColor::new(0.0, 0.0, 0.0, 1.0));

        let mut ui = ply.begin();

        ui.element().width(grow!()).height(grow!())
            .layout(|l| l.align(CenterX, CenterY))
            .children(|ui| {
                ui.text("Hello, Ply!", |t| t
                    .font_size(32)
                    .color(0xFFFFFF)
                );
            });

        ui.show(|_| {}).await;

        next_frame().await;
    }
}
```

Run it:

```bash
cargo run
```

You should see a window with "Hello, Ply!" centered on a black background.

## Breaking it down

### The prelude

```rust
use ply_engine::prelude::*;
```

One import gives you everything: `Ply`, `Ui`, `Id`, sizing macros (`grow!`, `fit!`, `fixed!`, `percent!`), alignment enums (`CenterX`, `CenterY`, `Left`, `Right`, `Top`, `Bottom`), layout directions (`TopToBottom`, `LeftToRight`), and the full macroquad prelude.

Ply re-exports macroquad's prelude (including `clear_background`, `next_frame`), but renames `Color` to `MacroquadColor` to avoid conflicts.

### Window configuration

```rust
fn window_conf() -> macroquad::conf::Conf { ... }
```

This configures the window title, size, DPI, anti-aliasing, and WebGL version. Ply runs on macroquad's rendering backend, you get GPU-accelerated rendering on every platform.

### The main loop

```rust
static DEFAULT_FONT: FontAsset = FontAsset::Path("assets/fonts/MyFont.ttf");
let mut ply = Ply::<()>::new(&DEFAULT_FONT).await;
```

`FontAsset` tells Ply where to find the font file. `Ply::new(&DEFAULT_FONT).await` creates the engine and loads your default font.  The `<()>` type parameter is for adding custom elements. We'll ignore it for now.

```rust
loop {
    clear_background(MacroquadColor::new(0.0, 0.0, 0.0, 1.0));
    let mut ui = ply.begin();
    // ... build your UI here ...
    ui.show(|_| {}).await;
    next_frame().await;
}
```

Every frame: clear the screen, call `ply.begin()` to get a `Ui` handle, build your element tree, then `ui.show()` to layout and render everything. The closure in `show()` is for custom element rendering, just like the `<()>`. You can ignore it for now.

### Building elements

```rust
ui.element().width(grow!()).height(grow!())
    .layout(|l| l.align(CenterX, CenterY))
    .children(|ui| {
        ui.text("Hello, Ply!", |t| t
            .font_size(32)
            .color(0xFFFFFF)
        );
    });
```

Everything in Ply is an element. Elements use a builder pattern:
- **Sizing**: using sizing macros
- **Layout**: describes how children are laid out
- **Children**: a closure where you build nested elements

`ui.text()` creates a text element, it takes a string and a config closure.

## Sizing macros

These four macros control how elements are sized:

| Macro           | Behavior                 |
|-----------------|--------------------------|
| `grow!()`       | Fill all available space |
| `fit!()`        | Shrink to fit content    |
| `fixed!(200.0)` | Exactly 200 pixels       |
| `percent!(50)`  | 50% of parent's size     |

`fit` and `grow` also accept optional min: `fit!(10)` (don't shrink below 10px), and optional max: `grow!(10, 100)` (grow between 10px and 100px)

## Zero-Clause BSD

Ply is licensed under the [Zero-Clause BSD](https://opensource.org/license/0bsd) license. Use it for anything, commercial, personal, open source, no attribution required, no strings attached. I love any and all uses of Ply, so go wild and build something amazing!

## Next steps

Now that you have a running app, learn how to build real layouts:

→ [Elements & Layout](/docs/elements-and-layout/)
