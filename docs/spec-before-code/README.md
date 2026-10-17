# Spec before code

The first code is written after three things exist: a prompt that says what
the tool is for, gap rounds that closed its open questions, and a spec whose
format a tool has checked. Agents build against the spec, not against a
conversation.

## Why

An agent that starts from a chat fills every gap itself, and its guesses
land as behaviour nobody decided. A spec moves those decisions to before
the build, where a human can answer them once and every worker reads the
same answer. Numbered decisions and invariants also give commits something
to cite, so anyone can trace a line of code back to the reason it exists.

## Evidence

- This repository closed 53 decisions in 7 rounds before writing a line of
  code, and its spec run waited until no open question blocked a node.
- Decisions made late are visible in the private repositories: during the
  waves of one, a new catalogue node arrived and one spec rule was reverted
  in substance and then restored.
- Spec text is short on purpose. It uses a compressed "caveman" encoding
  that an agent reads cheaply; the pages a human reads, like this one, are
  plain prose.

## What it takes

- [Intent](../intent/README.md) in a prompt file.
- [Gap rounds](../gap-rounds/README.md) until the open questions are closed.
- A [spec run](../spec-run/README.md) that lands one section per commit.
- [Spec rounds](../spec-rounds/README.md) that compare spec and code.

## Checked by

`paradigm/V1`, `paradigm/V5`, `paradigm/V7` and `paradigm/V8`, in
[`SPEC.md`](../../SPEC.md).

## Where next

Where it starts: [intent](../intent/README.md).
