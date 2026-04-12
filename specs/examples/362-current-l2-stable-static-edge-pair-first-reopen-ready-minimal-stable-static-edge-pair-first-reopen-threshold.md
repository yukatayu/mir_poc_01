# 362 — current L2 stable-static-edge-pair-first-reopen-ready minimal-stable-static-edge-pair-first-reopen threshold

## 目的

`specs/examples/361-current-l2-model-check-public-checker-second-reserve-inventory-ready-stable-static-edge-pair-first-reopen-comparison.md`
で stable-static edge-pair first reopen の current first choice を fixed した次段として、

- `e4` existing row と `e19` new row をどこまで minimum に残すか
- source-backed static-stop pair の reached-stage と kept-later line をどう handoff するか
- public operational surface actualization gate をどこまで threshold 側に残すか

を比較する。

ここで固定するのは
**current L2 stable-static-edge-pair-first-reopen-ready minimal-stable-static-edge-pair-first-reopen threshold**
であり、

- public operational surface actualization gate の具体形
- shared-space identity / auth layering reopen
- model-check concrete carrier first actualization gate

はまだ固定しない。

## 比較観点

1. edge-pair reopen を minimum に残せるか
2. new row `e19` と existing row `e4` の pair 読みを lossless に残せるか
3. kept-later malformed families と next repo-level line を明示できるか

## 比較対象

### 案 1. `e19` source file path と reached-stage だけを残す

#### 利点

- 軽い。

#### 欠点

- `e4` existing row との pair 読み、runner/regression/ladder actualization、kept-later malformed families が抜ける。

### 案 2. `reopen_kind + entry_criteria_refs + actualized_edge_pair_rows + reached_stage_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- `e4` existing row と `e19` new row の pair close を lossless に残せる。
- source sample / runner / regression / ladder actualization と fixture-static formal-hook route を同時に handoff できる。
- public operational surface actualization gate と other malformed families を kept-later として明示できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. public operational surface actualization gate の candidate shape まで threshold に含める

#### 利点

- next line は見えやすい。

#### 欠点

- threshold ではなく次段 package を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `reopen_kind + entry_criteria_refs + actualized_edge_pair_rows + reached_stage_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の意味は `e19` 単独 actualization ではなく、`e4` existing row と合わせた edge-pair close にある。
2. source sample / runner / regression / ladder actualization を残さないと、source-backed reopen の効果が threshold から落ちる。
3. public operational surface actualization gate は next repo-level line であり、threshold では kept-later に残すのが最小である。

## current first choice shape

```text
stable_static_edge_pair_reopen = {
  reopen_kind = current_l2_source_backed_fixture_static_edge_pair,
  entry_criteria_refs = [
    stable_static_malformed_post_contrast_sequencing,
    model_check_public_checker_second_reserve_inventory
  ],
  actualized_edge_pair_rows = [
    e4_existing_source_authored_static_stop,
    e19_new_source_authored_static_stop
  ],
  reached_stage_refs = [
    static_gate_reached_malformed,
    interpreter_not_reached_static_stop,
    formal_hook_reached_fixture_static_cluster
  ],
  guard_refs = [
    keep_duplicate_cluster_later,
    keep_try_rollback_malformed_static_family_later,
    keep_broader_public_operational_actualization_later
  ],
  kept_later_refs = [
    public_operational_surface_actualization_gate,
    shared_space_identity_auth_layering_reopen,
    model_check_concrete_carrier_first_actualization_gate
  ]
}
```

## practical reading

current minimal stable-static edge-pair reopen が示すのは、

- `e4` / `e19` は source-backed static-stop pair として current authored corpus に入った
- current source-backed static-stop pair は `fixture_static_cluster` current top を保つ
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は still later に残す
- repo-level next line は public operational surface actualization gate に進む

という最小 cut である。

## next promoted line

next promoted line は、
**stable-static-edge-pair-first-reopen-ready public-operational-surface-actualization-gate comparison**
に置く。

## open questions

- public operational surface actualization gate で `mir-runtime::current_l2`、`mir-ast::current_l2`、repo-local helper surface のどれを first candidate に置くか
- stable static malformed broader cluster の次段 reopen を missing-option / capability family と duplicate cluster のどちらから開くか
- source-backed static-stop pair の detached artifact bless / retention policy を later にどう接続するか
