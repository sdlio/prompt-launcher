# prompt-launcher

Spotlight-style prompt-library launcher for macOS. Global hotkey → fuzzy-search prompts → paste at cursor.

**Status:** Phase 3 complete (loader + search). Phase 1 (Tauri + macos-shim port) is next. Plan: [`docs/plans/v1.md`](docs/plans/v1.md). Greenlit 2026-05-26.

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

1. ⏳ Tauri scaffold + spike port (`macos-shim`). **Next — runs on macOS.**
2. ✅ `~/.prompts/` loader with hot-reload. (commits d3475b6, 48368f7, 1c088cf, 0bee3ee)
3. ✅ `nucleo-matcher` search + CLI smoke binary. (commits 0f70d7d, abf8bc1, b22ccfd)
4. ⏳ Webview UI (vanilla TS + Vite).
5. ⏳ End-to-end paste flow (capture → hide → restore → paste → touch).
6. ⏳ Accessibility onboarding + first-run `~/.prompts/` bootstrap.
7. ⏳ Timing harness + `<200ms` CI gate.
8. ⏳ Polish + signed `.app`.

See [`docs/plans/v1.md`](docs/plans/v1.md) for the full plan including exact file paths, code skeletons, verification gates, and forward-compat seams for v2 (Smithers-style agent actions, Linux/Wayland).

## Resuming on a new machine

If you've just cloned this repo and want to continue the build:

1. Install Rust 1.83 (the `rust-toolchain.toml` will auto-select it if rustup is installed).
2. `cargo build --workspace` — verifies the committed `Cargo.lock` resolves cleanly. Should compile all four lib crates plus the `prompt-launcher` bin stub.
3. `cargo test --workspace` — should be 20 / 20 passing (2 app-core + 7 prompt-store + 4 search-unit + 7 search-integration).
4. Read [`AGENTS.md`](AGENTS.md) for house rules + the locked-in spike findings (mandatory).
5. Read [`docs/plans/v1.md`](docs/plans/v1.md). Find the first `⏳` phase. Pick up there.
6. Phase 1 is the natural next; it requires macOS (the `macos-shim` crate links against `objc2-app-kit`, `enigo`, `arboard`, and the Tauri build needs to produce a `.app` bundle). Phases 4–8 are also macOS-only.

If you're using an AI coding assistant (Claude Code, Codex, etc.) to drive the build, paste `AGENTS.md` + the relevant phase section from `docs/plans/v1.md` into the assistant's context. The plan was written for agent-driven execution.

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
