# 48 — current L2 capability third checker spike

## 目的

この文書は、same-lineage first checker spike と missing-option second checker spike の次段として
**capability strengthening floor を third checker spike として helper-local に切り出してよいか**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- capability family を helper-local compare に乗せる順序判断
- その spike を current helper boundary を壊さずにどこへ置くか
- detached validation loop でどの smoke まで回してよいか

という最小 cut だけである。

## current 前提

current L2 では次が成立している。

- same-lineage floor は helper-local first checker spike として smoke 済みである
- missing-option structure floor も helper-local second checker spike として smoke 済みである
- static-only corpus baseline では、same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` の coverage が見えている
- capability family は stable cluster 8 kind の一部として `checked_reason_codes` と detached `reason_codes` に actualize 済みである
- `e13` に加えて `e20` が入り、capability floor は singleton fixture ではなくなった

したがって current 問いは、
**capability floor も helper-local third checker spike として actualize してよいか**
である。

## 比較観点

1. current corpus coverage が helper-local spike として十分か
2. fixture-side `checked_reason_codes` と detached static gate `reason_codes` をそのまま使えるか
3. public checker API や detached schema を増やさずに済むか
4. shared support helper 化や public checker cut と責務競合しないか

## 比較対象

### 案 1. capability strengthening floor を third checker spike にする

- `capability_strengthens`

だけを narrow に compare する helper-local checker spike を置く。

#### 利点

- `e13` / `e20` の 2 fixture があり、singleton family ではなくなった
- row shape が `from_capability` / `to_capability` の pair だけで素直である
- same-lineage / missing-option と同じ compare pattern を再利用できる
- public checker API をまだ固定せずに capability floor の evidence を厚くできる

#### 欠点

- coverage `2` は same-lineage / missing-option よりまだ薄い
- capability prohibition の richer static judgment までは表せず、helper-local exact compare に留まる

### 案 2. capability floor は corpus coverage 拡張だけで止める

#### 利点

- helper の数を増やさずに済む

#### 欠点

- capability floor だけ actual checker spike が無い非対称な状態になる
- same-lineage / missing-option と比べた proof-of-cut の密度が低いまま残る

### 案 3. same-lineage / missing-option / capability を先に shared helper へ統合する

#### 利点

- helper 数は減る

#### 欠点

- shared helper へ寄せる前に、capability family 単独の spike evidence が無い
- current detached validation loop の helper-local boundaryを早く抽象化しすぎる

## current judgment

current L2 の third checker spike として最も自然なのは
**案 1. capability strengthening floor を third checker spike にする**
である。

理由は次の通り。

1. `e13` / `e20` により capability floor は helper-local spike を置ける最小 coverage に達した
2. row shape が単純で、same-lineage / missing-option と同じ compare pattern を再利用できる
3. shared helper 化の前に capability family 単独の evidence を取っておく方が責務境界を保ちやすい
4. public checker API や detached schema を増やさずに済む

## current cut

current task で actualize してよいのは次である。

- `scripts/current_l2_capability_checker.py`
  - fixture-side `checked_reason_codes` と static gate artifact の actual reason rows を読み、
    capability family だけを filter して exact compare する non-production helper
- `scripts/current_l2_detached_loop.py smoke-capability-checker`
  - 1 fixture の static gate artifact を emit し、その artifact に対して capability third checker spike を回す wrapper

この cut では次を行わない。

- `lib.rs` / `harness.rs` public API への昇格
- same-lineage / missing-option checker と shared helper への統合
- new fixture schema field の導入
- theorem-level invariant proof

## smoke evidence

current smoke では少なくとも次が通っている。

- `e13-malformed-capability-strengthening`
  - `capability_strengthens`
- `e20-malformed-late-capability-strengthening`
  - `capability_strengthens`

どちらも `status: matched` であり、capability family だけを narrow に compare できる。

## 未決事項

- same-lineage / missing-option / capability を later task で shared support helper へ寄せるか
- capability floor をさらに増やす fixture が要るか
- final checker API をどこに置くか
