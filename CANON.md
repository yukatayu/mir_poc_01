# Canon Entry Point

`mirrorea_canon/` is the normative canon for the project's big-picture
direction, theory, ADRs, conformance expectations, and operating process.

Everything outside `mirrorea_canon/` in this repository is LAB: implementation
evidence, historical record, reports, samples, helper code, and operational
notes. Legacy `specs/` and `plan/` files remain useful LAB evidence, but they
are not authoritative when they conflict with canon.

Read canon first:

1. `mirrorea_canon/README.md`
2. `mirrorea_canon/MAP.md`
3. The task-specific canon files listed from there

Use LAB files to preserve and verify evidence. Promote or revise canon only by
the canon process: proposal, owner decision, ADR or file update where required,
`CHANGELOG.md` update where required, and `INDEX.json` regeneration. ADR-0015
records the owner-approved bounded route for Mir Theory v0 / I1+ Milestones
0--10, including evidence-gated Canon, proof-ledger, implementation, test, and
status updates. Outside that program, ADR-0014 permits only reversible L3
pre-registration in `mirrorea_canon/working/`; L2 remains fail-closed pending
an owner-authenticated trust anchor.
