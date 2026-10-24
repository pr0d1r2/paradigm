# The paradigm in Swift

Status: draft, written from one repository at its spec run:
[age-plugin-se](https://github.com/pr0d1r2/age-plugin-se), a fork of an
upstream age plugin, used daily by a private secrets tool. Its spec run
went up for review on 2026-10-18 as
[PR #1](https://github.com/pr0d1r2/age-plugin-se/pull/1), with paradigm
pinned at commit `6c7e6cd`. Nothing below has shipped: no gate step,
test or wave has run yet. Numbers are from 2026-10-18 and come from that
repository's `SPEC.md` research rows unless a section says otherwise.

The lifecycle is the same as in Rust: spec prompt, gap rounds, a federated
`SPEC.md`, guardrails first, parallel waves, TDD, a gate that refuses.
Swift changes two things. The toolchain cannot come from nix, so the dev
shell has to check the host instead of providing it. And most of the
fleet's tools read Rust only, so mirror tests, coverage and wave order
have no runner yet; this page says where each one stands and links the
issue that would close it. The headings are the ones every language page
shares (`paradigm/V3`).

## Stack

Swift 6.2 or newer, with the macOS SDK, both from the host. Swift 6.2 is
a hard floor, not a preference: Secure Enclave ML-KEM keys compile only
under `#if compiler(>=6.2)`, so an older compiler builds a stub that
quietly drops post-quantum support. Tests use XCTest, which ships only
with Xcode. swift-format and swiftlint come from the nix pin.

The deployment floor is macOS 14, `.macOS(.v14)` in `Package.swift`
(`paradigm/V28`, owner 2026-10-24). An API newer than that sits behind
`@available` and fails with a clean error: age-plugin-se's ML-KEM-768
needs macOS 26 and throws `pqUnavailable` below it
([PR #22](https://github.com/pr0d1r2/age-plugin-se/pull/22), merged
2026-10-21 as `590eea2`).

As of 2026-10-18 the fleet's nixpkgs pin (`26f4107e`) ships Swift 5.10.1,
swift-format 5.10.1 and swiftlint 0.63.2; the build Mac had Swift 6.3.3
and Command Line Tools 26.6, which carry Swift Testing but not XCTest.
Recount: `nix eval --raw` of each package's `.version` at that nixpkgs
revision, `swift --version`, and
`pkgutil --pkg-info=com.apple.pkg.CLTools_Executables`.

## Dev shell

The dev shell pins every tool it can (`paradigm/V20`) and checks the two
it cannot. nix cannot supply Xcode: it is not redistributable, nixpkgs
only offers a stub for a manually downloaded archive, and accepting its
licence needs sudo. So Xcode is installed once per machine by the
machine's own configuration, which also accepts the licence
(`xcodebuild -license accept`). The dev shell points `DEVELOPER_DIR` at
that Xcode and refuses to start, naming the fix, when Swift is older than
6.2 or XCTest is missing.

That is a stated deviation from `paradigm/V20`, which assumes nixpkgs
supplies the toolchain; how V20 should word it is open as
[paradigm#2](https://github.com/pr0d1r2/paradigm/issues/2). The pklith
side, a catalog row for a host tool with a version guard, is
[pklith#89](https://github.com/pr0d1r2/pklith/issues/89).

## Federation

A SwiftPM target is one directory, so a node is a target: `Sources/` and
`Tests/` are a node each. Other nodes are plain directories, such as the
test fixtures and the nix parts.

- A node's `SPEC.md` inside a target directory makes SwiftPM warn
  `found 1 file(s) which are unhandled`, so `Package.swift` excludes
  `SPEC.md` from each target.
- Directories that are not nodes still get a §F row saying so, so the
  federation stays exhaustive and nothing is unreachable by descent.
- In a fork, the upstream targets stay flat: moving upstream files would
  conflict with every upstream pull request. Fork work goes in new
  directories, which can then be built side by side.

## Gate map

Every check is a `.pklith` row rendered into hk steps (`paradigm/V23`).

- Style: `swift format lint --strict`, with a `.swift-format` matching
  the existing code; swiftlint for what swift-format lacks (force unwrap,
  force try, force cast) on new code.
- Build: `-warnings-as-errors`. It holds only once the tree is at 0
  warnings; age-plugin-se got there by fixing four upstream warnings
  first (4 on upstream `57ccea1`). Recount: `swift build` and count
  `warning:` lines.
- Toolchain: a step that refuses Swift older than 6.2.
- Tests: `swift test` on pre-push.

As of 2026-10-18, hk 2.7.0 has one Swift builtin, `swiftlint`, and the
pklith catalog has no Swift rows (recount: hk's `pkl/builtins/` and
pklith's `src/catalog`). So swift-format, the build and the tests start
as local rows, as Ruby's did, and are filed upstream as
[pklith#87](https://github.com/pr0d1r2/pklith/issues/87). Argument
parsing in a fork stays as upstream wrote it, with a waiver instead of
swift-argument-parser (the Swift counterpart of `paradigm/V29`'s clap)
and a test that the help text and the parser agree.

## Mirror tests

No answer yet. The Rust and shell pages pair every source file with a
test file, checked in both directions; nothing does that for SwiftPM
targets today. The rules, and a TDD-order check for XCTest, are asked
for in [pklith#88](https://github.com/pr0d1r2/pklith/issues/88). Until
they land, a Swift repository has no mirror guard.

## Coverage

Line coverage comes from `swift test --enable-code-coverage`, read with
`xcrun llvm-cov export`. The floor is meant to live in `.coverage` and
only ratchet up (`paradigm/V25`), with code that runs only against real
hardware or a real terminal exempted by `.pklith` rows that give the
reason.

No runner yet: `sherd coverage` reads cargo only, and Swift support is
asked for in [sherd#138](https://github.com/pr0d1r2/sherd/issues/138).
So the ratchet is specified but not gated anywhere.

## Commit versus push

`swift test` runs on push, not on commit, so a RED commit can land red
on purpose (`paradigm/V24`). A RED commit still compiles: tests land
with stubs that answer wrong (`paradigm/V44`). Which of the other steps
run on commit and which only on push is not settled yet; age-plugin-se
lays them one per commit in its guardrails stage.

## Waves

`sherd wave` reads Rust edges only, so a Swift repository writes its
wave order by hand in §T, after the dev shell and guardrails stages
(`paradigm/V13`). The hand-written plan also has to keep workers apart
where the code does not: at most one worker in a shared upstream target
per wave, and a test file written by one node's worker is off limits to
another worker in the same wave. A schedule for repositories like this
is asked for in [sherd#132](https://github.com/pr0d1r2/sherd/issues/132).
No wave has run yet.

## Release

No answer yet. There is no Swift counterpart to `cargo-release` in the
fleet, and none has been chosen. age-plugin-se plans fork tags of the
form `v<upstream version>-fork.<n>`, with the binary keeping upstream's
version string, and its one consumer pins the flake.

CI is open too. A Swift repository can only run its gate on a macOS
runner, and that runner needs a pinned Xcode, asked for in
[actlith#18](https://github.com/pr0d1r2/actlith/issues/18). CI on
`aarch64-darwin` alone, where `paradigm/V28` names four systems since
2026-10-24, is the other half of the deviation
[paradigm#2](https://github.com/pr0d1r2/paradigm/issues/2) records
against it. V28 leaves the Mac systems to each laptop's native pre-push
and the release binary to the universal check; the Linux pair is open.

## Gaps against the Rust flow

- The toolchain comes from the host, not from nix, so builds are not
  hermetic.
- No mirror-test or TDD-order guard
  ([pklith#88](https://github.com/pr0d1r2/pklith/issues/88)).
- No coverage ratchet runner
  ([sherd#138](https://github.com/pr0d1r2/sherd/issues/138)).
- No catalog rows for swift-format, the build or the tests
  ([pklith#87](https://github.com/pr0d1r2/pklith/issues/87)).
- Wave order by hand
  ([sherd#132](https://github.com/pr0d1r2/sherd/issues/132)).
- CI on macOS only, with no pinned Xcode yet
  ([actlith#18](https://github.com/pr0d1r2/actlith/issues/18)).
- No release tool.

## Anti-patterns

- Trusting `/usr/bin/git` or `/usr/bin/python3` on a Mac. They are Xcode
  shims, so an Xcode whose licence has not been accepted breaks tools
  that look unrelated. Put nix's git first on the dev shell's `PATH`.
- A non-Swift file in a target directory without an `exclude`: the
  warning it causes blocks `-warnings-as-errors`.
- Turning on warnings as errors before the tree is at 0 warnings.

This page grows as age-plugin-se ships; it stays a draft until then
(`paradigm/V6`). Where next: the other language pages,
[Ruby](RUBY.md) and [Shell](SHELL.md), and the stage pages from the
[landing page](../README.md).
