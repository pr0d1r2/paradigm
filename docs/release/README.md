# Release is a tool, and a human approves it

Releases go through `cargo-release`, with a Keep a Changelog file and a
version ladder that says what each rung promises. Anything that leaves the
repository, such as a publish, a push to another repository or a post,
needs a human to approve it explicitly.

## Why

A registry artifact cannot be taken back: a yanked crate is hidden, not
deleted. That makes publishing the one step where an agent's mistake is
permanent and public, so it is the step a human owns. Using the release
tool rather than a script keeps the mechanical checks (clean tree, allowed
branch, tag scheme, dry run first) in something already tested.

## Evidence

- Public members already publish this way: on 2026-10-16 pklith was at
  0.1.0 on crates.io, sherd at 0.5.4, microlith at 0.7.4 and itok at 0.3.1.
- microlith wrote a release script of about 250 lines with 36 tests before
  anyone asked whether `cargo-release` existed (its B24). It did, in the same pinned
  nixpkgs, and did every check the script did.
- This repository is never released as a package. Its version is the git
  tag members pin; its checker, `sherdwright`, stays unpublished until the
  owner approves a release.

## How to do it

1. Configure `release.toml` for `cargo-release`; keep `CHANGELOG.md` in
   Keep a Changelog form.
2. Decide what each version rung guarantees before the first publish.
3. Run the dry run, show it to the human, and publish only on their word.

## Checked by

`paradigm/V32` in [`SPEC.md`](../../SPEC.md). It has no runner yet, so the
checker reports it as a review item.

## Where next

A bug found after release goes through [backprop](../backprop/README.md).
