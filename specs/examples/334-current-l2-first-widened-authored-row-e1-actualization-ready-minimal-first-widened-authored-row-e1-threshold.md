# 334 — current L2 first-widened-authored-row-e1-actualization-ready minimal-first-widened-authored-row-e1 threshold

## 目的

`specs/examples/333-current-l2-proof-notebook-bridge-sketch-reopen-ordering-ready-first-widened-authored-row-e1-actualization-comparison.md`
で first widened authored row `e1` actualization の current first choice を fixed した次段として、

- actualized `e1` row の minimum をどこまでに留めるか
- current authored inventory と reached-stage row を minimum にどう残すか
- next line `e21` actualization へどう narrow に handoff するか

を比較する。

ここで固定するのは
**current L2 first-widened-authored-row-e1-actualization-ready minimal-first-widened-authored-row-e1 threshold**
であり、

- `e21` / `e3` widening
- theorem-side plain bridge sketch actualization
- compare-bless metadata

はまだ固定しない。

## 比較観点

1. actualized `e1` row と current reached-stage row を minimum に残せるか
2. current lowerer / runner / formal-hook / review-unit guard を minimum で読めるか
3. next line `e21` actualization へ narrow に handoff できるか

## 比較対象

### 案 1. actualized sample stem だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current authored inventory / reached-stage row / guard を repository memory に残しきれない。

### 案 2. `actualization_kind + entry_criteria_refs + actualized_row + reached_stage_refs + guard_refs` を持つ

#### 利点

- `e1` widening package の current cutを lossless に残せる。
- current authored inventory 昇格と reached-stage row を最小で読める。
- next `e21` actualization line への handoff を minimum に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. `e21` / `e3` widening や bridge sketch actualization まで threshold に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- threshold ではなく later line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + actualized_row + reached_stage_refs + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は `e1` actualization close であり、`e21` / `e3` / bridge sketch は next line に分けるべきである。
2. `e1` を authored row に上げたことだけでなく、current reached-stage row と formal-hook family guard を minimum に残す必要がある。
3. later line を threshold に持ち込まなくても、next promoted line を `e21` actualization に置けば handoff は十分明確である。

## current first choice shape

```text
first_widened_authored_row_actualization = {
  actualization_kind = current_l2_first_widened_authored_row_e1,
  entry_criteria_refs = [
    deferred_authored_row_widen_sequence,
    proof_notebook_bridge_sketch_reopen_ordering,
    source_sample_authoring_policy,
    theorem_first_concrete_tool_pilot
  ],
  actualized_row = {
    sample_stem = e1_place_atomic_cut,
    source_path_ref = samples/current-l2/e1-place-atomic-cut.txt,
    source_shape = helper_compatible_single_line_ensure,
    runner_acceptance = accepted_sample_set_member,
    regression_inventory_status = source_authored
  },
  reached_stage_refs = [
    static_gate_valid,
    interpreter_explicit_failure,
    runtime_try_cut_cluster_formal_hook
  ],
  guard_refs = [
    keep_multiline_clause_suite_widening_later,
    keep_e21_as_next_widen_slot,
    keep_e3_as_guarded_third_slot,
    keep_plain_bridge_sketch_after_authored_row_line
  ]
}
```

## practical reading

current minimal threshold が示すのは、

- `e1-place-atomic-cut` は current authored source sample に昇格した
- current source shape は helper-compatible single-line `ensure` に留める
- reached-stage row は `static gate -> interpreter(explicit_failure) -> runtime_try_cut_cluster formal hook`
- next promoted line は `e21` actualization に置く

という最小 cut である。

## next promoted line

next promoted line は、
**first-widened-authored-row-e1-actualization-ready second-widened-authored-row-e21 actualization**
に置く。

## open questions

- `e21` actualization でも `runtime_try_cut_cluster` current top だけで十分か
- `e3` widening 前に theorem-side / formal-hook guard comparison を docs-only package に留めるか
- representative prose multiline mirror を later widening line でどう扱うか
