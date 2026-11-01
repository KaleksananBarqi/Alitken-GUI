# Alitken Media Converter

A high-performance, borderless desktop media conversion, lossless trimming, and social clip framing utility built on Tauri v2 and Rust.

## Language

**Hybrid Dependency**:
A dependency resolution pattern that probes system PATH binaries first (`ffmpeg`, `magick`) and falls back to portable local downloads in `curlyzed/bin` if missing.
_Avoid_: Bundled binary, static-only dependency

**Startup Geometry**:
The fixed 500×500 px square window state enforced on startup and reverted upon clearing the queue.
_Avoid_: Mini mode, compact view

**Working Geometry**:
The expanded 1060×720 px resizable window state triggered when media files are loaded into the queue.
_Avoid_: Full mode, main window

**Stream Copy**:
A zero-re-encoding pipeline pass-through (`-c copy`) preserving pristine video and audio packets at near-instant speeds.
_Avoid_: Direct copy, raw passthrough

**Service Menu**:
A Linux desktop file action (KDE Dolphin Service Menu or Nautilus Script) providing right-click file conversion directly in file managers.
_Avoid_: Context menu shortcut, SendTo link
