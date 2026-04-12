# 344 — current L2 deferred `e3` actualization reopen timing-ready minimal-deferred-`e3`-actualization reopen threshold

## 目的

`specs/examples/343-current-l2-minimal-compare-ready-bridge-sketch-ready-deferred-e3-actualization-reopen-timing-comparison.md`
で deferred `e3` actualization reopen timing の current first choice を fixed した次段として、

- timing judgment をどこまで minimum に留めるか
- current guard と next actualization scope を minimum にどう残すか
- concrete tool pilot と second cluster sequencing を later にどう押し分けるか

を比較する。

ここで固定するのは
**current L2 deferred `e3` actualization reopen timing-ready minimal-deferred-`e3`-actualization reopen threshold**
であり、

- actual `e3` source row shape
- new formal-hook family
- concrete theorem / model-check tool pilot
- second source-sample cluster sequencing

はまだ固定しない。

## 比較観点

1. timing judgment と current guard を minimum に残せるか
2. next package の actual `e3` authored-row reopen scopeを minimum に残せるか
3. concrete tool pilot と second cluster sequencing を later に押し分けられるか

## 比較対象

### 案 1. reopen order だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current formal-hook / theorem-side guard と next actualization scope が弱い。

### 案 2. `timing_kind + entry_criteria_refs + current_guard_refs + next_actualization_refs + kept_later_refs` を持つ

#### 利点

- compare-ready bridge fixed 後の current guard と next actualization scope を lossless に残せる。
- package 3 で何を widen し、何を still later に残すかを minimum に書ける。

#### 欠点

- 案 1 より fields は増える。

### 案 3. actual `e3` source row / ladder shape まで minimum に含める

#### 利点

- 次段との接続は見えやすい。

#### 欠点

- threshold ではなく next actualization line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `timing_kind + entry_criteria_refs + current_guard_refs + next_actualization_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package は timing judgment close であり、actual `e3` source row shape 自体は next package に残すべきである。
2. current theorem-side / formal-hook guard を minimum に残さないと、`e3` actualization を concrete tool pilot より先に reopen する理由が弱い。
3. package 3 の scope と later line を narrow に押し分けられる。

## current first choice shape

```text
deferred_e3_actualization_reopen_timing = {
  timing_kind = current_l2_e3_actualization_before_concrete_tool_pilot,
  entry_criteria_refs = [
    compare_ready_docs_only_bridge_sketch,
    proof_notebook_review_unit,
    minimal_third_widened_row_e3_guard
  ],
  current_guard_refs = [
    preserve_row_local_theorem_consumer,
    preserve_runtime_try_cut_cluster_formal_hook_top,
    keep_e3_outside_current_formal_hook_family
  ],
  next_actualization_refs = [
    actual_e3_source_row,
    runner_accepted_set_widen,
    authored_inventory_widen,
    regression_ladder_widen
  ],
  kept_later_refs = [
    new_formal_hook_family,
    proof_model_check_first_concrete_tool_pilot,
    second_source_sample_cluster_sequencing
  ]
}
```

## practical reading

current minimal deferred `e3` actualization reopen timing が示すのは、

- compare-ready bridge sketch fixed 後の次段で `e3` actualization を reopen してよい
- ただし current theorem-side consumer と current formal-hook top はそのまま保つ
- package 3 では source row / runner / authored inventory / regression ladder を主対象にする
- concrete tool pilot と second cluster sequencing は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-deferred-`e3`-actualization-reopen-ready actual-`e3`-authored-row-reopen**
に置く。

## open questions

- package 3 で `e3` formal-hook reached row を still guarded に保つ minimum wording をどこに置くか
- regression helper の inventory / command bundle に `e3` authored row をどう narrow に入れるか
- concrete tool pilot を `e3` actualization の後段でどこまで source-sample pressure に寄せるか
