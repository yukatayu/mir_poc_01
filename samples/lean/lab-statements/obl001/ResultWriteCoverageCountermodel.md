# OBL-001 result/write coverage countermodel

This LAB-only Lean source imports the unchanged OBL-001 statement-shape draft.
It defines an experiment-local Result and write-membership relation. The
`untrackedCross` result contains the experiment-only write, but `GeneratedWrite`
is false for every result/write pair while `THM001StatementDraft` still holds.

The checked conclusion is limited to statement shape: the current LAB draft
does not itself connect every represented result write to `GeneratedWrite`.
It does not identify the experiment-local relation with Canon Core `c`, select
a Core representation or equality, falsify THM-001, or change any OBL status.

The governing research record and evidence plan are in
`mirrorea_canon/working/WRK-0007-obl001-result-write-coverage.md` and
`plan/wrk-0007-obl001-result-write-coverage.md`.
