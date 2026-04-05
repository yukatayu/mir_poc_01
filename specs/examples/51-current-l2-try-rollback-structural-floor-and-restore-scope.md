# 51 — current L2 try/rollback structural floor と restore-scope boundary

## 目的

この文書は、current L2 parser-free PoC と first checker cut inventory を前提に、
**`try` / rollback locality の structural floor と、`AtomicCut` frontier update / restore scope をどこまで checker cut に寄せるか**
を narrow に整理する。

ここで固定するのは final type system ではない。
固定するのは、

- `TryFallback` / `AtomicCut` について何が local / structural / decidable な floor か
- `place_anchor == current_place` gate と restore scope を runtime / proof boundary に残すべきか

という current docs-only judgment だけである。

## current code anchor

current implementation では次が成立している。

- `TryFallback` enter 時に `RollbackFrame { place_anchor, restore_snapshot }` を push する
- `place_anchor` は `TryFallback` entry 時の current `place`
- `restore_snapshot` は whole `place_store` snapshot
- `AtomicCut` は event を必ず追記する
- active rollback frame があり、かつ `place_anchor == current_place` のときだけ `restore_snapshot` を current `place_store` clone に更新する
- body failure catch 時には `restore_snapshot` を whole-store restore する

したがって current code anchor では、
**frontier update の gate は current `place` 一致で決まり、restore scope の実体は whole-store snapshot restore である。**

## representative runtime evidence

### E2

- `TryFallback` body 内に `AtomicCut` が無い
- failure 時は try-entry snapshot へ rollback

### E21

- `TryFallback` body 内で `AtomicCut` を `place_anchor == current_place` の位置に置く
- failure 時は post-cut snapshot へ rollback
- pre-cut mutation だけが残る

### E22

- `TryFallback` body 内で `AtomicCut` を nested place に置く
- `AtomicCut` event は残る
- ただし `place_anchor != current_place` なので frontier は更新されない
- failure 時は try-entry snapshot へ rollback

この 3 点により、current L2 では次が source-backed に言える。

1. `AtomicCut` event の存在だけでは frontier update を意味しない
2. frontier update の成否は dynamic `place_anchor == current_place` gate に依る
3. restore scope は current implementation では whole-store snapshot restore である

## 比較対象

### 案 1. restore scope まで first checker cut に寄せる

- `AtomicCut` の structural floor だけでなく、どこまで rollback が戻るかを checker judgment に入れる

#### 利点

- user-facing explanation と checker story が一見そろう

#### 欠点

- current code anchor は whole-store snapshot restore であり、checker に place-local restore scope を入れると docs/code がずれる
- dynamic `place_anchor == current_place` gate を structural / decidable judgment に押し込みやすい
- theorem prover / runtime proof boundary を early freeze しやすい

### 案 2. checker は structural floor に留め、frontier gate / restore scope は runtime / proof boundary に残す

- checker cut には region shape、statement shape、canonical chain non-interference だけを入れる
- `place_anchor == current_place` gate と whole-store snapshot restore は runtime semantics / proof boundary として残す

#### 利点

- current code anchor と一致する
- `E21` / `E22` の contrast case を runtime representative として素直に読める
- checker cut を local / structural / decidable 寄りに保てる
- restore scope refinement を将来 workstream へ残せる

#### 欠点

- user-facing explanation では structural floor と runtime gate を分けて説明する必要がある

### 案 3. docs では place-local restore のように要約し、actual code nuance を companion note に押し込む

#### 利点

- prose は短く見える

#### 欠点

- current repo の source-backed disciplineに反する
- `E22` の contrast case を誤って説明しやすい
- checker / runtime / theorem prover boundary が濁る

## current judgment

current L2 の next narrow step として自然なのは、
**案 2. checker は structural floor に留め、frontier gate / restore scope は runtime / proof boundary に残す**
である。

理由は次の通り。

1. current code anchor が whole-store snapshot restore である
2. `E21` / `E22` が dynamic gate の contrast を runtime evidence として与えている
3. first checker cut の entry criteria は local / structural / decidable floor に限る
4. restore scope refinement は theorem prover / future checker / runtime redesign と強く結びつく

## first checker cut に残してよいもの

- `TryFallback` region shape
- `fallback_body` を持つ control shape
- `AtomicCut` が ordinary statement node であること
- rollback / `atomic_cut` が canonical chain order 自体を変えないという structural boundary
- `AtomicCut` event と frontier update を区別して読む current docs-only rule

## first checker cut にまだ入れないもの

- `place_anchor == current_place` の dynamic gate 成否
- frontier update 後の restore scope の exact shape
- whole-store snapshot restore を place-local restore へ refinement するかどうか
- rollback / cut non-interference の一般証明
- nested place / distributed state を含む global safety / liveness

## current docs-only refinement

この文書により、current repo では次を同時に維持する。

- `AtomicCut` は first checker cut 候補 cluster に残してよい
- ただし checker cut へ上げるのは structural floor まで
- frontier update gate は runtime representative と step semantics で固定する
- restore scope 自体は runtime / proof boundary に残す

したがって current L2 では、
**`try` / rollback locality の checker-boundary整理を進めても、restore scope をまだ language core judgment に押し込まない**
のが自然である。

## OPEN に残すもの

- restore scope を将来 place-local snapshot へ refinement する必要があるか
- frontier gate を future checker API に一部持ち上げる価値があるか
- theorem prover 向け core relation で rollback frame をどう表すか
- model checker 候補 property として rollback locality をどこまで finite-state 化するか
