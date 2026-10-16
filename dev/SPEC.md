# SPEC

## §G GOAL

this repo's own gate steps (D39): internal links, linked anchors, posts ledger, numbers, name guard. Rust crate, `publish = false`, wired into `hk.pkl` through pklith, one step per commit. ⊥ shipped to members.

## §N NAV

rel|path|lens
up|.|-
self|dev|this repo's own gate steps, `publish = false`: links, anchors, posts, numbers, name guard
sib|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
sib|registry|member list: repo, status, pin, public evidence; private-count snapshot
sib|sherdwright|stage checker binary: history, trailers, waivers, report

## §C CONSTRAINTS

- one crate, one binary `dev`, clap verbs. offline: ⊥ network in any step.
- step = rule + checker in one commit; commit body shows planted violation rejected (`.:V21`).
- exit 0 clean · 1 violation · 2 usage. step missing its input says so on stderr & exits 0 (degrade loud, ⊥ silent).
- denylist lives in `.private/`, ⊥ committed; its contents ⊥ printed in output, only the file:line hit.

## §I INTERFACES

- cmd: `dev links` — ∀ relative link in tracked `*.md` resolves to a tracked file & existing heading anchor.
- cmd: `dev anchors` — ∀ page & anchor listed in `docs/POSTS.md` still resolves, | resolves via stub (D11, D12).
- cmd: `dev posts` — ledger rows well-formed (date, URL, pages); ledger page ⊥ removed, only superseded.
- cmd: `dev numbers` — ∀ typed number in pages has ledger row w/ as-of date; generated numbers match source (D35, `.:V34`).
- cmd: `dev names` — denylist present → staged text has no denylisted name; always: ∀ GitHub repo named | linked ∈ `dev/public-repos.toml` (`.:V37`).
- file: `dev/public-repos.toml` — `[[repo]]` `name`, `public` (date of `gh repo view` PUBLIC). adding row = own reviewed commit.

## §V INVARIANTS

V1: ∀ step proven by planted violation in a test & in its commit body; companion test accepts every real shape in this repo.
V2: name guard output ⊥ echoes denylist entries ∴ CI logs ⊥ leak them.
V3: steps deterministic: same tree → same verdict.

## §T TASKS

id|status|task|cites
T1|.|crate scaffold, clap, `publish = false`, mirror tests|V3
T2|.|`dev names` + `public-repos.toml` seeded from repos named today|V1,V2
T3|.|`dev links`|V1
T4|.|`dev posts` + empty `docs/POSTS.md`|V1
T5|.|`dev anchors`|V1
T6|.|`dev numbers` + `docs/NUMBERS.md` ledger|V1
