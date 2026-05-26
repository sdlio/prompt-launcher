//! Fuzzy search over an in-memory corpus of prompts. Title-weighted, recency-blended.

mod engine;
mod hit;
mod recency;

pub use engine::Search;
pub use hit::SearchHit;
pub use recency::recency_bonus;
