# Guardrails first, one check per commit

Before any product code, the gate goes in, one check per commit. Each commit
that adds a check says in its body how the check was shown to reject a
planted violation. The hooks are live from the commit that brings hk, so
every later commit, including the ones adding checks, passes through them.

## Why

A check nobody has seen fail is indistinguishable from a check that cannot
fail. Planting a violation and watching it get refused is the only proof a
guard works, and writing that proof into the commit makes it reviewable.
Laying the gate first means the product code is never written ungated, and
one check per commit means a broken check can be found and reverted alone.

## Evidence

- This repository's guardrails stage took 45 commits on 2026-10-17, and
  its gate now has 36 steps. Each check commit names its proof: pklith's
  catalog fixture for a built-in check, or a violation planted here for a
  custom one. The name guard, for example, was fed a page holding a
  private name and refused it by entry number, without printing the name.
- The gate checks itself: a step fails when `hk.pkl` evaluates to no steps.
  In a private repository a generator once produced an empty pre-commit
  that passed every other check, and workers noticed only from "no steps
  to run".
- One private repository bundled its guardrails into 6 commits; the next
  laid about 70, one each, before its first product test.
- [pr0d1r2/age-plugin-se PR #5](https://github.com/pr0d1r2/age-plugin-se/pull/5),
  still open at head `8b4151a` on 2026-10-18, is a public run of this
  stage on a Swift fork of an upstream age plugin. Its 47 commits all
  carry `Paradigm-Stage: guardrails`: the gate is seeded with no steps
  (`5fffb16`), then each check lands in its own commit, in pklith's lay
  order (hygiene, secrets, format, lint, spec, tests and coverage on
  push, then the gate's self-checks), and each body shows the planted
  violation being refused. The hooks are live from the seed commit, and
  nothing after it used `--no-verify`. Recount the commits with
  `git log --format='%(trailers:key=Paradigm-Stage,valueonly)' 152a9cf..8b4151a | grep -c '^guardrails$'`
  in that repository.
- Two proofs could not run the usual way. `no-core-worktree` was planted
  in a scratch clone, so the config every worktree shares was never
  touched (`bec1b2d`). For `confirm`, an `hk.pkl` cut to its `amends`
  line leaves hk no hook to run a step from, so `pklith confirm` ran
  directly and refused it with "does not evaluate to steps" (`b63408f`,
  `paradigm/V22`).
- A fork's own findings go in a baseline, not a reformat. Upstream code is
  left as it is, and its findings sit in exempt rows in `.pklith` and in
  `.typos.toml`, `.secretsignore` and `.markdownlintignore`, which may
  only shrink. That is how a fork gets the whole gate on day one.
- What the stage could not finish became spec rows rather than silence:
  a swiftlint baseline (120 upstream findings), the allowlist half of the
  name guard, coverage exclusions, and itok, which reads `.context-limits`
  with a different meaning than sherd does (`8b4151a`).

## How to do it

1. Seed the gate with `pklith seed --init` and read the `.pklith` it writes.
2. Lay checks with `pklith lay`, one commit each, through the hooks.
3. For each custom check, plant a violation, run the step, and put the
   refusal in the commit body.
4. Add the self-checks: generated steps match `.pklith`, every file type
   has a row, and the gate is not empty.
5. Ask the human to reload the shell once hooks are live.

## Checked by

`paradigm/V21` (one check per commit, planted proof) and `paradigm/V22` (a
gate with zero steps fails), in [`SPEC.md`](../../SPEC.md).

## Where next

The checks become one [gate](../gate/README.md) that runs the same way
everywhere.
