# Every bug leaves a rule behind

A bug found after the gate was green gets a row in the spec's bug log
(`§B`) and, wherever possible, the invariant (`§V`) that would have caught
it. The public record lists each one with the rule added, in the
repository's `LLM-DISCLAIMER.md`.

## Why

Fixing a bug fixes one instance. The invariant fixes the class, and puts a
check in front of the next agent that would otherwise make the same
mistake. Publishing the record is part of the method: a project that shows
its defects can be trusted about its other claims, and one that hides them
cannot.

## Evidence

- This repository logged 3 bugs on 2026-10-17, its first day with a spec,
  each with its rule ([defect record](../LLM-DISCLAIMER.md)). The first,
  stage trailers git could not parse, became V43 and a commit-msg step that
  asks git itself.
- In sherd, a rule that lived only in prose was lost about 20 times in one
  session before a runner enforced it (sherd's B12).
- In sherd, a claimed 95x speedup was measured again and was 2.1x
  (sherd's B4). The record keeps the number that was wrong.

## How to do it

1. Trace the cause, not the symptom.
2. Ask whether an invariant would catch a recurrence. If yes, add it to
   `§V` with a checker.
3. Append the `§B` row: date, cause, the invariant that now catches it.
4. Add the row to `docs/LLM-DISCLAIMER.md`, naming the fix.
5. If the cause could recur in a sibling node, open a sweep task.

## Checked by

`paradigm/V33` (bug, invariant, public record) and `paradigm/V34`
(counts in prose are generated, not typed), in [`SPEC.md`](../../SPEC.md).

## Where next

A rule that proves itself in one repository may belong to the whole fleet:
[propagation](../propagation/README.md).
