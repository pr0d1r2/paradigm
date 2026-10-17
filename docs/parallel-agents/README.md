# Parallel agents need walls, not supervision

Several agents can build one repository at once, each on its own node of
the spec, in its own git worktree. That works only when nothing mutable is
shared between them: not the git stash, not the git config, not the test
database, not the machine's cores. The walls are rules a gate checks, not
habits.

## Why

Every shared thing becomes a channel between workers. One worker's scratch
command changes another's checkout; one worker's stash pops into another's
tree; ten workers each running the heaviest step at once take the machine
down. None of these shows up when one agent works alone, which is why they
are learned the hard way the first time agents run side by side.

## Evidence

From private repositories, before the rules existed:

- 432 hk processes from tests that re-entered the hooks through inherited
  `GIT_*` variables.
- Every worktree acting on one checkout after a scratch `git init` wrote
  `core.worktree` into the shared config.
- Two workers popping each other's edits through hk's shared `git stash`.
- A load of 170 on 10 cores from mutation testing in every worker.
- 19 worktrees and 20 branches left behind in one repository; in another, 5
  unignored worktrees made a `SPEC.md` search find 124 files instead of 21.

## What it takes

- [Federation](../federation/README.md), so a worker reads only its node.
- [Waves](../waves/README.md) with one worktree per node, cleaned up after
  merge.
- A gate that refuses `stash` in hooks and `core.worktree` in shared config,
  and child git that drops `GIT_*`.
- [Reconciliation](../reconciliation/README.md) after every wave.

## Checked by

`paradigm/V14` to `paradigm/V17`, in [`SPEC.md`](../../SPEC.md).

## Where next

How a wave runs: [waves](../waves/README.md).
