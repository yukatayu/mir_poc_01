# 392 — current L2 stable-malformed-missing-option-first-source-backed-widening-actualization-ready minimal-stable-malformed-missing-option-first-source-backed-widening threshold

## 目的

`specs/examples/391-current-l2-final-public-parser-checker-runtime-first-later-gate-actualization-ready-stable-malformed-missing-option-first-source-backed-widening-actualization-comparison.md`
で missing-option family の source-backed widening first cut を fixed した次段として、

- current actualization cut の minimum をどこまでに留めるか
- actualized rows / staged guard / actualized surfaces / kept-later refs をどこまで threshold に残すか
- next line を public operational CLI second later gate へどう handoff するか

を比較する。

ここで固定するのは
**current L2 stable-malformed-missing-option-first-source-backed-widening-actualization-ready minimal-stable-malformed-missing-option-first-source-backed-widening threshold**
であり、

- `e17` source-authored actualization
- capability second reopen
- public operational CLI actual implementation
- final public parser/checker/runtime thin-facade later support

はまだ固定しない。

## 比較観点

1. actualized rows と staged same-family guard を lossless に minimum へ残せるか
2. sample / runner / ladder / regression helper へ widen した surface を明示できるか
3. public-side later line と malformed-side later line を同時に kept-later へ残せるか
4. triplet family judgment を single-row actualization と誤読させないか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + actualized_row_refs + staged_guard_refs + actualized_surface_refs + reached_stage_refs + kept_later_refs` を持つ

#### 利点

- `e16/e18` actualized rows と `e17` staged guard を同時に lossless に残せる。
- source sample / runner / ladder / regression helper widening を明示できる。
- public operational CLI second gate と thin-facade later support、capability second reopen を同時に kept-later に残せる。

#### 欠点

- fields はやや多い。

### 案 2. `e16/e18 を authored sample に加えた` という prose summary だけを残す

#### 利点

- 軽い。

#### 欠点

- `e17` staged guard と actualized surface が抜けやすい。
- next line への handoff が弱くなる。

### 案 3. `missing-option triplet widened` とだけ書く

#### 利点

- family judgment は見える。

#### 欠点

- 実際には `e17` は staged guard なので、actualized row と staged row の差が消える。
- runner / ladder / regression helper widening の証拠が薄くなる。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + actualized_row_refs + staged_guard_refs + actualized_surface_refs + reached_stage_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は `e16/e18` actualization と `e17` staged guard の両立であり、row split を lossless に残す必要がある。
2. current widening は sample file 追加だけでなく runner / ladder / regression helper を含むため、actualized surface refs を落とすと drift しやすい。
3. public operational CLI second gate と thin-facade later support が次段に来るため、kept-later refs を threshold に残した方が current roadmap を保ちやすい。

## current first choice shape

```text
stable_malformed_missing_option_source_backed_widening = {
  actualization_kind = current_l2_missing_option_source_backed_static_stop_pair,
  entry_criteria_refs = [
    stable_malformed_missing_option_first_reopen,
    helper_local_missing_option_compare,
    final_public_parser_checker_runtime_first_later_gate
  ],
  actualized_row_refs = [
    e16_malformed_missing_chain_head_option,
    e18_malformed_missing_successor_option
  ],
  staged_guard_refs = [
    e17_malformed_missing_predecessor_option,
    keep_triplet_family_judgment
  ],
  actualized_surface_refs = [
    samples_current_l2_source_files,
    current_l2_runner_accepted_sample_set,
    current_l2_source_lowering_tests,
    current_l2_source_sample_runner_tests,
    current_l2_source_sample_verification_ladder_tests,
    current_l2_source_sample_regression_inventory_and_bundle
  ],
  reached_stage_refs = [
    static_gate_reached_malformed,
    interpreter_not_reached_static_stop,
    formal_hook_reached_fixture_static_cluster
  ],
  kept_later_refs = [
    public_operational_cli_second_later_gate_actualization_comparison,
    final_public_parser_checker_runtime_thin_facade_later_support_actualization,
    stable_malformed_capability_second_reopen,
    duplicate_cluster_later_gate,
    try_rollback_malformed_static_family_later_gate
  ]
}
```

## practical reading

current minimal stable malformed missing-option first source-backed widening が示すのは、

- helper-local compare を entry evidence に保ったまま、`e16` と `e18` を source-authored static-stop pair に widen する
- current authored corpus / runner accepted set / ladder / regression helper を `e16/e18` まで widen する
- `e17` は same-family staged guard に留め、triplet family judgment は縮めない
- capability second、duplicate later、`TryFallback` / `AtomicCut` malformed-static later を維持する
- next line は public operational CLI second later gate に送る

という最小 cut である。

## next promoted line

next promoted line は、
**stable-malformed-missing-option-first-source-backed-widening-actualization-ready public-operational-cli-second-later-gate-actualization comparison**
に置く。

## open questions

- `e17` staged guard を source-authored row に上げる最小 bridge cut
- capability second reopen の sequencing
- thin-facade later support と public operational CLI second gate の close 順
