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

## §R RESEARCH

id|topic|finding|src
R1|fleet survey 2026-10-16|335 dirs · 213 `flake.nix` · 24 `Cargo.toml` · 146 `lefthook.yml` · 36 `hk.pkl` · 123 root `SPEC.md` · 36 `.context-limits` · 16 `.pklith` · 35 `AGENTS.md` · 19 `docs/LLM-DISCLAIMER.md` · 2 `SPEC-PROMPT.md`. upper bounds: backups & experiments counted. recount: `find` over sibling checkouts|`SPEC-PROMPT.md` Fleet survey
R2|federation depth 2026-10-16|node `SPEC.md` count, worktrees excluded: pklith 21 · sherd 22 · characterminator 21 · rekall 23 · xenolith 46 · microlith 3 · itok 1. one repo holds 437 `SPEC.md` copies under `.claude/worktrees/`|`SPEC-PROMPT.md` Fleet survey
R3|public members 2026-10-16|11 public repos follow the lifecycle; §V/§B summed over nodes, gate steps from `hk.pkl`, merged PRs from `gh pr list --state merged` (merge commits undercount: rebase merges). numbers move to `registry` & generator (D6)|`SPEC-PROMPT.md` Members
R4|lifecycle order in A & B|prompt → naming → survey → decision rounds (A 4 rounds, 23 decisions) → catalogue → format vendored → ck:spec one section per commit → `fix(spec): pass mth check` → dev shell & guardrails first → crate seed → `dev/` → federation → waves → dogfood|private, anonymized
R5|gap rounds after spec are real|B: ~15 commits between root spec & prompt marked historical|private, anonymized
R6|ck:check rounds leave no named commit|visible only as `docs(spec): round N interfaces match what the workers built`, backfilled invariants, `docs(node): plan X` ∴ D38 trailer|private, anonymized
R7|guardrails first|B bundled guardrails into 6 commits; A laid ~70, one check each, planted-violation proof each, before first product RED at ~commit 98|private, anonymized
R8|parallel workers broke hooks|scratch git inherited hook `GIT_*` → 432 hk processes; scratch `git init` wrote `core.worktree` into shared `.git/config`; hk stash shared across worktrees moved files between workers (also R)|private, anonymized
R9|gate ran zero steps|Pkl generator passed `pkl eval` & `hk validate`, hk evaluated empty pre-commit; noticed only from "no steps to run"|private, anonymized
R10|tests moved to pre-push|5 red goldens committed in A|private, anonymized
R11|ceiling churn|`.context-limits` changed 12× in A, 7× in B; one ceiling 1700 → 2100. slack that held: 12–20% over measured|private, anonymized
R12|worktrees never cleaned|A: 19 worktrees (281 MB) & 20 branches; R: 10 wave worktrees 35–662 commits behind; S2: 5 unignored → `SPEC.md` glob 124 ≠ 21|private, anonymized
R13|mutation per worker|load 170 on 10 cores (R)|private, anonymized
R14|cross-node bug|same flaky-test cause found twice in different nodes of B; §B row ⊥ reaches siblings|private, anonymized
R15|`.coverage` meanings|ratchet floor in 4 public members, tree-hash cache in 3|`SPEC-PROMPT.md` Members
R16|prose rules lose|sherd §B12: rule living only in prose lost ~20× in one session ∴ runners beat prose|sherd `SPEC.md` §B
R17|`sherdwright` availability 2026-10-16|free on crates.io, npm, PyPI, RubyGems, Homebrew core, Hex, NuGet, AUR, Debian, nixpkgs; no GitHub user or org; 0 repos named so; no DNS for `.com` `.dev` `.io` `.org`|`SPEC-PROMPT.md` Naming
