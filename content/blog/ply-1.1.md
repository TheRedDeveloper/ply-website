+++
title = "Ply 1.1: Building Polished UIs in Rust"
description = "How Ply 1.1 solves the main-thread blocking problem, brings OkLab color interpolation, and adds wrapping layouts."
date = 2026-04-05
[extra]
author = "RedDev"
+++

When I released Ply 1.0, a months ago, the response was incredible. The project already hit 250 stars on GitHub. But we have been continuing to improve Ply.

## The async nightmare

Applications need to do background work. If you do this on the main thread, your game stutters and your app freezes. If you do it in the background in Rust, you usually end up drowning in `mpsc` channels and wrapping everything in `Arc<Mutex<T>>`. Getting the result back to the UI thread is a chore.

Ply 1.1 fixes this with the new `jobs` module.

```rust
jobs::spawn(
  "save_game",
  || async move {
    Storage::new("my_app").await?
      .save_string("save.json",
        serde_json::to_string(&game_state).map_err(|e| e.to_string())?
      ).await
  },
  |result| {
    if let Err(e) = result {
      eprintln!("Failed to save game: {e}");
    }
  },
)?;

if jobs::is_running("save_game") {
  ui.text("{wave|Saving...}", |t| t.color(0xAAAAAA));
}
```

You spawn a job. It runs in the background. When it finishes, the completion callback runs on the main layout thread automatically. No channels to wire up. No mutexes freezing your UI. Works on all platforms, even wasm.

Paired with the new `storage` feature for reading and writing files across all platforms, state management feels a lot more natural.

## Polishing UI

Creating a UI is easy. Making it feel good is hard. Ply 1.1 introduces a `Lerp` trait across the engine as well as perceptually smooth color blending.

```rust
let c = Color::from_hex(0x1E1E1E).lerp_oklab(Color::from_hex(0x4CC9F0), t);
```

Oklab interpolation keeps the perceived brightness consistent, which makes animations look professional.

We also added a complete standard set of easing functions. From `ease_out_cubic` to `ease_in_out_bounce`, you no longer need to write your own math functions to make a button press feel snappy.

## Responsive layouts

Native layouts often feel rigid compared to the web. Ply's layout engine now supports wrapping.

```rust
ui.element()
  .layout(|l| l.wrap().wrap_gap(8))
  .children(|ui| {
    // Elements automatically flow to the next row
  });
```

We also added `contain` and `cover` aspect ratios, plus weighted `grow!` sizing. You can now tell an element to take up exactly twice as much remaining space as its sibling using `grow!(weight: 2.0)`. Building a responsive grid now takes seconds instead of manual math.

## The little things

There is a lot more in this release. We added setting scroll positions, drag-select for text inputs, a customizable scrollbar. Check out the full [changelog](https://github.com/TheRedDeveloper/ply-engine/releases) for all the details as well as a migration guide.

You can try the new features right now in the browser on the [interactive docs](/docs/getting-started/).

Update your CLI to get started:
```bash
cargo install plyx
```

Go build something smooth.

Feel free to [present your project](https://github.com/TheRedDeveloper/ply-engine/discussions/new?category=show-and-tell) or [ask any questions](https://github.com/TheRedDeveloper/ply-engine/issues/new?template=question.md) on [GitHub](https://github.com/TheRedDeveloper/ply-engine).