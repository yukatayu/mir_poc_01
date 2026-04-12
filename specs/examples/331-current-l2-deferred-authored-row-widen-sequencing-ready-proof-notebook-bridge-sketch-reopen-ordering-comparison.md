# 331 — current L2 deferred-authored-row-widen-sequencing-ready proof-notebook-bridge-sketch-reopen-ordering comparison

## 目的

`specs/examples/329-current-l2-theorem-first-concrete-tool-pilot-ready-deferred-authored-row-widen-sequencing-comparison.md`
と
`specs/examples/330-current-l2-deferred-authored-row-widen-sequencing-ready-minimal-deferred-authored-row-widen-sequencing-threshold.md`
で deferred authored-row widen sequencing を fixed した次段として、

- theorem-side bridge sketch を widen line のどこで reopen するか
- plain docs-only bridge sketch と compare-ready bridge sketch をどの順で reopen するか
- concrete theorem / model-check tool binding をどこまで後段に残すか

を比較する。

ここで固定するのは
**current L2 deferred-authored-row-widen-sequencing-ready proof-notebook-bridge-sketch-reopen-ordering comparison**
であり、

- docs-only bridge sketch の actual helper / emitter
- compare-bless metadata
- concrete theorem / model-check tool binding
- first widened row actualization そのもの

はまだ固定しない。

## scope

- entry criteria は `specs/examples/327...330` で fixed 済みの theorem-first review-unit pilot と authored-row widen sequencing に置く。
- current theorem-side cut は row-local `proof_notebook_review_unit` に保つ。
- ordering judgment だけを扱い、actual bridge sketch package は next theorem-side line に残す。

## current 前提

current repo では次が成立している。

1. theorem-side concrete consumer の current cut は tool-neutral formal hook input の row-local `proof_notebook_review_unit` に留まる。
2. deferred authored-row widen sequencing は `e1 -> e21 -> e3` に fixed 済みであり、current widened row line が theorem-side reopen より前に来る。
3. older theorem-line docs-first ladder では、plain docs-only bridge sketch は `specs/examples/140...`、compare-ready bridge sketch は `specs/examples/141...` に対応する。
4. compare-bless metadata と concrete tool binding は still later に残している。

したがって current 問いは、
**authored-row actualization line の後で、plain docs-only bridge sketch を first reopen に置き、compare-ready bridge sketch はその次段に残すのが自然か**
である。

## 比較観点

1. review-unit current cut を壊さないか
2. authored-row widen line と theorem-side reopen line の順序が明確か
3. compare-ready metadata / concrete tool binding を premature に混ぜないか
4. older theorem-line docs-first ladder (`140` -> `141`) と連続的か

## 比較対象

### 案 1. authored-row actualization line の後で plain docs-only bridge sketch を first reopen に置く

#### shape

```text
proof_notebook_bridge_reopen_order = {
  ordering_kind = current_l2_proof_notebook_bridge_reopen_after_authored_rows,
  authored_row_line_ref = phase6_authored_row_actualization_line,
  first_theorem_reopen_ref = phase5_plain_docs_only_bridge_sketch,
  second_theorem_reopen_ref = phase5_compare_ready_bridge_sketch,
  guard_refs = [
    keep_review_unit_as_current_cut,
    keep_compare_bless_metadata_later,
    keep_concrete_tool_binding_later
  ]
}
```

#### 利点

- review-unit current cut と authored-row actualization line の順序を保てる。
- plain bridge sketch (`140`) と compare-ready bridge sketch (`141`) の順序を docs-first ladder のまま維持できる。
- compare-bless metadata や concrete tool binding を later に押し分けやすい。

#### 欠点

- theorem-side actual bridge sketch package は still later に残る。

### 案 2. plain bridge sketch を飛ばし、compare-ready bridge sketch を first reopen に置く

#### 利点

- compare need まで一気に見える。

#### 欠点

- review-unit current cut の直後としては強すぎる。
- `140` を飛ばして `141` を先に採る形になり、older theorem-line ladder と連続しにくい。

### 案 3. authored-row actualization line より前に bridge sketch reopen を置く

#### 利点

- theorem-side docs-only lineを先に整理できる。

#### 欠点

- `e1 -> e21 -> e3` widen line より theorem-side pressure を前倒しする。
- current source-sample path mainline と逆順になる。

## current judgment

current L2 で最も自然なのは、
**案 1. authored-row actualization line の後で plain docs-only bridge sketch を first reopen に置く**
である。

理由は次の通り。

1. current review-unit cut と authored-row widen sequencing を壊さずに theorem-side next line を定義できる。
2. `specs/examples/140` plain bridge sketch と `141` compare-ready bridge sketch の順序をそのまま repo-level next ladder に再利用できる。
3. compare-bless metadata と concrete theorem/model-check binding を still later に残しやすい。

## current first choice details

- current theorem-side first reopen は plain docs-only bridge sketch (`bridge_subject_ref + review_units + bridge_goal_text`) に置く。
- compare-ready bridge sketch (`comparison_basis_refs` 付き) は second theorem-side reopen に残す。
- concrete theorem/model-check tool binding は authored-row lineと bridge-sketch line の後ろへ残す。
- current package では `proof_notebook_review_unit` helper / test / example 自体は widen しない。

## open questions

- plain bridge sketch actualization を docs-only spec lineのまま置くか、non-production helper へ寄せるか
- first widened row `e1` actualization のあとに maintenance を挟むか、そのまま `e21` actualization へ進むか
- `e3` guard comparison と theorem-side plain bridge sketch actualization のどちらを先に置くか
