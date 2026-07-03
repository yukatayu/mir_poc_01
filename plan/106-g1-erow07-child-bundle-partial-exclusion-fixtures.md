# plan/106 - G1 ELAB-07 child / bundle / partial exclusion fixtures

## Purpose

This file records the LAB-only test-only follow-up after
`plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`.

`plan/101` left three repair-shape regression risks around the exact
`ELAB-07` set item:

- three serialized singleton child alternatives;
- one or two partial child alternatives counted as complete coverage;
- textual-only or partial guidance presented as an executable repair witness.

The current Surface syntax does not have child-repair, bundle, partial
guidance, or textual-guidance syntax. These shapes can therefore only appear
through Rust-side repair emission or future payload-field widening. This
package fixes the current executable boundary with Rust-only JSON-shape
assertions on the exact positive `ELAB-07` payload.

The package does not add production repair logic and does not add sample rows.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- Canon entry points:
  `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`
- Canon elaboration boundary:
  `mirrorea_canon/theory/03-elaboration.md`
- Canon diagnostic theory:
  `mirrorea_canon/theory/10-diagnostics.md`
- Canon diagnostic format:
  `mirrorea_canon/spec/07-diagnostics-format.md`
- Canon proof-status ledger:
  `mirrorea_canon/theory/11-metatheory-ledger.md`
- LAB set / bundle vocabulary:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- LAB payload model:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- LAB executable prototype:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- LAB negative guards:
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- LAB row association guard:
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- LAB exact-locus guard:
  `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`
- LAB implementation and tests:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`,
  `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- LAB expected JSON:
  `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- Mapping review:
  sub-agent mapping findings from 2026-07-04 child / bundle / partial
  exclusion package.

If this LAB fixture conflicts with canon, canon wins.

## Decision level

This is `L3` LAB test evidence.

It is a characterization guard over the current non-final `ELAB-07` payload,
not a final repair ABI decision and not a bundle semantics decision.

## Implemented fixture

The new Rust helper
`assert_complete_set_insertion_not_bundle_or_partial` checks the exact
positive `ELAB-07` payload without requiring full-object equality.

The dedicated test
`elab07_set_insertion_is_not_child_bundle_or_partial_guidance` verifies:

- `suggested_repair` has exactly one item;
- the single item has `repair_shape = set_insertion`;
- `coverage_scope = complete_missing_set_for_associated_request`;
- `local_premise_after_edit = discharged_for_associated_request`;
- `insert_failures == missing_evidence`;
- `declared_failures_before == failure_row_context.declared_failures`;
- `local_effect.declared_failures_after == required_failures`;
- `element_insert_count == insert_failures.len() == 3`;
- singleton-only `missing_failure` and `declared_failures` are absent;
- `repair_group_id`, `bundle_semantics`, `child_repairs`, `partiality`,
  `guidance_text`, and `textual_guidance` are absent.

The existing exact-positive `ELAB-07` test also calls the same helper.

## What this closes

This closes the current executable fixture gap for:

| Prior risk | Current guard |
|---|---|
| three child singleton alternatives | repair vector length must be 1 and the item must be `set_insertion` |
| one or two child alternatives counted as complete | inserted set must equal missing evidence and insert count must be 3 |
| textual-only guidance | no textual guidance keys and complete premise-after-edit marker required |
| partial guidance | no `partiality` key and complete premise-after-edit marker required |
| bundle semantics | no repair group / bundle / child-repair keys |

The guard is intentionally shape-level. It does not claim that future explicit
bundle semantics are invalid; it only prevents accidental widening of the
current exact `ELAB-07` payload before such semantics are designed.

## Current executable status

| Shape | Current result |
|---|---|
| exact single `ELAB-07` sample row | one complete non-final `set_insertion` item |
| child singleton alternatives for `ELAB-07` | not emitted |
| conjunctive bundle fields for `ELAB-07` | not emitted |
| partial guidance for `ELAB-07` | not emitted |
| textual-only guidance for `ELAB-07` | not emitted |
| omitted `fails` / row creation proxy | no repair |
| event / role / owner / state / field retargeting proxies | no `set_insertion` repair |
| two requests in one `when` row | no `set_insertion` repair |
| two distinct same-event `when` rows, one request each | each row may receive its own exact `set_insertion` repair |
| proper subset / padded / duplicate variants from `plan/103` | no `set_insertion` repair |
| `ELAB-04` | no `suggested_repair` |
| `ELAB-10` | one singleton `E-ROW-002` item |
| `ELAB-13..16` | singleton `E-ROW-001` items |

Sample row count is unchanged.

## Remaining gaps

The following remain future work:

- final row identity;
- durable source-locus identity;
- true multi-target-row policy;
- true row movement / cross-row retargeting policy;
- explicit bundle semantics, if a later package chooses to add them;
- partial guidance policy and whether it belongs in `suggested_repair[]` or a
  separate field;
- `ELAB-04` mixed visibility / base branch repair ownership and ranking;
- broader set-insertion support beyond the exact `ELAB-07` fact pattern.

## Relation to OBL-025

This package strengthens one LAB executable repair witness by ensuring it stays
a complete local set item rather than a set of alternatives or partial
guidance. It does not prove OBL-025 and does not move canon proof status.

OBL-025 should continue to distinguish complete local repair witnesses from
grouped multi-edit witnesses and partial guidance.

## Non-claims

- No canon edit.
- No production repair generation change.
- No expected JSON change.
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
