use crate::hit::SearchHit;
use crate::recency::recency_bonus;
use app_core::Prompt;
use chrono::Utc;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32String};

/// Synchronous fuzzy search over an in-memory corpus of [`Prompt`]s.
///
/// Title matches are weighted 3x, tags 2x, body 1x; the blended `nucleo` score
/// is added to a 14-day half-life recency bonus (see [`recency_bonus`]). Empty
/// queries return every prompt sorted by `last_used` descending (never-used
/// prompts sort last).
pub struct Search {
    corpus: Vec<IndexedPrompt>,
}

struct IndexedPrompt {
    prompt: Prompt,
    title_h: Utf32String,
    tags_h: Utf32String,
    body_h: Utf32String,
}

impl Search {
    /// Build a fresh `Search` from the given prompts. Pre-computes per-field haystacks.
    pub fn new(prompts: Vec<Prompt>) -> Self {
        let corpus = prompts.into_iter().map(index_prompt).collect();
        Self { corpus }
    }

    /// Replace the entire corpus. Called on PromptStore hot-reload.
    pub fn update(&mut self, prompts: Vec<Prompt>) {
        self.corpus = prompts.into_iter().map(index_prompt).collect();
    }

    /// Run a query against the corpus. Returns hits sorted by score descending.
    /// Empty query is a special case: returns all prompts in `last_used` desc order.
    pub fn query(&self, q: &str) -> Vec<SearchHit> {
        let now = Utc::now();
        if q.trim().is_empty() {
            let mut hits: Vec<SearchHit> = self
                .corpus
                .iter()
                .map(|i| SearchHit {
                    prompt: i.prompt.clone(),
                    score: recency_bonus(now, i.prompt.frontmatter.last_used),
                })
                .collect();
            hits.sort_by(cmp_by_last_used);
            return hits;
        }

        let pattern = Pattern::parse(q, CaseMatching::Smart, Normalization::Smart);
        let mut matcher = Matcher::new(Config::DEFAULT);

        let mut hits: Vec<SearchHit> = Vec::with_capacity(self.corpus.len());
        for entry in &self.corpus {
            let t = pattern
                .score(entry.title_h.slice(..), &mut matcher)
                .unwrap_or(0) as f64;
            let g = pattern
                .score(entry.tags_h.slice(..), &mut matcher)
                .unwrap_or(0) as f64;
            let b = pattern
                .score(entry.body_h.slice(..), &mut matcher)
                .unwrap_or(0) as f64;
            if t + g + b == 0.0 {
                continue;
            }
            let nucleo_score = 3.0 * t + 2.0 * g + 1.0 * b;
            let score = nucleo_score + recency_bonus(now, entry.prompt.frontmatter.last_used);
            hits.push(SearchHit {
                prompt: entry.prompt.clone(),
                score,
            });
        }
        // Sort by score desc; ties broken by recency desc, then id asc — deterministic.
        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| cmp_by_last_used(a, b))
                .then_with(|| a.prompt.id.cmp(&b.prompt.id))
        });
        hits
    }
}

/// Order two hits by `last_used` descending; `None` sorts last.
fn cmp_by_last_used(a: &SearchHit, b: &SearchHit) -> std::cmp::Ordering {
    let a_ts = a.prompt.frontmatter.last_used;
    let b_ts = b.prompt.frontmatter.last_used;
    match (b_ts, a_ts) {
        (Some(bt), Some(at)) => bt.cmp(&at),
        (Some(_), None) => std::cmp::Ordering::Greater, // a has None → after b
        (None, Some(_)) => std::cmp::Ordering::Less,    // b has None → after a
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn index_prompt(prompt: Prompt) -> IndexedPrompt {
    let title_h = Utf32String::from(prompt.frontmatter.title.as_str());
    let tags_joined = prompt.frontmatter.tags.join(" ");
    let tags_h = Utf32String::from(tags_joined.as_str());
    let body_h = Utf32String::from(prompt.body.as_str());
    IndexedPrompt {
        prompt,
        title_h,
        tags_h,
        body_h,
    }
}
