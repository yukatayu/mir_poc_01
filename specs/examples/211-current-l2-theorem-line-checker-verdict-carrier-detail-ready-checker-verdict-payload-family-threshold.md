# 211 — current L2 theorem line checker-verdict-carrier-detail-ready checker-verdict-payload-family threshold

## 目的

`specs/examples/210-current-l2-theorem-line-actual-checker-result-payload-ready-checker-verdict-carrier-detail-threshold.md`
までを前提に、

- checker-verdict-carrier-detail-ready retained bridge に checker verdict payload family をどこまで近づけるか
- checker verdict payload family を retained bridge に入れるべきか
- checker verdict witness / transport を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-verdict-carrier-detail-ready checker-verdict-payload-family threshold** であり、
checker verdict witness / transport はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker verdict payload family は theorem-side retained bridge の field としてのみ扱う。
- checker verdict witness / transport line は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail` までは current first choice に入っている。
2. checker verdict carrier detail は bridge 側へ入るが、checker verdict payload family / witness / transport は still 後段に残す cut が固定済みである。
3. checker verdict payload family は、checker verdict carrier detail の次段に置ける next practical payload-family candidate として最も近い。

したがって current 問いは、
**checker verdict carrier detail の次段として checker verdict payload family を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker verdict witness / transport まで同時に呼ぶべきか**
である。

## 比較観点

1. checker verdict carrier detail と checker verdict payload family の line を narrow に切れるか
2. checker verdict payload family を retained bridge に足しても、checker verdict witness / transport を still 後段に残せるか
3. theorem-line retained bridge に checker verdict witness / transport を premature に押し込まないか
4. next later reopen を checker-verdict-witness-family comparison へ狭く進められるか

## 比較対象

### 案 1. checker verdict carrier detail を terminal cut にし、checker verdict payload family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker verdict payload family の premature actualization を避けられる

#### 欠点

- checker verdict carrier detail と checker verdict payload family の gap が prose 依存に残りやすい
- verdict payload line の next concrete candidate が current bridge で見えない

### 案 2. checker verdict payload family marker だけを持つ retained bridge にする

#### 読み

checker-verdict-carrier-detail-ready retained bridge に、
checker verdict witness / transport を導入せず
**`retained_payload_body_materialization_theorem_export_checker_verdict_payload_family`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_payload_family = {
    theorem_export_checker_verdict_carrier_detail_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail,
    next_payload_family = theorem_export_checker_verdict_payload
  }
}
```

#### 利点

- checker verdict payload family 自体は current bridge で見える
- checker verdict witness / transport を still 後段に残せる
- next later reopen を checker-verdict-witness-family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- payload family marker と witness / transport を誤読されない説明が要る

### 案 3. checker verdict witness / transport まで current bridge へ同時に入れる

#### 利点

- checker verdict line の先を一気に concrete にできる
- witness / transport へ直接つながりやすい

#### 欠点

- checker verdict payload family と witness / transport を同じ reopen で混ぜやすい
- theorem-line retained bridge を witness / transport line へ既成事実化しやすい
- next later reopen を narrow に保ちにくい

## current judgment

current L2 で最も自然なのは、
**案 2. checker verdict payload family marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. checker verdict carrier detail の次段として checker verdict payload family 自体は narrow に橋渡しできる
2. checker verdict witness / transport を still 後段に残せる
3. next later reopen を checker-verdict-witness-family comparison へ狭く進めやすい

## minimal checker-verdict-payload-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_payload_family = {
    theorem_export_checker_verdict_carrier_detail_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail,
    next_payload_family = theorem_export_checker_verdict_payload
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_verdict_payload_family`

`retained_payload_body_materialization_theorem_export_checker_verdict_payload_family` は、
checker verdict carrier detail の次段として
`theorem_export_checker_verdict_payload` family が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を checker verdict witness / transport には昇格させない。

## なぜ checker verdict witness / transport をまだ入れないか

checker verdict witness / transport を current phase で入れると、

- checker verdict carrier detail
- checker verdict payload family
- checker verdict witness / transport

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker verdict payload family 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-verdict-payload-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_verdict_carrier_detail_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_payload_family = {
    theorem_export_checker_verdict_carrier_detail_ref =
      retained_payload_theorem_export_checker_verdict_carrier_detail:e12.latest,
    next_payload_family = theorem_export_checker_verdict_payload
  }
}
```

ここで bridge が示すのは checker verdict payload family までであり、
checker verdict witness / transport まではまだ bridge に入れない。

### example B — witnessed draw checker-verdict-payload-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_verdict_carrier_detail_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_payload_family = {
    theorem_export_checker_verdict_carrier_detail_ref =
      retained_payload_theorem_export_checker_verdict_carrier_detail:sugoroku.draw.latest,
    next_payload_family = theorem_export_checker_verdict_payload
  }
}
```

draw case でも checker verdict payload family 自体は bridge で追えるが、
checker verdict witness / transport までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker verdict payload family comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_verdict_payload_family` だけを足す retained bridge である
- checker verdict witness / transport は still 後段に残す
- next later reopen は checker-verdict-payload-family-ready checker-verdict-witness-family comparison である

### not decided

- checker verdict witness family をどの field / row / payload family で切るか
- `retained_payload_body_materialization_theorem_export_checker_verdict_payload_family` を retained bridge のまま維持するか checker verdict witness family へ actualize するか
- checker verdict transport line をどの concrete threshold で呼ぶか
