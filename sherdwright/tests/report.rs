use sherdwright::report::{Finding, Report, Verdict};

fn finding(n: u32, verdict: Verdict) -> Finding {
    Finding {
        id: format!("paradigm/V{n}"),
        stage: "any".to_string(),
        verdict,
        reason: format!("reason {n}"),
        evidence: format!("SPEC.md:{n}"),
    }
}

fn every_verdict() -> Report {
    let verdicts = [
        Verdict::Hold,
        Verdict::Waived,
        Verdict::Fail,
        Verdict::Review,
        Verdict::Newer,
    ];
    let findings = (1..).zip(verdicts).map(|(n, v)| finding(n, v)).collect();
    Report { findings }
}

#[test]
fn names_each_verdict_as_the_json_contract_does() {
    let names: Vec<&str> = every_verdict()
        .findings
        .iter()
        .map(|f| f.verdict.name())
        .collect();
    assert_eq!(names, ["hold", "waived", "fail", "review", "newer"]);
}

#[test]
fn counts_the_review_invariants() {
    let mut report = every_verdict();
    assert_eq!(report.review(), 1);
    report.findings.push(finding(9, Verdict::Review));
    assert_eq!(report.review(), 2);
}

#[test]
fn exits_1_only_when_something_fails() {
    assert_eq!(every_verdict().exit_code(), 1);
    let held = Report {
        findings: vec![finding(1, Verdict::Hold), finding(2, Verdict::Review)],
    };
    assert_eq!(held.exit_code(), 0);
    assert_eq!(Report::default().exit_code(), 0);
}

#[test]
fn writes_the_json_contract() {
    let report = Report {
        findings: vec![finding(38, Verdict::Waived)],
    };
    let expected = concat!(
        r#"{"findings":[{"id":"paradigm/V38","stage":"any","verdict":"waived","#,
        r#""reason":"reason 38","evidence":"SPEC.md:38"}],"review":0}"#,
        "\n"
    );
    assert_eq!(report.json(), expected);
}

#[test]
fn writes_an_empty_report_as_json() {
    assert_eq!(Report::default().json(), "{\"findings\":[],\"review\":0}\n");
}

#[test]
fn same_report_gives_the_same_bytes() {
    assert_eq!(every_verdict().json(), every_verdict().json());
    assert_eq!(every_verdict().human(), every_verdict().human());
}

#[test]
fn human_report_lists_each_finding_and_the_review_count() {
    let text = every_verdict().human();
    let expected = "fail    paradigm/V3 (any): reason 3 [SPEC.md:3]";
    assert!(text.lines().any(|l| l == expected), "{text}");
    assert_eq!(text.lines().filter(|l| l.contains("paradigm/V")).count(), 5);
    assert!(
        text.ends_with("5 invariants · 1 fail · 1 review\n"),
        "{text}"
    );
}

#[test]
fn human_report_escapes_terminal_controls() {
    let mut hidden = finding(1, Verdict::Fail);
    hidden.reason = "ok\u{1b}[2K".to_string();
    let text = Report {
        findings: vec![hidden],
    }
    .human();
    assert!(!text.contains('\u{1b}'), "{text:?}");
    assert!(text.contains("ok\\u{1b}[2K"), "{text:?}");
}
