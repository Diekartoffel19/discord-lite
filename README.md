# discord-lite

A tiny Discord desktop wrapper for Windows, written in Rust.

Instead of bundling its own Chromium like the official Electron client, it opens
`https://discord.com/app` in the system's **WebView2** runtime (the Edge engine that
ships with Windows) using [`wry`](https://github.com/tauri-apps/wry) and
[`tao`](https://github.com/tauri-apps/tao). The executable is ~500 KB.

## Features

- Native window running Discord's web app
- Persistent login (profile stored in `%LOCALAPPDATA%\discord-lite`)
- Chromium flags that cut background work (`--disable-background-networking`, `--process-per-site`, ...)
- Links that open a new window go to your default browser
- No console window

## Build & run

Requires [Rust](https://rustup.rs) and the WebView2 runtime (preinstalled on Windows 10/11).

```
cargo build --release
target\release\discord-lite.exe
```

## Limitations

Same as Discord in a browser:

- Push-to-talk only works while the window is focused
- No tray icon, overlay, game activity or autostart
- Memory use depends on WebView2; compare in Task Manager for your setup

## Disclaimer

Not affiliated with or endorsed by Discord Inc. It only loads the official web app.
