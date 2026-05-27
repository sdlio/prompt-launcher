use std::sync::Mutex;

use prompt_store::FsPromptStore;
use search::{Search, SearchHit};
use serde::Serialize;
use tauri::Manager;

/// First N chars of a prompt body sent to the webview. Why a DTO: `Prompt` carries
/// a `PathBuf` (doesn't round-trip cleanly to JS) and the full body is wasted bytes
/// per keystroke. The webview only needs enough text for a one-line preview.
const PREVIEW_LEN: usize = 200;

/// Application state shared across Tauri commands. Built once in `setup` and
/// handed to commands via `tauri::State<'_, AppState>`.
pub struct AppState {
    /// Filesystem-backed prompt library. Kept around so `select_prompt` can
    /// re-`list()` to resolve an id (and Phase 5 will call `touch`).
    pub store: FsPromptStore,
    /// In-memory fuzzy index. Wrapped in a Mutex because `Search::query` takes
    /// `&self` but `Search::update` (hot-reload, Phase 5+) takes `&mut self`.
    pub search: Mutex<Search>,
}

impl AppState {
    pub fn new(store: FsPromptStore) -> Self {
        let prompts = store.list();
        Self {
            store,
            search: Mutex::new(Search::new(prompts)),
        }
    }
}

/// What the webview receives per hit. Mirrors the `SearchHit { prompt, score }`
/// shape but flattens the parts the UI actually renders, and clips the body
/// to a preview.
#[derive(Debug, Clone, Serialize)]
pub struct SearchHitDto {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub score: f64,
    pub preview: String,
}

impl From<SearchHit> for SearchHitDto {
    fn from(h: SearchHit) -> Self {
        Self {
            id: h.prompt.id,
            title: h.prompt.frontmatter.title,
            tags: h.prompt.frontmatter.tags,
            score: h.score,
            preview: preview_of(&h.prompt.body),
        }
    }
}

fn preview_of(body: &str) -> String {
    body.chars().take(PREVIEW_LEN).collect()
}

/// Pure function so the wiring is testable without a Tauri app context.
pub fn run_search(search: &Search, query: &str) -> Vec<SearchHitDto> {
    search
        .query(query)
        .into_iter()
        .map(SearchHitDto::from)
        .collect()
}

#[tauri::command]
pub async fn search(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Result<Vec<SearchHitDto>, String> {
    let guard = state.search.lock().map_err(|e| e.to_string())?;
    Ok(run_search(&guard, &query))
}

#[tauri::command]
pub async fn select_prompt(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    id: String,
) -> Result<(), String> {
    let prompt = state.store.list().into_iter().find(|p| p.id == id);
    let Some(prompt) = prompt else {
        return Err(format!("no prompt with id {id}"));
    };
    eprintln!("selected: {}", prompt.id);
    // PHASE 5: paste flow lands here — capture the pre-show frontmost pid,
    // call macos_shim::restore_focus(pid), macos_shim::paste_text(&prompt.body),
    // then store.touch(&prompt.id). Phase 4 stops at hide + log so the UI loop
    // is exercisable independently.
    if let Some(win) = app.get_webview_window("overlay") {
        win.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn hide_overlay(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use app_core::{Frontmatter, Prompt};
    use chrono::{Duration, Utc};
    use std::path::PathBuf;

    fn make_prompt(id: &str, title: &str, body: &str, hours_ago: Option<i64>) -> Prompt {
        Prompt {
            id: id.into(),
            path: PathBuf::from(format!("{id}.md")),
            frontmatter: Frontmatter {
                title: title.into(),
                tags: vec!["t1".into()],
                last_used: hours_ago.map(|h| Utc::now() - Duration::hours(h)),
                kind: "paste".into(),
            },
            body: body.into(),
        }
    }

    #[test]
    fn dto_carries_id_title_tags_score_preview() {
        let prompts = vec![make_prompt("a", "alpha", "body of alpha", None)];
        let search = Search::new(prompts);
        let hits = run_search(&search, "alpha");
        assert_eq!(hits.len(), 1);
        let h = &hits[0];
        assert_eq!(h.id, "a");
        assert_eq!(h.title, "alpha");
        assert_eq!(h.tags, vec!["t1".to_string()]);
        assert!(h.score > 0.0, "score should be positive on a match");
        assert!(h.preview.contains("body of alpha"));
    }

    #[test]
    fn scores_sort_descending() {
        let prompts = vec![
            make_prompt("a", "code review", "body about code review", None),
            make_prompt("b", "writing tips", "unrelated text", None),
            make_prompt("c", "code", "body", None),
        ];
        let search = Search::new(prompts);
        let hits = run_search(&search, "code review");
        assert_eq!(hits[0].id, "a", "exact title match should top the list");
        for w in hits.windows(2) {
            assert!(
                w[0].score >= w[1].score,
                "scores not monotonic: {} then {}",
                w[0].score,
                w[1].score
            );
        }
    }

    #[test]
    fn empty_query_returns_all_in_recency_order() {
        let prompts = vec![
            make_prompt("old", "old", "x", Some(48)),
            make_prompt("new", "new", "x", Some(1)),
            make_prompt("never", "never", "x", None),
        ];
        let search = Search::new(prompts);
        let hits = run_search(&search, "");
        assert_eq!(hits.len(), 3);
        assert_eq!(hits[0].id, "new", "most recent first");
        assert_eq!(hits[1].id, "old");
        assert_eq!(hits[2].id, "never", "never-used sorts last");
    }

    #[test]
    fn preview_caps_at_200_chars() {
        let big = "x".repeat(500);
        let prompts = vec![make_prompt("a", "alpha", &big, None)];
        let search = Search::new(prompts);
        let hits = run_search(&search, "alpha");
        assert_eq!(hits[0].preview.chars().count(), PREVIEW_LEN);
    }

    #[test]
    fn preview_handles_short_body() {
        let prompts = vec![make_prompt("a", "alpha", "tiny", None)];
        let search = Search::new(prompts);
        let hits = run_search(&search, "alpha");
        assert_eq!(hits[0].preview, "tiny");
    }
}
