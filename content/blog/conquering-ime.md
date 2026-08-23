+++
title = "Conquering IME, Community Projects, and the Road to Ply 2.0"
description = "A massive overhaul for international text input, community showcases, open-sourcing my projects, and GitHub Sponsorships."
date = 2026-08-23
[extra]
author = "RedDev"
+++

It has been over 4 months since the [Ply 1.1 release](/blog/ply-1-1/) in April.

Life got busy in good ways: I went on my senior trip to Berlin, found a boyfriend, moved in with him, started my first job, finished [Flowquill](https://github.com/TheRedDeveloper/flowquill), and went to university orientation camp. Between all that, Ply saw some heavy engineering work.

## Rewriting input methods

Back in April, someone opened an issue showing that typing Chinese characters ended up reversed on Windows.

If you only type in English, input seems straightforward: hit a key, get a character code, draw a glyph. Yet for most of the world, text input relies on multi-stage composition. With Pinyin, Kana-to-Kanji, Hangul, dead keys, or mobile voice-to-text, keystrokes aren't put down immediately. Ply had no real concept of an Input Method Editor (IME).

Over the past four months, I rewrote the OS-level input pipelines. On Windows, that meant dealing with Imm32 and Text Services Framework messages. Linux required supporting both legacy X11 XIM and Wayland `text-input-unstable-v3`. Android and iOS needed native virtual keyboard hooks and predictive text streams. On web, I built an invisible DOM bridge to sync canvas state. For macOS, I worked with a cloud Mac until 4 AM rewriting Cocoa input delegates.

The final diff was +4,884 lines and -1,019 lines, our largest single feature update so far. Input across all six platforms now behaves properly whether you type in English, Chinese, Japanese, or use accent keys.

## What people are building

Seeing what developers build with Ply is easily one of the best parts of working on it.

### Emergency weather tracking (rustywx)

[@kerryhatcher](https://github.com/kerryhatcher) is building [**rustywx**](https://github.com/kerryhatcher/rustywx), a radar and storm-tracking tool for EMA first responders, volunteer firefighters, and storm chasers on field laptops.

This hit close to home recently when I had to evacuate due to a forest fire one kilometer away. Being back home safe, it means a lot to know Ply may be used for real emergencies.

### 3D-printed PCB stencils (vcode)

[@pszsh](https://github.com/pszsh) (Jess) created [**vcode**](https://git.else-if.org/jess/vcode), a tool for 3D printing solder paste stencils. Normal slicers leave messy retraction blobs on tiny surface-mount pads. `vcode` uses Connected Fermat Spirals to generate continuous-line G-code. Jess built the interface in Ply and compiled it for both native desktop and WebAssembly.

### Music notation (Cadenzier)

[@twilit-jack](https://github.com/twilit-jack) is working on [**Cadenzier**](https://codeberg.org/cadenzier/cadenzier), a music notation editor. They have been testing architecture patterns in Ply, finding ways to manage signal bubbling and nested layouts with minimal boilerplate.

## License

The two games I built with Ply are now public on GitHub:

* **[Stratum](https://github.com/TheRedDeveloper/Stratum)**: The turn-based multiplayer strategy game that originally motivated Ply.
* **[Fungal Economics](https://github.com/TheRedDeveloper/FungalEconomics)**: A real-time strategy project centered around mycelial growth and resource networks.

I used the new [Ply Noncommercial License 1](/noncommercial-license/1/) so anyone can study the code, mod it, or run private servers, while keeping commercial forks off the table. If you want concrete examples of shaders, networking, or UI state in Ply, check out the repos.

## Next

Ply 1.2 is coming shortly, and I will do a dedicated release post once it ships.

Looking further out, planning for **Ply 2.0** is underway. The goal is to switch to an independent rendering backend. Dropping upstream dependencies will let us add first-class 3D support, combining Spline-like visual tooling with Ply's immediate-mode ergonomics, 2D/3D compositing, and zero ECS overhead.

## Sponsorships

I plan to keep developing Ply regardless, but balancing school, work, and a cross-platform engine takes time.

I opened **[GitHub Sponsorships](https://github.com/sponsors/TheRedDeveloper)** and **[Kofi](https://ko-fi.com/redderdeveloper)**. If Ply is useful to you or your project, backing it helps me focus more on Ply's development.

## Getting started

If you want to learn more about Ply, you can take a look at the [interactive docs](/docs/getting-started/), or as before you can drop by [GitHub Discussions](https://github.com/TheRedDeveloper/ply-engine/discussions) to share what you are working on.