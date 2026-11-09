# SPEC

## §G GOAL

page `docs/waves/README.md` — stage 7: `sherd` orders nodes into waves; each node built by own subagent in own worktree, TDD, merged one per merge.

## §N NAV

rel|path|lens
up|.|-
up|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
self|docs/waves|page `docs/waves/README.md`: `sherd` orders nodes into waves
sib|docs/intent|page `docs/intent/README.md`: a human writes `SPEC-PROMPT.md`
sib|docs/gap-rounds|page `docs/gap-rounds/README.md`: an agent reads the prompt against the fleet & lists gaps
sib|docs/spec-run|page `docs/spec-run/README.md`: `ck:spec` turns the prompt into `SPEC.md`, one section \| node per commit
sib|docs/federation|page `docs/federation/README.md`: the spec splits into sherd nodes, one per directory, so a worker reads one node's chain, ⊥
sib|docs/reconciliation|page `docs/reconciliation/README.md`: every wave closes w/ spec reconciliation (interfaces match what workers built, invented ru
sib|docs/spec-rounds|page `docs/spec-rounds/README.md`: gap & `ck:check` drift rounds on the spec repeat until a round finds nothing
sib|docs/dev-shell|page `docs/dev-shell/README.md`: nix flake + direnv pin every tool
sib|docs/guardrails|page `docs/guardrails/README.md`: guardrails before product code, one check per commit, each proven against a planted violat
sib|docs/gate|page `docs/gate/README.md`: one `hk.pkl` on commit, push & CI
sib|docs/release|page `docs/release/README.md`: `cargo-release`, Keep a Changelog, version ladder
sib|docs/backprop|page `docs/backprop/README.md`: bug after green → §B row + the §V invariant that would have caught it
sib|docs/propagation|page `docs/propagation/README.md`: a fleet-wide decision → issue in each affected repo & promoted into paradigm
sib|docs/guardrails-over-watching|page `docs/guardrails-over-watching/README.md`: nobody watches an agent work
sib|docs/spec-before-code|page `docs/spec-before-code/README.md`: intent, gap rounds & a checked spec come before the first line of code
sib|docs/parallel-agents|page `docs/parallel-agents/README.md`: parallel workers need isolation, budgets & cleanup, & what broke without them
sib|docs/unflattering-record|page `docs/unflattering-record/README.md`: the public defect record & numbers counted in the direction that costs

## §C CONSTRAINTS

- page shape & ledgers per parent `docs` §C.
- explains root invariants, each indexed by root §V & held here in full:
- `paradigm/V13`: waves start only after dev shell & guardrails stages landed. ∵ `.:R7`: guardrails laid late were bundled & unproven.
- `paradigm/V14`: one node per worker, own git worktree, own subagent, TDD RED → GREEN → REFACTOR; merged one node per merge. ∵ merge = review unit; node = isolation unit.
- `paradigm/V15`: workers ⊥ share mutable git state: ⊥ git stash in pre-commit (| hk `stash = "patch-file"`); child git in tests drops `GIT_*`; gate refuses `core.worktree` in shared config; ∀ worker own test DB | scratch dir. ∵ `.:R8`.
- `paradigm/V16`: `.claude/` ignored from first commit. merged wave removes its worktree & branch in same session; gate fails on worktree whose branch already merged. ∵ `.:R12`.
- `paradigm/V17`: worker count sized to machine; heavy steps (mutation, coverage) under load gate, 1 job per worker. ∵ `.:R13`.
- `paradigm/V45`: 1-to-1 mirror: ∀ source file exactly one mirror test at path of member's `exists` row (language page default, e.g. Shell `.:V30`, else member's spec); missing → `exists` fails, test w/o source → `orphan` fails; exempt file = its path in `except` + reason comment; whole rule waived only via `.:V38` (Nix: flake checks stand in). `exists` & `tdd-order` rows ! share select, target & `except`; `orphan` row = their inverse (tests → source); unchecked (`review`) until `docs/waves:T2`. line floor aims 100% (`.:V25`). ∵ coverage % ⊥ names untested file; age-plugin-se PR #16 (2026-10-18). promoted owner 2026-11-07.
- `paradigm/V46`: TDD order: source file changes only in same commit as | after a commit changing its mirror test, within the judged history: `HEAD`'s unpushed commits at pre-push (early warning, ⊥ other refs), PR range in CI (authoritative) ∴ source-only commit after its pushed test fails pre-push, passes CI. merge commits ⊥ judged ∴ conflict resolution editing source → rebase, ⊥ merge commit; unchecked (`review`) until `docs/waves:T2`. pre-gate scaffolds exempt (`.:V24`). ∵ test written after code ⊥ shown to fail; age-plugin-se PR #18, pklith#98. promoted owner 2026-11-07; strict = `.:T25`.

## §I INTERFACES

- page: `docs/waves/README.md`. anchors become public API once a post links them (D11).

## §T TASKS

id|status|task|cites
T1|x|draft page: claim, evidence from root §R & members, how-to, where next|-
T2|.|pklith issue w/ approval: `tdd-order` judges merge commits' own changes; refuse `exists`/`orphan`/`tdd-order` rows w/ differing mappings \| 2 sources → 1 target; selects cover ∀ source; then drop unchecked notes in `.:V45`, `.:V46`|`.:V46`,`.:V23`
