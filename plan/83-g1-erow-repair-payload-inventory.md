# plan/83 - G1 E-ROW repair payload inventory

## Purpose

This file inventories a non-final repair payload shape for future E-ROW
diagnostics, before any code prototype emits `suggested_repair[]`.

This is LAB repository memory. It does not implement repairs, does not freeze a
Diagnostic or repair ABI, does not state or prove OBL-024/025, does not claim
explanation soundness or completeness, does not claim conformance, and does not
edit canon.

## Source hierarchy

- Normative source: `mirrorea_canon/`
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

If this LAB inventory conflicts with canon, canon wins.

## Payload inventory, not ABI

`plan/82` says OBL-025 needs a suggested repair item that realizes an actual
single-edit repair witness. This file lists the minimum information a later
E-ROW prototype would need to expose if it includes `suggested_repair[]`.

The names below are candidate payload roles, not final JSON keys or public API.

## Minimal repair payload roles

| Role | Candidate meaning | Current status |
|---|---|---|
| `repair_family` | repair taxonomy family, initially `add-to-fails-row` | canon taxonomy exists; payload name not final |
| `diagnostic_family` | E-ROW family being repaired, e.g. E-ROW-001 or E-ROW-002 | helper still emits `generated_failure_not_declared` |
| `applies_to` | legacy code, candidate canon ID, and request id that the repair is attached to | candidate only |
| `target_kind` | declaration surface being edited, initially `when_fails_row` | candidate only |
| `target_context` | locus and event name for the nearest relevant failure row | candidate only |
| `target_span` | span for the nearest relevant `when ... fails` row when available | current LAB has source spans but no final multi-span policy |
| `use_span` | span for the generated request / use site that triggered the missing failure | current LAB has request/source span evidence |
| `missing_failure` | concrete missing failure family, e.g. `VisibilityDenied` | inferable from generated/declared failure sets, not emitted |
| `required_failures` | generated failure family set for replay / explanation context | carried on remote request evidence |
| `declared_failures` | declared `fails` row set | carried on remote request evidence |
| `local_effect` | declared failure row after adding missing failure(s) | candidate only |
| `single_edit_assumption` | why this payload is treated as one edit | OPEN atomicity policy |
| `local_premise` | row-containment premise this repair targets | relation inventory only |
| `non_goal` | explicitly not a guarantee of whole-program success | required guard |

## Candidate non-final shape

A later prototype could expose a repair item with this conceptual shape:

```text
repair = {
  repair_family: add-to-fails-row,
  applies_to: {
    legacy_code: generated_failure_not_declared,
    canon_id: E-ROW-001 | E-ROW-002,
    request_id: <request id>
  },
  target_kind: when_fails_row,
  target_context: { locus: <locus>, event_name: <event> },
  target_span: <declaration span when known>,
  use_span: <generated request / source span>,
  missing_failure: <one failure family>,
  required_failures: <generated failure set>,
  declared_failures: <declared fails row>,
  local_effect: declared_failures_after = declared_failures union missing_failure,
  local_premise: generated_failures_subset_declared_fails,
  non_goal: does_not_claim_runtime_success
}
```

This shape is intentionally not final JSON. It is a vocabulary check for a
future implementation package.

## E-ROW case split

| Case | Candidate repair | Boundary |
|---|---|---|
| E-ROW-001 generated failure not declared | add one missing generated failure family to nearest relevant `when ... fails` row | clean when the missing set has one family, or later atomicity says set insertion is one edit |
| E-ROW-002 undeclared `VisibilityDenied` | add `VisibilityDenied` to the relevant `fails` row | alternative visibility / observe-authority repairs remain OPEN |
| mixed E-ROW omission | inventory only | do not treat as clean single-edit unless atomicity policy is explicit |

## Current LAB evidence

Current Surface elaboration records useful inputs on generated remote requests:

- `required_failures`
- `declared_failures`
- `failure_row_complete`
- `generated_from`
- `source_span`

Current helper projections and expected JSON expose only `diagnostic_codes`,
remote request summaries, source-span entity kinds, obligation codes, and
`final_public_api_frozen: false`. They do not expose repair payloads.

## Implementation guidance for later package

If the next package implements code, the safest path is additive and
carrier-only first:

1. preserve legacy `diagnostic_codes`;
2. add non-final LAB-only diagnostic detail alongside the legacy projection;
3. include `canon_id`, `severity`, `rule_instance`, `failed_premise`,
   `missing_evidence`, and refs before repair rows;
4. defer `suggested_repair[]` until every emitted item can identify the
   missing failure, target row, and local premise;
5. mark every new detail as LAB/non-final in docs and expected JSON;
6. keep OBL-024/025 proof status unchanged.

A carrier-only prototype without repair rows is now the recommended first
implementation step. It does not advance OBL-025, but it reduces ABI and proof
overclaim risk.

`plan/93-g1-erow001-singleton-repair-assumption.md` later turns the
no-placeholder requirement into a LAB-only gate for a possible
non-visibility singleton `E-ROW-001` widening. It still does not widen repair
output.

## What remains OPEN

- Final Diagnostic ABI and repair payload JSON field names.
- Whether helper output exposes both `legacy_code` and `canon_id`.
- Exact rule-instance and premise identifiers.
- Declaration-site / use-site multi-span policy.
- Whether adding a set of missing failures is one edit or multiple edits.
- E-ROW-002 alternatives beyond add `VisibilityDenied` to `fails`.
- Repair ranking, multi-edit repairs, and localization.
- Repair application semantics.
- Whether the first implementation should include repair rows or stay
  carrier-only.

## Overclaim guards

- Do not treat this as final JSON or public ABI.
- Do not claim current LAB already emits repair payloads.
- Do not claim non-empty repair rows prove OBL-025.
- Do not claim add-to-fails-row makes runtime execution safe or successful.
- Do not claim all single-edit repairs are covered.
- Do not use mixed E-ROW evidence as a clean single-edit witness.
- Do not edit canon or move proof status from this LAB inventory.

## Suggested next packages

1. Carrier-only E-ROW diagnostic detail prototype without repair rows, adding
   non-final canon ID / severity / rule / premise / missing-evidence detail
   while preserving legacy output.
2. Repair-bearing E-ROW follow-up only after reading `plan/93`, preserving its
   no-placeholder `suggested_repair[]` guard, and intentionally changing the
   affected fixture expectations.
3. OBL-024/025 Lean statement drafts only after replay, repair, and payload
   vocabularies stabilize enough in LAB.

## Non-claims

- No canon edit.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No repair generation implementation.
- No OBL-024 statement.
- No OBL-024 proof.
- No OBL-025 statement.
- No OBL-025 proof.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
- No T1 transition.
- No T2 transition.
- No runtime behavior claim.
