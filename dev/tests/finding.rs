use paradigm_dev::finding::Finding;

#[test]
fn renders_path_line_and_what() {
    let finding = Finding::new("a.md", 3, "private name (denylist entry 2)");
    assert_eq!(
        finding.to_string(),
        "a.md:3: private name (denylist entry 2)"
    );
}
