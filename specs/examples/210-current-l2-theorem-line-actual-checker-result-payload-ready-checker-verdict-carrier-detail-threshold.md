# 210 — current L2 theorem line actual-checker-result-payload-ready checker-verdict-carrier-detail threshold

## 目的

`specs/examples/209-current-l2-theorem-line-checker-result-materialization-family-ready-actual-checker-result-payload-threshold.md`
までを前提に、

- actual-checker-result-payload-ready retained bridge に checker verdict carrier detail をどこまで近づけるか
- checker verdict carrier detail を retained bridge に入れるべきか
- checker verdict payload family / witness / transport を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の actual-checker-result-payload-ready checker-verdict-carrier-detail threshold** であり、
checker verdict payload family / witness / transport はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker verdict carrier detail は theorem-side retained bridge の field としてのみ扱う。
- checker verdict payload family / witness / transport line は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_result_payload` までは current first choice に入っている。
2. actual checker result payload は bridge 側へ入るが、checker verdict carrier detail は still 後段に残す cut が固定済みである。
3. checker verdict carrier detail は、actual checker result payload の直後に置ける next practical carrier candidate として最も近い。

したがって current 問いは、
**actual checker result payload の次段として checker verdict carrier detail を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker verdict payload family / witness / transport まで同時に呼ぶべきか**
である。

## 比較観点

1. actual checker result payload と checker verdict carrier detail の line を narrow に切れるか
2. checker verdict carrier detail を retained bridge に足しても、checker verdict payload family / witness / transport を still 後段に残せるか
3. theorem-line retained bridge に checker verdict payload family / witness / transport を premature に押し込まないか
4. next later reopen を checker-verdict-payload-family comparison へ狭く進められるか

## 比較対象

### 案 1. actual checker result payload を terminal cut にし、checker verdict carrier detail も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker verdict carrier detail の premature actualization を避けられる

#### 欠点

- actual checker result payload と checker verdict carrier detail の gap が prose 依存に残りやすい
- checker verdict line の next concrete candidate が current bridge で見えない

### 案 2. checker verdict carrier detail だけを持つ retained bridge にする

#### 読み

actual-checker-result-payload-ready retained bridge に、
checker verdict payload family / witness / transport を導入せず
**`retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail = {
    theorem_export_checker_result_payload_ref =
      retained_payload_body_materialization_theorem_export_checker_result_payload,
    carrier_kind = checker_verdict
  }
}
```

#### 利点

- checker verdict carrier detail 自体は current bridge で見える
- checker verdict payload family / witness / transport を still 後段に残せる
- next later reopen を checker-verdict-payload-family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- carrier detail と actual verdict payload を誤読されない説明が要る

### 案 3. checker verdict payload family / witness / transport まで current bridge へ同時に入れる

#### 利点

- checker verdict line の先を一気に concrete にできる
- verdict payload / witness / transport へ直接つながりやすい

#### 欠点

- checker verdict carrier detail と verdict payload family / witness / transport を同じ reopen で混ぜやすい
- theorem-line retained bridge を verdict payload / witness / transport line へ既成事実化しやすい
- next later reopen を narrow に保ちにくい

## current judgment

current L2 で最も自然なのは、
**案 2. checker verdict carrier detail だけを持つ retained bridge にする**
である。

理由は次の通り。

1. actual checker result payload の次段として checker verdict carrier detail 自体は narrow に橋渡しできる
2. checker verdict payload family / witness / transport を still 後段に残せる
3. next later reopen を checker-verdict-payload-family comparison へ狭く進めやすい

## minimal checker-verdict-carrier-detail-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail = {
    theorem_export_checker_result_payload_ref =
      retained_payload_body_materialization_theorem_export_checker_result_payload,
    carrier_kind = checker_verdict
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail`

`retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail` は、
actual checker result payload の次段として
checker verdict carrier detail 自体を theorem-side retained bridge で最小限に表す field である。

current task では、この carrier detail を checker verdict payload family / witness / transport には昇格させない。

## なぜ checker verdict payload family / witness / transport をまだ入れないか

checker verdict payload family / witness / transport を current phase で入れると、

- actual checker result payload
- checker verdict carrier detail
- checker verdict payload family / witness / transport

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker verdict carrier detail 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-verdict-carrier-detail-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_result_payload_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail = {
    theorem_export_checker_result_payload_ref =
      retained_payload_theorem_export_checker_result_payload:e12.latest,
    carrier_kind = checker_verdict
  }
}
```

ここで bridge が示すのは checker verdict carrier detail までであり、
checker verdict payload family / witness / transport まではまだ bridge に入れない。

### example B — witnessed draw checker-verdict-carrier-detail-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_result_payload_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail = {
    theorem_export_checker_result_payload_ref =
      retained_payload_theorem_export_checker_result_payload:sugoroku.draw.latest,
    carrier_kind = checker_verdict
  }
}
```

draw case でも checker verdict carrier detail 自体は bridge で追えるが、
checker verdict payload family / witness / transport までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker verdict carrier detail comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail` だけを足す retained bridge である
- checker verdict payload family / witness / transport は still 後段に残す
- next later reopen は checker-verdict-carrier-detail-ready checker-verdict-payload-family comparison である

### not decided

- checker verdict payload family をどの field / row / payload family で切るか
- `retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail` を retained bridge のまま維持するか checker verdict payload family へ actualize するか
- checker verdict witness / transport line をどの concrete threshold で呼ぶか
