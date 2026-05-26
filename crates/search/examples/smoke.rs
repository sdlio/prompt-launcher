//! Run: `cargo run -p search --example smoke -- "your query here"`
//!
//! Smoke-tests the synchronous `Search` API against the user's real `~/.prompts/` dir.
//! Not shipped in the Tauri bundle — purely for dogfooding ranking quality.

use anyhow::{anyhow, Result};
use prompt_store::FsPromptStore;
use search::Search;
use std::env;

fn main() -> Result<()> {
    let query: String = env::args().skip(1).collect::<Vec<_>>().join(" ");

    let home = dirs::home_dir().ok_or_else(|| anyhow!("could not resolve home dir"))?;
    let prompts_root = home.join(".prompts");

    if !prompts_root.exists() {
        println!("No ~/.prompts/ directory found. Create some .md files there to try this out.");
        return Ok(());
    }

    let store = FsPromptStore::new(prompts_root)?;
    let prompts = store.list();
    if prompts.is_empty() {
        println!("~/.prompts/ is empty (no .md files with valid frontmatter).");
        return Ok(());
    }

    let search = Search::new(prompts);
    let hits = search.query(&query);

    println!("Query: {query:?}    ({} hits)", hits.len());
    println!();
    println!(
        "{:>8}  {:<40}  {:<30}  {:<25}",
        "SCORE", "TITLE", "ID", "LAST_USED"
    );
    println!(
        "{:>8}  {:<40}  {:<30}  {:<25}",
        "-----", "-----", "--", "---------"
    );
    for hit in hits.iter().take(10) {
        let title = trim_to(&hit.prompt.frontmatter.title, 40);
        let id = trim_to(&hit.prompt.id, 30);
        let last_used = hit
            .prompt
            .frontmatter
            .last_used
            .map(|t| t.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
            .unwrap_or_else(|| "—".to_string());
        println!(
            "{:>8.2}  {:<40}  {:<30}  {}",
            hit.score, title, id, last_used
        );
    }
    Ok(())
}

fn trim_to(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_string();
    }
    let mut out: String = s.chars().take(n.saturating_sub(1)).collect();
    out.push('…');
    out
}
