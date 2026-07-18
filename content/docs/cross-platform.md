+++
title = "Cross-Platform Builds"
weight = 18
+++

Ply apps run on Linux, macOS, Windows, Android, iOS, and the web.
Desktop builds use `cargo build`. Other platforms use the `plyx` CLI.

## Desktop

No special tooling needed:

```bash
cargo build
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

Depending on your `cargo` version, this command can fail with several linker errors, which
complains about not finding certain symbols.  This is actually not an issue: those symbols are
provided by the runtime, so it is normal that the linked cannot find them yet.  To make `cargo`
ignore them, you need to add the following lines in the file `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = ["-C", "link-args=--allow-undefined"]
```

For CI pipelines, use `--auto` to skip interactive prompts:

```bash
plyx web --auto
```

## Android

Build an APK using Docker, this will download the 4GB docker image (2GB less than macroquad):

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

Before building to iOS, you should make sure your Mac has Xcode installed and your Rust toolchain is set to `aarch64-apple-darwin`. Otherwise switch to the correct toolchain and reinstall plyx:

```bash
rustup toolchain install stable-aarch64-apple-darwin
rustup default stable-aarch64-apple-darwin
cargo install plyx --force
```

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

| Command            | What it does                        |
|--------------------|-------------------------------------|
| `plyx init`        | Scaffold a new Ply project          |
| `plyx add`         | Add features or fonts interactively |
| `plyx web`         | Build for WASM                      |
| `plyx apk`         | Build Android APK                   |
| `plyx ios`         | Build for iOS                       |
| `plyx completions` | Generate shell completions          |

## Next steps

→ [Advanced Topics](/docs/advanced/)
