# 268 — current L2 checker-payload-row-detail-ready minimal-checker-payload-row-detail threshold

## 目的

`specs/examples/267-current-l2-minimal-checker-payload-row-family-ready-checker-payload-row-detail-comparison.md`
で checker payload row detail を
`payload_row_family_ref + row_source_ref + row_reason_kind`
まで切る判断を fixed した次段として、

- checker payload row detail の minimal field core をどこまでに留めるか
- actual row body をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
checker-payload-row-detail-ready minimal-checker-payload-row-detail threshold** であり、

- actual checker row body
- supported kind detail
- public checker payload schema / public checker API

はまだ固定しない。

## 比較観点

1. row family と自然に接続しているか
2. row detail を source-backed に見せる最小 row になっているか
3. actual row body を premature に混ぜないか
4. next 段の row body first cut を narrow に保てるか

## 比較対象

### 案 1. `payload_row_family_ref + row_source_ref + row_reason_kind` だけを持つ minimal row detail

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
- actual row body を still later に残せる
- next 段の row body first cut を軽く保てる

#### 欠点

- actual row body payload は still prose 依存である

### 案 2. row detail に actual row body marker を同時に足す

#### 利点

- actual checker payload schema への接続が直接見える

#### 欠点

- row detail first cut が重くなる
- `StaticReasonCodeRow` variant payload を premature に docs-first line に焼き込みやすい

### 案 3. row detail に supported kind summary を同時に足す

#### 利点

- kind inventory を row detail 側でまとめて見せやすい

#### 欠点

- current checker-cluster matrix / row family line で避けた summary creep を再導入しやすい
- row detail と summary の役割分担が曖昧になる

## current judgment

current L2 で最も自然なのは、
**案 1. `payload_row_family_ref + row_source_ref + row_reason_kind` だけを持つ minimal row detail**
である。

## current first choice shape

```text
actual_checker_payload_row_family_ready_sketch = {
  actual_checker_payload_row_family = {
    payload_family_ref = actual_checker_payload_family,
    row_family_kind = checked_reason_code_rows
  },
  actual_checker_payload_row = {
    payload_row_family_ref = actual_checker_payload_row_family,
    row_source_ref = fixture_checked_reason_codes,
    row_reason_kind = missing_lineage_assertion
  }
}
```

### この shape でまだ入れないもの

- `row_body_ref`
- actual `StaticReasonCodeRow` payload fields
- `supported_kind_refs`

これらは still later である。

## practical example

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = detached_static_gate_reason_codes,
  row_reason_kind = declared_target_mismatch
}
```

current row detail line に今ほしいのは、

- row がどの source array から来たか
- row がどの reason-code variant か

の 2 点だけである。
lineage edge、option name、scope などの actual payload body は、まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-checker-payload-row-detail-ready checker-payload-row-body comparison**
に置く。

## open questions

- `row_source_ref` token を later actual checker payload でも共有できるか
- `row_reason_kind` を current phase でどこまで stable enum-like token として読めるか
- actual row body の first cut をどこまでに留めるべきか
