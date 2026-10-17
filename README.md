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
