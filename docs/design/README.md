# Visual design artifacts

This directory holds the **visual reference** for the prompt-launcher overlay.

- **`../DESIGN.md`** — the canonical, machine-readable design spec (Google DESIGN.md format). Tokens, components, contrast rules. **This file is the binding contract.**
- **`overlay-reference.html`** — a self-contained, tweakable HTML proof of the overlay. Lets you flip through every v1 state (default / querying / empty / no-results / accessibility-permission / first-run), toggle dark/light/density, swap accents, and watch the open + close animations. **This file is a proof, not a contract** — when it disagrees with `DESIGN.md`, `DESIGN.md` wins.

## Opening the reference

```bash
open docs/design/overlay-reference.html
```

The Tweaks panel (top-right) lets you:

- Cycle the six v1 states
- Flip dark/light theme + dark/light simulated wallpaper
- Switch density (cozy / compact)
- Swap accent swatches
- Collapse the panel for clean screenshots

The overlay is keyboard-driven exactly like the real app:

| Key             | Action                                            |
|-----------------|---------------------------------------------------|
| Type            | Fuzzy-search prompts (50ms debounce)              |
| `↑` / `↓`       | Move selection                                    |
| `Enter`         | "Paste" — overlay dismisses with the close anim   |
| `Cmd+E` / `Cmd+Enter` | "Edit" — overlay dismisses, simulates `$EDITOR`  |
| `Esc`           | Clear query / dismiss                             |

The reference uses a tiny vanilla JS subsequence matcher to illustrate the
search behavior; the real app uses `nucleo-matcher` (see `crates/search/`).
The score blend (`3·title + 2·tags + 1·body + recency_bonus`) is documented
in both.

## Linting the spec

```bash
npx -y @google/design.md lint ../DESIGN.md
```

CI gate: zero warnings. WCAG AA contrast checks are the load-bearing reason
this file exists, so warnings are not soft signals — they're regressions.

## Exports

The DESIGN.md spec round-trips to two downstream formats, regenerated
on demand:

```bash
npx -y @google/design.md export --format tailwind ../DESIGN.md > ../tailwind.theme.json
npx -y @google/design.md export --format dtcg     ../DESIGN.md > ../tokens.json
```

These are checked in (`docs/tailwind.theme.json`, `docs/tokens.json`) so
diffs to the spec produce visible diffs to the exports — making token
drift impossible to miss in code review.
