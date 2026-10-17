# The spec run lands the law, one section at a time

`ck:spec` turns the prompt into `SPEC.md`: a goal, constraints, interfaces,
research, numbered invariants, tasks and a bug log, written in a compressed
"caveman" encoding an agent reads cheaply. It lands one section or one node
per commit, every commit passes the spec checks, and the prompt is then
frozen as historical input in a commit of its own.

## Why

The spec is the input every later worker reads, so a broken spec is a
broken build. Landing it a section at a time keeps each commit small enough
to review: a reader can see what the goal says before reading what the
invariants demand. Freezing the prompt afterwards settles which file is the
law. Where the two differ, the spec wins, and the prompt stays only as the
record of how the spec came to be.

## Evidence

- This repository's spec run took 32 commits: the format vendored, the
  root spec in 7 section commits, 22 node specs one per commit, the prompt
  frozen, and the landing page pointed at the spec. Every commit passed `mth check`,
  `sherd check` and `sherd validate`.
- The first drift round on it, run the next day, found that one commit had
  landed two sections together. The commit was split in history, which is
  how [`paradigm/V8`](../../SPEC.md) is meant to work: the rule finds its
  own first violation.
- The spec run waits while an open question blocks a node. Here it waited
  for a checker name and for where waivers live, which round 6 answered.

## How to do it

1. Vendor the spec format (`FORMAT.md`) so the checker and the writer read
   the same rules.
2. Run `ck:spec` from the prompt. Commit the goal, then each further
   section, then each node of the federation with its row in the parent's
   `§F` table.
3. After every commit run `mth fmt --check`, `mth check`, `sherd check`,
   `sherd validate` and `sherd budget`.
4. Change the prompt's first line to `HISTORICAL INPUT` in its own commit.

## Checked by

`paradigm/V7` (no start while a blocking question is open), `paradigm/V8`
(one section or node per commit, then the freeze) and `paradigm/V9` (the
spec checks pass after every edit), all in [`SPEC.md`](../../SPEC.md).

## Where next

A spec with independent parts is split into nodes, one per directory, in
[federation](../federation/README.md).
