//! macOS-only shim for frontmost-app capture, focus return, and paste synthesis.
//! Spike-validated 2026-05-26 (~170ms hotkey→paste with 150ms focus-settle).

#[cfg(target_os = "macos")]
mod focus;
#[cfg(target_os = "macos")]
mod paste;

#[cfg(target_os = "macos")]
pub use focus::{capture_frontmost_pid, restore_focus};
#[cfg(target_os = "macos")]
pub use paste::paste_text;
