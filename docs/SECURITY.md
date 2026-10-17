# Security policy

## Reporting a vulnerability

Please report vulnerabilities privately, through GitHub's
[private vulnerability reporting](https://github.com/pr0d1r2/paradigm/security/advisories/new),
not in a public issue. Include what you ran, what happened, and what you
expected.

You will get an acknowledgement within a week. A fix, or a reason there
will not be one, follows as soon as the issue is understood.

## Supported versions

paradigm has no tag yet. Fixes land on `main`; once tags exist, the latest
one is supported.

## What paradigm trusts

Most of this repository is prose. The parts that run are its gate and two
small crates, so treat these files as code, the same as a Makefile:

- `hk.pkl` and `hk.pklith.pkl`: hk runs their steps on every commit and
  push.
- `.pklith`: a `## checks` row's `check` column becomes an hk step command.
- `.githooks/`: the dev shell points git at these hooks.
- `dev/` and `sherdwright/`: Rust crates the gate builds and runs. Neither
  reaches the network.

A report that one of these runs something it was not told to, or that the
name guard can be made to let a private name through or to print one, is
in scope.
