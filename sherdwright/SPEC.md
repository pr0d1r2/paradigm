# SPEC

## §G GOAL

`sherdwright` — checker for lifecycle invariants pklith cannot see: prompt, gap rounds, spec runs, waves, reconciliation, releases (D32). reads member's git history, `Paradigm-Stage:` trailers & `.pklith` `exempt` rows; reports ∀ root invariant hold | waived | fail | review.

## §N NAV

rel|path|lens
up|.|-
self|sherdwright|stage checker binary: history, trailers, waivers, report
sib|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
sib|registry|member list: repo, status, pin, public evidence; private-count snapshot
sib|dev|this repo's own gate steps, `publish = false`: links, anchors, posts, numbers, name guard
sib|seed|per-language instruction blocks between markers; flake app `#seed`

## §C CONSTRAINTS

- Rust, clap, own crate in workspace. name evidence: `.:R17`.
- deterministic & offline (D7): ⊥ LLM, ⊥ network, except one opt-in remote verb.
- read-only on members: ⊥ writes into a member tree.
- child git drops `GIT_*` & runs against explicit repo path (`.:V15`).
- covers only invariants tagged `sherdwright` in root §V; file-level ones belong to pklith profile.

## §I INTERFACES

- cmd: `sherdwright check [<dir>] [--format human|json]` — report for member at `<dir>` (default `.`) against its pinned tag. exit 0 no fail · 1 fail · 2 usage. ? exact flags pending CLI sketch (T1).
- cmd: `sherdwright counts` ? — opt-in: dated anonymized private-member count snapshot → `registry/private-counts.toml` (D34).
- cmd: `sherdwright remote <owner/repo>` ? — opt-in: same report from remote repo; only verb using network (D7).
- lib: `stage::Stage` — root §I.trailer vocabulary, `FromStr` & `Display` round-trip. `history::trailers(repo)` → ∀ commit oldest first w/ the `Paradigm-Stage` values git parses.
- json: per invariant `id` (`paradigm/V<n>`), `stage`, `verdict` ∈ `hold` `waived` `fail` `review` `newer`, `reason`, `evidence` (commit | file:line).

## §V INVARIANTS

V1: same member tree + same paradigm tag → byte-identical report.
V2: waiver w/o reason = `fail` (`.:V38`).
V3: invariant newer than member's pin → verdict `newer`, ⊥ `fail`; report shows gap to latest (`.:V40`).
V4: `review` invariants listed, & their count printed in every report (`.:V41`).
V5: ∀ check proven by planted violation fixture & accepted by a real conforming history fixture.

## §T TASKS

id|status|task|cites
T1|.|gap round: CLI sketch for `sherdwright` added to prompt-derived §I, human answers|-
T2|.|crate scaffold, clap, report types, json|V1
T3|x|trailer reader: `Paradigm-Stage:` vocabulary from root §I.trailer|V1
T4|.|inception checks: `.:V1`, `.:V2`, `.:V3`, `.:V5`, `.:V7`, `.:V8`, `.:V10`|V5
T5|.|wave checks: `.:V13`, `.:V14`, `.:V18`, `.:V21`|V5
T6|.|waiver & pin logic: `.:V38`, `.:V40`, `.:V41`|V2,V3,V4
T7|.|remaining `sherdwright` invariants: `.:V27`, `.:V33`, `.:V36`, `.:V39`|V5
T8|.|opt-in `counts` & `remote` verbs|V1
T9|.|candidate (`.:V6`): `digest <a> <b>`: §V §I §F delta, ∀ `SPEC.md` at last `reconcile-<a>` commit vs last `reconcile-<b>`, `mth fmt` form|V1
T10|.|candidate (`.:V6`): scope: worker merge touches only own node dir, else fail & new §T row|V5
T11|.|candidate (`.:V6`): `remote`: ∀ PR landed on `main` (rebase \| merge commit) after tag `signoff-v1`; unsigned expected 0. signoff = head approval on T9 digest & 0-`fail` report, ⊥ code read|V4
