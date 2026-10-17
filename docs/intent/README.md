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
