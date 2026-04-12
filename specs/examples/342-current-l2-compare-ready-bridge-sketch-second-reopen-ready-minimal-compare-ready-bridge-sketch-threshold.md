# 342 — current L2 compare-ready bridge-sketch second reopen-ready minimal-compare-ready bridge-sketch threshold

## 目的

`specs/examples/341-current-l2-minimal-plain-proof-notebook-bridge-sketch-ready-compare-ready-bridge-sketch-second-reopen-comparison.md`
で compare-ready bridge sketch second reopen の current first choice を fixed した次段として、

- compare-ready actual shape をどこまで minimum に留めるか
- current entry criteria と kept-later line を minimum にどう残すか
- deferred `e3` actualization reopen timing への handoff を narrow にどう残すか

を比較する。

ここで固定するのは
**current L2 compare-ready bridge-sketch second reopen-ready minimal-compare-ready bridge-sketch threshold**
であり、

- bless decision state
- review-session metadata
- helper / emitter code anchor
- concrete theorem / model-check tool binding

はまだ固定しない。

## 比較観点

1. compare-ready bridge sketch を minimum に保てるか
2. current theorem-side entry criteria と deferred `e3` timing handoff を minimum に残せるか
3. bless / session / helper / tool binding line を later に押し分けられるか

## 比較対象

### 案 1. compare-ready bridge sketch shape だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- entry criteria と kept-later line が弱い。

### 案 2. `actualization_kind + entry_criteria_refs + bridge_sketch_shape + kept_later_refs` を持つ

#### 利点

- compare-ready docs-only actualization の cut と later reopen guard を lossless に残せる。
- current theorem-side entry criteria と deferred `e3` timing handoff を minimum に残せる。
- bless / session / helper / tool binding を later に押し分けやすい。

#### 欠点

- 案 1 より fields は増える。

### 案 3. bless / review-session metadata まで minimum に含める

#### 利点

- 次段との接続は見えやすい。

#### 欠点

- threshold ではなく later reopen line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + bridge_sketch_shape + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package は compare-ready bridge sketch second reopen close であり、bless/session metadata は still later に残すべきである。
2. plain bridge sketch actualization と deferred `e3` actualization reopen timing を entry / handoff として minimum に残せる。
3. helper / emitter / concrete tool binding を still later に保てる。

## current first choice shape

```text
compare_ready_proof_notebook_bridge_actualization = {
  actualization_kind = current_l2_compare_ready_docs_only_bridge_sketch,
  entry_criteria_refs = [
    proof_notebook_review_unit,
    plain_docs_only_bridge_sketch,
    minimal_third_widened_row_e3_guard
  ],
  bridge_sketch_shape = {
    bridge_subject_ref,
    review_units,
    bridge_goal_text,
    comparison_basis_refs
  },
  kept_later_refs = [
    bless_decision_state,
    review_session_metadata,
    helper_emitter_code_anchor,
    concrete_tool_binding,
    deferred_e3_actualization_reopen_timing
  ]
}
```

## practical reading

current minimal compare-ready bridge sketch second reopen が示すのは、

- current theorem-side second actualization は docs-only compare-ready bridge sketch に留める
- compare-ready shape は `bridge_subject_ref + review_units + bridge_goal_text + comparison_basis_refs` に留める
- bless / review-session / helper / tool binding は still later に残す
- next line は deferred `e3` actualization reopen timing に置く

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-compare-ready-bridge-sketch-ready deferred-e3-actualization-reopen-timing**
に置く。

## open questions

- deferred `e3` actualization reopen timing を compare-ready bridge sketch の直後に置く current cutをどこまで minimum に残すか
- later bless/session metadata line を current compare-ready bridge shape にどう重ねるか
- helper / emitter code anchor を compare-ready bridge lineに still 入れない current cutを保つか
