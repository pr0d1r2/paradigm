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
