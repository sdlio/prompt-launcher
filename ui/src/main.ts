import { invoke } from "@tauri-apps/api/core";

type Hit = {
  id: string;
  title: string;
  tags: string[];
  score: number;
  preview: string;
};

const KNOWN_KINDS = new Set([
  "claude",
  "codex",
  "chatgpt",
  "shared",
  "writing",
]);

const SEARCH_DEBOUNCE_MS = 50;

const input = document.getElementById("q") as HTMLInputElement;
const list = document.getElementById("list") as HTMLUListElement;
const count = document.getElementById("count") as HTMLSpanElement;

let hits: Hit[] = [];
let selectedIdx = 0;
let debounceTimer: number | undefined;
let inflight = 0;

function pathFromId(id: string): string {
  return id.replace(/__/g, "/");
}

function kindFromId(id: string): string | null {
  const slash = pathFromId(id);
  const top = slash.split("/", 1)[0] ?? "";
  return KNOWN_KINDS.has(top) ? top : null;
}

function iconLetter(id: string, title: string): string {
  const kind = kindFromId(id);
  const source = kind ?? title;
  const ch = source.trim().charAt(0);
  return (ch || "?").toUpperCase();
}

function makeRow(hit: Hit, idx: number): HTMLLIElement {
  const li = document.createElement("li");
  li.className = "row" + (idx === selectedIdx ? " selected" : "");
  li.dataset.idx = String(idx);

  const icon = document.createElement("div");
  icon.className = "row-icon";
  const kind = kindFromId(hit.id);
  if (kind !== null) icon.dataset.kind = kind;
  icon.textContent = iconLetter(hit.id, hit.title);

  const body = document.createElement("div");
  body.className = "row-body";

  const title = document.createElement("div");
  title.className = "row-title";
  title.textContent = hit.title || pathFromId(hit.id);

  const meta = document.createElement("div");
  meta.className = "row-meta";

  const path = document.createElement("span");
  path.className = "path";
  path.textContent = pathFromId(hit.id);
  meta.appendChild(path);

  if (hit.tags.length > 0) {
    const dot = document.createElement("span");
    dot.className = "dot";
    meta.appendChild(dot);
    for (const t of hit.tags) {
      const tag = document.createElement("span");
      tag.className = "tag";
      tag.textContent = t;
      meta.appendChild(tag);
    }
  }

  body.append(title, meta);

  const trail = document.createElement("div");
  trail.className = "row-trail";
  const kbd = document.createElement("span");
  kbd.className = "kbd";
  const kbdEl = document.createElement("kbd");
  kbdEl.textContent = "↵";
  kbd.appendChild(kbdEl);
  trail.appendChild(kbd);

  li.append(icon, body, trail);

  li.addEventListener("mousemove", () => {
    if (selectedIdx !== idx) {
      selectedIdx = idx;
      render();
    }
  });
  li.addEventListener("click", () => {
    selectedIdx = idx;
    void selectCurrent();
  });

  return li;
}

function render(): void {
  const frag = document.createDocumentFragment();
  for (let i = 0; i < hits.length; i++) {
    frag.appendChild(makeRow(hits[i]!, i));
  }
  list.replaceChildren(frag);
  count.textContent = `${hits.length} result${hits.length === 1 ? "" : "s"}`;

  const selectedEl = list.children[selectedIdx] as HTMLElement | undefined;
  if (selectedEl) {
    selectedEl.scrollIntoView({ block: "nearest" });
  }
}

async function runSearch(query: string): Promise<void> {
  const token = ++inflight;
  try {
    const result = await invoke<Hit[]>("search", { query });
    // Drop stale responses if a newer request was fired.
    if (token !== inflight) return;
    hits = result;
    selectedIdx = hits.length === 0 ? 0 : Math.min(selectedIdx, hits.length - 1);
    if (query !== input.value) {
      // Input mutated between debounce schedule and resolution; selection reset.
      selectedIdx = 0;
    }
    render();
  } catch (e) {
    console.error("search failed", e);
    hits = [];
    selectedIdx = 0;
    render();
  }
}

function scheduleSearch(): void {
  window.clearTimeout(debounceTimer);
  debounceTimer = window.setTimeout(() => {
    void runSearch(input.value);
  }, SEARCH_DEBOUNCE_MS);
}

async function selectCurrent(): Promise<void> {
  const hit = hits[selectedIdx];
  if (!hit) return;
  try {
    await invoke("select_prompt", { id: hit.id });
  } catch (e) {
    console.error("select_prompt failed", e);
    return;
  }
  resetForNextShow();
}

async function dismiss(): Promise<void> {
  try {
    await invoke("hide_overlay");
  } catch (e) {
    console.error("hide_overlay failed", e);
  }
  resetForNextShow();
}

function resetForNextShow(): void {
  input.value = "";
  selectedIdx = 0;
  // Pre-warm the recency-ordered list for the next show.
  void runSearch("");
}

input.addEventListener("input", () => {
  selectedIdx = 0;
  scheduleSearch();
});

document.addEventListener("keydown", (e) => {
  if (e.key === "Escape") {
    e.preventDefault();
    void dismiss();
    return;
  }
  if (e.key === "Enter") {
    e.preventDefault();
    void selectCurrent();
    return;
  }
  if (e.key === "ArrowDown") {
    e.preventDefault();
    if (hits.length === 0) return;
    selectedIdx = Math.min(hits.length - 1, selectedIdx + 1);
    render();
    return;
  }
  if (e.key === "ArrowUp") {
    e.preventDefault();
    if (hits.length === 0) return;
    selectedIdx = Math.max(0, selectedIdx - 1);
    render();
    return;
  }
  // Cmd+1..9: jump-select rows 1..9.
  if (e.metaKey && /^[1-9]$/.test(e.key)) {
    const i = Number(e.key) - 1;
    if (i < hits.length) {
      e.preventDefault();
      selectedIdx = i;
      void selectCurrent();
    }
  }
});

// Re-focus the input every time the overlay regains focus (hotkey re-show).
window.addEventListener("focus", () => input.focus());

input.focus();
void runSearch("");

console.log("ui boot ok");
