# 271 — current L2 minimal-checker-payload-row-body-ready checker-payload-supported-kind-summary comparison

## 目的

`specs/examples/270-current-l2-checker-payload-row-body-ready-minimal-checker-payload-row-body-threshold.md`
で checker payload row body を
`row_body` variant-local slot bundle
まで切る判断を fixed した次段として、

- checker payload row body の次に checker payload supported kind summary を docs-first line として切るべきか
- supported kind summary を切るならどの layer へ置くのが自然か
- actual stable subset inventory をどこまで current line に見せるべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-checker-payload-row-body-ready checker-payload-supported-kind-summary comparison** であり、

- public checker payload schema
- public checker API
- final checker implementation contract

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- `actual_checker_payload_row` の次段としてだけ supported kind summary を比較する。
- actual checker implementation / public checker API には進まない。

## current 前提

current repo では次が成立している。

1. checker-cluster matrix line は `coverage_state` で package close にしてよい。
2. actual checker payload family は `payload_family_kind + source_refs` minimal bundle まで切れている。
3. checker payload row family は `payload_family_ref + row_family_kind` minimal bundle まで切れている。
4. checker payload row detail は `payload_row_family_ref + row_source_ref + row_reason_kind` minimal bundle まで切れている。
5. checker payload row body は `row_body` variant-local slot bundle まで切れている。
6. actual source では、`crates/mir-semantics/src/lib.rs` の `is_supported_checked_reason_code` が current stable subset を持ち、detached static gate artifact には `reason_codes_scope = "stable-clusters-only"` が already ある。

したがって current 問いは、
**checker payload row body の次段として supported kind summary を 1 段切るべきか、切るならどの layer が current first choice か**
である。

## 比較観点

1. payload family / row family / row detail / row body と summary の役割分担が明瞭か
2. current stable subset inventory を source-backed に narrow に見せられるか
3. `row_reason_kind` や `row_body` を duplicate に読ませないか
4. next 段の public checker payload schema comparison を narrow に保てるか

## 比較対象

### 案 1. supported kind summary は still prose に残す

#### 利点

- line は最も軽い
- public checker payload schema 誤読を増やしにくい

#### 欠点

- current stable subset inventory が still prose 依存に残る
- `is_supported_checked_reason_code` と detached `reason_codes_scope` の current source-backed 接点を docs 上で見せにくい

### 案 2. `payload_row_family_ref` keyed な独立 summary line を足す

#### shape

```text
actual_checker_payload_supported_kind_summary = {
  payload_row_family_ref = actual_checker_payload_row_family,
  supported_kind_scope = stable_clusters_only,
  supported_kind_refs = [
    missing_lineage_assertion,
    lineage_assertion_edge_mismatch,
    declared_target_missing,
    declared_target_mismatch,
    capability_strengthens,
    missing_chain_head_option,
    missing_predecessor_option,
    missing_successor_option
  ]
}
```

#### 利点

- payload family は source bridge、row family は row grouping、row detail / row body は actual row、summary は stable kind inventory、という役割分担が明瞭である
- current stable subset inventory を source-backed に narrow に見せられる
- `row_reason_kind` や `row_body` を row 単位の情報として残せる
- next 段の public checker payload schema comparison を still later に残せる

#### 欠点

- docs-first object が 1 段増える
- summary naming discipline が要る

### 案 3. `supported_kind_refs[]` を row family に直接畳み込む

#### shape

```text
actual_checker_payload_row_family = {
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows,
  supported_kind_refs = [...]
}
```

#### 利点

- object 数は増えない
- row family と summary を同じ family line で見せられる

#### 欠点

- row grouping marker と stable kind inventory の役割分担が曖昧になる
- `supported_kind_scope` を later で明示しにくい
- row family minimal cut を後段 summary 都合で厚くしやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `payload_row_family_ref` keyed な独立 summary line を足す**
である。

理由は次の通り。

1. current package では payload family / row family / row detail / row body の役割分担が already 明瞭であり、supported kind summary も独立 line にした方が ratchet を崩さない
2. `is_supported_checked_reason_code` の stable subset と detached `reason_codes_scope` を、row family に畳み込まずに narrow に actualize できる
3. public checker payload schema は still later に残せる

## current reading

current docs-only では、
checker payload row body の次段として
**`actual_checker_payload_supported_kind_summary`**
を docs-first line に切ってよい。

この line は、

- `payload_row_family_ref` で row family にぶら下がる
- `supported_kind_scope` で current stable subset の境界を示す
- `supported_kind_refs` で current supported inventory を示す

という 3 つの役割だけに留める。

### practical example

```text
actual_checker_payload_row_family = {
  payload_family_ref = actual_checker_payload_family,
  row_family_kind = checked_reason_code_rows
}

actual_checker_payload_supported_kind_summary = {
  payload_row_family_ref = actual_checker_payload_row_family,
  supported_kind_scope = stable_clusters_only,
  supported_kind_refs = [
    missing_lineage_assertion,
    lineage_assertion_edge_mismatch,
    declared_target_missing,
    declared_target_mismatch,
    capability_strengthens,
    missing_chain_head_option,
    missing_predecessor_option,
    missing_successor_option
  ]
}
```

この current reading が示すのは、

- current stable subset inventory は row family 単位の summary として見せる
- row detail / row body の per-row actualization とは別段に読む

という分離だけである。

## next promoted line

next promoted line は、
**checker-payload-supported-kind-summary-ready minimal-checker-payload-supported-kind-summary threshold**
に置く。

## what is decided here

### decided

- checker payload row body の次段として checker payload supported kind summary を docs-first line に切ってよい
- current first choice は `payload_row_family_ref` keyed な独立 summary line である
- public checker payload schema は still later に残す

### not decided

- minimal checker payload supported kind summary の final field set
- `supported_kind_scope` と detached `reason_codes_scope` の naming をどこまで揃えるか
- public checker payload schema / public checker API

## open questions

- `supported_kind_scope` を current phase でどこまで stable token として読めるか
- `supported_kind_refs` の ordering を enum order / support filter order のどちらに揃えるべきか
- public checker payload schema の first reopen cut を summary の後にどこへ置くべきか
