# Every wave ends by reconciling the spec

When a wave's nodes are merged, the spec is compared with what the workers
built. Interfaces are corrected to match the code, rules a worker invented
along the way become invariants, and the coverage floor rises to what the
wave reached. Each of these is its own commit, so the round shows in
history.

## Why

Workers make decisions the spec did not cover. Left alone, those decisions
live only in code, and the next wave builds against a spec that is no
longer true. Reconciling after every wave keeps the spec the law rather
than a plan someone once had. Raising the coverage floor in the same pass
turns what the wave achieved into a ratchet the next wave cannot fall
below.

## Evidence

- In the private repositories, drift rounds left no commit named after
  them. They show up only as "interfaces match what the workers built"
  after each wave, as backfilled "decisions the spec did not cover", and as
  "floor follows wave N" commits.
- A bug found in one node of one repository was found again, from the same
  cause, in a sibling node: a bug log row in one node does not reach the
  others unless a sweep is opened.
- This repository's first drift round, on 2026-10-17, found 6 violations and
  2 drifts against its own spec and recorded 3 bugs, each with the rule
  that now catches it.

## How to do it

1. After the last merge of a wave, run `ck:check` against the whole tree.
2. Fix `§I` where the code is right and the spec is stale; add a `§V`
   invariant for each rule a worker made up.
3. Raise the coverage floor to the measured value in its own commit.
4. If a bug's cause can recur in a sibling node, open a task that sweeps
   the siblings.
5. Mark these commits with `Paradigm-Stage: reconcile-<n>`.

## Checked by

`paradigm/V18` (reconciliation and floor commits per wave) and
`paradigm/V19` (sweep sibling nodes), in [`SPEC.md`](../../SPEC.md).

## Where next

The rules are stated in full, with their reasons, in
[`SPEC.md`](../../SPEC.md).
