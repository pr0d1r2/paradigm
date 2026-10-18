# One pinned shell for agents, hooks and CI

Every tool the repository runs comes from one nix flake, pinned by
`flake.lock`, and enters with `direnv` on `cd`. The agent, the git hooks and
CI all use that same shell. Its nixpkgs is followed from one fleet-wide
lock, never spelled per repository.

## Why

A gate is only as good as its agreement with itself. If the hook on a
laptop runs one version of a linter and CI runs another, a commit can be
green in one and red in the other, and an agent with nobody watching cannot
tell which to believe. Pinning the shell removes the disagreement instead
of policing it. Following one nixpkgs for the whole fleet means a compiler
upgrade is one reviewed change, not one per repository.

## Evidence

- This repository's shell comes from pklith's `lib.devShell`, plus the Rust
  toolchain and the spec tools; on 2026-10-17 it gave hk 1.58.1 and rustc
  1.95.0, the compiler of the fleet's nixpkgs pin.
- A gate step checks that `flake.lock` holds exactly one nixpkgs.
- The shell has a hole it says out loud: hk refuses to run hooks outside it.
  A commit made from a plain terminal stops with "hk is not on PATH, so
  nothing was checked", which is the point: a gate that cannot run has not
  passed.
- "Same shell" includes what the shell inherits.
  [pr0d1r2/age-plugin-se PR #9](https://github.com/pr0d1r2/age-plugin-se/pull/9),
  merged on 2026-10-18 as `e9e7a34`, pins Xcode 26.6 for its CI
  job by setting `DEVELOPER_DIR`, but `nix develop` overwrites
  `DEVELOPER_DIR` with the stdenv's Apple SDK before the shell hook runs,
  so the pin was dropped without a word and the hook fell back to the
  image's default Xcode. The fix hands the pin over as
  `HOST_DEVELOPER_DIR`, read first by the hook, and the commit shows the
  guard refusing a bad one (`131b65e`). A CI run on its final head entered the
  shell on Swift 6.3 from the pinned Xcode
  ([run 39136577671](https://github.com/pr0d1r2/age-plugin-se/actions/runs/39136577671)).
  Recount with the `xcode=` argument in `git show e9e7a34^2:.actlith` in
  that repository and the run log's `toolchain:` line.

## How to do it

1. Write `flake.nix` with `nixpkgs.follows` set to the fleet lock, and a dev
   shell holding every tool the gate names.
2. Put `use flake` in `.envrc` and run `direnv allow`.
3. Ask the human to reload the shell, then continue from inside it.

## Checked by

`paradigm/V20` in [`SPEC.md`](../../SPEC.md): flake, direnv, pinned tools,
one nixpkgs.

## Where next

Inside the shell, [guardrails](../guardrails/README.md) go in before any
product code.
