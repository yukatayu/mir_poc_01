# 212 — current L2 theorem line checker-verdict-payload-family-ready checker-verdict-witness-family threshold

## 目的

`specs/examples/211-current-l2-theorem-line-checker-verdict-carrier-detail-ready-checker-verdict-payload-family-threshold.md`
までを前提に、

- checker-verdict-payload-family-ready retained bridge に checker verdict witness family をどこまで近づけるか
- checker verdict witness family を retained bridge に入れるべきか
- checker verdict transport を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-verdict-payload-family-ready checker-verdict-witness-family threshold** であり、
checker verdict transport はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker verdict witness family は theorem-side retained bridge の field としてのみ扱う。
- checker verdict transport line は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_verdict_payload_family` までは current first choice に入っている。
2. checker verdict payload family は bridge 側へ入るが、checker verdict witness / transport は still 後段に残す cut が固定済みである。
3. checker verdict witness family は、checker verdict payload family の次段に置ける next practical witness-family candidate として最も近い。

したがって current 問いは、
**checker verdict payload family の次段として checker verdict witness family を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker verdict transport まで同時に呼ぶべきか**
である。

## 比較観点

1. checker verdict payload family と checker verdict witness family の line を narrow に切れるか
2. checker verdict witness family を retained bridge に足しても、checker verdict transport を still 後段に残せるか
3. theorem-line retained bridge に checker verdict transport を premature に押し込まないか
4. next later reopen を checker-verdict-transport-family comparison へ狭く進められるか

## 比較対象

### 案 1. checker verdict payload family を terminal cut にし、checker verdict witness family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker verdict witness family の premature actualization を避けられる

#### 欠点

- checker verdict payload family と checker verdict witness family の gap が prose 依存に残りやすい
- witness line の next concrete candidate が current bridge で見えない

### 案 2. checker verdict witness family marker だけを持つ retained bridge にする

#### 読み

checker-verdict-payload-family-ready retained bridge に、
checker verdict transport を導入せず
**`retained_payload_body_materialization_theorem_export_checker_verdict_witness_family`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_witness_family = {
    theorem_export_checker_verdict_payload_family_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_payload_family,
    next_witness_family = theorem_export_checker_verdict_witness
  }
}
```

#### 利点

- checker verdict witness family 自体は current bridge で見える
- checker verdict transport を still 後段に残せる
- next later reopen を checker-verdict-transport-family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- witness family marker と transport を誤読されない説明が要る

### 案 3. checker verdict transport まで current bridge へ同時に入れる

#### 利点

- checker verdict line の先を一気に concrete にできる
- transport line へ直接つながりやすい

#### 欠点

- checker verdict witness family と transport を同じ reopen で混ぜやすい
- theorem-line retained bridge を transport line へ既成事実化しやすい
- next later reopen を narrow に保ちにくい

## current judgment

current L2 で最も自然なのは、
**案 2. checker verdict witness family marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. checker verdict payload family の次段として checker verdict witness family 自体は narrow に橋渡しできる
2. checker verdict transport を still 後段に残せる
3. next later reopen を checker-verdict-transport-family comparison へ狭く進めやすい

## minimal checker-verdict-witness-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_witness_family = {
    theorem_export_checker_verdict_payload_family_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_payload_family,
    next_witness_family = theorem_export_checker_verdict_witness
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_verdict_witness_family`

`retained_payload_body_materialization_theorem_export_checker_verdict_witness_family` は、
checker verdict payload family の次段として
`theorem_export_checker_verdict_witness` family が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を checker verdict transport には昇格させない。

## なぜ checker verdict transport をまだ入れないか

checker verdict transport を current phase で入れると、

- checker verdict payload family
- checker verdict witness family
- checker verdict transport

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker verdict witness family 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-verdict-witness-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_verdict_payload_family_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_witness_family = {
    theorem_export_checker_verdict_payload_family_ref =
      retained_payload_theorem_export_checker_verdict_payload_family:e12.latest,
    next_witness_family = theorem_export_checker_verdict_witness
  }
}
```

ここで bridge が示すのは checker verdict witness family までであり、
checker verdict transport まではまだ bridge に入れない。

### example B — witnessed draw checker-verdict-witness-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_payload_family_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_verdict_payload_family_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_witness_family = {
    theorem_export_checker_verdict_payload_family_ref =
      retained_payload_theorem_export_checker_verdict_payload_family:sugoroku.draw.latest,
    next_witness_family = theorem_export_checker_verdict_witness
  }
}
```

draw case でも checker verdict witness family 自体は bridge で追えるが、
checker verdict transport までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker verdict witness family comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_verdict_witness_family` だけを足す retained bridge である
- checker verdict transport は still 後段に残す
- next later reopen は checker-verdict-witness-family-ready checker-verdict-transport-family comparison である

### not decided

- checker verdict transport family をどの field / row / payload family で切るか
- `retained_payload_body_materialization_theorem_export_checker_verdict_witness_family` を retained bridge のまま維持するか checker verdict transport family へ actualize するか
- checker verdict transport carrier detail / payload / receipt をどの concrete threshold で呼ぶか
