use sherdwright::stage::Stage;

#[test]
fn reads_each_stage_without_a_round() {
    let names = ["intent", "spec-run", "dev-shell", "guardrails", "gate"];
    for name in names
        .into_iter()
        .chain(["release", "backprop", "propagation"])
    {
        let stage: Result<Stage, String> = name.parse();
        assert_eq!(stage.map(|s| s.to_string()), Ok(name.to_string()));
    }
}

#[test]
fn reads_each_numbered_stage_with_its_round() {
    for name in ["gap-round-5", "ck-check-round-1", "wave-12", "reconcile-2"] {
        let stage: Result<Stage, String> = name.parse();
        assert_eq!(stage.map(|s| s.to_string()), Ok(name.to_string()));
    }
    assert_eq!("wave-3".parse::<Stage>(), Ok(Stage::Wave(3)));
}

#[test]
fn refuses_a_name_outside_the_vocabulary() {
    for name in ["gap-round", "wave-", "wave-x", "shipping", "Intent", ""] {
        assert!(name.parse::<Stage>().is_err(), "{name}");
    }
}
