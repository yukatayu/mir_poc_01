# 390 — current L2 final-public-parser-checker-runtime-first-later-gate-actualization-ready minimal-final-public-parser-checker-runtime-first-later-gate threshold

## 目的

`specs/examples/389-current-l2-stable-malformed-missing-option-first-reopen-actualization-ready-final-public-parser-checker-runtime-first-later-gate-actualization-comparison.md`
で final public parser / checker / runtime first later gate actualization の current first choice を fixed した次段として、

- current actualization cut の minimum をどこまでに留めるか
- public entry / nested report / support / excluded / guard / kept-later refs をどこまで threshold に残すか
- next line を malformed-side source-backed widening と public operational CLI second gate にどう handoff するか

を比較する。

ここで固定するのは
**current L2 final-public-parser-checker-runtime-first-later-gate-actualization-ready minimal-final-public-parser-checker-runtime-first-later-gate threshold**
であり、

- final public parser grammar
- standalone parser/checker support public entry
- public operational CLI actual shape
- repo-layout 非依存の final input contract

はまだ固定しない。

## 比較観点

1. runtime-led thin facade と nested carrier cut を lossless に minimum へ残せるか
2. support-only tranche と excluded bucket を分けて保てるか
3. library-before-CLI later ordering と repo-layout guard を同時に維持できるか
4. malformed-side widening と CLI second gate への handoff を clean に残せるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_public_entry_refs + nested_public_report_refs + support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- runtime-led thin facade と nested report cut を lossless に残せる。
- support-only tranche と excluded bucket を別々に保持できる。
- malformed-side widening と CLI second gate の両方へ current line を handoff しやすい。

#### 欠点

- fields はやや多い。

### 案 2. `run_current_l2_source_sample` だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- nested report cut、support bucket、excluded bucket が落ちやすい。
- path resolver / accepted-set / host-plan coupling に関する guard が弱くなる。

### 案 3. `mir_runtime::current_l2` module 名だけを minimum に残す

#### 利点

- 表現は単純である。

#### 欠点

- symbol-level cut が失われ、module-wide promotion の誤読を招きやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_public_entry_refs + nested_public_report_refs + support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は symbol-level cut と guard の整理であり、runtime entry / nested report / support / excluded を lossless に残す必要がある。
2. current later ordering の意味は `run_current_l2_source_sample` current gate を保ったまま CLI second gate を遅らせることにあるため、excluded bucket と guard を threshold に残さないと drift しやすい。
3. malformed-side widening reserve と public operational CLI second gate を同時に kept-later に残す方が next snapshot の秩序が保ちやすい。

## current first choice shape

```text
final_public_parser_checker_runtime_first_later_gate = {
  actualization_kind = current_l2_runtime_led_thin_library_facade,
  entry_criteria_refs = [
    parser_checker_runtime_public_surface_inventory,
    public_operational_surface_actualization_gate,
    public_operational_cli_final_public_contract_later_gate
  ],
  chosen_public_entry_refs = [
    run_current_l2_source_sample,
    CurrentL2SourceSampleRunReport
  ],
  nested_public_report_refs = [
    CurrentL2LoweredSourceProgram,
    CurrentL2RuntimeSkeletonReport,
    CurrentL2CheckerFloorReport,
    RunReport
  ],
  support_refs = [
    run_current_l2_runtime_skeleton,
    lower_current_l2_fixed_source_text,
    static_gate_program_detailed,
    run_program_to_completion,
    FixtureHostStub_run_program,
    mir_ast_current_l2_parser_carrier_floor
  ],
  excluded_refs = [
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_python_orchestration_helpers,
    example_emitters_and_support_modules
  ],
  guard_refs = [
    keep_library_before_cli_later_ordering,
    do_not_promote_by_pub_visibility_only,
    keep_explicit_fixture_host_plan_coupling_outside_final_contract,
    keep_repo_layout_and_accepted_set_outside_final_contract,
    keep_partial_parser_surface_non_public
  ],
  kept_later_refs = [
    stable_malformed_missing_option_first_source_backed_widening_actualization,
    public_operational_cli_second_later_gate_actualization_comparison,
    final_parser_grammar,
    public_theorem_model_check_checker_migration
  ]
}
```

## practical reading

current minimal final public parser / checker / runtime first later gate が示すのは、

- runtime-led thin facade の first public cut は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` に置く
- parser/checker/runtime の detail は nested report carrier に留める
- `run_current_l2_runtime_skeleton`、`lower_current_l2_fixed_source_text`、semantic/checker core、parser carrier floor は support-only bucket に留める
- path resolver、accepted-set hard-coding、repo-local helper / example surface は excluded bucket に残す
- next line は malformed-side widening、その後段 reserve は public operational CLI second gate に置く

という最小 cut である。

## next promoted line

next promoted line は、
**final-public-parser-checker-runtime-first-later-gate-actualization-ready stable-malformed-missing-option-first-source-backed-widening-actualization comparison**
に置く。

## open questions

- runtime-led thin facade の次段で standalone support entry を later public support として切る必要があるか
- path-based sample selection と explicit `FixtureHostPlan` coupling を final public input contract からどう薄くするか
- public operational CLI second gate を runtime-led thin facade とどう分けて actualize するか
