# 273 — current L2 minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison

## 目的

`specs/examples/272-current-l2-checker-payload-supported-kind-summary-ready-minimal-checker-payload-supported-kind-summary-threshold.md`
で checker payload supported kind summary を
`payload_row_family_ref + supported_kind_scope + supported_kind_refs`
minimal bundle に留める判断を fixed した次段として、

- checker payload side の current package の後に public checker payload schema を docs-first line として切るべきか
- public checker payload schema を切るならどの layer が current first choice か
- public checker API をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison** であり、

- public checker API
- final parser / checker boundary
- final checker implementation contract

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- actual checker payload family / row family / row detail / row body / supported kind summary までを前提にする。
- public checker API / emitted artifact / transport surface には進まない。

## current 前提

current repo では次が成立している。

1. actual checker payload family は `payload_family_kind + source_refs` minimal bundle まで切れている。
2. checker payload row family は `payload_family_ref + row_family_kind` minimal bundle まで切れている。
3. checker payload row detail は `payload_row_family_ref + row_source_ref + row_reason_kind` minimal bundle まで切れている。
4. checker payload row body は `row_body` variant-local slot bundle まで切れている。
5. checker payload supported kind summary は `payload_row_family_ref + supported_kind_scope + supported_kind_refs` minimal bundle まで切れている。
6. actual source では、`crates/mir-semantics/src/lib.rs` の `StaticReasonCodeRow` enum、fixture-side `expected_static.checked_reason_codes`、detached static gate `reason_codes` / `reason_codes_scope`、`scripts/current_l2_reason_codes_assist.py` の helper reading が already あり、checker-side vocabulary と stable subset inventory の接点は source-backed に見えている。

したがって current 問いは、
**checker payload supported kind summary の次段として public checker payload schema を 1 段切るべきか、切るならどの layer が current first choice か**
である。

## 比較観点

1. already cut 済みの checker-side line を user-facing schema reading として 1 箇所に束ねられるか
2. row family / row detail / row body / supported kind summary の役割分担を崩さないか
3. public checker API や final parser boundary を premature に固定しないか
4. next 段の minimal public checker payload schema threshold を narrow に保てるか

## 比較対象

### 案 1. public checker payload schema は still prose に残す

#### 利点

- line は最も軽い
- public surface の premature actualization を避けやすい

#### 欠点

- actual checker payload family / row family / row detail / row body / supported kind summary の current package が still fragmented に見える
- public checker payload schema と public checker API の境界が prose 依存に残る

### 案 2. docs-only `public_checker_payload_schema_ready_sketch` wrapper を 1 段切る

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

- current checker-side package を 1 つの docs-first schema reading として束ねられる
- source bridge / row grouping / row discriminator / row body / supported inventory の役割分担を保てる
- public checker API や transport surface は still later に残せる
- next 段の minimal public checker payload schema threshold を narrow に比較しやすい

#### 欠点

- docs-only wrapper が 1 段増える
- `*_ref` naming discipline が要る

### 案 3. flattened public row schema ないし public checker API へ直接進む

#### 読み

public checker payload schema を
serializer-like row list ないし public checker API に近い carrier として扱い、
row detail / row body / supported summary を flatten した public surface へ直接進む。

#### 利点

- public checker-facing surface を早く見せられる

#### 欠点

- current phase では still early である
- row family / row detail / row body / supported summary の分離を崩しやすい
- final parser / checker boundary と public checker API を premature に既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. docs-only `public_checker_payload_schema_ready_sketch` wrapper を 1 段切る**
である。

理由は次の通り。

1. actual checker payload family / row family / row detail / row body / supported kind summary の current package は already source-backed に切れており、次段ではそれらを 1 箇所に束ねる schema reading を置く方が自然である
2. wrapper sketch に留めれば public checker API を still later に残せる
3. flattened public row schema や public checker API へ直接進むと、current checker-side docs-first line を actual public contract と誤読させやすい

## current reading

current docs-only では、
checker payload supported kind summary の次段として
**`public_checker_payload_schema_ready_sketch`**
を 1 段切ってよい。

この wrapper が示すのは、

- current checker-side package を public checker payload schema reading としてまとめてよいこと
- ただし schema wrapper 自体は docs-only であり、public checker API ではないこと

の 2 点だけである。

### practical example

```text
public_checker_payload_schema_ready_sketch = {
  actual_checker_payload_family_ref = actual_checker_payload_family,
  checker_payload_row_family_ref = actual_checker_payload_row_family,
  checker_payload_row_detail_ref = actual_checker_payload_row_detail,
  checker_payload_row_body_ref = actual_checker_payload_row_body,
  checker_payload_supported_kind_summary_ref = actual_checker_payload_supported_kind_summary
}
```

この current reading で still later に残すものは、

- public checker API contract
- parser / checker public boundary
- emitted artifact / transport surface

である。

## next promoted line

next promoted line は、
**public-checker-payload-schema-ready minimal-public-checker-payload-schema threshold**
に置く。

## what is decided here

### decided

- checker payload supported kind summary の次段として public checker payload schema を docs-first line に切ってよい
- current first choice は `public_checker_payload_schema_ready_sketch` wrapper である
- public checker API は still later に残す

### not decided

- minimal public checker payload schema の final field set
- `*_ref` naming を current phase でどこまで stable token として読むか
- public checker API / parser boundary / emitted artifact surface

## open questions

- minimal public checker payload schema を 5 ref wrapper に留めるべきか、さらに軽い cut があるか
- row detail / row body を later で flattened public row schema へ寄せる first reopen cut をどこへ置くべきか
- public checker API の first reopen cut を schema wrapper の後にどこへ置くべきか
