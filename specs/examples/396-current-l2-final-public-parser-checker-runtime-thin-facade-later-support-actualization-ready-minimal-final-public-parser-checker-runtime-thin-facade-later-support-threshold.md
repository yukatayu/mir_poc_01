# 396 — current L2 final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready minimal-final-public-parser-checker-runtime-thin-facade-later-support threshold

## 目的

`specs/examples/395-current-l2-public-operational-cli-second-later-gate-actualization-comparison-ready-final-public-parser-checker-runtime-thin-facade-later-support-actualization-comparison.md`
で final public parser/checker/runtime thin-facade later support actualization の current first choice を fixed した次段として、

- current support cut の minimum をどこまでに留めるか
- chosen support entry / explicit input / deeper-support / excluded / guard / kept-later refs をどこまで threshold に残すか
- next line を stable malformed capability second reopen actualization comparison へどう handoff するか

を比較する。

ここで固定するのは
**current L2 final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready minimal-final-public-parser-checker-runtime-thin-facade-later-support threshold**
であり、

- final public parser grammar
- final public parser/checker/runtime API
- public operational CLI concrete command/flag naming
- final host/input contract

はまだ固定しない。

## 比較観点

1. chosen support entry と explicit input cut を lossless に残せるか
2. deeper-support / excluded bucket と guard を明示できるか
3. thin facade first cut、CLI second gate、next malformed reopen line を同時に handoff できるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_support_refs + explicit_input_refs + deeper_support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- support cut の symbol-level shape と guard を lossless に残せる。
- thin facade first cut と CLI second gate を巻き戻さず、next line を capability second reopen へ handoff しやすい。

#### 欠点

- fields はやや多い。

### 案 2. `run_current_l2_runtime_skeleton` だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- explicit input surface、deeper-support、excluded bucket が落ちやすい。

### 案 3. `run_current_l2_runtime_skeleton` と `lower_current_l2_fixed_source_text` を together minimum に残す

#### 利点

- support line の厚みは見える。

#### 欠点

- partial parser surface promotion を threshold 側で先取りする。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_support_refs + explicit_input_refs + deeper_support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は standalone support cut と guard の整理であり、symbol-level support shape を lossless に残す必要がある。
2. `Program` / `FixtureHostPlan` / optional parser bridge input という explicit support input を threshold に残さないと、later support cut の意味が弱くなる。
3. thin facade first cut と CLI second gate を既に fixed 済みであるため、excluded / kept-later refs を threshold 側で明示する方が drift を防ぎやすい。

## current first choice shape

```text
final_public_parser_checker_runtime_thin_facade_later_support = {
  actualization_kind = current_l2_runtime_support_entry,
  entry_criteria_refs = [
    final_public_parser_checker_runtime_first_later_gate_actualization,
    public_operational_cli_second_later_gate_actualization_comparison,
    parser_checker_runtime_public_surface_inventory
  ],
  chosen_support_refs = [
    run_current_l2_runtime_skeleton,
    CurrentL2RuntimeSkeletonReport
  ],
  explicit_input_refs = [
    Program,
    FixtureHostPlan,
    optional_CurrentL2ParserBridgeInput
  ],
  deeper_support_refs = [
    lower_current_l2_fixed_source_text,
    static_gate_program_detailed,
    run_program_to_completion,
    FixtureHostStub_run_program,
    mir_ast_current_l2_parser_carrier_floor
  ],
  excluded_refs = [
    run_current_l2_source_sample,
    CurrentL2SourceSampleRunReport,
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ],
  guard_refs = [
    keep_runtime_led_thin_facade_as_first_public_cut,
    keep_cli_separate_as_second_gate,
    keep_partial_parser_surface_out_of_current_support_cut,
    keep_repo_layout_and_sample_policy_out_of_support_cut
  ],
  kept_later_refs = [
    stable_malformed_capability_second_reopen_actualization_comparison,
    public_operational_cli_concrete_shell_naming,
    final_public_parser_grammar,
    final_host_input_contract
  ]
}
```

## practical reading

current minimal final public parser/checker/runtime thin-facade later support が示すのは、

- later support cut は `run_current_l2_runtime_skeleton` と `CurrentL2RuntimeSkeletonReport` に置く
- explicit input surface は `Program`、`FixtureHostPlan`、optional parser bridge input に留める
- `lower_current_l2_fixed_source_text` と semantic/checker core、parser carrier floor は deeper-support bucket に残す
- thin facade first cut と CLI second gate は既存 fixed line に据え置く
- path resolver / accepted-set / repo-local helper surface は excluded bucket に残す
- next line は stable malformed capability second reopen actualization comparison に進める

という最小 cut である。

## next promoted line

next promoted line は、
**final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready stable-malformed-capability-second-reopen-actualization-comparison**
に置く。

## open questions

- `lower_current_l2_fixed_source_text` を support cut へ later 昇格させる必要があるか
- explicit `FixtureHostPlan` coupling を later support でどこまで保つか
- CLI second gate の concrete command/flag naming と support cut をどこで接続するか
