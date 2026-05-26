use prompt_store::FsPromptStore;
use std::fs;
use tempfile::TempDir;

fn write_prompt(dir: &std::path::Path, rel: &str, contents: &str) {
    let full = dir.join(rel);
    fs::create_dir_all(full.parent().unwrap()).unwrap();
    fs::write(full, contents).unwrap();
}

#[test]
fn loads_prompts_recursively_with_correct_ids() {
    let dir = TempDir::new().unwrap();
    write_prompt(
        dir.path(),
        "claude/code-review.md",
        "---\ntitle: \"Code review\"\ntags: [claude, review]\n---\n\nReview this code:\n",
    );
    write_prompt(dir.path(), "misc.md", "---\ntitle: \"Misc\"\n---\n\nBody\n");

    let store = FsPromptStore::new(dir.path().to_path_buf()).expect("new ok");
    let prompts = store.list();

    assert_eq!(prompts.len(), 2);
    let ids: Vec<&str> = prompts.iter().map(|p| p.id.as_str()).collect();
    assert!(ids.contains(&"claude__code-review.md"));
    assert!(ids.contains(&"misc.md"));

    let code_review = prompts
        .iter()
        .find(|p| p.id == "claude__code-review.md")
        .unwrap();
    assert_eq!(code_review.frontmatter.title, "Code review");
    assert_eq!(code_review.frontmatter.tags, vec!["claude", "review"]);
    assert_eq!(code_review.frontmatter.kind, "paste");
    assert!(code_review.body.contains("Review this code"));
}

#[test]
fn skips_non_markdown_files_and_files_without_frontmatter() {
    let dir = TempDir::new().unwrap();
    write_prompt(dir.path(), "valid.md", "---\ntitle: \"v\"\n---\nbody\n");
    write_prompt(
        dir.path(),
        "no-frontmatter.md",
        "just body, no frontmatter\n",
    );
    write_prompt(dir.path(), "notes.txt", "not markdown\n");

    let store = FsPromptStore::new(dir.path().to_path_buf()).expect("new ok");
    let prompts = store.list();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].id, "valid.md");
}

#[test]
fn touch_writes_last_used_atomically_and_preserves_body() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("p.md");
    let original = "---\ntitle: \"P\"\ntags: [x]\n---\nbody line 1\nbody line 2\n";
    fs::write(&path, original).unwrap();

    let store = FsPromptStore::new(dir.path().to_path_buf()).unwrap();
    let id = store.list()[0].id.clone();
    store.touch(&id).expect("touch ok");

    let reloaded = fs::read_to_string(&path).unwrap();
    assert!(
        reloaded.contains("last_used:"),
        "expected last_used in: {reloaded}"
    );
    assert!(reloaded.contains("title: P") || reloaded.contains("title: \"P\""));
    assert!(reloaded.contains("body line 1"));
    assert!(reloaded.contains("body line 2"));

    // Re-load and verify last_used parses as a real timestamp via the store.
    let store2 = FsPromptStore::new(dir.path().to_path_buf()).unwrap();
    let p = store2.list().into_iter().find(|p| p.id == id).unwrap();
    assert!(
        p.frontmatter.last_used.is_some(),
        "last_used should round-trip"
    );
}

#[test]
fn touch_unknown_id_returns_error() {
    let dir = TempDir::new().unwrap();
    let store = FsPromptStore::new(dir.path().to_path_buf()).unwrap();
    let result = store.touch("does-not-exist.md");
    assert!(result.is_err());
}
