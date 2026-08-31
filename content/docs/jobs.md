+++
title = "Jobs"
weight = 16
+++

Use the `jobs` module to run async work in the background.

## Spawn a job

```rust
jobs::spawn(
  "save_profile",
  || async move {
    let storage = Storage::new("my_app/data").await?;
    storage.save_string("profile.json", "{\"name\":\"Ply\"}").await
  },
  |result: Result<(), String>| {
    match result {
      Ok(()) => println!("Saved"),
      Err(err) => println!("Save failed: {}", err),
    }
  },
)?;
```

`on_complete` runs on the main thread during `ply.begin()`.
Spawning with an existing ID returns an error.

## Query job state

```rust
if jobs::is_running("save_profile") {
  // show loading UI
}

let active_jobs = jobs::list();
```

## Next steps

→ [Localization](/docs/localization/)
