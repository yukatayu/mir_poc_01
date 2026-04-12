# 332 — current L2 proof-notebook-bridge-sketch-reopen-ordering-ready minimal-proof-notebook-bridge-sketch-reopen-ordering threshold

## 目的

`specs/examples/331-current-l2-deferred-authored-row-widen-sequencing-ready-proof-notebook-bridge-sketch-reopen-ordering-comparison.md`
で proof-notebook bridge-sketch reopen ordering の current first choice を fixed した次段として、

- ordering minimum をどこまでに留めるか
- theorem-side first reopen / second reopen と guard を minimum にどう残すか
- next maintenance / widened-row actualization line への handoff を minimum にどう反映するか

を比較する。

ここで固定するのは
**current L2 proof-notebook-bridge-sketch-reopen-ordering-ready minimal-proof-notebook-bridge-sketch-reopen-ordering threshold**
であり、

- actual bridge sketch helper / emitter
- compare-bless metadata
- concrete theorem / model-check tool binding
- widened row actualization 自体

はまだ固定しない。

## 比較観点

1. first theorem-side reopen と second theorem-side reopen を minimum に残せるか
2. review-unit current cut と authored-row actualization line を guard に残せるか
3. next maintenance / widened-row line へ narrow に handoff できるか

## 比較対象

### 案 1. first reopen ref だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- compare-ready second reopen と current guard が弱い。

### 案 2. `ordering_kind + authored_row_line_ref + first_theorem_reopen_ref + second_theorem_reopen_ref + guard_refs` を持つ

#### 利点

- plain bridge sketch first / compare-ready second の order を lossless に残せる。
- review-unit current cut と authored-row line の guard を minimum に残せる。
- actual bridge sketch package と concrete tool binding を still later に押し分けられる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. compare-bless metadata や concrete tool binding まで minimum に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- threshold ではなく later reopen line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `ordering_kind + authored_row_line_ref + first_theorem_reopen_ref + second_theorem_reopen_ref + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は ordering close であり、actual bridge sketch や compare metadata は next package に分けるべきである。
2. plain bridge sketch first / compare-ready second の order を minimum に残しておかないと、later theorem-side ladder が弱い。
3. authored-row actualization line を guard に残すことで、theorem-side reopen が source-sample lineを前倒ししない cut を repository memory に残せる。

## current first choice shape

```text
proof_notebook_bridge_reopen_order = {
  ordering_kind = current_l2_proof_notebook_bridge_reopen_after_authored_rows,
  authored_row_line_ref = phase6_authored_row_actualization_line,
  first_theorem_reopen_ref = phase5_plain_docs_only_bridge_sketch,
  second_theorem_reopen_ref = phase5_compare_ready_bridge_sketch,
  guard_refs = [
    keep_review_unit_as_current_cut,
    keep_compare_bless_metadata_later,
    keep_concrete_tool_binding_later,
    keep_maintenance_before_next_actualization
  ]
}
```

## practical reading

current minimal proof-notebook bridge-sketch reopen ordering が示すのは、

- theorem-side first reopen は plain docs-only bridge sketch に置く
- compare-ready bridge sketch は second reopen に残す
- authored-row actualization line を先に閉じる
- current next line は maintenance、その後に first widened row `e1` actualization に戻る

という最小 cut である。

## next promoted line

next promoted line は、
**proof-notebook-bridge-sketch-reopen-ordering-ready mirror-sweep-follow-up-maintenance**
に置く。

## open questions

- plain docs-only bridge sketch actualization を spec-only に留めるか
- bridge sketch actualization を `e1` / `e21` widen line のどこで reopen するか
- compare-ready bridge sketch と `e3` theorem-side guard comparison の先後関係をどう置くか
