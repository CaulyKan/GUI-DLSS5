# DLSS5 Neural Render

Rust + Tauri v2 desktop tool for DLSS Neural Rendering preview and export.

## Build

```powershell
cargo build --release --manifest-path src-tauri\Cargo.toml
```

The executable is written to `src-tauri\target\release\dlss5-tauri.exe`.

After it has been built once, double-click `run.bat` in the project root. It is
ASCII-only and works in legacy Windows PowerShell/CMD code-page environments.
The launcher fixes the working directory so the host and the selected RTX
runtime can be found reliably.

## Included runtimes

- `nvngx_dlssnr.dll` — RTX 50 native runtime
- `nvngx_dlssnr_40.dll` — RTX 40 compatibility runtime
- `nvngx_dlssnr_30.dll` — RTX 30 compatibility runtime

Choose the runtime before the first DLSS preview. The NGX session is process-scoped, so a runtime change after preview creation requires an app restart. If a compatibility DLL is not accepted by the installed driver/GPU, the log reports the exact native error instead of failing silently.


## Interaction

- Import or drag in supported images/videos; paste images from the clipboard.
- Preview rendering is capped to the viewport-friendly 1280px edge to keep memory and interaction responsive; full-resolution processing is used for normal image/video export.
- Switch between original, DLSS, fixed-window split comparison, and A/B layouts.
- Use the mouse wheel to zoom and middle/right drag to pan.
- Set values precisely with sliders or numeric fields.
- Select a destination before image/video export.

Video decoding and encoding use FFmpeg available on the system path.
