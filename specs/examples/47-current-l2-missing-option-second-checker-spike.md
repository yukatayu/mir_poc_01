# 47 — current L2 missing-option second checker spike

## 目的

この文書は、same-lineage first checker spike の次段として
**missing-option structure floor を second checker spike として helper-local に切り出してよいか**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- same-lineage first spike の次に missing-option family を切る順序判断
- その spike を current helper boundary を壊さずにどこへ置くか
- detached validation loop でどの smoke まで回してよいか

という最小 cut だけである。

## current 前提

current L2 では次が成立している。

- same-lineage floor は helper-local first checker spike として smoke 済みである
- static-only corpus baseline では、same-lineage floor `4`、capability floor `1`、missing-option structure floor `3` の coverage が見えている
- missing-option family は stable cluster 8 kind の一部として `checked_reason_codes` と detached `reason_codes` に actualize 済みである
- capability floor は singleton かつ coverage `1` に留まる

したがって current 問いは、
**same-lineage の次に actualize する spike は missing-option family と capability family のどちらが自然か**
である。

## 比較観点

1. current corpus coverage が enough か
2. fixture-side `checked_reason_codes` と detached static gate `reason_codes` をそのまま使えるか
3. second spike として cluster まとまりが十分あるか
4. capability floor の coverage を厚くする task より先に進めてよいか

## 比較対象

### 案 1. missing-option structure floor を second spike にする

- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

だけを narrow に compare する helper-local checker spike を置く。

#### 利点

- coverage が `3` あり、cluster まとまりもある
- same-lineage first spike と同じく、fixture-side row と actual row を filter するだけで構成できる
- capability floor を急いで public checker 側へ押し込まなくて済む

#### 欠点

- row shape が `head` / `option` / `scope` に分かれているため、same-lineage より説明は少し増える

### 案 2. capability strengthening floor を second spike にする

- `capability_strengthens` だけを先に切り出す。

#### 利点

- prohibition としての語彙は単純である

#### 欠点

- coverage が `1` と薄い
- singleton family のため、second spike としても evidence 密度が弱い

### 案 3. capability floor の coverage を増やしてから second spike を決める

#### 利点

- capability family の evidence density は改善する

#### 欠点

- already available な missing-option family の actual helper cut が遅れる
- current mainline の narrow progression が鈍る

## current judgment

current L2 の second checker spike として最も自然なのは
**案 1. missing-option structure floor を second spike にする**
である。

理由は次の通り。

1. current corpus coverage が `3` と second spike として十分ある
2. same-lineage first spike と同じ helper-local compare pattern を再利用できる
3. capability floor は coverage `1` のため、先に actual helper cut を広げるより evidence を厚くする task に送る方が自然である

## current cut

current task で actualize してよいのは次である。

- `scripts/current_l2_missing_option_checker.py`
  - fixture-side `checked_reason_codes` と static gate artifact の actual reason rows を読み、
    missing-option family だけを filter して exact compare する non-production helper
- `scripts/current_l2_detached_loop.py smoke-missing-option-checker`
  - 1 fixture の static gate artifact を emit し、その artifact に対して missing-option checker spike を回す wrapper

この cut では次を行わない。

- `lib.rs` / `harness.rs` public API への昇格
- capability checker spike との統合
- new fixture schema field の導入
- theorem-level invariant proof

## smoke evidence

current smoke では少なくとも次が通っている。

- `e16-malformed-missing-chain-head-option`
  - `missing_chain_head_option`
- `e17-malformed-missing-predecessor-option`
  - `missing_predecessor_option`

どちらも `status: matched` であり、missing-option family だけを narrow に compare できる。

## 未決事項

- `e18-malformed-missing-successor-option` まで routine smoke として常設するか
- capability floor coverage をどの fixture で増やすか
- missing-option checker spike を later task で shared support helper に寄せるか
- final checker API をどこに置くか
