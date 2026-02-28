+++
title = "Accessibility"
weight = 13
+++

Ply has built-in screen reader support via AccessKit on desktop and a
JavaScript accessibility bridge on the web. Keyboard navigation works out of the box.

The `a11y` feature is enabled by default. It pulls in
AccessKit for Linux, macOS, Windows, and Android.

Sadly AccessKit's team is still working on iOS support. The adapter has been funded since November 2025. As soon as it comes out we will be supporting iOS.

## Marking elements

Use `.accessibility()` on any element to expose it to assistive
technology:

```rust
ui.element()
  .id("submit")
  .width(fixed!(120.0))
  .height(fixed!(40.0))
  .background_color(0xB91414)
  .corner_radius(6.0)
  .accessibility(|a| a.button("Submit"))
  .on_press(|_| { /* handle click */ })
  .children(|ui| {
    ui.text("Submit", |t| t.font_size(14).color(0xFFFFFF));
  });
```

The `.button("Submit")` shorthand sets the role to Button, the label to
"Submit", and marks the element as focusable.

## Roles

Role shortcuts on the builder:

| Shorthand                  | Role           | Auto-focusable |
|----------------------------|----------------|----------------|
| `.button("label")`         | Button         | yes            |
| `.link("label")`           | Link           | yes            |
| `.heading("label", level)` | Heading(level) | no             |
| `.static_text("label")`    | StaticText     | no             |
| `.checkbox("label")`       | Checkbox       | yes            |
| `.slider("label")`         | Slider         | yes            |
| `.image("alt text")`       | Image          | no             |

For other roles, use `.role()` directly:

```rust
.accessibility(|a| a
  .role(AccessibilityRole::Dialog)
  .label("Settings")
)
```

Available roles: `None`, `Button`, `Link`, `Heading`, `Label`,
`StaticText`, `TextInput`, `TextArea`, `Checkbox`, `RadioButton`,
`Slider`, `Group`, `List`, `ListItem`, `Menu`, `MenuItem`, `MenuBar`,
`Tab`, `TabList`, `TabPanel`, `Dialog`, `AlertDialog`, `Toolbar`,
`Image`, `ProgressBar`.

## Properties

```rust
.accessibility(|a| a
  .slider("Volume")
  .description("Adjusts the master volume from 0 to 100")
  .value("75")
  .value_min(0.0)
  .value_max(100.0)
)
```

| Method              | What it does                        |
|---------------------|-------------------------------------|
| `.label("text")`    | Screen reader label                 |
| `.description("t")` | Extended description                |
| `.value("75")`      | Current value (sliders, progress)   |
| `.value_min(0.0)`   | Minimum value                       |
| `.value_max(100.0)` | Maximum value                       |
| `.checked(true)`    | Checked state (checkboxes, radios)  |
| `.focusable()`      | Adds to tab order                   |

## Accessible text

Static text elements can be exposed with `.accessible()`:

```rust
ui.text("Welcome to Ply", |t| t.font_size(24).color(0xFFFFFF).accessible());
```

Without `.accessible()`, text is purely visual and invisible to screen
readers.

## Tab order

By default, focusable elements are tabbed in insertion order. Use
`.tab_index()` for explicit ordering:

```rust
ui.element()
  .id("second")
  .accessibility(|a| a.button("Second").tab_index(2))
  .empty();

ui.element()
  .id("first")
  .accessibility(|a| a.button("First").tab_index(1))
  .empty();
```

## Directional focus

Override arrow key focus movement for custom navigation patterns:

```rust
.accessibility(|a| a
  .button("Item A")
  .focus_right("item_b")
  .focus_down("item_c")
)
```

| Method             | Arrow key   |
|--------------------|-------------|
| `.focus_right(id)` | Right arrow |
| `.focus_left(id)`  | Left arrow  |
| `.focus_up(id)`    | Up arrow    |
| `.focus_down(id)`  | Down arrow  |

## Focus ring

When an element receives focus via keyboard (Tab or arrow keys), Ply
draws a focus ring around it. Mouse clicks do not trigger the ring.

To hide the ring on a specific element:

```rust
.accessibility(|a| a.focusable().disable_ring())
```

You can also customize the ring's color and width:

```rust
.accessibility(|a| a
  .button("Submit")
  .ring_color(0x0078FF)
  .ring_width(3)
)
```

| Method            | Default   | Description              |
|-------------------|-----------|--------------------------|
| `.ring_color(c)`  | `#FF3C28` | Ring color               |
| `.ring_width(w)`  | `2`       | Ring thickness in pixels |
| `.disable_ring()` | —         | Hides the ring entirely  |

## Live regions

Announce dynamic content changes to the screen reader:

```rust
// Polite: waits for current speech to finish
ui.element()
  .id("status")
  .accessibility(|a| a.static_text("3 items loaded").live_region_polite())
  .empty();

// Assertive: interrupts immediately
ui.element()
  .id("error")
  .accessibility(|a| a.static_text("Connection lost").live_region_assertive())
  .empty();
```

## Keyboard navigation

These work automatically for focusable elements:

| Key           | Action                            |
|---------------|-----------------------------------|
| Tab           | Focus next element                |
| Shift + Tab   | Focus previous element            |
| Enter / Space | Activate focused element          |
| Arrow keys    | Directional focus (if configured) |

## Platform integration

| Platform        | Backend                                 |
|-----------------|-----------------------------------------|
| Linux           | AccessKit (AT-SPI)                      |
| macOS           | AccessKit (NSAccessibility)             |
| Windows         | AccessKit (UI Automation)               |
| Android         | AccessKit (Android Accessibility)       |
| Web (WASM)      | JavaScript accessibility bridge         |

On the web, Ply uses a JavaScript bridge that creates a hidden DOM tree
mirroring the accessible elements, so screen readers see standard HTML
semantics.

Look out for iOS support in the future.

## Checkbox example

```rust
ui.element()
  .id("accept_terms")
  .width(fixed!(24.0))
  .height(fixed!(24.0))
  .background_color(if checked { 0xFFC32C } else { 0x3A3533 })
  .corner_radius(4.0)
  .accessibility(|a| a.checkbox("Accept terms").checked(checked))
  .on_press(move |_| {
    checked = !checked;
  })
  .children(|ui| {
    if checked {
      ui.text("✓", |t| t.font_size(16).color(0x181515));
    }
  });
```

## Next steps

→ [Sound](/docs/sound/)
