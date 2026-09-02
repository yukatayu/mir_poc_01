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
`CHANGELOG.md` update where required, and `INDEX.json` regeneration.

- ADR-0015 / PROPOSAL-018 and LAB Plan 247 record the closed Mir Theory v0 /
  I1+ M0--M10 program. Its accepted cuts are immutable history/regression
  baseline and grant no successor authority.
- ADR-0026 / PROPOSAL-029 and LAB Plan 249 record the closed Mirrorea I2
  Systems Foundation SYS-0--SYS-7 program. Its accepted cuts are immutable
  history/regression baseline and grant no successor authority.
- ADR-0033 / PROPOSAL-036 and canon plan/05 record the transport-neutral I3
  entry contract. PROPOSAL-037 / ADR-0034 consume it for the Mirrorea I3
  Distributed Foundation bounded program. Plan 250 is the sole current
  roadmap. PROPOSAL-038 / ADR-0035 and PROPOSAL-039 / ADR-0036 close ALIGN-1/2;
  PROPOSAL-040 / ADR-0037 close I3-0 and select QUIC reliable stream as the
  private provisional adapter. I3-1 is closed by ADR-0038 and the bounded
  two-process runtime is closed by ADR-0039. Program execution is owner-paused,
  no semantic milestone is active, and I3-3 remains inactive until explicit
  resume. TLS-over-TCP framed reliable stream remains the deferred comparison
  baseline, QUIC datagrams are excluded, and OPEN-032 is resolved only for this
  bounded program. Official I3 lifecycle entry is not implied.
- Outside ADR-0034's fixed scope, ADR-0014 permits only reversible L3
  pre-registration in `mirrorea_canon/working/`; L2 remains fail-closed pending
  an owner-authenticated trust anchor.
