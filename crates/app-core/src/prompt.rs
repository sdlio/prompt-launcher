use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Frontmatter parsed from a `~/.prompts/*.md` file.
///
/// `kind` defaults to `"paste"` so existing prompt files without a `kind:` field load
/// as paste actions. v2 will introduce additional kinds (e.g. `"agent"` for Smithers-style
/// agent workflow triggers) — the field is reserved so the v1 → v2 transition is additive.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Frontmatter {
    pub title: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub last_used: Option<DateTime<Utc>>,
    #[serde(default = "default_kind")]
    pub kind: String,
}

fn default_kind() -> String {
    "paste".to_string()
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            tags: Vec::new(),
            last_used: None,
            kind: default_kind(),
        }
    }
}

/// A loaded prompt: frontmatter + body + stable id derived from its file path.
#[derive(Debug, Clone)]
pub struct Prompt {
    /// Stable id derived from the path relative to the prompts root, with `/` -> `__`.
    /// e.g. `claude/code-review.md` -> `claude__code-review.md`.
    pub id: String,
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    pub body: String,
}
