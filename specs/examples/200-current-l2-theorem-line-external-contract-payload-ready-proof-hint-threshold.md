# 200 — current L2 theorem line external-contract-payload-ready proof-hint threshold

## 目的

`specs/examples/199-current-l2-theorem-line-actual-external-contract-ready-consumer-specific-external-contract-payload-threshold.md`
までを前提に、

- external-contract-payload-ready retained bridge に `proof_hint` をどこまで近づけるか
- `proof_hint` を minimal payload enrichment に留めるべきか
- `consumer_hint` や second consumer pressure まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の external-contract-payload-ready proof-hint threshold** であり、
`consumer_hint` と second consumer pressure はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current external-contract-payload-ready retained bridge を起点にする。
- `consumer_hint` と second consumer pressure は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_external_contract_payload` までは current first choice に入っている。
2. `proof_notebook` first consumer の current minimal payload は `goal_text` に留める cut が固定済みである。
3. `proof_hint` / `consumer_hint` は still 後段に残している。

したがって current 問いは、
**consumer-specific external contract payload の次段として `proof_hint` を current bridge に近づけるなら、minimal enrichment を first cut にするべきか、それとも `consumer_hint` や second consumer pressure まで同時に切るべきか**
である。

## 比較観点

1. `goal_text` payload と `proof_hint` enrichment の line を narrow に切れるか
2. `proof_hint` を current bridge では minimal enrichment に留め、`consumer_hint` を still 後段に残せるか
3. theorem-line retained bridge に `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に押し込まないか
4. next later reopen を `consumer_hint` enrichment または second consumer pressure comparison へ狭く進められるか

## 比較対象

### 案 1. minimal consumer-specific payload を terminal cut にし、`proof_hint` も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- `proof_hint` の premature actualization を避けられる

#### 欠点

- `goal_text` payload と `proof_hint` enrichment の比較結果が prose 依存に残りやすい
- theorem worker 向け説明力の増分を current bridge で扱えない

### 案 2. minimal `proof_hint` enrichment だけを持つ retained bridge にする

#### 読み

external-contract-payload-ready retained bridge に、
`consumer_hint` や second consumer pressure を導入せず
**`retained_payload_body_materialization_external_contract_proof_hint`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch,
  retained_payload_body_materialization_external_contract_proof_hint = {
    external_contract_payload_ref = retained_payload_body_materialization_external_contract_payload,
    proof_hint
  }
}
```

#### 利点

- `proof_hint` enrichment 自体は current bridge で見える
- `consumer_hint` を still 後段に残せる
- next later reopen を `consumer_hint` enrichment または second consumer pressure comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- `proof_hint` と consumer-specific adapter pressure を誤読されない説明が要る

### 案 3. `consumer_hint` や second consumer pressure を current bridge へ同時に入れる

#### 利点

- notebook payload を一気に richer にできる
- `proof_assistant_adapter` line へ直接つながりやすい

#### 欠点

- `proof_hint` enrichment と `consumer_hint` / second consumer pressure を同じ reopen で混ぜやすい
- theorem-line retained bridge を actual exported / adapter-facing contract へ既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal `proof_hint` enrichment だけを持つ retained bridge にする**
である。

理由は次の通り。

1. `goal_text` payload の次段として theorem worker 向け minimal enrichment 自体は narrow に橋渡しできる
2. `consumer_hint` と second consumer pressure を still 後段に残せる
3. next later reopen を `consumer_hint` enrichment または second consumer pressure comparison へ狭く進めやすい

## minimal proof-hint-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch,
  retained_payload_body_materialization_external_contract_proof_hint = {
    external_contract_payload_ref = retained_payload_body_materialization_external_contract_payload,
    proof_hint
  }
}
```

### `retained_payload_body_materialization_external_contract_proof_hint`

`retained_payload_body_materialization_external_contract_proof_hint` は、
`proof_notebook` first consumer に向けた `proof_hint` enrichment を
theorem-side retained bridge で最小限に表す field である。

current task では、この enrichment を `consumer_hint` を含む richer payload には昇格させない。

## なぜ `consumer_hint` をまだ入れないか

`consumer_hint` を current phase で入れると、

- `goal_text` payload
- `proof_hint` enrichment
- `consumer_hint` enrichment
- second consumer pressure

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず `proof_hint` を伴う minimal enrichment 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain proof-hint-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch = bridge_sketch:e12.external_contract_payload_ready,
  retained_payload_body_materialization_external_contract_proof_hint = {
    external_contract_payload_ref = retained_payload_external_contract_payload:e12.latest,
    proof_hint = "lineage edge の消失箇所だけを辿れば十分"
  }
}
```

ここで notebook bridge が知るのは `proof_hint` までであり、
`consumer_hint` まではまだ bridge に入れない。

### example B — witnessed draw proof-hint-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch = bridge_sketch:sugoroku.draw.external_contract_payload_ready,
  retained_payload_body_materialization_external_contract_proof_hint = {
    external_contract_payload_ref = retained_payload_external_contract_payload:sugoroku.draw.latest,
    proof_hint = "authority draw と witness receipt を同じ roll id で照合する"
  }
}
```

draw notebook payload の proof hint は bridge で追えるが、
`consumer_hint` までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で `proof_hint` enrichment comparison を切り、current first choice は `retained_payload_body_materialization_external_contract_proof_hint` だけを足す retained bridge である
- `consumer_hint` と second consumer pressure は still 後段に残す
- next later reopen は proof-hint-ready consumer-hint / second-consumer-pressure comparison である

### not decided

- `consumer_hint` enrichment をどの field で切るか
- `proof_hint` enrichment を retained bridge のまま維持するか richer payload へ actualize するか
- `proof_assistant_adapter` / `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何とみなすか
