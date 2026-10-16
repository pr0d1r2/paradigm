# SPEC

## §G GOAL

public pages that posts link to (D9, D10). reader arrives cold from one link on one page ∴ every page stands alone: claim, why, evidence, where next.

## §F FEDERATION

dir|owns|⊥owns|tokens
intent|page `docs/intent/README.md`: a human writes `SPEC-PROMPT.md`|other pages (siblings), the rules (root §V)|-
gap-rounds|page `docs/gap-rounds/README.md`: an agent reads the prompt against the fleet & lists gaps|other pages (siblings), the rules (root §V)|-
spec-run|page `docs/spec-run/README.md`: `ck:spec` turns the prompt into `SPEC.md`, one section \| node per commit|other pages (siblings), the rules (root §V)|-
federation|page `docs/federation/README.md`: the spec splits into sherd nodes, one per directory, so a worker reads one node's chain, ⊥|other pages (siblings), the rules (root §V)|-
waves|page `docs/waves/README.md`: `sherd` orders nodes into waves|other pages (siblings), the rules (root §V)|-
reconciliation|page `docs/reconciliation/README.md`: every wave closes w/ spec reconciliation (interfaces match what workers built, invented ru|other pages (siblings), the rules (root §V)|-
spec-rounds|page `docs/spec-rounds/README.md`: gap & `ck:check` drift rounds on the spec repeat until a round finds nothing|other pages (siblings), the rules (root §V)|-
dev-shell|page `docs/dev-shell/README.md`: nix flake + direnv pin every tool|other pages (siblings), the rules (root §V)|-
guardrails|page `docs/guardrails/README.md`: guardrails before product code, one check per commit, each proven against a planted violat|other pages (siblings), the rules (root §V)|-
gate|page `docs/gate/README.md`: one `hk.pkl` on commit, push & CI|other pages (siblings), the rules (root §V)|-
release|page `docs/release/README.md`: `cargo-release`, Keep a Changelog, version ladder|other pages (siblings), the rules (root §V)|-
backprop|page `docs/backprop/README.md`: bug after green → §B row + the §V invariant that would have caught it|other pages (siblings), the rules (root §V)|-
propagation|page `docs/propagation/README.md`: a fleet-wide decision → issue in each affected repo & promoted into paradigm|other pages (siblings), the rules (root §V)|-
guardrails-over-watching|page `docs/guardrails-over-watching/README.md`: nobody watches an agent work|other pages (siblings), the rules (root §V)|-
spec-before-code|page `docs/spec-before-code/README.md`: intent, gap rounds & a checked spec come before the first line of code|other pages (siblings), the rules (root §V)|-
parallel-agents|page `docs/parallel-agents/README.md`: parallel workers need isolation, budgets & cleanup, & what broke without them|other pages (siblings), the rules (root §V)|-
unflattering-record|page `docs/unflattering-record/README.md`: the public defect record & numbers counted in the direction that costs|other pages (siblings), the rules (root §V)|-

## §N NAV

rel|path|lens
up|.|-
self|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children

## §C CONSTRAINTS

- page shape: claim first, evidence second (numbers pulled from members, commit links, member links), how-to last. prose, ⊥ caveman. readable on phone (D13).
- every page states which root invariants it explains, by `paradigm/V<n>`.
- stage & topic pages live in child nodes as `docs/<node>/README.md`. inception children written first (root §G FOCUS).
- language pages share headings: stack; dev shell; federation; gate map; mirror tests; coverage; commit vs push; waves; release; gaps against Rust flow; anti-patterns.
- page in `POSTS.md` ⊥ removed, only superseded; renamed page | anchor leaves stub (D11, D12).
- typed number carries as-of date & `NUMBERS.md` row (D35).

## §I INTERFACES

- page: `docs/PARADIGM.md` — essay, short, why (D37).
- page: `docs/SPEC-PROMPT.md` — prompt format & template (`.:V1`–`.:V3`).
- page: `docs/MEMBERS.md` — rendered from `registry/members.toml`, numbers generated.
- page: `docs/RUST.md` (reference) · `docs/RUBY.md` · `docs/SHELL.md` · `docs/NIX.md` — language pages.
- page: `docs/LEGACY.md` — lefthook migration path & concept map to pklith catalog candidates (D46).
- page: `docs/LLM-DISCLAIMER.md` — this repo's public defect record (`.:V33`).
- file: `docs/POSTS.md` — posts ledger (D12). `docs/NUMBERS.md` — numbers ledger (D35).

## §V INVARIANTS

V1: ∀ page opens w/ its claim in first paragraph & ends w/ where to go next.
V2: ∀ page in `POSTS.md` resolves at its posted path & anchors (via stub if moved).
V3: language pages carry the same headings in the same order.

## §T TASKS

id|status|task|cites
T1|.|`PARADIGM.md` essay|V1
T2|.|`SPEC-PROMPT.md` format page & template|V1
T3|.|`RUST.md` reference page; align existing `RUBY.md` `SHELL.md` `NIX.md` headings|V3
T4|.|`MEMBERS.md` from registry|V1
T5|.|`LEGACY.md`|V1
T6|.|`LLM-DISCLAIMER.md`, `POSTS.md`, `NUMBERS.md` skeletons|V2
