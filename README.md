# Bevy Editor with Tauri

Experimental editor with web pased UI, and bevy based viewport.

### Setup

**Prerequisites**

- See https://tauri.studio/docs/getting-started/prerequisites
- WASM target `rustup target install wasm32-unknown-unknown`

Currently, you'll need to clone this repository and run it manually.

1. Install dependencies

```bash
$ yarn install
```

2. Build viewport

```bash
$ yarn build:viewport
```

3. Run app

```bash
$ yarn tauri dev
```
