# prompt-launcher

Spotlight-style prompt-library launcher for macOS. Global hotkey → fuzzy-search prompts → paste at cursor.

**Status:** Phase 0 (scaffolding) · Greenlit 2026-05-26 · Plan: `~/.hermes/plans/2026-05-26-prompt-launcher-v1.md`

## Wedge

You use LLM CLIs (Claude Code, ChatGPT, Codex) all day. You have ~50 prompts you reach for repeatedly. Right now you retype them or hunt through a notes app. This app: hit `Cmd+Shift+Space`, type 3 chars, hit Enter, the prompt is at your cursor — anywhere, any app.

## Architecture

```
+--------------------+   global       +-------------------+   paste    +----------+
| previously-focused | <--hotkey----- |  Tauri overlay    | --Cmd+V--> | original |
| app (Claude, etc.) |                |  (webview UI)     |            | (paste)  |
+--------------------+                +-------------------+            +----------+
                                              |
                                              v
                                      +----------------+
                                      | ~/.prompts/*.md|
                                      | (nucleo search)|
                                      +----------------+
```

## Crates

- `app-core` — shared types, `PromptAction` trait, `Context` struct.
- `macos-shim` — frontmost-app capture, focus return, paste synthesis. macOS-only.
- `prompt-store` — `~/.prompts/` loader, frontmatter parser, hot-reload via `notify`.
- `search` — `nucleo`-backed fuzzy matcher over prompts.
- `app-tauri` — the Tauri binary; wires everything together + webview UI.

## Phases

1. Tauri scaffold + spike port (`macos-shim`).
2. `~/.prompts/` loader with hot-reload.
3. `nucleo` search + CLI smoke binary.
4. Webview UI.
5. End-to-end paste flow.
6. Accessibility onboarding + first-run.
7. Timing harness + `<200ms` CI gate.
8. Polish + signed `.app`.

## Why these choices

- **Tauri 2** validated by spike (`~/spikes/prompt-launcher-core/001-hotkey-paste-loop/`) — ~170ms hotkey→paste, all sub-questions VALIDATED on macOS Sonoma+.
- **macOS-only v1** — spike confirmed macOS; Linux/Wayland deferred (Wayland synthetic-paste story is genuinely hard; see dossier § 8).
- **No UI framework** — webview has ~3 interactive elements; vanilla TS keeps cold-start lean.
- **`nucleo` over `fzf-rs`** — dossier's choice; Helix/Zed-quality matcher with no foreign-process dependency.
- **Mandatory `.app` bundling** — load-bearing spike finding; macOS Sonoma+ silently drops synthetic Cmd+V from bare binaries even with Accessibility granted.

## v2 directions (deferred, seams baked in)

- Kicking off agent workflows (e.g. Smithers) — `PromptAction` trait with `kind: paste | agent` field in frontmatter; v1 ships `PasteAction`, v2 adds `AgentAction`.
- Linux X11 + Wayland.
- Variables (`{{filename}}`, `{{selection}}`).
- Per-app routing.
