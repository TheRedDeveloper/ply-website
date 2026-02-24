+++
title = "Sound"
weight = 14
+++

Ply uses macroquad's built-in audio system (quad-snd) for sound
playback. Look out for an official Sound API in the future.

You'll need to add quad_snd to your Cargo.toml:

```toml
TODO
```

## Loading sounds

```rust
let click_sound = load_sound("assets/click.wav").await.unwrap();
```

Supported formats: WAV, OGG. Load sounds in your async main before the
game loop.

## Playing sounds

```rust
play_sound(
    &click_sound,
    PlaySoundParams {
        looped: false,
        volume: 0.8,
    },
);
```

## Looping music

```rust
let music = load_sound("assets/music.ogg").await.unwrap();

play_sound(
    &music,
    PlaySoundParams {
        looped: true,
        volume: 0.5,
    },
);
```

## Stopping sounds

```rust
stop_sound(&music);
```

## UI sound example

Play a click when a button is pressed:

```rust
let clicked = ui.element()
    .id("play_btn")
    .width(fixed!(120.0))
    .height(fixed!(40.0))
    .background_color(0xFFC32C)
    .corner_radius(6.0)
    .on_press(|_| {})
    .children(|ui| {
        ui.text("Play", |t| t.font_size(14).color(0x181515));
    });

if ply.is_pressed("play_btn") {
    play_sound(&click_sound, PlaySoundParams { looped: false, volume: 1.0 });
}
```

## WASM notes

On the web, browsers require user interaction before playing audio. The
audio context is automatically started on the first click/touch. Sounds
queued before that are played once the context activates.

## Next steps

→ [Networking](/docs/networking/)
