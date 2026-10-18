# SPEC

## §G GOAL

instruction seed per language: gives a repo the paradigm's words, between markers, pinned to a tag (D28–D31, D36). ⊥ code, ⊥ config, ⊥ gate steps (D30).

## §N NAV

rel|path|lens
up|.|-
self|seed|per-language instruction blocks between markers; flake app `#seed`
sib|docs|public pages: essay, prompt format, members page, language pages, ledgers; stage & topic pages in children
sib|registry|member list: repo, status, pin, public evidence; private-count snapshot
sib|dev|this repo's own gate steps, `publish = false`: links, anchors, posts, numbers, name guard
sib|sherdwright|stage checker binary: history, trailers, waivers, report

## §C CONSTRAINTS

- runs from this repo's flake: `nix run github:pr0d1r2/paradigm/<tag>#seed`. flake input revision = pin (D4, D36).
- skeleton seeder & pklith call it, ⊥ copy it (D36). public pages describe skeleton seeder only generically while private.
- writes only between `paradigm:begin` / `paradigm:end` markers (root §I.marker); text outside markers untouched.
- per language: block includes that language page's rules (D31). `rust` default.

## §I INTERFACES

- cmd: `nix run github:pr0d1r2/paradigm/<tag>#seed -- <lang> [<dir>]` — writes | refreshes seeded blocks in `<dir>` (default `.`).
- template: `seed/<lang>/AGENTS.md` — paradigm block: lifecycle, gate rules (⊥ `--no-verify`, refuse when hk missing), commit format, worker isolation, worktree cleanup, wave-end ritual, reload checkpoints, links into hub.
- template: `seed/<lang>/SPEC-PROMPT.md` — prompt template citing paradigm tag; human fills only intent, naming, survey, domain decisions, non-goals, open questions.
- template: `seed/LLM-DISCLAIMER.md` skeleton, `seed/gitignore` lines (`.claude/`), essay pointer paragraph w/ member's own numbers (D37).

## §V INVARIANTS

V1: seed idempotent: second run on seeded tree → ⊥ diff.
V2: ⊥ byte outside markers changes; tree w/o markers gets blocks appended, ⊥ rewritten.
V3: block header names tag ∴ drift check (`.:V39`) compares against pinned tag.

## §T TASKS

id|status|task|cites
T1|.|`rust` templates from root §V & `docs/SPEC-PROMPT.md`|V3
T2|.|marker writer + flake app `#seed`|V1,V2
T3|.|`ruby` `shell` `nix` templates from language pages|V3
T4|.|`swift` templates from `docs/SWIFT.md`, after T1 & T2; page still draft ∴ block says so (paradigm#1)|V3
