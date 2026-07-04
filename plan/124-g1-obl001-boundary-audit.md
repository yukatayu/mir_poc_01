# plan/124 - G1 OBL-001 boundary audit

## Purpose

This file is LAB repository memory.

It closes the narrow boundary audit required by
`plan/123-g1-scn01-visibility-negative-actualization.md`: check whether the
current LAB OBL-001 statement-shape draft can carry `ELAB-11`, `ELAB-12`, and
`ELAB-17` without adding helper-JSON vocabulary, final diagnostic ABI fields,
or sample-specific names to the Lean statement.

This file does not edit canon, does not move G0/G1 gate status, does not
discharge OBL-001 / OBL-002 / OBL-020 / OBL-021, does not claim C-static
conformance, and does not change runtime, transport, diagnostic, repair, or
Core IR ABI status.

## Verdict

The current LAB OBL-001 boundary is sufficient for the three audited static
evidence rows. No Lean predicate refinement is needed for this checkpoint.

The reason is that
`samples/lean/lab-statements/obl001/THM001StatementDraft.lean` already keeps the
required proof-interface hooks abstract:

- owner-directed generated writes:
  `GeneratedWriteSound`, `RequestEvidenceSound`, `RequestForWrite`, and
  `OwnerDirectedRequest`;
- request-local authority / failure / dependency / span evidence:
  `RequestCarriesAuthorityObligations`,
  `RequestCarriesFailureContainment`,
  `RequestCarriesDependencyEvidence`, and `RequestCarriesSpanEvidence`;
- result-level RHS, failure, authority, span, visibility, and nested-locus
  postconditions:
  `AllRhsReadsRecorded`, `GeneratedFailuresContained`,
  `AuthorityObligationsRepresented`, `SourceSpansPreserved`,
  `VisibleWriteConsequencesExplicit`, and
  `NoAmbientAuthorityFromNestedLocus`.

These names are abstract proposition fields. They are intentionally not final
Surface/Core JSON fields, diagnostic ABI names, runtime dispatcher names, or
sample vocabulary.

## Evidence-to-predicate mapping

| Evidence row | Static pressure | OBL-001 hook that carries it | Boundary |
|---|---|---|---|
| `ELAB-11` | SCN-01 owner-directed write from `BrowserClient[self]` to `World` for `player[self].position` | `GeneratedWriteSound` through existential `RequestEvidenceSound`, including `RequestForWrite` and `OwnerDirectedRequest` | no request-serving runtime semantics |
| `ELAB-11` | same-field RHS read of `player[self].position` recorded as a dependency row | `AllRhsReadsRecorded` plus request-local `RequestCarriesDependencyEvidence` | no final OPEN-014 read materialization policy |
| `ELAB-11` | visible write generates publish / observe consequences | `VisibleWriteConsequencesExplicit` | no final telemetry, viewer, or publish/observe ABI |
| `ELAB-11` | positive generated failure-row containment | `GeneratedFailuresContained` plus request-local `RequestCarriesFailureContainment` | no diagnostic ABI freeze and no proof of general containment |
| `ELAB-12` | SCN-02 owner-directed write from `BrowserClient[self]` to `S` for `player[target].hp` | `GeneratedWriteSound` through existential `RequestEvidenceSound`, including `RequestForWrite` and `OwnerDirectedRequest` | no store mutation or runtime admission claim |
| `ELAB-12` | RHS dependencies for `player[target].hp` and `player[self].atk` | `AllRhsReadsRecorded` plus request-local `RequestCarriesDependencyEvidence` | no final cross-locus read transport claim |
| `ELAB-12` | nested `S { ... }` is not ambient authority | `NoAmbientAuthorityFromNestedLocus` and owner-directed request evidence | no G3 / THM-004 authority proof |
| `ELAB-17` | rejected SCN-01 visible-write row missing only `VisibilityDenied` | `GeneratedFailuresContained` / `RequestCarriesFailureContainment` mark the failed premise boundary that would be required for a successful result | no OBL-024/025 discharge and no final Diagnostic / repair ABI |
| `ELAB-17` | rejected row still projects request, RHS dependency, publish, and observe context | same hooks as `ELAB-11`, used as failure-context evidence, not as a successful THM-001 postcondition | no successful elaboration claim for the rejected row |

## Why no new Lean predicate is added

Adding predicates named after `remote_request_summaries`,
`dependency_summaries`, `auto_publish`, `auto_observe`,
`lab_diagnostic_details`, `E-ROW-002`, `VisibilityDenied`,
`suggested_repair`, or `diagnostic_soundness_projection` would import current
LAB carrier names into OBL-001 and blur the boundary between:

- THM-001 / OBL-001 assignment elaboration soundness;
- OBL-024 diagnostic soundness;
- OBL-025 repair completeness;
- helper-local JSON evidence;
- final Core IR / Diagnostic / repair ABI decisions.

The existing predicates are deliberately more abstract than the current helper
projection. That abstraction is the safe boundary for this checkpoint.

## Actual audit result

The audit therefore closes the immediate `plan/123` sequencing guard:

- `ELAB-11` and `ELAB-12` can be cited as exact current static pressure for the
  existing OBL-001 abstract hooks.
- `ELAB-17` can be cited as exact current static negative evidence for the
  failure-containment boundary, while remaining a rejected row and not a
  successful THM-001 instance.
- Diagnostic projection / repair payload details in `ELAB-17` remain OBL-024 /
  OBL-025 pressure evidence only. They are not OBL-001 content.
- The Lean statement draft should stay unchanged unless a later package finds a
  concrete missing abstraction that cannot be expressed through the existing
  predicate groups.
- Further SCN fixture expansion is now allowed only if it is separately justified
  by a concrete G1 bridge gap. The current narrow remaining candidate is still
  SCN-02 direct-local-write static guard, if it becomes a blocker.

## Reviewer finding

Read-only sidecar review agreed with the verdict: the existing abstract
OBL-001 predicates are sufficient with a scope caveat. The reviewer highlighted
the same hook mapping:

- remote writes through `GeneratedWriteSound`, `RequestEvidenceSound`,
  `RequestForWrite`, and `OwnerDirectedRequest`;
- RHS dependency through `RequestCarriesDependencyEvidence` and
  `AllRhsReadsRecorded`;
- visible consequences through `VisibleWriteConsequencesExplicit`;
- generated failure containment through `GeneratedFailuresContained` and
  `RequestCarriesFailureContainment`.

The reviewer also identified the main drift risk: keep `ELAB-17` explicitly
classified as failure-containment pressure for OBL-001, while diagnostic
projection and repair payload fields stay with OBL-024 / OBL-025 LAB evidence.

Oracle follow-up `follow-up-for-the-mirrorea-2` independently reached the same
decision: no concrete missing abstraction was found and no Lean change is the
smallest safe next action. Its useful extra warning was manifest staleness:
after `ELAB-17`, `plan/122` must not continue to read as if the exact SCN-01
`VisibilityDenied` negative were still a current gap. This package therefore
updates `plan/122` with a post-`plan/123` / `plan/124` addendum.

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No OBL-001 completion.
- No OBL-002 proof discharge.
- No OBL-020 / OBL-021 completion.
- No proof skeleton completion.
- No C-static / C-runtime / C-distributed conformance claim.
- No runtime request serving, store mutation, occurrence ordering, admission
  lifecycle, stale-membership runtime failure, or distributed transport claim.
- No final Diagnostic JSON, repair payload, association key, replay, Core IR,
  source map, runtime, transport, projection, or public API freeze.
- No promotion of helper/sample/report/Lean compile-check evidence to canon.

## Next allowed move

The next safe move is not OBL-001 predicate refinement by default.

Allowed narrow follow-ups:

1. add a docs-only / test-only guard only if an actual drift risk appears in the
   statement sync tests;
2. actualize the SCN-02 direct-local-write static guard only if the remaining
   direct-local-write rejection gap becomes a concrete blocker for the G1 bridge;
3. return to OBL-020 / OBL-021 reserve refinement only if review finds a
   statement-boundary overfit or missing abstraction there.

If none of those blockers appears, the project should keep the current
OBL-001 boundary stable and continue the G1 bridge without changing the Lean
predicate surface.
