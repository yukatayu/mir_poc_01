# 199 — current L2 theorem line actual-external-contract-ready consumer-specific external contract payload threshold

## 目的

`specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
までを前提に、

- actual-external-contract-ready retained bridge に consumer-specific external contract payload をどこまで近づけるか
- consumer-specific payload を minimal payload に留めるべきか
- `proof_hint` / `consumer_hint` や second consumer pressure まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の actual-external-contract-ready consumer-specific external contract payload threshold** であり、
`proof_assistant_adapter` / `theorem_export_checker` line はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current actual-external-contract-ready retained bridge を起点にする。
- `proof_hint` / `consumer_hint` と second consumer pressure は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_external_contract` までは current first choice に入っている。
2. first practical consumer class は `proof_notebook` に置く cut が固定済みである。
3. notebook-side lightweight attachment の current first choice は `goal_text` である。

したがって current 問いは、
**actual external contract の次段として consumer-specific external contract payload を current bridge に近づけるなら、minimal payload を first cut にするべきか、それとも `proof_hint` / `consumer_hint` や second consumer pressure まで同時に切るべきか**
である。

## 比較観点

1. actual external contract と consumer-specific payload の line を narrow に切れるか
2. consumer-specific payload を current bridge では minimal payload に留め、`proof_hint` / `consumer_hint` を still 後段に残せるか
3. theorem-line retained bridge に `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に押し込まないか
4. next later reopen を payload enrichment ないし second consumer pressure comparison へ狭く進められるか

## 比較対象

### 案 1. actual external contract を terminal cut にし、consumer-specific payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- notebook-side payload の premature actualization を避けられる

#### 欠点

- consumer-specific payload comparison の結果が prose 依存に残りやすい
- `proof_notebook` first consumer class と `goal_text` attachment の line が current bridge で見えない

### 案 2. minimal consumer-specific external contract payload だけを持つ retained bridge にする

#### 読み

actual-external-contract-ready retained bridge に、
`proof_hint` / `consumer_hint` や second consumer pressure を導入せず
**`retained_payload_body_materialization_external_contract_payload`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch,
  retained_payload_body_materialization_external_contract_payload = {
    external_contract_ref = retained_payload_body_materialization_external_contract,
    goal_text
  }
}
```

#### 利点

- consumer-specific payload 自体は current bridge で見える
- `proof_hint` / `consumer_hint` を still 後段に残せる
- next later reopen を payload enrichment または second consumer pressure comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- minimal payload と richer notebook / adapter payload を誤読されない説明が要る

### 案 3. `proof_hint` / `consumer_hint` や second consumer pressure を current bridge へ同時に入れる

#### 利点

- notebook-side payload を一気に concrete にできる
- `proof_assistant_adapter` line へ直接つながりやすい

#### 欠点

- minimal payload と richer payload を同じ reopen で混ぜやすい
- `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に呼びやすい
- theorem-line retained bridge を actual exported contract へ既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal consumer-specific external contract payload だけを持つ retained bridge にする**
である。

理由は次の通り。

1. actual external contract の次段として notebook-side minimal payload 自体は narrow に橋渡しできる
2. `proof_hint` / `consumer_hint` や second consumer pressure を still 後段に残せる
3. next later reopen を payload enrichment または second consumer pressure comparison へ狭く進めやすい

## minimal consumer-specific-external-contract-payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch,
  retained_payload_body_materialization_external_contract_payload = {
    external_contract_ref = retained_payload_body_materialization_external_contract,
    goal_text
  }
}
```

### `retained_payload_body_materialization_external_contract_payload`

`retained_payload_body_materialization_external_contract_payload` は、
`proof_notebook` first consumer に向けた minimal consumer-specific payload を
theorem-side retained bridge で最小限に表す field である。

current task では、この payload を `proof_hint` / `consumer_hint` を含む richer payload には昇格させない。

## なぜ `proof_hint` / `consumer_hint` をまだ入れないか

`proof_hint` / `consumer_hint` を current phase で入れると、

- actual external contract
- notebook-side minimal payload
- richer notebook payload
- `proof_assistant_adapter` / `theorem_export_checker` pressure

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず `goal_text` を伴う minimal payload 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain consumer-specific payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch = bridge_sketch:e12.actual_external_contract_ready,
  retained_payload_body_materialization_external_contract_payload = {
    external_contract_ref = retained_payload_external_contract:e12.latest,
    goal_text = "canonical normalization 後も edge mismatch が復元されないことを示す"
  }
}
```

ここで notebook bridge が知るのは minimal payload までであり、
`proof_hint` / `consumer_hint` まではまだ bridge に入れない。

### example B — witnessed draw consumer-specific payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_contract_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch = bridge_sketch:sugoroku.draw.actual_external_contract_ready,
  retained_payload_body_materialization_external_contract_payload = {
    external_contract_ref = retained_payload_external_contract:sugoroku.draw.latest,
    goal_text = "authority draw と witness receipt が replay 時にも一致することを示す"
  }
}
```

draw notebook payload の minimal shape は bridge で追えるが、
`proof_hint` / `consumer_hint` までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で consumer-specific external contract payload comparison を切り、current first choice は `retained_payload_body_materialization_external_contract_payload` だけを足す retained bridge である
- `proof_hint` / `consumer_hint` と second consumer pressure は still 後段に残す
- next later reopen は external-contract-payload-ready payload enrichment / second-consumer-pressure comparison である

### not decided

- richer consumer-specific payload をどの field で切るか
- minimal payload を retained bridge のまま維持するか richer notebook payload へ actualize するか
- `proof_assistant_adapter` / `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何とみなすか
