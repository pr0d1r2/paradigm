# LLM disclaimer

Coding agents write the code and most of the text in this repository. A
human owns the intent, answers the open questions, reviews the diff and
approves anything outward-facing, such as a push, a publish or a post.
Everything else is a gate that refuses: the steps in [`hk.pkl`](../hk.pkl),
generated from [`.pklith`](../.pklith).

Before trusting anything here:

- Read the spec ([`SPEC.md`](../SPEC.md)) as the law. Pages explain it and
  can lag behind it.
- Treat a number as true on the date the [numbers ledger](NUMBERS.md) gives
  for it, and no later.
- Look at the defect record below. Every defect found after the gate was
  green is listed with the invariant added so it cannot recur.

## Defect record

Each row is a bug the gate did not catch at the time, from the bug log
(§B) of [`SPEC.md`](../SPEC.md).

| id | found | what went wrong | rule added |
| --- | --- | --- | --- |
| B1 | 2026-10-17 | The `Paradigm-Stage:` line sat in its own paragraph above `Co-Authored-By:`, so git parsed none of the 38 stage trailers written that way. | V43, and the commit-msg step that asks git itself |
| B2 | 2026-10-17 | This repository's own spec prompt froze without a config sketch or a CLI sketch, and with its process section after its open questions. | V3 stays; T13 waives it for this repository with B2 as the reason |
| B3 | 2026-10-17 | The commits of the spec run cite no spec ids, because nothing checked V27 yet. | V27 applies from the commit that brings hk; the commit-msg step checks it |
