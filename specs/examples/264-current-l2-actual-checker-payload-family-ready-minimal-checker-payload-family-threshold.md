# 264 — current L2 actual-checker-payload-family-ready minimal-checker-payload-family threshold

## 目的

`specs/examples/263-current-l2-supported-kind-summary-ready-actual-checker-payload-family-comparison.md`
で actual checker payload family を docs-first bridge として 1 段切る判断を固定した次段として、

- actual checker payload family をどの field で止めるのが最小か
- `checked_reason_codes` / `reason_codes` の actual source との接点をどこまで current line に持ち込むべきか
- `supported_kind_refs[]` のような kind summary を family 側へ immediate に持ち込むべきか

を比較する。

ここで固定するのは **minimal checker payload family の current threshold** であり、

- actual checker schema
- public checker API
- supported kind detail の final payload

はまだ固定しない。

## scope

- current L2 checker-side docs-first line を扱う。
- actual machine-check source として `expected_static.checked_reason_codes` / detached static gate `reason_codes` を参照する。
- actual checker implementation や public checker API には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix line は `coverage_state` までで止める方が自然である。
2. current next step として actual checker payload family を docs-first bridge として 1 段切ってよい。
3. current actual machine-check source としては、
   - fixture-side `expected_static.checked_reason_codes`
   - detached static gate `reason_codes`
   がある。

したがって current 問いは、
**actual checker payload family の minimal docs-first shape をどこで止めるべきか**
である。

## 比較観点

1. actual machine-check source との橋渡しを十分に見せられるか
2. public checker API や actual payload schema を premature に固定しないか
3. `supported_kind_refs[]` のような detail を still later に残せるか
4. next 段で checker payload row family へ自然に進めるか

## 比較対象

### 案 1. `payload_family_kind` だけの marker に留める

```text
actual_checker_payload_family = {
  payload_family_kind = static_reason_code_row_family
}
```

#### 利点

- 最も軽い
- public schema 誤読を抑えやすい

#### 欠点

- current machine-check source との接点が still 弱い
- checker-cluster matrix line から actual source への bridge としては情報が足りない

### 案 2. `payload_family_kind + source_refs` を持つ minimal bundle にする

```text
actual_checker_payload_family = {
  payload_family_kind = static_reason_code_row_family,
  source_refs = [
    fixture_checked_reason_codes,
    detached_static_gate_reason_codes
  ]
}
```

#### 利点

- current actual machine-check source との bridge を明示できる
- actual payload schema や kind detail までは still later に残せる
- next 段で checker payload row family を narrow に比較しやすい

#### 欠点

- symbolic ref naming discipline が要る
- marker-only よりは 1 段重い

### 案 3. `payload_family_kind + source_refs + supported_kind_refs[]` まで current line に入れる

```text
actual_checker_payload_family = {
  payload_family_kind = static_reason_code_row_family,
  source_refs = [...],
  supported_kind_refs = [...]
}
```

#### 利点

- current actual machine-check source と kind inventory の接点を最も直接に見せられる

#### 欠点

- current line を actual checker payload schema に近づけすぎる
- stable cluster inventory の current corpus state を premature に family row へ焼き込みやすい
- next 段の row family comparison を先食いしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `payload_family_kind + source_refs` を持つ minimal bundle**
である。

理由は次の通り。

1. actual checker payload family を切る以上、current machine-check source との bridge は明示した方が自然である
2. しかし `supported_kind_refs[]` を足すと actual payload detail に近づきすぎる
3. `payload_family_kind + source_refs` なら docs-first bridge として十分であり、row family detail は still later に残せる

## current reading

current docs-only では、

```text
actual_checker_payload_family = {
  payload_family_kind = static_reason_code_row_family,
  source_refs = [
    fixture_checked_reason_codes,
    detached_static_gate_reason_codes
  ]
}
```

まででよい。

### practical example

```text
actual_checker_payload_family = {
  payload_family_kind = static_reason_code_row_family,
  source_refs = [
    fixture_checked_reason_codes,
    detached_static_gate_reason_codes
  ]
}
```

この bundle が示すのは、

- fixture corpus 側の machine-check source
- detached static gate 側の machine-check source

の 2 本が current same family に属する、という bridge だけである。
個別 kind set や row payload detail まではまだ示さない。

## next promoted line

next promoted line は、
**minimal-checker-payload-family-ready checker-payload-row-family comparison**
に置く。

ここで比べるのは、

1. row family を still prose に残す
2. row family を docs-first comparison として 1 段切る
3. supported kind detail / actual schema まで同時に呼ぶ

のどこまでが current first choice として自然かである。

## what is decided here

### decided

- actual checker payload family は docs-first bridge として切ってよい
- current minimal shape は `payload_family_kind + source_refs` まででよい
- `supported_kind_refs[]` は still later に残す

### not decided

- checker payload row family をどこまで current line に出すか
- actual checker schema / public checker API
- supported kind detail / transport / witness

## open questions

- `fixture_checked_reason_codes` と `detached_static_gate_reason_codes` の symbolic namespace を later でどう揃えるか
- checker payload row family を current first choice としてどの粒度で切るか
- future parser-side / protocol-side checker payload family と naming を共有できるか
