# Guardrails, not watching

Nobody watches an agent work here. A human writes the intent, answers the
open questions, reviews the diff and approves anything that leaves the
repository. Everything else is decided by a gate that refuses. The claim
is that this is safer than watching, not just cheaper.

## Why

Watching does not scale and does not repeat. A person reading an agent's
output catches what they happen to notice, on the day they are paying
attention, and nothing of it carries over to the next session. A gate
catches the same thing every time, for every agent, on every commit, and
it says exactly what it refused and why. The human's attention moves to the
places a gate cannot reach: what to build, which open question to answer
how, and whether a change should exist at all.

## Evidence

- In sherd, a rule kept only in prose was broken about 20 times in one
  session; the lesson its bug log records is that runners beat prose
  (sherd's B12).
- This repository's gate refused one of its own commits while it was being
  built, for a malformed table, and its checks flagged a test over the
  length limit and a file type with no checks. Each refusal named the fix,
  and nobody was watching.
- The gate guards itself too: a step fails if the gate evaluates to no
  steps, if a generated step is stale, or if a file type has no checks.

## What it takes

- A [dev shell](../dev-shell/README.md) that pins every tool.
- [Guardrails first](../guardrails/README.md), each proven against a
  planted violation.
- One [gate](../gate/README.md) on commit, push and CI, with no
  `--no-verify`.
- A human who owns intent and approval, and nothing in between.

## Checked by

`paradigm/V21`, `paradigm/V22`, `paradigm/V23` and the name guard
`paradigm/V37`, in [`SPEC.md`](../../SPEC.md).

## Where next

How the gate goes in, check by check: [guardrails](../guardrails/README.md).
