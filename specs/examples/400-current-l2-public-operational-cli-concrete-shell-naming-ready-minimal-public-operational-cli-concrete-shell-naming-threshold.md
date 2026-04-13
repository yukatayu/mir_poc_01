# 400 — current L2 public-operational-cli-concrete-shell-naming-ready minimal-public-operational-cli-concrete-shell-naming threshold

## 目的

`specs/examples/399-current-l2-stable-malformed-capability-second-reopen-actualization-ready-public-operational-cli-concrete-shell-naming-comparison.md`
で public operational CLI concrete shell naming の current first choice を fixed した次段として、

- current shell naming cut の minimum をどこまでに留めるか
- chosen shell / argument / output / excluded / guard / kept-later refs をどこまで threshold に残すか
- next line を stable malformed capability second source-backed widening actualization comparison へどう handoff するか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-concrete-shell-naming-ready minimal-public-operational-cli-concrete-shell-naming threshold**
であり、

- actual public CLI binary
- final `mir` top-level hierarchy
- final public parser / checker / runtime API
- final host/input contract

はまだ固定しない。

## 比較観点

1. chosen shell naming と operational shell concern を lossless に残せるか
2. excluded bucket と guard を明示できるか
3. runtime-led thin facade / operational wrapper / later support split を巻き戻さずに next malformed line へ handoff できるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_shell_refs + operational_argument_refs + operational_output_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- current L2 scoped shell naming と excluded bucket を drift なく残せる。
- final hierarchy や support verb の premature promotion を guard で抑えやすい。
- next line を malformed capability source-backed widening actualization comparison へ handoff しやすい。

#### 欠点

- fields はやや多い。

### 案 2. command family / verb 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- `<sample>`、`--host-plan`、`--format` の operational shell concern が落ちやすい。
- excluded bucket と guard が薄くなりやすい。

### 案 3. actual binary / shell parser binding まで threshold に含める

#### 利点

- 具体的に見える。

#### 欠点

- docs-only naming comparison を越えて actual public CLI binding を premature に固定する。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_shell_refs + operational_argument_refs + operational_output_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は naming cut と excluded bucket の整理であり、support verb や repo-local helper を public shell から外す guard を残す必要がある。
2. `<sample>`、`--host-plan`、`--format pretty|json` は current operational shell concern そのものであり、これを落とすと naming threshold の意味が弱くなる。
3. actual binary や final hierarchy は still later なので、docs-only naming cut の threshold では kept-later refs に留める方が自然である。

## current first choice shape

```text
public_operational_cli_concrete_shell_naming = {
  actualization_kind = current_l2_scoped_docs_only_operational_shell,
  entry_criteria_refs = [
    public_operational_cli_second_later_gate_actualization_comparison,
    final_public_parser_checker_runtime_thin_facade_later_support_actualization,
    public_operational_cli_final_public_contract_later_gate
  ],
  chosen_shell_refs = [
    mir-current-l2,
    run-source-sample
  ],
  operational_argument_refs = [
    sample_selector_positional_argument,
    host_plan_explicit_path_flag
  ],
  operational_output_refs = [
    format_pretty_or_json_flag
  ],
  excluded_refs = [
    final_mir_top_level_hierarchy,
    inventory_and_regression_repo_local_verbs,
    runtime_skeleton_or_lowering_support_verbs,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ],
  guard_refs = [
    keep_runtime_led_thin_facade_as_first_public_cut,
    keep_operational_wrapper_as_second_gate,
    do_not_promote_repo_local_maintenance_helpers,
    do_not_fix_final_host_input_contract_here
  ],
  kept_later_refs = [
    stable_malformed_capability_second_source_backed_widening_actualization_comparison,
    public_operational_cli_concrete_shell_actualization_comparison,
    final_public_parser_checker_runtime_api,
    final_host_input_contract
  ]
}
```

## practical reading

current minimal public operational CLI concrete shell naming が示すのは、

- current docs-only shell family は `mir-current-l2 run-source-sample`
- current operational shell concern は `<sample>`、`--host-plan <path>`、`--format pretty|json`
- final `mir` hierarchy、inventory / regression helper verb、support-only verb、repo-local helper naming は excluded bucket
- actual binary / shell parser binding / final host-input contract は kept-later
- next line は stable malformed capability second source-backed widening actualization comparison

という最小 cut である。

## next promoted line

next promoted line は、
**public-operational-cli-concrete-shell-naming-ready stable-malformed-capability-second-source-backed-widening-actualization-comparison**
に置く。

## open questions

- current-L2 scoped shell naming を later actualization でどう executable 名へ落とすか
- `--format pretty|json` を later public shell で enum flag のまま保つか
- support verb や inventory/regression verb を separate maintenance shell として later 整理するか
