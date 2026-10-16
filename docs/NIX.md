# The paradigm in Nix

Status: draft, written from [claudinix](https://github.com/pr0d1r2/claudinix),
[nix-fakecloud](https://github.com/pr0d1r2/nix-fakecloud),
[nix-hk](https://github.com/pr0d1r2/nix-hk),
[nixpkgs-lock](https://github.com/pr0d1r2/nixpkgs-lock) and two private
nix-heavy repositories. Numbers are from 2026-10-16.

Nix is the floor every other page stands on: the dev shell pins every tool
the gate runs, so a laptop and a CI runner run the same versions. This page
is about repositories where nix is also the product.

## Format and lint

nixfmt (check on commit, fix on demand), `statix check .` and
`deadnix --fail`, each an hk step.

## Flake input hygiene

- One nixpkgs revision for the whole fleet, from `nixpkgs-lock`; every
  input `follows` it.
- A lock-graph guard: an exact set of root inputs, exactly one nixpkgs
  node, and the `follows` in place.
- A binary cache substituter in `nixConfig`, so entering the shell does
  not build hk from source.

## Flake checks

`nix/checks.nix` with one script per check. On push, `nix flake check`
runs only when the push touches a file the flake reads: a `.nix` file,
`flake.lock`, or a path a tracked `.nix` file names.

## Testing modules

- NixOS modules have `pkgs.testers.runNixOSTest` VM tests next to them
  (35 in one member).
- bats files call `nix eval` and `nix build` for the rest.
- Linux-only work from macOS goes to a remote Linux builder, with a guard
  that keeps darwin evaluation honest.

## Binary cache

CI pushes builds to cachix (never from fork PRs) and verifies signatures.

## Pin bumps

An automated upstream check opens a PR that bumps the pin; the gate
decides whether it lands.

## Federation

A node is a module area or a cluster of scripts and flake outputs, as in
[the shell page](SHELL.md).

## Gaps against the Rust flow

- The repositories that supply the gate have no hk gate of their own:
  nix-hk runs CI only, and nixpkgs-lock still has a lefthook file.
- No coverage measure for nix itself; VM tests and flake checks stand in.

## Moving off lefthook

The legacy runner pulled about 40 `nix-lefthook-*` flake inputs, each with
a `-src` twin, which bloated every lock file. hk plus pklith replace them
with one input and a generated step list.

## Anti-patterns

- `nix flake check` on every push regardless of what changed.
- More than one nixpkgs in the lock graph.
- Inline shell in nix strings: see [the shell page](SHELL.md).
