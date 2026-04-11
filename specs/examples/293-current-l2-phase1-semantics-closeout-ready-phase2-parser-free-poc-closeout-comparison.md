# 293 — current L2 phase1-semantics-closeout-ready phase2-parser-free-poc-closeout comparison

## 目的

`specs/examples/291-current-l2-parser-to-checker-reconnect-freeze-ready-phase1-semantics-closeout-comparison.md`
と
`specs/examples/292-current-l2-phase1-semantics-closeout-ready-minimal-phase1-semantics-closeout-threshold.md`
で Phase 1 closeout を fixed した次段として、

- parser-free PoC / detached validation loop をどの closeout cut で mainline companion baseline と呼べるか
- compile gate、detached loop policy、helper responsibility split をどこまで同じ package で閉じるべきか
- bless / reference update、final retention/path policy、public exporter API をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 phase1-semantics-closeout-ready phase2-parser-free-poc-closeout comparison**
であり、

- final retention/path policy
- public exporter API
- reference update / bless workflow
- production host interface

はまだ固定しない。

## scope

- current parser-free PoC / detached validation loop closeout だけを扱う。
- `plan/07-parser-free-poc-stack.md` と
  `plan/09-helper-stack-and-responsibility-map.md`
  の current rule を維持する。
- detached loop は compare-only / non-production helper として扱う。
- `target/current-l2-detached/` は current default candidate に留める。
- reference update / bless と final retention/path policy には進まない。

## current 前提

current repo では次が成立している。

1. `mir-semantics` には parser-free current L2 minimal interpreter と host harness があり、interpreter suite が通る。
2. detached loop には bundle / aggregate / static gate の 3 example emitter と unit tests がある。
3. `smoke-fixture`、`compare-fixture-aggregates`、`smoke-try-rollback-structural-checker` は current smoke family として使える。
4. detached loop は compare-only / non-production helper であり、compare helper の exit code `1` は informational difference に留める。
5. `reference update / bless` は detached loop helper に入れず、final retention/path policy と接続する later decision に残している。

したがって current 問いは、
**code 主線を変えずに、Phase 2 closeout として何を snapshot 側へ明示すればよいか**
である。

## 比較観点

1. compile / test / smoke gate を current baseline として前面化できるか
2. detached loop が compare-only non-production helper だと minimum に出せるか
3. runtime bundle/aggregate path と static-gate-side checker smoke family、display-only assist の責務分離を保てるか
4. bless / retention / public API を premature に closeout minimum に混ぜないか

## 比較対象

### 案 1. compile gate list だけを snapshot に足し、policy / boundary は詳細 docs に残す

#### 利点

- 軽い
- `plan/07` / `plan/09` をそのまま正本にできる

#### 欠点

- detached loop が compare-only non-production helper だと snapshot から読み取りにくい
- bless / retention / public API をまだ決めていない stop line が見えにくい
- helper responsibility split が top-level で再確認しにくい

### 案 2. `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs + retained_later_refs` を持つ narrow closeout bundle を切る

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

- compile gate と helper policy を同じ closeout cut に置ける
- detached loop の compare-only / non-production boundary を明示できる
- reference update / bless と final retention/path policy を still later に残せる
- next promoted line を Phase 4 closeout へ移しやすい

#### 欠点

- compile gate list だけよりは field が増える

### 案 3. bless / retention / public exporter API まで minimum に含める

#### 利点

- closeout の外形は大きく見える

#### 欠点

- current helper boundary を超えて final workflow を premature に固定しやすい
- `target/current-l2-detached/` default candidate を final policy と誤読しやすい
- Phase 2 closeout より広い運用 finalization になる

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs + retained_later_refs` を持つ narrow closeout bundle を切る**
である。

理由は次の通り。

1. parser-free baseline 自体はすでに code で成立しており、必要なのは snapshot 側の closeout である
2. detached loop の compare-only / non-production boundary と helper split を同時に明示できる
3. bless / retention / public API を still later に残したまま Phase 2 を閉じられる
4. current mainline を Phase 4 shared-space closeout へ移せる

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

## この shape でまだ入れないもの

- reference update / bless の actual workflow
- final retention/path policy
- public exporter API
- production host interface

これらは still later である。

## practical reading

current Phase 2 closeout が示すのは、次の 4 点だけである。

1. interpreter suite、example compile gate、detached loop unit tests、runtime/static smoke family は current baseline である
2. detached loop は compare-only / non-production helper である
3. runtime bundle/aggregate path、static-gate-side checker smoke family、display-only assist は責務を分けている
4. bless / retention / public API は later decision に残す

## next promoted line

next promoted line は、
**phase2-parser-free-poc-closeout-ready phase4-shared-space-closeout comparison**
に置く。

## open questions

- reference update / bless workflow をどの phase で actualize するか
- final retention/path policy を detached loop helper とどこで切り分けるか
- public exporter API を設けるならどの layer から始めるか
