# Federation: one spec per directory

A spec that grows past what one agent can read at once is split along the
directory tree. Each directory that owns something gets its own `SPEC.md`,
the parent lists it in a `§F` table, and `sherd` derives the navigation
(`§N`) between them. A worker building one node reads its chain, the root
plus every ancestor plus the node, and nothing else.

## Why

The cost of an agent's context is paid before it writes a line. One file
for the whole repository grows until no worker can hold it, and then every
worker reads everything to change one thing. A node per directory keeps the
read bounded by the work: the chain is small, it is measured, and a
ceiling in `.context-limits` turns growth into a reviewed decision instead
of a drift.

## Evidence

- Federation is already the norm in the public members: 21 nodes in
  pklith, 22 in sherd, 46 in xenolith, counted on 2026-10-16.
- This repository federates into 23 nodes. On 2026-10-17 its root chain cost
  4,207 tokens and its deepest chain 5,062, each with a ceiling about 18%
  above that.
- Ceilings that sit too close to the measurement churn: in two private
  repositories `.context-limits` changed 12 and 7 times, and one ceiling
  moved from 1,700 to 2,100. Slack of 12 to 20% held.
- [pr0d1r2/age-plugin-se PR #1](https://github.com/pr0d1r2/age-plugin-se/pull/1),
  merged on 2026-10-18, federates a Swift fork into 5 nodes, in a
  language `sherd` cannot read. `sherd wave` put all 5 in one round,
  because it follows only Rust edges (sherd#132), so the wave plan is a
  hand-written task row (`ce0ae61`). Two upstream directories that are not
  nodes still got `§F` rows, without a `SPEC.md`, so the table lists
  every directory (`ef938c4`).
- The root's task table rides in every child chain (sherd#129). Two new
  rows left the root at 2,440 of its 2,600-token ceiling, 6% slack, and
  the ceilings were raised the same day in their own commit with the
  measurement (`a601247`). Recount the nodes with
  `git ls-tree -r --name-only 3080ab9 | grep -c 'SPEC.md$'` and the
  chains with `sherd budget` in that repository.

## How to do it

1. Give each directory that owns a concern its own `SPEC.md`, with `§G`
   saying what it owns.
2. List each child in the parent's `§F` table: what it owns, and what it
   does not own and where that lives.
3. Run `sherd sync` to write every `§N`. Never edit `§N` by hand.
4. Measure with `sherd budget` and set each node's ceiling in
   `.context-limits` at the measurement plus 12 to 20%. A raise is its own
   commit with the new measurement.

## Checked by

`paradigm/V11` (a node per directory, generated navigation) and
`paradigm/V12` (ceilings with slack, raised only with a measurement), in
[`SPEC.md`](../../SPEC.md).

## Where next

Independent nodes can be built side by side, in
[waves](../waves/README.md).
