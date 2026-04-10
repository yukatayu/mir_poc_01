# 209 — current L2 theorem line checker-result-materialization-family-ready actual-checker-result-payload threshold

## 目的

`specs/examples/208-current-l2-theorem-line-actual-exported-checker-payload-ready-checker-result-materialization-family-threshold.md`
までを前提に、

- checker-result-materialization-family-ready retained bridge に actual checker result payload をどこまで近づけるか
- actual checker result payload を retained bridge に入れるべきか
- checker verdict carrier detail を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-result-materialization-family-ready actual-checker-result-payload threshold** であり、
checker verdict carrier detail はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- actual checker result payload は theorem-side retained bridge の field としてのみ扱う。
- checker verdict carrier detail と actual checker verdict payload family は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_result_materialization_family` までは current first choice に入っている。
2. checker result materialization family は bridge 側へ symbolic marker として入るが、actual checker result payload は still 後段に残す cut が固定済みである。
3. `theorem_export_checker_result_payload` family は next practical result payload candidate として最も近い。

したがって current 問いは、
**checker result materialization family の次段として actual checker result payload を retained bridge に近づけるなら、それを current first cut にするべきか、それとも checker verdict carrier detail まで同時に呼ぶべきか**
である。

## 比較観点

1. checker result materialization family と actual checker result payload の line を narrow に切れるか
2. actual checker result payload を retained bridge に足しても、checker verdict carrier detail を still 後段に残せるか
3. theorem-line retained bridge に checker verdict carrier detail を premature に押し込まないか
4. next later reopen を checker-verdict-carrier-detail comparison へ狭く進められるか

## 比較対象

### 案 1. checker result materialization family marker を terminal cut にし、actual checker result payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual checker result payload の premature actualization を避けられる

#### 欠点

- checker result materialization family と actual checker result payload の gap が prose 依存に残りやすい
- actual checker result payload 自体が current bridge では見えない

### 案 2. actual checker result payload だけを持つ retained bridge にする

#### 読み

checker-result-materialization-family-ready retained bridge に、
checker verdict carrier detail を導入せず
**`retained_payload_body_materialization_theorem_export_checker_result_payload`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_result_payload = {
    theorem_export_checker_result_materialization_family_ref =
      retained_payload_body_materialization_theorem_export_checker_result_materialization_family,
    payload_family = theorem_export_checker_result_payload
  }
}
```

#### 利点

- actual checker result payload 自体は current bridge で見える
- checker verdict carrier detail を still 後段に残せる
- next later reopen を checker-verdict-carrier-detail comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- actual checker result payload と checker verdict carrier detail を誤読されない説明が要る

### 案 3. checker verdict carrier detail まで current bridge へ同時に入れる

#### 利点

- checker result line の先を一気に concrete にできる
- actual checker result payload と verdict carrier detail の接続を bridge で直接見える

#### 欠点

- actual checker result payload と checker verdict carrier detail を同じ reopen で混ぜやすい
- theorem-line retained bridge を checker verdict carrier detail へ既成事実化しやすい
- verdict-specific compare / transport / witness line を premature に呼びやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual checker result payload だけを持つ retained bridge にする**
である。

理由は次の通り。

1. checker result materialization family の次段として actual checker result payload 自体は narrow に橋渡しできる
2. checker verdict carrier detail を still 後段に残せる
3. next later reopen を checker-verdict-carrier-detail comparison へ狭く進めやすい

## minimal actual-checker-result-payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_result_payload = {
    theorem_export_checker_result_materialization_family_ref =
      retained_payload_body_materialization_theorem_export_checker_result_materialization_family,
    payload_family = theorem_export_checker_result_payload
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_result_payload`

`retained_payload_body_materialization_theorem_export_checker_result_payload` は、
checker result materialization family の次段として
actual checker result payload 自体を theorem-side retained bridge で最小限に表す field である。

current task では、この payload を checker verdict carrier detail には昇格させない。

## なぜ checker verdict carrier detail をまだ入れないか

checker verdict carrier detail を current phase で入れると、

- checker result materialization family
- actual checker result payload
- checker verdict carrier detail

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず actual checker result payload 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain actual-checker-result-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_result_materialization_family_ready,
  retained_payload_body_materialization_theorem_export_checker_result_payload = {
    theorem_export_checker_result_materialization_family_ref =
      retained_payload_theorem_export_checker_result_materialization_family:e12.latest,
    payload_family = theorem_export_checker_result_payload
  }
}
```

ここで bridge が示すのは actual checker result payload までであり、
checker verdict carrier detail まではまだ bridge に入れない。

### example B — witnessed draw actual-checker-result-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_result_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_result_materialization_family_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_result_materialization_family_ready,
  retained_payload_body_materialization_theorem_export_checker_result_payload = {
    theorem_export_checker_result_materialization_family_ref =
      retained_payload_theorem_export_checker_result_materialization_family:sugoroku.draw.latest,
    payload_family = theorem_export_checker_result_payload
  }
}
```

draw case でも actual checker result payload 自体は bridge で追えるが、
checker verdict carrier detail までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual checker result payload comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_result_payload` だけを足す retained bridge である
- checker verdict carrier detail は still 後段に残す
- next later reopen は checker-result-materialization-family-ready checker-verdict-carrier-detail comparison である

### not decided

- checker verdict carrier detail をどの field / row / payload family で切るか
- `theorem_export_checker_result_payload` を retained bridge のまま維持するか checker verdict carrier detail へ actualize するか
- checker verdict witness / transport line をどの concrete threshold で呼ぶか
