# 274 — current L2 public-checker-payload-schema-ready minimal-public-checker-payload-schema threshold

## 目的

`specs/examples/273-current-l2-minimal-checker-payload-supported-kind-summary-ready-public-checker-payload-schema-comparison.md`
で public checker payload schema を
docs-only `public_checker_payload_schema_ready_sketch`
wrapper として 1 段切る判断を fixed した次段として、

- public checker payload schema の minimal field core をどこまでに留めるか
- current checker-side package のどの line を schema wrapper に含めるのが自然か
- public checker API をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
public-checker-payload-schema-ready minimal-public-checker-payload-schema threshold** であり、

- public checker API
- final parser / checker boundary
- emitted public checker payload contract

はまだ固定しない。

## 比較観点

1. current checker-side package を十分に束ねつつ、schema wrapper を still minimal に保てるか
2. source bridge / row grouping / row discriminator / row body / supported inventory の役割分担を崩さないか
3. public checker API や emitted surface を premature に含めないか
4. next 段の public checker API comparison を narrow に保てるか

## 比較対象

### 案 1. `actual_checker_payload_family_ref + checker_payload_supported_kind_summary_ref` だけを持つ light wrapper

#### shape

```text
public_checker_payload_schema_ready_sketch = {
  actual_checker_payload_family_ref = actual_checker_payload_family,
  checker_payload_supported_kind_summary_ref = actual_checker_payload_supported_kind_summary
}
```

#### 利点

- 最も軽い
- source bridge と stable inventory の接点だけは示せる

#### 欠点

- row family / row detail / row body の current line が still implicit に残る
- public checker payload schema と actual supported kind summary の接点は見えても、per-row shape が schema reading で見えにくい

### 案 2. `actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref` を持つ minimal wrapper

#### shape

```text
public_checker_payload_schema_ready_sketch = {
  actual_checker_payload_family_ref = actual_checker_payload_family,
  checker_payload_row_family_ref = actual_checker_payload_row_family,
  checker_payload_row_detail_ref = actual_checker_payload_row_detail,
  checker_payload_row_body_ref = actual_checker_payload_row_body,
  checker_payload_supported_kind_summary_ref = actual_checker_payload_supported_kind_summary
}
```

#### 利点

- current checker-side package をそのまま 1 箇所に束ねられる
- source bridge / row grouping / row discriminator / row body / stable inventory の役割分担を保てる
- public checker API / emitted surface / parser boundary は still later に残せる

#### 欠点

- ref field が 5 本になり、light wrapper よりは 1 段重い
- `*_ref` naming discipline が要る

### 案 3. minimal wrapper に `schema_version` / flattened row entries / public API 近傍 field まで同時に入れる

#### 利点

- public checker-facing surface に一気に近づける

#### 欠点

- current phase では still heavy である
- schema wrapper と public checker API を同じ reopen で混ぜやすい
- current checker-side docs-first line を actual emitted contract と誤読させやすい

## current judgment

current L2 で最も自然なのは、
**案 2. 5 ref を持つ minimal wrapper**
である。

理由は次の通り。

1. public checker payload schema を切る以上、current checker-side package の各 line を schema reading で追える方が自然である
2. ただし `schema_version` / flattened row entries / public checker API 近傍 field まで持ち込むのは still early である
3. 5 ref wrapper なら current package を 1 箇所に束ねつつ、public checker API を still later に残せる

## current first choice shape

```text
public_checker_payload_schema_ready_sketch = {
  actual_checker_payload_family_ref = actual_checker_payload_family,
  checker_payload_row_family_ref = actual_checker_payload_row_family,
  checker_payload_row_detail_ref = actual_checker_payload_row_detail,
  checker_payload_row_body_ref = actual_checker_payload_row_body,
  checker_payload_supported_kind_summary_ref = actual_checker_payload_supported_kind_summary
}
```

### この shape でまだ入れないもの

- `schema_version`
- flattened public checker row entries
- emitted payload transport / receipt / channel surface
- public checker API contract

これらは still later である。

## practical reading

current wrapper が示すのは、

- source bridge は `actual_checker_payload_family_ref`
- row grouping は `checker_payload_row_family_ref`
- per-row discriminator は `checker_payload_row_detail_ref`
- variant-local row body は `checker_payload_row_body_ref`
- stable subset inventory は `checker_payload_supported_kind_summary_ref`

の 5 つが current public checker payload schema reading に属する、ということだけである。

individual row flattening や emitted transport はまだ schema wrapper に入れない。

## next promoted line

next promoted line は、
**minimal-public-checker-payload-schema-ready public-checker-api comparison**
に置く。

## open questions

- `actual_checker_payload_family_ref` と `checker_payload_*_ref` の naming family を later でどこまで shared token にするか
- flattened public checker row schema を later でどの line から reopen するか
- public checker API を schema wrapper の次段でどこまで narrow に切るべきか
