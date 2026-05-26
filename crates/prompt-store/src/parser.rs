use anyhow::{Context, Result};
use app_core::{Frontmatter, Prompt};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use std::path::Path;

/// Parse a single `.md` file into a `Prompt`.
///
/// Returns `Ok(None)` if the file has no leading `---` frontmatter delimiter (treated as
/// a freeform note), `Ok(Some(prompt))` on success, or `Err` on I/O / malformed YAML.
pub fn parse_file(prompts_root: &Path, file_path: &Path) -> Result<Option<Prompt>> {
    let raw = std::fs::read_to_string(file_path)
        .with_context(|| format!("read {}", file_path.display()))?;
    if !raw.trim_start().starts_with("---") {
        return Ok(None);
    }
    let matter = Matter::<YAML>::new();
    let parsed = matter
        .parse::<Frontmatter>(&raw)
        .with_context(|| format!("parse frontmatter for {}", file_path.display()))?;
    let Some(frontmatter) = parsed.data else {
        return Ok(None);
    };
    let rel = file_path.strip_prefix(prompts_root).unwrap_or(file_path);
    let id = rel
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "__");
    Ok(Some(Prompt {
        id,
        path: file_path.to_path_buf(),
        frontmatter,
        body: parsed.content,
    }))
}
