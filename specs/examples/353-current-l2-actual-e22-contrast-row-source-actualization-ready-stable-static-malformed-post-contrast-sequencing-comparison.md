# 353 — current L2 actual-e22-contrast-row-source-actualization-ready stable-static-malformed-post-contrast-sequencing comparison

## 目的

`specs/examples/352-current-l2-actual-e22-contrast-row-source-actualization-ready-minimal-actual-e22-contrast-row-threshold.md`
で `e22` actualization の minimum を fixed した次段として、

- `e22` contrast close の後にどの static malformed family を second broader cluster に置くか
- stable typed/static reason-code line と duplicate cluster と try/rollback malformed-static line をどう分けるか
- current source-sample mainline をどこで一旦 public surface inventory へ handoff するか

を比較する。

ここで固定するのは
**current L2 actual-e22-contrast-row-source-actualization-ready stable-static-malformed-post-contrast-sequencing comparison**
であり、

- stable static malformed first concrete reopen actualization
- parser / checker / runtime public surface inventory
- Mirrorea / shared-space docs-first re-entry

はまだ固定しない。

## scope

- entry criteria は `specs/examples/34` `39` `43` `44` `45` `55` と `315...352` の stable reason-code line / source-sample line に置く。
- current package は docs-only sequencing close であり、new sample file や static fixture actualization は増やさない。
- `e22` contrast pair は fixed entry criteria とし、broader cluster selection だけを narrow に決める。

## current 前提

current repo では次が成立している。

1. `e22` contrast row actualization により、first post-sextet runtime contrast pair `e21` / `e22` は source-backed に閉じている。
2. stable static malformed side では、typed/static reason-code lineが already settled しており、duplicate declaration cluster は stable inventory の外に保たれている。
3. `e4-malformed-lineage` と `e19-malformed-target-mismatch` は stable edge-pair family の source-backed anchor である。
4. `TryFallback` / `AtomicCut` malformed static family は `specs/examples/55` により still later と読む current docs-only judgment がある。

したがって current 問いは、
**`e22` の次段を stable reason-code / fixture-static cluster に置き、その first reopen は edge-pair side `e4` / `e19` に留めるのが最小か**
である。

## 比較観点

1. `e22` contrast close の後段として breadth を抑えられるか
2. duplicate cluster と stable reason-code cluster を混ぜずに済むか
3. try/rollback malformed-static family と broader runtime family を still later に押し分けられるか
4. repo-level next package を public surface inventoryへ handoff できるか

## 比較対象

### 案 1. stable reason-code / fixture-static cluster を second broader cluster にし、first reopen は `e4` / `e19` edge-pair sideに置く

#### shape

```text
post_contrast_cluster = {
  cluster_kind = stable_reason_code_fixture_static_cluster,
  first_reopen_rows = [
    e4_malformed_lineage,
    e19_malformed_target_mismatch
  ],
  guard_refs = [
    keep_duplicate_cluster_outside_stable_inventory,
    keep_try_rollback_malformed_static_family_later,
    keep_broader_runtime_families_later
  ]
}
```

#### 利点

- current stable typed/static line と current fixture-static line をそのまま使える。
- `e4` / `e19` は edge-pair family の already settled anchor なので、new AST/runtime contract を増やさない。
- duplicate cluster と broader runtime family を混ぜずに narrow reopen point を残せる。

#### 欠点

- actual first reopen は次段 package に残る。

### 案 2. duplicate declaration cluster (`e14` / `e15`) を second broader cluster にする

#### 利点

- malformed static inventory の breadth は見えやすい。

#### 欠点

- stable typed/static reason-code line と current split が崩れる。
- current docs では duplicate cluster は stable inventory の外に保つ cut が強い。

### 案 3. `TryFallback` / `AtomicCut` malformed static family を second broader cluster にする

#### 利点

- `e22` contrast と family continuity は見えやすい。

#### 欠点

- `specs/examples/55` の still-later judgmentと衝突する。
- helper / wording / artifact taxonomy を同時に reopen しやすい。

### 案 4. expiry / request-contract など broader runtime family を second broader cluster にする

#### 利点

- runnable line は続く。

#### 欠点

- static malformed sequencing task そのものを飛ばすことになる。
- parser pressure と runtime pressure を先に呼び込みやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. stable reason-code / fixture-static cluster を second broader cluster にし、first reopen は `e4` / `e19` edge-pair sideに置く**
である。

理由は次の通り。

1. `e22` contrast close の次段としては、broader runtime family より stable fixture-static side の方が narrow である。
2. duplicate cluster は current stable inventory の外に保つ reading が already settled している。
3. try/rollback malformed-static family は runtime representative だけで当面十分という docs-only judgment があり、ここで reopen しない方が一貫する。

## current first choice details

- second broader cluster は `stable_reason_code_fixture_static_cluster` に置く。
- first concrete reopen point は `e4-malformed-lineage` / `e19-malformed-target-mismatch` edge-pair sideに置く。
- duplicate declaration cluster は stable inventory の外へ残す。
- `TryFallback` / `AtomicCut` malformed static family は `specs/examples/55` current cut のまま still later に残す。
- repo-level next promoted line は `parser / checker / runtime public surface inventory` に handoff し、stable static edge-pair first reopen は Macro 4 reopen point として残す。

## next promoted line

next promoted line は、
**stable-static-malformed-post-contrast-sequencing-ready parser-checker-runtime-public-surface-inventory comparison**
に置く。

## open questions

- stable static edge-pair first reopen を `e4/e19` compare だけにするか、missing-option / capability family も同時に inventory 化するか
- public surface inventory の後で stable static edge-pair reopen を mainline に戻す順番
- duplicate cluster を later typed source-of-truth line にいつ接続するか
