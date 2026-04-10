# 213 — current L2 theorem line checker-verdict-witness-family-ready checker-verdict-transport-family threshold

## 目的

`specs/examples/212-current-l2-theorem-line-checker-verdict-payload-family-ready-checker-verdict-witness-family-threshold.md`
までを前提に、

- checker-verdict-witness-family-ready retained bridge に checker verdict transport family をどこまで近づけるか
- checker verdict transport family を retained bridge に入れるべきか
- checker verdict transport carrier detail / payload / receipt を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-verdict-witness-family-ready checker-verdict-transport-family threshold** であり、
checker verdict transport carrier detail / payload / receipt はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker verdict transport family は theorem-side retained bridge の field としてのみ扱う。
- checker verdict transport carrier detail / payload / receipt は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_verdict_witness_family` までは current first choice に入っている。
2. checker verdict witness family は bridge 側へ入るが、checker verdict transport carrier detail / payload / receipt は still 後段に残す cut が固定済みである。
3. checker verdict transport family は、checker verdict witness family の次段に置ける next practical transport-family candidate として最も近い。

したがって current 問いは、
**checker verdict witness family の次段として checker verdict transport family を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker verdict transport carrier detail / payload / receipt まで同時に呼ぶべきか**
である。

## 比較観点

1. checker verdict witness family と checker verdict transport family の line を narrow に切れるか
2. checker verdict transport family を retained bridge に足しても、checker verdict transport carrier detail / payload / receipt を still 後段に残せるか
3. theorem-line retained bridge に transport carrier detail / payload / receipt を premature に押し込まないか
4. next later reopen を checker-verdict-transport-carrier-detail comparison へ狭く進められるか

## 比較対象

### 案 1. checker verdict witness family を terminal cut にし、checker verdict transport family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker verdict transport family の premature actualization を避けられる

#### 欠点

- checker verdict witness family と checker verdict transport family の gap が prose 依存に残りやすい
- transport line の next concrete candidate が current bridge で見えない

### 案 2. checker verdict transport family marker だけを持つ retained bridge にする

#### 読み

checker-verdict-witness-family-ready retained bridge に、
checker verdict transport carrier detail / payload / receipt を導入せず
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_family`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_family = {
    theorem_export_checker_verdict_witness_family_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_witness_family,
    next_transport_family = theorem_export_checker_verdict_transport
  }
}
```

#### 利点

- checker verdict transport family 自体は current bridge で見える
- checker verdict transport carrier detail / payload / receipt を still 後段に残せる
- next later reopen を checker-verdict-transport-carrier-detail comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- transport family marker と actual transport carrier を誤読されない説明が要る

### 案 3. checker verdict transport carrier detail / payload / receipt まで current bridge へ同時に入れる

#### 利点

- transport line の先を一気に concrete にできる
- transport payload / receipt へ直接つながりやすい

#### 欠点

- checker verdict transport family と transport carrier detail / payload / receipt を同じ reopen で混ぜやすい
- theorem-line retained bridge を actual transport line へ既成事実化しやすい
- next later reopen を narrow に保ちにくい

## current judgment

current L2 で最も自然なのは、
**案 2. checker verdict transport family marker だけを持つ retained bridge にする**
である。

理由は次の通り。

1. checker verdict witness family の次段として checker verdict transport family 自体は narrow に橋渡しできる
2. checker verdict transport carrier detail / payload / receipt を still 後段に残せる
3. next later reopen を checker-verdict-transport-carrier-detail comparison へ狭く進めやすい

## minimal checker-verdict-transport-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_family = {
    theorem_export_checker_verdict_witness_family_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_witness_family,
    next_transport_family = theorem_export_checker_verdict_transport
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_verdict_transport_family`

`retained_payload_body_materialization_theorem_export_checker_verdict_transport_family` は、
checker verdict witness family の次段として
`theorem_export_checker_verdict_transport` family が practical candidate として存在することだけを
theorem-side retained bridge で最小限に表す field である。

current task では、この marker を checker verdict transport carrier detail / payload / receipt には昇格させない。

## なぜ checker verdict transport carrier detail / payload / receipt をまだ入れないか

checker verdict transport carrier detail / payload / receipt を current phase で入れると、

- checker verdict witness family
- checker verdict transport family
- checker verdict transport carrier detail / payload / receipt

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker verdict transport family 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-verdict-transport-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_verdict_witness_family_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_family = {
    theorem_export_checker_verdict_witness_family_ref =
      retained_payload_theorem_export_checker_verdict_witness_family:e12.latest,
    next_transport_family = theorem_export_checker_verdict_transport
  }
}
```

ここで bridge が示すのは checker verdict transport family までであり、
checker verdict transport carrier detail / payload / receipt まではまだ bridge に入れない。

### example B — witnessed draw checker-verdict-transport-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_witness_family_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_verdict_witness_family_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_family = {
    theorem_export_checker_verdict_witness_family_ref =
      retained_payload_theorem_export_checker_verdict_witness_family:sugoroku.draw.latest,
    next_transport_family = theorem_export_checker_verdict_transport
  }
}
```

draw case でも checker verdict transport family 自体は bridge で追えるが、
checker verdict transport carrier detail / payload / receipt までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker verdict transport family comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_verdict_transport_family` だけを足す retained bridge である
- checker verdict transport carrier detail / payload / receipt は still 後段に残す
- next later reopen は checker-verdict-transport-family-ready checker-verdict-transport-carrier-detail comparison である

### not decided

- checker verdict transport carrier detail をどの field / row / payload family で切るか
- `retained_payload_body_materialization_theorem_export_checker_verdict_transport_family` を retained bridge のまま維持するか checker verdict transport carrier detail へ actualize するか
- checker verdict transport payload / receipt / channel body をどの concrete threshold で呼ぶか
