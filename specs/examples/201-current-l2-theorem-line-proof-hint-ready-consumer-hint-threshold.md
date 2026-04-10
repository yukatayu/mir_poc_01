# 201 — current L2 theorem line proof-hint-ready consumer-hint threshold

## 目的

`specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`
までを前提に、

- proof-hint-ready retained bridge に `consumer_hint` をどこまで近づけるか
- `consumer_hint` を minimal consumer-side enrichment に留めるべきか
- second consumer pressure まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の proof-hint-ready consumer-hint threshold** であり、
second consumer pressure はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current proof-hint-ready retained bridge を起点にする。
- second consumer pressure は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_external_contract_proof_hint` までは current first choice に入っている。
2. `proof_notebook` first consumer の current enrichment は `goal_text + proof_hint` までに留める cut が固定済みである。
3. second consumer pressure は still 後段に残している。

したがって current 問いは、
**`proof_hint` enrichment の次段として `consumer_hint` を current bridge に近づけるなら、minimal consumer-side enrichment を first cut にするべきか、それとも second consumer pressure まで同時に切るべきか**
である。

## 比較観点

1. `proof_hint` enrichment と `consumer_hint` enrichment の line を narrow に切れるか
2. `consumer_hint` を current bridge では minimal enrichment に留め、second consumer pressure を still 後段に残せるか
3. theorem-line retained bridge に `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に押し込まないか
4. next later reopen を second consumer pressure comparison へ狭く進められるか

## 比較対象

### 案 1. `proof_hint` enrichment を terminal cut にし、`consumer_hint` も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- `consumer_hint` の premature actualization を避けられる

#### 欠点

- `proof_hint` と `consumer_hint` の比較結果が prose 依存に残りやすい
- proof notebook 向け minimal consumer-side guidance の増分を current bridge で扱えない

### 案 2. minimal `consumer_hint` enrichment だけを持つ retained bridge にする

#### 読み

proof-hint-ready retained bridge に、
second consumer pressure を導入せず
**`retained_payload_body_materialization_external_contract_consumer_hint`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch,
  retained_payload_body_materialization_external_contract_consumer_hint = {
    external_contract_proof_hint_ref = retained_payload_body_materialization_external_contract_proof_hint,
    consumer_hint
  }
}
```

#### 利点

- `consumer_hint` enrichment 自体は current bridge で見える
- second consumer pressure を still 後段に残せる
- next later reopen を second consumer pressure comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- `consumer_hint` と second consumer pressure を誤読されない説明が要る

### 案 3. second consumer pressure を current bridge へ同時に入れる

#### 利点

- `proof_assistant_adapter` line へ直接つながりやすい
- second consumer pressure の compare を先に片付けられる

#### 欠点

- `consumer_hint` enrichment と second consumer pressure を同じ reopen で混ぜやすい
- theorem-line retained bridge を machine-facing contract へ既成事実化しやすい
- `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal `consumer_hint` enrichment だけを持つ retained bridge にする**
である。

理由は次の通り。

1. `proof_hint` の次段として proof notebook 向け minimal consumer-side enrichment 自体は narrow に橋渡しできる
2. second consumer pressure を still 後段に残せる
3. next later reopen を second consumer pressure comparison へ狭く進めやすい

## minimal consumer-hint-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch,
  retained_payload_body_materialization_external_contract_consumer_hint = {
    external_contract_proof_hint_ref = retained_payload_body_materialization_external_contract_proof_hint,
    consumer_hint
  }
}
```

### `retained_payload_body_materialization_external_contract_consumer_hint`

`retained_payload_body_materialization_external_contract_consumer_hint` は、
`proof_notebook` first consumer に向けた `consumer_hint` enrichment を
theorem-side retained bridge で最小限に表す field である。

current task では、この enrichment を second consumer pressure には昇格させない。

## なぜ second consumer pressure をまだ入れないか

second consumer pressure を current phase で入れると、

- `goal_text` payload
- `proof_hint` enrichment
- `consumer_hint` enrichment
- `proof_assistant_adapter` / `theorem_export_checker` pressure

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず `consumer_hint` を伴う minimal enrichment 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain consumer-hint-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch = bridge_sketch:e12.external_contract_proof_hint_ready,
  retained_payload_body_materialization_external_contract_consumer_hint = {
    external_contract_proof_hint_ref = retained_payload_external_contract_proof_hint:e12.latest,
    consumer_hint = "review notebook では lineage edge ごとに static gate 証拠を横に並べて確認する"
  }
}
```

ここで notebook bridge が知るのは `consumer_hint` までであり、
`proof_assistant_adapter` pressure まではまだ bridge に入れない。

### example B — witnessed draw consumer-hint-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_contract_consumer_hint_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_contract_proof_hint_ready_sketch = bridge_sketch:sugoroku.draw.external_contract_proof_hint_ready,
  retained_payload_body_materialization_external_contract_consumer_hint = {
    external_contract_proof_hint_ref = retained_payload_external_contract_proof_hint:sugoroku.draw.latest,
    consumer_hint = "receipt 画面では roll id と authority commit id を同時に見比べる"
  }
}
```

draw notebook payload の consumer hint は bridge で追えるが、
second consumer pressure までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で `consumer_hint` enrichment comparison を切り、current first choice は `retained_payload_body_materialization_external_contract_consumer_hint` だけを足す retained bridge である
- second consumer pressure は still 後段に残す
- next later reopen は consumer-hint-ready second-consumer-pressure comparison である

### not decided

- second consumer pressure をどの field / row / consumer split で切るか
- `consumer_hint` enrichment を retained bridge のまま維持するか richer notebook payload へ actualize するか
- `proof_assistant_adapter` / `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何とみなすか
