# Third-party notices

paradigm is mostly prose. Its two crates, `dev` (this repository's own gate
steps) and `sherdwright` (the stage checker), are never published, and they
share one direct dependency. This file says what it is, why it is there, and
what licence the set carries.

## The one dependency

[`clap`](https://crates.io/crates/clap) parses arguments. It is not
hand-rolled because argument parsing via clap is a fleet rule (root `V29`):
hand-parsed arguments drift from their help text.

It has no network feature, and the build runs offline from the tracked
`Cargo.lock`. `cargo tree -e normal` lists every crate it pulls in.

## Licences

Every crate in that tree is available under MIT. `unicode-ident` adds the
Unicode-3.0 licence for the Unicode data it embeds, which permits use and
redistribution with its notice kept. The whole tree can be taken under MIT
alongside this project.

## Not dependencies: the tools in the gate

These appear in the dev shell and the gate, and are **not** linked into
either crate.

- [`hk`](https://github.com/jdx/hk): the gate runner.
- [`pklith`](https://github.com/pr0d1r2/pklith): writes the hk steps from
  `.pklith`.
- [`microlith`](https://github.com/pr0d1r2/microlith) (`mth`): owns the
  `SPEC.md` format and checks it.
- [`sherd`](https://github.com/pr0d1r2/sherd): checks the federated spec,
  its token budgets and the coverage floor.
- [`itok`](https://github.com/pr0d1r2/itok): token counting for the
  budgets.

## Trademarks

Nominative use only; no affiliation or endorsement is implied.

- **Rust** and **Cargo** are trademarks of the Rust Foundation.
- **NixOS** and **Nix** are trademarks of the NixOS Foundation.
- **GitHub** and **GitHub Actions** are trademarks of GitHub, Inc.
- **Claude** and **Anthropic** are trademarks of Anthropic PBC.

## paradigm itself

Everything else in this repository is licensed under the MIT License; see
[`LICENSE`](../LICENSE).
