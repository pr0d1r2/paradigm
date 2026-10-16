# SPEC

## §G GOAL

`paradigm` — the lifecycle fleet repos are built with by coding agents, written ONCE, versioned by git tag & CHECKED, ⊥ only described. agents write code; guardrails decide what lands; human owns intent, gap answers, diff review & every outward-facing act.

FOCUS (prompt decision 41): sherdified SDD & the INCEPTION that starts it — spec prompt → gap rounds → spec run → federation into sherd nodes → waves of isolated workers → wave-end reconciliation. these stages get the deepest pages, the first invariants & the first checker coverage. fleet conventions stated briefly & cited, ⊥ argued.

SCOPE OF ROOT §V: root §V = the FLEET INVARIANTS, the rules every member keeps, this repo included (prompt decision 1). cited outside this repo as `paradigm/V<n>`. rules only this repo keeps live in its nodes.

LINEAGE: `SPEC-PROMPT.md` (HISTORICAL after this run). where it & this spec differ, this spec wins. prompt decisions cited as `D<n>`.

## §C CONSTRAINTS

- Rust where code needed. one cargo workspace: `dev` (publish = false) & `sherdwright`. `unsafe` forbidden.
- prose for humans in pages; caveman only in `SPEC.md` files (D13). page = claim first, evidence second, how-to last; readable on phone.
- dev shell = nix flake + direnv; tools pinned by `flake.lock`; nixpkgs follows nixpkgs-lock. human reloads shell at each checkpoint.
- gate = one `hk.pkl`, materialized by pklith from `.pklith`, run on commit, push & CI. CI = GitHub Actions rendered by the CI materializer (private repo A; named by role only until D44 commit lands, D53).
- ⊥ `--no-verify`. missing hk refuses. `main` PR-only once `no-commit-to-branch` lands; merge by rebase | merge commit, ⊥ squash.
- atomic commits, one topic each. ∀ commit from gap round 5 on carries `Paradigm-Stage: <stage>` trailer (§I.trailer). subjects | bodies cite `T`/`V`/`B` ids served.
- public by default. private repos ⊥ named in committed text: letter + generic category in prose, count only in registry (D8, D34). local denylist in `.private/`, ⊥ committed.
- links are public API: page path | heading anchor a post links to ⊥ breaks; rename leaves stub pointing to new place (D11).
- numbers: member numbers pulled from member repos, ⊥ typed (D6, D26). until generator owns a number, typed number carries as-of date & numbers-ledger row (D35).
- `paradigm` is never a released package: version = git tag members pin (D4, D27). `sherdwright` may publish under own name, with approval.
- paradigm writes into a member ONLY seeded blocks between markers, ONLY when member runs seed (D29, D30). ⊥ writes code, config or gate steps into members.
- checker deterministic: same member trees + same paradigm version → same report. ⊥ LLM. ⊥ network, except one opt-in verb reading remote repos (D7).
- outward-facing acts need explicit human approval: push to other repos, publish, issues in members, posts.
- not a second gate: paradigm ⊥ runs members' lint | test steps (non-goal). not an agent harness: usage limits & scheduling belong to harness.

## §I INTERFACES

- page: `README.md` — landing page. links every page below. owned here; pages owned by `docs`.
- file: `SPEC.md` root §V — fleet invariant ids, cited `paradigm/V<n>`. ids monotonic, ⊥ reused, ⊥ renumbered.
- tag: `v<semver>` — the paradigm version a member pins (D4). new invariant ⊥ fails an older pin.
- file: `registry/members.toml` — member list (`registry` node, D33).
- cmd: `nix run github:pr0d1r2/paradigm/<tag>#seed -- <lang>` — instruction seed (`seed` node, D36). `<lang>` ∈ `rust` `ruby` `shell` `nix`.
- cmd: `sherdwright` — stage checker, report per invariant hold | waived | fail (`sherdwright` node, D32, D51).
- profile: pklith catalog profile for file-level fleet invariants. lives in pklith, ⊥ here (D32); this repo proposes it.
- trailer: `Paradigm-Stage: <stage>`, `<stage>` ∈ `intent` `gap-round-<n>` `spec-run` `ck-check-round-<n>` `dev-shell` `guardrails` `wave-<n>` `reconcile-<n>` `gate` `release` `backprop` `propagation` (D38).
- file (member side): `.pklith` `exempt` row — waiver of `paradigm/V<n>` with reason (D52).
- marker (member side): seeded block between `<!-- paradigm:begin <block> <tag> -->` & `<!-- paradigm:end <block> -->` (D29).
