# 337 — current L2 second-widened-authored-row-e21-actualization-ready third-widened-row-e3 theorem-side formal-hook guard comparison

## 目的

`specs/examples/335-current-l2-first-widened-authored-row-e1-actualization-ready-second-widened-authored-row-e21-actualization-comparison.md`
と
`specs/examples/336-current-l2-second-widened-authored-row-e21-actualization-ready-minimal-second-widened-authored-row-e21-threshold.md`
で second widened authored row `e21` actualization を fixed した次段として、

- deferred third row `e3-option-admit-chain` をどの位置で widen するか
- その前に theorem-side / formal-hook family で何を guard comparison に残すか
- plain bridge sketch reopen line より前に `admit` family widening を既成事実化しない cut をどう残すか

を比較する。

ここで固定するのは
**current L2 second-widened-authored-row-e21-actualization-ready third-widened-row-e3 theorem-side formal-hook guard comparison**
であり、

- `e3` source row actualization
- runner accepted set widening
- `runtime_try_cut_cluster` 以外の new formal-hook family
- compare-ready bridge sketch
- concrete theorem / model-check tool binding

はまだ固定しない。

## scope

- entry criteria は `specs/examples/329...336` で fixed 済みの widen sequencing、bridge-sketch reopen ordering、`e1` / `e21` actualization に置く。
- current theorem-side cut は row-local `proof_notebook_review_unit` に保つ。
- current formal-hook top は `runtime_try_cut_cluster` / `fixture_static_cluster` のまま保つ。
- `e3` を actual authored row に入れる前の guard comparison だけを扱う。

## current 前提

current repo では次が成立している。

1. authored row として current reached-stage inventory に入っているのは `e1` / `e2` / `e21` / `e4` / `e23` の quintet である。
2. `e3-option-admit-chain` は representative / fixture / source target path を持つが、current authored row には入っていない。
3. widened authored-row sequencing は `e1 -> e21 -> e3` に fixed 済みである。
4. theorem-side bridge-sketch reopen ordering は plain docs-only bridge sketch first / compare-ready bridge sketch second に fixed 済みである。
5. theorem-side concrete consumer の current cut は tool-neutral formal hook artifact を入力にする row-local `proof_notebook_review_unit` に留まる。

したがって current 問いは、
**`e3` をいま runtime authored row に昇格させるのではなく、admit-family / theorem-side / formal-hook の guard comparison を 1 package 挟んで current cut を固定する方が自然か**
である。

## 比較観点

1. current authored quintet と `runtime_try_cut_cluster` top を壊さないか
2. `e3` の admit-family pressure を theorem-side line へ silent に押し込まないか
3. plain bridge sketch first / compare-ready bridge sketch second の reopen ordering と整合するか
4. later `e3` actualization や concrete tool binding を premature に混ぜないか

## 比較対象

### 案 1. `e3` actualization 前に theorem-side / formal-hook guard comparison を 1 package 挟む

#### shape

```text
third_widened_row_guard = {
  guard_kind = current_l2_e3_theorem_side_formal_hook_guard_compare,
  deferred_row_ref = source_sample:e3-option-admit-chain,
  current_formal_hook_top_ref = runtime_try_cut_cluster,
  current_theorem_consumer_ref = proof_notebook_review_unit,
  bridge_reopen_order_ref = plain_bridge_first_compare_ready_second,
  kept_later_refs = [
    actual_e3_source_row,
    admit_family_formal_hook_widen,
    compare_ready_bridge_sketch,
    concrete_tool_binding
  ]
}
```

#### 利点

- current authored quintet と reached-stage ladder を壊さずに `e3` の pressure だけを切り出せる。
- `admit` family widening を `runtime_try_cut_cluster` に silent に吸収しなくてよい。
- theorem-side plain bridge sketch actualization を next line に置きやすい。

#### 欠点

- `e3` 自体の actual source-row widen は still later に残る。

### 案 2. `e3` を current runtime row としてすぐ actualize する

#### 利点

- authored row inventory が増える。

#### 欠点

- current `runtime_try_cut_cluster` と theorem-side row-local review-unit cut に `admit` family pressure をそのまま押し込む。
- bridge sketch reopen line より先に `e3` widen を進める形になり、ordering fixed と逆流しやすい。

### 案 3. `e3` 用の new formal-hook / theorem-side family を先に widen してから actualize する

#### 利点

- `admit` family を専用 carrier で扱える可能性はある。

#### 欠点

- plain bridge sketch / compare-ready bridge sketch / concrete tool binding を横断して reopen しやすい。
- current phase の docs-first theorem-side lineとしては強すぎる。

## current judgment

current L2 で最も自然なのは、
**案 1. `e3` actualization 前に theorem-side / formal-hook guard comparison を 1 package 挟む**
である。

理由は次の通り。

1. current authored quintet と `runtime_try_cut_cluster` reached row をそのまま維持できる。
2. `e3` は `admit` family pressure を持つため、current theorem-side / formal-hook cut を widening する前に guard comparison を明示しておく方が staged line に合う。
3. bridge-sketch reopen ordering で fixed した plain-first / compare-ready-second line を壊さず、next line を theorem-side plain bridge sketch actualization に進められる。

## current first choice details

- `e3-option-admit-chain` は current authored row にまだ入れない。
- current runner accepted set / regression bundle / README ladder は quintet のまま保つ。
- `e3` actualization は theorem-side plain bridge sketch actualization と compare-ready bridge sketch second reopen の後段 reopen に残す。
- concrete theorem / model-check tool binding は `e3` guard comparison に混ぜない。

## next promoted line

next promoted line は、
**third-widened-row-e3-theorem-side-formal-hook-guard-comparison-ready plain-proof-notebook-bridge-sketch-actualization**
に置く。

## open questions

- `e3` actualization を plain bridge sketch actualization の直後に reopen するか、compare-ready bridge sketch の後段に置くか
- later `e3` widening で formal hook を new family に分けるか、theorem-side bridge line だけを先に増やすか
- `E21` / `E22` contrast を `e3` line より前に reopen する必要があるか
