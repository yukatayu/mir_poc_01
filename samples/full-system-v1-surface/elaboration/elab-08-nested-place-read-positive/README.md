# ELAB-08 Nested Place Read Positive

Surface source authority: `main/src/nested-place-read-positive.mir`.

Expected evidence: a foreign nested `S { ... }` read generates an owner-directed read request with source spans rather than a direct ambient-authority read.
