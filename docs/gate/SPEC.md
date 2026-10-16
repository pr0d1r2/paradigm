# SPEC

## §G GOAL

page `docs/gate/README.md` — stage 8: one `hk.pkl` on commit, push & CI; ⊥ `--no-verify`; PR-only `main`.

## §N NAV

rel|path|lens
up|.|-
up|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
self|docs/gate|page `docs/gate/README.md`: one `hk.pkl` on commit, push & CI
sib|docs/intent|page `docs/intent/README.md`: a human writes `SPEC-PROMPT.md`
sib|docs/gap-rounds|page `docs/gap-rounds/README.md`: an agent reads the prompt against the fleet & lists gaps
sib|docs/spec-run|page `docs/spec-run/README.md`: `ck:spec` turns the prompt into `SPEC.md`, one section \| node per commit
sib|docs/federation|page `docs/federation/README.md`: the spec splits into sherd nodes, one per directory, so a worker reads one node's chain, ⊥
sib|docs/waves|page `docs/waves/README.md`: `sherd` orders nodes into waves
sib|docs/reconciliation|page `docs/reconciliation/README.md`: every wave closes w/ spec reconciliation (interfaces match what workers built, invented ru
sib|docs/spec-rounds|page `docs/spec-rounds/README.md`: gap & `ck:check` drift rounds on the spec repeat until a round finds nothing
sib|docs/dev-shell|page `docs/dev-shell/README.md`: nix flake + direnv pin every tool
sib|docs/guardrails|page `docs/guardrails/README.md`: guardrails before product code, one check per commit, each proven against a planted violat
sib|docs/release|page `docs/release/README.md`: `cargo-release`, Keep a Changelog, version ladder
sib|docs/backprop|page `docs/backprop/README.md`: bug after green → §B row + the §V invariant that would have caught it
sib|docs/propagation|page `docs/propagation/README.md`: a fleet-wide decision → issue in each affected repo & promoted into paradigm
sib|docs/guardrails-over-watching|page `docs/guardrails-over-watching/README.md`: nobody watches an agent work
sib|docs/spec-before-code|page `docs/spec-before-code/README.md`: intent, gap rounds & a checked spec come before the first line of code

## §C CONSTRAINTS

- page shape & ledgers per parent `docs` §C.
- explains root invariants `paradigm/V23`, `paradigm/V24`, `paradigm/V25`, `paradigm/V26`, `paradigm/V27`, `paradigm/V28`, `paradigm/V29`, `paradigm/V30`, `paradigm/V31`.

## §I INTERFACES

- page: `docs/gate/README.md`. anchors become public API once a post links them (D11).

## §T TASKS

id|status|task|cites
T1|.|draft page: claim, evidence from root §R & members, how-to, where next|-
