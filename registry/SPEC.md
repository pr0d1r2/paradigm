# SPEC

## §G GOAL

one file lists every member: repo, status, paradigm pin, public evidence. changed only by reviewed PR (D33). paradigm itself first row (D1).

## §N NAV

rel|path|lens
up|.|-
self|registry|member list: repo, status, pin, public evidence; private-count snapshot
sib|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
sib|dev|this repo's own gate steps, `publish = false`: links, anchors, posts, numbers, name guard
sib|sherdwright|stage checker binary: history, trailers, waivers, report
sib|seed|per-language instruction blocks between markers; flake app `#seed`

## §C CONSTRAINTS

- public members by name; private members only as dated anonymized count snapshot (D8, D34).
- member needs GitHub remote (D45). self-hosted remote w/o pipeline stays out; learnings stay in language pages.
- member numbers (steps, invariants, bugs, nodes, coverage floor) pulled by generator, ⊥ typed here (D6).

## §I INTERFACES

- file: `registry/members.toml` — `[[member]]` tables: `repo` (`owner/name`), `status` ∈ `member` `legacy`, `pin` (paradigm tag | `-` before first tag), `public` (date `gh repo view` showed PUBLIC).
- file: `registry/private-counts.toml` — dated snapshot rows, counts only, written by opt-in verb (`sherdwright` node), committed via reviewed PR (D34).

## §V INVARIANTS

V1: ∀ `repo` ∈ public allowlist (`.:V37`). private repo ⊥ appears by name.
V2: `status` ∈ `member` `legacy`. lefthook-gated repo = `legacy`, checked only for migration invariants (D46).
V3: ⊥ typed member numbers. a number shown beside a member comes from generator | numbers ledger w/ as-of date (D35).
V4: CI checks drift for public members only; snapshot rows carry their date & ⊥ fail CI for being old (D34).

## §T TASKS

id|status|task|cites
T1|.|write `members.toml`: paradigm first, then the 11 public members of `.:R3`; `nixpkgs-lock` status `legacy` (D46)|V1,V2
T2|.|schema check step in `dev` (status set, date shape, allowlist)|V1,V2
T3|.|first private-count snapshot via opt-in verb, reviewed PR|V4
