# 336 — current L2 second-widened-authored-row-e21-actualization-ready minimal-second-widened-authored-row-e21 threshold

## 目的

`specs/examples/335-current-l2-first-widened-authored-row-e1-actualization-ready-second-widened-authored-row-e21-actualization-comparison.md`
で second widened authored row `e21` actualization の current first choice を fixed した次段として、

- actualized `e21` row の minimum をどこまでに留めるか
- current authored inventory と reached-stage row を minimum にどう残すか
- next line `e3` theorem-side / formal-hook guard comparisonへどう narrow に handoff するか

を比較する。

ここで固定するのは
**current L2 second-widened-authored-row-e21-actualization-ready minimal-second-widened-authored-row-e21 threshold**
であり、

- `E21` / `E22` contrast comparison
- `e3` theorem-side / formal-hook guard comparison
- theorem-side plain bridge sketch actualization

はまだ固定しない。

## 比較観点

1. actualized `e21` row と current reached-stage row を minimum に残せるか
2. current lowerer / runner / formal-hook / review-unit guard を minimum で読めるか
3. next line `e3` theorem-side / formal-hook guard comparisonへ narrow に handoff できるか

## 比較対象

### 案 1. actualized sample stem だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current authored inventory / reached-stage row / guard を repository memory に残しきれない。

### 案 2. `actualization_kind + entry_criteria_refs + actualized_row + reached_stage_refs + guard_refs` を持つ

#### 利点

- `e21` widening package の current cut を lossless に残せる。
- current authored inventory 昇格と reached-stage row を最小で読める。
- next `e3` guard comparison line への handoff を minimum に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. `E21` / `E22` contrast や bridge sketch actualization まで threshold に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- threshold ではなく later line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + actualized_row + reached_stage_refs + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は `e21` actualization close であり、`E21` / `E22` contrast や `e3` / bridge sketch は next line に分けるべきである。
2. `e21` を authored row に上げたことだけでなく、current reached-stage row と formal-hook family guard を minimum に残す必要がある。
3. later line を threshold に持ち込まなくても、next promoted line を `e3` theorem-side / formal-hook guard comparison に置けば handoff は十分明確である。

## current first choice shape

```text
second_widened_authored_row_actualization = {
  actualization_kind = current_l2_second_widened_authored_row_e21,
  entry_criteria_refs = [
    deferred_authored_row_widen_sequence,
    proof_notebook_bridge_sketch_reopen_ordering,
    source_sample_authoring_policy,
    theorem_first_concrete_tool_pilot,
    first_widened_authored_row_e1_actualization
  ],
  actualized_row = {
    sample_stem = e21_try_atomic_cut_frontier,
    source_path_ref = samples/current-l2/e21-try-atomic-cut-frontier.txt,
    source_shape = helper_compatible_try_atomic_cut_frontier,
    runner_acceptance = accepted_sample_set_member,
    regression_inventory_status = source_authored
  },
  reached_stage_refs = [
    static_gate_valid,
    interpreter_success,
    runtime_try_cut_cluster_formal_hook
  ],
  guard_refs = [
    keep_e21_e22_contrast_later,
    keep_e3_as_guarded_third_slot,
    keep_plain_bridge_sketch_after_authored_row_line
  ]
}
```

## practical reading

current minimal threshold が示すのは、

- `e21-try-atomic-cut-frontier` は current authored source sample に昇格した
- current source shape は helper-compatible source row に留める
- reached-stage row は `static gate -> interpreter(success) -> runtime_try_cut_cluster formal hook`
- next promoted line は `e3` theorem-side / formal-hook guard comparison に置く

という最小 cut である。

## next promoted line

next promoted line は、
**second-widened-authored-row-e21-actualization-ready third-widened-row-e3 theorem-side / formal-hook guard comparison**
に置く。

## open questions

- `E21` / `E22` contrast をどの later package で public compare に上げるか
- `e3` guard comparisonを docs-only first choice に留めるか
- theorem-side plain bridge sketch actualization をどの timing で reopen するか
