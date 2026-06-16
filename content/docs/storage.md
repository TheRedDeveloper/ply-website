+++
title = "Storage"
weight = 15
+++

Ply includes a cross-platform persistent storage API behind the `storage` feature.

## Setup

Enable storage in `Cargo.toml`:

```toml
ply-engine = { version = "1.1", features = ["storage"] }
```

Or add it with the CLI:

```bash
plyx add storage
```

## Create a storage root

```rust
let storage = Storage::new("my_app/data").await?;
```

`Storage::new` creates (or opens) a root folder for your app data. Use a relative path like `my_app/data`, this makes it relative to the platform's app data location:
- Windows: `%APPDATA%`
- macOS: `~/Library/Application Support`
- Linux: `~/.local/share`
- Android: App-specific storage directory
- iOS: `~/Documents`
- Web: OPFS root

## Save and load text

```rust
storage.save_string("settings.json", "{\"theme\":\"dark\"}").await?;
let settings = storage.load_string("settings.json").await?;
```

`load_string` returns `Result<Option<String>, String>`. `Ok(None)` if the file is missing.

## Save and load bytes

```rust
let bytes = vec![1, 2, 3, 4];
storage.save_bytes("saves/slot1.bin", &bytes).await?;
let slot = storage.load_bytes("saves/slot1.bin").await?;
```

`load_bytes` returns `Result<Option<Vec<u8>>, String>`. `Ok(None)` if the file is missing.

## Remove and export

```rust
storage.remove("saves/old_slot.bin").await?;
storage.export("settings.json").await?;
```

- `remove` deletes a file.
- `export` opens a platform-native save flow for that file.

## Next steps

→ [Jobs](/docs/jobs/)
