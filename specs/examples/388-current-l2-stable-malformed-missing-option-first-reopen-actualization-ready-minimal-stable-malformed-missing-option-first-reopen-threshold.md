# 388 — current L2 stable-malformed-missing-option-first-reopen-actualization-ready minimal-stable-malformed-missing-option-first-reopen threshold

## 目的

`specs/examples/387-current-l2-docs-first-io-host-facing-port-boundary-ready-stable-malformed-missing-option-first-reopen-actualization-comparison.md`
で missing-option first reopen actualization の current first choice を fixed した次段として、

- current actualization cut の minimum をどこまでに留めるか
- triplet family、entry evidence、staging note、guard、kept-later refs をどこまで threshold に残すか
- next line を final public parser / checker / runtime first later gate actualization comparison へどう handoff するか

を比較する。

ここで固定するのは
**current L2 stable-malformed-missing-option-first-reopen-actualization-ready minimal-stable-malformed-missing-option-first-reopen threshold**
であり、

- actual source-backed widening code
- capability second actualization
- duplicate / try-rollback malformed-static later gate の具体形

はまだ固定しない。

## 比較観点

1. family judgment と actualization mode を lossless に minimum へ残せるか
2. helper-local entry evidence と source-backed widening first judgment を同時に持てるか
3. `e16` lead staging note を threshold に残しつつ family shrink を避けられるか
4. next line と kept-later family を clean に handoff できるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_reopen_family_refs + chosen_row_refs + actualization_mode_refs + staging_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- triplet family judgment、helper-local entry evidence、source-backed widening first を一体で残せる。
- capability second、duplicate later、try-rollback malformed-static later の guard を lossless に保持できる。
- final public parser/checker/runtime first later gate actualization comparison と malformed-side reserve line を同時に handoff できる。

#### 欠点

- fields はやや多い。

### 案 2. `source-backed widening first` という prose summary だけを残す

#### 利点

- 軽い。

#### 欠点

- triplet family、helper-local entry evidence、staging note が抜けやすい。
- capability second / duplicate later / try-rollback later の guard が薄くなる。

### 案 3. `e16` lead だけを threshold に残す

#### 利点

- implementation cut は分かりやすい。

#### 欠点

- family judgment を unnecessary に縮める。
- `e17/e18` を later family と誤読しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_reopen_family_refs + chosen_row_refs + actualization_mode_refs + staging_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は family judgment と actualization mode の整理であり、triplet family・entry evidence・guard を lossless に残す必要がある。
2. `e16` lead は implementation note であって family shrink ではないため、staging refs として separate に持つ方が自然である。
3. next line を public-side actualization comparison に handoff しつつ、Macro 4 側の widening reserve line を kept-later refs に残せる。

## current first choice shape

```text
stable_malformed_missing_option_first_reopen = {
  actualization_kind = current_l2_missing_option_source_backed_first_reopen,
  entry_criteria_refs = [
    stable_malformed_broader_followup_inventory,
    helper_local_missing_option_compare,
    current_source_sample_runner_and_ladder
  ],
  chosen_reopen_family_refs = [
    missing_option_structure_floor
  ],
  chosen_row_refs = [
    e16_malformed_missing_chain_head_option,
    e17_malformed_missing_predecessor_option,
    e18_malformed_missing_successor_option
  ],
  actualization_mode_refs = [
    reuse_helper_local_compare_as_entry_evidence,
    prefer_source_backed_widening_first
  ],
  staging_refs = [
    allow_e16_lead_implementation_cut,
    keep_capability_second_for_separate_package
  ],
  guard_refs = [
    do_not_mix_duplicate_cluster,
    do_not_mix_try_rollback_malformed_static_family,
    do_not_shrink_triplet_family_to_single_row
  ],
  kept_later_refs = [
    final_public_parser_checker_runtime_first_later_gate_actualization,
    stable_malformed_capability_second_reopen_actualization,
    duplicate_cluster_later_gate,
    try_rollback_malformed_static_family_later_gate
  ]
}
```

## practical reading

current minimal stable malformed missing-option first reopen actualization が示すのは、

- helper-local missing-option compare は entry evidence として再利用する
- first reopen family は `e16/e17/e18` triplet に置く
- current next actualization mode は source-backed widening first に置く
- `e16` lead は implementation staging note に留め、family judgment は縮めない
- capability second、duplicate later、try-rollback malformed-static later を維持する

という最小 cut である。

## next promoted line

next promoted line は、
**stable-malformed-missing-option-first-reopen-actualization-ready final-public-parser-checker-runtime-first-later-gate-actualization comparison**
に置く。

## open questions

- source-backed widening 実装を triplet 一括で行うか、`e16` lead から始めるか
- capability second reopen をどこで reopen するか
- duplicate cluster later gate を malformed widening line と別に保ち続けるか
