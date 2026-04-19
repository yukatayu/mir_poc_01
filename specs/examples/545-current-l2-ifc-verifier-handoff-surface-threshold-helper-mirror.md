# 545 — current L2 IFC verifier-handoff-surface threshold helper mirror

## 目的

`specs/examples/285-current-l2-minimal-public-checker-boundary-ready-verifier-handoff-surface-comparison.md`
と
`specs/examples/286-current-l2-verifier-handoff-surface-ready-minimal-verifier-handoff-surface-threshold.md`
では、

- verifier handoff surface comparison の first candidate を docs-only mixed-row bridge に置くこと
- current minimum を
  `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を actual emitted verifier handoff artifact や final parser grammar に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_verifier_handoff_surface_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- `subject_kind` / `subject_ref` / `proof_obligation_rows`
- theorem / protocol / runtime-policy dedicated handoff artifact family
- actual emitted verifier handoff artifact
- tool-specific schema / actual emitter policy
- final parser grammar / query token / shared generic entry
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/544` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_public_checker_boundary_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた verifier-handoff-surface line を、
**actual emitted verifier handoff artifact や theorem / protocol / runtime-policy dedicated split に上げずに、
minimal verifier-handoff-surface ready sketch に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は
   `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode`
   に留める
4. `handoff_surface_kind` は `docs_only_mixed_row_bundle_verifier_surface` に留める
5. `public_checker_boundary_ref` は `public_checker_boundary_ready_sketch` に留める
6. `proof_obligation_matrix_ref` は `current_l2_proof_obligation_matrix` に留める
7. `handoff_artifact_mode` は `docs_only_mixed_row_bundle` に留める
8. `next_comparison_target_ref` は `minimal_parser_subset_freeze_comparison` に留める
9. `subject rows` / dedicated handoff artifact family / actual emitted verifier handoff artifact / tool-specific schema / final parser grammar / final public verifier contract は still later に残す
10. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | handoff surface | boundary ref | matrix ref | artifact mode | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `docs_only_mixed_row_bundle_verifier_surface` | `public_checker_boundary_ready_sketch` | `current_l2_proof_obligation_matrix` | `docs_only_mixed_row_bundle` | IFC authority success sideでも verifier-handoff-surface minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `docs_only_mixed_row_bundle_verifier_surface` | `public_checker_boundary_ready_sketch` | `current_l2_proof_obligation_matrix` | `docs_only_mixed_row_bundle` | authority miss negative sideでも同じ verifier-handoff-surface ready sketch を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `docs_only_mixed_row_bundle_verifier_surface` | `public_checker_boundary_ready_sketch` | `current_l2_proof_obligation_matrix` | `docs_only_mixed_row_bundle` | label-flow negative sideでも同じ verifier-handoff-surface ready sketch を共有する |

current helper-local cut の deferred surface ref は次に留める。

1. `subject_kind`
2. `subject_ref`
3. `proof_obligation_rows`
4. `theorem_handoff_artifact_ref`
5. `protocol_handoff_artifact_ref`
6. `runtime_policy_handoff_artifact_ref`
7. `actual_emitted_verifier_handoff_artifact`
8. `tool_specific_schema_and_actual_emitter_policy`
9. `final_parser_grammar`
10. `query_token_and_shared_generic_entry`

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_verifier_handoff_surface_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_verifier_handoff_surface_threshold_manifest,
  handoff_surface_kind = docs_only_mixed_row_bundle_verifier_surface,
  public_checker_boundary_ref = public_checker_boundary_ready_sketch,
  proof_obligation_matrix_ref = current_l2_proof_obligation_matrix,
  handoff_artifact_mode = docs_only_mixed_row_bundle,
  next_comparison_target_ref = minimal_parser_subset_freeze_comparison,
  deferred_surface_refs = [
    subject_kind,
    subject_ref,
    proof_obligation_rows,
    theorem_handoff_artifact_ref,
    protocol_handoff_artifact_ref,
    runtime_policy_handoff_artifact_ref,
    actual_emitted_verifier_handoff_artifact,
    tool_specific_schema_and_actual_emitter_policy,
    final_parser_grammar,
    query_token_and_shared_generic_entry
  ],
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_public_checker_boundary_threshold` の次段にある helper-local threshold である
2. current checker-side package を verifier-handoff-surface ready sketch の minimum として束ねる line であり、actual emitted verifier handoff artifact ではない
3. current summary は docs-only mixed-row bridge に留め、subject row や boundary-specific split は still later に残す
4. current threshold は docs-first verifier-handoff-surface reading の helper-local mirror であり、final external consumer contract を意味しない
5. current next promoted line は `minimal_parser_subset_freeze_comparison` であり、parser/public API adoption ではない

## 5. why this is enough

- `specs/examples/285` により、verifier handoff surface comparison の first candidate は docs-only mixed-row bridge でよい
- `specs/examples/286` により、その minimum は `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` まででよい
- current repo では `actual_public_checker_boundary_threshold` が helper-local actualization 済みであり、その次段を docs-only mixed-row bridge に留める条件が揃っている
- current checker-side line の immediate pressure は minimal parser subset freeze comparison へ進むことであり、actual emitted verifier handoff artifact の adoption ではない

したがって current repo は、
actual emitted verifier handoff artifact や final parser grammar を still later に残したまま、
verifier-handoff-surface ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/285`
  - `specs/examples/286`
  - `specs/examples/544`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_verifier_handoff_surface_threshold` は helper-local / sample-local に留める
- current minimal bundle は docs-only mixed-row bridge bundle で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- `subject_kind` / `subject_ref` / `proof_obligation_rows`
- theorem / protocol / runtime-policy dedicated handoff artifact family
- actual emitted verifier handoff artifact
- tool-specific schema / actual emitter policy
- final parser grammar / query token / shared generic entry
- final public verifier contract

## 8. retained later

- `subject_kind` / `subject_ref` / `proof_obligation_rows`
- theorem / protocol / runtime-policy dedicated handoff artifact family
- actual emitted verifier handoff artifact
- tool-specific schema / actual emitter policy
- final parser grammar / query token / shared generic entry
- final public verifier contract
