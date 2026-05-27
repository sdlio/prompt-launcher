# Phase 4 prompt for Claude Code (paste this in macOS)

You are continuing work on `prompt-launcher` — a macOS-only Tauri app for hotkey-driven prompt pasting. The repo lives at `~/code/prompt-launcher`. Phases 0, 1, 2, 3 are complete and committed. Your job is **Phase 4 only**: the webview UI.

## Mandatory pre-flight (do these first, in order)

1. `cd ~/code/prompt-launcher`
2. `git status` — confirm a clean tree. If dirty, STOP and ask the user.
3. Read **`AGENTS.md`** (top of repo) — house rules. Especially: TDD, phase-tagged commits, `feat(phase-4): <thing>`, locked spike findings.
4. Read **`docs/plans/v1.md`** lines covering Phase 4 (search for `## Phase 4`). The plan is the source of truth for task structure.
5. Read **`docs/DESIGN.md`** end-to-end. This is the **binding visual contract** for the UI you're building. Tokens, components, contrast rules. WCAG warnings are load-bearing — `npx -y @google/design.md lint docs/DESIGN.md` must stay clean if you change it (you almost certainly should not change it in Phase 4 — just consume it).
6. Read **`crates/app-core/src/prompt.rs`** — the `Prompt` + `Frontmatter` types you'll serialize to JS. Note `id` is path-derived (`claude/code-review.md` → `claude__code-review.md`).
7. Read **`crates/search/src/hit.rs`** and **`crates/search/src/engine.rs`** — `Search::query(&str) -> Vec<SearchHit>` returns `{ prompt, score }` sorted descending. This is what `search` Tauri command wraps.
8. Read **`crates/app-tauri/src/main.rs`** (or wherever Phase 1 landed the hotkey + window-show wiring) — you need to know how the overlay window is registered and how `macos-shim` is reached. The Phase 1 work is your foundation; do not rewrite it.

If any of those files don't exist or look wrong, STOP and ask the user before touching code.

## Visual reference

`docs/design/overlay-reference.html` (in-repo) is a static HTML proof of the overlay design. Open it in a browser to see the target look. The **binding** spec is `docs/DESIGN.md` — the HTML is the visual companion. When the two disagree, the spec wins; the HTML is allowed to drift.

## Phase 4 scope (three tasks, three commits)

### Task 4.1 — Scaffold vanilla TS + Vite frontend

**Create:**
- `ui/package.json` — name `prompt-launcher-ui`, type `module`, scripts `dev`/`build`/`preview`/`typecheck`, devDeps `typescript`, `vite`, `@tauri-apps/api`. Pin versions; no `^`.
- `ui/vite.config.ts` — Vite config. `build.outDir` set so `tauri.conf.json`'s `frontendDist` finds it (likely `../dist/ui` or `dist`; match what `app-tauri/tauri.conf.json` expects — check before guessing).
- `ui/tsconfig.json` — strict, `target` ES2022, `module` ESNext, `moduleResolution` Bundler, `noUncheckedIndexedAccess` true.
- `ui/index.html` — minimal `<html>` shell with a `<div id="app"></div>` and the script tag for `src/main.ts`.
- `ui/src/main.ts` — empty entry that just `console.log("ui boot ok")` for now (the wiring lands in 4.3).
- `ui/src/style.css` — root tokens **copied from `docs/DESIGN.md`** (colors, spacing, type scale, radius). No new tokens; just translate the spec into CSS custom properties. If a token in the spec doesn't have an obvious CSS mapping, ask before inventing.

**Verify before committing 4.1:**
- `cd ui && npm install && npm run build` succeeds.
- `cd ui && npm run typecheck` (run `tsc --noEmit`) succeeds.
- `cargo build --workspace` from repo root still passes (you may need to update `tauri.conf.json` `frontendDist` to point at the new `ui/dist`).

**Commit:** `feat(phase-4): vanilla TS + Vite frontend scaffold`

### Task 4.2 — Three Tauri commands

In `crates/app-tauri/src/commands.rs` (create if absent):

```rust
#[tauri::command]
pub async fn search(state: tauri::State<'_, AppState>, query: String) -> Result<Vec<SearchHitDto>, String>

#[tauri::command]
pub async fn select_prompt(state: tauri::State<'_, AppState>, app: AppHandle, id: String) -> Result<(), String>

#[tauri::command]
pub fn hide_overlay(window: tauri::Window) -> Result<(), String>
```

- **`search`**: delegates to `Search::query`. Map `Vec<SearchHit>` to a `SearchHitDto { id, title, tags, score, preview }` (don't ship full body to JS — preview is first ~200 chars of body). Why a DTO: `Prompt` carries `PathBuf` which doesn't serialize cleanly to JS and the full body is wasted bytes per keystroke.
- **`select_prompt`**: look up the body via `PromptStore`, then for now (Phase 5 wires the real paste flow) — call `hide_overlay` and log `selected: {id}`. **Leave a `// PHASE 5: paste flow lands here` marker.** Phase 5 wires `restore_focus` → `paste_text` → `touch(id)`. Do not implement that here; Phase 5 is a separate commit and the gate requires it staying out of Phase 4.
- **`hide_overlay`**: `window.hide()`. Used by Esc.

Register in `main.rs`:
```rust
.invoke_handler(tauri::generate_handler![commands::search, commands::select_prompt, commands::hide_overlay])
```

**TDD:** for `search` and the DTO mapping, write a unit test in `crates/app-tauri/src/commands.rs` (cfg-test module) with a hand-built `AppState` and a small fixed prompt set. Verify DTO has expected fields, scores sort descending, query "" returns all prompts in recency order (matches `Search::query` semantics).

**Verify before committing 4.2:**
- `cargo test --workspace` green (20 prior tests + your new ones).
- `cargo clippy --workspace --all-targets -- -D warnings` clean.

**Commit:** `feat(phase-4): search/select_prompt/hide_overlay Tauri commands`

### Task 4.3 — UI wires input → search → list → Enter → select_prompt

In `ui/src/main.ts`:
- Top-bar `<input>` with placeholder per `docs/DESIGN.md`.
- `<ul>` list of hits below.
- On input: 50ms-debounced `invoke('search', { query })` → render `<li>` per hit with title + tags pill row.
- Arrow Down/Up: move selection (visual highlight per spec).
- Enter: `invoke('select_prompt', { id: selected.id })`.
- Esc: `invoke('hide_overlay')`.
- Cmd+1..9 (stretch, only if cheap): jump-select rows 1..9.

Styling **must** come from the CSS tokens you ported in 4.1. No magic hex values in this file.

Debounce implementation: simple `setTimeout` + clear. Don't pull in `lodash`. Don't pull in any state library. Plain DOM.

**Verify before committing 4.3:**
- `cd ui && npm run build && npm run typecheck` green.
- `cargo tauri build` produces a bundle. `codesign --force --deep --sign - target/release/bundle/macos/prompt-launcher.app` (per AGENTS.md house rule #3).
- **Manual smoke:** open the bundled `.app`, fire the global hotkey, type a query that matches a known prompt in `~/.prompts/`, see results appear, arrow-key the selection, hit Enter — overlay should close and the console should log the selection (paste flow is Phase 5).

**Commit:** `feat(phase-4): UI wires input → search → list → select`

## Phase 4 gate (per `docs/plans/v1.md`)

> Bundle opens, hotkey shows overlay, typing fuzzy-matches `~/.prompts/`, arrow keys move selection, Enter pastes (using Phase 1's `macos-shim` directly), Esc dismisses.

**Caveat:** the plan says "Enter pastes" at the Phase 4 gate, but the paste *flow* (capture pid → restore focus → paste → touch) is Phase 5's Task 5.1. For Phase 4, "Enter pastes" reduces to "Enter calls `select_prompt`, which logs the selection and closes the overlay." The actual paste happens after Phase 5 lands. If the plan and this prompt disagree, the plan wins and you should ask for clarification before scope-creeping into 5.1.

## What NOT to do

- Don't touch Phases 5–8. Don't pre-stage anything for them beyond the `// PHASE 5:` marker.
- Don't add a frontend framework (React/Vue/Svelte). Plain DOM is the brief.
- Don't invent new design tokens. Consume `docs/DESIGN.md` as-is.
- Don't change the dossier-locked crate choices (`tauri`, `nucleo`, `arboard`, `enigo`, `objc2-app-kit`).
- Don't run `cargo run` on the unbundled binary — synthetic Cmd+V silently fails on Sonoma+ without `.app` bundling + codesign. Always build the bundle for manual smoke.
- Don't `git push`. Local commits only — the user pushes from their machine.

## Reporting back

When all three tasks are committed:
1. `git log --oneline -5` — should show three new `feat(phase-4):` commits on top of `fbbb5e0 docs: add DESIGN.md spec + overlay reference HTML`.
2. `cargo test --workspace` final count.
3. Updated `README.md` phase checklist (Phase 4 → ✅).
4. One-paragraph summary of the manual smoke run: what you typed, what appeared, what happened on Enter/Esc.

If you hit a blocker — missing file from Phase 1, design spec ambiguity, a crate version conflict — STOP and surface it. Don't paper over it.
