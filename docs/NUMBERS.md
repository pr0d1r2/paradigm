# Numbers ledger

Every number typed into a page by hand, with the date it was true and how
to count it again. A number pulled from a member repository by a generator
leaves this list; the count of rows here is expected to fall (decision 35
of the [spec prompt](../SPEC-PROMPT.md)).

| page | number | as of | recount |
| --- | --- | --- | --- |
| docs/reconciliation | 6 violations, 2 drifts, 3 bugs in drift round 1 | 2026-10-17 | `Paradigm-Stage: ck-check-round-1` commits and §B in `SPEC.md` |
| docs/waves | 432 hk processes; load 170 on 10 cores; 19 worktrees, 281 MB, 20 branches | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/federation | 21, 22 and 46 nodes in pklith, sherd and xenolith | 2026-10-16 | `find <member> -name SPEC.md -not -path '*/.claude/*'` |
| docs/federation | 23 nodes; root chain 4,207 tokens, deepest 5,062 | 2026-10-17 | `sherd budget` |
| docs/federation | 12 and 7 ceiling changes; 1,700 to 2,100; 12 to 20% slack | 2026-10-16 | private survey, anonymized; not recountable here |
| docs/spec-run | 32 spec-run commits: 7 root sections, 22 nodes | 2026-10-17 | `git log --format=%B \| grep -c '^Paradigm-Stage: spec-run'` |
