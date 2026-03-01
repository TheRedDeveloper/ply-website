+++
title = "Building apps in Rust shouldn't be this hard, so I made Ply."
description = "The story behind Ply: Why I built a new app engine in Rust, what I learned from every framework I tried, and what 1.0 looks like."
date = 2026-03-01
[extra]
author = "RedDev"
+++

I wanted to build a game in Rust. A multiplayer board game with a server, a client, and a common crate that holds all the shared game logic. Clean architecture. One language. Full control.

That's when I ran into a wall.

## Everything I tried fell short

[Bevy](https://bevy.org/) crams you into an ECS that turns simple things into thousands of lines of virtual database queries. Its UI system is macro-and-node-based with `impl Bundle` and `..default()` scattered everywhere. Bevy's architecture wouldn't work with what I had spent weeks building for the server.

[Iced](https://iced.rs/) looked promising until I saw the code. `..default()` everywhere. `.into()` on every line. The nesting is unclear and everything reads backwards, where the top element ends up at the bottom of the code.

[egui](https://www.egui.rs/) was better, but you're manually calling `.add_space()` for gaps and allocating rects. For a simple UI it's fine. For a real app, it gets tiring fast.

[Slint](https://slint.dev/) impressed me with its clean nesting, but it's a separate markup language. You can't cleanly integrate it into Rust or connect it to your existing systems. `parent.width` references and `in property <real>` declarations don't belong in a Rust codebase.

I also looked at non-Rust options. I'm an avid Unity developer including VR. I've had experiences with Unigine and opened Unreal once to get confused and irritated. They're all too clunky. They give you zero control. Making a server and Godot work together with a shared crate? Not happening. I also had a terrible previous experience with Tauri trying to make a scooter rental app.

Eventually I found [macroquad](https://macroquad.rs/). It said it would run anywhere, and it felt close to what I wanted, inspired by Love2D's simplicity. But after a few hours, it was clear: if I kept going like this, I wouldn't be done in years. Macroquad is a rendering library, not an app engine. No layout system, no text input, no UI structure at all.

So I needed something on top of it.

## Clay, and hitting the wall

I'd heard about [Clay](https://github.com/nicbarker/clay) from YouTube, a C layout library. I used Rust bindings and paired it with macroquad. I called it Clayquad.

At first, it was great. I could finally build my game at a reasonable speed. Then reality set in.

Bugs appeared everywhere. Use-after-frees. Race conditions in the C bindings. No texture management. I was `Box::leak`ing images every frame just to satisfy the borrow checker. The documentation was sparse, so everything took forever to figure out.

Then I hit hard limits. I wanted shaders. Impossible. I wanted rotation, one of the three fundamental graphics operations, and Clay couldn't do it. Scrolling had to be implemented manually. Text input didn't exist (those are only on, what, 99% of interactive applications?). I couldn't even imagine cross-platform accessibility support.

I had to build something better.

## Porting, rewriting, and rewriting again

I kept building on top of Clayquad. The renderer, a text styling system, vector graphics support. Features kept stacking up through the end of 2025, but the foundation was still C. And the syntax was getting uglier with every feature I added. `.end()` calls everywhere, deep indentation for every nested element, declarations that were painful to read.

I started analyzing every UI framework I could find: Iced, egui, Slint, Bevy, HTML/CSS, Qt/QML. Studying what each one got right and wrong. I knew what the API should look like before I touched any code.

In February I focused on this project. I ported the layout engine to 100% Rust, stayed up until five in the morning getting it working. The next day I implemented the new API I'd been designing. Then came shaders, accessibility, the cli, networking... and this website.

I settled on **builder pattern + closures**. Closures cure the `.end()` problem. Builder methods are cleaner than specifying every property with `..Default::default()`. You can chain `.shader()` calls, choose `.degrees()` or `.radians()`, and everything stays readable.

One `use ply_engine::prelude::*` gives you everything. We use `Into<T>` everywhere. When `.background_color()` accepts `Into<Color>`, it takes hex integers, float tuples, or macroquad colors. When `.image()` accepts `Into<ImageSource>`, it takes file paths, embedded bytes, textures, or vector graphics. No `hex_to_macroquad_color!()` wrappers. No ceremony.

Every second you don't spend looking up how to construct a `FloatingElementBuilder` is a second saved.

## The philosophy

The core principle behind every decision in Ply: **make it easier while giving you full control.**

Why immediate-mode, rebuilding the UI every frame? Because it's actually faster than tracking mutations. No matter how complicated your UI is, the layout takes a fraction of a percent of total frame time, most goes to libnvidia or the GPU. You have to redraw every frame anyway. Love2D already proved this works. Immediate-mode gives you complete control over what gets rendered and when.

Why a single prelude? Because no developer wants to manage imports. One import standardizes what you can do and eliminates useless boilerplate.

Why managers (`TEXTURE_MANAGER`, `MATERIAL_MANAGER`, `FONT_MANAGER`, `NET_MANAGER`)? Because everything runs in a loop, and there are few good ways to persist state between iterations. Back in Clayquad, you had three options for images: always loaded, loaded every frame, or build your own caching system. Ply's managers handle all of that in the background. Tell the engine where your image is, it handles caching, eviction, and lifetime. The same pattern applies to materials, fonts, and network requests. All simplifying memory across frames so you never think about it.

## What 1.0 looks like

Ply 1.0 ships with everything I wished existed when I started:

- **Layout engine** with flexbox-like sizing, padding, gaps, alignment, scrolling, and floating elements
- **Full text input**: cursor, selection, undo/redo, multiline, password mode, all keyboard shortcuts
- **Rich text styling**: inline colors, wave, pulse, gradient, typewriter, shadow, per character
- **GLSL shaders** on any element, with built-in effects and a SPIR-V build pipeline
- **Accessibility** via AccessKit on desktop, JavaScript bridge on web
- **Debug view**: a Chrome DevTools-style inspector. No other Rust UI library has this
- **HTTP + WebSocket** networking that never blocks the UI
- **TinyVG vector graphics** with on-demand rasterization
- **Rotation**: both visual and shape-level
- **Sound** playback
- **Cross-platform**: Linux, macOS, Windows, Android, iOS, and web, all from one codebase

## Interactive docs

Any engine is only as good as its documentation. An engine might have great features, but if it takes you two hours to figure them out, those features are just distractions.

Nobody should need to read as much source code as I did to build something. Nobody should need to make as many pull requests as I did. Everything should be easy to use.

So I built an [interactive documentation](/docs/getting-started/). Live code playgrounds where you can tweak values and see the result instantly. Every concept has an interactive example. The docs teach by doing, not by lecturing.

## Try it

```bash
cargo install plyx
plyx init
```

Two commands to get an app with a font from Google Fonts, feature flags, and a project structure.

Check out the [examples](/examples/), there's a shader playground, a snake game, and a todo app. [On the home page](/) you'll also find an interpreter so you can try some of ply's syntax live. Everything runs in the browser.

Explore the [interactive docs](/docs/getting-started/), they'll show you interactive examples where you can tinker with the code right in the browser. The source is on [GitHub](https://github.com/TheRedDeveloper/ply-engine), licensed under [Zero-Clause BSD](https://opensource.org/license/0bsd). Use it for anything, no attribution required.

Go build something.
