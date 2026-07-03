# plan/105 - G1 ELAB-07 set-insertion exact-locus guard hardening

## Purpose

This file records the LAB-only follow-up after
`plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`.

`plan/102` intentionally described the executable `set_insertion` payload as
the exact current `ELAB-07` fact pattern. After `plan/104`, the internal row
association was narrow enough to distinguish two same-event `when` rows, but
the emission predicate still accepted nearby retargeted shapes when the
failure set arithmetic matched:

- same role, different event name;
- different role, same event name;
- same role/event, different owner locus;
- same role/event, different target state name;
- same role/event/state, different target field.

This package narrows the LAB set path to the current `ELAB-07` source locus:

```text
target_ref = when_fails_row|locus=role:BrowserClient|event=attack
owner_locus = S
state_name = player
field_name = hp
```

The guard is deliberately local to the non-final `ELAB-07` prototype. It does
not define final row identity, final source-locus identity, or general
set-insertion policy.

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
- LAB design gate:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- LAB executable prototype:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- LAB negative guards:
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- LAB row association guard:
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- LAB implementation:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- LAB tests:
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- Parser source for current representability limits:
  `crates/mir-ast/src/surface_alpha.rs`

If this LAB hardening conflicts with canon, canon wins.

## Decision level

This is `L3` LAB implementation evidence.

It is an executable guard around one prototype, not a canon source-locus or row
identity decision.

## TDD evidence

RED phase:

- Rust target:
  `cargo test -p mir-semantics --test surface_to_core_elaboration elab07_set_insertion_is_not_emitted -- --nocapture`
  failed for event, role, and state-field retargeting proxies because each
  still received a `set_insertion` repair. Owner-locus and state-name
  retargeting fixtures were added after reviewer feedback to cover every
  exact-locus predicate dimension.

GREEN phase:

- The same focused target passed after adding the exact LAB locus guard.
- The exact `ELAB-07` positive path still passed.
- The distinct same-event row association positive path from `plan/104` still
  passed.
- The full `surface_to_core_elaboration` integration test file passed.

## Implemented boundary

The set path now requires all existing `plan/102..104` guards plus the current
`ELAB-07` locus:

| Field | Required LAB value |
|---|---|
| `target_ref` | `when_fails_row|locus=role:BrowserClient|event=attack` |
| `owner_locus` | `S` |
| `state_name` | `player` |
| `field_name` | `hp` |

The guard is implemented as internal constants in
`crates/mir-semantics/src/surface_to_core_elaboration.rs`. These constants are
not exported and do not create a public ABI.

## Negative fixtures

New Rust-only fixtures:

| Fixture | Shape | Expected output |
|---|---|---|
| omitted failure row / row-creation proxy | `when attack(...) { ... }` with no `fails` clause | reject with LAB detail; no repair |
| event retargeting proxy | `when heal(...) fails MissingCapability` | reject with LAB detail; no `set_insertion` repair |
| role retargeting proxy | `AdminClient[self] { when attack(...) fails MissingCapability { ... } }` | reject with LAB detail; no `set_insertion` repair |
| owner-locus retargeting proxy | `T { player[target].hp = 1 }` under the same role/event | reject with LAB detail; no `set_insertion` repair |
| state-name retargeting proxy | `S { enemy[target].hp = 1 }` under the same role/event | reject with LAB detail; no `set_insertion` repair |
| state-field retargeting proxy | `player[target].score = 1` under the same role/event | reject with LAB detail; no `set_insertion` repair |

Existing coverage retained:

- `plan/103` already covers row-splitting pressure as one `when` row producing
  multiple generated requests.
- `plan/104` already covers two distinct same-event rows, one request each.

## Representability limits

The current `SurfaceWhenBlock` has one `failure_row: Vec<String>` and no
first-class repair target row object. Therefore:

- "missing target row" and "row creation" collapse to omitted `fails` in
  current Surface syntax;
- true row splitting is only approximated by multiple generated requests in
  one `when` row;
- true row movement or cross-row retargeting has no first-class current
  diagnostic carrier;
- event / role / owner / state / field retargeting tests are current
  sample-identity proxies, not final row movement semantics.

Those limits are part of the evidence, not hidden implementation detail.

## Current executable status

| Shape | Current result |
|---|---|
| exact single `ELAB-07` sample row | one non-final `set_insertion` item |
| omitted `fails` / row creation proxy | no repair |
| event retargeting proxy | no `set_insertion` repair |
| role retargeting proxy | no `set_insertion` repair |
| owner-locus retargeting proxy | no `set_insertion` repair |
| state-name retargeting proxy | no `set_insertion` repair |
| state-field retargeting proxy | no `set_insertion` repair |
| two requests in one `when` row | no `set_insertion` repair |
| two distinct same-event `when` rows, one request each | each row may receive its own exact `set_insertion` repair |
| proper subset / padded / duplicate variants from `plan/103` | no `set_insertion` repair |
| `ELAB-04` | no `suggested_repair` |
| `ELAB-10` | one singleton `E-ROW-002` item |
| `ELAB-13..16` | singleton `E-ROW-001` items |

Sample row count is unchanged.

## Relation to OBL-025

This package narrows one LAB executable repair candidate. It does not prove
OBL-025 and does not move canon proof status.

The exact-locus guard may make the candidate local witness less misleading,
but OBL-025 should remain abstract until row identity, whole-gap coverage,
multi-request / multi-target policy, and repair vocabulary stability are
strong enough to mention in proof-facing terms.

## Non-claims

- No canon edit.
- No final source-locus identity model.
- No final row identity model.
- No final Diagnostic ABI.
- No final repair payload ABI.
- No general set-insertion support.
- No bundle semantics support.
- No partial-guidance output or coverage support.
- No repair ranking.
- No visibility ranking.
- No multi-edit support.
- No true row creation support.
- No true row splitting support.
- No true row movement support.
- No true row retargeting support.
- No cross-row retargeting policy.
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
