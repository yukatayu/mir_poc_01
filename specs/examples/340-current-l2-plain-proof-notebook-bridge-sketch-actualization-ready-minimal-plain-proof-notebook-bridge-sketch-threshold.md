# 340 — current L2 plain proof-notebook bridge-sketch actualization-ready minimal-plain-proof-notebook-bridge-sketch threshold

## 目的

`specs/examples/339-current-l2-minimal-third-widened-row-e3-guard-ready-plain-proof-notebook-bridge-sketch-actualization-comparison.md`
で plain proof-notebook bridge sketch actualization の current first choice を fixed した次段として、

- minimum actual shape をどこまでに留めるか
- docs-only actualization と later compare-ready reopen をどう分離するか
- next line への handoff を minimum にどう残すか

を比較する。

ここで固定するのは
**current L2 plain proof-notebook bridge-sketch actualization-ready minimal-plain-proof-notebook-bridge-sketch threshold**
であり、

- compare-ready metadata
- bless / review-session metadata
- helper / emitter code anchor
- concrete theorem / model-check tool binding

はまだ固定しない。

## 比較観点

1. plain docs-only bridge sketch shape を minimum に残せるか
2. row-local review-unit current cut と compare-ready second reopen を guard に残せるか
3. next line へ narrow に handoff できるか

## 比較対象

### 案 1. bridge sketch shape だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- entry criteria と kept-later line が弱い。

### 案 2. `actualization_kind + entry_criteria_refs + bridge_sketch_shape + kept_later_refs` を持つ

#### 利点

- docs-only actualization の cut と later compare-ready reopen の guard を lossless に残せる。
- current theorem-side consumer と `e3` guard comparison を entry criteria に残せる。
- helper / emitter / concrete tool binding を later に押し分けやすい。

#### 欠点

- 案 1 より fields は増える。

### 案 3. compare-ready metadata まで minimum に含める

#### 利点

- 次段との接続は見えやすい。

#### 欠点

- threshold ではなく second reopen line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + bridge_sketch_shape + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package は plain bridge sketch actualization close であり、compare-ready metadata は second reopen に残すべきである。
2. `specs/examples/140` の shape を current line に持ち直す cut と later reopen line を両方 minimum に残せる。
3. helper / emitter / concrete tool binding を still later に保てる。

## current first choice shape

```text
plain_proof_notebook_bridge_actualization = {
  actualization_kind = current_l2_plain_docs_only_bridge_sketch,
  entry_criteria_refs = [
    proof_notebook_review_unit,
    plain_bridge_first_compare_ready_second,
    minimal_third_widened_row_e3_guard
  ],
  bridge_sketch_shape = {
    bridge_subject_ref,
    review_units,
    bridge_goal_text
  },
  kept_later_refs = [
    comparison_basis_refs,
    bless_decision_state,
    review_session_metadata,
    helper_emitter_code_anchor,
    concrete_tool_binding,
    deferred_e3_actualization_reopen_timing
  ]
}
```

## practical reading

current minimal plain proof-notebook bridge sketch actualization が示すのは、

- current theorem-side first actualization は docs-only bridge sketch に留める
- bridge sketch shape は `bridge_subject_ref + review_units + bridge_goal_text` に留める
- compare-ready bridge sketch second reopen を next line に置く
- helper / emitter と concrete tool binding は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-plain-proof-notebook-bridge-sketch-ready compare-ready-bridge-sketch-second-reopen**
に置く。

## open questions

- compare-ready bridge sketch second reopen の threshold に `comparison_basis_refs` 以外をどこまで入れるか
- deferred `e3` actualization reopen timing を compare-ready bridge sketch の前後どちらに置くか
- later helper / emitter code anchor を plain bridge sketch lineへ戻す必要があるか
