# Green means working

A green gate here is meant to say the code works, not that the tests
ran. An agent under pressure to go green has easy ways to get there
without making anything work: delete the failing test, weaken its
assertion, write the test after the code so it never had to fail, or
loosen the gate itself. Each of those is a way green lies, and each one
needs a check that refuses it. Where no check exists yet, this page says
so.

## Why

A test suite only proves something if it would fail on wrong code. Line
coverage does not show that: a file can be fully run by a test that
asserts nothing. A test written after the code may never have been seen
failing, so nobody knows whether it can. And a gate that the change under
review can edit judges itself. Three checks together close most of this:
a mirror test for every source file a rule selects (exceptions named
below), the test changing before or with its source, and mutation testing that changes the code and expects the tests
to fail.

## Ways green lies, and what refuses each

- **A source file with no test, hidden inside a coverage percentage.**
  Every source file a `.pklith` `exists` row selects has exactly one
  mirror test at the path the row names, and a file without one fails by
  name (`paradigm/V45`). A file the row leaves out is listed in its
  `except` cell with a reason. In this repository the rows select the
  Rust sources of `dev` and `sherdwright`; each crate's `lib.rs` is
  excepted, since it only declares the modules its siblings' tests
  cover. Nix files are outside the rule: flake checks stand in for them.
- **A test deleted, or left behind when its source went.** The same
  mapping read the other way, an `orphan` row, fails a test whose source
  is gone, so tests and sources stay paired (`paradigm/V45`).
- **A test written after the code.** A `tdd-order` row fails a commit
  that changes a source file before any commit changed its test
  (`paradigm/V46`). Pre-push judges only the unpushed commits, as an
  early warning: a source-only commit after an already pushed test
  commit fails there, though CI, judging the whole pull request, accepts
  it. A test and its source changed in the same commit pass, so the
  check does not prove the test was seen failing. It selects the same
  files as the `exists` row, so an excepted `lib.rs` can change without
  its own test changing first. A RED commit, with `RED` in its subject,
  may be committed failing, and tests run on push, not on commit
  (`paradigm/V24`).
- **A coverage floor lowered to let a change through.** `.coverage` is a
  ratchet: `sherd coverage --record` only raises the floor
  (`paradigm/V25`). A hand edit can still lower it, and the gate then
  measures against the edited floor, so a lower floor needs a reviewed
  commit that says why.
- **A gate skipped or emptied.** There is one `hk.pkl` on commit, push
  and CI, `--no-verify` is not used, and `main` takes pull requests only,
  never squashed, so the separate test and source commits the order
  check judged stay on `main` (`paradigm/V23`). A gate that evaluates to no steps is a failure
  (`paradigm/V22`).
- **An assertion weakened until it passes anything.** Mutation testing
  changes the code and expects a test to fail; a change no test notices
  is a gap. Today that runs weekly for Ruby only, outside the hooks
  (`paradigm/V31`). It changes the code, not the tests, so when another
  test kills the same mutants, a weakened assertion leaves the score
  unchanged.

## What green does not prove yet

- Mutation testing beyond Ruby is a candidate, not a rule. The proposal
  is dual band: a code band that changes the implementation and expects
  the tests to fail, and a test band that changes the assertions and
  expects them to fail against the real implementation, each with its
  own ratcheted floor.
- The strict order, a RED commit with tests only, then GREEN with source
  only, then REFACTOR with no assertion change, is a candidate too.
- Two gaps in the order check are open upstream: merge commits are not
  judged ([pklith#137](https://github.com/pr0d1r2/pklith/issues/137)),
  so a conflict resolution that edits a source file is rebased, not
  merged; and rows that disagree with each other are not refused
  ([pklith#138](https://github.com/pr0d1r2/pklith/issues/138)).
  `paradigm/V45` and `paradigm/V46` are counted as checked by review
  until this repository's pinned gate carries both fixes and
  `docs/waves:T2` is done.
- A pull request can still edit the gate files that judge it, a
  lowered `.coverage` floor among them. Keeping
  gate changes in their own commits, judged by the base branch's gate, is
  a candidate.
- Green never proves the spec asked for the right thing. That stays with
  the human who wrote the intent.

## Evidence

- In age-plugin-se,
  [PR #16](https://github.com/pr0d1r2/age-plugin-se/pull/16) mirrors
  each source file in its own test, checked by an `exists` and an
  `orphan` row, after coverage numbers alone had not named the untested
  file. [PR #18](https://github.com/pr0d1r2/age-plugin-se/pull/18) adds
  the `tdd-order` row from
  [pklith#98](https://github.com/pr0d1r2/pklith/pull/98).
- This repository's own gate took the same rows in
  [PR #46](https://github.com/pr0d1r2/paradigm/pull/46), one check per
  commit, each proven against a planted violation. Laying them found a
  mistake in this repository's own rule: an `orphan` row copied from its
  `exists` row checks the sources again and never fails, so the rule now
  says the `orphan` row is the inverse.

## What it takes

- [Guardrails first](../guardrails/README.md), each proven against a
  planted violation.
- One [gate](../gate/README.md) on commit, push and CI.
- [Waves](../waves/README.md) built test first, one node per worker.

## Checked by

`paradigm/V45`, `paradigm/V46`, `paradigm/V24`, `paradigm/V25`,
`paradigm/V22`, `paradigm/V23` and `paradigm/V31`, in
[`SPEC.md`](../../SPEC.md).

## Where next

How the mirror and order rules are written: [waves](../waves/README.md).
