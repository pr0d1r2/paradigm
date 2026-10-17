use paradigm_dev::message::{check, cites, stage};

const BODY: &str = "ci(x): add a step (T4, V21)\n\nWhy it matters.\n";

const STAGES: [&str; 11] = [
    "intent",
    "gap-round-5",
    "spec-run",
    "ck-check-round-1",
    "dev-shell",
    "guardrails",
    "wave-2",
    "reconcile-2",
    "gate",
    "release",
    "backprop",
];

#[test]
fn accepts_each_stage_of_the_vocabulary() {
    for name in STAGES.into_iter().chain(["propagation"]) {
        let line = format!("Paradigm-Stage: {name}\n");
        assert_eq!(stage(&line), Ok(name.to_string()));
    }
}

#[test]
fn refuses_a_stage_outside_the_vocabulary() {
    assert!(stage("Paradigm-Stage: gap-round\n").is_err());
    assert!(stage("Paradigm-Stage: shipping\n").is_err());
}

#[test]
fn refuses_trailers_without_a_stage() {
    assert!(stage("Co-Authored-By: someone\n").is_err());
}

#[test]
fn finds_bare_and_node_qualified_ids() {
    assert!(cites("fix: thing (T4)"));
    assert!(cites("see dev:T7 and V43."));
    assert!(cites("B12, then"));
    assert!(!cites("Typo in a TV show, version 2"));
}

#[test]
fn passes_a_message_with_a_stage_and_an_id() {
    assert_eq!(check(BODY, "Paradigm-Stage: guardrails\n"), Ok(()));
}

#[test]
fn refuses_a_message_that_cites_no_id() {
    let text = "ci(x): add a step\n\nWhy it matters.\n";
    assert!(check(text, "Paradigm-Stage: guardrails\n").is_err());
}

#[test]
fn passes_git_s_own_subjects() {
    assert_eq!(check("Merge branch 'x'\n", ""), Ok(()));
    assert_eq!(check("fixup! ci(x): y\n", ""), Ok(()));
}

#[test]
fn ignores_comment_lines_and_the_scissors() {
    let text =
        "ci(x): y\n# T9 in a comment\n# ------------------------ >8 ------------------------\nV3\n";
    assert!(check(text, "Paradigm-Stage: gate\n").is_err());
}
