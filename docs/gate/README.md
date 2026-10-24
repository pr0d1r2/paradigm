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
- [pr0d1r2/age-plugin-se PR #5](https://github.com/pr0d1r2/age-plugin-se/pull/5),
  merged on 2026-10-18 as `bd8b6a7`, lays a gate of 36 steps on
  commit, 3 more on push and 2 on commit-msg in a Swift fork of an
  upstream age plugin. `swift test` and `sherd coverage --check` run on
  push, against a `.coverage` floor of 87.67% of the lines under
  `Sources/`, written by `sherd coverage --record` (`b29993a`,
  `paradigm/V25`). The push step and the flake's `checks.<system>.tests`
  run the same `nix/test.sh`, so nix and the hook cannot disagree about
  what a test run is (`fa0b766`, `2bc41af`). Recount the steps with
  `R10` in `git show bd8b6a7^2:SPEC.md` and the floor with
  `git show bd8b6a7^2:.coverage` in that repository.
- Some rows nix cannot supply. The toolchain, build
  (`-warnings-as-errors`) and test steps run the host Xcode's Swift, which
  pklith's catalog cannot name yet (pklith#89), so they are local
  `.pklith` rows with no nix package.
- A member without Rust can still run the name guard
  ([`paradigm/V37`](../../SPEC.md)). `gate/names.sh` is the denylist half
  in shell, with bats tests, and reports a hit by entry number without
  printing the name (`513994a`). The allowlist half waits, because which
  third-party repositories go on that list is the owner's call.
- Its `.context-limits` has two meanings: `sherd budget` reads it as
  chain ceilings per node, while itok reads the same file as ceilings per
  file and refuses the root `SPEC.md`. So itok is not in that gate yet
  (`8b4151a`). It is the same clash `.coverage` had before
  `paradigm/V25` gave it one meaning.
- [pr0d1r2/age-plugin-se PR #9](https://github.com/pr0d1r2/age-plugin-se/pull/9),
  merged on 2026-10-18 as `e9e7a34`, takes the same gate into CI.
  actlith renders `.github/workflows/ci.yml` from a one-row `.actlith`
  registry, and the job's command is the hooks' own
  `hk check --all --check`, so CI holds no second gate definition
  (`1ed70b9`, `paradigm/V23`). The job pins Xcode 26.6 on `macos-latest`
  through actlith's `xcode=` argument. A missing Xcode fails the job; it
  never falls back to the image default. Its run on the final head
  passed 43 steps in 3m08s, build, `swift test`, coverage and
  `release-tests` included
  ([run 39136577671](https://github.com/pr0d1r2/age-plugin-se/actions/runs/39136577671)).
  All 17 of its commits carry `Paradigm-Stage: guardrails`. Recount the
  steps with `pkl eval -x 'hooks["check"].steps.keys.length' hk.pkl` at
  `e9e7a34^2` and the commits with
  `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' e9e7a34^1..e9e7a34^2 | grep -c '^guardrails$'`
  in that repository.
- The workflow is gated like code. Three new steps, `actlith-check`,
  `actionlint` and `zizmor`, and `yamllint` widened to the fork's
  `ci.yml`, landed one per commit, each body showing a planted violation
  refused: a hand-edited timeout and a stray workflow file, an undefined
  matrix key, a template injection, a duplicate `jobs:` key (`1ed70b9`,
  `57c2bd0`, `28ac57f`, `e4fbe56`, `paradigm/V21`). Only the untouched
  upstream workflows stay exempt.
- Its CI runs on one system, `aarch64-darwin`: a Secure Enclave plugin
  has only macOS (the member's decision 12, raised as paradigm#2).
  Making `gate (macos-latest)` a required check on `main` is an owner
  step in that repository (its T22), not yet done.
- After its characterization run (see [waves](../waves/README.md)), the
  same member bounds what its floor leaves out.
  [PR #17](https://github.com/pr0d1r2/age-plugin-se/pull/17), merged on
  2026-10-18 as `30d0ee3`, adopts sherd's function exclusions
  (sherd#141, sherd#142): four `exclude <path>:<declaration> <reason>`
  rows in `.coverage`, one per Secure Enclave key function, under
  `excluded 22`, a cap on excluded lines that `sherd coverage --record`
  only lowers. The floor is then 99.56% (1,129 of 1,134 lines), and the
  5 lines still unrun stay counted, not excluded. The floor may only
  rise (`paradigm/V25`) and the cap beside it may only fall, so the gap
  cannot hide in exclusions. Recount with `git show 30d0ee3:.coverage`.
- [PR #16](https://github.com/pr0d1r2/age-plugin-se/pull/16) mirrors
  each `Sources/<Stem>.swift` in `Tests/Fork/<Stem>ForkTests.swift`,
  checked by two `.pklith` rules rows, `exists` and `orphan`.
  [PR #18](https://github.com/pr0d1r2/age-plugin-se/pull/18) adds a
  `tdd-order` row (pklith#98): a source changes only with or after its
  mirror test, judged commit by commit at pre-push and over the pull
  request's range in CI (`paradigm/V24`).
- [PR #19](https://github.com/pr0d1r2/age-plugin-se/pull/19) tests the
  excluded functions on hardware. A pre-push `hardware` row runs a
  Secure Enclave suite on the Mac, and prints a notice and runs nothing
  when `CI` is set, so the floor stays the number CI measures. Recount
  the rows with
  `git show 6a321ef:.pklith | grep -E 'exists|orphan|tdd-order|^hardware'`
  in that repository.
- The same member is where the fleet's platform matrix comes from
  (`paradigm/V28`, as the owner amended it on 2026-10-24).
  [PR #22](https://github.com/pr0d1r2/age-plugin-se/pull/22), merged on
  2026-10-21 as `590eea2`, lowers its floor to macOS 14
  (`.macOS(.v14)`) so an Intel MacBook Pro on macOS 15 can be a second
  dev and hardware-test Mac. ML-KEM-768 needs macOS 26, so every
  declaration of it sits behind `@available(macOS 26.0, *)`, and below 26
  each post-quantum path throws `pqUnavailable` instead of trapping. A
  new pre-push row, `gate/universal.sh`, builds release for arm64 and
  for x86_64, joins them with `lipo`, and refuses unless exactly those
  two slices are present, each with `minos 14.0` by `vtool -show-build`.
  Its commit shows it refusing `Package.swift` set back to `.v26`
  (`5dfaba5`). The row sits in the `test` category, so it is a declared
  test step (`paradigm/V24`), and CI runs it too, cross-building the
  x86_64 slice on the arm64 runner. There, and on an Apple silicon Mac,
  that slice is built and inspected, never run; running it is what the
  Intel laptop's own pre-push is for.
- A floor recorded on the newest macOS does not hold on an older one.
  [PR #24](https://github.com/pr0d1r2/age-plugin-se/pull/24), merged on
  2026-10-23 as `42470ac`, makes the pre-push `coverage` row read the
  macOS major version and, below 26, print a notice and exit 0, because
  the post-quantum code cannot run there. A version it cannot read is
  refused, never skipped. CI and macOS 26 hosts hold the floor.
- Its flake builds for the two Mac systems only, so the Linux pair the
  amended `paradigm/V28` names is still open there, and no CI runner
  covers `x86_64-darwin` yet: actlith maps that system to no GitHub
  runner and refuses it (its `SPEC.md` §I at `3cbb360`, 2026-10-24), and
  GitHub's Intel macOS runners are shrinking. Whether CI gets GitHub's
  Intel image or a self-hosted runner is open as root T19; until then an
  Intel Mac is covered by its own pre-push gate only. Recount with
  `git show 590eea2:gate/universal.sh`, `git show 590eea2:Package.swift`,
  `git show 42470ac:gate/coverage.sh` and
  `git show 6e4d47d:flake.nix | grep -- '-darwin\|-linux'` in that
  repository, and the `runners:` line of actlith's `SPEC.md`.

## How to do it

1. Keep one `hk.pkl`. Commit, push and CI hooks all reference its steps;
   [actlith](https://github.com/pr0d1r2/actlith) renders the GitHub Actions
   workflow that runs it in CI.
2. Put tests and coverage on pre-push; say `RED` in the subject of a commit
   that is red on purpose, and make it compile.
3. Give `.coverage` one meaning, a ratchet floor, and let only
   `sherd coverage --record` move it.
4. Build and test on `aarch64-darwin`, `x86_64-darwin`, `x86_64-linux`
   and `aarch64-linux`. Push from each laptop so its pre-push runs
   natively; let CI cover the rest.
5. Set the macOS floor to 14.0 (Rust: `[env]` pin with `force = true`,
   `.macOS(.v14)` in Swift), gate newer APIs with `@available` and a
   clean error, and add the universal check as a test-stage step.
6. Use clap for arguments; one mirrored `.bats` per shell script.
7. Protect `main` once `no-commit-to-branch` lands.

## Checked by

`paradigm/V23` to `paradigm/V31` in [`SPEC.md`](../../SPEC.md): one gate,
declared test stage, one `.coverage` meaning, gate suppliers gated, ids in
commits, four systems with a macOS 14.0 floor and a universal Mac
binary, clap, shell tests and Ruby mutation off the gate.

## Where next

What passes the gate can be [released](../release/README.md).
