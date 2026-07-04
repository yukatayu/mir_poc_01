# plan/109 - G1 OBL-024 repo-local Lean statement draft

## Purpose

This file records a LAB-only Lean-checked statement-shape draft for OBL-024
explanation soundness.

The draft lives at
`samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`.
It checks that the OBL-024 relation can be expressed with abstract vocabulary
for emitted diagnostics, report-local association keys, future proof-level
diagnostic association, reported rule instances, failed premises, bindings,
report-local replay anchors, future proof-level replay relations, and
non-repair mixed diagnostic branch boundaries.

This package does not edit canon, does not move
`mirrorea_canon/theory/11-metatheory-ledger.md`, does not prove OBL-024, does
not claim OBL-024 completion, does not freeze Diagnostic ABI / JSON fields /
request IDs / branch IDs / association-key ABI / replay semantics /
diagnostic ordering / diagnostic equality, does not claim root-cause
uniqueness, does not alter OBL-025 repair completeness, and does not claim
conformance or G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB diagnostic carrier inventory:
  `plan/80-g1-diagnostic-carrier-inventory.md`
- LAB OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- LAB E-ROW carrier-only prototype:
  `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`
- LAB E-ROW precondition context:
  `plan/85-g1-erow-carrier-precondition-hardening.md`
- LAB OBL-025 contrast:
  `plan/87-g1-obl025-lean-statement-draft.md`
- LAB mixed-row branch contrast:
  `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- LAB OBL-025 branch-local non-coverage contrast:
  `plan/108-g1-obl025-branch-local-noncoverage-refinement.md`
- LAB statement artifact:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
- LAB executable projection carrier:
  `plan/110-g1-obl024-executable-projection-carrier.md`
- LAB replay vocabulary preflight:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md`
- LAB Lean replay vocabulary refinement:
  `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md`
- LAB Lean association vocabulary refinement:
  `plan/114-g1-obl024-lean-association-vocabulary-refinement.md`
- LAB association guard hardening:
  `plan/115-g1-obl024-association-guard-hardening.md`
- LAB explanation:
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md`
- LAB manifest:
  `samples/lean/manifest.json`

If this LAB statement conflicts with canon, canon wins.

## What was added

`samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
introduces a LAB-only namespace:

```text
MirCore.Lab.OBL024.StatementDraft
```

The file defines abstract carriers and predicates:

- `Vocab`: abstract types for environment, context, locus, judgment input,
  rejection, diagnostic, diagnostic id, rule instance, failed premise,
  bindings, diagnostic family, missing evidence kind, span, report-local
  association key, proof-level association witness, diagnostic branch,
  report-local replay anchor, and proof-level replay witness;
- `Pred`: abstract proposition fields for well-scoped inputs, current evidence
  boundary, covered diagnostic-soundness cases, rejecting judgments,
  diagnostic-to-rejection association, report-local association keys,
  proof-level association relation, diagnostic field projection, actual rule instances,
  premise membership, binding reconstruction, replay anchor compatibility,
  proof-level replay witness / relation, diagnostic-id / diagnostic-family
  compatibility, missing-evidence matching, span blame, and non-repair
  mixed-row diagnostic branch boundary predicates;
- `ReportLocalAssociationKeyCompatible`: helper relation requiring the
  diagnostic's non-final report-local association key to be compatible with
  the scoped rejected judgment and diagnostic;
- `DiagnosticAssociationCompatible`: helper relation requiring the scoped
  diagnostic-to-rejection association, report-local key compatibility, and a
  future proof-level association witness / relation;
- `ReportedDiagnosticShape`: helper relation requiring a diagnostic to report
  id, rule, premise, bindings, family, missing evidence, and primary span;
- `ReportLocalReplayAnchorCompatible`: helper relation requiring the
  report-local replay anchor to match the reported rule / premise / bindings
  while remaining non-final LAB evidence;
- `ReplaySoundAtReportedPremise`: helper relation requiring actual rule /
  premise / binding reconstruction plus a proof-level replay relation at the
  reported premise;
- `MixedDiagnosticBranchBoundary`: helper relation requiring every branch of a
  mixed-row diagnostic gap to classify some missing evidence, remain
  classification / partition evidence, and not become an independent failed
  premise;
- `DiagnosticSoundForRejection`: helper relation tying an associated diagnostic
  to its reported and replayed failed premise;
- `OBL024StatementDraft`: the compile-check-only `Prop` shape.

## Statement reading

For every well-scoped rejecting judgment input and associated diagnostic inside
the current LAB evidence boundary, if the diagnostic is in a covered
soundness case, then there must exist reported rule / premise / bindings /
family / missing evidence / span / association and replay witnesses such that:

- the diagnostic reports a non-final report-local association key compatible
  with the scoped rejected judgment and diagnostic;
- a future proof-level association witness / relation connects the
  diagnostic to the rejected judgment;
- the diagnostic reports the diagnostic id, rule instance, failed premise,
  bindings, diagnostic family, missing evidence, and primary span;
- the reported rule instance is actual for the same judgment input and
  rejection;
- the failed premise belongs to that rule instance under the reported
  bindings;
- the bindings reconstruct the local failed premise;
- the report-local replay anchor is compatible with the reported rule,
  premise, and bindings, while remaining non-final LAB evidence;
- a future proof-level replay witness / relation states trace-local failure at
  the reported premise;
- diagnostic id, diagnostic family, missing evidence, and blame span match
  that premise;
- every mixed-row diagnostic branch classifies some missing evidence while
  staying partition evidence, not an independent failed premise.

The statement is intentionally existential and abstract. It does not specify
final Diagnostic JSON fields, request IDs, branch IDs, association-key ABI,
replay implementation, diagnostic equality, diagnostic ordering, or a public
theorem namespace. `plan/113` refines the draft so the current
`trace_local_replay` evidence is represented by `ReportLocalReplayAnchor`,
while `ProofLevelReplayWitness` / `ProofLevelReplayRelation` remain future
proof-level vocabulary. `plan/114` refines the draft so the current
`lab_association_key` evidence is represented by `ReportLocalAssociationKey`,
while `ProofLevelAssociationWitness` / `ProofLevelAssociationRelation`
remain future proof-level vocabulary. `plan/115` guards that this report-local
key is not semantic association by key equality and not a branch-local
association key.

## Relation to current E-ROW evidence

Current executable LAB evidence remains E-ROW-shaped:

- `plan/84` adds non-final `lab_diagnostic_details` with `canon_id`,
  `rule_instance`, `failed_premise`, and `missing_evidence`.
- `plan/85` adds non-final `request_context` and `failure_row_context`,
  enough to name the row-containment precondition for current evidence.
- `plan/110` adds non-final executable `diagnostic_soundness_projection`
  evidence inside current `lab_diagnostic_details`, tying the diagnostic detail
  to helper-local diagnostic id, report-local association key, reported
  bindings, and report-local trace replay anchor.
- `plan/114` mirrors the association part in Lean as a report-local
  association key that can be compatible with future proof-level
  association vocabulary, without identifying the two roles.
- `plan/113` mirrors this in Lean as a report-local replay anchor that can be
  compatible with future proof-level replay vocabulary, without identifying the
  two roles.
- `ELAB-04`, `ELAB-07`, and `ELAB-10` remain the main current E-ROW carrier
  rows; later singleton and set-insertion packages add repair evidence for
  OBL-025, not OBL-024.

The Lean statement is not specialized to E-ROW as the whole theorem.
`CurrentEvidenceBoundary` and `CoveredDiagnosticSoundnessCase` keep the LAB
statement draft narrower than final OBL-024.

For mixed rows such as `ELAB-04`, the top-level diagnostic remains the owner of
the failed premise. Every `DiagnosticBranch` for that diagnostic gap can
classify base / visibility pressure only as partition evidence, without turning
branches into independent premises and without importing OBL-025 repair coverage
vocabulary.

## Relation to OBL-025

OBL-024 is explanation soundness. It asks whether an emitted diagnostic's
reported rule and failed premise are actual and replayable.

OBL-025 is explanation completeness. It asks whether a suitable diagnostic
with non-empty repair guidance is emitted when a single-edit repair exists.

This package adds no repair predicates, no `suggested_repair[]` shape, no
repair witness, no repair ranking, and no whole-gap repair coverage relation.

## Open questions

- Should the first future proof target quantify over every diagnostic family,
  or over an E-ROW fragment plus later generalization?
- What is the final carrier shape for diagnostic field projection?
- Should replay be whole-judgment replay, rule-local replay witness, or both?
- How should diagnostic equality / ordering interact with OBL-021
  elaboration-determinism vocabulary?
- What is the final multi-span declaration-site / use-site blame model?
- How should mixed-row associated diagnostics avoid double-counting one
  generated request?

## Suggested next packages

1. Keep OBL-024 as compile-check-only until association vocabulary, replay
   vocabulary, and diagnostic projection are stable enough to state a real
   theorem.
2. Treat `plan/110` executable projection evidence as helper-local LAB carrier
   hardening; do not promote its JSON names or report-local association key to final ABI.
3. Use `plan/112` and `plan/113` when separating current report-local replay
   anchors from a future proof-level replay relation in docs and Lean.
4. Use `plan/114` when separating current report-local association keys from
   future proof-level association relation in docs and Lean.
5. Revisit OBL-021 diagnostic equivalence only if diagnostic comparison blocks
   future OBL-024 proof shape.

## Non-claims

- No canon edit.
- No proof-status movement.
- No final Diagnostic ABI.
- No final JSON field names.
- No final request ID.
- No final branch ID.
- No final association-key ABI.
- No final replay engine.
- No final diagnostic equality.
- No final diagnostic ordering.
- No root-cause uniqueness.
- No diagnostic ranking.
- No mixed branch as independent premise claim.
- No OBL-024 proof.
- No OBL-024 completion.
- No OBL-025 proof.
- No OBL-025 completion.
- No repair output widening.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
