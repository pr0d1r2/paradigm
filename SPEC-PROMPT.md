DRAFT INPUT. This file is the prompt for the first ck:spec run. When that run
lands SPEC.md, change this line to HISTORICAL INPUT: SPEC.md and the node
specs it federates become the law, and where they differ from this file, they
win. Keep the rest of this file unchanged for context.

paradigm (working name; see Naming)

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

- `paradigm` is a placeholder. It fails the naming rule: a dictionary word,
  taken on most registries, with many GitHub users and repositories.
- Pick the real name before the first push, by the fleet naming rule: free on
  crates.io, npm, PyPI, RubyGems, Homebrew core, Hex, NuGet, AUR, Debian and
  nixpkgs; no GitHub user or org; near-zero GitHub repositories of that name;
  no live .com/.org/.dev/.io domain; no active tool of that name in the same
  problem space. Record each check with its date, as the earlier prompts did.
- Prefer the fleet's stone family (`-lith`) or ancient, paleo,
  Buddhist/Vedic themes. Coined compounds pass more often than dictionary
  words.
- The concept stays if the name changes. Every file refers to "the paradigm"
  in prose and uses the name only for the crate, the binary and the repo.

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
  roster do), repository skeletons (seeded elsewhere), CI workflow rendering,
  or any member's product decisions.
- The paradigm is a statement plus a checker. It never edits a member
  repository.

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

# Non-goals

- Not a template generator or repository seeder.
- Not a second gate: the paradigm never runs lint or test steps itself.
- Not a process for non-Rust or non-nix repositories in its first version;
  the lefthook-era repositories stay out until they move to hk.
- Not an agent harness: usage limits, session budgets and agent scheduling
  belong to the harness, not to repositories.

