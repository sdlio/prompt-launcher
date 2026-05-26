---
version: alpha
name: Prompt Launcher
description: Spotlight-style overlay for a personal prompt library. Calm, dense, keyboard-first; macOS-vibrancy aesthetic.

colors:
  # Brand
  primary:        "#3D6FE8"   # accent — the *only* hue used for interaction
  primary-strong: "#2F5BD0"   # accent on hover / pressed (darkens to keep WCAG AA on white text)
  primary-on:     "#FFFFFF"   # ink that sits on top of primary fills
  primary-tint:   "#5B8CFF"   # display-only accent — selection fills, focus glow, brand dot, match-highlight ink

  # Surface
  surface:        "#222226"   # opaque fallback for the overlay (real app uses vibrancy)
  surface-raised: "#2B2B2F"   # row hover
  surface-selected: "#2F3956" # row selected (= accent ~22% over surface)
  surface-tag:    "#2F2F33"   # tag chip background
  surface-match:  "#34446B"   # fuzzy-match highlight background

  # Ink (text on dark surface)
  ink:            "#F6F6F6"   # primary text — titles, input, big state copy
  ink-muted:      "#ABABAD"   # secondary text — status hints, sub copy
  ink-faint:      "#7F7F81"   # tertiary — paths, timestamps, kbd hints at rest

  # Hairlines / borders
  hairline:        "#343437"   # row dividers, sub-component borders
  hairline-strong: "#414144"   # outlined affordances (e.g. ⌘E kbd chip)

  # Kind chips (left-rail row icons; map 1:1 to top-level subdirectory in ~/.prompts/)
  kind-claude:    "#E88A4D"
  kind-codex:     "#6FD06F"
  kind-chatgpt:   "#59C1A5"
  kind-shared:    "#B8A4FF"
  kind-writing:   "#FFD166"

typography:
  # Single family for everything except mono. macOS native first; Inter is a
  # documented web-rendered fallback if the webview can't reach SF Pro.
  display:
    fontFamily: "-apple-system, SF Pro Display, Inter, sans-serif"
    fontSize: 15px
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: "-0.01em"
  input:
    fontFamily: "-apple-system, SF Pro Text, Inter, sans-serif"
    fontSize: 19px
    fontWeight: 400
    lineHeight: 1.2
    letterSpacing: "-0.005em"
  body:
    fontFamily: "-apple-system, SF Pro Text, Inter, sans-serif"
    fontSize: 14px
    fontWeight: 500
    lineHeight: 1.25
    letterSpacing: "-0.005em"
  meta:
    fontFamily: "-apple-system, SF Pro Text, Inter, sans-serif"
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.2
  hint:
    fontFamily: "-apple-system, SF Pro Text, Inter, sans-serif"
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.2
  mono:
    fontFamily: "ui-monospace, SF Mono, JetBrains Mono, Menlo, monospace"
    fontSize: 11px
    fontWeight: 500
    fontFeature: "tnum"
  tag:
    fontFamily: "-apple-system, SF Pro Text, Inter, sans-serif"
    fontSize: 10.5px
    fontWeight: 500
    letterSpacing: "0.01em"
  brand:
    fontFamily: "-apple-system, SF Pro Text, Inter, sans-serif"
    fontSize: 11px
    fontWeight: 700
    letterSpacing: "0.04em"

rounded:
  # Discrete radii — no continuous scale. Reach for one of these four.
  xs: 2px      # match-highlight background
  sm: 4px      # kbd chip
  md: 7px      # CTA button, segmented control
  lg: 8px      # row, row-icon
  xl: 14px     # the overlay itself

spacing:
  # 4px base unit. Use these names, not raw pixel values, in component padding.
  xs: 4px
  sm: 6px
  md: 8px
  lg: 12px
  xl: 14px
  xxl: 16px
  xxxl: 18px
  xxxxl: 22px

components:
  # ── The overlay shell ──────────────────────────────────────────
  overlay:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.xl}"
    width: 600px
    height: 400px

  # ── Search input ──────────────────────────────────────────────
  input:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    typography: "{typography.input}"
    padding: 18px

  input-placeholder:
    textColor: "{colors.ink-faint}"
    typography: "{typography.input}"

  # ── Result row ────────────────────────────────────────────────
  row:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink}"
    rounded: "{rounded.lg}"
    padding: 10px
    height: 52px

  row-hover:
    backgroundColor: "{colors.surface-raised}"

  row-selected:
    backgroundColor: "{colors.surface-selected}"
    textColor: "{colors.ink}"

  row-title:
    textColor: "{colors.ink}"
    typography: "{typography.body}"

  row-meta:
    textColor: "{colors.ink-faint}"
    typography: "{typography.meta}"

  row-icon:
    backgroundColor: "{colors.surface-tag}"
    textColor: "{colors.ink-muted}"
    rounded: "{rounded.md}"
    size: 26px

  # ── Tag chip ──────────────────────────────────────────────────
  tag:
    backgroundColor: "{colors.surface-tag}"
    textColor: "#B2B2B3"
    rounded: "{rounded.xs}"
    typography: "{typography.tag}"
    padding: 1px

  # ── Keyboard chip (kbd) ───────────────────────────────────────
  kbd:
    backgroundColor: "{colors.hairline}"
    textColor: "#C1C1C2"
    rounded: "{rounded.sm}"
    typography: "{typography.mono}"
    padding: 1px

  kbd-outlined:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink-muted}"
    rounded: "{rounded.sm}"
    typography: "{typography.mono}"
    padding: 1px

  # ── Status row ────────────────────────────────────────────────
  status:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.ink-muted}"
    typography: "{typography.hint}"
    padding: 8px

  # ── CTA (used in onboarding + first-run states) ───────────────
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.primary-on}"
    rounded: "{rounded.md}"
    typography: "{typography.body}"
    padding: 8px

  button-primary-hover:
    backgroundColor: "{colors.primary-strong}"
    textColor: "{colors.primary-on}"

  # ── Onboarding badge (Accessibility / Welcome chips above the CTA) ──
  badge-accent:
    backgroundColor: "{colors.surface-selected}"
    textColor: "#9BB6FF"
    rounded: "{rounded.md}"
    typography: "{typography.brand}"
    padding: 3px

  # ── Decorative accent (selection fills, brand dot, focus glow) ──
  # Reference-only component so the linter sees primary-tint is used.
  decorative-accent:
    backgroundColor: "{colors.primary-tint}"
    rounded: "{rounded.xs}"

  # ── Tweaks panel keycap separator (uses hairline-strong) ──
  divider-strong:
    backgroundColor: "{colors.hairline-strong}"
    height: 1px

  # ── Kind chips (reference components so all 5 hues are spec-tracked) ──
  # The runtime row-icon swaps `backgroundColor` to a 18%-alpha mix of the
  # kind hue and `textColor` to the hue itself. These reference components
  # keep the kind palette anchored in the spec.
  kind-icon-claude:
    backgroundColor: "{colors.surface-tag}"
    textColor: "{colors.kind-claude}"
    rounded: "{rounded.md}"
    size: 26px

  kind-icon-codex:
    backgroundColor: "{colors.surface-tag}"
    textColor: "{colors.kind-codex}"
    rounded: "{rounded.md}"
    size: 26px

  kind-icon-chatgpt:
    backgroundColor: "{colors.surface-tag}"
    textColor: "{colors.kind-chatgpt}"
    rounded: "{rounded.md}"
    size: 26px

  kind-icon-shared:
    backgroundColor: "{colors.surface-tag}"
    textColor: "{colors.kind-shared}"
    rounded: "{rounded.md}"
    size: 26px

  kind-icon-writing:
    backgroundColor: "{colors.surface-tag}"
    textColor: "{colors.kind-writing}"
    rounded: "{rounded.md}"
    size: 26px

  # ── Fuzzy-match highlight ─────────────────────────────────────
  match-highlight:
    backgroundColor: "{colors.surface-match}"
    textColor: "#9BB6FF"
    rounded: "{rounded.xs}"
    typography: "{typography.body}"
---

## Overview

The Prompt Launcher overlay is a single keyboard-first surface that appears
when the user fires `Cmd+Shift+Space` from any macOS app, and disappears the
moment they hit `Enter` to paste — or `Cmd+E` to open the prompt in
`$EDITOR`. Everything in this file exists to serve one rule:

**The overlay must feel instant. Visual style never argues with speed.**

Aesthetic posture: macOS Sonoma. Vibrancy-tinted surface, a single accent
hue (`#5B8CFF`), monochrome-by-default ink, type-as-hierarchy, no
decoration. Anything that would slow first-paint or distract from the
search input has been deliberately omitted: no shadows on rows, no icons
on the input, no headings, no helper illustrations, no gradient
backgrounds inside the overlay itself.

This DESIGN.md is the source of truth. The reference implementation at
`/Users/<you>/.../prompt-launcher-design/Prompt Launcher Overlay.html` is
a visual proof and tweakable playground; the tokens here are what gets
ported to the webview `style.css`.

## Colors

The palette is intentionally small: **two accent tokens, one surface family,
three inks, five kind-chips**. Don't add new hues.

- **Primary `#3D6FE8`** — the interactive accent used wherever white text
  sits on the accent fill. CTA buttons, primary-action surfaces in
  onboarding. This is darker than the reference HTML's bright blue
  because **white text on the brighter shade fails WCAG AA contrast**
  (3.16:1 vs. the required 4.5:1). Darkening to `#3D6FE8` brings it to
  4.81:1 — passes AA for body text.
- **Primary-strong `#2F5BD0`** — *darker* hover/pressed state for
  primary-filled buttons. Hover *darkens*, never brightens; brightening
  the accent on hover would put white-on-blue contrast below AA. This
  matches macOS native button conventions (`NSButton` accent style
  darkens on press).
- **Primary-tint `#5B8CFF`** — the cheerful bright blue from the
  reference HTML, used **only** for decoration: row-selection fill (at
  22% alpha, so contrast doesn't apply), focus-glow shadow, the brand
  dot in the status row, and the *ink* of the match-highlight (which
  sits on `surface-match` and gets bumped to `#9BB6FF` for AA). The
  brightness is the personality of the app — keeping it for decoration
  means we don't lose the launcher's character to accessibility.
- **Primary-on `#FFFFFF`** — text that sits on primary fills.
- **Surface `#222226`** — the opaque fallback for the overlay. In the
  real macOS build the overlay is **NSVisualEffectView with
  `behindWindow` material**, so this hex is what the surface *resolves
  to* when vibrancy is unavailable (Reduce Transparency, screenshots,
  certain HDR contexts). All component tokens reference this so the dark
  fallback stays cohesive.
- **Ink** triplet — `#F6F6F6 / #ABABAD / #7F7F81`. These are the
  composited equivalents of `rgba(255,255,255, 0.96 / 0.62 / 0.42)` over
  surface. The runtime CSS uses the alpha forms so they read correctly
  over vibrancy *and* the solid fallback; the spec keeps the resolved
  hexes for contrast checking.
- **Kind chips** — `claude / codex / chatgpt / shared / writing` map 1:1
  to the **top-level subdirectory** in `~/.prompts/`. Any other
  subdirectory falls back to `ink-muted` for the chip background. Don't
  add new kind colors without adding a corresponding kind in the search
  crate.

### The two-accent split is load-bearing — don't collapse it

If you find yourself reaching for the bright `primary-tint` in a button
fill, stop. The reference HTML used `#5B8CFF` everywhere and it failed
WCAG AA for white-on-accent (3.16:1). The fix is **two tokens, two
jobs**: `primary` for fills that carry text, `primary-tint` for
decoration. The hues are nearly indistinguishable in actual use because
selection fills are at 22% alpha and the brand dot is 8 px.

### Vibrancy & translucency at runtime

The component tokens above use hex (opaque). The runtime CSS uses
`rgba()` so the surface picks up vibrancy from `NSVisualEffectView`
behind the webview. The translation is mechanical:

| Token            | Spec hex   | Runtime rgba                    |
|------------------|------------|---------------------------------|
| `surface`        | `#222226`  | `rgba(28,28,32,0.78)` + blur 38px |
| `hairline`       | `#343437`  | `rgba(255,255,255,0.08)`        |
| `ink`            | `#F6F6F6`  | `rgba(255,255,255,0.96)`        |
| `ink-muted`      | `#ABABAD`  | `rgba(255,255,255,0.62)`        |
| `surface-selected` | `#2F3956` | `color-mix(in oklch, var(--accent) 22%, transparent)` |

When in doubt, **use the runtime alpha form in CSS**; the hex in this
spec is for spec validation (WCAG contrast checks) and downstream
exports (Tailwind, DTCG).

### Light theme (deferred — v1.x)

A light theme is implemented in the reference HTML's Tweaks panel for
exploration only. **v1 ships dark-only.** When light lands as a real
shipping option in v1.x, override these tokens; everything else holds:

| Token        | Light value |
|--------------|-------------|
| `surface`         | `#F8F8FA` |
| `ink`             | `#141414` |
| `ink-muted`       | `#707070` |
| `ink-faint`       | `#9F9FA0` |
| `hairline`        | `#E7E7E8` |
| `hairline-strong` | `#D5D5D7` |
| `row-hover`       | `#EFEFF1` |

Primary and kind colors stay the same in both themes.

## Typography

Single sans family across the whole UI: **`-apple-system`** in the macOS
build (resolves to SF Pro Display for the bigger sizes, SF Pro Text for
body and below). `Inter` is the documented web fallback for the
exploratory HTML artifact only — production webview *will* have SF Pro.

Monospace is reserved for **three things and three things only**:

1. Filesystem paths in row metadata (`claude/code-review.md`)
2. The `kbd` chips in the status row and selected-row trail
3. The result count in the status row (`tnum` for stable digit width)

If you find yourself reaching for `mono` for anything else — body copy,
labels, headings — you've made a mistake. Use the sans + size hierarchy
instead.

Sizes follow a tight, deliberate scale: **11 / 12 / 14 / 15 / 19 px**.
Five steps cover every text element in the overlay. Don't introduce a
sixth without a written reason.

Weights: **400 / 500 / 600 / 700**. 400 for input + meta, 500 for row
titles + body, 600 for big-state headings ("No prompts yet"), 700 only
for the all-caps brand mark in the status row.

## Layout

The overlay is a **fixed 600×400 px frameless window**, centered on the
active display. This size came out of the spike and is non-negotiable
for v1 — it's the size that fits "one search input, ~6 visible rows, one
status row" comfortably on every Retina + non-Retina display we tested.

Internal layout is three rigid rows, top to bottom:

| Region   | Height | Purpose                              |
|----------|--------|--------------------------------------|
| Input    | ~58 px | Search glyph + input + `esc` pill    |
| List     | flex   | Scrollable result rows               |
| Status   | ~34 px | Brand mark · kbd hints · count       |

Density modes are **cozy (default)** and **compact**. Compact lowers
input/row padding by ~30% and shrinks the input size from 19 → 17 px.
There are exactly two density modes; resist the urge to add a third.

### Spacing scale

4 px base. Reach for `xs (4) / sm (6) / md (8) / lg (12) / xl (14) /
xxl (16) / xxxl (18) / xxxxl (22)`. Anything else is a smell.

- **Row inner padding**: `10px 16px` (cozy), `7px 16px` (compact)
- **Input outer padding**: `18px 22px` (cozy), `14px 20px` (compact)
- **Status outer padding**: `8px 14px`
- **Row gap**: `2px` — rows are visually adjacent, separation comes from
  hover/select fills, not whitespace.

### Scroll

The list region uses `scrollbar-gutter: stable`; the scrollbar is 8 px
wide, `hairline-strong` thumb, no track. Scroll happens on `ArrowDown`
past the last visible row, never in response to scroll wheels alone (the
overlay should *never* take wheel focus when the user wasn't reaching for
it).

## Elevation & Depth

The overlay has **one elevation layer**. Don't stack cards.

The macOS window-level shadow handles everything visually; we add a
single subtle inner highlight (`box-shadow: 0 1px 0 rgba(255,255,255,0.05)
inset`) and an outer hairline (`0 0 0 1px rgba(0,0,0,0.55)`) to keep the
overlay from dissolving into bright wallpapers. Drop shadows on rows,
chips, the CTA, or any other internal element are **forbidden**.

The exception: the **CTA button** in onboarding / first-run gets a tight
`6px 16px -6px color-mix(in oklch, var(--accent) 60%, transparent)` glow.
It's the only element that earns lift.

Vibrancy backdrop: `blur(38px) saturate(180%)`. These two numbers were
tuned against macOS Sonoma's wallpaper aesthetics — don't change them
without rerunning the eyeball test against at least three wallpapers.

## Shapes

Four radii cover everything:

| Radius      | Used by                                |
|-------------|----------------------------------------|
| `xs` (2 px) | Match-highlight pill                   |
| `sm` (4 px) | `kbd` chips                            |
| `md` (7 px) | CTA button, segmented control buttons  |
| `lg` (8 px) | Result rows, row icons, scrollbar thumb|
| `xl` (14 px)| The overlay itself                     |

Tags use `rounded.xs` (3 px in CSS — close enough; the spec rounds to xs).
Don't reach for radius values outside this set.

## Components

The component table in the front matter is the contract. A few items
need extra context:

### `row` vs `row-hover` vs `row-selected`

Only `row-selected` shows the `↵` and `⌘E` trail chips at full opacity.
On `row-hover` they stay invisible — hover is for "I'm looking at this"
not "I'm about to act on this". The keyboard caret (selection) is the
load-bearing affordance; mouse hover is a secondary nicety.

### `button-primary` is exclusively for onboarding

The only place a primary CTA appears in v1 is the two onboarding states:
**Accessibility-permission** and **first-run bootstrap**. The main flow
has no buttons because keyboard is the only input. If a future feature
wants a button inside the main flow, escalate it — it's an architecture
question, not a UI one.

### `kbd` vs `kbd-outlined`

- `kbd` (filled) — the canonical keycap. Used in the status row, hint
  lists, and the selected-row `↵` chip.
- `kbd-outlined` — only used for the **secondary action `⌘E`** on the
  selected row, where two adjacent kbd chips would otherwise weigh the
  same. The outlined variant says "this is the secondary action" without
  shouting it.

### Match-highlight is type *and* background

`<span class="match">` gets `primary-strong` ink **and** `surface-match`
background. Either alone would be less legible; both together pop without
shouting. Don't use just one.

## Do's and Don'ts

**Do**

- Use one accent. If a new feature needs color, it gets accent or it
  gets ink. Anything else is a mistake.
- Reach for type hierarchy (size + weight) before reaching for color,
  borders, or icons. Five sizes and four weights are already too many; we
  don't need a sixth or a fifth.
- Test every visual change against at least: dark wallpaper, light
  wallpaper, Reduce Transparency on, Reduce Motion on. The reference
  HTML's Tweaks panel exists to make this easy.
- Match the spike's timing constants — `150 ms` focus-settle, `200 ms`
  clipboard restore. Visual transitions for the overlay itself must be
  ≤ `140 ms` (open) and ≤ `110 ms` (close). Anything longer makes the
  paste feel laggy.
- Respect `prefers-reduced-motion`. Both the `pop` open animation and the
  `dismiss` close animation must collapse to opacity-only (or none) when
  the user has reduced motion on.
- Use the `mono` font for paths, kbd, and tabular numbers. Nothing else.
- Bump `last_used` *after* paste, never before. The recency-sort on the
  next open reflects what the user did.

**Don't**

- **Don't add a sixth color.** Loyalty oaths included.
- **Don't add icons to the input row, status row, or rows themselves**
  (the kind-chip is a colored letter, not an icon — it's just typography
  in a colored box). If you find yourself sketching an icon, sketch a
  type weight instead.
- **Don't show a toast on paste.** The reference HTML has one only so
  reviewers can see what happened in a static demo. In the real app the
  overlay just hides; the user sees the text appear in the previously-
  focused app and that's the only confirmation needed.
- **Don't animate row selection.** `ArrowDown` must move the selection
  instantly. Smooth-scroll the row into view if it's offscreen; never
  fade the selected-row fill.
- **Don't introduce a second elevation layer** (no popovers, no tooltips,
  no inline expandables). If a feature needs more than the three rigid
  layout regions, escalate it.
- **Don't reach for non-Apple fonts in production.** Web fonts are
  permission to fail — the overlay should *never* show a font flash on
  the critical paste path.
- **Don't make the overlay resizable, draggable, or persistent.** The
  overlay is summoned by hotkey, used in ≤ 5 seconds, and dismissed.
  Anything that violates that loop is out of scope.
