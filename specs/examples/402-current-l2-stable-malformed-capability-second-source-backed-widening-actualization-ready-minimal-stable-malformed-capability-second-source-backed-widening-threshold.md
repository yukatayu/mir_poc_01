# 402 — current L2 stable-malformed-capability-second-source-backed-widening-actualization-ready minimal-stable-malformed-capability-second-source-backed-widening threshold

## 目的

`specs/examples/401-current-l2-public-operational-cli-concrete-shell-naming-ready-stable-malformed-capability-second-source-backed-widening-actualization-comparison.md`
で stable malformed capability second source-backed widening actualization の current first choice を fixed した次段として、

- current actualization comparison result の minimum をどこまでに留めるか
- chosen family / entry evidence / actualized surface / guard / kept-later refs をどこまで threshold に残すか
- next line を public operational CLI concrete shell actualization comparison へどう handoff するか

を比較する。

ここで固定するのは
**current L2 stable-malformed-capability-second-source-backed-widening-actualization-ready minimal-stable-malformed-capability-second-source-backed-widening threshold**
であり、

- actual source-backed widening 実装
- public operational CLI actual shell
- duplicate cluster の actual reopen
- `TryFallback` / `AtomicCut` malformed-static broader reopen

はまだ固定しない。

## 比較観点

1. chosen family と entry evidence を lossless に残せるか
2. actualized surface と guard を明示できるか
3. current next line を public operational CLI concrete shell actualization comparison へ clean に handoff できるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_family_refs + entry_evidence_refs + actualized_surface_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- `e13/e20` pair judgment、helper / parser / fixture-static evidence、actualized surface を同時に残せる。
- public operational CLI actual shell との sequencing を drift なく handoff しやすい。

#### 欠点

- fields はやや多い。

### 案 2. `chosen_family_refs + actualized_surface_refs` だけを残す

#### 利点

- 軽い。

#### 欠点

- helper-local capability compare と stage1 reconnect widen を落としやすい。
- duplicate later / malformed-static later の guard が薄くなりやすい。

### 案 3. `e13` lead staging note まで threshold から落とす

#### 利点

- minimum はさらに軽い。

#### 欠点

- implementation order だけを narrow に取っても pair judgment を保つ current reading が見えにくくなる。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_family_refs + entry_evidence_refs + actualized_surface_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は pair judgment / entry evidence / actualized surface の 3 点を揃えることであり、どれかを落とすと why now が弱くなる。
2. current malformed-side widening は missing-option family precedentとの symmetry を重視しているため、guard と kept-later refs を threshold 側にも残す方が自然である。
3. next package は public operational CLI concrete shell actualization comparison であり、その sequencing を threshold 側に残す必要がある。

## current first choice shape

```text
stable_malformed_capability_second_source_backed_widening = {
  actualization_kind = current_l2_capability_source_authored_static_stop_pair,
  entry_criteria_refs = [
    stable_malformed_capability_second_reopen_actualization,
    stable_malformed_missing_option_first_source_backed_widening_actualization,
    public_operational_cli_concrete_shell_naming_comparison
  ],
  chosen_family_refs = [
    capability_strengthening_floor,
    e13_malformed_capability_strengthening,
    e20_malformed_late_capability_strengthening
  ],
  entry_evidence_refs = [
    helper_local_capability_compare,
    stage1_parser_reconnect_widening,
    fixture_static_gate_capability_rows
  ],
  actualized_surface_refs = [
    source_sample_files,
    source_lowering_runner_ladder_widening,
    regression_inventory_and_regression_bundle_widening,
    fixture_static_formal_hook_smoke_widening
  ],
  guard_refs = [
    keep_e13_e20_pair_judgment,
    keep_public_report_shape_unchanged,
    keep_duplicate_cluster_later,
    keep_try_rollback_malformed_static_later,
    keep_theorem_model_check_public_checker_later
  ],
  kept_later_refs = [
    public_operational_cli_concrete_shell_actualization_comparison,
    duplicate_cluster_later_reopen,
    try_rollback_malformed_static_broader_reopen
  ]
}
```

## practical reading

current minimal stable malformed capability second source-backed widening が示すのは、

- current actualization family は `e13/e20` source-authored static-stop pair
- helper-local capability compare、stage1 reconnect widen、fixture-static capability rows を entry evidence に再利用する
- current actualized surface は source sample / lowerer / runner / ladder / regression helper / fixture-static formal-hook smoke までに留める
- public/report shape、theorem/model-check/public-checker widening、duplicate cluster、malformed-static broader reopen は kept-later / guard に残す
- repo-level next package は public operational CLI concrete shell actualization comparison に進めてよい

という minimum である。

## next promoted line

next promoted line は、
**stable-malformed-capability-second-source-backed-widening-actualization-ready public-operational-cli-concrete-shell-actualization comparison**
に置く。

## open questions

- capability source-backed widening actualization の implementation order を `e13` lead staged cut にするか
- capability source-backed widening close 後に duplicate cluster と malformed-static broader reopen をどう並べるか
- public operational CLI concrete shell actualization を repo-level next actual package にするか、capability widening actualization を先に取るか
