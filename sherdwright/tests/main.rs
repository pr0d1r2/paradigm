use std::process::Command;

#[test]
fn prints_its_version() {
    let out = Command::new(env!("CARGO_BIN_EXE_sherdwright"))
        .arg("--version")
        .output();
    let text = out
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    assert!(text.starts_with("sherdwright "));
}
