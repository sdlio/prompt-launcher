# Agent house rules for prompt-launcher

## Authoritative sources

- **Implementation plan:** `~/.hermes/plans/2026-05-26-prompt-launcher-v1.md`
- **Dossier:** `~/.hermes/ideas/research/007-prompt-library-launcher.md`
- **Spike README:** `~/spikes/prompt-launcher-core/001-hotkey-paste-loop/README.md`
- **Backlog entry:** `~/.hermes/ideas/backlog.md` (#007 YAML — `stack_choice`, `macos_integration_shape`, all `known_gotcha:` fields)

Read these before touching code. The spike's `known_gotcha:` fields in the backlog YAML are mandatory reading.

## House rules

1. **TDD always.** Every code-producing task starts with a failing test, then minimal impl. The `test-driven-development` skill applies.
2. **macOS-only v1.** Don't add cross-platform abstractions speculatively. The `PromptAction` and `PromptStore` traits exist as v2 seams; don't expand them.
3. **Mandatory `.app` bundling.** Local dev runs `cargo tauri build && codesign --force --deep --sign - target/release/bundle/macos/prompt-launcher.app`. Bare-binary `cargo run` is **not** supported — synthetic Cmd+V silently fails on Sonoma+ from unbundled binaries.
4. **Phase-tagged commits.** `feat(phase-N): <thing>` or `fix(phase-N): <thing>`. One task → one commit. Phase 0 scaffolding is one bulk commit.
5. **No re-litigating dossier decisions.** Crate choices (`tauri`, `nucleo`, `arboard`, `enigo`, `objc2-app-kit`) are locked by spike + dossier. If you want to swap, raise it as a question — don't unilaterally rewrite.
6. **Wedge discipline.** v1 is "global hotkey → fuzzy search → paste." Anything outside that wedge (variables, per-app routing, agent workflows, Linux) is v2. The forward-compat seams (`PromptAction` trait + `Context` struct + `PromptStore` trait) exist precisely so v2 doesn't need a v1 rewrite — use them; don't expand them.
7. **Never modify `~/.prompts/` files outside the explicit `touch(id)` last-used update.** Users own their prompt library; the app touches `last_used` and that's it.

## Locked-in spike findings (do not re-discover)

- `objc2-app-kit` 0.2 features must include `"libc"` to expose `processIdentifier()`/`runningApplicationWithProcessIdentifier()`.
- `tauri` needs `features = ["macos-private-api"]` for transparent windows.
- `enigo` 0.2: use `Key::Other(9)` (kVK_ANSI_V), **never** `Key::Unicode('v')` (SIGTRAP when Cmd held).
- `NSApplicationActivateIgnoringOtherApps` = `1 << 1`; define manually because 0.2's binding is inconsistent.
- `ActivationPolicy::Accessory` is essential — without it the focus-return illusion breaks.
- Focus-settle delay is 150ms (empirically tuned); 30–60ms produces misses.
- Clipboard restore delay after paste is 200ms.

## Out-of-scope for v1 (do not implement)

- Linux, X11, Wayland.
- Windows.
- Handlebars-style variables (`{{filename}}`, `{{selection}}`).
- Per-app routing (auto-pick Claude prompts when Claude Code is focused).
- Multi-step prompts.
- Agent action kind (Smithers integration). The trait seam exists; **don't** add `AgentAction` impl.
- Sync (git, dedicated service, anything beyond `~/.prompts/`).
- Configurable hotkey, configurable focus-settle delay, configurable storage path. All v1.x.
