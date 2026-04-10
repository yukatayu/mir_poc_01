# 204 — current L2 theorem line proof-assistant-adapter-contract-ready theorem-export-checker-pressure threshold

## 目的

`specs/examples/203-current-l2-theorem-line-second-consumer-pressure-ready-proof-assistant-adapter-contract-threshold.md`
までを前提に、

- proof-assistant-adapter-contract-ready retained bridge に `theorem_export_checker` pressure をどこまで近づけるか
- checker-facing pressure を symbolic marker に留めるべきか
- actual checker-facing contract まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の proof-assistant-adapter-contract-ready theorem-export-checker-pressure threshold** であり、
actual checker-facing contract はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- first machine-facing checker candidate は `theorem_export_checker` に限る。
- actual checker-facing contract と exported checker payload は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_proof_assistant_adapter_contract` までは current first choice に入っている。
2. actual `proof_assistant_adapter` contract は current retained bridge に入るが、checker-facing line は still 後段に残す cut が固定済みである。
3. `theorem_export_checker` は later candidate だが、next practical checker pressure としては最も近い。

したがって current 問いは、
**actual adapter-facing contract の次段として `theorem_export_checker` pressure を retained bridge に近づけるなら、symbolic pressure marker を first cut にするべきか、それとも actual checker-facing contract まで同時に切るべきか**
である。

## 比較観点

1. actual adapter-facing contract と checker-facing pressure の line を narrow に切れるか
2. checker-facing pressure を symbolic marker に留め、actual checker-facing contract を still 後段に残せるか
3. theorem-line retained bridge に checker-facing actual contract を premature に押し込まないか
4. next later reopen を checker-facing contract comparison へ狭く進められるか

## 比較対象

### 案 1. actual `proof_assistant_adapter` contract を terminal cut にし、`theorem_export_checker` pressure も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker-facing pressure の premature actualization を避けられる

#### 欠点

- adapter-facing contract と checker-facing pressure の比較結果が prose 依存に残りやすい
- `theorem_export_checker` が next practical checker pressure であることを current bridge で見えにくい

### 案 2. symbolic `theorem_export_checker` pressure marker だけを持つ retained bridge にする

#### 読み

proof-assistant-adapter-contract-ready retained bridge に、
actual checker-facing contract を導入せず
**`retained_payload_body_materialization_theorem_export_checker_pressure`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_pressure = {
    source_consumer_ref = retained_payload_body_materialization_proof_assistant_adapter_contract,
    next_consumer_class = theorem_export_checker
  }
}
```

#### 利点

- checker-facing pressure 自体は current bridge で見える
- actual checker-facing contract を still 後段に残せる
- next later reopen を checker-facing contract comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- symbolic pressure marker と actual checker-facing contract を誤読されない説明が要る

### 案 3. actual checker-facing contract まで current bridge へ同時に入れる

#### 利点

- machine-facing checker line へ直接つながりやすい
- checker-facing pressure の先を一気に concrete にできる

#### 欠点

- checker-facing pressure と actual checker-facing contract を同じ reopen で混ぜやすい
- theorem-line retained bridge を actual checker-facing contract へ既成事実化しやすい
- exported checker payload pressure を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic `theorem_export_checker` pressure marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. actual adapter-facing contract の次段として checker-facing pressure 自体は narrow に橋渡しできる
2. actual checker-facing contract を still 後段に残せる
3. next later reopen を checker-facing contract comparison へ狭く進めやすい

## minimal theorem-export-checker-pressure-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_pressure = {
    source_consumer_ref = retained_payload_body_materialization_proof_assistant_adapter_contract,
    next_consumer_class = theorem_export_checker
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_pressure`

`retained_payload_body_materialization_theorem_export_checker_pressure` は、
actual `proof_assistant_adapter` contract の次段として `theorem_export_checker` pressure が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を actual checker-facing contract には昇格させない。

## なぜ actual checker-facing contract をまだ入れないか

actual checker-facing contract を current phase で入れると、

- actual `proof_assistant_adapter` contract
- symbolic `theorem_export_checker` pressure
- actual checker-facing contract

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker-facing pressure 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain theorem-export-checker-pressure-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch = bridge_sketch:e12.proof_assistant_adapter_contract_ready,
  retained_payload_body_materialization_theorem_export_checker_pressure = {
    source_consumer_ref = retained_payload_proof_assistant_adapter_contract:e12.latest,
    next_consumer_class = theorem_export_checker
  }
}
```

ここで bridge が示すのは checker-facing pressure までであり、
actual checker-facing contract まではまだ bridge に入れない。

### example B — witnessed draw theorem-export-checker-pressure-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_proof_assistant_adapter_contract_ready_sketch = bridge_sketch:sugoroku.draw.proof_assistant_adapter_contract_ready,
  retained_payload_body_materialization_theorem_export_checker_pressure = {
    source_consumer_ref = retained_payload_proof_assistant_adapter_contract:sugoroku.draw.latest,
    next_consumer_class = theorem_export_checker
  }
}
```

draw case でも checker-facing pressure 自体は bridge で追えるが、
actual checker-facing contract までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で `theorem_export_checker` pressure comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_pressure` だけを足す retained bridge である
- actual checker-facing contract は still 後段に残す
- next later reopen は theorem-export-checker-pressure-ready checker-facing contract comparison である

### not decided

- actual checker-facing contract をどの field / row / consumer split で切るか
- `theorem_export_checker` pressure を retained bridge のまま維持するか checker-facing contract へ actualize するか
- exported checker payload pressure を later candidate のまま維持する concrete threshold を何とみなすか
