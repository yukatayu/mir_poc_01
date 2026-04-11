# 215 — current L2 theorem line checker-verdict-transport-carrier-detail-ready checker-verdict-transport-payload threshold

## 目的

`specs/examples/214-current-l2-theorem-line-checker-verdict-transport-family-ready-checker-verdict-transport-carrier-detail-threshold.md`
までを前提に、

- checker-verdict-transport-carrier-detail-ready retained bridge に checker verdict transport payload をどこまで近づけるか
- checker verdict transport payload を retained bridge に入れるべきか
- checker verdict transport receipt / channel body を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-verdict-transport-carrier-detail-ready checker-verdict-transport-payload threshold** であり、
checker verdict transport receipt / channel body はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker verdict transport payload は theorem-side retained bridge の field としてのみ扱う。
- checker verdict transport receipt / channel body は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail` までは current first choice に入っている。
2. checker verdict transport carrier detail は bridge 側へ入るが、checker verdict transport payload / receipt / channel body は still 後段に残す cut が固定済みである。
3. checker verdict transport payload は、checker verdict transport carrier detail の直後に置ける next practical payload candidate として最も近い。

したがって current 問いは、
**checker verdict transport carrier detail の次段として checker verdict transport payload を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker verdict transport receipt / channel body まで同時に呼ぶべきか**
である。

## 比較観点

1. checker verdict transport carrier detail と checker verdict transport payload の line を narrow に切れるか
2. checker verdict transport payload を retained bridge に足しても、checker verdict transport receipt / channel body を still 後段に残せるか
3. theorem-line retained bridge に transport receipt / channel body を premature に押し込まないか
4. next later reopen を checker-verdict-transport-receipt comparison へ狭く進められるか

## 比較対象

### 案 1. checker verdict transport carrier detail を terminal cut にし、checker verdict transport payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker verdict transport payload の premature actualization を避けられる

#### 欠点

- checker verdict transport carrier detail と checker verdict transport payload の gap が prose 依存に残りやすい
- transport line の next concrete candidate が current bridge で見えない

### 案 2. checker verdict transport payload だけを持つ retained bridge にする

#### 読み

checker-verdict-transport-carrier-detail-ready retained bridge に、
checker verdict transport receipt / channel body を導入せず
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_carrier_detail_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload = {
    theorem_export_checker_verdict_transport_carrier_detail_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail,
    payload_kind = checker_verdict_transport_payload
  }
}
```

#### 利点

- checker verdict transport payload 自体は current bridge で見える
- checker verdict transport receipt / channel body を still 後段に残せる
- next later reopen を checker-verdict-transport-receipt comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- payload と receipt / channel body を誤読されない説明が要る

### 案 3. checker verdict transport receipt / channel body まで current bridge へ同時に入れる

#### 利点

- transport line の先を一気に concrete にできる
- receipt / channel body へ直接つながりやすい

#### 欠点

- checker verdict transport payload と transport receipt / channel body を同じ reopen で混ぜやすい
- theorem-line retained bridge を actual transport line へ既成事実化しやすい
- next later reopen を narrow に保ちにくい

## current judgment

current L2 で最も自然なのは、
**案 2. checker verdict transport payload だけを持つ retained bridge にする**
である。

理由は次の通り。

1. checker verdict transport carrier detail の次段として checker verdict transport payload 自体は narrow に橋渡しできる
2. checker verdict transport receipt / channel body を still 後段に残せる
3. next later reopen を checker-verdict-transport-receipt comparison へ狭く進めやすい

## minimal checker-verdict-transport-payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_carrier_detail_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload = {
    theorem_export_checker_verdict_transport_carrier_detail_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail,
    payload_kind = checker_verdict_transport_payload
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload`

`retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload` は、
checker verdict transport carrier detail の次段として
checker verdict transport payload 自体を theorem-side retained bridge で最小限に表す field である。

current task では、この payload を checker verdict transport receipt / channel body には昇格させない。

## なぜ checker verdict transport receipt / channel body をまだ入れないか

checker verdict transport receipt / channel body を current phase で入れると、

- checker verdict transport carrier detail
- checker verdict transport payload
- checker verdict transport receipt / channel body

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker verdict transport payload 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-verdict-transport-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_carrier_detail_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_verdict_transport_carrier_detail_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload = {
    theorem_export_checker_verdict_transport_carrier_detail_ref =
      retained_payload_theorem_export_checker_verdict_transport_carrier_detail:e12.latest,
    payload_kind = checker_verdict_transport_payload
  }
}
```

ここで bridge が示すのは checker verdict transport payload までであり、
checker verdict transport receipt / channel body まではまだ bridge に入れない。

### example B — witnessed draw checker-verdict-transport-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_carrier_detail_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_verdict_transport_carrier_detail_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload = {
    theorem_export_checker_verdict_transport_carrier_detail_ref =
      retained_payload_theorem_export_checker_verdict_transport_carrier_detail:sugoroku.draw.latest,
    payload_kind = checker_verdict_transport_payload
  }
}
```

draw case でも checker verdict transport payload 自体は bridge で追えるが、
checker verdict transport receipt / channel body までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker verdict transport payload comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_verdict_transport_payload` だけを足す retained bridge である
- checker verdict transport receipt / channel body は still 後段に残す
- next later reopen は checker-verdict-transport-payload-ready checker-verdict-transport-receipt comparison である

### not decided

- checker verdict transport receipt をどの field / row / payload family で切るか
- `retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail` を retained bridge のまま維持するか checker verdict transport payload へ actualize するか
- checker verdict transport channel body と low-level memory-order family をどの concrete threshold で呼ぶか
