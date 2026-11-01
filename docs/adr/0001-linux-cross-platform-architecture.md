# 0001. Cross-Platform Linux Support Architecture

Linux support for Alitken (targeting Fedora/RPM and universal AppImage) is implemented using zero-regression conditional compilation (`#[cfg(target_os = ...)]`) and hybrid external dependencies.

## Context
Alitken was originally built exclusively for Windows 10/11 with native DWM acrylic effects, Windows Media Foundation (WMF), and hardcoded `.exe` binary paths in `%LOCALAPPDATA%\curlyzed\bin`. To bring Alitken to Linux (specifically Fedora/KDE/GNOME) while keeping upstream pull request compatibility, we require a clean cross-platform backend and build pipeline without breaking Windows invariants.

## Decision
1. **Hybrid Dependency Model**: Check system `PATH` first (`/usr/bin/ffmpeg`, `/usr/bin/magick`). If present and valid (version 5.0+), use them directly. If missing, fall back to downloading portable Linux binaries into `~/.local/share/curlyzed/bin`.
2. **GPU Acceleration Detection**: Probe graphics hardware via `/sys/class/drm` and `lspci` to dispatch NVENC on NVIDIA and VA-API / QSV on Intel/AMD, with fallback to `libx264` or `libopenh264`.
3. **Packaging Strategy**: Target native Fedora `.rpm` and universal `.AppImage` via Tauri v2 bundler and a dedicated `build.sh` script.
4. **Desktop Integration**: Replace Windows SendTo shortcuts with XDG Desktop Entry MIME associations and file manager integration (KDE Dolphin Service Menus & Nautilus Scripts).
5. **Upstream PR Readiness**: Maintain 100% branding and GPLv3 licensing, isolating Linux-specific logic behind `#[cfg(target_os = "linux")]` and `#[cfg(unix)]`.
