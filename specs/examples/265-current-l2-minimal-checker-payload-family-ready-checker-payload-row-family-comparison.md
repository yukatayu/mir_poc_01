# 265 — current L2 minimal-checker-payload-family-ready checker-payload-row-family comparison

## 目的

`specs/examples/264-current-l2-actual-checker-payload-family-ready-minimal-checker-payload-family-threshold.md`
で actual checker payload family の minimal shape を
`payload_family_kind + source_refs`
に留める判断を固定した次段として、

- checker payload family の次に checker payload row family を docs-first line として切るべきか
- row family を切るならどの field まで current line に持ち込むのが自然か
- `supported_kind_refs[]` や actual row payload をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-checker-payload-family-ready checker-payload-row-family comparison** であり、

- actual checker row payload schema
- supported kind detail
- public checker API

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- `actual_checker_payload_family` の次段としてだけ row family を比較する。
- actual checker implementation / public checker API には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix line は `coverage_state` で current package close にしてよい。
2. current next step として actual checker payload family を docs-first bridge に切ってよい。
3. current minimal checker payload family は、
   - `payload_family_kind`
   - `source_refs`
   を持つ minimal bundle である。
4. `supported kind` summary は current checker-cluster matrix にも payload family にもまだ入れない。

したがって current 問いは、
**minimal checker payload family の次段として checker payload row family を 1 段切るべきか、切るならどこまでが current first choice か**
である。

## 比較観点

1. payload family と row family の役割分担が明瞭か
2. source bridge と row-family marker を混ぜすぎないか
3. supported kind detail や actual row payload を premature に固定しないか
4. next 段で checker payload row detail を narrow に比較しやすいか

## 比較対象

### 案 1. payload family で止め、row family は still prose に残す

#### 利点

- line は最も軽い
- public checker payload schema 誤読を増やしにくい

#### 欠点

- payload family の次段として actual row grouping がどこにあるか still prose 依存である
- checker payload row detail へ進む narrow bridge が弱い

### 案 2. `payload_family_ref + row_family_kind` を持つ checker payload row family を足す

#### shape

```text
actual_checker_payload_row_family = {
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows
}
```

#### 利点

- payload family と row family の役割分担が明瞭である
- source bridge は payload family に留めたまま、row-family marker だけを next line に切れる
- supported kind detail / actual row payload を still later に残せる
- next 段で checker payload row detail を narrow に比較しやすい

#### 欠点

- `row_family_kind` naming discipline が要る
- marker を 1 段増やすぶん prose-only よりは重い

### 案 3. row family に `supported_kind_refs[]` や actual row payload を同時に足す

#### 利点

- actual checker payload detail への接続が強く見える

#### 欠点

- row family と actual row payload の役割を同時に読ませやすい
- supported kind detail を premature に固定しやすい
- next 段の row detail comparison を先食いしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `payload_family_ref + row_family_kind` を持つ checker payload row family を足す**
である。

理由は次の通り。

1. payload family が source bridge を担い、row family が row grouping marker を担う、という役割分担が最も明瞭である
2. `supported kind` summary や actual row payload は still later に残せる
3. checker payload row detail へ進む narrow ratchet を保てる

## current reading

current docs-only では、

```text
actual_checker_payload_row_family = {
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows
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

actual_checker_payload_row_family = {
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows
}
```

この current reading が示すのは、

- source bridge は `actual_checker_payload_family`
- row grouping marker は `actual_checker_payload_row_family`

という 2 段の cut だけであり、
kind summary や actual row payload まではまだ示さない。

## next promoted line

next promoted line は、
**checker-payload-row-family-ready minimal-checker-payload-row-family threshold**
に置く。

そこで比べる first candidates は、少なくとも次である。

1. `payload_family_ref + row_family_kind`
2. 上に `supported_kind_refs[]` を足す
3. 上に actual row payload marker を足す

## what is decided here

### decided

- checker payload family の次段として checker payload row family を docs-first line に切ってよい
- current first choice は `payload_family_ref + row_family_kind` である
- supported kind detail / actual row payload は still later に残す

### not decided

- minimal checker payload row family の final naming
- checker payload row detail をどこまで current line に出すか
- actual checker schema / public checker API

## open questions

- `checked_reason_code_rows` を current row-family kind token として十分に読めるか
- checker payload row detail の first cut をどこまでに留めるべきか
- future protocol-side / theorem-side checker payload row family と naming を共有できるか
