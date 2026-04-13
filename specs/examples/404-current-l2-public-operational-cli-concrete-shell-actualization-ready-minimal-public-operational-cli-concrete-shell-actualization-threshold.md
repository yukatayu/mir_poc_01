# 404 — current L2 public-operational-cli-concrete-shell-actualization-ready minimal-public-operational-cli-concrete-shell-actualization threshold

## 目的

`specs/examples/403-current-l2-stable-malformed-capability-second-source-backed-widening-actualization-ready-public-operational-cli-concrete-shell-actualization-comparison.md`
で public operational CLI concrete shell actualization の current first choice を fixed した次段として、

- current actual shell cut の minimum をどこまでに留めるか
- chosen shell / delegated entry / operational arguments / output / support / excluded / guard / kept-later refs をどこまで threshold に残すか
- next line を malformed capability source-backed widening actual package へどう handoff するか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-concrete-shell-actualization-ready minimal-public-operational-cli-concrete-shell-actualization threshold**
であり、

- installed public binary
- final `mir` top-level hierarchy
- final public parser / checker / runtime API
- final host/input contract

はまだ固定しない。

## 比較観点

1. chosen shell と delegated thin-facade entry を lossless に残せるか
2. support / excluded bucket と guard を明示できるか
3. next malformed capability actual package へ clean に handoff できるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_shell_refs + delegated_entry_refs + operational_argument_refs + operational_output_refs + support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- current actual shell concern の minimum を drift なく残せる。
- thin facade / support / excluded bucket の split を threshold に保持できる。
- next line を malformed capability source-backed widening actualization へ送りやすい。

#### 欠点

- fields はやや多い。

### 案 2. chosen shell と delegated entry だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- `<sample>`、`--host-plan`、`--format pretty|json` の operational shell concern が落ちやすい。
- support / excluded bucket が薄くなる。

### 案 3. installed binary / concrete packaging まで threshold に含める

#### 利点

- 具体性は高い。

#### 欠点

- current package が docs-only actualization comparison であることを越えて、later packaging を premature に固定する。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_shell_refs + delegated_entry_refs + operational_argument_refs + operational_output_refs + support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は actual shell concern と excluded bucket の整理であり、delegated thin-facade entry と operational shell concern を threshold に残す必要がある。
2. support-only / deeper-support bucket を guard なしで省くと、`run_current_l2_runtime_skeleton` や `lower_current_l2_fixed_source_text` の誤昇格を防ぎにくい。
3. installed binary や final host/input contract は still later なので、kept-later refs に留める方が自然である。

## current first choice shape

```text
public_operational_cli_concrete_shell_actualization = {
  actualization_kind = current_l2_scoped_rust_concrete_shell_over_thin_facade,
  entry_criteria_refs = [
    public_operational_cli_concrete_shell_naming,
    public_operational_cli_second_later_gate_actualization_comparison,
    final_public_parser_checker_runtime_thin_facade_later_support_actualization
  ],
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
  ],
  guard_refs = [
    keep_runtime_led_thin_facade_as_first_public_cut,
    keep_operational_wrapper_as_second_gate,
    keep_runtime_skeleton_as_later_support_cut,
    do_not_promote_repo_local_maintenance_helpers,
    do_not_fix_installed_binary_or_final_host_contract_here
  ],
  kept_later_refs = [
    stable_malformed_capability_second_source_backed_widening_actualization,
    installed_public_binary_packaging,
    final_mir_top_level_hierarchy,
    final_public_parser_checker_runtime_api,
    final_host_input_contract
  ]
}
```

## practical reading

current minimal public operational CLI concrete shell actualization が示すのは、

- current actual shell concern は `mir-current-l2 run-source-sample`
- delegated entry / report は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport`
- operational shell concern は `<sample>`、`--host-plan <path>`、`--format pretty|json`
- `run_current_l2_runtime_skeleton` は support refs に留め、`lower_current_l2_fixed_source_text` と repo-local helper 群は excluded bucket
- installed binary packaging、final `mir` hierarchy、final public parser/checker/runtime API、final host/input contract は kept-later
- next line は stable malformed capability second source-backed widening actualization

という最小 cut である。

## next promoted line

next promoted line は、
**public-operational-cli-concrete-shell-actualization-ready stable-malformed-capability-second-source-backed-widening-actualization**
に置く。

## open questions

- current-L2 scoped shell を later actual binary へどう packaging するか
- `--format pretty|json` を later public shell でも current naming のまま保つか
- maintenance helper shell と public shell を later でどう切り分けるか
