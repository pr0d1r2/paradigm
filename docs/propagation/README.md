# Fleet rules start local and are promoted

A decision made in one repository is local until it has shipped there. When
it turns out to matter fleet-wide, it is filed as an issue in each affected
repository and promoted into the paradigm, where it gets an id, a reason
and a checker. Members then cite it by id instead of copying it.

## Why

Copies drift. Two of the fleet's spec prompts already repeat one guardrail
roster nearly word for word, and nothing says which differences were meant.
A rule with one home and an id can be cited, waived with a reason, and
checked; a copied paragraph can only go stale. Waiting until the member
ships keeps unproven guesses out of the fleet rules.

## Evidence

- Several fleet rules here began as one repository's decision: clap for
  argument parsing came from three private repositories while every public
  member still parsed by hand, and tracking issues were filed in each.
- Worker isolation, worktree cleanup and the wave-end ritual were each
  learned from one repository's incident and are now fleet invariants
  (`paradigm/V15`, `V16`, `V18`).
- This repository's own first wave produced one candidate,
  `paradigm/V44`: a commit that is red on purpose must still compile,
  because pre-commit lints every target.

## How to do it

1. Mark each decision local or a candidate when you make it.
2. After the member ships, file an issue in each repository the rule
   affects, with the human's approval.
3. Promote the rule here with its id, its reason, its stage and its checker.
4. In the members, replace the copied text with a citation by id.

## Checked by

`paradigm/V35` (file and promote) and `paradigm/V42` (convention values are
read from members, not typed here), in [`SPEC.md`](../../SPEC.md).

## Where next

The full list of fleet rules is in [`SPEC.md`](../../SPEC.md).
