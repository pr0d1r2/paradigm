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

