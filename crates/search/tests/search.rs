use app_core::{Frontmatter, Prompt};
use chrono::{Duration, Utc};
use search::Search;
use std::path::PathBuf;

fn mk_prompt(
    id: &str,
    title: &str,
    tags: &[&str],
    body: &str,
    last_used_days_ago: Option<i64>,
) -> Prompt {
    let last_used = last_used_days_ago.map(|d| Utc::now() - Duration::days(d));
    Prompt {
        id: id.to_string(),
        path: PathBuf::from(format!("/tmp/{id}.md")),
        frontmatter: Frontmatter {
            title: title.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            last_used,
            kind: "paste".to_string(),
        },
        body: body.to_string(),
    }
}

#[test]
fn exact_title_match_outranks_body_match() {
    let prompts = vec![
        mk_prompt("a", "Code review", &[], "do other thing", None),
        mk_prompt("b", "Refactor Rust", &[], "ask for a code review", None),
    ];
    let search = Search::new(prompts);
    let hits = search.query("code review");
    assert!(!hits.is_empty(), "expected matches");
    assert_eq!(hits[0].prompt.id, "a", "title-match prompt should rank first");
}

#[test]
fn recent_outranks_older_for_equal_match() {
    let prompts = vec![
        mk_prompt("old", "Code review", &[], "body", Some(60)),
        mk_prompt("fresh", "Code review", &[], "body", Some(0)),
    ];
    let search = Search::new(prompts);
    let hits = search.query("code review");
    assert!(hits.len() >= 2);
    assert_eq!(hits[0].prompt.id, "fresh", "recent prompt should rank first");
    assert_eq!(hits[1].prompt.id, "old");
}

#[test]
fn empty_query_returns_all_in_last_used_desc_order() {
    let prompts = vec![
        mk_prompt("a", "A", &[], "body", Some(10)),
        mk_prompt("b", "B", &[], "body", Some(1)),
        mk_prompt("c", "C", &[], "body", None), // never used → sorts last
    ];
    let search = Search::new(prompts);
    let hits = search.query("");
    assert_eq!(hits.len(), 3);
    assert_eq!(hits[0].prompt.id, "b", "most-recent first");
    assert_eq!(hits[1].prompt.id, "a");
    assert_eq!(hits[2].prompt.id, "c", "never-used last");
}

#[test]
fn no_match_returns_empty() {
    let prompts = vec![mk_prompt("a", "Code review", &["claude"], "body text", None)];
    let search = Search::new(prompts);
    let hits = search.query("xyzqq-no-such-string");
    assert!(hits.is_empty());
}

#[test]
fn tag_match_works() {
    let prompts = vec![
        mk_prompt("a", "Refactor", &["claude", "review"], "do something", None),
        mk_prompt("b", "Something", &["unrelated"], "do something", None),
    ];
    let search = Search::new(prompts);
    let hits = search.query("claude");
    assert!(!hits.is_empty());
    assert_eq!(hits[0].prompt.id, "a");
}

#[test]
fn update_replaces_corpus() {
    let mut search = Search::new(vec![mk_prompt("a", "A", &[], "body", None)]);
    assert_eq!(search.query("").len(), 1);
    search.update(vec![
        mk_prompt("b", "B", &[], "body", None),
        mk_prompt("c", "C", &[], "body", None),
    ]);
    let hits = search.query("");
    assert_eq!(hits.len(), 2);
}


#[test]
fn empty_query_ties_break_by_id_for_deterministic_ordering() {
    // Two prompts both with last_used = None must sort in a fully deterministic
    // way; the cmp_by_last_used tiebreaker is `id` ascending.
    let prompts = vec![
        mk_prompt("zzz", "Z prompt", &[], "body", None),
        mk_prompt("aaa", "A prompt", &[], "body", None),
        mk_prompt("mmm", "M prompt", &[], "body", None),
    ];
    let search = Search::new(prompts);
    let hits = search.query("");
    assert_eq!(hits.len(), 3);
    // Insertion order was zzz, aaa, mmm. Deterministic id-ascending order is aaa, mmm, zzz.
    assert_eq!(hits[0].prompt.id, "aaa");
    assert_eq!(hits[1].prompt.id, "mmm");
    assert_eq!(hits[2].prompt.id, "zzz");
}
