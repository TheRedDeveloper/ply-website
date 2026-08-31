+++
title = "Sound"
weight = 18
+++

Ply's audio system is powered by macroquad's built-in audio (quad-snd),
re-exported through the prelude behind the `audio` feature.

## Setup

Enable the `audio` feature in your `Cargo.toml`:

```toml
ply-engine = { version = "1.1", features = ["audio"] }
```

Or add it with the CLI:

```bash
plyx add audio
```

All audio types and functions are available through the prelude.

## Loading sounds

```rust
let click_sound = load_sound("assets/click.wav").await.unwrap();
```

Supported formats: WAV and OGG. Load sounds in your async `main`
before the game loop. You can also load from raw bytes:

```rust
let beep = load_sound_from_bytes(include_bytes!("../assets/beep.wav")).await.unwrap();
```

## Playing sounds

For a one-off sound effect:

```rust
play_sound_once(&click_sound);
```

For more control over volume and looping:

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

## Adjusting volume

Change the volume of a playing sound:

```rust
set_sound_volume(&music, 0.3);
```

## API reference

| Function                       | Description                   |
|--------------------------------|-------------------------------|
| `load_sound(path)`             | Load a sound file (async)     |
| `load_sound_from_bytes(data)`  | Load from raw bytes (async)   |
| `play_sound(sound, params)`    | Play with volume/loop control |
| `play_sound_once(sound)`       | Play once at full volume      |
| `stop_sound(sound)`            | Stop a playing sound          |
| `set_sound_volume(sound, vol)` | Adjust volume (0.0–1.0)       |

## WASM notes

On the web, browsers require user interaction before playing audio. The
audio context is automatically started on the first click/touch. Sounds
queued before that are played once the context activates.

## Next steps

→ [Cross-Platform Builds](/docs/cross-platform/)
