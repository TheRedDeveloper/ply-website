+++
title = "IDs & State"
weight = 5
+++

Ply rebuilds your UI every frame. IDs are how the engine knows that this frame's
"submit button" is the same one as last frame's, so it can carry over focus,
scroll positions, hover state, and text input.

## Automatic IDs

Every element gets an ID automatically, derived from its parent and its position
among siblings. You don't have to think about it:

```rust
ui.element().width(grow!()).height(grow!())
  .layout(|l| l.direction(TopToBottom).gap(8).padding(12))
  .children(|ui| {
    // Auto-ID based on parent id + child index 0
    ui.element().width(grow!()).height(fixed!(40.0))
      .background_color(0x262220).empty();

    // Auto-ID based on parent id + child index 1
    ui.element().width(grow!()).height(fixed!(40.0))
      .background_color(0x3A3533).empty();
  });
```

Auto IDs are stable as long as children stay in the same order. For most
elements this is all you need.

## Explicit IDs

Set an ID with `.id()` when you need to reference an element, or for interactive elements when the parent or child index might change.

```rust
let sidebar_id = ui.element()
  .id("sidebar")
  .width(fixed!(200.0))
  .height(grow!())
  .background_color(0x181515)
  .layout(|l| l.direction(TopToBottom).gap(4).padding(8))
  .children(|ui| {
    ui.text("Navigation", |t| t.font_size(14).color(0xFFC32C));
  });

if let Some(bbox) = ui.bounding_box(sidebar_id) {
  println!("Sidebar bounding box: {:?}", bbox);
}
```
{{ demo(id="explicit_id_demo", height=200) }}

`.children()` and `.empty()` return the element's `Id`. But anywhere you need to give an id you can also put in the label: `ui.bounding_box("sidebar")` works too.

## Indexed IDs

When you render a list, each item needs a unique ID. Pass a `(&str, u32)` tuple:

{% example(id="indexed_nav_demo", height=785, base_height=525, height_per_item=65, list_param="items") %}
let items = [
[[items:  "$",]]
];
let active = [[active]];

for (i, label) in items.iter().enumerate() {
  let bg = if i == active { 0x3A3533 } else { 0x262220 };
  ui.element()
    .id(("nav_item", i as u32))
    .width(grow!())
    .height(fixed!(36.0))
    .background_color(bg)
    .corner_radius(6.0)
    .layout(|l| l.padding(8).align(Left, CenterY))
    .children(|ui| {
      ui.text(label, |t| t.font_size(14).color(0xE8E0DC));
    });
}
---
l.items=Home|Settings|Profile|About
n.active=0
{% end %}

The string `"nav_item"` and the index `i` are hashed together. Each item gets
a stable ID regardless of how many items are in the list. Whenever you need an indexed ID, just pass a tuple: `ply.set_focus(("nav_item", 2))`. 

## Inline state

You can check the state of the element you are currently building inside its `.children()` closure:

{% example(id="hover_press_demo", height=650) %}
ui.element()
  .width(fit!())
  .height(fixed!(40.0))
  .corner_radius(8.0)
  .children(|ui| {
    let bg = if ui.pressed() {
      [[press_color]]
    } else if ui.hovered() {
      [[hover_color]]
    } else {
      [[default_color]]
    };

    ui.element()
      .width(fit!())
      .height(grow!())
      .background_color(bg)
      .corner_radius(8.0)
      .layout(|l| l.padding((0, 16, 0, 16)).align(CenterX, CenterY))
      .children(|ui| {
        ui.text("Hover me", |t| t.font_size(14).color(0xE8E0DC));
      });
  });
---
c.press_color=0xFF654D
c.hover_color=0x3A3533
c.default_color=0x262220
{% end %}

| Method               | What it does                          |
|----------------------|---------------------------------------|
| `ui.hovered()`       | Is pointer over this element?         |
| `ui.pressed()`       | Is pointer held down on this element  |
| `ui.just_pressed()`  | Was this element pressed this frame?  |
| `ui.just_released()` | Was this element released this frame? |
| `ui.focused()`       | Does this element have keyboard focus |
| `ui.scroll_offset()` | What's this element's scroll offset   |

These check the currently open parent element, the one whose `.children()`
closure you're inside.

## Querying state by ID

When you need to check state from anywhere use `Ply` methods with an ID:

```rust
if ply.is_pressed("card") {
  // card is being held down
}

if ply.is_just_pressed("card") {
  // one frame only
}

if ply.is_just_released("card") {
  // one frame only
}

let mut ui = ply.begin();

let card_id = ui.element()
  .id("card")
  .width(fixed!(200.0))
  .height(fixed!(120.0))
  .background_color(0x262220)
  .corner_radius(8.0)
  .children(|ui| {
    ui.text("Hello", |t| t.font_size(18).color(0xE8E0DC));
  });

if ui.pointer_over(card_id) {
  // pointer is over the card
}
```

`Ui` is just a `Ply` that has begun, so that you can start making elements.
You can use these query methods on both `ply` and `ui`.

```rust
ui.set_focus("search_box");

if let Some(focused) = ply.focused_element() {
    // focused is an Id
}

ui.clear_focus();

let value = ui.get_text_value("editor");

ui.set_text_value("editor", "hello world");

let pos = ui.get_cursor_pos("editor");

ui.set_cursor_pos("editor", 5);

if let Some((start, end)) = ui.get_selection_range("editor") {
    // there's an active selection start to end
}
ui.set_selection("editor", 0, 10);  // select first 10 chars

if let Some(data) = ply.scroll_container_data("my_list") {
    // data.scroll_position, data.content_dimensions, etc.
}

ui.set_scroll_position("my_list", (0.0, 600.0));
```

## Constructing IDs directly

You can create IDs without an element builder:

```rust
let id = Id::new("my_button");
let id = Id::new_index("item", 3);
```

Useful when comparing IDs.

## Next steps

→ [Interactivity](/docs/interactivity/)

