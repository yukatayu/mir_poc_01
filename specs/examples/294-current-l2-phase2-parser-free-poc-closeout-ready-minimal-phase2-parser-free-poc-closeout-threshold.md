# 294 — current L2 phase2-parser-free-poc-closeout-ready minimal-phase2-parser-free-poc-closeout threshold

## 目的

`specs/examples/293-current-l2-phase1-semantics-closeout-ready-phase2-parser-free-poc-closeout-comparison.md`
で Phase 2 closeout comparison の first candidate を
narrow closeout bundle に置く判断を fixed した次段として、

- Phase 2 closeout の minimum をどこまでに留めるか
- compile gate / helper boundary / detached loop policy を minimum にどう反映するか
- bless / retention / public API をどこまで still later に残すか

を比較する。

ここで固定するのは
**current L2 phase2-parser-free-poc-closeout-ready minimal-phase2-parser-free-poc-closeout threshold**
であり、

- final retention/path policy
- reference update / bless workflow
- public exporter API
- production host interface

はまだ固定しない。

## 比較観点

1. Phase 2 closeout と final workflow / API finalization を distinguish できるか
2. compile gate と helper boundary を minimum に反映できるか
3. detached loop が compare-only non-production helper だと minimum に含められるか
4. next promoted line を narrow に Phase 4 closeout へ進められるか

## 比較対象

### 案 1. `compile_gate_refs` だけを持つ

#### shape

```text
phase2_parser_free_closeout_ready_sketch = {
  compile_gate_refs = [
    interpreter_regression_suite,
    detached_loop_unit_suite,
    detached_example_compile_gate,
    runtime_smoke_fixture_gate,
    single_fixture_aggregate_compare_gate,
    static_gate_checker_smoke_gate
  ]
}
```

#### 利点

- 軽い
- compile/test/smoke gate は見える

#### 欠点

- helper boundary と detached loop policy が minimum に現れない
- bless / retention / public API を still later に残す stop line が弱い

### 案 2. `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs + retained_later_refs` を持つ

#### shape

```text
phase2_parser_free_closeout_ready_sketch = {
  closeout_kind = parser_free_companion_baseline,
  compile_gate_refs = [
    interpreter_regression_suite,
    detached_loop_unit_suite,
    detached_example_compile_gate,
    runtime_smoke_fixture_gate,
    single_fixture_aggregate_compare_gate,
    static_gate_checker_smoke_gate
  ],
  helper_boundary_refs = [
    bundle_runtime_path,
    aggregate_compare_convenience,
    static_gate_checker_smoke_family,
    display_only_authoring_assists
  ],
  detached_loop_policy_refs = [
    compare_only_non_production_helper,
    target_current_l2_detached_default_candidate,
    diff_exit_code_one_is_informational
  ],
  retained_later_refs = [
    reference_update_bless_workflow,
    final_retention_path_policy,
    public_exporter_api
  ]
}
```

#### 利点

- compile gate、helper split、compare-only policy を minimum に同時に反映できる
- `target/current-l2-detached/` default candidate を final policy と誤読しにくい
- bless / retention / public API を retained-later line に残せる

#### 欠点

- 案 1 より field が増える

### 案 3. bless / retention / public API ref まで minimum に含める

#### 利点

- closeout minimum は大きく見える

#### 欠点

- current helper boundary を超えて運用 finalization を premature に混ぜる
- Phase 2 closeout ではなく future workflow decision を minimum に押し込む

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current baseline に必要なのは gate と policy/boundary の closeout である
2. detached loop の compare-only non-production status を minimum に反映できる
3. bless / retention / public API を still later に残しやすい

## current first choice shape

```text
phase2_parser_free_closeout_ready_sketch = {
  closeout_kind = parser_free_companion_baseline,
  compile_gate_refs = [
    interpreter_regression_suite,
    detached_loop_unit_suite,
    detached_example_compile_gate,
    runtime_smoke_fixture_gate,
    single_fixture_aggregate_compare_gate,
    static_gate_checker_smoke_gate
  ],
  helper_boundary_refs = [
    bundle_runtime_path,
    aggregate_compare_convenience,
    static_gate_checker_smoke_family,
    display_only_authoring_assists
  ],
  detached_loop_policy_refs = [
    compare_only_non_production_helper,
    target_current_l2_detached_default_candidate,
    diff_exit_code_one_is_informational
  ],
  retained_later_refs = [
    reference_update_bless_workflow,
    final_retention_path_policy,
    public_exporter_api
  ]
}
```

### この shape でまだ入れないもの

- reference update / bless の actual command
- final retention/path policy
- public exporter API
- production host interface

これらは still later である。

## practical reading

current minimal Phase 2 closeout が示すのは、

- parser-free PoC は current companion baseline として回る
- detached loop は compare-only non-production helper である
- compile/test/smoke gate は source-backed に固定した
- bless / retention / public API finalization は later に残した

という最小 cut である。

## next promoted line

next promoted line は、
**phase2-parser-free-poc-closeout-ready phase4-shared-space-closeout comparison**
に置く。

## open questions

- reference update / bless workflow を detached loop helper の外でどう actualize するか
- final retention/path policy をいつ固定するか
- public exporter API を bundle/batch/example のどこから切り出すか
