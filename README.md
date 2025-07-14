# Panopticum

## Application dependencies
- yt-dlp
- ffmpeg
- System WebView

## Project anatomy
### Stack
- Languages - TypeScript, Rust
- App core - Tauri 2
- App UI - Vue 3, ShadcnVue, Tailwind
- Tools - Vite, Nodemon, Typeshare

### Monorepo map
- core - Application backend, driven by Tauri engine
- ui - Application user interface, driven by Vue 3 framework
- schemas - Reused models and constants, driven by Rust and Typeshare
