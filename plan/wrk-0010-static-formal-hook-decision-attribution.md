# WRK-0010 static formal-hook decision attribution evidence

**LAB evidence.** This record does not change Canon diagnostics, theorem/OBL
status, carrier, helper/schema, runtime, Gate/Phase, conformance, or workflow.
It records the registered run at clean base
`b25685061cea126eb19d747a3ba8148d3080f7a2`.

## Question and comparison rule

The question is whether each e4/e5/e12/e14 `fixture_static_cluster` formal-hook
artifact preserves its static-gate decision payload: literal verdict, raw
reasons, and `detached_noncore` presence/reason-code value; or an explicit
lossless reference to the exact static-gate artifact. A same-fixture identifier
alone is not an exact artifact reference and no inferred lookup is allowed.

## Reproduced execution

The committed command ran with temporary root
`/tmp/mirrorea-wrk0010-static-attribution.hp20Wu`. The formal-hook support
target passed 5/5 tests; the four static smokes succeeded; the unchanged
current-L2 regression passed all 23 commands. The temporary root is disposable
and is not retained.

## Literal matrix

| Fixture | Static-gate payload | Formal-hook payload | Classification |
| --- | --- | --- | --- |
| e4 | `malformed`; `lineage assertion does not describe primary -> mirror`; `[{"kind":"lineage_assertion_edge_mismatch","predecessor":"primary","successor":"mirror"}]` | `canonical_normalization_law`, `no_re_promotion`; `static_gate_artifact:e4_malformed_lineage` only | no decision-payload attribution |
| e5 | `underdeclared`; `missing lineage assertion for primary -> mirror`; `[{"kind":"missing_lineage_assertion","predecessor":"primary","successor":"mirror"}]` | `canonical_normalization_law`, `no_re_promotion`; `static_gate_artifact:e5_underdeclared_lineage` only | no decision-payload attribution |
| e12 | `underdeclared`; `declared access target is missing for primary -> mirror`; `[{"kind":"declared_target_missing","predecessor":"primary","successor":"mirror"}]` | `canonical_normalization_law`, `no_re_promotion`; `static_gate_artifact:e12_underdeclared_target_missing` only | no decision-payload attribution |
| e14 | `malformed`; `duplicate option declaration profile_ref is visible from root / session / profile_access and root / session / profile_access`; `detached_noncore` absent | `canonical_normalization_law`, `no_re_promotion`; `static_gate_artifact:e14_malformed_duplicate_option_declaration` only | no decision-payload attribution |

All four formal hooks are `fixture_static_cluster` and contain the same
`canonical_normalization_law` then `no_re_promotion` obligation kinds. Their
typed `static_gate_artifact` references have only the displayed fixture IDs,
not an exact artifact path or digest. They therefore cannot recover the
selected decision payload under the registered rule.

## Result and stop line

**Result class: no decision-payload attribution.** The expected full-attribution
falsifier did not occur. This classifies a bounded existing artifact shape only.
It does not say the hook is defective, that the static gate has Canon diagnostic
meaning, that any payload must be added, or that any theorem/OBL/carrier result
follows. No source is changed.

## Reopen condition

Only a separately registered existing-lane question with an explicit
lossless-reference source or a distinct coverage relation may reopen this area.
Do not add fields, schema, helper, fixture, test, runner, or semantic mapping
from this result.
