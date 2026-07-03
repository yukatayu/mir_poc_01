# plan/104 - G1 ELAB-07 set-insertion row-identity guard hardening

## Purpose

This file records the LAB-only row-identity hardening after
`plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`.

`plan/103` deliberately used a conservative request count to avoid
over-emitting `set_insertion` repairs when one `when` failure row generated
multiple remote requests. That count originally keyed by the public LAB
`target_ref`:

```text
when_fails_row|locus=...|event=...
```

This package keeps that public `target_ref` stable, but makes the internal
association key narrower by including the existing `SurfaceWhenBlock` source
span. The goal is to avoid suppressing the exact `ELAB-07` set repair across
two distinct same-event `when` rows under one role locus.

This package does not implement general set-insertion support. It does not
implement a final row identity model, bundle semantics, partial guidance,
repair ranking, visibility ranking, multi-edit repair, final Diagnostic /
repair ABI, OBL-024/025 proof, conformance, canon movement, or G1 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon entry points:
  `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon static semantics:
  `mirrorea_canon/spec/03-static-semantics.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB negative guard hardening:
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- LAB positive prototype:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- LAB payload model:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- LAB assumption gate:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- LAB executable preflight:
  `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- LAB implementation:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- LAB tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- Parser source for the existing span:
  `crates/mir-ast/src/surface_alpha.rs`

If this LAB hardening conflicts with canon, canon wins.

## Decision level

This is `L3` LAB implementation evidence.

The package is an internal association-key correction. It is not a canon row
identity decision.

## TDD evidence

RED phase:

- Rust target:
  `cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_suppressed_across_distinct_same_event_rows -- --nocapture`
  failed because two distinct `when attack(...) fails MissingCapability` rows
  under the same role locus collapsed into one public target reference. The
  first set repair was retrospectively suppressed and the second detail did not
  receive one.

GREEN phase:

- The same Rust target passed after the internal association key was changed to
  include `when.span.start..when.span.end`.
- The previous `plan/103` multi-request suppression test still passed.
- The full `surface_to_core_elaboration` integration test file passed.

## Implemented boundary

Public LAB diagnostic output remains stable:

```text
failure_row_context.target_ref =
  when_fails_row|locus=role:BrowserClient|event=attack
```

Internal association now uses:

```text
association_key =
  target_ref + "|span=" + when.span.start + ".." + when.span.end
```

`association_key` is stored only in the serialization-skipped LAB
`SurfaceLabDiagnosticFailureRowContext` field. It is used for:

- `failure_row_request_counts`;
- retrospective suppression of already-emitted `set_insertion` repairs.

It is not emitted in sample JSON and does not change expected JSON files.

## Current executable status

| Shape | Current result |
|---|---|
| exact single `ELAB-07` sample row | one non-final `set_insertion` item |
| two requests in one `when` row | no `set_insertion` repair |
| two distinct same-event `when` rows, one request each | each row may receive its own exact `set_insertion` repair |
| proper subset / padded / duplicate variants from `plan/103` | no `set_insertion` repair |
| `ELAB-04` | no `suggested_repair` |
| `ELAB-10` | one singleton `E-ROW-002` item |
| `ELAB-13..16` | singleton `E-ROW-001` items |

Sample row count is unchanged.

## Remaining limitations

This is still not a final row identity model:

- source spans are adequate for current parser-generated LAB evidence, but not
  a final ABI identity;
- source-preserving edits can move spans, so span identity is not a durable
  cross-version row identity;
- true multi-target-row policy remains unresolved;
- row creation / splitting / movement / retargeting policy remains unresolved;
- broader set-insertion support remains out of scope.

The current package only fixes the immediate same-event under-suggestion caused
by counting with public `target_ref`.

## Relation to OBL-025

This package removes one local under-suggestion artifact in LAB repair evidence.
It still does not prove OBL-025 and does not move canon proof status.

OBL-025 should remain abstract until row identity, whole-gap coverage,
multi-request / multi-target policy, and repair vocabulary stability are strong
enough to mention in proof-facing terms.

## Non-claims

- No canon edit.
- No final row identity model.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No general set-insertion support.
- No bundle semantics support.
- No partial-guidance output or coverage support.
- No repair ranking.
- No visibility ranking.
- No multi-edit support.
- No row creation, row splitting, row movement, or row retargeting support.
- No `VisibilityDenied` / `E-ROW-002` widening.
- No `ELAB-04` reclassification.
- No `ELAB-10` change.
- No `ELAB-13..16` change.
- No OBL-024 proof.
- No OBL-025 proof.
- No OBL-025 completion.
- No explanation soundness claim.
- No explanation completeness claim.
- No conformance claim.
- No G0 exit.
- No G1 exit.
