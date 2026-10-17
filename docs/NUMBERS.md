# Numbers ledger

Every number typed into a page by hand, with the date it was true and how
to count it again. A number pulled from a member repository by a generator
leaves this list; the count of rows here is expected to fall (decision 35
of the [spec prompt](../SPEC-PROMPT.md)).

| page | number | as of | recount |
| --- | --- | --- | --- |
| docs/reconciliation | 6 violations, 2 drifts, 3 bugs in drift round 1 | 2026-10-17 | `Paradigm-Stage: ck-check-round-1` commits and §B in `SPEC.md` |
| docs/waves | 432 hk processes; load 170 on 10 cores; 19 worktrees, 281 MB, 20 branches | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/federation | 21, 22 and 46 nodes in pklith, sherd and xenolith | 2026-10-16 | `find <member> -name SPEC.md -not -path '*/.claude/*'` |
| docs/federation | 23 nodes; root chain 4,207 tokens, deepest 5,062 | 2026-10-17 | `sherd budget` |
| docs/federation | 12 and 7 ceiling changes; 1,700 to 2,100; 12 to 20% slack | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/spec-run | 32 spec-run commits: 7 root sections, 22 nodes | 2026-10-17 | `git log --format=%B \| grep -c '^Paradigm-Stage: spec-run'` |
| docs/gap-rounds | 7 rounds; round 5 closed 8 open questions | 2026-10-16 | `# Decisions` headings and `Closed by decision 3x/4x` lines in `SPEC-PROMPT.md` |
| docs/gap-rounds | about 15 commits between first spec and frozen prompt | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/gap-rounds | 38 stage lines git could not parse | 2026-10-17 | bug B1 in `SPEC.md` |
| docs/intent | 28 commits on the prompt before the spec run | 2026-10-16 | `git rev-list --count <first spec-run commit>^ -- SPEC-PROMPT.md` |
| docs/intent | 53 decisions in 7 rounds | 2026-10-16 | numbered entries and `# Decisions` headings in `SPEC-PROMPT.md` |
| docs/intent | 4 rounds and 23 decisions in one private repository | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/propagation | clap in three private repositories | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/backprop | 3 bugs on the first spec day | 2026-10-17 | §B in `SPEC.md` |
| docs/backprop | about 20 losses; 95x claimed, 2.1x measured | 2026-10-16 | sherd `SPEC.md` B12 and B4 |
| docs/release | pklith 0.1.0, sherd 0.5.4, microlith 0.7.4, itok 0.3.1 | 2026-10-16 | crates.io pages of each crate |
| docs/release | about 250 lines and 36 tests | 2026-10-16 | microlith `SPEC.md` B24 |
| docs/gate | 36 steps, 34 on commit | 2026-10-17 | `pkl eval` of `hooks["check"]` and `hooks["pre-commit"]` in `hk.pkl` |
| docs/gate | 5 red goldens committed | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/guardrails | 45 guardrails commits; 36 gate steps | 2026-10-17 | `git log --format=%B \| grep -c '^Paradigm-Stage: guardrails'`; `pkl eval -x 'hooks["check"].steps.keys.length' hk.pkl` |
| docs/guardrails | 6 bundled commits; about 70 single-check commits | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/dev-shell | hk 1.58.1, rustc 1.95.0 | 2026-10-17 | `hk --version`, `rustc --version` in the dev shell |
| docs/spec-rounds | 6 violations, 2 drifts, bugs B1 to B3 | 2026-10-17 | §B in `SPEC.md` |
| docs/guardrails-over-watching | about 20 losses of a prose rule | 2026-10-16 | sherd `SPEC.md` B12 |
