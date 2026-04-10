# 205 — current L2 theorem line theorem-export-checker-pressure-ready checker-facing contract threshold

## 目的

`specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md`
までを前提に、

- theorem-export-checker-pressure-ready retained bridge に actual checker-facing contract をどこまで近づけるか
- actual checker-facing contract を minimal contract に留めるべきか
- exported checker payload まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の theorem-export-checker-pressure-ready checker-facing contract threshold** であり、
exported checker payload はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- first machine-facing checker candidate は `theorem_export_checker` に限る。
- exported checker payload と checker result materialization family は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_pressure` までは current first choice に入っている。
2. `theorem_export_checker` pressure は symbolic marker に留め、actual checker-facing contract は still 後段に残す cut が固定済みである。
3. `proof_assistant_adapter` contract は second practical consumer line として current retained bridge に入っている。

したがって current 問いは、
**symbolic `theorem_export_checker` pressure の次段として actual checker-facing contract を retained bridge に近づけるなら、minimal checker-facing contract を first cut にするべきか、それとも exported checker payload まで同時に切るべきか**
である。

## 比較観点

1. symbolic checker-facing pressure と actual checker-facing contract の line を narrow に切れるか
2. actual checker-facing contract を current bridge では minimal contract に留め、exported checker payload を still 後段に残せるか
3. theorem-line retained bridge に exported checker payload を premature に押し込まないか
4. next later reopen を exported-checker-payload-pressure comparison へ狭く進められるか

## 比較対象

### 案 1. symbolic `theorem_export_checker` pressure marker を terminal cut にし、actual checker-facing contract も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual checker-facing contract の premature actualization を避けられる

#### 欠点

- symbolic checker-facing pressure と actual checker-facing contract の比較結果が prose 依存に残りやすい
- `theorem_export_checker` が practical checker candidate として actual contract を持つことを current bridge で見えにくい

### 案 2. minimal actual checker-facing contract だけを持つ retained bridge にする

#### 読み

theorem-export-checker-pressure-ready retained bridge に、
exported checker payload を導入せず
**`retained_payload_body_materialization_theorem_export_checker_contract`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_contract = {
    theorem_export_checker_pressure_ref = retained_payload_body_materialization_theorem_export_checker_pressure,
    consumer_class = theorem_export_checker
  }
}
```

#### 利点

- actual checker-facing contract 自体は current bridge で見える
- exported checker payload を still 後段に残せる
- next later reopen を exported-checker-payload-pressure comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- symbolic pressure marker と actual checker-facing contract を誤読されない説明が要る

### 案 3. exported checker payload まで current bridge へ同時に入れる

#### 利点

- checker-facing line の先を一気に concrete にできる
- exported checker payload family へ直接つながりやすい

#### 欠点

- actual checker-facing contract と exported checker payload を同じ reopen で混ぜやすい
- theorem-line retained bridge を exported checker payload family へ既成事実化しやすい
- payload-specific compare / bless / retention line を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal actual checker-facing contract だけを持つ retained bridge にする**
である。

理由は次の通り。

1. symbolic checker-facing pressure の次段として actual checker-facing contract 自体は narrow に橋渡しできる
2. exported checker payload を still 後段に残せる
3. next later reopen を exported-checker-payload-pressure comparison へ狭く進めやすい

## minimal checker-facing-contract-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_contract = {
    theorem_export_checker_pressure_ref = retained_payload_body_materialization_theorem_export_checker_pressure,
    consumer_class = theorem_export_checker
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_contract`

`retained_payload_body_materialization_theorem_export_checker_contract` は、
symbolic `theorem_export_checker` pressure の次段として
`theorem_export_checker` が actual checker-facing contract を持つことを
theorem-side retained bridge で最小限に表す field である。

current task では、この contract を exported checker payload には昇格させない。

## なぜ exported checker payload をまだ入れないか

exported checker payload を current phase で入れると、

- symbolic `theorem_export_checker` pressure
- actual checker-facing contract
- exported checker payload

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず actual checker-facing contract 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-facing-contract-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch = bridge_sketch:e12.theorem_export_checker_pressure_ready,
  retained_payload_body_materialization_theorem_export_checker_contract = {
    theorem_export_checker_pressure_ref = retained_payload_theorem_export_checker_pressure:e12.latest,
    consumer_class = theorem_export_checker
  }
}
```

ここで bridge が示すのは actual checker-facing contract までであり、
exported checker payload まではまだ bridge に入れない。

### example B — witnessed draw checker-facing-contract-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_pressure_ready_sketch = bridge_sketch:sugoroku.draw.theorem_export_checker_pressure_ready,
  retained_payload_body_materialization_theorem_export_checker_contract = {
    theorem_export_checker_pressure_ref = retained_payload_theorem_export_checker_pressure:sugoroku.draw.latest,
    consumer_class = theorem_export_checker
  }
}
```

draw case でも actual checker-facing contract 自体は bridge で追えるが、
exported checker payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual checker-facing contract comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_contract` だけを足す retained bridge である
- exported checker payload は still 後段に残す
- next later reopen は theorem-export-checker-contract-ready exported-checker-payload-pressure comparison である

### not decided

- exported checker payload をどの field / row / payload family で切るか
- `theorem_export_checker` contract を retained bridge のまま維持するか exported checker payload family へ actualize するか
- exported checker payload pressure を later candidate のまま維持する concrete threshold を何とみなすか
