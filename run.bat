@echo off
setlocal
set "APP=%~dp0src-tauri\target\release\dlss5-tauri.exe"
if not exist "%APP%" (
  echo Build missing: src-tauri\target\release\dlss5-tauri.exe
  echo Run: cargo build --release --manifest-path src-tauri\Cargo.toml
  pause
  exit /b 1
)
pushd "%~dp0"
start "" "%APP%"
popd
