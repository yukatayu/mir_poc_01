# 403 — current L2 stable-malformed-capability-second-source-backed-widening-actualization-ready public-operational-cli-concrete-shell-actualization comparison

## 目的

`specs/examples/402-current-l2-stable-malformed-capability-second-source-backed-widening-actualization-ready-minimal-stable-malformed-capability-second-source-backed-widening-threshold.md`
で malformed capability side の next actual package minimum を fixed した次段として、

- current-L2 scoped docs-only shell naming を actual shell concern へどの narrow cut から reopen するか
- runtime-led thin facade first cut、Rust-side operational wrapper second gate、runtime skeleton later support cutをどこまで current actual shell concern の前提に残すか
- concrete shell concern と final public parser/checker/runtime API、repo-local maintenance helper、final host/input contract をどこまで separate に保つか

を比較する。

ここで固定するのは
**current L2 stable-malformed-capability-second-source-backed-widening-actualization-ready public-operational-cli-concrete-shell-actualization comparison**
であり、

- installed public binary
- final `mir` top-level hierarchy
- final public parser / checker / runtime API
- final host/input contract
- repo-local inventory / regression helper shell

はまだ固定しない。

## scope

- current package は docs-only actualization comparison に留める。
- `specs/examples/399...400` の current-L2 scoped shell naming を entry evidence に使う。
- `specs/examples/393...394` の Rust-side operational wrapper second gate と `specs/examples/395...396` の runtime skeleton later support cut を巻き戻さない。
- `run_current_l2_source_sample` の public/report shape は current first cut のまま維持する。

## current 前提

current repo では次が成立している。

1. `specs/examples/389...390` により、current first later cut は `run_current_l2_source_sample` + `CurrentL2SourceSampleRunReport` の runtime-led thin facade に固定済みである。
2. `specs/examples/393...394` により、public operational CLI second gate は Rust-side operational wrapper over thin facade に固定済みであり、current shell concern は `sample_selector_argument` / `explicit_host_plan_input_mode` / `source_sample_run_report_json_or_pretty_summary` に narrow に残している。
3. `specs/examples/395...396` により、`run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` は later support cut に留め、`lower_current_l2_fixed_source_text`、semantic/checker core、parser carrier floor は deeper-support bucket に置いている。
4. `specs/examples/399...400` により、current docs-only shell naming は `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` に固定済みである。
5. repo-local Python helper、cargo example emitter、accepted-set hard-coding、path resolver、inventory/regression helper verb は current public shell concern から excluded bucket に置いている。

したがって current 問いは、
**current-L2 scoped shell naming を保ったまま、actual shell concern の current first cut を thin facade over `run_current_l2_source_sample` を包む Rust-side concrete shell に留めるのが最小か**
である。

## 比較観点

1. runtime-led thin facade first cut、Rust-side operational wrapper second gate、runtime skeleton later support cutを同時に巻き戻さずに済むか
2. current-L2 scoped shell naming と actual shell concern を lossless に残せるか
3. repo-local maintenance helper / support-only verb / deeper-support entry を public operational shell から外したままにできるか
4. next line を malformed capability source-backed widening actual package へ clean に handoff できるか

## 比較対象

### 案 1. current-L2 scoped Rust concrete shell over thin facade を current first cut に置く

#### shape

```text
public_operational_cli_concrete_shell = {
  actualization_kind = current_l2_scoped_rust_concrete_shell_over_thin_facade,
  chosen_shell_refs = [
    mir-current-l2,
    run-source-sample
  ],
  delegated_entry_refs = [
    run_current_l2_source_sample,
    CurrentL2SourceSampleRunReport
  ],
  operational_argument_refs = [
    sample_selector_positional_argument,
    host_plan_explicit_path_flag
  ],
  operational_output_refs = [
    format_pretty_or_json_flag
  ],
  support_refs = [
    run_current_l2_runtime_skeleton,
    CurrentL2RuntimeSkeletonReport
  ],
  excluded_refs = [
    lower_current_l2_fixed_source_text,
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_inventory_and_regression_helpers,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ]
}
```

#### 利点

- current-L2 scoped naming と current operational shell concern をそのまま actual shell concern に送れる。
- thin facade / operational wrapper / later support の 3 段 split を保てる。
- repo-local maintenance helper と deeper-support helper を public shell concern へ誤昇格させにくい。

#### 欠点

- installed binary 名、final top-level hierarchy、host-plan decoupling は still later に残る。

### 案 2. repo-local Python helper を current actual shell とみなす

#### 利点

- すぐに callable な surface は見えやすい。

#### 欠点

- inventory / regression / detached-loop helper と public operational shell の境界が崩れやすい。
- Rust-heavy core + mixed-tool helper workflow の current split と衝突する。

### 案 3. `run_current_l2_source_sample` をそのまま concrete shell とみなす

#### 利点

- surface は最小である。

#### 欠点

- library contract と shell concern の差が消える。
- `mir-current-l2 run-source-sample` naming を actual shell concern として separate に保てない。

## current judgment

current L2 で最も自然なのは、
**案 1. current-L2 scoped Rust concrete shell over thin facade を current first cut に置く**
である。

理由は次の通り。

1. naming package で fixed した `mir-current-l2 run-source-sample` は docs-only shell family として十分 narrow であり、そのまま thin facade over `run_current_l2_source_sample` を包む actual shell concern に送るのが自然である。
2. repo-local Python helper 群は regression / detached-loop / docs maintenance の責務を持っており、public operational shell へ昇格させると current public/support split を壊しやすい。
3. runtime skeleton later support cut と deeper-support cut を別 bucket に残すことで、current actual shell concern が過剰に肥大化するのを防げる。

## current first choice details

- current actual shell concern は `mir-current-l2 run-source-sample` を current-L2 scoped Rust concrete shell として扱う cut に置く。
- delegated entry / report は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` に留める。
- operational argument surface は `<sample>`、`--host-plan <path>`、`--format pretty|json` に留める。
- `run_current_l2_runtime_skeleton` と `CurrentL2RuntimeSkeletonReport` は current shell concern の support refs に残すが、直接の public shell entry には昇格させない。
- `lower_current_l2_fixed_source_text`、path resolver、accepted-set hard-coding、repo-local inventory / regression helper、repo-local Python helper、cargo example emitter / support module は excluded bucket に残す。
- installed binary packaging、final `mir` hierarchy、final public parser/checker/runtime API、final host/input contract は kept-later に残す。

## next promoted line

next promoted line は、
**public-operational-cli-concrete-shell-actualization-comparison-ready stable-malformed-capability-second-source-backed-widening-actualization**
に置く。

## open questions

- current-L2 scoped Rust concrete shell を later actual binary へどの packaging cut で落とすか
- `--host-plan` を later public contract で explicit path のまま保つか
- repo-local inventory / regression helper shell を maintenance shell として later separate 命名するか
