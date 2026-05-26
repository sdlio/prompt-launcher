use app_core::Frontmatter;

#[test]
fn frontmatter_kind_defaults_to_paste_when_absent() {
    let input = "title: \"hi\"\n";
    let fm: Frontmatter = serde_yaml::from_str(input).expect("parse ok");
    assert_eq!(fm.kind, "paste");
    assert_eq!(fm.title, "hi");
    assert!(fm.tags.is_empty());
    assert!(fm.last_used.is_none());
}

#[test]
fn frontmatter_parses_full_block() {
    let input = "title: \"Code review\"\ntags: [claude, review]\nlast_used: 2026-05-26T00:00:00Z\nkind: agent\n";
    let fm: Frontmatter = serde_yaml::from_str(input).expect("parse ok");
    assert_eq!(fm.title, "Code review");
    assert_eq!(fm.tags, vec!["claude", "review"]);
    assert_eq!(fm.kind, "agent");
    assert!(fm.last_used.is_some());
}
