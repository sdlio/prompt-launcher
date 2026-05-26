//! Filesystem-backed prompt store: load ~/.prompts/*.md, atomic touch, hot-reload.

mod fs_store;
mod parser;

pub use app_core::{Frontmatter, Prompt};
pub use fs_store::{Event, FsPromptStore, WatchHandle};
