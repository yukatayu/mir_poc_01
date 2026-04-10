# 208 — current L2 theorem line actual-exported-checker-payload-ready checker-result-materialization-family threshold

## 目的

`specs/examples/207-current-l2-theorem-line-theorem-export-checker-payload-pressure-ready-actual-exported-checker-payload-threshold.md`
までを前提に、

- actual-exported-checker-payload-ready retained bridge に checker result materialization family をどこまで近づけるか
- checker result materialization family を symbolic marker に留めるべきか
- actual checker result payload まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の actual-exported-checker-payload-ready checker-result-materialization-family threshold** であり、
actual checker result payload はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker result materialization family は theorem-side retained bridge の family marker としてのみ扱う。
- actual checker result payload family と checker verdict carrier detail は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_payload` までは current first choice に入っている。
2. actual exported checker payload は bridge 側へ入るが、checker result materialization family は still 後段に残す cut が固定済みである。
3. `theorem_export_checker_result_payload` family は next practical result candidate として最も近い。

したがって current 問いは、
**actual exported checker payload の次段として checker result materialization family を retained bridge に近づけるなら、symbolic family marker を first cut にするべきか、それとも actual checker result payload まで同時に切るべきか**
である。

## 比較観点

1. actual exported checker payload と checker result materialization family の line を narrow に切れるか
2. checker result materialization family を symbolic marker に留め、actual checker result payload を still 後段に残せるか
3. theorem-line retained bridge に actual checker result payload を premature に押し込まないか
4. next later reopen を actual checker result payload comparison へ狭く進められるか

## 比較対象

### 案 1. actual exported checker payload を terminal cut にし、checker result materialization family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker result materialization family の premature actualization を避けられる

#### 欠点

- actual exported checker payload と checker result materialization family の gap が prose 依存に残りやすい
- `theorem_export_checker_result_payload` family が next practical candidate であることを current bridge で見えにくい

### 案 2. symbolic checker result materialization family marker だけを持つ retained bridge にする

#### 読み

actual-exported-checker-payload-ready retained bridge に、
actual checker result payload を導入せず
**`retained_payload_body_materialization_theorem_export_checker_result_materialization_family`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_result_materialization_family = {
    theorem_export_checker_payload_ref = retained_payload_body_materialization_theorem_export_checker_payload,
    next_payload_family = theorem_export_checker_result_payload
  }
}
```

#### 利点

- checker result materialization family 自体は current bridge で見える
- actual checker result payload を still 後段に残せる
- next later reopen を actual checker result payload comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- symbolic family marker と actual checker result payload を誤読されない説明が要る

### 案 3. actual checker result payload まで current bridge へ同時に入れる

#### 利点

- checker-facing line の先を一気に concrete にできる
- actual exported checker payload と checker result payload の接続を bridge で直接見える

#### 欠点

- checker result materialization family と actual checker result payload を同じ reopen で混ぜやすい
- theorem-line retained bridge を actual checker result payload family へ既成事実化しやすい
- result-specific compare / verdict carrier detail を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic checker result materialization family marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. actual exported checker payload の次段として checker result materialization family 自体は narrow に橋渡しできる
2. actual checker result payload を still 後段に残せる
3. next later reopen を actual checker result payload comparison へ狭く進めやすい

## minimal checker-result-materialization-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_result_materialization_family = {
    theorem_export_checker_payload_ref = retained_payload_body_materialization_theorem_export_checker_payload,
    next_payload_family = theorem_export_checker_result_payload
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_result_materialization_family`

`retained_payload_body_materialization_theorem_export_checker_result_materialization_family` は、
actual exported checker payload の次段として
`theorem_export_checker_result_payload` family が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を actual checker result payload には昇格させない。

## なぜ actual checker result payload をまだ入れないか

actual checker result payload を current phase で入れると、

- actual exported checker payload
- symbolic checker result materialization family
- actual checker result payload

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker result materialization family 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-result-materialization-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch = bridge_sketch:e12.theorem_export_checker_payload_ready,
  retained_payload_body_materialization_theorem_export_checker_result_materialization_family = {
    theorem_export_checker_payload_ref = retained_payload_theorem_export_checker_payload:e12.latest,
    next_payload_family = theorem_export_checker_result_payload
  }
}
```

ここで bridge が示すのは checker result materialization family までであり、
actual checker result payload まではまだ bridge に入れない。

### example B — witnessed draw checker-result-materialization-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_payload_ready_sketch = bridge_sketch:sugoroku.draw.theorem_export_checker_payload_ready,
  retained_payload_body_materialization_theorem_export_checker_result_materialization_family = {
    theorem_export_checker_payload_ref = retained_payload_theorem_export_checker_payload:sugoroku.draw.latest,
    next_payload_family = theorem_export_checker_result_payload
  }
}
```

draw case でも checker result materialization family 自体は bridge で追えるが、
actual checker result payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker result materialization family comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_result_materialization_family` だけを足す retained bridge である
- actual checker result payload は still 後段に残す
- next later reopen は actual-exported-checker-payload-ready actual-checker-result-payload comparison である

### not decided

- actual checker result payload をどの field / row / payload family で切るか
- `theorem_export_checker_result_payload` family marker を retained bridge のまま維持するか actual checker result payload へ actualize するか
- checker verdict carrier detail をどの concrete threshold で呼ぶか
