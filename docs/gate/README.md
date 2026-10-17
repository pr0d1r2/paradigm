# One gate, on commit, on push and in CI

One `hk.pkl` defines every check. The same file runs on commit, on push and
in CI. Without hk the hooks refuse rather than skip, `--no-verify` is never
used, and once `main` is protected every change arrives through a pull
request, merged by rebase or merge commit and never squashed.

## Why

Two gate definitions drift, and the drift is invisible until a change
passes one and fails the other. With one file there is nothing to drift.
Refusing when hk is missing keeps "nothing ran" from looking like "all
passed". Never squashing keeps the commit trail intact, since commits cite
the spec ids they serve and the checker reads stage trailers from them.

## Evidence

- This repository splits its 36 steps by cost: 34 run on every commit, and
  tests and coverage run on push, because a commit that is red on purpose
  must be committable and tests inside commit hooks re-entered the hooks
  from parallel workers in a private repository.
- The seeded pre-commit used `git stash`, which every worktree shares. In
  hk 1.58 the alternative `"patch-file"` is only an alias of `"git"`, so the
  gate now checks without stashing, and a step refuses any stash in a hook.
- Moving tests to pre-push in one private repository let 5 red goldens be
  committed: pre-push alone does not stop a red commit, so a commit that is
  red on purpose must say so.

## How to do it

1. Keep one `hk.pkl`. Commit, push and CI hooks all reference its steps.
2. Put tests and coverage on pre-push; say `RED` in the subject of a commit
   that is red on purpose, and make it compile.
3. Give `.coverage` one meaning, a ratchet floor, and let only
   `sherd coverage --record` move it.
4. Use clap for arguments; one mirrored `.bats` per shell script.
5. Protect `main` once `no-commit-to-branch` lands.

## Checked by

`paradigm/V23` to `paradigm/V31` in [`SPEC.md`](../../SPEC.md): one gate,
declared test stage, one `.coverage` meaning, gate suppliers gated, ids in
commits, CI on three platforms, clap, shell tests and Ruby mutation off the
gate.

## Where next

What passes the gate can be [released](../release/README.md).
