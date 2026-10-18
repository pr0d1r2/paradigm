# Intent comes first, in a file

Before any code, and before any spec, a human writes down what the tool is
for in one file: `SPEC-PROMPT.md`. An agent reads that file at every stage
that follows, so the intent is written once, where it can be argued with,
rather than spread over a chat history nobody can review.

## Why

An agent works without anyone watching it. What it builds is decided by
what it was told, and a chat log is a poor place to have told it: it has no
order, nobody can review it, and the next session cannot see it. A file in
the repository has all three. It can be diffed, reviewed and answered, and
it stays readable after the spec it produced has moved on.

## Evidence

- This repository's own prompt grew in 28 commits on 2026-10-16 before the
  spec run, and ended with 53 numbered decisions in 7 dated rounds
  ([`SPEC-PROMPT.md`](../../SPEC-PROMPT.md)).
- In the two private repositories that ran the whole lifecycle, the order
  seen in history is the same: prompt, naming with evidence, a dated
  survey, numbered decision rounds (one had 4 rounds and 23 decisions),
  then the spec.
- The two earliest prompts in the fleet repeat the same guardrail roster
  almost word for word, and the later one points at the earlier with "same
  as". The earlier one was already frozen, so the pointer rots. That is why
  a prompt now cites the paradigm by version and states only deviations.
- [pr0d1r2/age-plugin-se PR #1](https://github.com/pr0d1r2/age-plugin-se/pull/1),
  merged on 2026-10-18, is a public run of the whole inception on a fork
  of an upstream age plugin. Its intent is 1 commit, `26bdbe0`: a prompt
  with 8 decisions and 6 open questions. Recount the commit with
  `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 57ccea1..3080ab9^2 | grep -c '^intent$'`
  in that repository.
- That draft broke section order (`paradigm/V3`) the same way this
  repository's prompt did (B2): `# Process` sat after `# Open questions`.
  Gap round 1 found it and folded the section into decision 10
  (`37dadd7`).
- Its first version also named a private repository while the pull
  request was already public. The prompt's own rule said to check every
  repository named before the first push, but it was prose with nothing
  to run it: a repository has no gate at the intent stage. Removing the
  name meant rewriting the branch and force-pushing, and rewritten public
  history is not fully gone. So the name guard
  ([`paradigm/V37`](../../SPEC.md)) starts too late for this stage until
  something runs it before the first push.

## How to do it

1. Start the file with a status line: `DRAFT INPUT`. It changes to
   `HISTORICAL INPUT` in its own commit once the spec run lands.
2. Near the top, write `Paradigm: paradigm <tag>`, then only what differs
   from that version. Never copy the roster or the conventions.
3. Keep the sections in this order: status; name and one-line pitch;
   problem and goals; naming with dated availability evidence and rejected
   names; a dated survey with numbers; a pointer to a companion catalogue,
   if any; a config sketch; a CLI sketch; decisions in numbered, dated
   rounds; non-goals; prior art and rejected designs; open questions.
4. Give every number a date and a way to count it again.

## Checked by

[`paradigm/V1`](../../SPEC.md) (a prompt exists before the first spec, with
its status line), `paradigm/V2` (cite, do not copy), `paradigm/V3` (section
order) and `paradigm/V4` (dated numbers). This repository breaks V3 itself
and says so: its prompt froze without a config or CLI sketch (bug B2 in the
[defect record](../LLM-DISCLAIMER.md)).

## Where next

The prompt is not finished when it is written. [Gap rounds](../gap-rounds/README.md)
read it against the fleet and turn every open question into a decision.
