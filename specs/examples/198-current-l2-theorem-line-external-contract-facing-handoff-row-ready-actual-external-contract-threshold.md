# 198 — current L2 theorem line external-contract-facing-handoff-row-ready actual external contract threshold

## 目的

`specs/examples/197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md`
までを前提に、

- external-contract-facing-handoff-row-ready retained bridge に actual external contract をどこまで近づけるか
- actual external contract を theorem-side retained bridge では minimal contract に留めるべきか
- consumer-specific external contract payload まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の external-contract-facing-handoff-row-ready actual external contract threshold** であり、
consumer-specific external contract payload はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current external-contract-facing-handoff-row-ready retained bridge を起点にする。
- consumer-specific external contract payload と public checker contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_external_handoff_row` までは current first choice に入っている。
2. first practical consumer class は `proof_notebook` に置く cut が固定済みである。
3. actual external contract は still 後段に残している。

したがって current 問いは、
**external-contract-facing handoff row の次段として actual external contract を current bridge に近づけるなら、minimal actual contract を first cut にするべきか、それとも consumer-specific external contract payload まで同時に切るべきか**
である。

## 比較観点

1. external-contract-facing handoff row と actual external contract の line を narrow に切れるか
2. actual external contract を current bridge では minimal contract に留め、consumer-specific payload を still 後段に残せるか
3. theorem-side retained bridge に `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に押し込まないか
4. next later reopen を consumer-specific external contract payload comparison へ狭く進められるか

## 比較対象

### 案 1. external-contract-facing handoff row を terminal cut にし、actual external contract も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual external contract の premature actualization を避けられる

#### 欠点

- actual external contract comparison の結果が prose 依存に残りやすい
- `proof_notebook` first consumer class と actual external handoff line の接続が current bridge で見えない

### 案 2. minimal actual external contract だけを持つ retained bridge にする

#### 読み

external-contract-facing-handoff-row-ready retained bridge に、
consumer-specific external contract payload を導入せず
**`retained_payload_body_materialization_external_contract`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch,
  retained_payload_body_materialization_external_contract = {
    consumer_class = proof_notebook,
    handoff_row_ref = retained_payload_body_materialization_external_handoff_row
  }
}
```

#### 利点

- actual external contract 自体は current bridge で見える
- consumer-specific payload を still 後段に残せる
- next later reopen を consumer-specific external contract payload comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- actual external contract と consumer-specific payload を誤読されない説明が要る

### 案 3. consumer-specific external contract payload まで current bridge へ同時に入れる

#### 利点

- `proof_notebook` 向け payload に一気に近づく
- later reopen の一部を早めに片付けられる

#### 欠点

- actual external contract と consumer-specific payload を同じ reopen で混ぜやすい
- `proof_assistant_adapter` / `theorem_export_checker` pressure を premature に呼びやすい
- theorem-line retained bridge を actual exported contract へ既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal actual external contract だけを持つ retained bridge にする**
である。

理由は次の通り。

1. external-contract-facing handoff row の次段として actual external contract 自体は narrow に橋渡しできる
2. consumer-specific payload を still 後段に残せる
3. next later reopen を consumer-specific external contract payload comparison へ狭く進めやすい

## minimal actual-external-contract-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch,
  retained_payload_body_materialization_external_contract = {
    consumer_class = proof_notebook,
    handoff_row_ref = retained_payload_body_materialization_external_handoff_row
  }
}
```

### `retained_payload_body_materialization_external_contract`

`retained_payload_body_materialization_external_contract` は、
`proof_notebook` first consumer class に向けた actual external contract を
theorem-side retained bridge で最小限に表す field である。

current task では、この contract を consumer-specific external contract payload には昇格させない。

## なぜ consumer-specific external contract payload をまだ入れないか

consumer-specific external contract payload を current phase で入れると、

- external-contract-facing handoff row
- actual external contract
- notebook-specific external contract payload
- `proof_assistant_adapter` / `theorem_export_checker` pressure

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず actual external contract 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain actual-external-contract-ready retained bridge

```text
proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch = bridge_sketch:e12.retained_payload_external_handoff_row_ready,
  retained_payload_body_materialization_external_contract = {
    consumer_class = proof_notebook,
    handoff_row_ref = retained_payload_external_handoff_row:e12.latest
  }
}
```

ここで notebook bridge が知るのは actual external contract までであり、
consumer-specific payload まではまだ bridge に入れない。

### example B — witnessed draw actual-external-contract-ready retained bridge

```text
proof_notebook_bridge_retained_payload_actual_external_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_external_handoff_row_ready,
  retained_payload_body_materialization_external_contract = {
    consumer_class = proof_notebook,
    handoff_row_ref = retained_payload_external_handoff_row:sugoroku.draw.latest
  }
}
```

draw external contract の actual shape は notebook bridge で追えるが、
consumer-specific payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual external contract comparison を切り、current first choice は `retained_payload_body_materialization_external_contract` だけを足す retained bridge である
- consumer-specific external contract payload は still 後段に残す
- next later reopen は actual-external-contract-ready consumer-specific external contract payload comparison である

### not decided

- consumer-specific external contract payload をどの field で切るか
- actual external contract を retained bridge のまま維持するか consumer-specific payload へ actualize するか
- `proof_assistant_adapter` / `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何とみなすか
