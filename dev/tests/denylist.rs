use paradigm_dev::denylist::Denylist;

#[test]
fn skips_blank_lines_and_comments() {
    let list = Denylist::parse("# names\n\nSecretRepo\n  other-one  \n");
    assert_eq!(list.len(), 2);
}

#[test]
fn matches_case_insensitively_and_numbers_entries_from_one() {
    let list = Denylist::parse("SecretRepo\nother-one\n");
    assert_eq!(list.hit("see the OTHER-ONE here"), Some(2));
    assert_eq!(list.hit("nothing here"), None);
}
