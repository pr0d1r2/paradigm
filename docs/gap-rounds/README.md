# Gap rounds turn questions into decisions

An agent reads the prompt against the fleet and lists what is missing,
what contradicts itself and what nobody has decided. The human answers,
and each answer is appended to the prompt as a numbered decision in a new,
dated round. The round ends when its questions are closed, not when the
prompt looks long enough.

## Why

A question nobody asked does not stay open: the agent answers it while
building, quietly, and the answer lands as law. A gap round makes the agent
ask before it builds, and makes the answer a decision with a number that
later commits, specs and pages can cite. Numbers only work if they stay put,
so an earlier decision is edited only to align with a later one, and the
edit names the decision that amended it. It is never silently reversed.

## Evidence

- This repository ran 7 rounds on its own prompt before the spec run. Round
  5 alone closed 8 open questions; round 7 amended decision 44 in place,
  and decision 44 now points at decision 53 instead of silently losing its
  last sentence ([`SPEC-PROMPT.md`](../../SPEC-PROMPT.md)).
- Gap rounds do not stop at the spec. In one private repository about 15
  commits sat between the first spec and the commit that froze its prompt:
  a security surface, research rows for platform unknowns, release order,
  named profiles.
- [pr0d1r2/age-plugin-se PR #1](https://github.com/pr0d1r2/age-plugin-se/pull/1),
  merged on 2026-10-18, shows the same in public. Gap round 1 took 4
  commits, one topic each, for decisions 9 to 20, and closed all 6 open
  questions. Round 2 is 1 commit, decisions 21 and 22, and it came from
  probes run while writing the spec, not from reading the prompt: Command
  Line Tools ship no XCTest, and a node `SPEC.md` inside a SwiftPM target
  makes the build warn (`576b77d`). Recount with
  `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 57ccea1..3080ab9^2 | sort | uniq -c`
  in that repository.
- In the same pull request the owner said to proceed without stopping, so
  the agent answered round 1 with its own recommendations. The round's
  heading says so, and each answer stands until the owner amends it by a
  later decision. The prompt format has no way yet to mark who answered a
  single decision, so that note covers the whole round.
- Rounds that leave no file are still recorded: from round 5 on, every
  commit carries a `Paradigm-Stage: gap-round-<n>` trailer, which the
  checker reads from history.

## How to do it

1. Ask an agent for the gaps, contradictions and undecided questions in the
   prompt, read against the repositories it describes.
2. Answer them. Append the answers as `# Decisions, round N (date)`, each
   numbered after the last.
3. Move each answered entry out of `# Open questions`, keeping its number,
   and point it at the decision that closed it.
4. Mark each decision local or a candidate for the paradigm. A candidate is
   promoted only after the member that made it ships.
5. Commit each topic separately, with the `Paradigm-Stage:` trailer
   directly above `Co-Authored-By:`, so git parses it.

## Checked by

[`paradigm/V5`](../../SPEC.md) (append, align, never reverse) and
`paradigm/V6` (local or candidate). The trailer rule is `paradigm/V36`, and
`paradigm/V43` says git must be able to parse it: this repository wrote its
first 38 stage lines in a way git could not read (bug B1 in the
[defect record](../LLM-DISCLAIMER.md)).

## Where next

When no open question blocks a node, the [spec run](../spec-run/README.md)
turns the prompt into `SPEC.md`.
