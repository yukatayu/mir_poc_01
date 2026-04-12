# 329 — current L2 theorem-first-concrete-tool-pilot-ready deferred-authored-row-widen-sequencing comparison

## 目的

`specs/examples/327-current-l2-source-sample-authoring-bless-regression-policy-ready-theorem-first-concrete-tool-pilot-comparison.md`
と
`specs/examples/328-current-l2-theorem-first-concrete-tool-pilot-ready-minimal-theorem-first-concrete-tool-pilot-threshold.md`
で theorem-first concrete tool pilot を fixed した次段として、

- deferred authored row `e1` / `e3` / `e21` をどの順で widened authored row に移すか
- current runner / formal hook / review-unit cut をどこまで保ったまま widen できるか
- bridge-sketch reopen と concrete tool binding をどこまで後段に残すか

を比較する。

ここで固定するのは
**current L2 theorem-first-concrete-tool-pilot-ready deferred-authored-row-widen-sequencing comparison**
であり、

- first widened row の actual source authoring
- runner accepted set の widening
- bridge sketch reopen
- compare-bless metadata
- concrete theorem / model-check tool binding

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...328` で fixed 済みの source corpus scope / mapping / lowering / runner / ladder / source-sample policy / theorem-first pilot に置く。
- current authored source sample は `e2` / `e4` / `e23` first trio に留める。
- widening order は `e1` / `e21` / `e3` の relative sequencing だけを扱い、actual widening package そのものは次段に残す。

## current 前提

current repo では次が成立している。

1. current runner accepted set と regression helper authored set は `e2` / `e4` / `e23` first trio に固定されている。
2. `e1` と `e21` は runtime-side detached bundle formal hook の current top `runtime_try_cut_cluster` に still 接続しやすい。
3. `e3` は `E3-variant` row であり、current formal hook / proof-notebook review-unit family へそのままは接続しない。
4. theorem-side current cut は row-local `proof_notebook_review_unit` に留まり、bridge sketch / compare-bless metadata / concrete theorem/model-check binding は still later に残している。

したがって current 問いは、
**current runtime formal-hook family に乗る widen を先に閉じ、`admit` family pressure を持つ `e3` はその後ろへ送るのが自然か**
である。

## 比較観点

1. current first-trio runner / regression / ladder cut を壊さないか
2. tool-neutral formal hook と theorem-first review-unit pilot の current family を保てるか
3. `E21` / `E22` contrast や `admit` family pressure を premature に前倒ししないか
4. bridge-sketch reopen と concrete tool binding を still later に残せるか

## 比較対象

### 案 1. `e1 -> e21 -> e3` の順で widen sequencing を固定する

#### shape

```text
deferred_authored_row_widen_sequence = {
  sequencing_kind = current_l2_deferred_authored_row_sequence,
  ordered_rows = [
    { sample_stem = e1_place_atomic_cut, sequence_slot = first_runtime_try_cut_compatible_widen },
    { sample_stem = e21_try_atomic_cut_frontier, sequence_slot = second_runtime_try_cut_compatible_widen },
    { sample_stem = e3_option_admit_chain, sequence_slot = third_admit_family_guarded_widen }
  ],
  guard_refs = [
    keep_first_trio_as_current_authored_set_until_actualization,
    keep_runtime_try_cut_cluster_as_current_runtime_formal_hook_top,
    defer_e21_e22_contrast_pressure,
    defer_bridge_sketch_reopen,
    defer_admit_family_theorem_side_widen
  ]
}
```

#### 利点

- `e1` と `e21` を current runtime formal-hook family の内側で先に比べられる。
- `E21` / `E22` contrast や `e3` admit-family pressure を still later に押し分けやすい。
- bridge sketch reopen と theorem-side widening を premature に混ぜずに済む。

#### 欠点

- `e3` actualization は 3 本目まで遅れる。

### 案 2. `e21 -> e1 -> e3` の順で widen sequencing を固定する

#### 利点

- stage 2 `try` / rollback family を先に厚くできる。

#### 欠点

- `E21` / `E22` contrast pressure を `e1` より先に呼び込みやすい。
- smaller first widen としての `e1` を後ろへ送る理由が弱い。

### 案 3. `e3` を先に置く、または 3 本を同時 widen にする

#### 利点

- table 上は早く一様になる。

#### 欠点

- current formal hook / review-unit pilot family にそのまま乗らない `e3` の pressure を先に呼び込む。
- authored-row widen だけでなく theorem-side / formal-hook widen comparison を同時に要求しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. `e1 -> e21 -> e3` の順で widen sequencing を固定する**
である。

理由は次の通り。

1. `e1` は current runtime formal-hook top と theorem-first pilot guard を壊さずに widen しやすい最小 runtime row である。
2. `e21` も同じ runtime family に属しうるが、frontier update / `E21` contrast pressure を含むため `e1` の後ろへ置く方が staged line に合う。
3. `e3` は `admit` family と theorem-side widening pressure を呼ぶため、runtime try/cut compatible widen の後ろへ送るのが最小である。

## current first choice details

- first widened authored row の候補は `e1-place-atomic-cut` に置く。
- second widened authored row の候補は `e21-try-atomic-cut-frontier` に置く。
- `e3-option-admit-chain` は third slot に置き、actual widening の前に current formal hook / theorem-side guard とどう整合させるかを比較する余地を残す。
- current package では runner accepted set / regression helper accepted set / README ladder row 自体はまだ widen しない。

## open questions

- `e1` actualization の source text を exact representative prose mirror に寄せるか、fixed-subset helper-compatible source に留めるか
- `e21` widening の後に `E21` / `E22` contrast をどの task で surface するか
- `e3` widening を current formal-hook top の外で扱うか、theorem-side family widening comparison を先に置くか
