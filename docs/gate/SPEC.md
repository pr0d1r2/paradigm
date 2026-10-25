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
sib|docs/parallel-agents|page `docs/parallel-agents/README.md`: parallel workers need isolation, budgets & cleanup, & what broke without them
sib|docs/unflattering-record|page `docs/unflattering-record/README.md`: the public defect record & numbers counted in the direction that costs

## §C CONSTRAINTS

- page shape & ledgers per parent `docs` §C.
- explains root invariants, each indexed by root §V & held here in full:
- `paradigm/V23`: one `hk.pkl` on commit, push & CI. missing hk refuses. ⊥ `--no-verify`. `main` PR-only after `no-commit-to-branch`; merge by rebase | merge commit, ⊥ squash. ∵ one gate definition ∴ callers cannot drift; squash erases the commit trail `.:V27` needs.
- `paradigm/V24`: test stage declared, ⊥ discovered: tests on pre-push; commit red on purpose says `RED` in subject; scaffolds laid before TDD-order gate listed in exemption list w/ reasons. ∵ `.:R10`.
- `paradigm/V25`: `.coverage` = ratchet floor, one meaning fleet-wide; caches use another name. ∵ `.:R15`.
- `paradigm/V26`: gate suppliers run hk too; lefthook leaves fleet. lefthook-gated repo = registry status `legacy`, checked only for migration invariants (D25, D46). ∵ supplier w/o gate = unguarded root of every gate.
- `paradigm/V27`: commit subject | body cites the `T`/`V`/`B` ids it serves, checked from the commit that brings hk on; earlier commits were ungated & ⊥ judged retroactively. ∵ spec → commit trail searchable; `.:B3`.
- `paradigm/V28`: CI = GitHub Actions running same `hk.pkl`, rendered by actlith; member needs GitHub remote. ∀ member builds & tests on `aarch64-darwin` `x86_64-darwin` `x86_64-linux` `aarch64-linux`: pre-push native on ∀ owner laptop, CI the rest. macOS floor 14.0 (Rust `.cargo/config.toml` `[env] MACOSX_DEPLOYMENT_TARGET = { value = "14.0", force = true }`, Swift `.macOS(.v14)`), newer API behind `@available` w/ clean error. test-stage step (`.:V24`), pre-push | release: both Mac archs' release builds `lipo`-joined = exactly arm64 + x86_64, each `minos 14.0`. owner 2026-10-24, was "on 3 platforms" (#2). ∵ D45; both Mac archs are owner dev hosts ∴ real hardware runs each.
- `paradigm/V29`: argument parsing via clap. ∵ D47: hand-parsed args in every public member drift from help text.
- `paradigm/V30`: Shell: one mirrored `.bats` per script; kcov per-script floor on Linux CI only, macOS skips w/ stated reason. ∵ D48.
- `paradigm/V31`: Ruby mutation score = weekly scheduled ratchet outside hk, ⊥ per worker. ∵ D49, `.:R13`.
- `paradigm/V44`: RED commit compiles: tests land w/ stubs that answer wrong, ∵ pre-commit clippy builds `--all-targets` & refuses a test that ⊥ compiles. ∵ wave 1 (dev, sherdwright).

## §I INTERFACES

- page: `docs/gate/README.md`. anchors become public API once a post links them (D11).

## §T TASKS

id|status|task|cites
T1|x|draft page: claim, evidence from root §R & members, how-to, where next|-
