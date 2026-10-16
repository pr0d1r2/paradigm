# The paradigm in shell

Status: draft, written from [claudinix](https://github.com/pr0d1r2/claudinix)
and two private shell and nix repositories (an appliance image and fleet
infrastructure; 109 to 153 scripts each, almost all with a mirrored test).
Numbers are from 2026-10-16.

Shell is where agents most often hide logic where no test can reach it:
inside functions, justfile recipes, nix strings and hook configuration.
Most of the rules below exist to pull that logic out into files that can be
linted and tested one by one.

## Stack

bash, shellcheck, shfmt (indent from `.editorconfig`), and plain bats
without helper libraries, all from the nix dev shell.

## One script, one job

No shell functions except logging (`info`, `warn`, `fatal`, kept in an
allowlist). Anything else becomes its own script. That is what makes one
test per script possible.

## Extract, never embed

- Justfile recipes are one line: `bash scripts/x/y.sh {{args}}`.
- Nix modules load shell with `builtins.readFile`, never inline.
- [xenolith](https://github.com/pr0d1r2/xenolith) refuses programs
  embedded in `.nix`, `.pkl`, justfiles and `.envrc`, and finds scripts
  that are loaded but missing, or present but never loaded.

## Mirror tests

`scripts/a/b.sh` pairs with `tests/unit/scripts/a/b.bats`, checked in both
directions: a script without a test fails, and so does a test whose script
is gone, because an orphan test passes forever. In `.pklith` this is an
`exists` rule and an `orphan` rule.

## Writing a bats test

Build a fixture tree under `$BATS_TEST_TMPDIR`. Stub commands through
environment seams that each script names in its header (`NIX`, `KCOV`, and
so on). A failing test says what to do.

## TDD order from history

On push, a commit that adds a script must have its mirrored test already in
the parent commit. The check follows renames, refuses shallow clones, and
prints how to split the commit.

## Coverage

The mirror is the floor that works today. Line coverage with kcov is
specified (a per-script floor in `.coverage`, ratcheting up, skips only
with a reason) but runs on Linux only, so a Mac sends it to a Linux
builder. It is not gated anywhere yet.

## `.pklith` rows

`sh` and `.envrc` get shellcheck, shfmt and xenolith; `bats` gets a parse
check, shellcheck and shfmt. Fixtures, lure files and zsh (no shellcheck
mode) get rows that say "no checks, because ...": the reason is mandatory,
so nothing is unchecked by accident.

## Commit versus push

Fast linters on commit. bats, the mirror and TDD order on push, because a
RED commit is red on purpose.

## Federation

A node is a directory that owns a cluster of scripts or a module area, not
a single script. Interface rows name the script and its behaviour;
invariants name the bats file that proves them.

## Waves and CI

Waves use worktrees landed one at a time. CI runs the same gate:
`nix develop -c hk check --all`, then `nix flake check`.

## Gaps against the Rust flow

- Line coverage is not gated.
- Some members still hand-write `hk.pkl` instead of rendering `.pklith`.
- One member has no CI and no PR-only main.
- Release is a script, with nothing like a semver or package check.

## Anti-patterns

- Shell functions: they cannot be tested one at a time.
- Inline shell in nix or justfiles: it escapes lint and tests.
- Tests only one way round: orphans pass forever.
- bats on pre-commit: RED commits become impossible.
- Agent worktrees inside the repository and not ignored: a `SPEC.md` glob
  found 124 files instead of 21.
