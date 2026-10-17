# Agents write the code; guardrails decide what lands

This is a way of building software with coding agents when nobody watches
them work. It has been used across a fleet of repositories, and this page
says why it is shaped the way it is. The rules themselves live in
[`SPEC.md`](../SPEC.md), each with a reason and the tool that checks it.

## The bet

Watching an agent is the obvious way to keep it honest, and it does not
work for long. A person reading output catches what they happen to notice,
on the days they are paying attention, and none of it carries over to the
next session. So the bet here is the opposite: put the human's attention
where no tool can go, and let tools refuse everything else.

The human owns four things. They write the intent, answer the questions the
agents raise, review the diff, and approve anything that leaves the
repository. Everything between those points is decided by a gate: a set of
checks that run on every commit, on every push and in CI, the same way each
time, and that say exactly what they refused and why.

## The order matters

The work goes in a fixed order, and most of the method is in that order.
Intent is written down before anything else, in a prompt file that can be
reviewed. Gap rounds turn every open question into a numbered decision
before the build, so agents do not answer them silently while building.
The spec comes next, in a format a tool checks. Then the pinned tools, then
the gate, one proven check at a time. Only then does product code start,
split by directory so that several agents can build at once without
touching each other. Each wave of that work ends by bringing the spec back
in line with what was built.

## What is checked, and what is not

A rule that nothing runs is a wish. So every rule names its checker: a gate
step, a file-level check, or a history checker that reads the stage of
each commit from its trailers. That history checker is still being built:
today it only reads the trailers, so every rule it will own is checked by
review for now. Where no checker exists yet, the rule is marked as a review
item and counted, and the count is expected to fall.

The record is kept in public, failures included. Every bug found after the
gate was green is listed with the rule added to stop it recurring, and every
number in these pages carries the date it was true and a way to count it
again. This repository's own first day with a spec produced three such bugs,
and they are listed in its [defect record](LLM-DISCLAIMER.md).

## Where to start

- The stages, from [intent](intent/README.md) to
  [reconciliation](reconciliation/README.md).
- The arguments: [guardrails over watching](guardrails-over-watching/README.md),
  [spec before code](spec-before-code/README.md),
  [parallel agents](parallel-agents/README.md) and
  [the unflattering record](unflattering-record/README.md).
