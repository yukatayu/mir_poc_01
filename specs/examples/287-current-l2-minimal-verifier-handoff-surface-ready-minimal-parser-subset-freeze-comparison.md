# 287 — current L2 minimal-verifier-handoff-surface-ready minimal-parser-subset-freeze comparison

## 目的

`specs/examples/285-current-l2-minimal-public-checker-boundary-ready-verifier-handoff-surface-comparison.md`
と
`specs/examples/286-current-l2-verifier-handoff-surface-ready-minimal-verifier-handoff-surface-threshold.md`
で Phase 5 verifier handoff gate を narrow に固定した次段として、

- Phase 6 front-half actual parser first tranche をどの accepted cluster で止めるべきか
- reject / malformed family と retained-later family をどこで分けるべきか
- `mir-ast/tests/support/current_l2_stage*` private helper と future public parser API の境界をどこで freeze するべきか

を比較する。

ここで固定するのは **current Phase 3 reopen line の
minimal-verifier-handoff-surface-ready minimal-parser-subset-freeze comparison** であり、

- public parser API
- final parser grammar
- span-rich diagnostics
- full request head / full clause suite / richer predicate grammar

はまだ固定しない。

## scope

- current L2 parser spike evidence だけを扱う。
- `specs/examples/29-current-l2-first-parser-subset-inventory.md` の full inventory 自体は維持する。
- `specs/examples/73-current-l2-first-parser-spike-staging.md` の checker-led staged order も維持する。
- stage 1 / 2 / 3 helper は test-only / private のままに留める。
- reconnect freeze そのものは次段に残す。

## current 前提

current repo では次が成立している。

1. first parser cut inventory には stage 1 chain/declaration、stage 2 try/rollback、stage 3 request/admit/predicate cluster が並んでいる。
2. actual parser spike の current staged order は
   stage 1 -> stage 2 -> stage 3
   checker-led spike である。
3. `mir-ast` public crate は still placeholder skeleton だが、
   stage 1 / stage 2 / stage 3 の helper-local / test-only evidence は既にある。
4. Phase 3 の next mainline は parser widening より first checker reconnect に寄せる current judgmentを持つ。

したがって current 問いは、
**actual parser first tranche として何を accepted cluster に入れ、何を retained-later floor に残すと最小か**
である。

## 比較観点

1. accepted cluster が existing parser-side evidence と first checker cut floor の両方に近いか
2. malformed / spillover / retained-later を混ぜずに書けるか
3. stage 3 request/admit/predicate cluster を premature に public-looking first tranche へ入れないか
4. next promoted line を parser-to-checker reconnect freeze へ進められるか

## 比較対象

### 案 1. stage 1 chain / declaration structural floor だけを actual parser first tranche にする

#### 含むもの

- option declaration core
- chain head / fallback row
- edge-local lineage metadata
- declaration-side guard slot

#### 利点

- 最も軽い
- `specs/examples/113...` の first reconnect family と直結する

#### 欠点

- stage 2 `try` / rollback structural floor を still later に残しすぎる
- Phase 6 front-half parser gate としては structural coverage が少し薄い

### 案 2. stage 1 + stage 2 structural floor を accepted cluster にし、stage 3 を retained-later floor に残す

#### accepted cluster

- stage 1 chain / declaration structural floor
  - option declaration core
  - chain head / fallback row
  - edge-local lineage metadata
  - declaration-side guard slot
- stage 2 try / rollback structural floor
  - single `try { ... } fallback { ... }` block
  - `atomic_cut` structural placement floor
  - statement head は `atomic_cut` と opaque `Other` だけを区別する

#### reject / malformed family

- `missing edge-local lineage metadata`
- `missing fallback body`
- `atomic_cut` fallback placement

#### retained-later floor

- declaration-side `admit` attached slot
- request head (`perform on` / `perform via`)
- request-local `require` / `ensure`
- minimal predicate fragment
- multiline attachment frame
- fixed two-slot request clause suite

#### 利点

- `specs/examples/73...` の staged order と整合する
- stage 1 と stage 2 の existing test-only evidence を actual parser first tranche 候補として固定できる
- stage 3 request/admit/predicate cluster は retained-later floor として整理できる
- next reconnect freeze で stage 1 summary と stage 2 structural contract を両方扱いやすい

#### 欠点

- 案 1 よりは 1 段広い
- stage 2 の accepted cluster には statement-body opaque cut が含まれるため、reader に minimality を丁寧に説明する必要がある

### 案 3. stage 1 + stage 2 + selected stage 3 cluster を actual parser first tranche に入れる

#### 例

- declaration-side `admit`
- request head
- request-local `require` / `ensure`
- minimal predicate fragment

#### 利点

- user-facing surface に早く近づける
- stage 3 helper-local evidence を public-ish parser tranche に寄せやすい

#### 欠点

- syntax pressure が一気に上がる
- request head / clause suite / predicate fragment / public parser API を同時に既成事実化しやすい
- `specs/examples/112...113` の reconnect-first sequencing を弱めやすい

## current judgment

current L2 で最も自然なのは、
**案 2. stage 1 + stage 2 structural floor を accepted cluster にし、stage 3 を retained-later floor に残す**
である。

理由は次の通り。

1. checker-led staged order と first checker reconnect line に最も整合する
2. stage 1 / stage 2 は actual parser-side evidence が既にあり、first tranche freeze に必要な source-backed 性がある
3. stage 3 request/admit/predicate cluster は helper-local evidence が厚くても、Phase 6 front-half の minimal parser gate としては still heavy である
4. reconnect freeze の次段で、accepted parser floor と checker floor を narrow に対応付けやすい

## current first choice reading

- actual parser first tranche の accepted cluster は stage 1 + stage 2 structural floor に留める
- truly malformed な first-tranche reject family は
  - `missing edge-local lineage metadata`
  - `missing fallback body`
  - `atomic_cut` fallback placement
  に留める
- stage 3 request/admit/predicate cluster は **retained-later floor** として扱い、
  current helper が返す spillover wording は public parser rejection contract ではなく
  out-of-tranche evidence として読む

## practical reading

current minimal parser subset freeze が示すのは、

- actual parser first tranche は declaration / chain と try/rollback structural floor まででよい
- request/admit/predicate surface は helper-local evidence を保ったまま later tranche に残す
- public parser API や final grammar を決めずに Phase 6 front-half parser gate を narrow に固定できる

の 3 点である。

## next promoted line

next promoted line は、
**minimal-parser-subset-freeze-ready parser-to-checker-reconnect-freeze comparison**
に置く。

## open questions

- stage 3 cluster を actual parser first tranche に上げる next threshold をどこに置くか
- retained-later floor のうち declaration-side `admit` と request-local clause suite のどちらを先に reopen するか
- public parser API をどの non-production boundary から actualize するか
