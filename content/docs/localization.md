+++
title = "Localization"
weight = 17
+++

> ply-engine 1.2 is not yet released, so the `locales` feature is not yet available. You can still use `ply-locales` standalone.

Ply apps can be localized into multiple languages using `ply-locales`, a procedural macro crate built for Project Fluent. This crate can be used standalone, or through the `locales` feature in `ply-engine`.

Translations are checked at compile time. Message keys become typed Rust functions, and missing variables or typos are caught before your app builds.

## Setup

Add `ply-locales` and `fluent-bundle` to your `Cargo.toml`:

```toml
[dependencies]
ply-locales = "0.1"
fluent-bundle = "0.16"
```

Or use the `locales` feature in `ply-engine` (when 1.2 is released):

```toml
[dependencies]
ply-engine = { version = "1.2", features = ["locales"] } # ply-engine 1.2 is not yet released
```

(or add it with the CLI `plyx add locales` in the future when 1.2 is released)

## Demonstration

{{ localization_example(height=500) }}

## Project structure

Place translation files in a `locales/` directory (or any other directory you specify):

<div class="project-view" style="border-radius: 12px; margin-bottom: 1.5rem;">
   <div class="pv-tree">
      <div class="pv-item">
         <div class="pv-entry">Cargo.toml</div>
      </div>
      <div class="pv-item">
         <div class="pv-entry pv-dir"><span>locales</span></div>
         <div class="pv-children">
            <div class="pv-item">
               <div class="pv-entry">de-DE.ftl</div>
            </div>
            <div class="pv-item">
               <div class="pv-entry">en-US.ftl</div>
            </div>
            <div class="pv-item">
               <div class="pv-entry">es-ES.ftl</div>
            </div>
         </div>
      </div>
      <div class="pv-item">
         <div class="pv-entry pv-dir"><span>src</span></div>
         <div class="pv-children">
            <div class="pv-item">
               <div class="pv-entry">main.rs</div>
            </div>
         </div>
      </div>
   </div>
</div>

`ply-locales` supports flat files (`locales/de-DE.ftl`), subdirectories (`locales/en-US/main.ftl`), or a mix of both. File and directory names must be valid language identifiers like `en-US`, `de-DE`, `fr` or `zh-Hant-TW`.

## Generating the typed API

Annotate a module with `#[ply_locales::ply_locales]`. By default, `"en-US"` is used as the fallback default locale:

```rust
#[ply_locales::ply_locales("locales")]
pub mod t {}

// Or with a default locale other than "en-US":
#[ply_locales::ply_locales("locales", default = "de-DE")]
pub mod t {}
```

`ply-locales` generates typed Rust functions for every message in the default locale:

- Fluent keys in kebab-case or camelCase become snake_case
- Messages with variables accept any type implementing `Into<FluentValue>`
- Hovering over `t::order_summary()` in an editor shows the raw Fluent definition and links directly to the `.ftl` line number.

### Using translations in Ply UI

The `locales` feature provides `ply_locales` in the prelude and includes fluent-bundle automatically. You can just pass the strings into `ui.text()`:

```rust
use ply_engine::prelude::*;

#[ply_locales("locales")] pub mod t {}

ui.element()
  .layout(|l| l.direction(TopToBottom).gap(8).padding(16))
  .children(|ui| {
    ui.text(&t::welcome_hero(), |t| t.font_size(24).color(0xFFFFFF));
    ui.text(&t::quest_status(3), |t| t.font_size(16).color(0x9E9590));
  });
```

## Usage

`ply-locales` manages everything for you:

| Function / Constant      | Description                                  |
|--------------------------|----------------------------------------------|
| `t::AVAILABLE_LOCALES`   | Slice of all locales                         |
| `t::set_locale("es-ES")` | Switches active locale (`true` if available) |
| `t::current_locale()`    | Current locale                               |

### Custom functions

You can define custom Rust functions:

```rust
#[ply_locales("locales")]
pub mod t {
    pub fn upper(s: &str) -> String {
        s.to_uppercase()
    }
}
```

And call them in your `.ftl` files:

```fluent
greet = Hello, { UPPER($name) }!
```

- Rust `snake_case` functions map to uppercase in Fluent
- Calls in Fluent templates are checked at compile time
- Functions remain directly callable in Rust as well

### `VOID`

`ply-locales` provides a built-in `VOID()` function. `VOID` lets you satisfy variable requirements without displaying the variable:

```fluent
user-greeting = Hello, { $name }!{ VOID($gender) }
```

## Compile-time safety

Readable errors are always emitted at compile time, such as mismatched variables:

```text
error: Mismatched Fluent variables in message 'order-summary' for locale 'de-DE'
        --> locales/de-DE.ftl:2
         |
       2 | order-summary = Bestellung { $order_id } für { $client } hat { $item_count } Artikel.
         |
         = expected: [$order_id, $customer, $item_count]
         = found:    [$order_id, $client, $item_count]
```

Missing arguments:

```text
error: Missing argument 'hey' in call to term '-variable-inside' in message 'greet' for locale 'en-US'
        --> locales/en-US.ftl:2:28-43
         |
       2 | greet = Hello, { $name } { -variable-inside }!
         |                            ^^^^^^^^^^^^^^^^ Missing argument 'hey'
```

Circular dependencies:

```text
error: Circular dependency detected in locale 'en-US'
        --> locales/en-US.ftl
         |
       1 | -a = { -b }
       2 | -b = { -a }
         |
         = cycle: -a -> -b -> -a
```

and many more! Syntax errors also emit readable compile-time errors:

```text
error: Syntax error in Fluent file 'locales/en-US.ftl'
        --> locales/en-US.ftl:1:16-18
         |
       1 | greet = Hello, { }
         |                ^^^ Expression can't be empty
```

Missing translations emit a compiler warning and fall back to the default language at runtime.

Additional messages in a translated locale emit a compiler warning and are ignored at runtime.

## Next steps

→ [Sound](/docs/sound/)
