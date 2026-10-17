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
