# Waves: isolated workers, one node each

Once the spec has independent nodes, `sherd wave` orders them into rounds.
In each round every ready node is built by its own agent, in its own git
worktree, test first, and merged back one node per merge. Waves start only
after the dev shell and the guardrails are in place.

## Why

Parallel agents are fast only while they cannot touch each other. Each
shares the repository's git directory, its hooks and the machine, and every
one of those has broken a build when two workers used it at once. Isolation
is what makes the speed safe, and the gate must already exist, because no
human is watching the workers.

## Evidence

From private repositories that ran waves, before these rules existed:

- Tests that ran a scratch `git` inherited the hook's `GIT_*` variables and
  re-entered the hooks: 432 hk processes and fork failures.
- A scratch `git init` wrote `core.worktree` into the shared `.git/config`,
  so every worktree acted on one checkout.
- hk's `git stash` is shared by all worktrees and moved files between
  workers.
- Mutation testing in every worker pushed the load to 170 on 10 cores.
- Worktrees were never cleaned: 19 worktrees (281 MB) and 20 branches left
  in one repository.

## How to do it

1. Land the dev shell and the guardrails first.
2. Run `sherd wave` and build one round at a time.
3. Give each node its own worktree and its own agent, and keep pre-commit
   free of `git stash` (this repository's gate refuses `stash` in a hook,
   and refuses `core.worktree` in shared config).
4. In tests, start child git with the `GIT_*` variables removed.
5. Size the worker count to the machine, and run heavy steps one job per
   worker.
6. When a node merges, remove its worktree and branch in the same session.

This repository built its first wave sequentially, in one checkout, at the
owner's request; the rules above apply as soon as workers run side by side.

## Checked by

`paradigm/V13` (guardrails first), `paradigm/V14` (one node, one worktree,
test first), `paradigm/V15` (no shared mutable git state),
`paradigm/V16` (worktree cleanup) and `paradigm/V17` (load), in
[`SPEC.md`](../../SPEC.md).

## Where next

Every wave ends with [reconciliation](../reconciliation/README.md): the
spec is brought back in line with what the workers built.
