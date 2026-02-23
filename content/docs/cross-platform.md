+++
title = "Cross-Platform Builds"
weight = 16
+++

Ply apps run on Linux, macOS, Windows, Android, iOS, and the web.
Desktop builds use `cargo run`. Other platforms use the `plyx` CLI.

## Desktop

No special tooling needed:

```bash
cargo run
```

Works on Linux, macOS, and Windows. Cross-compile with:

```bash
cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-apple-darwin
```

## Web (WASM)

Build with `plyx`:

```bash
plyx web
```

This compiles to `wasm32-unknown-unknown`, generates an `index.html`,
and bundles the JavaScript bridge (`ply_bundle.js`) which handles input,
audio, networking, and accessibility on the web.

For CI pipelines, use `--auto` to skip interactive prompts:

```bash
plyx web --auto
```

## Android

Build an APK using Docker:

```bash
plyx apk
```

Or use a local NDK installation:

```bash
plyx apk --native
```

Install directly to a connected device:

```bash
plyx apk --install
```

## iOS

Generate an Xcode project:

```bash
plyx ios
```

Build for a physical device:

```bash
plyx ios --device
```

Generate a GitHub Actions workflow for CI:

```bash
plyx ios --actions
```

## plyx CLI reference

| Command              | What it does                          |
|----------------------|---------------------------------------|
| `plyx init`          | Scaffold a new Ply project            |
| `plyx add`           | Add features or fonts interactively   |
| `plyx web`           | Build for WASM                        |
| `plyx apk`           | Build Android APK                     |
| `plyx ios`           | Build for iOS                         |
| `plyx completions`   | Generate shell completions            |

## Next steps

→ [Advanced Topics](/docs/advanced/)
