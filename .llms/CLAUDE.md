# rpg_2026 — Project Guide for Claude Agents

## Goal

Build a 2D RPG game in the spirit of classic top-down Legend of Zelda titles (NES/SNES era). The player will explore overworld maps, enter dungeons, fight enemies, collect items, and interact with NPCs. The project targets both native desktop and web browsers via WebAssembly.

The current codebase is a **placeholder Snake game** used to establish the tech stack and project skeleton. Snake will be replaced as RPG systems are built out.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Language | Rust (Edition 2024) |
| Renderer | [Macroquad](https://macroquad.rs/) 0.4 |
| Build targets | Native binary + `wasm32-unknown-unknown` |
| Web runtime | Macroquad WASM JS bundle (`web/mq_js_bundle.js`) |
| Toolchain | Stable Rust (pinned in `rust-toolchain.toml`) |
| CI / deploy | GitHub Actions → GitHub Pages (`web/`) |

The game window is **400 × 600 px** (portrait). The top 400 × 400 px is the game viewport; the bottom 400 × 200 px is reserved for on-screen control buttons (D-pad + A/B). Both keyboard/mouse (desktop) and touch (mobile/browser) input are supported.

---

## Repository Layout

```
rpg_2026/
├── assets/          # 2D sprites, tilesets, and sound files
├── src/
│   ├── main.rs      # Window setup, main loop, input routing
│   ├── common/      # Reusable game utility libraries (no game logic)
│   └── ui/          # UI layout: game viewport + on-screen buttons
├── web/             # Static web deployment artifacts
│   ├── index.html   # Canvas host page
│   ├── mq_js_bundle.js
│   └── rpg_2026.wasm  (compiled output, not checked in as source)
├── .llms/           # LLM context documents (this file lives here)
├── Cargo.toml
└── rust-toolchain.toml
```

### `src/common/`

Pure utility code with no dependency on game state. Intended as an internal library. Key sub-modules:

- **`num/`** — integer/float vectors (`IVec2`, `FVec2`, etc.), `IRect`, `lerp`, `Range`
- **`data/`** — `Arr2d` (2D grid), `Dir4` (cardinal directions), `DirH`, `Spin`
- **`ds/`** — `Fifo` queue, generic `Queue`
- **`anim/`** — `Animation`, `AnimationMap`, `Animator` (sprite frame sequencing)
- **`map/`** — `Tilemap`, `Tileset`, Tiled-editor JSON deserializer
- **`time/`** — `Countdown`, `Timer`, frame-rate-independent time helpers
- **`mq/`** — thin Macroquad wrappers for drawing, texture loading, and window management

Most `common/` modules are **ready but unused** — they were written in anticipation of RPG features.

### `src/ui/`

Houses the live game code and the on-screen button layout.

- **`game.rs`** — current Snake placeholder; will be replaced by RPG game logic
- **`buttons.rs`** — D-pad and action-button rendering + touch/mouse input

### `assets/`

Holds 2D image and audio files. Currently sparse. As RPG development proceeds, expect sub-directories such as `sprites/`, `tilesets/`, and `sfx/`.

---

## Architecture Notes

- **Input priority**: on-screen buttons are checked before keyboard so that web/mobile users are never blocked.
- **Grid coordinates**: game logic operates in integer tile space; rendering converts to screen pixels.
- **State machine pattern**: game states (e.g., `Playing`, `GameOver`) are modelled as Rust enums.
- **`common/` is side-effect free**: nothing in `common/` calls Macroquad directly except `mq/`, keeping utilities portable and testable.

---

## Development Conventions

- Run `cargo build` for a native debug build.
- Run `cargo build --target wasm32-unknown-unknown --release` to produce the WASM binary.
- Copy the resulting `.wasm` to `web/` for local testing with any static file server.
- Tests live in `common/` under `test/` sub-modules; run with `cargo test`.

---

## Roadmap Context (as of project start)

The Snake placeholder will be progressively replaced with:

1. **Tilemap rendering** — draw overworld maps using `common/map/` + Tiled JSON assets
2. **Player entity** — movement, collision against tiles, sprite animation via `common/anim/`
3. **Camera** — scroll the viewport to follow the player
4. **NPCs & enemies** — simple AI, dialogue system
5. **Items & inventory**
6. **Dungeons & room transitions**
7. **Audio** — sound effects and music via Macroquad audio API

This list is indicative, not prescriptive. Consult open issues and recent commits for the current priority.
