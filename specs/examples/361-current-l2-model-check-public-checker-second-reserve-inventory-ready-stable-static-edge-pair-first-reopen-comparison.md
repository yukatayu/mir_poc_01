# 361 — current L2 model-check-public-checker-second-reserve-inventory-ready stable-static-edge-pair-first-reopen comparison

## 目的

`specs/examples/360-current-l2-model-check-public-checker-second-reserve-inventory-ready-minimal-model-check-public-checker-second-reserve-inventory-threshold.md`
で model-check/public-checker second reserve inventory の minimum を fixed した次段として、

- stable static malformed broader cluster の first reopen をどの shape で actualize するか
- `e4` / `e19` edge-pair のうち deferred 側 `e19` をどこまで source-backed line へ上げるか
- duplicate cluster、`TryFallback` / `AtomicCut` malformed-static family、public operational surface gate をどう kept-later に押し分けるか

を比較する。

ここで固定するのは
**current L2 model-check-public-checker-second-reserve-inventory-ready stable-static-edge-pair-first-reopen comparison**
であり、

- public operational surface actualization gate
- shared-space identity / auth layering reopen
- model-check concrete carrier first actualization gate

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...360` で fixed 済みの source corpus / lowering / runner / ladder / stable static malformed sequencing / public surface inventory / shared-space docs-first re-entry / model-check reserve inventory に置く。
- current package では stable static malformed broader cluster を全面 widen せず、`e4` / `e19` edge-pair sideだけを actualize する。
- `e19` formal hook は current `fixture_static_cluster` route のままに留め、new theorem/model-check carrier は増やさない。

## current 前提

current repo では次が成立している。

1. `e4-malformed-lineage` は current source-authored static-stop row として、runner / regression / ladder / fixture-static formal-hook smoke まで actualize 済みである。
2. `e19-malformed-target-mismatch` は fixture/static side では already stable cluster に入っているが、source sample / runner accepted set / regression inventory / ladder にはまだ入っていない。
3. stable static malformed post-contrast sequencing により、second broader cluster は stable reason-code / fixture-static cluster、first reopen point は `e4` / `e19` edge-pair sideに fixed 済みである。
4. duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は still later と読む current cut がある。

したがって current 問いは、
**current edge-pair reopen を `e19` source row actualization に narrow し、`e4` existing source row と合わせて source-backed static pair に閉じるのが最小か**
である。

## 比較観点

1. stable-static edge-pair reopen を narrow に閉じられるか
2. current fixture-static formal-hook route を維持しつつ source-backed line を厚くできるか
3. duplicate cluster と try/rollback malformed-static family を still later に保てるか
4. repo-level next line を public operational surface actualization gate へ clean に handoff できるか

## 比較対象

### 案 1. deferred 側 `e19` だけを source-authored row に actualize し、`e4` existing row と合わせて edge-pair line を閉じる

#### shape

```text
stable_static_edge_pair_reopen = {
  selected_rows = [
    e4_existing_source_authored_static_stop,
    e19_new_source_authored_static_stop
  ],
  actualization_surface = [
    source_sample_file,
    runner_accepted_set,
    regression_inventory,
    verification_ladder
  ],
  formal_hook_route = fixture_static_cluster,
  guard_refs = [
    keep_duplicate_cluster_later,
    keep_try_rollback_malformed_static_family_later,
    keep_public_operational_actualization_as_next_gate
  ]
}
```

#### 利点

- broader stable cluster を一気に widen せず、`e4` / `e19` edge-pair にだけ narrow に戻せる。
- `e19` は fixture/static side の stable anchor を already 持つため、new semantic checker logic を増やさずに source-backed line を厚くできる。
- public operational surface actualization gate を repo-level next line として clean に残せる。

#### 欠点

- missing-option / capability family や duplicate cluster は次段へ残る。

### 案 2. stable reason-code / fixture-static cluster をまとめて broader actual corpus へ上げる

#### 利点

- stable static malformed side の breadth は増える。

#### 欠点

- current package が太くなり、edge-pair reopen の narrow packageではなくなる。
- duplicate cluster や other malformed family との境界が弱くなる。

### 案 3. `e19` は fixture/static side のまま据え置き、repo-level current line を public operational surface gate に直接送る

#### 利点

- docs 更新量は減る。

#### 欠点

- `e4` / `e19` edge-pair first reopen という current mainline を未完了のまま先送りする。
- stable-static edge-pair line が source-backed pair として閉じない。

## current judgment

current L2 で最も自然なのは、
**案 1. deferred 側 `e19` だけを source-authored row に actualize し、`e4` existing row と合わせて edge-pair line を閉じる**
である。

理由は次の通り。

1. stable-static edge-pair first reopen の current contract は broader cluster actualization ではなく `e4` / `e19` pair close である。
2. `e19` は fixture/static side の stable anchor を already 持つため、current helper-local source path に narrow に接続できる。
3. public operational surface actualization gate は repo-level next line として独立に切った方が、public contract pressure と source-sample widening が混線しない。

## current first choice details

- `samples/current-l2/e19-malformed-target-mismatch.txt` を helper-compatible source row として actualize する。
- `mir_runtime::current_l2` accepted sample set、lowering ratchet、source sample runner、verification ladder に `e19` row を加える。
- `python3 scripts/current_l2_source_sample_regression.py` authored inventory / regression bundle に `e19` static formal-hook smoke を加える。
- `e19` reached-stage は `static gate = reached(malformed)`、`interpreter = not reached (static stop)`、`formal hook = reached(fixture_static_cluster)` に置く。
- duplicate cluster、`TryFallback` / `AtomicCut` malformed-static family、broader public operational actualization は kept-later に残す。

## next promoted line

next promoted line は、
**stable-static-edge-pair-first-reopen-ready public-operational-surface-actualization-gate comparison**
に置く。

## open questions

- stable static malformed broader cluster の次段を missing-option / capability family と duplicate cluster のどちらから reopen するか
- `fixture_static_cluster` route を source-runner-native artifact route に later 移す必要があるか
- public operational surface actualization gate で first public-pressure candidate をどの compile-ready tranche に置くか
