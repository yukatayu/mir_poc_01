# 398 — current L2 stable-malformed-capability-second-reopen-actualization-ready minimal-stable-malformed-capability-second-reopen threshold

## 目的

`specs/examples/397-current-l2-final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready-stable-malformed-capability-second-reopen-actualization-comparison.md`
で stable malformed capability second reopen actualization comparison の current first choice を fixed した次段として、

- current comparison result の minimum をどこまでに留めるか
- chosen family / entry evidence / actualization mode / guard / kept-later refs をどこまで threshold に残すか
- next line を public operational CLI concrete shell naming comparison へどう handoff するか

を比較する。

ここで固定するのは
**current L2 stable-malformed-capability-second-reopen-actualization-ready minimal-stable-malformed-capability-second-reopen threshold**
であり、

- actual source-backed widening 実装
- duplicate cluster の actual reopen
- `TryFallback` / `AtomicCut` malformed-static broader reopen
- public operational CLI actual shell

はまだ固定しない。

## 比較観点

1. chosen family と entry evidence を lossless に残せるか
2. actualization mode と staging note を明示できるか
3. missing-option first / capability second ordering と kept-later guard を同時に残せるか

## 比較対象

### 案 1. `actualization_kind + entry_criteria_refs + chosen_family_refs + entry_evidence_refs + actualization_mode_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- current pair judgment と helper/source/parser evidence を lossless に残せる。
- next malformed-side widening point と repo-level next package を同時に handoff しやすい。

#### 欠点

- fields はやや多い。

### 案 2. `chosen_family_refs + actualization_mode_refs` だけを残す

#### 利点

- 軽い。

#### 欠点

- helper-local compare、stage1 reconnect、guard が落ちやすい。

### 案 3. `e13` lead staging note までを threshold から落とす

#### 利点

- minimum はさらに軽い。

#### 欠点

- pair judgment を保ったまま staged implementation cut を取れる current reading が見えにくくなる。

## current judgment

current L2 で最も自然なのは、
**案 1. `actualization_kind + entry_criteria_refs + chosen_family_refs + entry_evidence_refs + actualization_mode_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は capability family second reopen の cut を narrow に整理することであり、family / evidence / guard を同時に残す必要がある。
2. helper-local capability compare と stage1 reconnect widen を entry evidence から落とすと、why now の説明が弱くなる。
3. next package は public operational CLI concrete shell naming comparison へ進むが、capability malformed-side next reopen point 自体は source-backed widening actualization に置くため、その両方を threshold 側に残す方が自然である。

## current first choice shape

```text
stable_malformed_capability_second_reopen = {
  actualization_kind = current_l2_capability_source_backed_second_reopen,
  entry_criteria_refs = [
    stable_malformed_broader_followup_inventory,
    stable_malformed_missing_option_first_source_backed_widening_actualization,
    final_public_parser_checker_runtime_thin_facade_later_support_actualization
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
  actualization_mode_refs = [
    source_backed_widening_first,
    e13_lead_allowed_only_as_staging_note
  ],
  guard_refs = [
    keep_missing_option_first_capability_second_ordering,
    keep_duplicate_cluster_later,
    keep_try_rollback_malformed_static_later,
    do_not_shrink_family_to_singleton
  ],
  kept_later_refs = [
    public_operational_cli_concrete_shell_naming_comparison,
    stable_malformed_capability_second_source_backed_widening_actualization_comparison,
    duplicate_cluster_later_reopen,
    try_rollback_malformed_static_broader_reopen
  ]
}
```

## practical reading

current minimal stable malformed capability second reopen が示すのは、

- capability second reopen は `e13/e20` pair judgment を維持する
- helper-local capability compare、stage1 reconnect widen、fixture-static rows を entry evidence に再利用する
- next malformed-side actualization mode は source-backed widening first に置く
- `e13` lead は implementation staging note に留め、family judgment 自体は縮めない
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は still later に残す
- repo-level next package は public operational CLI concrete shell naming comparison に進めてよい

という minimum である。

## next promoted line

next promoted line は、
**stable-malformed-capability-second-reopen-actualization-ready public-operational-cli-concrete-shell-naming comparison**
に置く。

## open questions

- capability second source-backed widening actualization を pair 一括か `e13` lead staged cut かのどちらで閉じるか
- duplicate cluster と malformed-static broader line を capability actualization の後にどう reopen するか
- public operational CLI concrete shell naming close 後に、capability malformed-side actualization と CLI actual shell のどちらを先に進めるか
