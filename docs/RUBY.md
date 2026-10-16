# The paradigm in Ruby

Status: draft, written from one private Rails application built with the
paradigm (about 1,400 commits in 10 days, about 1,110 Ruby files and 1,040
spec files). It is not named here. Numbers are from 2026-10-16.

The lifecycle is the same as in Rust: spec prompt, gap rounds, a federated
`SPEC.md`, guardrails first, parallel waves, TDD, a gate that refuses. What
changes is the tooling each stage runs on, and two stages that need more
care in Ruby: what a node is, and how parallel workers share a database.

## Stack

Ruby and Rails from the nix dev shell (YJIT on in production), no global
gems and no Homebrew. Gems for the gate come through `gemset.nix`, and a
gate step keeps it in step with `Gemfile.lock`. RSpec, FactoryBot,
Capybara, shoulda-matchers, VCR with WebMock, parallel_tests.

## Federation: packs are nodes

Each leaf node is a packwerk package: `package.yml`, `SPEC.md`, its own
`app/` or `lib/`, and its own `spec/` with factories. The nearest pack is
the nearest sherd node. Hubs (`guardrails/`, `product/`, `potential/` for
parked ideas) hold law, not code.

- The wave order comes from `package.yml` dependencies, which
  `packwerk check` enforces, so the build DAG is checked code, not prose.
- Packwerk privacy is the seam between nodes.
- The application shell names no pack; each pack wires itself in.
- sherd's code verbs read Rust only, so splits and waves were done by hand
  and the gap was filed upstream.

## Gate map

Every check is a `.pklith` row rendered into hk steps.

- Style and design: rubocop with every cop on, plugins, ERB, and custom
  cops; Sandi Metz limits as cops; reek, flog and flay with their own
  thresholds.
- Security and supply: brakeman, bundler-audit, a license allow-list.
- Views: an ERB linter.
- Database: migrating from empty must reproduce `db/schema.rb`;
  database-consistency; production eager-loads without dev gems.
- Structure: packwerk validate and check; one Ruby and one Rails version
  everywhere; logic in shell is capped and moves to Ruby `lib/`.
- Tests and coverage, below.

pklith's built-in catalog had only rubocop, so every other Ruby row started
local and was filed upstream as a catalog request.

## Mirror tests

One spec file per source file, checked in both directions, for `app/`,
`lib/`, ERB views, custom cops and JavaScript. In `.pklith` these are
`exists` and `orphan` rule pairs per pack. An orphan spec is as much a
failure as a missing one: it passes forever and tests nothing.

A further guard checks that every example actually reaches its body.

## Coverage

SimpleCov measures lines, branches, methods and ERB views. The floor is
100% on all three and only ratchets up. Mutation testing has a recorded
floor and runs weekly, one job per worker, never in the gate and only in a
throwaway worktree.

## Performance fuses

- A frozen query budget per example, and N+1 detection with strict
  loading on by default.
- CPU pace budgets per example, per controller action and per job. Under
  parallel waves these flake unless the budget accounts for machine load.

## Recorded externals

Every external response comes from a recorded cassette, never from a
hand-written stub. Each background job has its own queue, enforced by a
custom cop.

## Custom cops as law

Fix one case by hand, then write a cop whose autocorrect fixes the rest.
The rule then holds for code not yet written.

## Commit versus push

On commit, rubocop, specs and coverage run only on what the change reaches
(guard-style selection); coverage from a partial run goes to its own
directory. On push, the whole tree.

## Waves

Each wave worker gets a git worktree, one core, its own test database and
`HK_JOBS=1`. hk's stash must be `patch-file`: the git stash is shared
across worktrees, and two workers committing at once popped each other's
edits.

## Release

Release is a deploy: staging, then production, with post-deploy checks.
There is no package and no semver check.

## Gaps against the Rust flow

- No CI: the pre-push gate is the only gate before code lands.
- No PR-only main.
- sherd code verbs do not read Ruby yet.
- No factory-per-model check: factories live per pack, unchecked.

## Anti-patterns

- Wave worktrees left behind after merge (here: 10, up to 662 commits
  stale).
- Mutation runs on every core of every worker (load 170 on 10 cores).
- `.rubocop_todo.yml`, and inline disables without a reason.
- Hand-written HTTP stubs.
- Logic in shell or in Pkl strings.
