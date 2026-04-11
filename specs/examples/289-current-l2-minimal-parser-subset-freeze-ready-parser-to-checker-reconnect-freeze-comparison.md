# 289 — current L2 minimal-parser-subset-freeze-ready parser-to-checker-reconnect-freeze comparison

## 目的

`specs/examples/287-current-l2-minimal-verifier-handoff-surface-ready-minimal-parser-subset-freeze-comparison.md`
と
`specs/examples/288-current-l2-minimal-parser-subset-freeze-ready-minimal-parser-subset-freeze-threshold.md`
で minimal parser subset freeze を
stage 1 + stage 2 structural floor に固定した次段として、

- first checker reconnect をどの floor まで actual bridge とみなすべきか
- stage 1 summary contract と stage 2 try/rollback structural contract をどう併置するべきか
- stage 3 request/predicate line、`e19` direct target mismatch、`E21` / `E22` runtime contrast をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 3 -> 5 bridge line の
minimal-parser-subset-freeze-ready parser-to-checker-reconnect-freeze comparison** であり、

- public checker API
- public parser API
- stage 3 request contract reconnect
- runtime / proof boundary actualization

はまだ固定しない。

## scope

- minimal parser subset freeze の accepted cluster から first checker cut への接続だけを扱う。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` の 6 cluster は維持する。
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` の readiness judgment も維持する。
- current helper-local / test-only contract widening を前提に public API actualization へ進まない。

## current 前提

current repo では次が成立している。

1. minimal parser subset freeze は stage 1 + stage 2 structural floorに固定済みである。
2. stage 1 parser-side reconnect evidence には `Stage1ReconnectClusters` 3-bool summary contract がある。
3. stage 2 parser-side reconnect evidence には
   `checked_try_rollback_structural_verdict` /
   `checked_try_rollback_structural_findings`
   に直接合わせる helper-local summary contract がある。
4. `specs/examples/113...` により first reconnect family は stage 1 から始める current judgment があるが、
   `plan/07-parser-free-poc-stack.md` では stage 2 structural floor を second reconnect step として already narrow に整理している。

したがって current 問いは、
**minimal parser subset freeze の accepted cluster を first checker cut へ reconnect するとき、どこまでを current tranche の bridge に含めるのが最小か**
である。

## 比較観点

1. accepted parser floor と checker floor の対応を narrow に保てるか
2. stage 1 summary contract と stage 2 structural contract を混同せず並べられるか
3. stage 3 request/predicate line、`e19`、`E21` / `E22` を premature に bridge へ入れないか
4. Phase 3 self-driven reconnect line を一区切りとして freeze し、その次の mainline を closeout sweep へ戻せるか

## 比較対象

### 案 1. stage 1 reconnect summary だけを bridge にする

#### 含むもの

- `same_lineage_floor`
- `missing_option_structure_floor`
- `capability_strengthening_floor`

#### 利点

- `specs/examples/113...` の first reconnect family judgment と最も直接に一致する
- bridge は最も軽い

#### 欠点

- minimal parser subset freeze が含む stage 2 accepted cluster を bridge から落としてしまう
- Phase 6 front-half parser/checker gate としては structural coverage が不足する

### 案 2. stage 1 reconnect summary + stage 2 try/rollback structural contract を bridge にする

#### 含むもの

- stage 1 reconnect summary
  - `same_lineage_floor`
  - `missing_option_structure_floor`
  - `capability_strengthening_floor`
- stage 2 try/rollback structural contract
  - malformed pair:
    - `missing fallback body`
    - `atomic_cut` fallback placement
  - valid one-shot `atomic_cut in try body` no-findings smoke

#### 利点

- minimal parser subset freeze の accepted cluster と整合する
- stage 1 / stage 2 の existing helper-local contract をそのまま narrow bridge に使える
- stage 3 request/predicate line、`e19`、`E21` / `E22` を still later に残せる
- current reconnect line を一区切りとして freeze しやすい

#### 欠点

- stage 1 summary contract と stage 2 row/detail contract の heterogeneity を説明する必要がある
- stage 1 only より 1 段重い

### 案 3. stage 3 request contract subset と `e19` / `E21` / `E22` まで bridge に入れる

#### 利点

- parser side と checker side を一気に広く結べる
- runtime-facing surface に早く近づける

#### 欠点

- stage 3 request/predicate reopen、`e19` summary redesign、runtime/proof contrast を同時に抱え込む
- current narrow progression を壊しやすい
- Phase 3 self-driven freeze line を曖昧にする

## current judgment

current L2 で最も自然なのは、
**案 2. stage 1 reconnect summary + stage 2 try/rollback structural contract を bridge にする**
である。

理由は次の通り。

1. minimal parser subset freeze の accepted cluster と対応が取れる
2. stage 1 / stage 2 ともに source-backed helper-local contract があり、narrow bridge に十分である
3. stage 3 request/predicate line、`e19`、`E21` / `E22` を still later に残せる
4. current reconnect line を freeze した後、repo mainline を Phase 1 / 2 / 4 / 5 closeout sweep へ戻しやすい

## current first choice reading

- parser-to-checker reconnect bridge は stage 1 summary + stage 2 structural contract に留める
- stage 3 request/predicate reconnect は retained-later floor に残す
- `e19` direct target mismatch family は stage 1 summary redesign pressure があるため still later に残す
- `E21` / `E22` runtime contrast は runtime / proof boundary に近いため still later に残す

## practical reading

current parser-to-checker reconnect freeze が示すのは、

- accepted parser subset から first checker cut へ渡す minimal bridge は stage 1 と stage 2 までで十分である
- current tranche では stage 3 request line や runtime-facing contrast を bridge に混ぜない
- Phase 3 reconnect line はここで一区切りと読んでよい

の 3 点である。

## next promoted line

next promoted line は、
**Phase 1 semantics / invariants / notation final sweep**
に置く。

## open questions

- stage 1 summary contract と stage 2 structural contract を future compile path でどう統合するか
- `e19` direct target mismatch をどの bridge redesign で reopen するか
- stage 3 request contract subset reconnect をいつ reopen するか
