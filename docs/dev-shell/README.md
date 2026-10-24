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
- "Same shell" also means the same shell on every system the member
  builds for: `aarch64-darwin`, `x86_64-darwin`, `x86_64-linux` and
  `aarch64-linux` since the owner's decision of 2026-10-24
  (`paradigm/V28`), because a laptop's pre-push gate cannot run without
  one. Until 2026-10-21 pklith's `lib.devShell` refused `x86_64-darwin`.
  [pr0d1r2/pklith PR #103](https://github.com/pr0d1r2/pklith/pull/103),
  merged that day as `028c65c`, adds the system to its one `systems`
  list. microlith and itok publish no package for it, so pklith builds
  them from their locked source there; on the other three systems the shell
  and catalog derivations are unchanged.
  [pr0d1r2/age-plugin-se PR #23](https://github.com/pr0d1r2/age-plugin-se/pull/23),
  merged the same day as `6e4d47d`, then gives that member a dev shell
  on `x86_64-darwin`. Its first `nix develop` on an Intel Mac builds hk,
  sherd, microlith, itok, pklith and actlith from source, since no cache
  holds them for that system. Both PRs evaluated the Intel shell without
  building it: neither author had an x86_64 builder.
- The system has an end date. nixpkgs warns that 26.05 is its last
  release with `x86_64-darwin` (pklith's T76, first seen 2026-10-21).
  The decision falls before any bump of the fleet's nixpkgs lock past
  26.05 (root T21). Recount with
  `git show 028c65c:flake.nix | grep -A5 'systems ='` in pklith, T76 in
  pklith's `SPEC.md`, and `git show 6e4d47d:flake.nix` in age-plugin-se.

## How to do it

1. Write `flake.nix` with `nixpkgs.follows` set to the fleet lock, and a dev
   shell holding every tool the gate names, for each of the four systems.
2. Put `use flake` in `.envrc` and run `direnv allow`.
3. Ask the human to reload the shell, then continue from inside it.

## Checked by

`paradigm/V20` in [`SPEC.md`](../../SPEC.md): flake, direnv, pinned tools,
one nixpkgs, a shell on every system `paradigm/V28` names.

## Where next

Inside the shell, [guardrails](../guardrails/README.md) go in before any
product code.
