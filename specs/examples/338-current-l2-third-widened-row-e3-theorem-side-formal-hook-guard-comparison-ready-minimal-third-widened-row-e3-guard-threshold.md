# 338 — current L2 third-widened-row-e3 theorem-side formal-hook guard comparison-ready minimal-third-widened-row-e3 guard threshold

## 目的

`specs/examples/337-current-l2-second-widened-authored-row-e21-actualization-ready-third-widened-row-e3-theorem-side-formal-hook-guard-comparison.md`
で third widened row `e3` theorem-side / formal-hook guard comparison の current first choice を fixed した次段として、

- minimum guard をどこまでに留めるか
- current authored quintet と theorem-side current cut をどう守るか
- plain bridge sketch actualization への handoff を minimum にどう残すか

を比較する。

ここで固定するのは
**current L2 third-widened-row-e3 theorem-side formal-hook guard comparison-ready minimal-third-widened-row-e3 guard threshold**
であり、

- `e3` source row actualization
- new formal-hook family
- compare-ready bridge sketch
- concrete theorem / model-check tool binding

はまだ固定しない。

## 比較観点

1. `e3` deferred row と current theorem-side / formal-hook guard を minimum に残せるか
2. current authored quintet を壊さない later cut を minimum に残せるか
3. next line の plain bridge sketch actualization へ narrow に handoff できるか

## 比較対象

### 案 1. `deferred_row_ref` だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current theorem-side / formal-hook guard と next line handoff が弱い。

### 案 2. `guard_kind + deferred_row_ref + current_guard_refs + kept_later_refs` を持つ

#### 利点

- `e3` deferred row、current theorem-side consumer、current formal-hook top、bridge reopen ordering を minimum に残せる。
- current authored quintet を保ったまま next line を plain bridge sketch actualization に渡せる。
- compare-ready bridge sketch と concrete tool binding を later に押し分けやすい。

#### 欠点

- 案 1 より fields は増える。

### 案 3. future `e3` actualization shape まで minimum に含める

#### 利点

- 後段の actualization 接続は見えやすい。

#### 欠点

- threshold ではなく later reopen line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `guard_kind + deferred_row_ref + current_guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package は guard comparison close であり、`e3` actualization shape 自体はまだ later reopen に残すべきである。
2. current theorem-side consumer / formal-hook top / bridge reopen ordering を minimum に残さないと、`e3` line の遅延理由が弱い。
3. plain bridge sketch actualization を next promoted line に置く cut を repository memory に残せる。

## current first choice shape

```text
third_widened_row_guard = {
  guard_kind = current_l2_e3_theorem_side_formal_hook_guard_compare,
  deferred_row_ref = source_sample:e3-option-admit-chain,
  current_guard_refs = [
    current_authored_quintet,
    runtime_try_cut_cluster,
    proof_notebook_review_unit,
    plain_bridge_first_compare_ready_second
  ],
  kept_later_refs = [
    actual_e3_source_row,
    admit_family_formal_hook_widen,
    compare_ready_bridge_sketch,
    concrete_tool_binding
  ]
}
```

## practical reading

current minimal third widened row `e3` guard が示すのは、

- `e3` は deferred row のまま保つ
- current authored quintet と theorem-side row-local review-unit cut を維持する
- next line は theorem-side plain bridge sketch actualization に置く
- compare-ready bridge sketch と concrete tool binding は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-third-widened-row-e3-guard-ready plain-proof-notebook-bridge-sketch-actualization**
に置く。

## open questions

- `e3` actualization を plain bridge sketch actualization の直後に reopen するか
- compare-ready bridge sketch second reopen の後段に `e3` widen を残すか
- `e3` guard line と `E21` / `E22` contrast reopen の相互順序をどう置くか
