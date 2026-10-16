# SPEC

## §G GOAL

`paradigm` — the lifecycle fleet repos are built with by coding agents, written ONCE, versioned by git tag & CHECKED, ⊥ only described. agents write code; guardrails decide what lands; human owns intent, gap answers, diff review & every outward-facing act.

FOCUS (prompt decision 41): sherdified SDD & the INCEPTION that starts it — spec prompt → gap rounds → spec run → federation into sherd nodes → waves of isolated workers → wave-end reconciliation. these stages get the deepest pages, the first invariants & the first checker coverage. fleet conventions stated briefly & cited, ⊥ argued.

SCOPE OF ROOT §V: root §V = the FLEET INVARIANTS, the rules every member keeps, this repo included (prompt decision 1). cited outside this repo as `paradigm/V<n>`. rules only this repo keeps live in its nodes.

LINEAGE: `SPEC-PROMPT.md` (HISTORICAL after this run). where it & this spec differ, this spec wins. prompt decisions cited as `D<n>`.

## §F FEDERATION

dir|owns|⊥owns|tokens
docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children|landing `README.md` (root), member data (`registry`)|-

## §N NAV

rel|path|lens
up|-|-
self|.|-

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

## §V INVARIANTS

Fleet invariants. Each: `[stage · checker]`, rule, `∵` reason. checker ∈ `sherdwright` (history & trailers) · `pklith` (file-level profile) · a roster tool · `review` (no runner yet ∴ counted unchecked, V41).

V1: [intent · sherdwright] ∀ member: `SPEC-PROMPT.md` ∃ in history before first root `SPEC.md`. line 1 = `DRAFT INPUT` from its first commit; flips to `HISTORICAL INPUT` in own commit after spec run. ∵ intent is the human's & ! stay readable; one repo added status only after spec run (prompt format).
V2: [intent · sherdwright] prompt carries `Paradigm: paradigm <tag>` near top & states deviations only. ⊥ copied roster, ⊥ copied conventions, ⊥ "same as <sibling>". ∵ sibling prompt frozen while its spec moves ∴ pointer rots; 2 prompts already repeat one roster near word for word.
V3: [intent · sherdwright] prompt sections in order: status · name & pitch · problem & goals · naming w/ dated evidence & rejected names · dated survey · catalogue pointer? · config sketch · CLI sketch · decision rounds (numbered, dated) · non-goals · prior art & rejected designs · open questions. ∵ fixed order ∴ gap round finds absence by position.
V4: [intent · review] ∀ number in prompt | page carries date & recount method. ∵ undated number cannot be re-measured; microlith & sherd §B both record flattering numbers wrong.
V5: [gap-round · sherdwright] gap answers append as new dated round of numbered decisions. earlier decision edited only to ALIGN & names the amending decision. ⊥ silent reversal, ⊥ renumber. ∵ decisions cited by number from commits, specs & pages.
V6: [gap-round · review] ∀ decision tagged local | candidate. candidate promoted into paradigm after member ships, ⊥ before. ∵ unshipped rule = guess.
V7: [spec-run · sherdwright] spec run ⊥ starts while open question blocking a node is open; asks instead of guessing. ∵ guessed answer lands as law.
V8: [spec-run · sherdwright] spec run lands `SPEC.md` one section | node per commit, trailer `spec-run`; prompt flip to HISTORICAL in own commit after. ∵ R4: section-per-commit is how A & B landed specs reviewably.
V9: [spec-run · mth, sherd] after ∀ `SPEC.md` edit: `mth fmt --check`, `mth check`, `sherd check`, `sherd validate`, `sherd budget` exit 0. ∵ spec is the worker's input ∴ broken spec = broken wave.
V10: [spec-round · sherdwright] gap & drift rounds on spec (gap pass + `ck:check`) repeat until a round finds nothing. ∀ round leaves commit w/ trailer `ck-check-round-<n>` | `gap-round-<n>`; clean round leaves empty commit. ∵ R6: rounds otherwise invisible in history.
V11: [federation · sherd] federation by directory: ∀ node own `SPEC.md` w/ §F & §N; §N generated (`sherd sync`), ⊥ hand-edited. node spec + budget + `.context-limits` row bound what its worker reads. ∵ worker context = node chain, ⊥ whole repo.
V12: [federation · itok] ceiling = measured + 12–20% slack. raise = own reviewed commit w/ measurement; sherdify before raising. ∵ R11.
V13: [waves · sherdwright] waves start only after dev shell & guardrails stages landed. ∵ R7: guardrails laid late were bundled & unproven.
V14: [waves · sherdwright] one node per worker, own git worktree, own subagent, TDD RED → GREEN → REFACTOR; merged one node per merge. ∵ merge = review unit; node = isolation unit.
V15: [waves · pklith] workers ⊥ share mutable git state: ⊥ git stash in pre-commit (| hk `stash = "patch-file"`); child git in tests drops `GIT_*`; gate refuses `core.worktree` in shared config; ∀ worker own test DB | scratch dir. ∵ R8.
V16: [waves · pklith] `.claude/` ignored from first commit. merged wave removes its worktree & branch in same session; gate fails on worktree whose branch already merged. ∵ R12.
V17: [waves · review] worker count sized to machine; heavy steps (mutation, coverage) under load gate, 1 job per worker. ∵ R13.
V18: [reconcile · sherdwright] ∀ wave ends w/ spec reconciliation commit (§I matches what workers built; rules workers invented → §V) & coverage floor commit, trailer `reconcile-<n>`. ∵ R6: this is where drift rounds become visible.
V19: [reconcile · review] §B row whose cause can recur elsewhere opens sweep task over sibling nodes. ∵ R14.
V20: [dev-shell · pklith] nix flake + direnv; tools pinned by `flake.lock`; nixpkgs follows nixpkgs-lock; human reload checkpoint after shell lands. ∵ same tool versions for worker, hook & CI.
V21: [guardrails · sherdwright] guardrails before product code: one check per commit; commit body shows planted violation rejected; hooks live from commit that brings hk. ∵ R7; guard never seen rejecting ≡ guard that cannot.
V22: [guardrails · pklith] gate running zero steps = failure: evaluated step count ! equal declared count. ∵ R9.
V23: [gate · pklith] one `hk.pkl` on commit, push & CI. missing hk refuses. ⊥ `--no-verify`. `main` PR-only after `no-commit-to-branch`; merge by rebase | merge commit, ⊥ squash. ∵ one gate definition ∴ callers cannot drift; squash erases the commit trail V27 needs.
V24: [gate · pklith] test stage declared, ⊥ discovered: tests on pre-push; commit red on purpose says `RED` in subject; scaffolds laid before TDD-order gate listed in exemption list w/ reasons. ∵ R10.
V25: [gate · pklith] `.coverage` = ratchet floor, one meaning fleet-wide; caches use another name. ∵ R15.
V26: [gate · pklith] gate suppliers run hk too; lefthook leaves fleet. lefthook-gated repo = registry status `legacy`, checked only for migration invariants (D25, D46). ∵ supplier w/o gate = unguarded root of every gate.
V27: [gate · sherdwright] commit subject | body cites the `T`/`V`/`B` ids it serves. ∵ spec → commit trail searchable.
V28: [gate · pklith] CI = GitHub Actions running same `hk.pkl`, rendered by the CI materializer on 3 platforms. member needs GitHub remote. ∵ D45.
V29: [gate · pklith] argument parsing via clap. ∵ D47: hand-parsed args in every public member drift from help text.
V30: [gate · pklith] Shell: one mirrored `.bats` per script; kcov per-script floor on Linux CI only, macOS skips w/ stated reason. ∵ D48.
V31: [gate · review] Ruby mutation score = weekly scheduled ratchet outside hk, ⊥ per worker. ∵ D49, R13.
V32: [release · review] `cargo-release`, Keep a Changelog, version ladder. publish & push to other repos ! explicit human approval. ∵ registry artifact immutable.
V33: [backprop · sherdwright] bug found after green → §B row + the §V invariant that would have caught it (`ck:backprop`). public record in `docs/LLM-DISCLAIMER.md`, each naming its fix commit. ∵ bug w/o invariant recurs.
V34: [docs · pklith] ∀ count stated in prose (disclaimer, README, posts) generated, ⊥ typed; drift step fails when stale. ∵ a disclaimer's stated §V/§B counts drifted from its spec (D26).
V35: [propagation · review] decision found fleet-wide → issue in ∀ affected repo & promoted into paradigm. ∵ copies in member prompts drift.
V36: [any · sherdwright] stage commits carry `Paradigm-Stage:` trailer from §I.trailer vocabulary. ∵ stages that leave no file are checked from history (D38).
V37: [any · pklith] name guard: local denylist present → commit & push refuse private name in staged text. CI (no denylist) checks allowlist: ∀ repo named | linked ∈ committed public list, each row dated by `gh repo view` showing PUBLIC; adding row = own reviewed commit. ∵ D40; a record naming what it scrubbed reintroduces it.
V38: [any · sherdwright] waiver = `.pklith` `exempt` row: `paradigm/V<n>` + reason. waiver w/o reason fails. ∵ one place to waive (D5, D52).
V39: [any · sherdwright] seeded block ≡ pinned paradigm tag; drift fails. update = bump pin & regenerate in one reviewed commit; text outside markers stays member's. ∵ D29.
V40: [any · sherdwright] member pins paradigm tag; new invariant ⊥ fails old pin; report shows gap to latest. ∵ D4.
V41: [any · sherdwright] report lists ∀ invariant hold | waived | fail; count of `review` invariants reported, expected to fall. ∵ D3: rule w/o runner reads as law & is ⊥.
V42: [any · review] shared convention values (edition & MSRV, clippy deny list & thresholds, rustfmt, workspace w/ `dev/` crate, flake inputs & cachix, hk fast/all split, commit-msg format, changelog check, size limits, ASCII-only, cargo-deny, `rust-mirror` test layout, `unsafe` forbid) read from public members' files, ⊥ typed here; each gets own invariant once pulled. private prompts only unnamed cross-check; disagreement → gap-round question. ∵ D43.

## §T TASKS

id|status|task|cites
T1|.|D44 commit: after `gh repo view` shows repo A PUBLIC (≥ 2026-10-17), name it as CI materializer in roster, §C & every node; allowlist row|V28,V37
T2|.|dev shell: `flake.nix`, `.envrc`, nixpkgs follows nixpkgs-lock, mth & sherd & itok pinned; ask human to reload|V20
T3|.|`.gitignore`: `.claude/`, `.private/`, `target/`|V16,V37
T4|.|guardrails via pklith, one check per commit, planted violation each: name guard first, then mth & sherd spec steps, links, zero-step check; ask human to reload|V21,V22,V37,V9
T5|.|cargo workspace: `dev` & `sherdwright` members, `unsafe` forbidden, clap|V29
T6|.|pull shared convention values from public members' files; split V42 into one invariant per convention|V42
T7|.|`.context-limits` measured w/ slack per node once nodes stable|V11,V12
T8|.|waves over nodes via `sherd wave`, inception pages first (§G FOCUS)|V13,V14,V18
T9|.|propose pklith catalog profile for `pklith`-checked invariants upstream, by issue, w/ approval|V41
T10|.|first tag `v0.1.0` once docs, registry & seed exist; this repo pins it as first member|V40
T11|.|pre-push: ∀ repo named in committed text checked by `gh repo view`; private ones removed|V37

## §B BUGS

id|date|cause|fix
