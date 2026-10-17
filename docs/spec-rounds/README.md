# The spec gets gap and drift rounds too

A spec is not right because it passed its format checks. After the spec
run, gap rounds keep asking what it leaves undecided, and drift rounds
(`ck:check`) compare it with the code, read-only, item by item. The rounds
repeat until one finds nothing, and each round leaves a commit, even when it
is empty.

## Why

The format checker proves the spec is well formed, not that it is true.
Drift starts on day one: the first code written against a spec already
disagrees with it somewhere. A drift round that writes nothing and reports
everything gives the human the choice of fixing the code or amending the
spec, instead of letting the agent decide quietly. Recording each round as
a commit makes the rounds countable, which is how anyone can tell whether
they happened at all.

## Evidence

- This repository's first drift round, run on its fresh spec on
  2026-10-17, found 6 violations and 2 drifts. Three became bugs B1 to B3,
  each with the rule that now catches it, and the round's fixes carry the
  `Paradigm-Stage: ck-check-round-1` trailer.
- One finding was in the stage trailers themselves: written as their own
  paragraph, git parsed none of them. A drift round found it because it
  asked git, not the text.
- In private repositories the rounds left no commit named after them; they
  show only as later "decisions the spec did not cover". That invisibility
  is why every round now leaves a trailer.

## How to do it

1. Run `ck:check --all` against the spec: invariants, interfaces and tasks.
2. For each violation choose: fix the code, or record a bug with
   `ck:backprop` and amend the spec.
3. Commit the fixes with `Paradigm-Stage: ck-check-round-<n>`. A clean
   round gets an empty commit with the same trailer.
4. Repeat until a round finds nothing.

## Checked by

`paradigm/V10` in [`SPEC.md`](../../SPEC.md): rounds repeat until clean and
each leaves a trailer.

## Where next

With the spec settled, the [dev shell](../dev-shell/README.md) pins every
tool the work will run.
