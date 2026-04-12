# 370 — current L2 stable-malformed-broader-follow-up-inventory-ready minimal-stable-malformed-broader-follow-up-inventory threshold

## 目的

`specs/examples/369-current-l2-model-check-concrete-carrier-first-actualization-gate-ready-stable-malformed-broader-follow-up-inventory-comparison.md`
で broader malformed follow-up inventory の current first choice を fixed した次段として、

- missing-option first / capability second の sequencing minimum をどこまでに留めるか
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family をどこまで kept-later として明示するか
- repo-level next line と Macro 4 next reopen point をどう両立して handoff するか

を比較する。

ここで固定するのは
**current L2 stable-malformed-broader-follow-up-inventory-ready minimal-stable-malformed-broader-follow-up-inventory threshold**
であり、

- missing-option first reopen の具体形
- capability second reopen の具体形
- final public contract / CLI の具体形

はまだ固定しない。

## 比較観点

1. sequencing close を minimum に留められるか
2. first / second reopen family と kept-later family を lossless に残せるか
3. repo-level next line と Macro 4 next reopen point を両方 handoff できるか

## 比較対象

### 案 1. family 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- row family、guard、kept-later family が抜けやすい。
- repo-level next line と Macro 4 next reopen point の両立が弱い。

### 案 2. `inventory_kind + entry_criteria_refs + selected_first_reopen_family_refs + selected_first_reopen_row_refs + selected_second_reopen_family_refs + selected_second_reopen_row_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- missing-option first / capability second の順序を lossless に残せる。
- duplicate cluster と try/rollback malformed-static family を kept-later として明示できる。
- public operational later gate と Macro 4 next malformed reopen point の両方へ handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. missing-option first reopen actualization の具体形まで threshold に含める

#### 利点

- 次段は見えやすい。

#### 欠点

- threshold ではなく later actualization package を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `inventory_kind + entry_criteria_refs + selected_first_reopen_family_refs + selected_first_reopen_row_refs + selected_second_reopen_family_refs + selected_second_reopen_row_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は sequencing / inventory close であり、actual widening は次段へ残すべきである。
2. missing-option first / capability second の distinction を threshold 側で残さないと、broader follow-up inventory fixed の意味が薄い。
3. duplicate cluster、try/rollback malformed-static family、repo-level next line を一緒に handoff するには guard / kept-later が必要である。

## current first choice shape

```text
stable_malformed_broader_followup = {
  inventory_kind = current_l2_stable_malformed_followup_order,
  entry_criteria_refs = [
    stable_static_post_contrast_sequencing,
    stable_static_edge_pair_first_reopen,
    current_checker_spike_stable_cluster_baseline
  ],
  selected_first_reopen_family_refs = [
    missing_option_structure_floor
  ],
  selected_first_reopen_row_refs = [
    e16_malformed_missing_chain_head_option,
    e17_malformed_missing_predecessor_option,
    e18_malformed_missing_successor_option
  ],
  selected_second_reopen_family_refs = [
    capability_strengthening_floor
  ],
  selected_second_reopen_row_refs = [
    e13_malformed_capability_strengthening,
    e20_malformed_late_capability_strengthening
  ],
  guard_refs = [
    keep_duplicate_cluster_outside_stable_inventory,
    keep_try_rollback_malformed_static_family_later,
    keep_actual_source_backed_widening_for_separate_package
  ],
  kept_later_refs = [
    public_operational_cli_final_public_contract_later_gate,
    stable_malformed_missing_option_first_reopen_actualization,
    stable_malformed_capability_second_reopen_actualization,
    duplicate_cluster_later_gate,
    try_rollback_malformed_static_family_later_gate
  ]
}
```

## practical reading

current minimal stable malformed broader follow-up inventory が示すのは、

- stable malformed next reopen order は missing-option first / capability second に置く
- duplicate cluster は stable inventory の外へ残す
- `TryFallback` / `AtomicCut` malformed-static family は still later に残す
- repo-level next line は public operational CLI / final public contract later gate に進める

という最小 cut である。

## next promoted line

next promoted line は、
**stable-malformed-broader-follow-up-inventory-ready public-operational-cli-final-public-contract-later-gate comparison**
に置く。

## open questions

- missing-option first reopen actualization を helper-local compare と source-backed wideningのどちらから開くか
- capability second reopen を same track で直列に開くか
- duplicate cluster later gate を stable malformed widening と別 macro line に置くか
