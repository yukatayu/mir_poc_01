# 330 — current L2 deferred-authored-row-widen-sequencing-ready minimal-deferred-authored-row-widen-sequencing threshold

## 目的

`specs/examples/329-current-l2-theorem-first-concrete-tool-pilot-ready-deferred-authored-row-widen-sequencing-comparison.md`
で deferred authored-row widen sequencing の current first choice を fixed した次段として、

- sequencing minimum をどこまでに留めるか
- ordered widen row と guard を minimum にどう残すか
- bridge-sketch reopen との handoff を minimum にどう反映するか

を比較する。

ここで固定するのは
**current L2 deferred-authored-row-widen-sequencing-ready minimal-deferred-authored-row-widen-sequencing threshold**
であり、

- actual widened source file
- runner / regression helper widening
- theorem-side bridge sketch actualization
- compare-bless metadata

はまだ固定しない。

## 比較観点

1. ordered widen row を minimum に残せるか
2. current formal hook / theorem-side guard を minimum で読めるか
3. next bridge-sketch ordering line へ narrow に handoff できるか

## 比較対象

### 案 1. ordered sample stems だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- current entry criteria と theorem-side / runner guard が弱い。

### 案 2. `sequencing_kind + fixed_entry_criteria_refs + ordered_widen_rows + guard_refs` を持つ

#### 利点

- `e1 -> e21 -> e3` current order と guard を lossless に残せる。
- current package が docs-only sequencing close であることを保てる。
- next bridge-sketch ordering line への handoff を minimum に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. actual widened-row task や bridge-sketch reopen task まで minimum に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- threshold ではなく later actualization line を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `sequencing_kind + fixed_entry_criteria_refs + ordered_widen_rows + guard_refs` を持つ**
である。

理由は次の通り。

1. current task は sequencing close であり、actual widening や theorem-side reopen は next package に分けるべきである。
2. `e1 -> e21 -> e3` order だけでは current formal-hook / theorem-side guard を repository memory に残しきれない。
3. bridge-sketch reopen ordering は next line なので、その handoff を minimum guard に残す必要がある。

## current first choice shape

```text
deferred_authored_row_widen_sequence = {
  sequencing_kind = current_l2_deferred_authored_row_sequence,
  fixed_entry_criteria_refs = [
    phase6_source_sample_authoring_policy,
    phase6_theorem_first_concrete_tool_pilot
  ],
  ordered_widen_rows = [
    { sample_stem = e1_place_atomic_cut, sequence_slot = first_runtime_try_cut_compatible_widen },
    { sample_stem = e21_try_atomic_cut_frontier, sequence_slot = second_runtime_try_cut_compatible_widen },
    { sample_stem = e3_option_admit_chain, sequence_slot = third_admit_family_guarded_widen }
  ],
  guard_refs = [
    keep_first_trio_as_current_authored_set_until_actualization,
    keep_runtime_try_cut_cluster_as_current_runtime_formal_hook_top,
    avoid_bridge_sketch_premature_reopen,
    avoid_compare_bless_metadata_premature_merge,
    avoid_concrete_tool_binding_backpressure
  ]
}
```

## practical reading

current minimal deferred-authored-row widen sequencing が示すのは、

- current order は `e1 -> e21 -> e3` に置く
- `e1` / `e21` は current runtime formal-hook family の内側で先に比べる
- `e3` は theorem-side / formal-hook guard を要する third slot に残す
- next line は proof-notebook bridge-sketch reopen ordering に置く

という最小 cut である。

## next promoted line

next promoted line は、
**deferred-authored-row-widen-sequencing-ready proof-notebook-bridge-sketch-reopen-ordering comparison**
に置く。

## open questions

- first widened row actualization で runner accepted set と regression helper accepted set をどう同期するか
- `e3` widening 前に theorem-side family compare を 1 package 挟むべきか
- bridge-sketch reopen ordering の後で first widened row actualization をどう tasks snapshot に戻すか
