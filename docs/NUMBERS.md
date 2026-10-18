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
| docs/federation | age-plugin-se PR #1: 5 nodes; root chain 2,440 of 2,600 tokens, 6% slack | 2026-10-18 | in age-plugin-se: `git ls-tree -r --name-only 3080ab9 \| grep -c 'SPEC.md$'`; `sherd budget` at `a601247^`, and that commit's message |
| docs/spec-run | 32 spec-run commits: 7 root sections, 22 nodes | 2026-10-17 | `git log --format=%B \| grep -c '^Paradigm-Stage: spec-run'` |
| docs/spec-run | age-plugin-se PR #1: 22 spec-run commits, 7 root sections, 4 nodes | 2026-10-18 | in age-plugin-se: `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 57ccea1..3080ab9^2 \| grep -c '^spec-run$'`; `docs(spec):` subjects in that range |
| docs/gap-rounds | 7 rounds; round 5 closed 8 open questions | 2026-10-16 | `# Decisions` headings and `Closed by decision 3x/4x` lines in `SPEC-PROMPT.md` |
| docs/gap-rounds | about 15 commits between first spec and frozen prompt | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/gap-rounds | 38 stage lines git could not parse | 2026-10-17 | bug B1 in `SPEC.md` |
| docs/gap-rounds | age-plugin-se PR #1: round 1 in 4 commits, decisions 9 to 20, 6 open questions closed; round 2 in 1 commit, decisions 21 and 22 | 2026-10-18 | in age-plugin-se: `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 57ccea1..3080ab9^2 \| sort \| uniq -c`; `Closed by` lines in `SPEC-PROMPT.md` |
| docs/intent | 28 commits on the prompt before the spec run | 2026-10-16 | `git rev-list --count <first spec-run commit>^ -- SPEC-PROMPT.md` |
| docs/intent | 53 decisions in 7 rounds | 2026-10-16 | numbered entries and `# Decisions` headings in `SPEC-PROMPT.md` |
| docs/intent | 4 rounds and 23 decisions in one private repository | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/intent | age-plugin-se PR #1: 1 intent commit; 8 decisions and 6 open questions in the draft | 2026-10-18 | in age-plugin-se: `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 57ccea1..3080ab9^2 \| grep -c '^intent$'`; numbered entries under `# Decisions` and `# Open questions` in `git show 26bdbe0:SPEC-PROMPT.md` |
| docs/propagation | clap in three private repositories | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/backprop | 3 bugs on the first spec day | 2026-10-17 | §B in `SPEC.md` |
| docs/backprop | about 20 losses; 95x claimed, 2.1x measured | 2026-10-16 | sherd `SPEC.md` B12 and B4 |
| docs/release | pklith 0.1.0, sherd 0.5.4, microlith 0.7.4, itok 0.3.1 | 2026-10-16 | crates.io pages of each crate |
| docs/release | about 250 lines and 36 tests | 2026-10-16 | microlith `SPEC.md` B24 |
| docs/gate | 36 steps, 34 on commit | 2026-10-17 | `pkl eval` of `hooks["check"]` and `hooks["pre-commit"]` in `hk.pkl` |
| docs/gate | 5 red goldens committed | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/gate | age-plugin-se PR #5 (open, head `8b4151a`): 36 steps on commit, 3 on push, 2 on commit-msg; coverage floor 87.67% | 2026-10-18 | in age-plugin-se: `R10` in `git show 8b4151a:SPEC.md`, against `git show 8b4151a:hk.pklith.pkl`; `git show 8b4151a:.coverage` |
| docs/guardrails | 45 guardrails commits; 36 gate steps | 2026-10-17 | `git log --format=%B \| grep -c '^Paradigm-Stage: guardrails'`; `pkl eval -x 'hooks["check"].steps.keys.length' hk.pkl` |
| docs/guardrails | 6 bundled commits; about 70 single-check commits | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/guardrails | age-plugin-se PR #5 (open, head `8b4151a`): 47 guardrails commits; 120 upstream swiftlint findings | 2026-10-18 | in age-plugin-se: `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 152a9cf..8b4151a \| grep -c '^guardrails$'`; `T18` in `git show 8b4151a:SPEC.md` |
| docs/dev-shell | hk 1.58.1, rustc 1.95.0 | 2026-10-17 | `hk --version`, `rustc --version` in the dev shell |
| docs/spec-rounds | 6 violations, 2 drifts, bugs B1 to B3 | 2026-10-17 | §B in `SPEC.md` |
| docs/guardrails-over-watching | about 20 losses of a prose rule | 2026-10-16 | sherd `SPEC.md` B12 |
| docs/spec-before-code | 53 decisions in 7 rounds | 2026-10-16 | `SPEC-PROMPT.md` |
| docs/parallel-agents | 432 processes; load 170 on 10 cores; 19 worktrees, 20 branches; 124 files instead of 21 | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/unflattering-record | 3 bugs; 38 trailers; 95x and 2.1x; 3 false numbers | 2026-10-17 | §B in `SPEC.md`; sherd B4; microlith §G |
| docs/SWIFT.md | Swift 6.2 floor; Swift, swift-format 5.10.1 and swiftlint 0.63.2 in nixpkgs `26f4107e`; Swift 6.3.3 and Command Line Tools 26.6 on the build Mac; hk 2.7.0 with 1 Swift builtin, 0 pklith Swift rows; 4 upstream warnings, now 0 | 2026-10-18 | age-plugin-se `SPEC.md` §R (R2 to R7): `nix eval --raw` of `<pkg>.version` at nixpkgs `26f4107e`, `swift --version`, `pkgutil --pkg-info=com.apple.pkg.CLTools_Executables`, hk `pkl/builtins/`, pklith `src/catalog`, `warning:` lines from `swift build` |
