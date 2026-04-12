# 369 — current L2 model-check-concrete-carrier-first-actualization-gate-ready stable-malformed-broader-follow-up-inventory comparison

## 目的

`specs/examples/368-current-l2-model-check-concrete-carrier-first-actualization-gate-ready-minimal-model-check-concrete-carrier-first-actualization-gate-threshold.md`
で model-check side の first gate を fixed した次段として、

- `e4/e19` edge-pair close の後に broader stable malformed follow-up をどの順で reopen するか
- missing-option family、capability family、duplicate cluster、`TryFallback` / `AtomicCut` malformed-static family をどう押し分けるか
- repo-level next line を public operational later gate へ handoff しつつ、Macro 4 side の next malformed reopen point をどう残すか

を比較する。

ここで固定するのは
**current L2 model-check-concrete-carrier-first-actualization-gate-ready stable-malformed-broader-follow-up-inventory comparison**
であり、

- missing-option first reopen の actual source-backed widening
- capability second reopen の actual source-backed widening
- duplicate cluster の actual promotion
- `TryFallback` / `AtomicCut` malformed-static family の actual source-backed promotion

はまだ固定しない。

## scope

- current package は docs-only sequencing / inventory close に留める。
- `e4/e19` edge-pair close を current entry criteria とし、broader malformed follow-up order だけを narrow に固定する。
- helper-local checker spike と static fixture corpus の existing evidence を再利用し、new code / sample / fixture は増やさない。

## current 前提

current repo では次が成立している。

1. `specs/examples/353...354` により、stable static malformed post-contrast sequencing の second broader cluster は stable reason-code / fixture-static cluster に固定済みである。
2. `specs/examples/361...362` により、stable-static edge-pair first reopen も fixed 済みであり、`e4-malformed-lineage` と `e19-malformed-target-mismatch` は source-backed static-stop pair として閉じている。
3. `specs/examples/47` により、missing-option structure floor は helper-local second checker spike として actual evidence を持つ。
4. `specs/examples/48` により、capability strengthening floor も helper-local third checker spike として actual evidence を持つ。
5. duplicate cluster は stable inventory の外に保つ current split が強く、`specs/examples/55` により `TryFallback` / `AtomicCut` malformed-static family は still later に残している。

したがって current 問いは、
**stable malformed broader follow-up の next inventory を missing-option first / capability second に置き、duplicate cluster と try/rollback malformed-static family を kept-later に残すのが最小か**
である。

## 比較観点

1. stable malformed cluster の current split を保てるか
2. current checker-spike evidence をそのまま broader reopen order の根拠に使えるか
3. duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family を混ぜずに済むか
4. repo-level next line を public operational later gate へ clean に handoff できるか

## 比較対象

### 案 1. missing-option family を first reopen、capability family を second reopen に置く

#### shape

```text
stable_malformed_followup = {
  inventory_kind = current_l2_stable_malformed_followup_order,
  first_reopen_family = missing_option_structure_floor,
  first_reopen_rows = [e16, e17, e18],
  second_reopen_family = capability_strengthening_floor,
  second_reopen_rows = [e13, e20],
  kept_later = [
    duplicate_cluster,
    try_rollback_malformed_static_family
  ]
}
```

#### 利点

- `specs/examples/47` と `48` の current helper-local evidence をそのまま broader follow-up order に接続できる。
- missing-option family は coverage が厚く、capability family はその次点なので順序が素直である。
- duplicate cluster と try/rollback malformed-static family を混ぜずに narrow reopen line を残せる。

#### 欠点

- actual source-backed widening 自体は次段へ残る。

### 案 2. duplicate cluster を next reopen にする

#### 利点

- duplicate declaration line を早く visible にできる。

#### 欠点

- stable cluster と duplicate cluster の current split を崩す。
- `checked_reasons` / detached `reason_codes` current reading と衝突しやすい。

### 案 3. `TryFallback` / `AtomicCut` malformed-static family を next reopen にする

#### 利点

- `e21/e22` runtime contrast pair との family continuity は見えやすい。

#### 欠点

- `specs/examples/55` の still-later judgment と衝突する。
- dedicated AST structural helper / wording / artifact cut を同時に reopen しやすい。

### 案 4. missing-option / capability を同じ actual widening package に混ぜる

#### 利点

- malformed widening の速度は上がる。

#### 欠点

- sequencing close と actual widening を一度に扱うことになり、current docs-only package としては重い。
- capability floor の evidence density は missing-option より薄いため、first reopen と second reopen の distinction が消える。

## current judgment

current L2 で最も自然なのは、
**案 1. missing-option family を first reopen、capability family を second reopen に置く**
である。

理由は次の通り。

1. `e4/e19` edge-pair close の次段としては、duplicate cluster や try/rollback malformed-static family より missing-option / capability family の方が current stable split と整合する。
2. missing-option family は helper-local evidence が厚く、capability family はその次点なので、順序を docs-only で先に固定する価値がある。
3. duplicate cluster と try/rollback malformed-static family は current repo で別の risk / helper pressure を持っており、この package に混ぜない方が drift を抑えられる。

## current first choice details

- first reopen family は `missing-option structure floor` に置く。
- first reopen row family は `e16-malformed-missing-chain-head-option` / `e17-malformed-missing-predecessor-option` / `e18-malformed-missing-successor-option` に置く。
- second reopen family は `capability strengthening floor` に置く。
- second reopen row family は `e13-malformed-capability-strengthening` / `e20-malformed-late-capability-strengthening` に置く。
- duplicate cluster (`e14` / `e15`) は stable inventory の外へ kept-later に残す。
- `TryFallback` / `AtomicCut` malformed-static family は dedicated AST structural helper pressure を持つ later line に残す。
- repo-level next promoted line は `public operational CLI / final public contract later gate` に置きつつ、Macro 4 side の next malformed reopen point は missing-option first reopen actualization に残す。

## next promoted line

next promoted line は、
**stable-malformed-broader-follow-up-inventory-ready public-operational-cli-final-public-contract-later-gate comparison**
に置く。

## open questions

- missing-option first reopen を docs-only actualization comparison と source-backed wideningのどちらから開くか
- capability second reopen を missing-option close の直後に続けるか
- duplicate cluster を stable source-of-truth line にいつ接続するか
- `TryFallback` / `AtomicCut` malformed-static family を dedicated AST structural helper とどう接続するか
