# The unflattering record

Every defect found after the gate was green is published, with the rule
added so it cannot recur. Every number in a page carries the date it was
true and a way to count it again. When a number turns out wrong, the
correction is shown, not swapped in quietly.

## Why

A project built by agents makes claims about itself that a reader cannot
easily check. The only thing that earns trust is a record that shows the
failures as plainly as the successes, and numbers that anyone can recount.
A number reported in the direction that flatters the work is the first one
to measure again.

## Evidence

- This repository's [defect record](../LLM-DISCLAIMER.md) lists 3 bugs from
  its first day with a spec, including one in its own convention: 38 stage
  trailers that git could not read.
- Typed numbers are listed in a [numbers ledger](../NUMBERS.md) with a date
  and a recount, and the ledger is expected to shrink as a generator takes
  them over.
- In sherd a 95x speedup was measured again and was 2.1x; in microlith a
  document of measured evidence carried 3 numbers that were false. Both
  records keep the wrong number next to the right one.
- One member's disclaimer stated counts of invariants and bugs that had
  drifted from its own spec, which is why counts in prose must be generated.

## What it takes

- [Backprop](../backprop/README.md): every bug gets a row and, where
  possible, a rule.
- A public `LLM-DISCLAIMER.md` with the defect record.
- Dated numbers, and generated counts wherever a generator exists.

## Checked by

`paradigm/V4` (dated numbers), `paradigm/V33` (public defect record) and
`paradigm/V34` (generated counts), in [`SPEC.md`](../../SPEC.md).

## Where next

How a bug becomes a rule: [backprop](../backprop/README.md).
