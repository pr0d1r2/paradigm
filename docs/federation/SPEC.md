# SPEC

## §G GOAL

page `docs/federation/README.md` — the spec splits into sherd nodes, one per directory, so a worker reads one node's chain, ⊥ whole repo.

## §N NAV

rel|path|lens
up|.|-
up|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
self|docs/federation|page `docs/federation/README.md`: the spec splits into sherd nodes, one per directory, so a worker reads one node's chain, ⊥
sib|docs/intent|page `docs/intent/README.md`: a human writes `SPEC-PROMPT.md`
sib|docs/gap-rounds|page `docs/gap-rounds/README.md`: an agent reads the prompt against the fleet & lists gaps
sib|docs/spec-run|page `docs/spec-run/README.md`: `ck:spec` turns the prompt into `SPEC.md`, one section \| node per commit
sib|docs/waves|page `docs/waves/README.md`: `sherd` orders nodes into waves
sib|docs/reconciliation|page `docs/reconciliation/README.md`: every wave closes w/ spec reconciliation (interfaces match what workers built, invented ru
sib|docs/spec-rounds|page `docs/spec-rounds/README.md`: gap & `ck:check` drift rounds on the spec repeat until a round finds nothing
sib|docs/dev-shell|page `docs/dev-shell/README.md`: nix flake + direnv pin every tool
sib|docs/guardrails|page `docs/guardrails/README.md`: guardrails before product code, one check per commit, each proven against a planted violat
sib|docs/gate|page `docs/gate/README.md`: one `hk.pkl` on commit, push & CI
sib|docs/release|page `docs/release/README.md`: `cargo-release`, Keep a Changelog, version ladder

## §C CONSTRAINTS

- page shape & ledgers per parent `docs` §C.
- explains root invariants `paradigm/V11`, `paradigm/V12`.

## §I INTERFACES

- page: `docs/federation/README.md`. anchors become public API once a post links them (D11).

## §T TASKS

id|status|task|cites
T1|.|draft page: claim, evidence from root §R & members, how-to, where next|-
