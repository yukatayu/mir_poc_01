# plan/125 - G1 SCN-02 direct-local-write blocker review

## Purpose

This file is LAB repository memory.

It reviews the remaining `plan/122` SCN-02 negative (b) gap: whether the G1
bridge must now add a dedicated executable guard for an implementation that
treats nested `S { ... }` as a direct local write instead of an owner-directed
request.

This file does not edit canon, does not promote a new runnable sample row, does
not close G0 or G1, does not move proof-obligation status, does not claim
C-static conformance, and does not change runtime, transport, Core IR,
diagnostic, repair, or public API status.

## Verdict

No new executable guard is needed at this checkpoint.

The current G1 bridge can proceed with the existing evidence because the
required static shape is already covered by:

- exact positive SCN-02 owner-directed elaboration evidence in `ELAB-12`;
- structural cross-place write evidence in `ELAB-02`;
- structural indexed-state ambient-authority rejection evidence in `IDX-05`;
- the existing OBL-001 abstract hooks audited in `plan/124`.

The important scope rule is negative: do not claim that SCN-02 negative (b) is
already exact executable negative evidence. It remains structural support only
until a future package adds a dedicated negative/conformance row.

## Canon pressure

The canon pressure is real but does not force a new LAB row immediately.

`mirrorea_canon/theory/03-elaboration.md` states that every cross-locus
consequence must appear in generated edges, row containment must hold, and
`O { ... }` from `L != O` yields requests rather than ambient authority.

`mirrorea_canon/spec/03-static-semantics.md` states that cross-locus writes need
write capability or owner mediation and that nested `O { ... }` from `L != O` is
never ambient authority.

`mirrorea_canon/scenarios/SCN-02-attack.md` requires a request edge to `S`,
dependency rows for `player[target].hp` and `player[self].atk`, generated
failure containment, and non-ambient nested `S { ... }`. It also lists the bad
implementation pattern "claim the nested block as local write" as a C-static
failure.

Those canon statements define a later conformance obligation. They do not imply
that the current LAB bridge must represent the bad implementation as a runnable
sample before moving to the next G1 review step.

## Existing LAB evidence

`ELAB-12` is the exact current positive SCN-02 static evidence:

- source shape:
  `BrowserClient[self] { when attack(target) { S { player[target].hp =
  player[target].hp - player[self].atk } } }`;
- one write `remote_request_summaries` row;
- `requester_locus = role:BrowserClient`;
- `owner_locus = S`;
- `state_name = player`;
- `key_expr = target`;
- `generated_from = nested_place_block`;
- `failure_row_complete = true`;
- two `rhs_indexed_read` dependency rows for `target.hp` and `self.atk`;
- generated edge kinds include `message_envelope` and `remote_write_request`;
- no diagnostics.

`ELAB-02` is structural support for the same mechanism: a role-authored nested
`S { ... }` write does not switch ambient authority and elaborates to an
owner-directed remote write request.

`IDX-05` is structural negative support for the indexed-state checker: a nested
`S { ... }` block inside a role instance is rejected by the checker if it is
treated as a direct write to S-owned indexed state rather than being elaborated
through a generated request.

Together, these rows block the immediate drift that would make G1 cite an
ambient-authority reading as evidence. They still do not constitute a dedicated
SCN-02 direct-local-write conformance negative.

## Existing test guard reading

Rust elaboration coverage already checks the local mechanism:

- `elaborates_nested_place_write_into_owner_directed_remote_request` asserts a
  nested role-authored write creates exactly one write request from
  `role:BrowserClient` to `S` with `generated_from = nested_place_block` and a
  `generated_remote_write_request` transition.
- `records_assignment_rhs_reads_as_dependencies_without_remote_read_materialization`
  asserts the SCN-02-shaped assignment creates one write request, no observation
  rows, and two `rhs_indexed_read` dependency rows for `target.hp` and
  `self.atk`.

The Python helper tests also run `ELAB-12` against its expected JSON. The direct
Python assertion focuses on the two dependencies, but the helper comparison
still checks the expected `remote_request_summaries` fields.

This is enough for current drift detection. A new test-only guard would become
useful only if a future change weakens that helper comparison, introduces a
second elaborator, or starts claiming exact SCN-02 negative (b) evidence.

## Relation to OBL-001

`plan/124` already audited `ELAB-12` against OBL-001. The existing abstract
boundary can carry SCN-02 through:

- `GeneratedWriteSound`;
- `RequestEvidenceSound`;
- `RequestForWrite`;
- `OwnerDirectedRequest`;
- `RequestCarriesDependencyEvidence`;
- `AllRhsReadsRecorded`;
- `NoAmbientAuthorityFromNestedLocus`.

No Lean predicate refinement is needed for this review. Adding helper-specific
or bad-implementation-specific predicates would overfit OBL-001 to current LAB
fixtures and blur the boundary between abstract proof shape, helper JSON,
future conformance suites, and final Core IR.

## Future trigger for a dedicated guard

Add a narrow executable guard later if one of these becomes true:

- the project wants to cite SCN-02 negative (b) as exact executable negative
  evidence;
- a C-static conformance suite begins covering implementation-negative
  variants;
- a second frontend or elaborator can produce owner-local write artifacts;
- Core IR grows an owner-local write representation that could appear alongside
  or instead of the generated request;
- the claim changes from "the request edge exists" to "no direct local write
  artifact can also be emitted."

The narrowest future guard should stay LAB/test-only: use the SCN-02 source
shape and fail if nested `S { ... }` changes the requester locus to `S`, omits
the remote write request, or emits an owner-local write artifact for the same
assignment.

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No OBL-001 completion.
- No proof discharge.
- No proof skeleton completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No exact executable negative evidence claim for SCN-02 negative (b).
- No runtime request serving, store mutation, occurrence ordering, admission
  lifecycle, stale-membership runtime failure, or distributed transport claim.
- No final grammar/API/Core IR/diagnostic/repair/runtime/transport/projection
  ABI freeze.
- No sample status relabel.

## Next allowed move

The SCN-02 direct-local-write candidate should be closed as non-blocking for the
current G1 bridge.

The next safe package should either:

1. audit OBL-020 / OBL-021 statement boundaries if the G1 bridge needs another
   proof-boundary check; or
2. continue a narrow G1 ordinary-assignment support review without adding new
   executable rows unless a concrete blocker appears.
