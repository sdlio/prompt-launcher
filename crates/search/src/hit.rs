use app_core::Prompt;

/// A single search hit: the prompt and its blended score.
/// Higher score = better match. The `Search::query` result is sorted descending.
#[derive(Debug, Clone)]
pub struct SearchHit {
    pub prompt: Prompt,
    pub score: f64,
}
