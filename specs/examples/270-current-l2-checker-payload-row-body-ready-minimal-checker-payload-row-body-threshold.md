# 270 — current L2 checker-payload-row-body-ready minimal-checker-payload-row-body threshold

## 目的

`specs/examples/269-current-l2-minimal-checker-payload-row-detail-ready-checker-payload-row-body-comparison.md`
で checker payload row body を
`row_body` variant-local slot bundle まで切る判断を fixed した次段として、

- checker payload row body の minimal field core をどこまでに留めるか
- actual `StaticReasonCodeRow` serializer shape をどこまで still later に残すか
- supported kind summary をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
checker-payload-row-body-ready minimal-checker-payload-row-body threshold** であり、

- supported kind summary
- public checker payload schema
- public checker API

はまだ固定しない。

## 比較観点

1. row detail と自然に接続しているか
2. actual variant-local payload slot を source-backed に見せる最小 body になっているか
3. `row_reason_kind` や source ref を duplicate に混ぜないか
4. next 段の supported kind summary comparison を narrow に保てるか

## 比較対象

### 案 1. `row_body` に variant-local slot bundle だけを持たせる

#### shape

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = fixture_checked_reason_codes,
  row_reason_kind = missing_lineage_assertion,
  row_body = {
    predecessor = primary,
    successor = mirror
  }
}
```

#### 利点

- row detail と row body の役割分担が明瞭である
- actual variant-local payload slot を narrow に見せられる
- `row_reason_kind` / `row_source_ref` を detail 側に残せる
- supported kind summary を still later に残せる

#### 欠点

- variant-local slot shape は still prose 補助が要る

### 案 2. `row_body` に `kind` echo まで持たせる

#### 利点

- existing `StaticReasonCodeRow` serializer との近さは上がる

#### 欠点

- `row_reason_kind` と `row_body.kind` が duplicate になる
- row detail と row body の切り分けが崩れる
- docs-first line を serializer shape へ寄せすぎる

### 案 3. `row_body` に supported kind summary まで同時に足す

#### 利点

- row payload と kind inventory を同じ line で見せやすい

#### 欠点

- row body と summary の役割分担が曖昧になる
- current checker-side line で繰り返し避けてきた summary creep を再導入しやすい
- next 段の supported kind summary comparison を先食いしやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `row_body` に variant-local slot bundle だけを持たせる**
である。

## current first choice shape

```text
actual_checker_payload_row_ready_sketch = {
  actual_checker_payload_row = {
    payload_row_family_ref = actual_checker_payload_row_family,
    row_source_ref = fixture_checked_reason_codes,
    row_reason_kind = missing_lineage_assertion,
    row_body = {
      predecessor = primary,
      successor = mirror
    }
  }
}
```

### practical example

```text
actual_checker_payload_row = {
  payload_row_family_ref = actual_checker_payload_row_family,
  row_source_ref = detached_static_gate_reason_codes,
  row_reason_kind = missing_chain_head_option,
  row_body = {
    head = profile_ref,
    scope = profile_access
  }
}
```

### この shape でまだ入れないもの

- `row_body.kind`
- `supported_kind_refs`
- public checker payload row schema

これらは still later である。

## next promoted line

next promoted line は、
**minimal-checker-payload-row-body-ready checker-payload-supported-kind-summary comparison**
に置く。

## open questions

- `row_body` の variant-local slot shape を docs-first line から actual checker payload schema へどう橋渡しするか
- supported kind summary を row body の後段でどこまで narrow に比較するか
- missing-option family の `head` / `option` split を current phase でどこまで stable に読むか
