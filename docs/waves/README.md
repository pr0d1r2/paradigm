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

From a public member that started from inherited code, all merged on
2026-10-18 and cited as evidence for paradigm#22:

- [pr0d1r2/age-plugin-se](https://github.com/pr0d1r2/age-plugin-se), a
  Swift fork of an upstream age plugin, recorded its coverage floor at
  87.67% when its guardrails landed, and its first waves changed
  `Sources/` with about 130 lines still unrun (PRs #6 and #11). Waves
  wait for guardrails (`paradigm/V13`), but nothing asked that the
  inherited code be pinned by tests first, so the member did it after.
- [PR #12](https://github.com/pr0d1r2/age-plugin-se/pull/12) adds tests
  only and takes `Sources/` from 88.62% (1,020 of 1,151 lines) to
  92.96% (1,070 of 1,151). Its body sorts the 131 unrun lines into three
  classes: 50 testable now, all covered there; 42 that need a refactor,
  mostly `CLI.main`'s dispatch; and 39 untestable in CI (Secure Enclave
  keys, the terminal, the entry point, an older-macOS guard). It found
  no bugs.
- [PR #14](https://github.com/pr0d1r2/age-plugin-se/pull/14) is the
  refactor class: `CLI.main` hands its dispatch to an injectable
  `CLI.run`, and the floor reaches 96.02% (1,110 of 1,156).
  [PR #15](https://github.com/pr0d1r2/age-plugin-se/pull/15) runs the
  built binary as a child process whose coverage counts, and reaches
  97.66% (1,129 of 1,156) with tests only. The exclusions and the cap
  that followed are on the [gate](../gate/README.md) page.
- All 30 commits of its PRs #12 and #14 to #19 carry
  `Paradigm-Stage: guardrails`, since the trailer vocabulary has no word
  for this stage. Recount in
  that repository with `git show <merge>:.coverage` at `c75b7fc`,
  `963b701` and `522226e`, whose notes give the lines, and
  `git diff --shortstat c75b7fc^1 c75b7fc -- Sources`, which is empty.

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
