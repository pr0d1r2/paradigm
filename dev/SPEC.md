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
sib|seed|per-language instruction blocks between markers; flake app `#seed`

## §C CONSTRAINTS

- one crate, one binary `dev`, clap verbs. offline: ⊥ network in any step.
- step = rule + checker in one commit; commit body shows planted violation rejected (`.:V21`).
- exit 0 clean · 1 violation · 2 usage. missing OPTIONAL input (`.private/denylist`) → says so on stderr & continues; missing REQUIRED file (`dev/public-repos.toml`) → exit 2.
- depends on `sherdwright` by path: the stage vocabulary has one home.
- denylist lives in `.private/`, ⊥ committed; its contents ⊥ printed in output, only the file:line hit.

## §I INTERFACES

- step: `links` — lychee `--offline --include-fragments` over tracked `*.md` (`.pklith`): ∀ relative link & anchor resolves. ⊥ a `dev` verb: tool exists ∴ ⊥ rewritten.
- cmd: `dev anchors` — ∀ page & anchor listed in `docs/POSTS.md` still resolves, | resolves via stub (D11, D12).
- cmd: `dev posts` — ledger rows well-formed (date, URL, pages); ledger page ⊥ removed, only superseded.
- cmd: `dev numbers` — ∀ typed number in pages has ledger row w/ as-of date; generated numbers match source (D35, `.:V34`).
- cmd: `dev names` — denylist present → staged text has no denylisted name; always: ∀ GitHub repo named | linked ∈ `dev/public-repos.toml` (`.:V37`).
- cmd: `dev commit-msg <file>` — message's final paragraph holds `Paradigm-Stage:` w/ stage ∈ root §I.trailer vocabulary (`.:V43`); subject | body cites a `T`/`V`/`B` id (`.:V27`).
- file: `dev/public-repos.toml` — `self` (this repo, `owner/name`, exempt until public) & `[[repo]]` rows: `name` (`owner/name`), `public` (date of `gh repo view` PUBLIC). adding row = own reviewed commit.

## §V INVARIANTS

V1: ∀ step proven by planted violation in a test & in its commit body; companion test accepts every real shape in this repo.
V2: name guard output ⊥ echoes denylist entries ∴ CI logs ⊥ leak them.
V3: steps deterministic: same tree → same verdict.
V4: test fixtures spell fake repo names via `concat!` ∴ `names` scans dev's own tests & finds no real reference.
V5: `names` skips a file that is gone | ⊥ UTF-8: staged deletions & binaries hold no names to leak.

## §T TASKS

id|status|task|cites
T1|x|crate scaffold, clap, `publish = false`, mirror tests|V3
T2|x|`dev names` + `public-repos.toml` seeded from repos named today|V1,V2
T3|x|relative links & anchors via lychee `links` step, ⊥ `dev` verb|V1
T4|.|`dev posts` + empty `docs/POSTS.md`|V1
T5|.|`dev anchors`|V1
T6|.|`dev numbers` + `docs/NUMBERS.md` ledger|V1
T7|x|`dev commit-msg` + commit-msg hook|V1
T8|.|`dev readme` & `dev notices`: generated README badge block (gate steps, coverage floor, invariants, bugs, nodes, deps) & notices crate table from `Cargo.lock`, drift step fails when stale, as fleet `-dev` crates do. until then README carries only count-free badges (`.:V34`)|V1

## §B BUGS

id|date|cause|fix
B1|2026-10-25|`GIT_*` drop loops (here & `sherdwright` history) covered only when a hook set `GIT_*` ∴ CI, w/o any, read 97.30% < floor 98.65%; local gate ⊥ saw it|`drop_git_vars` takes names, test passes them; CI (`.:V28`) runs w/o hook env
