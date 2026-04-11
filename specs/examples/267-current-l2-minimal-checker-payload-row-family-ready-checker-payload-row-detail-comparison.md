# 267 — current L2 minimal-checker-payload-row-family-ready checker-payload-row-detail comparison

## 目的

`specs/examples/266-current-l2-checker-payload-row-family-ready-minimal-checker-payload-row-family-threshold.md`
で checker payload row family の minimal shape を
`payload_family_ref + row_family_kind`
に留める判断を fixed した次段として、

- checker payload row family の次に checker payload row detail を docs-first line として切るべきか
- row detail を切るならどこまでが current first choice か
- actual variant payload body をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-checker-payload-row-family-ready checker-payload-row-detail comparison** であり、

- actual checker row body
- supported kind detail
- public checker payload schema

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- `actual_checker_payload_row_family` の次段としてだけ row detail を比較する。
- actual checker implementation / public checker API には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix line は `coverage_state` で package close にしてよい。
2. actual checker payload family は `payload_family_kind + source_refs` minimal bundle まで切れている。
3. checker payload row family は `payload_family_ref + row_family_kind` minimal bundle まで切れている。
4. `checked_reason_codes` と detached static gate `reason_codes` は actual source evidence であり、各 row は `StaticReasonCodeRow` variant として materialize される。

したがって current 問いは、
**checker payload row family の次段として row detail を 1 段切るべきか、切るならどの field までが current first choice か**
である。

## 比較観点

1. row family と row detail の役割分担が明瞭か
2. source array と reason-code variant の接点を narrow に見せられるか
3. variant payload body を premature に固定しないか
4. next 段で checker payload row body を narrow に比較しやすいか

## 比較対象

### 案 1. row family で止め、row detail は still prose に残す

#### 利点

- line は最も軽い
- actual row body 誤読を増やしにくい

#### 欠点

- `StaticReasonCodeRow` の actual variant line が still prose 依存である
- row body comparison へ進む narrow bridge が弱い

### 案 2. `payload_row_family_ref + row_source_ref + row_reason_kind` を持つ row detail を足す

#### shape

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = missing_lineage_assertion
}
```

#### 利点

- row family と row detail の役割分担が明瞭である
- source array と reason-code variant の接点を narrow に見せられる
- variant payload body を still later に残せる
- next 段で checker payload row body を narrow に比較しやすい

#### 欠点

- `row_reason_kind` naming discipline が要る
- row source と body payload を別段に読む discipline が要る

### 案 3. row detail に variant payload body を同時に足す

#### 利点

- actual checker payload schema への接続が強く見える

#### 欠点

- row detail と row body の役割を同時に読ませやすい
- actual `StaticReasonCodeRow` payload を premature に docs-first line に焼き込みやすい
- next 段の row body comparison を先食いしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `payload_row_family_ref + row_source_ref + row_reason_kind` を持つ row detail を足す**
である。

理由は次の通り。

1. row family が grouping marker を担い、row detail が source array と reason-code variant の接点を担う、という役割分担が明瞭である
2. variant payload body は still later に残せる
3. current Phase 5 の docs-first ratchet を崩さずに actual row body comparison へ進める

## current reading

current docs-only では、

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = missing_lineage_assertion
}
```

まででよい。

### practical example

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = detached_static_gate_reason_codes,
  row_reason_kind = capability_strengthens
}
```

この current reading が示すのは、

- row はどの source array から来たか
- row はどの reason-code variant か

という 2 点だけであり、
variant payload body はまだ current task で扱わない。

## next promoted line

next promoted line は、
**checker-payload-row-detail-ready minimal-checker-payload-row-detail threshold**
に置く。

## what is decided here

### decided

- checker payload row family の次段として checker payload row detail を docs-first line に切ってよい
- current first choice は `payload_row_family_ref + row_source_ref + row_reason_kind` である
- actual row body は still later に残す

### not decided

- minimal checker payload row detail の final naming
- actual checker row body をどこまで current line に出すか
- public checker payload schema / public checker API

## open questions

- `fixture_checked_reason_codes` / `detached_static_gate_reason_codes` を row source token として十分に読めるか
- `row_reason_kind` の token set をどこまで current phase で stable にしてよいか
- actual row body を next line でどこまで narrow に切るべきか
