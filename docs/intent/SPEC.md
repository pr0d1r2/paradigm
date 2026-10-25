# SPEC

## §G GOAL

page `docs/intent/README.md` — stage 1: a human writes `SPEC-PROMPT.md` — intent, naming w/ evidence, dated survey, sketches, numbered decisions, prior art. ⊥ code before it.

## §N NAV

rel|path|lens
up|.|-
up|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
self|docs/intent|page `docs/intent/README.md`: a human writes `SPEC-PROMPT.md`
sib|docs/gap-rounds|page `docs/gap-rounds/README.md`: an agent reads the prompt against the fleet & lists gaps
sib|docs/spec-run|page `docs/spec-run/README.md`: `ck:spec` turns the prompt into `SPEC.md`, one section \| node per commit
sib|docs/federation|page `docs/federation/README.md`: the spec splits into sherd nodes, one per directory, so a worker reads one node's chain, ⊥
sib|docs/waves|page `docs/waves/README.md`: `sherd` orders nodes into waves
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
- `paradigm/V1`: ∀ member: `SPEC-PROMPT.md` ∃ in history before first root `SPEC.md`. line 1 = `DRAFT INPUT` from its first commit; flips to `HISTORICAL INPUT` in own commit after spec run. ∵ intent is the human's & ! stay readable; one repo added status only after spec run (prompt format).
- `paradigm/V2`: prompt carries `Paradigm: paradigm <tag>` near top & states deviations only. ⊥ copied roster, ⊥ copied conventions, ⊥ `same as <sibling>`. ∵ sibling prompt frozen while its spec moves ∴ pointer rots; 2 prompts already repeat one roster near word for word.
- `paradigm/V3`: prompt sections in order: status · name & pitch · problem & goals · naming w/ dated evidence & rejected names · dated survey · catalogue pointer? · config sketch · CLI sketch · decision rounds (numbered, dated) · non-goals · prior art & rejected designs · open questions. ∵ fixed order ∴ gap round finds absence by position.
- `paradigm/V4`: ∀ number in prompt | page carries date & recount method. ∵ undated number cannot be re-measured; microlith & sherd §B both record flattering numbers wrong.

## §I INTERFACES

- page: `docs/intent/README.md`. anchors become public API once a post links them (D11).

## §T TASKS

id|status|task|cites
T1|x|draft page: claim, evidence from root §R & members, how-to, where next|-
