# 206 — current L2 theorem line theorem-export-checker-contract-ready exported-checker-payload-pressure threshold

## 目的

`specs/examples/205-current-l2-theorem-line-theorem-export-checker-pressure-ready-checker-facing-contract-threshold.md`
までを前提に、

- theorem-export-checker-contract-ready retained bridge に exported checker payload pressure をどこまで近づけるか
- exported checker payload pressure を symbolic marker に留めるべきか
- actual exported checker payload まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の theorem-export-checker-contract-ready exported-checker-payload-pressure threshold** であり、
actual exported checker payload はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- actual exported checker payload と checker result materialization family は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_contract` までは current first choice に入っている。
2. actual checker-facing contract は current retained bridge に入るが、exported checker payload は still 後段に残す cut が固定済みである。
3. `theorem_export_checker_payload` family は next practical payload candidate として最も近い。

したがって current 問いは、
**actual checker-facing contract の次段として exported checker payload pressure を retained bridge に近づけるなら、symbolic pressure marker を first cut にするべきか、それとも actual exported checker payload まで同時に切るべきか**
である。

## 比較観点

1. actual checker-facing contract と exported checker payload pressure の line を narrow に切れるか
2. exported checker payload pressure を symbolic marker に留め、actual exported checker payload を still 後段に残せるか
3. theorem-line retained bridge に actual exported checker payload を premature に押し込まないか
4. next later reopen を actual exported checker payload comparison へ狭く進められるか

## 比較対象

### 案 1. actual checker-facing contract を terminal cut にし、exported checker payload pressure も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- exported checker payload pressure の premature actualization を避けられる

#### 欠点

- actual checker-facing contract と exported checker payload pressure の比較結果が prose 依存に残りやすい
- `theorem_export_checker_payload` family が next practical candidate であることを current bridge で見えにくい

### 案 2. symbolic exported checker payload pressure marker だけを持つ retained bridge にする

#### 読み

theorem-export-checker-contract-ready retained bridge に、
actual exported checker payload を導入せず
**`retained_payload_body_materialization_theorem_export_checker_payload_pressure`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_payload_pressure = {
    theorem_export_checker_contract_ref = retained_payload_body_materialization_theorem_export_checker_contract,
    next_payload_family = theorem_export_checker_payload
  }
}
```

#### 利点

- exported checker payload pressure 自体は current bridge で見える
- actual exported checker payload を still 後段に残せる
- next later reopen を actual exported checker payload comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- symbolic pressure marker と actual exported checker payload を誤読されない説明が要る

### 案 3. actual exported checker payload まで current bridge へ同時に入れる

#### 利点

- checker-facing line の先を一気に concrete にできる
- checker result materialization family へ直接つながりやすい

#### 欠点

- exported checker payload pressure と actual exported checker payload を同じ reopen で混ぜやすい
- theorem-line retained bridge を actual exported checker payload family へ既成事実化しやすい
- payload-specific compare / retention line を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic exported checker payload pressure marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. actual checker-facing contract の次段として exported checker payload pressure 自体は narrow に橋渡しできる
2. actual exported checker payload を still 後段に残せる
3. next later reopen を actual exported checker payload comparison へ狭く進めやすい

## minimal exported-checker-payload-pressure-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_payload_pressure = {
    theorem_export_checker_contract_ref = retained_payload_body_materialization_theorem_export_checker_contract,
    next_payload_family = theorem_export_checker_payload
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_payload_pressure`

`retained_payload_body_materialization_theorem_export_checker_payload_pressure` は、
actual checker-facing contract の次段として
`theorem_export_checker_payload` family が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を actual exported checker payload には昇格させない。

## なぜ actual exported checker payload をまだ入れないか

actual exported checker payload を current phase で入れると、

- actual checker-facing contract
- symbolic exported checker payload pressure
- actual exported checker payload

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず exported checker payload pressure 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain exported-checker-payload-pressure-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch = bridge_sketch:e12.theorem_export_checker_contract_ready,
  retained_payload_body_materialization_theorem_export_checker_payload_pressure = {
    theorem_export_checker_contract_ref = retained_payload_theorem_export_checker_contract:e12.latest,
    next_payload_family = theorem_export_checker_payload
  }
}
```

ここで bridge が示すのは exported checker payload pressure までであり、
actual exported checker payload まではまだ bridge に入れない。

### example B — witnessed draw exported-checker-payload-pressure-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_contract_ready_sketch = bridge_sketch:sugoroku.draw.theorem_export_checker_contract_ready,
  retained_payload_body_materialization_theorem_export_checker_payload_pressure = {
    theorem_export_checker_contract_ref = retained_payload_theorem_export_checker_contract:sugoroku.draw.latest,
    next_payload_family = theorem_export_checker_payload
  }
}
```

draw case でも exported checker payload pressure 自体は bridge で追えるが、
actual exported checker payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で exported checker payload pressure comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_payload_pressure` だけを足す retained bridge である
- actual exported checker payload は still 後段に残す
- next later reopen は theorem-export-checker-payload-pressure-ready actual-exported-checker-payload comparison である

### not decided

- actual exported checker payload をどの field / row / payload family で切るか
- `theorem_export_checker_payload` pressure を retained bridge のまま維持するか actual exported checker payload へ actualize するか
- checker result materialization family をどの concrete threshold で呼ぶか
