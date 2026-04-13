# 372 — current L2 public-operational-cli-final-public-contract-later-gate-ready minimal-public-operational-cli-final-public-contract-later-gate threshold

## 目的

`specs/examples/371-current-l2-stable-malformed-broader-follow-up-inventory-ready-public-operational-cli-final-public-contract-later-gate-comparison.md`
で public operational later-gate ordering の current first choice を fixed した次段として、

- current candidate / first later gate / second later gate の minimum をどこまでに留めるか
- excluded bucket と guard をどこまで threshold に持つか
- next promoted line と kept-later public pressure line をどう残すか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-final-public-contract-later-gate-ready minimal-public-operational-cli-final-public-contract-later-gate threshold**
であり、

- final public parser / checker / runtime API の actual shape
- public operational CLI の actual shape
- support symbol の promotion timing

はまだ固定しない。

## 比較観点

1. current candidate と first/second later gate を lossless に残せるか
2. excluded bucket と guard を明示できるか
3. shared-space next line と public operational later pressure を両立して handoff できるか

## 比較対象

### 案 1. later gate 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current candidate、excluded bucket、guard が抜けやすい。

### 案 2. `gate_kind + entry_criteria_refs + current_candidate_refs + first_later_gate_refs + second_later_gate_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- current candidate、library-side first gate、CLI second gate、excluded bucket を一体で残せる。
- repo layout / accepted-set / host-plan coupling を current final contract へ混ぜない guard を維持できる。
- shared-space next line と separate later public pressure line を両方 handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. final public parser / checker / runtime API の support symbol 群まで threshold に含める

#### 利点

- next actualization は見えやすい。

#### 欠点

- later actualization package を threshold で先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `gate_kind + entry_criteria_refs + current_candidate_refs + first_later_gate_refs + second_later_gate_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は later ordering close であり、actual public contract design は次段へ残すべきである。
2. current candidate と excluded bucket を threshold 側で残さないと、CLI second gate の意味が弱くなる。
3. shared-space next line と Macro 7 later actualization line を一緒に handoff するには kept-later refs が必要である。

## current first choice shape

```text
public_operational_later_gate = {
  gate_kind = current_l2_library_before_cli_later_gate,
  entry_criteria_refs = [
    public_operational_surface_actualization_gate,
    stable_malformed_broader_followup_inventory
  ],
  current_candidate_refs = [
    current_l2_source_sample_runner_entry
  ],
  first_later_gate_refs = [
    final_public_parser_checker_runtime_api
  ],
  second_later_gate_refs = [
    public_operational_cli
  ],
  excluded_refs = [
    source_sample_path_resolution,
    repo_layout_bound_accepted_sample_set,
    example_emitters_and_support_modules,
    repo_local_python_orchestration_helpers
  ],
  guard_refs = [
    do_not_promote_by_pub_visibility_only,
    keep_repo_layout_and_accepted_set_outside_final_contract,
    avoid_freezing_explicit_host_plan_coupling,
    keep_cli_separate_from_library_contract
  ],
  kept_later_refs = [
    shared_space_admission_compile_time_visibility_reopen,
    final_public_parser_checker_runtime_api_actualization,
    public_operational_cli_second_gate,
    public_theorem_model_check_checker_migration_line
  ]
}
```

## practical reading

current minimal public operational later gate が示すのは、

- current gate entry は `run_current_l2_source_sample` に据え置く
- first later gate は final public parser / checker / runtime API に置く
- public operational CLI は second later gate に残す
- repo layout / accepted-set / repo-local helpers は current final contract の外に残す
- repo-level next line は shared-space admission / compile-time visibility reopen に進める

という最小 cut である。

## next promoted line

next promoted line は、
**public-operational-cli-final-public-contract-later-gate-ready shared-space-admission-compile-time-visibility-reopen comparison**
に置く。

## open questions

- final public parser / checker / runtime API first later gate actualization を symbol-level でどう narrow に切るか
- public operational CLI second gate を repo-local Python helper とどう分離するか
- explicit `FixtureHostPlan` coupling を library-side final contract でどこまで保つか
