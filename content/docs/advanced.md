+++
title = "Advanced Topics"
weight = 19
+++

## Custom elements

Ply's engine is generic over a `CustomElementData` type. This lets you
attach arbitrary data to elements and handle it in your own renderer:

```rust
#[derive(Clone, Default, Debug)]
enum MyCustom {
  #[default]
  None,
  Chart { data: Vec<f32> },
  Canvas { id: u32 },
}
```

Attach it to an element:

```rust
ui.element()
  .width(grow!())
  .height(fixed!(200.0))
  .custom_element(MyCustom::Chart { data: values.clone() })
  .empty();
```

This spits out a `Custom<CustomElementData>` render command in the `Ui::show(|command| { println!("{:?}", command); })` callback.

## Headless mode

Create a Ply instance without GPU or window, useful for testing layout
logic:

```rust
let mut ply = Ply::new_headless((800.0, 600.0));
```

You'll need to provide a text measurement function:

```rust
ply.set_measure_text_function(|text, font_size| {
  // Return (width, height) based on your font metrics
  (text.len() as f32 * font_size * 0.6, font_size)
});
```

Then use `ply.begin()`, build your UI, call `ply.eval()`, and inspect
the resulting layout, all without a GPU context.

## Performance tuning

### Element count

By default, Ply pre-allocates capacity for elements. If you know your
UI has a large number of elements, set the limit up front:

```rust
ply.max_element_count(10_000);
```

### Text measurement cache

Ply caches text measurements by word. Increase the cache for text-heavy
UIs:

```rust
ply.max_measure_text_cache_word_count(50_000);
```

### Culling

Disable culling to render off-screen elements:

```rust
ply.set_culling(false);
```

I don't know why you would want to do this, but it's there if you need it.

## Custom text measurement

If you're using Ply's engine without macroquad's renderer (which we don't reccommend), provide your own text measurement:

```rust
ply.set_measure_text_function(|text, font_size| {
  let width = my_font.measure(text, font_size);
  let height = font_size;
  (width, height)
});
```

## Debug mode

Toggle debug view programmatically:

```rust
ply.set_debug_mode(true);
```

Or check the current state:

```rust
if ply.is_debug_mode() { /* ... */ }
```

See [Debug View](/docs/debug-view/) for details on what the debug
overlay shows.

## Layout dimensions

Update the layout viewport (e.g. on window resize):

```rust
ply.set_layout_dimensions(Dimensions {
  width: screen_width(),
  height: screen_height(),
});
```

## Pointer state

Query the raw pointer state:

```rust
let state = ply.pointer_state();
// state.position, state.is_down, etc.
```

## Manager Globals

Ply has different manager globals like `TEXTURE_MANAGER`, `MATERIAL_MANAGER`, `FONT_MANAGER` and `NET_MANAGER`. You can configure their eviction sensitivity.