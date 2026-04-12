# 346 — current L2 actual-`e3`-authored-row-reopen-ready minimal-actual-`e3`-authored-row threshold

## 目的

`specs/examples/345-current-l2-minimal-deferred-e3-actualization-reopen-ready-actual-e3-authored-row-reopen-comparison.md`
で actual `e3` authored-row reopen の current first choice を fixed した次段として、

- actualized `e3` row の minimum をどこまでに留めるか
- current reached-stage row と guarded formal-hook reading を minimum にどう残すか
- next line を proof / model-check first concrete tool pilot にどう narrow handoff するか

を比較する。

ここで固定するのは
**current L2 actual-`e3`-authored-row-reopen-ready minimal-actual-`e3`-authored-row threshold**
であり、

- new formal-hook family
- proof / model-check first concrete tool pilot の concrete carrier
- second source-sample cluster sequencing
- public surface inventory

はまだ固定しない。

## 比較観点

1. actualized `e3` row と current reached-stage row を minimum に残せるか
2. current theorem-side consumer / formal-hook top guard を minimum に残せるか
3. next line を proof / model-check first concrete tool pilot に narrow handoff できるか

## 比較対象

### 案 1. sample stem と source path だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current reached-stage row と guarded formal-hook reading を repository memory に残しきれない。

### 案 2. `actualization_kind + entry_criteria_refs + actualized_row + reached_stage_refs + guard_refs` を持つ

#### 利点

- `e3` widening package の current cut を lossless に残せる。
- source-authored row と guarded formal-hook reading を同時に minimum に残せる。
- next line の theorem/model-check concrete pilot への handoff が明確になる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. proof/model-check pilot の concrete carrier まで threshold に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- threshold ではなく next reopen を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + actualized_row + reached_stage_refs + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は `e3` actualized row close であり、proof/model-check pilot 自体は next line に分けるべきである。
2. source-authored row だけでなく、`formal hook = not reached (guarded)` を minimum に残す必要がある。
3. current theorem-side consumer / formal-hook top guard を threshold に残すことで、later widening を premature にしない。

## current first choice shape

```text
third_widened_authored_row_actualization = {
  actualization_kind = current_l2_third_widened_authored_row_e3,
  entry_criteria_refs = [
    deferred_e3_actualization_reopen_timing,
    minimal_third_widened_row_e3_guard,
    compare_ready_docs_only_bridge_sketch,
    source_sample_authoring_policy
  ],
  actualized_row = {
    sample_stem = e3_option_admit_chain,
    source_path_ref = samples/current-l2/e3-option-admit-chain.txt,
    source_shape = helper_compatible_inline_admit_chain,
    runner_acceptance = accepted_sample_set_member,
    regression_inventory_status = source_authored
  },
  reached_stage_refs = [
    static_gate_valid,
    interpreter_success,
    formal_hook_not_reached_guarded
  ],
  guard_refs = [
    keep_row_local_proof_notebook_review_unit_consumer,
    keep_runtime_try_cut_cluster_as_current_formal_hook_top,
    keep_new_formal_hook_family_later,
    keep_proof_model_check_first_concrete_tool_pilot_next,
    keep_second_source_sample_cluster_sequencing_after_concrete_tool_pilot
  ]
}
```

## practical reading

current minimal threshold が示すのは、

- `e3-option-admit-chain` は current authored source sample に昇格した
- current source shape は helper-compatible inline-`admit` chain に留める
- reached-stage row は `static gate(valid) -> interpreter(success) -> formal hook not reached (guarded)` である
- next promoted line は proof / model-check first concrete tool pilot に置く

という最小 cut である。

## next promoted line

next promoted line は、
**actual-`e3`-authored-row-reopen-ready proof-model-check-first-concrete-tool-pilot**
に置く。

## open questions

- proof/model-check pilot で first concrete carrier を review-unit reuse に留めるか
- `e3` guarded row を theorem-side example coverage にどこまで使うか
- second source-sample cluster sequencing を theorem/model-check pilot の直後にどこまで source-backed に寄せるか
