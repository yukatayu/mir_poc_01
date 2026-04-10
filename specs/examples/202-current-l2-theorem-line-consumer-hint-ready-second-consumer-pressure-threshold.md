# 202 — current L2 theorem line consumer-hint-ready second-consumer-pressure threshold

## 目的

`specs/examples/201-current-l2-theorem-line-proof-hint-ready-consumer-hint-threshold.md`
までを前提に、

- consumer-hint-ready retained bridge に second consumer pressure をどこまで近づけるか
- second consumer pressure を symbolic marker に留めるべきか
- `proof_assistant_adapter` actual contract や `theorem_export_checker` pressure まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の consumer-hint-ready second-consumer-pressure threshold** であり、
actual `proof_assistant_adapter` contract と `theorem_export_checker` pressure はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- second practical consumer candidate は `proof_assistant_adapter` に限る。
- actual adapter schema と `theorem_export_checker` line は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_external_contract_consumer_hint` までは current first choice に入っている。
2. `proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate である。
3. consumer-side enrichment family は `goal_text + proof_hint + consumer_hint` までに留める cut が固定済みである。

したがって current 問いは、
**`consumer_hint` enrichment の次段として second consumer pressure を retained bridge に近づけるなら、symbolic pressure marker を first cut にするべきか、それとも actual adapter contract や `theorem_export_checker` pressure まで同時に切るべきか**
である。

## 比較観点

1. first consumer enrichment family と second consumer pressure の line を narrow に切れるか
2. second consumer pressure を symbolic marker に留め、actual adapter contract を still 後段に残せるか
3. theorem-line retained bridge に `theorem_export_checker` pressure を premature に押し込まないか
4. next later reopen を proof-assistant-adapter contract comparison へ狭く進められるか

## 比較対象

### 案 1. `consumer_hint` enrichment を terminal cut にし、second consumer pressure も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- machine-facing pressure の premature actualization を避けられる

#### 欠点

- first consumer line と second consumer pressure の境界が prose 依存に残りやすい
- `proof_assistant_adapter` が second practical candidate であることを current bridge で見えにくい

### 案 2. symbolic second consumer pressure marker だけを持つ retained bridge にする

#### 読み

consumer-hint-ready retained bridge に、
actual adapter contract や `theorem_export_checker` pressure を導入せず
**`retained_payload_body_materialization_external_contract_second_consumer_pressure`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch,
  retained_payload_body_materialization_external_contract_second_consumer_pressure = {
    source_consumer_ref = retained_payload_body_materialization_external_contract_consumer_hint,
    next_consumer_class = proof_assistant_adapter
  }
}
```

#### 利点

- second consumer pressure 自体は current bridge で見える
- actual adapter contract と `theorem_export_checker` pressure を still 後段に残せる
- next later reopen を proof-assistant-adapter contract comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- symbolic marker と actual adapter contract を誤読されない説明が要る

### 案 3. actual `proof_assistant_adapter` contract や `theorem_export_checker` pressure を current bridge へ同時に入れる

#### 利点

- machine-facing line へ直接つながりやすい
- second consumer pressure の先を一気に concrete にできる

#### 欠点

- symbolic pressure marker、actual adapter contract、later checker pressure を同じ reopen で混ぜやすい
- theorem-line retained bridge を machine-facing contract へ既成事実化しやすい
- `theorem_export_checker` pressure を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic second consumer pressure marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. first consumer enrichment family の次段として second consumer pressure 自体は narrow に橋渡しできる
2. actual adapter contract と `theorem_export_checker` pressure を still 後段に残せる
3. next later reopen を proof-assistant-adapter contract comparison へ狭く進めやすい

## minimal second-consumer-pressure-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch,
  retained_payload_body_materialization_external_contract_second_consumer_pressure = {
    source_consumer_ref = retained_payload_body_materialization_external_contract_consumer_hint,
    next_consumer_class = proof_assistant_adapter
  }
}
```

### `retained_payload_body_materialization_external_contract_second_consumer_pressure`

`retained_payload_body_materialization_external_contract_second_consumer_pressure` は、
`proof_notebook` first consumer line の次段として
`proof_assistant_adapter` pressure が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を actual adapter contract には昇格させない。

## なぜ actual adapter contract をまだ入れないか

actual `proof_assistant_adapter` contract を current phase で入れると、

- first consumer enrichment family
- symbolic second consumer pressure
- actual adapter-facing contract
- `theorem_export_checker` pressure

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず second consumer pressure 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain second-consumer-pressure-ready retained bridge

```text
proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch = bridge_sketch:e12.external_contract_consumer_hint_ready,
  retained_payload_body_materialization_external_contract_second_consumer_pressure = {
    source_consumer_ref = retained_payload_external_contract_consumer_hint:e12.latest,
    next_consumer_class = proof_assistant_adapter
  }
}
```

ここで bridge が示すのは second consumer pressure までであり、
adapter schema まではまだ bridge に入れない。

### example B — witnessed draw second-consumer-pressure-ready retained bridge

```text
proof_notebook_bridge_retained_payload_second_consumer_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch = bridge_sketch:sugoroku.draw.external_contract_consumer_hint_ready,
  retained_payload_body_materialization_external_contract_second_consumer_pressure = {
    source_consumer_ref = retained_payload_external_contract_consumer_hint:sugoroku.draw.latest,
    next_consumer_class = proof_assistant_adapter
  }
}
```

draw case でも second consumer pressure 自体は bridge で追えるが、
actual adapter contract までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で second consumer pressure comparison を切り、current first choice は `retained_payload_body_materialization_external_contract_second_consumer_pressure` だけを足す retained bridge である
- actual `proof_assistant_adapter` contract と `theorem_export_checker` pressure は still 後段に残す
- next later reopen は second-consumer-pressure-ready proof-assistant-adapter contract comparison である

### not decided

- actual `proof_assistant_adapter` contract をどの field / row bundle で切るか
- `proof_assistant_adapter` pressure を retained bridge のまま維持するか adapter-facing contract へ actualize するか
- `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何とみなすか
