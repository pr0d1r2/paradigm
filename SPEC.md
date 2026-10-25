# SPEC

## §G GOAL

`paradigm` — the lifecycle fleet repos are built with by coding agents, written ONCE, versioned by git tag & CHECKED, ⊥ only described. agents write code; guardrails decide what lands; human owns intent, gap answers, signoff (digest & green report, ⊥ code read) & every outward-facing act.

FOCUS (prompt decision 41): sherdified SDD & the INCEPTION that starts it — spec prompt → gap rounds → spec run → federation into sherd nodes → waves of isolated workers → wave-end reconciliation. these stages get the deepest pages, the first invariants & the first checker coverage. fleet conventions stated briefly & cited, ⊥ argued.

SCOPE OF ROOT §V: root §V = the FLEET INVARIANTS, the rules every member keeps, this repo included (prompt decision 1). cited outside this repo as `paradigm/V<n>`. rules only this repo keeps live in its nodes.

LINEAGE: `SPEC-PROMPT.md` (HISTORICAL after this run). where it & this spec differ, this spec wins. prompt decisions cited as `D<n>`.

## §F FEDERATION

dir|owns|⊥owns|tokens
docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children|landing `README.md` (root), member data (`registry`)|-
registry|member list: repo, status, pin, public evidence; private-count snapshot|checking members (`sherdwright`), page prose (`docs`)|-
dev|this repo's own gate steps, `publish = false`: links, anchors, posts, numbers, name guard|member checks (`sherdwright`), seeding (`seed`)|-
sherdwright|stage checker binary: history, trailers, waivers, report|file-level invariants (pklith profile), writes into members|-
seed|per-language instruction blocks between markers; flake app `#seed`|code, config, gate steps (skeleton seeder, pklith)|-

## §N NAV

rel|path|lens
up|-|-
self|.|-

## §C CONSTRAINTS

- Rust where code needed. one cargo workspace: `dev` (publish = false) & `sherdwright`. `unsafe` forbidden.
- prose for humans in pages; caveman only in `SPEC.md` files (D13). page = claim first, evidence second, how-to last; readable on phone.
- dev shell per V20; human reloads shell at each checkpoint.
- gate = one `hk.pkl`, materialized by pklith from `.pklith`, run on commit, push & CI. CI = GitHub Actions rendered by actlith (CI materializer, repo A of D44; public 2026-10-17) from the pipelith core.
- gate rules per V23.
- atomic commits, one topic each. ∀ commit from gap round 5 on carries `Paradigm-Stage:` trailer (V36, V43); ids per V27.
- public by default. private repos ⊥ named in committed text: letter + generic category in prose, count only in registry (D8, D34). local denylist in `.private/`, ⊥ committed.
- links are public API: page path | heading anchor a post links to ⊥ breaks; rename leaves stub pointing to new place (D11).
- numbers: member numbers pulled from member repos, ⊥ typed (D6, D26). until generator owns a number, typed number carries as-of date & numbers-ledger row (D35).
- `paradigm` is never a released package: version = git tag members pin (D4, D27). `sherdwright` may publish under own name, with approval.
- paradigm writes into a member ONLY seeded blocks between markers, ONLY when member runs seed (D29, D30). ⊥ writes code, config or gate steps into members.
- checker deterministic: same member trees + same paradigm version → same report. ⊥ LLM. ⊥ network, except one opt-in verb reading remote repos (D7).
- outward-facing acts need explicit human approval: push to other repos, publish, issues in members, posts.
- not a second gate: paradigm ⊥ runs members' lint | test steps (non-goal). not an agent harness: usage limits & scheduling belong to harness.
- root §V = index (owner 2026-10-25): ids `paradigm/V<n>` stay here, being public; a staged rule's full text & `∵` live in its `docs/<node>` §C. `any` rules stay whole. sherdify before raising (V12).

## §I INTERFACES

- page: `README.md` — landing page. links every page below. owned here; pages owned by `docs`.
- file: `SPEC.md` root §V — fleet invariant ids, cited `paradigm/V<n>`. ids monotonic, ⊥ reused, ⊥ renumbered.
- tag: `v<semver>` — the paradigm version a member pins (D4). new invariant ⊥ fails an older pin.
- file: `registry/members.toml` — member list (`registry` node, D33).
- cmd: `nix run github:pr0d1r2/paradigm/<tag>#seed -- <lang>` — instruction seed (`seed` node, D36). `<lang>` ∈ `rust` `ruby` `shell` `nix` `swift`.
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

Fleet invariants. Each: `[stage · checker]`, rule, `∵` reason. checker = one checker | a set: `sherdwright` (history & trailers) · `pklith` (file-level profile) · a roster tool (`mth`, `sherd`, `itok`) · `review` (no runner yet ∴ counted unchecked, V41). a staged rule is a summary → its `docs/<node>`, whose §C holds rule & `∵`.

V1: [intent · sherdwright] `SPEC-PROMPT.md` precedes first root `SPEC.md`, status line `DRAFT` → `HISTORICAL` → `docs/intent`
V2: [intent · sherdwright] prompt pins `Paradigm: paradigm <tag>`, states deviations only → `docs/intent`
V3: [intent · sherdwright] prompt sections in fixed order → `docs/intent`
V4: [intent · review] ∀ number in prompt | page dated w/ recount method → `docs/intent`
V5: [gap-round · sherdwright] gap answers append as dated numbered decisions, ⊥ silent reversal → `docs/gap-rounds`
V6: [gap-round · review] decision tagged local | candidate; candidate promoted after member ships → `docs/gap-rounds`
V7: [spec-run · sherdwright] spec run ⊥ starts while a blocking open question is open → `docs/spec-run`
V8: [spec-run · sherdwright] spec run lands one section | node per commit, trailer `spec-run` → `docs/spec-run`
V9: [spec-run · mth, sherd] ∀ `SPEC.md` edit passes `mth` & `sherd` checks → `docs/spec-run`
V10: [spec-round · sherdwright] gap & drift rounds repeat until clean, each leaves a trailer commit → `docs/spec-rounds`
V11: [federation · sherd] federation by directory: ∀ node own `SPEC.md`, §N generated → `docs/federation`
V12: [federation · itok] ceiling = measured + 12–20%; sherdify before raising → `docs/federation`
V13: [waves · sherdwright] waves start after dev shell & guardrails → `docs/waves`
V14: [waves · sherdwright] one node per worker, worktree & subagent, TDD, one node per merge → `docs/waves`
V15: [waves · pklith] workers ⊥ share mutable git state → `docs/waves`
V16: [waves · pklith] `.claude/` ignored; merged wave removes its worktree & branch → `docs/waves`
V17: [waves · review] worker count sized to machine; heavy steps under load gate → `docs/waves`
V18: [reconcile · sherdwright] ∀ wave ends w/ reconcile & coverage floor commits, trailer `reconcile-<n>` → `docs/reconciliation`
V19: [reconcile · review] recurring §B cause opens a sibling sweep task → `docs/reconciliation`
V20: [dev-shell · pklith] nix flake + direnv, pinned by `flake.lock`, one nixpkgs → `docs/dev-shell`
V21: [guardrails · sherdwright] guardrails before product code, one check per commit, planted violation → `docs/guardrails`
V22: [guardrails · pklith] gate running zero steps = failure → `docs/guardrails`
V23: [gate · pklith] one `hk.pkl` on commit, push & CI; ⊥ `--no-verify`; `main` PR-only, ⊥ squash → `docs/gate`
V24: [gate · pklith] test stage declared: tests on pre-push, `RED` in subject → `docs/gate`
V25: [gate · pklith] `.coverage` = ratchet floor, one meaning → `docs/gate`
V26: [gate · pklith] gate suppliers run hk; lefthook = `legacy` → `docs/gate`
V27: [gate · sherdwright] commit cites the `T`/`V`/`B` ids it serves → `docs/gate`
V28: [gate · pklith] CI = actlith-rendered GitHub Actions running `hk.pkl`; 4 systems; macOS floor 14.0 → `docs/gate`
V29: [gate · pklith] argument parsing via clap → `docs/gate`
V30: [gate · pklith] Shell: mirrored `.bats` per script, kcov floor on Linux CI → `docs/gate`
V31: [gate · review] Ruby mutation = weekly ratchet outside hk → `docs/gate`
V32: [release · review] `cargo-release`, Keep a Changelog; publish ! human approval → `docs/release`
V33: [backprop · sherdwright] bug after green → §B row + the §V that catches it → `docs/backprop`
V34: [docs · pklith] ∀ count in prose generated, ⊥ typed → `docs/backprop`
V35: [propagation · review] fleet-wide decision → issue ∀ affected repo & promoted → `docs/propagation`
V36: [any · sherdwright] stage commits carry `Paradigm-Stage:` trailer from §I.trailer vocabulary. ∵ stages that leave no file are checked from history (D38).
V37: [any · pklith] name guard: local denylist present → commit & push refuse private name in staged text. CI (no denylist) checks allowlist: ∀ repo named | linked ∈ committed public list, each row dated by `gh repo view` showing PUBLIC; adding row = own reviewed commit. ∵ D40; a record naming what it scrubbed reintroduces it.
V38: [any · sherdwright] waiver = `.pklith` `exempt` row: `paradigm/V<n>` + reason. waiver w/o reason fails. ∵ one place to waive (D5, D52).
V39: [any · sherdwright] seeded block ≡ pinned paradigm tag; drift fails. update = bump pin & regenerate in one reviewed commit; text outside markers stays member's. ∵ D29.
V40: [any · sherdwright] member pins paradigm tag; new invariant ⊥ fails old pin; report shows gap to latest. ∵ D4.
V41: [any · sherdwright] report lists ∀ invariant hold | waived | fail; count of `review` invariants reported, expected to fall. ∵ D3: rule w/o runner reads as law & is ⊥.
V42: [any · review] shared convention values read from public members, ⊥ typed → `docs/propagation`
V43: [any · sherdwright] `Paradigm-Stage:` ! parse as git trailer: `git log --format='%(trailers:key=Paradigm-Stage)'` returns it ∴ line sits in final paragraph w/ other trailers. a body line that only LOOKS like a trailer = ⊥ trailer. ∵ B1.
V44: [gate · review] RED commit compiles → `docs/gate`

## §T TASKS

id|status|task|cites
T4|x|guardrails via pklith, one check per commit, planted violation each|V21,V22,V27,V37,V43,V9
T6|.|pull shared convention values from public members' files; split V42 into one invariant per convention|V42
T8|.|waves over nodes via `sherd wave`, inception pages first (§G FOCUS)|V13,V14,V18
T9|.|propose pklith catalog profile for `pklith`-checked invariants upstream, by issue, w/ approval|V41
T10|.|first tag `v0.1.0` once docs, registry & seed exist; this repo pins it as first member|V40
T11|.|pre-push: ∀ repo named in committed text checked by `gh repo view`; private ones removed|V37
T13|.|once `.pklith` exists: waive `paradigm/V3` for this repo, reason B2. `sherdwright` CLI sketch lands in its §I via `sherdwright:T1`; seed prompt template enforces V3 order (`seed:T1`)|V3,V38
T14|.|`.context-limits` one meaning fleet-wide, as V25 for `.coverage`: sherd reads node chain ceilings, itok reads per-file ceilings from same file & refuses root `SPEC.md` ∴ a member ⊥ gates both (age-plugin-se PR #5, paradigm#8). pick one; other tool's file gets another name; issue in sherd \| itok w/ approval, then own invariant|V11,V12,V25
T15|.|actlith `xcode=` pin ⊥ survive `nix develop`: it sets `DEVELOPER_DIR`, the stdenv overwrites it before the shell hook ∴ CI ran the image default Xcode w/o a word; age-plugin-se hands it as `HOST_DEVELOPER_DIR` by hand (PR #9, `131b65e`). pick where the handoff lives (actlith gate kind \| pklith `lib.devShell` \| each member); issue there w/ approval; evidence only, ⊥ invariant until decided|V20,V28
T16|.|characterization stage for inherited code (forks, legacy), between guardrails & first wave (paradigm#22): tests only, ⊥ source change; uncovered lines inventoried (a) testable now (b) after refactor (c) untestable in CI; exclusion cap beside floor, floor only rises, cap only falls; V13 entry criterion for inherited code; trailer value TBD. evidence: age-plugin-se PRs #12, #14 to #19 (2026-10-18; docs/waves, docs/gate). wording = owner's; ⊥ invariant until decided|V13,V18,V24,V25,V36
T17|.|pklith cargo row for V28's universal check (Swift model: age-plugin-se `gate/universal.sh`); issue w/ approval|V28,V24
T18|.|DECIDED owner 2026-10-24 (#25): V28's `.cargo/config.toml` ∵ cargo reads it in ∀ 4 build places (pre-push, CI, `buildRustPackage`, bare cargo); ⊥ pipelith alone (CI only). pklith catalog row refuses w/o it, seed writes it (in progress); `lib.devShell` export optional|V28,V20
T19|.|DECIDED where, owner 2026-10-24 (#25): pipelith sets CI's systems once, incl. Intel macOS; forge renderers render it (actlith refuses `x86_64-darwin` 2026-10-24). runner GitHub's Intel \| self-hosted = owner's, open|V28
T20|.|rollout: pilot sherd, then one PR per repo|V28,V35
T21|.|RISK 2026-10-21: nixpkgs 26.05 last w/ x86_64-darwin (pklith T76); decide before nixpkgs-lock bump past 26.05|V20,V28
T22|.|candidate (V6): mechanical arms: tool does an agent step's mechanical share, LLM only what no check decides; repeated LLM decision → check|V6,V33
T23|.|candidates (V6), pklith issue w/ approval: §F ⊥owns as path & import check (`xnl graph` \| `ast-grep`); `.duplication` ratchet; `cargo-mutants` weekly|V6,V25,V31
T24|.|candidate (V6): per release rebuild one node here from its node chain alone; its tests fail → §B; ⊥ member tests (§C)|V6,V32

## §B BUGS

id|date|cause|fix
B1|2026-10-17|`Paradigm-Stage:` written as own paragraph above `Co-Authored-By:` ∴ git parsed 0 of 38 stage trailers; found by ck:check round 1|V43
B2|2026-10-17|this repo's prompt froze w/o config sketch & CLI sketch, `# Process` after `# Open questions` ∴ breaks V3; frozen HISTORICAL ∴ ⊥ fixable in place|V3,T13
B3|2026-10-17|31 spec-run commits cite no `T`/`V`/`B` id: nothing gated V27 before hk ∴ rule read as law w/o runner|V27,T4
