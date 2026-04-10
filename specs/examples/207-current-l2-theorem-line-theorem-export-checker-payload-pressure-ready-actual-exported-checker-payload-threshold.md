# 207 — current L2 theorem line theorem-export-checker-payload-pressure-ready actual-exported-checker-payload threshold

## 目的

`specs/examples/206-current-l2-theorem-line-theorem-export-checker-contract-ready-exported-checker-payload-pressure-threshold.md`
までを前提に、

- theorem-export-checker-payload-pressure-ready retained bridge に actual exported checker payload をどこまで近づけるか
- actual exported checker payload を retained bridge に入れるべきか
- checker result materialization family を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の theorem-export-checker-payload-pressure-ready actual-exported-checker-payload threshold** であり、
checker result materialization family はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- exported checker payload は theorem-side retained bridge の field としてのみ扱う。
- checker result materialization family と actual checker result payload family は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_payload_pressure` までは current first choice に入っている。
2. exported checker payload pressure は bridge 側へ symbolic marker として入るが、actual exported checker payload は still 後段に残す cut が固定済みである。
3. `theorem_export_checker_payload` family は next practical payload candidate として最も近い。

したがって current 問いは、
**exported checker payload pressure の次段として actual exported checker payload を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker result materialization family まで同時に呼ぶべきか**
である。

## 比較観点

1. exported checker payload pressure と actual exported checker payload の line を narrow に切れるか
2. actual exported checker payload を retained bridge に足しても、checker result materialization family を still 後段に残せるか
3. theorem-line retained bridge に checker result materialization family を premature に押し込まないか
4. next later reopen を checker-result-materialization-family comparison へ狭く進められるか

## 比較対象

### 案 1. exported checker payload pressure marker を terminal cut にし、actual exported checker payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual exported checker payload の premature actualization を避けられる

#### 欠点

- exported checker payload 自体が current bridge では見えない
- checker-facing contract と actual exported checker payload のあいだの gap が prose 依存に残りやすい

### 案 2. actual exported checker payload だけを持つ retained bridge にする

#### 読み

theorem-export-checker-payload-pressure-ready retained bridge に、
checker result materialization family を導入せず
**`retained_payload_body_materialization_theorem_export_checker_payload`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_payload = {
    theorem_export_checker_payload_pressure_ref = retained_payload_body_materialization_theorem_export_checker_payload_pressure,
    payload_family = theorem_export_checker_payload
  }
}
```

#### 利点

- actual exported checker payload 自体は current bridge で見える
- checker result materialization family を still 後段に残せる
- next later reopen を checker-result-materialization-family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- actual exported checker payload と checker result materialization family を誤読されない説明が要る

### 案 3. checker result materialization family まで current bridge へ同時に入れる

#### 利点

- checker-facing line の先を一気に concrete にできる
- exported checker payload と checker result family の接続を bridge で直接見える

#### 欠点

- actual exported checker payload と checker result materialization family を同じ reopen で混ぜやすい
- theorem-line retained bridge を checker result materialization family へ既成事実化しやすい
- payload-specific compare / retention line を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual exported checker payload だけを持つ retained bridge にする**
である。

理由は次の通り。

1. exported checker payload pressure の次段として actual exported checker payload 自体は narrow に橋渡しできる
2. checker result materialization family を still 後段に残せる
3. next later reopen を checker-result-materialization-family comparison へ狭く進めやすい

## minimal actual-exported-checker-payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_payload = {
    theorem_export_checker_payload_pressure_ref = retained_payload_body_materialization_theorem_export_checker_payload_pressure,
    payload_family = theorem_export_checker_payload
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_payload`

`retained_payload_body_materialization_theorem_export_checker_payload` は、
exported checker payload pressure の次段として
actual exported checker payload 自体を theorem-side retained bridge で最小限に表す field である。

current task では、この payload を checker result materialization family には昇格させない。

## なぜ checker result materialization family をまだ入れないか

checker result materialization family を current phase で入れると、

- exported checker payload pressure
- actual exported checker payload
- checker result materialization family

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず actual exported checker payload 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain actual-exported-checker-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch = bridge_sketch:e12.theorem_export_checker_payload_pressure_ready,
  retained_payload_body_materialization_theorem_export_checker_payload = {
    theorem_export_checker_payload_pressure_ref = retained_payload_theorem_export_checker_payload_pressure:e12.latest,
    payload_family = theorem_export_checker_payload
  }
}
```

ここで bridge が示すのは actual exported checker payload までであり、
checker result materialization family まではまだ bridge に入れない。

### example B — witnessed draw actual-exported-checker-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_pressure_ready_sketch = bridge_sketch:sugoroku.draw.theorem_export_checker_payload_pressure_ready,
  retained_payload_body_materialization_theorem_export_checker_payload = {
    theorem_export_checker_payload_pressure_ref = retained_payload_theorem_export_checker_payload_pressure:sugoroku.draw.latest,
    payload_family = theorem_export_checker_payload
  }
}
```

draw case でも actual exported checker payload 自体は bridge で追えるが、
checker result materialization family までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual exported checker payload comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_payload` だけを足す retained bridge である
- checker result materialization family は still 後段に残す
- next later reopen は theorem-export-checker-payload-ready checker-result-materialization-family comparison である

### not decided

- checker result materialization family をどの field / row / payload family で切るか
- `theorem_export_checker_payload` を retained bridge のまま維持するか checker result materialization family へ actualize するか
- checker result materialization family をどの concrete threshold で呼ぶか
