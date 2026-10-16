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

