# 543 — current L2 IFC shared-output-contract threshold helper mirror

## 目的

`specs/examples/281-current-l2-minimal-public-checker-command-surface-ready-shared-output-contract-comparison.md`
と
`specs/examples/282-current-l2-shared-output-contract-ready-minimal-shared-output-contract-threshold.md`
では、

- shared output contract comparison の first candidate を shared family checker support の summary line に置くこと
- current minimum を
  `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を parser-front public checker boundary や emitted verifier handoff surface に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_shared_output_contract_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- detached loop wrapper path line の public surface 昇格
- row snippet textual rendering の public contract 化
- generic shared public checker entry
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/542` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_public_checker_command_surface_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた shared-output-contract line を、
**parser-front public checker boundary や emitted verifier handoff surface に上げずに、
minimal shared-output-contract ready sketch に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は
   `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref`
   に留める
4. `output_contract_kind` は `family_checker_row_compare_summary` に留める
5. `checker_cluster_name` は source-backed shared summary の representative anchor として
   `same_lineage_evidence_floor`
   に留める
6. `checker_status` は source-backed shared summary の representative status として
   `matched`
   に留める
7. `public_checker_payload_schema_ref` は `public_checker_payload_schema_ready_sketch` に留める
8. `next_comparison_target_ref` は `public_checker_boundary_comparison` に留める
9. wrapper path line と row snippet textual rendering は source-backed evidence として保持するが、current shape の first-class field には上げない
10. generic shared public checker entry / parser-front public checker boundary / emitted verifier handoff surface / final public verifier contract は still later に残す
11. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | output_contract_kind | checker_cluster_name | checker_status | current reading |
|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `family_checker_row_compare_summary` | `same_lineage_evidence_floor` | `matched` | IFC authority success sideでも shared-output-contract minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `family_checker_row_compare_summary` | `same_lineage_evidence_floor` | `matched` | authority miss negative sideでも同じ shared-output-contract ready sketch を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `family_checker_row_compare_summary` | `same_lineage_evidence_floor` | `matched` | label-flow negative sideでも同じ shared-output-contract ready sketch を共有する |

current helper-local cut の representative source-backed pressure は次に留める。

1. `scripts/current_l2_family_checker_support.py` が出す `cluster: ...` / `status: ...` summary line
2. `scripts/current_l2_same_lineage_checker.py`
3. `scripts/current_l2_detached_loop.py smoke-same-lineage-checker`

ここで `checker_cluster_name = same_lineage_evidence_floor` と
`checker_status = matched` は、
current source-side IFC trio 自身が shared family checker を直接実行しているという意味ではない。
current repo に actual source-backed で存在する shared summary line の representative anchor を、
helper-local threshold shape として mirror している、という意味である。

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_shared_output_contract_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_shared_output_contract_threshold_manifest,
  output_contract_kind = family_checker_row_compare_summary,
  checker_cluster_name = same_lineage_evidence_floor,
  checker_status = matched,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch,
  next_comparison_target_ref = public_checker_boundary_comparison,
  deferred_surface_refs = [
    static_gate_artifact_path,
    fixture_path,
    artifact_path,
    fixture_rows_textual_rendering,
    actual_rows_textual_rendering,
    generic_shared_public_checker_entry,
    parser_front_public_checker_boundary,
    verifier_handoff_surface
  ],
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_public_checker_command_surface_threshold` の次段にある helper-local threshold である
2. current checker-side package を shared-output-contract ready sketch の minimum として束ねる line であり、parser-front public checker boundary ではない
3. current summary は representative shared summary line に留め、wrapper path line や row snippet textual rendering は still later に残す
4. current threshold は docs-first shared-output-contract reading の helper-local mirror であり、final public output contract を意味しない
5. current next promoted line は `public_checker_boundary_comparison` であり、verifier handoff surface ではない

## 5. why this is enough

- `specs/examples/281` により、shared output contract comparison の first candidate は shared family checker support の summary line でよい
- `specs/examples/282` により、その minimum は `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` まででよい
- current repo では `scripts/current_l2_family_checker_support.py`、`scripts/current_l2_same_lineage_checker.py`、`smoke-same-lineage-checker` が source-backed に存在している
- `actual_public_checker_command_surface_threshold` が helper-local actualization 済みであり、IFC trio を current checker-side representative corpus に使ってよい

したがって current repo は、
parser-front public checker boundary や emitted verifier handoff surface を still later に残したまま、
shared-output-contract ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/281`
  - `specs/examples/282`
  - `specs/examples/542`
- source-backed pressure
  - `scripts/current_l2_family_checker_support.py`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_detached_loop.py`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_shared_output_contract_threshold` は helper-local / sample-local に留める
- current minimal bundle は representative shared summary line で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- detached loop wrapper path line の public surface 昇格
- row snippet textual rendering の public contract 化
- generic shared public checker entry
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract

## 8. retained later

- detached loop wrapper path line の public surface 昇格
- row snippet textual rendering の public contract 化
- generic shared public checker entry
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract
