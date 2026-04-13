# 394 — current L2 public-operational-cli-second-later-gate-actualization-comparison-ready minimal-public-operational-cli-second-later-gate threshold

## 目的

`specs/examples/393-current-l2-stable-malformed-missing-option-first-source-backed-widening-actualization-ready-public-operational-cli-second-later-gate-actualization-comparison.md`
で public operational CLI second later gate actualization comparison の current first choice を fixed した次段として、

- current operational wrapper cut の minimum をどこまでに留めるか
- delegated library entry / operational request / operational output / support / excluded / guard / kept-later refs をどこまで threshold に残すか
- next line を final public parser/checker/runtime thin-facade later support actualization へどう handoff するか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-second-later-gate-actualization-comparison-ready minimal-public-operational-cli-second-later-gate threshold**
であり、

- concrete command name
- concrete flag set
- final host/input contract
- raw FFI / engine / visualizer binding

はまだ固定しない。

## 比較観点

1. delegated library entry と operational shell concern を lossless に残せるか
2. support-only / excluded bucket と host/input guard を明示できるか
3. next line の thin-facade later support actualization と malformed capability reserve を clean に handoff できるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + delegated_library_refs + operational_request_refs + operational_output_refs + support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- library-side thin facade と CLI-side operational shell を separate に残せる。
- support-only / excluded bucket と host/input guard を落とさずに済む。
- next line を thin-facade later support actualization へ handoff しやすい。

#### 欠点

- fields はやや多い。

### 案 2. CLI second gate 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- delegated library entry、excluded bucket、host/input guard が落ちやすい。

### 案 3. command name / flag 名まで threshold に含める

#### 利点

- 具体的に見える。

#### 欠点

- operational shell naming を threshold で premature に固定する。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + delegated_library_refs + operational_request_refs + operational_output_refs + support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は Rust-side operational wrapper と delegated thin facade の境界整理であり、delegated library entry と excluded bucket を落とさず残す必要がある。
2. CLI second gate の意味は library-first ordering を保つことにあるため、support-only / excluded / guard を threshold に残さないと drift しやすい。
3. concrete command 名や flag 名は still OPEN なので、threshold では operational shell concern の種類だけを残す方が自然である。

## current first choice shape

```text
public_operational_cli_second_later_gate = {
  actualization_kind = current_l2_rust_operational_wrapper_over_thin_facade,
  entry_criteria_refs = [
    public_operational_cli_final_public_contract_later_gate,
    final_public_parser_checker_runtime_first_later_gate_actualization,
    docs_first_io_host_facing_port_boundary
  ],
  delegated_library_refs = [
    run_current_l2_source_sample,
    CurrentL2SourceSampleRunReport
  ],
  operational_request_refs = [
    sample_selector_argument,
    explicit_host_plan_input_mode
  ],
  operational_output_refs = [
    source_sample_run_report_json_or_pretty_summary
  ],
  support_refs = [
    run_current_l2_runtime_skeleton,
    lower_current_l2_fixed_source_text
  ],
  excluded_refs = [
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ],
  guard_refs = [
    keep_library_before_cli_later_ordering,
    do_not_promote_repo_local_python_helpers,
    keep_explicit_host_plan_decoupling_open,
    keep_host_facing_io_and_adapter_line_separate
  ],
  kept_later_refs = [
    final_public_parser_checker_runtime_thin_facade_later_support_actualization,
    stable_malformed_capability_second_reopen_actualization_comparison,
    concrete_command_name_and_flag_set,
    final_host_input_contract
  ]
}
```

## practical reading

current minimal public operational CLI second later gate が示すのは、

- CLI second gate は Rust-side operational wrapper over thin facade に留める
- delegated library entry は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` に据え置く
- `sample_selector_argument` と `explicit_host_plan_input_mode`、`source_sample_run_report_json_or_pretty_summary` だけを operational shell concern に残す
- `run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は still support-only bucket に留める
- path resolver、accepted-set hard-coding、repo-local Python helpers、cargo example emitters は excluded bucket に残す
- next line は thin-facade later support actualization に進める

という最小 cut である。

## next promoted line

next promoted line は、
**public-operational-cli-second-later-gate-actualization-comparison-ready final-public-parser-checker-runtime-thin-facade-later-support-actualization**
に置く。

## open questions

- concrete command name / flag naming をどこで固定するか
- explicit host-plan input mode を later support cut とどう整合させるか
- operational wrapper で JSON / pretty summary をどこまで current default にするか
