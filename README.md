# paradigm

How a fleet of repositories is built by coding agents without anyone
watching them: agents write the code, guardrails decide what lands.

Status: draft, not yet public. The rules are specified in
[`SPEC.md`](SPEC.md) and the node specs it federates;
[`SPEC-PROMPT.md`](SPEC-PROMPT.md) is the prompt they grew from. Read the
[LLM disclaimer](docs/LLM-DISCLAIMER.md) before trusting anything here.

Start with the essay: [Agents write the code; guardrails decide what
lands](docs/PARADIGM.md).

## The stages

From the first file to the last commit of a wave. The first six are the
inception: how a repository starts.

1. [Intent](docs/intent/README.md): a prompt file before any code.
2. [Gap rounds](docs/gap-rounds/README.md): open questions become numbered
   decisions.
3. [Spec run](docs/spec-run/README.md): the spec lands one section per
   commit.
4. [Federation](docs/federation/README.md): one spec per directory.
5. [Waves](docs/waves/README.md): isolated workers, one node each.
6. [Reconciliation](docs/reconciliation/README.md): every wave ends by
   bringing the spec back in line.

Then the rest of the lifecycle:
[spec rounds](docs/spec-rounds/README.md),
[dev shell](docs/dev-shell/README.md),
[guardrails](docs/guardrails/README.md),
[gate](docs/gate/README.md),
[release](docs/release/README.md),
[backprop](docs/backprop/README.md) and
[propagation](docs/propagation/README.md).

## Trying it

There is no starter template yet; the [seed](seed/SPEC.md) that will
write one is still a spec. Until it lands, a new repository starts by
hand, one stage at a time:

1. Write `SPEC-PROMPT.md` as the [intent](docs/intent/README.md) page
   describes, and answer its open questions in
   [gap rounds](docs/gap-rounds/README.md).
2. Get cavekit through nix-cavekit (below) and let `ck:spec` turn the
   prompt into `SPEC.md`, one section per commit
   ([spec run](docs/spec-run/README.md)).
3. Split the spec with sherd, one `SPEC.md` per directory
   ([federation](docs/federation/README.md)).
4. Pin the tools in a nix [dev shell](docs/dev-shell/README.md), then add
   the [guardrails](docs/guardrails/README.md) through pklith, one proven
   check per commit.
5. Build in [waves](docs/waves/README.md) and end each one with a
   [reconciliation](docs/reconciliation/README.md).

This repository went through the same steps, and its history shows each
of them: every commit names its stage in a `Paradigm-Stage:` trailer.

## Tools

The pages name these tools without stopping to introduce them.

- [cavekit](https://github.com/JuliusBrussee/cavekit): the `ck:spec`,
  `ck:build`, `ck:check` and `ck:backprop` skills for Claude Code, over one
  `SPEC.md`. Install it from
  [nix-cavekit](https://github.com/pr0d1r2/nix-cavekit), not the plugin
  marketplace: the marketplace copy is out of date.
- [sherd](https://github.com/pr0d1r2/sherd): federates `SPEC.md` over a
  directory tree, with token budgets, navigation and parallel waves.
- [microlith](https://github.com/pr0d1r2/microlith): the spec format
  checker, `mth fmt` and `mth check`.
- [hk](https://github.com/jdx/hk): the git hook runner every gate runs on;
  [nix-hk](https://github.com/pr0d1r2/nix-hk) puts it in the flake.
- [pklith](https://github.com/pr0d1r2/pklith): writes the `hk` steps from
  what the repository's files need, and refuses a file type no step checks.
- [nixpkgs-lock](https://github.com/pr0d1r2/nixpkgs-lock): one nixpkgs
  revision for every repository.
- [itok](https://github.com/pr0d1r2/itok): token ceilings per spec node
  (`.context-limits`).
- [characterminator](https://github.com/pr0d1r2/characterminator): allowed
  characters per file type (`.ctrm`).
- [rekall](https://github.com/pr0d1r2/rekall): rules and skills from agent
  memory, with an honesty gate.
- [sherdwright](sherdwright/SPEC.md): will check a repository's history
  against the stages. It lives here and is mostly a spec: only the part
  that reads each commit's `Paradigm-Stage:` trailer is built, and the
  commit-msg hook uses it to refuse an unknown stage. No stage rule is
  checked yet.

## The arguments

- [Guardrails, not watching](docs/guardrails-over-watching/README.md)
- [Spec before code](docs/spec-before-code/README.md)
- [Parallel agents need walls, not supervision](docs/parallel-agents/README.md)
- [The unflattering record](docs/unflattering-record/README.md)

## Language pages

The main pages use Rust as the worked example. Other languages:

- [Ruby](docs/RUBY.md)
- [Shell](docs/SHELL.md)
- [Nix](docs/NIX.md)

## Ledgers

- [Defect record](docs/LLM-DISCLAIMER.md)
- [Numbers ledger](docs/NUMBERS.md): every typed number, dated
- [Posts ledger](docs/POSTS.md): pages that posts link to
