# 203 — current L2 theorem line second-consumer-pressure-ready proof-assistant-adapter contract threshold

## 目的

`specs/examples/202-current-l2-theorem-line-consumer-hint-ready-second-consumer-pressure-threshold.md`
までを前提に、

- second-consumer-pressure-ready retained bridge に actual `proof_assistant_adapter` contract をどこまで近づけるか
- actual adapter-facing contract を minimal contract に留めるべきか
- `theorem_export_checker` pressure まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の second-consumer-pressure-ready proof-assistant-adapter contract threshold** であり、
`theorem_export_checker` pressure はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- second practical consumer candidate は `proof_assistant_adapter` に限る。
- `theorem_export_checker` pressure と actual checker-facing contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_external_contract_second_consumer_pressure` までは current first choice に入っている。
2. second consumer pressure は symbolic marker に留め、actual adapter-facing contract は still 後段に残す cut が固定済みである。
3. `proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate である。

したがって current 問いは、
**second consumer pressure marker の次段として actual `proof_assistant_adapter` contract を retained bridge に近づけるなら、minimal adapter-facing contract を first cut にするべきか、それとも `theorem_export_checker` pressure まで同時に切るべきか**
である。

## 比較観点

1. symbolic second consumer pressure と actual adapter-facing contract の line を narrow に切れるか
2. actual adapter-facing contract を current bridge では minimal contract に留め、`theorem_export_checker` pressure を still 後段に残せるか
3. theorem-line retained bridge に checker-facing pressure を premature に押し込まないか
4. next later reopen を theorem-export-checker-pressure comparison へ狭く進められるか

## 比較対象

### 案 1. second consumer pressure marker を terminal cut にし、actual adapter-facing contract も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual adapter-facing contract の premature actualization を避けられる

#### 欠点

- second consumer pressure と actual adapter-facing contract の比較結果が prose 依存に残りやすい
- `proof_assistant_adapter` が second practical consumer の actual contract 候補であることを current bridge で見えにくい

### 案 2. minimal actual `proof_assistant_adapter` contract だけを持つ retained bridge にする

#### 読み

second-consumer-pressure-ready retained bridge に、
`theorem_export_checker` pressure を導入せず
**`retained_payload_body_materialization_proof_assistant_adapter_contract`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch,
  retained_payload_body_materialization_proof_assistant_adapter_contract = {
    second_consumer_pressure_ref = retained_payload_body_materialization_external_contract_second_consumer_pressure,
    consumer_class = proof_assistant_adapter
  }
}
```

#### 利点

- actual adapter-facing contract 自体は current bridge で見える
- `theorem_export_checker` pressure を still 後段に残せる
- next later reopen を theorem-export-checker-pressure comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- symbolic pressure marker と actual adapter-facing contract を誤読されない説明が要る

### 案 3. `theorem_export_checker` pressure まで current bridge へ同時に入れる

#### 利点

- second consumer line の先を一気に concrete にできる
- machine-facing checker line へ直接つながりやすい

#### 欠点

- actual adapter-facing contract と checker-facing pressure を同じ reopen で混ぜやすい
- theorem-line retained bridge を checker-facing contract へ既成事実化しやすい
- second practical consumer と later checker pressure の順番を premature に潰しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal actual `proof_assistant_adapter` contract だけを持つ retained bridge にする**
である。

理由は次の通り。

1. symbolic second consumer pressure の次段として actual adapter-facing contract 自体は narrow に橋渡しできる
2. `theorem_export_checker` pressure を still 後段に残せる
3. next later reopen を theorem-export-checker-pressure comparison へ狭く進めやすい

## minimal proof-assistant-adapter-contract-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch,
  retained_payload_body_materialization_proof_assistant_adapter_contract = {
    second_consumer_pressure_ref = retained_payload_body_materialization_external_contract_second_consumer_pressure,
    consumer_class = proof_assistant_adapter
  }
}
```

### `retained_payload_body_materialization_proof_assistant_adapter_contract`

`retained_payload_body_materialization_proof_assistant_adapter_contract` は、
`proof_assistant_adapter` が second practical consumer candidate として actual adapter-facing contract を持つことを
theorem-side retained bridge で最小限に表す field である。

current task では、この contract を checker-facing pressure には昇格させない。

## なぜ `theorem_export_checker` pressure をまだ入れないか

`theorem_export_checker` pressure を current phase で入れると、

- symbolic second consumer pressure
- actual adapter-facing contract
- checker-facing pressure

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず actual `proof_assistant_adapter` contract 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain proof-assistant-adapter-contract-ready retained bridge

```text
proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch = bridge_sketch:e12.second_consumer_pressure_ready,
  retained_payload_body_materialization_proof_assistant_adapter_contract = {
    second_consumer_pressure_ref = retained_payload_external_contract_second_consumer_pressure:e12.latest,
    consumer_class = proof_assistant_adapter
  }
}
```

ここで bridge が示すのは actual adapter-facing contract までであり、
checker-facing pressure まではまだ bridge に入れない。

### example B — witnessed draw proof-assistant-adapter-contract-ready retained bridge

```text
proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch = bridge_sketch:sugoroku.draw.second_consumer_pressure_ready,
  retained_payload_body_materialization_proof_assistant_adapter_contract = {
    second_consumer_pressure_ref = retained_payload_external_contract_second_consumer_pressure:sugoroku.draw.latest,
    consumer_class = proof_assistant_adapter
  }
}
```

draw case でも actual adapter-facing contract 自体は bridge で追えるが、
checker-facing pressure までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual `proof_assistant_adapter` contract comparison を切り、current first choice は `retained_payload_body_materialization_proof_assistant_adapter_contract` だけを足す retained bridge である
- `theorem_export_checker` pressure は still 後段に残す
- next later reopen は proof-assistant-adapter-contract-ready theorem-export-checker-pressure comparison である

### not decided

- `theorem_export_checker` pressure をどの field / row / consumer split で切るか
- actual `proof_assistant_adapter` contract を retained bridge のまま維持するか checker-facing pressure へ actualize するか
- `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何とみなすか
