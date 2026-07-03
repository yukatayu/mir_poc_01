# plan/85 - G1 E-ROW carrier precondition hardening

## Purpose

This file records the LAB-only hardening of the E-ROW diagnostic detail carrier
before any repair-bearing prototype emits `suggested_repair[]`.

This is LAB repository memory. It does not edit canon, does not freeze a
Diagnostic or repair ABI, does not emit repair rows, does not state or prove
OBL-024/025, does not claim explanation soundness or completeness, does not
claim conformance, and does not claim G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- LAB E-ROW alignment:
  `plan/79-g1-erow-diagnostic-alignment.md`
- LAB diagnostic carrier inventory:
  `plan/80-g1-diagnostic-carrier-inventory.md`
- LAB OBL-024 relation inventory:
  `plan/81-g1-obl024-statement-shape-inventory.md`
- LAB OBL-025 relation inventory:
  `plan/82-g1-obl025-statement-shape-inventory.md`
- LAB repair payload inventory:
  `plan/83-g1-erow-repair-payload-inventory.md`
- LAB carrier-only prototype:
  `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md`

If this LAB note conflicts with canon, canon wins.

## Implemented hardening

`SurfaceLabDiagnosticDetail` now includes two non-final context subobjects:

| Field | Current meaning |
|---|---|
| `request_context` | local generated-request identity and target facts needed to replay the row-containment premise |
| `failure_row_context` | local `when ... fails` row facts and the concrete generated/declared/missing failure sets |

These fields are carrier context only. They are not repair payloads and are not
public ABI.

## `request_context`

Current fields:

| Field | Source |
|---|---|
| `request_id` | existing generated remote request id, e.g. `req-0001`; local elaboration sequence only |
| `request_kind` | existing generated remote request kind, currently `read` or `write` |
| `generated_from` | existing remote request provenance such as `cross_locus_read_expression` or `nested_place_block` |
| `requester_locus` | current actor / role locus that generated the request |
| `owner_locus` | owner place of the indexed state |
| `state_name` | indexed state name |
| `key_expr` | indexed access key expression |
| `field_name` | accessed field when available |

## `failure_row_context`

Current fields:

| Field | Source |
|---|---|
| `target_kind` | fixed LAB value `when_fails_row` |
| `target_locus` | same locus as the generated request authorizing context |
| `event_name` | surrounding `when` event name |
| `required_failures` | generated failure set before containment check |
| `declared_failures` | surrounding `when ... fails` row |
| `missing_failures` | `required_failures - declared_failures` |
| `local_premise` | fixed LAB value `generated_failures_subset_declared_fails` |

`missing_evidence` remains equal to `failure_row_context.missing_failures` for
this prototype, preserving the earlier carrier while making the containment
precondition explicit.

## E-ROW split

The previous split remains unchanged:

| Case | Current carrier output |
|---|---|
| Only missing generated failure is `VisibilityDenied` | `E-ROW-002` |
| Any other missing set, including mixed visibility/non-visibility omissions | `E-ROW-001` |

`ELAB-04` remains mixed E-ROW-shaped evidence, `ELAB-07` remains clean
non-visibility E-ROW-001 evidence, and `ELAB-10` remains clean E-ROW-002
evidence.

## Deferred until repair-bearing prototype

The following are intentionally deferred:

- `suggested_repair[]`
- `repair_family`
- `target_span`
- `use_span`
- declaration-site / use-site multi-span policy
- `single_edit_assumption`
- `local_effect`
- repair ranking
- repair application semantics

These belong to a later repair-bearing prototype or OBL-025 statement draft
only after the vocabulary is stable enough to reject placeholder repair rows.

## Code and sample surfaces

- Rust report carrier:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- Rust tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- Python helper tests:
  `scripts/tests/test_surface_mir_samples.py`
- Expected JSON evidence:
  `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
  `samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json`

`scripts/surface_mir_samples.py` did not need logic changes because it already
passes through non-empty `lab_diagnostic_details`.

## What this changes

- E-ROW carrier evidence can now identify the generated request and the local
  failure-row containment precondition.
- Expected JSON for `ELAB-04`, `ELAB-07`, and `ELAB-10` now records request and
  failure-row context.
- Tests can distinguish real precondition context from placeholder repair
  payloads.

## What this does not change

- No final Diagnostic ABI.
- No repair payload ABI.
- No `suggested_repair[]`.
- No OBL-024 statement or proof.
- No OBL-025 statement or proof.
- No explanation soundness or completeness claim.
- No C-static conformance claim.
- No G0 exit.
- No G1 exit.
- No T1/T2 transition.
- No runtime MessageEnvelope dispatch claim.
- No final Surface runtime / transport / viewer claim.

## Follow-up

The next repair-bearing package should remain deferred until tests can reject
placeholder `suggested_repair[]` items and every repair item can identify the
target failure row, missing failure, local premise, and single-edit assumption
without making `request_id` or current JSON key names public ABI.
