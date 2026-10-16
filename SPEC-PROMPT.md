DRAFT INPUT. This file is the prompt for the first ck:spec run. When that run
lands SPEC.md, change this line to HISTORICAL INPUT: SPEC.md and the node
specs it federates become the law, and where they differ from this file, they
win. Keep the rest of this file unchanged for context.

paradigm

The software development lifecycle the fleet's Rust repositories are built
with, written down once, and checked rather than only described.

The short version: agents write the code; guardrails decide what lands.
Nobody watches an agent work. A human owns the intent (this kind of file),
answers the gaps, reviews the diff, and approves anything outward-facing.
Everything else is a gate that refuses.

Today that lifecycle lives in four places at once: in each repository's
`SPEC.md` constraints, in its `AGENTS.md` and `docs/LLM-DISCLAIMER.md`, in
the spec prompts that started each repository, and in agent memory. The
copies drift. Two existing spec prompts already repeat the same guardrail
roster and the same conventions list nearly word for word, and the later one
points at the earlier one ("Process: same as ...") although the earlier one
is frozen as historical input.

Goals:
- one canonical, versioned description of the lifecycle, from first prompt to
  release and bug backprop
- every claim in it backed by something that executes: a gate step, a
  conformance check, or a measurement in the spec's research section
- new projects cite the paradigm and state only their deviations, instead of
  copying conventions
- a member repository can be checked against the paradigm, and the report
  says which invariants hold, which are waived (with reason) and which fail
- a short human essay that explains why, linked from every member README

Write in Rust where code is needed. Prose for humans, caveman encoding for
spec text, as in the member repositories.
Use atomic git commits per topic for human readability.

# Naming

- `paradigm` stays (decision 27). The repository is referenced, not
  released: no package is published under this name, so the fleet naming
  rule for registries does not apply to it.
- If a tool ever ships from here (the conformance checker), that tool gets
  its own name under the fleet naming rule (free on crates.io, npm, PyPI,
  RubyGems, Homebrew core, Hex, NuGet, AUR, Debian and nixpkgs; no GitHub
  user or org; near-zero repositories; no live domains; no active tool of
  that name in the same space), with dated evidence.
- No `pr0d1r2/paradigm` repository exists on GitHub (checked 2026-10-16).

# Fleet survey (2026-10-16)

Local checkouts next to this repository, read-only. Numbers include backups
and experiments, so they are upper bounds; the spec run recounts against a
member list.
- 335 directories; 213 have `flake.nix`; 24 have `Cargo.toml`.
- Gate runners: 146 `lefthook.yml`, 36 `hk.pkl`. hk is the current runner;
  lefthook is the legacy one that pklith replaces.
- Spec-driven: 123 `SPEC.md` files at repository roots; 36 `.context-limits`;
  16 `.pklith`, 16 `.ctrm`, 16 `.coverage`; 35 `AGENTS.md`; 19
  `docs/LLM-DISCLAIMER.md`; 2 `SPEC-PROMPT.md`.
- Federation depth (node `SPEC.md` files, excluding worktrees): pklith 21,
  sherd 22, characterminator 21, rekall 23, xenolith 46, microlith 3, itok 1.
- Parallel builds leave traces: one repository holds 437 `SPEC.md` copies
  under `.claude/worktrees/`, one per subagent worktree of its sherd waves,
  and its history merges one node per wave ("Merge wave 7 node src/cli").

Conclusion: the lifecycle is already uniform enough to state once. What is
missing is the single statement and a way to check a repository against it.

# Members (public, surveyed 2026-10-16)

Public repositories that follow the paradigm and may be named. §V and §B
are summed over every node of the federation (each node numbers from 1).
Gate steps are commit/push from `hk.pkl` or the AGENTS.md table (about
plus or minus 1). Merged PRs come from `gh pr list --state merged`: merge
commits undercount, because PRs here merge by rebase.

repo|role|nodes|§V|§B|gate|coverage|crates.io|merged PRs
pklith|`.pklith` to hk steps; the roster's registry|21|125|22|41/48|100.00 floor|0.1.0|17
sherd|SPEC.md federation, nav, budgets, waves|22|336|156|29/37|93.09 floor|0.5.4|84
microlith|SPEC.md format: `mth fmt`, `mth check`|3|53|46|34/38|99.4 cache|0.7.4|70
itok|token budgets in `.context-limits`|1|123|42|29/36|98.3 cache|0.3.1|50
characterminator|allowed characters per file type|21|120|52|17/18|97.6 cache|0.1.1|10
rekall|rules and skills from agent memory|23|72|32|33/40|99.33 floor|0.4.0|31
xenolith|one language per file, test layout, TDD order|46|174|51|30/46|97.96 floor|0.1.0|23
claudinix|nix inside Claude Code cloud sessions|8|55|76|40/44|none|-|40
nix-hk|hk as a flake; supplies every gate|1|0|6|CI only|none|-|7
nix-fakecloud|fakecloud packaged for the fleet pin|1|28|3|CI only|none|-|0
nixpkgs-lock|the fleet's one nixpkgs revision|1|0|4|lefthook + CI|none|-|70

What the members show:
- Every Rust member has the nix dev shell, `.context-limits`, one `hk.pkl`
  for commit and push with CI calling it, `release.toml`, an
  LLM-DISCLAIMER and §B rows paired with §V.
- No member has a `SPEC-PROMPT.md`, now or in history. The practice exists
  only in private repositories so far; this repository makes it public.
  xenolith's first commit ("initial spec from another session") is the
  closest public trace.
- Inconsistent today: wave merges (visible mainly in sherd), RED-tagged
  commits (xenolith 136, claudinix 29, others near 0), tracked
  `.githooks/` (pklith and sherd only), what `.coverage` means (a ratchet
  floor in four members, a tree-hash cache in three), federation (itok is
  one node), and the gate suppliers themselves: nix-hk has no hk gate and
  nixpkgs-lock still runs lefthook.
- Records worth citing: sherd §B12 (a rule that lived only in prose lost
  about 20 times in one session; runners beat prose); sherd §B4 (a claimed
  95x speedup was 2.1x); microlith raised its spec ceiling four times; a
  disclaimer's stated §V/§B counts drifted from its spec.

# Learnings from private repositories (anonymized)

Private and unreleased repositories that use nix-hk, surveyed read-only on
2026-10-16. They are never named in this repository: each is a letter and
a generic category. Rust: A (CI workflow generator), B (load-gating CLI),
C (repository skeleton seeder), D (local service emulator), E (status line
renderer), F (provisioning orchestrator, spec and gate only). Ruby: R (a
web application). Shell: S1 (an appliance image), S2 (fleet
infrastructure).

repo|commits|nodes|§V|§B|RED commits|wave merges|built in
A|200|23|49|7|20|19|about 2 days
B|415|29|217|9|155|2|about 2 days
C|33|1|29|3|-|0|-
D|39|8|34|7|-|0|-
R|about 1,400|33|-|-|-|fast-forward|10 days

How the lifecycle actually ran (A and B, the complete cases):
- Order seen in history: prompt; naming with evidence; survey; numbered
  decision rounds inside the prompt (A: 4 rounds, 23 decisions; B: 3 plus
  a late one); a domain catalogue in `docs/` and a commit aligning earlier
  decisions with it; spec format vendored; ck:spec landing one section
  per commit; `fix(spec): pass mth check`; spec reordered so dev shell and
  guardrails come first; guardrails; crate seed; `dev/` crate; federation
  scaffold; waves; dogfood.
- Gap rounds after the first spec are real: B has about 15 commits between
  "add root spec" and "mark spec prompt as historical input" (security
  surface, research rows for platform unknowns, quiet mode, release order
  and cost budget, named profiles, flake exports, an overnight build
  harness).
- ck:check rounds leave no commit named after them. They show as
  `docs(spec): round N interfaces match what the workers built` after
  each wave, as "decisions the spec did not cover" (rules workers made,
  backfilled as invariants), and as `docs(node): plan X` before code.
- Guardrails first, learned between B and A: B bundled guardrails into 6
  task commits; A laid about 70 guardrail commits, one check each, rule
  then check, each citing its planted-violation proof, before the first
  product RED at about commit 98.
- After each wave A raised the coverage floor in its own commit (`floor
  follows wave N`).

What broke, and what the prompt should have said up front:
- Parallel workers broke the hooks: tests ran scratch git with the hook's
  `GIT_*` environment and re-entered the hooks (432 hk processes, fork
  failures); a scratch `git init` wrote `core.worktree` into the shared
  `.git/config`, so every worktree acted on one checkout; hk's git stash
  is shared across worktrees and moved files between workers (in R too:
  two threads popped each other's edits). Fixes: no stash in pre-commit
  (or `stash = "patch-file"`), tests out of pre-commit, child git drops
  `GIT_*`, a gate step refuses `core.worktree`.
- A gate silently ran zero steps: a Pkl generator passed `pkl eval` and
  `hk validate`, yet hk's evaluator produced an empty pre-commit. Workers
  noticed only from "no steps to run".
- Moving tests to pre-push let red tests be committed (5 goldens in A).
- Scaffolds laid before the TDD-order gate needed an exemption list,
  invented per repository.
- The same flaky-test cause was found twice in different nodes of B: a
  §B row in one node does not reach its siblings.
- Budget churn: `.context-limits` changed 12 times in A and 7 in B;
  "tersen to fit" and "re-measure ceilings" recur; one ceiling went from
  1700 to 2100. Slack that worked: 12 to 20% over measured.
- Worktrees are never cleaned: A keeps 19 agent worktrees (281 MB) and 20
  branches; R keeps 10 wave worktrees 35 to 662 commits behind; S2 keeps 5
  unignored ones, so a `SPEC.md` glob finds 124 files instead of 21.
- Mutation testing run by every wave worker on every core pushed load to
  170 on 10 cores (R).
- A tool that lays commits landed its own re-laid commits in the real
  history, 4 to 6 times each (C).
- Mid-build scope creep: a new catalogue node arrived during waves, and
  one spec rule was reverted in substance and restored (A).

Fleet decisions made locally, not yet promoted:
- Argument parsing: clap in A, B, C; hand-parsed in E, pklith and every
  public member (tracking issues are filed).
- `unsafe`: `forbid` everywhere except B (`deny` plus a gate that allows
  it in one FFI node only).
- Coverage: four implementations and two meanings of `.coverage`.
- Commit subjects: D cites `(Tn, Vn)` ids in subjects, which makes the
  spec-to-commit trail searchable.
- Federation timing: at spec time (B), after guardrails (A), mid-build
  (D), never (C, E). By module in most, by verb in F.
- No CI and no PR-only main in R and S1: the pre-push gate is the only
  gate.

# The lifecycle, as practised today

Describe each stage in the spec as its own buildable unit, with its inputs,
its outputs, the tool that checks it, and its exit condition.

1. Intent. A human writes `SPEC-PROMPT.md`: what the tool is, goals, naming
   with availability evidence, a dated survey, a config and CLI sketch,
   numbered decisions, prior art and rejected designs. A large domain gets an
   authoritative companion catalogue in `docs/` that the prompt defers to.
2. Gap rounds on the prompt. An agent reads the prompt against the fleet and
   lists gaps, contradictions and undecided questions. The human answers.
   Answers are appended as a new round of numbered decisions; earlier
   decisions are amended in place only to align, never silently reversed.
3. Spec run. `ck:spec` turns the prompt into `SPEC.md` (sections
   G/F/N/C/I/R/V/T/B). The prompt is marked HISTORICAL in its own commit.
4. Gap and drift rounds on the spec. Repeated gap-finding passes plus
   `ck:check` (spec against code, read-only) until a round finds nothing.
   `mth check`, `sherd check`, `sherd validate` and `sherd budget` pass after
   every edit.
5. Dev shell. Nix flake with direnv, tools pinned by `flake.lock`. The human
   reloads the shell and the session continues.
6. Guardrails first, before product code. One check per commit; each new
   step proves in its commit body that it rejects a planted violation; hooks
   are live from the commit that brings hk. The human reloads again.
7. Sherd waves. Once the spec has independent nodes, `sherd` orders them into
   waves; each node is built by its own subagent in its own git worktree, in
   TDD (RED, GREEN, REFACTOR), and merged one node per merge. A node's spec,
   budget and `.context-limits` row bound what that subagent must read.
8. Gate. The same `hk.pkl` runs on commit, on push and in CI on three
   platforms. Missing hk refuses. `--no-verify` is never used. `main` is
   PR-only after `no-commit-to-branch` lands; merge by rebase or merge
   commit, never squash.
9. Release. `cargo-release`, Keep a Changelog, version ladder. Publishing and
   pushing to other repositories need explicit human approval.
10. Backprop. A bug found after green gets a `§B` row and the `§V` invariant
    that would have caught it (`ck:backprop`). The public record of such
    defects stays in `docs/LLM-DISCLAIMER.md`, each naming its fix commit.
11. Fleet propagation. A decision that turns out to be fleet-wide is filed as
    an issue in each affected repository and, from now on, promoted into the
    paradigm.

# What the paradigm owns, and what it does not

- Owns: the lifecycle above; the fleet invariants (the rules every member
  keeps); the guardrail roster with each tool's role; the shared conventions
  list; the spec prompt format; the member registry; the essay.
- Does not own: any gate step's implementation (pklith and the tools in the
  roster do), code and configuration skeletons (seeded elsewhere), CI
  workflow rendering, or any member's product decisions.
- The paradigm is a statement, a checker, and a seed of instructions
  (decisions 28 to 31). It writes into a member only the marked blocks it
  seeds, when the member runs the seed; it never edits anything else.

# Guardrail roster (moves here from the member prompts)

- pklith: materialise hk git hooks from what the repository's files require;
  installed when entering the dev shell (`lib.devShell`, tracked
  `.githooks/`).
- nix-hk: hk in the flake.
- nixpkgs-lock: one nixpkgs revision for the whole fleet.
- sherd: SPEC.md federation over a directory DAG, budgets, navigation, wave
  builds in parallel subagents.
- microlith: spec operations (`mth fmt`, `mth check`, `mth archive`).
- itok: context limits (`.context-limits`).
- characterminator: allowed characters per file type (`.ctrm`).
- rekall: rules and skills from agent memory, with its honesty gate.
- xenolith: one language per file, `rust-mirror` test layout, `tdd-order`.

# Shared conventions (move here from the member prompts)

Lift the conventions list that the two existing prompts repeat (Cargo
edition and MSRV, clippy deny list and thresholds, rustfmt, workspace with a
`dev/` crate, flake inputs and cachix, hk.pkl fast/all split, commit-msg
format, changelog check, coverage floor plus ratchet, size limits,
ASCII-only, spec sections and caveman encoding, cargo-deny, CI on three
runners, release flow, test layout, PR-only main). Each becomes a numbered
fleet invariant with its reason, so a member can cite it by id and waive it
by id.

# Spec prompt format (improved)

The format is part of the paradigm. What the two existing prompts taught:

- Status line first, from day one: DRAFT INPUT, then HISTORICAL INPUT. One
  repository added it only after the spec run, in its own commit.
- `Paradigm: <name> <version>` near the top, then deviations only. No copied
  roster, no copied conventions, no "same as <sibling repo>": a sibling's
  prompt is frozen and its spec moves on, so the pointer rots.
- Sections in this order: status; name and one-line pitch; problem and goals;
  naming with dated availability evidence and rejected names; dated survey
  with numbers; companion catalogue pointer if any; config sketch; CLI
  sketch; decisions in numbered rounds (dated); non-goals; prior art and
  rejected designs; open questions.
- Open questions are a section, not an absence. A gap round moves an entry
  from open questions to a numbered decision. The spec run refuses to start
  while a question that blocks a node is open, and asks instead of guessing.
- Every decision says whether it is local or a candidate for the paradigm.
  Candidates are promoted after the member ships, not before.
- Every number has a date and a way to recount it.
- Before the first push, every repository named in a committed file is
  checked with `gh repo view`; private repositories are not named.

# Language pages

The main pages describe the lifecycle with Rust as the worked example,
because every public member is Rust. Each other language gets its own page
in `docs/`, linked from the main pages, with the same headings so a reader
can compare: stack; dev shell; federation (what a node is); gate map (stage
to tool to `.pklith` row); mirror tests; coverage (and what it means for
that language); commit versus push; waves; release; gaps against the Rust
flow; anti-patterns.

- `docs/RUST.md`: the reference; clippy limits, `dev/` crate, cargo-release.
- `docs/RUBY.md`: from R. packwerk packs as sherd nodes, mirror specs both
  ways for app, lib, views and JavaScript, a three-criteria 100% floor
  plus views, query and CPU pace budgets per example, recorded externals
  only, custom cops that encode a fix once, guard-style selection on
  commit, waves with a database per worker, release as deploy.
- `docs/SHELL.md`: from claudinix, S1 and S2. No shell functions except
  logging, so one script has one test; mirror `.bats` both ways; TDD order
  checked from history; extract, never embed (justfile one-liners, nix
  `readFile`, xenolith guard); kcov per-script floors (Linux only); tests
  on push because RED commits are red on purpose.
- `docs/NIX.md`: nixfmt, statix, deadnix; one nixpkgs with `follows` and a
  lock-graph guard; flake checks scoped to the files the flake reads;
  `runNixOSTest` modules; Linux work from macOS on a remote builder; cachix
  push and signature checks; pin bumps by PR; lefthook migration notes.
- Python and JavaScript appear only as fixtures or a handful of files in
  the fleet; no page until a member is built in them.

# Decisions (2026-10-16)

1. The paradigm is itself built with the paradigm: SPEC-PROMPT.md, gap
   rounds, spec run, guardrails first, waves, gate, PR-only main. It is the
   first member in its own registry.
2. Deliverables, in order: `SPEC.md` (the statement, caveman encoded);
   `docs/PARADIGM.md` (the essay, prose, short); `docs/SPEC-PROMPT.md` (the
   prompt format above, with a template); the member registry; the
   conformance checker.
3. Fleet invariants carry stable ids, a reason, the stage they belong to and
   the tool that checks them. An invariant with no checker is listed as
   unchecked, and the count of unchecked invariants is a reported number,
   expected to fall.
4. Versioned: the paradigm is released, and a member pins the version it
   conforms to. A new invariant does not fail an old pin; the report shows
   the gap to the latest version.
5. Waivers: a member waives an invariant by id with a reason in its own
   config, as pklith's `exempt` table does. A waiver without a reason fails.
6. Member numbers in the registry and the essay (steps, invariants, bugs,
   nodes, coverage floor) are pulled from member repositories, never typed,
   and a drift step fails when they go stale, as pklith's README badges do.
7. Deterministic: same member trees and same paradigm version give the same
   report. No LLM, no network in the checker, except an opt-in verb that
   reads remote repositories.
8. Public by default, like the guardrail tools. Private members appear in
   the registry only by count, never by name.

# Decisions, round 2 (2026-10-16): a public hub first

9. Primary purpose: this repository is the public documentation hub that
   posts (LinkedIn and similar) link to. Readers arrive cold, from one link,
   on one page. Every page stands alone: what the claim is, why, the
   evidence, and where to go next.
10. Deliverables reorder (supersedes the order in decision 2): `README.md`
    as the landing page, the essay, one page per lifecycle stage and per
    topic a post is likely to argue (guardrails over watching, spec before
    code, parallel agents, the unflattering record), the members page, the
    spec prompt format, the language pages. The conformance checker comes
    after the docs, and feeds their numbers.
11. Links are a public API. A page path or heading anchor that a post links
    to never breaks: a rename leaves a stub that points to the new place.
    The gate checks internal links and that no linked anchor disappears.
12. Posts ledger: `docs/POSTS.md` lists each post (date, URL, pages it
    links). A page in the ledger cannot be removed, only superseded.
13. Page shape: claim first, evidence second (numbers pulled from members,
    commit links, member links), how-to last. Prose for humans; caveman
    encoding stays in SPEC.md only. Short enough to read from a phone.
14. The name is now part of every posted URL, so it must be final before
    the first post links here. Until then, no post links to this repository.
    (Settled by decision 27.)

# Decisions, round 3 (2026-10-16): rules the survey earned

Each is a candidate fleet invariant; the spec run gives it an id, a
checker, and the incident that earned it.

15. Worker isolation. Parallel workers never share mutable git state: no
    git stash in pre-commit (or hk `stash = "patch-file"`), child git in
    tests drops `GIT_*`, a gate step refuses `core.worktree` in shared
    config, and each worker gets its own test database or scratch dir.
16. Worktree lifecycle. `.claude/` is ignored from the first commit. A
    merged wave removes its worktree and branch in the same session; a
    gate step fails on a worktree whose branch is already merged.
17. Wave-end ritual. Every wave ends with a spec reconciliation commit
    (interfaces match what workers built; rules workers invented become
    invariants) and a coverage floor commit. This is where ck:check rounds
    become visible in history.
18. Test stage policy is declared, not discovered. Tests run on pre-push;
    a commit that is red on purpose says RED in its subject; scaffolds laid
    before the TDD-order gate go in an exemption list with reasons.
19. A gate that runs zero steps is a failure. The gate checks its own
    evaluated step count against the declared one.
20. Ceilings carry 12 to 20% slack over measured. A raise is its own
    reviewed commit with the measurement; sherdify before raising.
21. Cross-node bugs: a §B row whose cause can recur elsewhere opens a sweep
    task over sibling nodes.
22. Worker count is sized to the machine, and heavy steps (mutation,
    coverage) run under a load gate, one job per worker.
23. Commit bodies or subjects cite the `T`/`V`/`B` ids they serve, so the
    spec-to-commit trail is searchable.
24. `.coverage` has one meaning fleet-wide: the ratchet floor. Caches move
    to another name.
25. The gate suppliers get the gate: nix-hk and nixpkgs-lock run hk too,
    and lefthook leaves the fleet.
26. Every count stated in prose (disclaimer, README, posts) is generated,
    extending decision 6 to members' own docs.

# Decisions, round 4 (2026-10-16): name and seeding

27. The name is `paradigm`, final. The repository is a reference hub, never
    a released package, so registry availability does not matter.
    "Versioned" in decision 4 means git tags that members pin, not
    published releases. Supersedes the placeholder note in Naming.
28. The paradigm seeds new repositories with words. Most of what a spec
    prompt has carried so far is the guardrails and the process, not the
    product. That part moves here, and a seed gives a new repository:
    - an `AGENTS.md` paradigm block: the lifecycle, the gate rules
      (never `--no-verify`, refuse when hk is missing), commit format,
      worker isolation, worktree cleanup, wave-end ritual, reload
      checkpoints, and where to read more (links into this hub);
    - a `SPEC-PROMPT.md` template whose process, guardrail roster and
      conventions are cited by paradigm version, leaving the human only
      product intent, naming, survey, domain decisions, non-goals and open
      questions;
    - a `docs/LLM-DISCLAIMER.md` skeleton and `.gitignore` with `.claude/`.
29. Seeded blocks sit between generated markers (as pklith's README badges
    do) and a drift step fails when a block differs from the pinned
    paradigm version. Updating means bumping the pin and regenerating, in
    one reviewed commit; text outside the markers stays the member's own.
30. Division of labour for a new repository: the skeleton seeder lays
    files and configuration (flake, Cargo, `hk.pkl`), pklith lays the gate
    one check per commit, and the paradigm lays the instructions. The
    paradigm never writes code or gate steps.
31. The seed is per language: the `AGENTS.md` block and the prompt
    template include the language page's rules (Rust by default; Ruby,
    Shell, Nix from their pages).

# Decisions, round 5 (2026-10-16): gap round on this prompt

32. Conformance checker (closes open question 1): file-level invariants
    become a pklith catalog profile; a small separate binary covers only
    the stages pklith cannot see (prompt, gap rounds, spec runs, waves,
    releases). The binary gets its own name under the fleet naming rule
    (open question 10).
33. Member list (closes open question 4): one explicit registry file in
    this repository, changed only by a reviewed PR.
34. Private counts: an opt-in local verb writes a dated, anonymized
    snapshot of private-member counts, committed through a reviewed PR.
    CI checks drift for public members only; snapshot rows show their
    date. This is the opt-in verb of decision 7 and the count of
    decision 8.
35. Numbers before the generator exists: a page may carry a typed number
    only with an as-of date, and every such number is listed in a numbers
    ledger. The generator takes ledger rows over one by one; the count of
    typed numbers is reported and expected to fall. Aligns decisions 6
    and 26 with the docs-first order of decision 10.

# Non-goals

- Not a code or configuration seeder: it seeds instructions and the spec
  prompt template only (decision 30).
- Not a second gate: the paradigm never runs lint or test steps itself.
- Not a process for repositories outside nix and hk in its first version:
  Rust, Ruby, Shell and Nix members on hk are in (Language pages); the
  lefthook-era repositories stay out until they move to hk.
- Not an agent harness: usage limits, session budgets and agent scheduling
  belong to the harness, not to repositories.

# Prior art and rejected designs

- A handbook wiki or a long CONTRIBUTING file: rejected. Prose that nothing
  executes is exactly what drifts today.
- A shared template repository: rejected, as copy-and-sync bots were for
  CI workflows: copies fight local edits, and nothing says which values
  were meant to differ.
- Putting the fleet invariants into pklith's catalog only: partly right,
  because pklith already enforces file coverage, but the lifecycle has
  stages pklith cannot see (the prompt, gap rounds, spec runs, waves,
  releases). Open question 1 decides the split.
- Process frameworks (agile ceremonies, ISO-style quality manuals): they
  assume humans write the code and other humans watch. Here agents write it,
  gates refuse, and the human owns intent and approval.
- Differentiator: the lifecycle is stated once, every rule names its
  checker, and a member's conformance is a report, not a claim.

# Open questions

1. Conformance checker: a new binary, or a pklith profile plus a small
   report verb? Recommendation: start as a pklith catalog profile for the
   file-level invariants and a separate small binary only for stages pklith
   cannot see; decide during the spec run's research.
2. Where the essay lives: only here, or also rendered into each member's
   `docs/PROCESS.md` by its `dev/` crate? Recommendation: here only, plus a
   generated one-paragraph pointer with that member's numbers.
3. How to check stages that leave no file (gap rounds, `ck:check` rounds):
   commit trailers, a spec log section, or not at all? A stage that cannot
   be checked is reported as unchecked, not dropped.
4. Member list: explicit registry file, or discovery by a marker file in
   each member? Recommendation: explicit file, reviewed by PR.
5. Final name. Closed by decision 27: `paradigm`.
6. Coverage outside Rust: for shell, is the one-test-per-script mirror the
   floor until kcov is gated, and how are floors measured on Linux only
   compared with macOS runs? For Ruby, does mutation score join the gate
   or stay a weekly ratchet?
7. Argument parsing: promote clap to a fleet invariant (three private
   repositories use it; tracking issues are filed in the public ones), or
   record hand parsing as the rule?
8. CI and PR-only main for members on a self-hosted remote with no
   pipeline (R, S1): waive with a reason, or require a runner?
9. Seed command: does the seed (decisions 28 to 31) run from this
   repository's flake (`nix run`), from the skeleton seeder, or from
   pklith's `seed` verb?

# Process

The lifecycle above, applied to this repository: dev shell, then ask me to
reload; guardrails first, one check per commit, then ask me to reload; TDD;
1-to-1 unit coverage with floor and ceiling, aiming at 100%; federated
SPEC-DAG built in sherd waves once the spec has independent nodes.
