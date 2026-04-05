# 46 — current L2 same-lineage first checker spike

## 目的

この文書は、first checker cut 候補 cluster のうち
**same-lineage static evidence floor を最初の actual checker spike として helper-local に切り出してよいか**
を narrow に整理する。

ここで固定するのは final checker API ではない。
固定するのは、

- first spike をどの cluster から始めるか
- その spike を current helper boundary を壊さずにどこへ置くか
- detached validation loop でどの smoke まで回してよいか

という最小 cut だけである。

## current 前提

current L2 では次が成立している。

- first checker cut の候補 cluster として、same-lineage floor / capability floor / missing-option structure floor が整理済みである
- static-only corpus baseline では、same-lineage floor `4`、capability floor `1`、missing-option structure floor `3` の coverage が見えている
- `checked_reasons` と `checked_reason_codes` は additive coexistence のままであり、stable cluster 8 fixture は zero follow-up で aligned している
- detached static gate artifact は `checker_core.static_verdict` / `checker_core.reasons` / helper-local `detached_noncore.reason_codes` を持つ

したがって current 問いは、
**actual checker helper を始めるなら same-lineage / capability / missing-option のどれから切るのが最小か**
である。

## 比較観点

1. current corpus coverage が十分にあるか
2. fixture-side `checked_reason_codes` と detached static gate `reason_codes` をそのまま使えるか
3. helper-local spike に留めやすく、public API と誤読されにくいか
4. parser boundary / first checker cut / theorem prover boundary の順序を壊さないか

## 比較対象

### 案 1. same-lineage static evidence floor から始める

- `missing_lineage_assertion`
- `lineage_assertion_edge_mismatch`
- `declared_target_missing`
- `declared_target_mismatch`

だけを narrow に compare する helper-local checker spike を置く。

#### 利点

- corpus coverage が `4` と current cluster で最も厚い
- kind row が predecessor / successor pair を中心にまとまっており、cluster 境界が読みやすい
- `checked_reason_codes` と detached `reason_codes` の stable row をそのまま filter できる
- first checker cut の same-lineage evidence floor と自然に対応する

#### 欠点

- malformed / underdeclared split 自体は別 axis に残る
- current helper は kind row の exact equality compare に留まり、より rich な static judgment ではない

### 案 2. missing-option structure floor から始める

- `missing_chain_head_option`
- `missing_predecessor_option`
- `missing_successor_option`

を先に切り出す。

#### 利点

- fixture coverage が `3` と比較的ある
- missing-option family がまとまっている

#### 欠点

- same-lineage evidence floor の方が first checker cut entry criteria の中心に近い
- row shape が `head` / `option` / `scope` で分かれ、最初の spike としては pair-based family より説明が増える

### 案 3. capability strengthening floor から始める

- `capability_strengthens` だけを先に切り出す。

#### 利点

- prohibition としては単純である

#### 欠点

- coverage が `1` と薄い
- singleton family のため、first spike の代表例としては弱い

## current judgment

current L2 の first checker spike として最も自然なのは
**案 1. same-lineage static evidence floor から始める**
である。

理由は次の通り。

1. current corpus coverage が `4` と最も厚い
2. fixture-side `checked_reason_codes` と detached `reason_codes` の stable row を filter するだけで narrow compare を作れる
3. same-lineage evidence floor は first checker cut entry criteria の中心 cluster であり、parser boundary / theorem prover boundary との handoff も読みやすい
4. helper-local spike に留める限り、public checker API や final schema を早く固定しなくて済む

## current cut

current task で actualize してよいのは次である。

- `scripts/current_l2_same_lineage_checker.py`
  - fixture-side `checked_reason_codes` と static gate artifact の actual reason rows を読み、
    same-lineage family だけを filter して exact compare する non-production helper
- `scripts/current_l2_detached_loop.py smoke-same-lineage-checker`
  - 1 fixture の static gate artifact を emit し、その artifact に対して same-lineage checker spike を回す wrapper

この cut では次を行わない。

- `lib.rs` / `harness.rs` public API への昇格
- new fixture schema field の導入
- capability / missing-option checker spike との統合
- theorem-level invariant proof

## smoke evidence

current smoke では少なくとも次が通っている。

- `e4-malformed-lineage`
  - `lineage_assertion_edge_mismatch`
- `e12-underdeclared-target-missing`
  - `declared_target_missing`

どちらも `status: matched` であり、same-lineage family だけを narrow に compare できる。

## 未決事項

- same-lineage checker spike を later task で `examples/support/` 共有 helper へ寄せるか
- missing-option structure floor を次点 spike として続けるか
- capability floor coverage を増やしてから checker helper cut を広げるか
- final checker API をどこに置くか
