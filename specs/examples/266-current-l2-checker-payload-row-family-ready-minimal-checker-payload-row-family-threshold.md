# 266 — current L2 checker-payload-row-family-ready minimal-checker-payload-row-family threshold

## 目的

`specs/examples/265-current-l2-minimal-checker-payload-family-ready-checker-payload-row-family-comparison.md`
で checker payload row family を
`payload_family_ref + row_family_kind`
まで切る判断を fixed した次段として、

- checker payload row family の minimal field core をどこまでに留めるか
- `supported_kind_refs[]` や actual row payload marker をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
checker-payload-row-family-ready minimal-checker-payload-row-family threshold** であり、

- actual checker payload row detail
- supported kind detail
- actual checker payload schema / public checker API

はまだ固定しない。

## 比較観点

1. payload family と自然に接続しているか
2. row family を source-backed に見せる最小 row になっているか
3. supported kind detail や actual row payload marker を premature に混ぜないか
4. checker payload row detail の next reopen を narrow に保てるか

## 比較対象

### 案 1. `payload_family_ref + row_family_kind` だけを持つ minimal row family

#### shape

```text
actual_checker_payload_row_family = {
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows
}
```

#### 利点

- payload family と row family の役割分担が明瞭である
- supported kind detail / actual row payload を still later に残せる
- next 段の row detail first cut を軽く保てる

#### 欠点

- supported kind inventory は still prose 依存である

### 案 2. row family に `supported_kind_refs[]` を同時に足す

#### 利点

- kind coverage を早く見せやすい

#### 欠点

- row family と supported kind summary を同時に読ませやすい
- current checker-cluster matrix line で避けた premature kind summary を別 line で再導入しやすい

### 案 3. row family に actual row payload marker を同時に足す

#### 利点

- next 段の row detail への接続が直接見える

#### 欠点

- row family first cut が重くなる
- actual checker payload schema / public checker API に近づきすぎる

## current judgment

current L2 で最も自然なのは、
**案 1. `payload_family_ref + row_family_kind` だけを持つ minimal row family**
である。

## current first choice shape

```text
actual_checker_payload_family_ready_sketch = {
  actual_checker_payload_family = {
    payload_family_kind = static_reason_code_row_family,
    source_refs = [
      fixture_checked_reason_codes,
      detached_static_gate_reason_codes
    ]
  },
  actual_checker_payload_row_family = {
    payload_family_ref = actual_checker_payload_family,
    row_family_kind = checked_reason_code_rows
  }
}
```

### この shape でまだ入れないもの

- `supported_kind_refs`
- `checker_payload_rows`
- actual public checker row payload marker

これらは still later である。

## practical example

```text
fixture expected_static.checked_reason_codes
detached static gate reason_codes
```

current row family line に今ほしいのは、

- `static_reason_code_row_family`
- `checked_reason_code_rows`

の 2 段がつながっている、という grouping だけである。
各 row の payload や supported kind inventory は、まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-checker-payload-row-family-ready checker-payload-row-detail comparison**
に置く。

## open questions

- `checked_reason_code_rows` の canonical token set をどこまで current phase で固定してよいか
- checker payload row detail を `row_subject_ref + row_reason_kind` のような first cut で十分に読めるか
- supported kind detail を row detail より後段に残す順序でよいか
