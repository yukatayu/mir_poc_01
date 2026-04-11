# 246 — current L2 theorem line handoff-transport-family-ready minimal-handoff-transport-family threshold

## 目的

`specs/examples/245-current-l2-theorem-line-minimal-handoff-carrier-detail-ready-handoff-transport-family-comparison.md`
で `handoff_carrier_detail_ref + handoff_transport_family` row を current first choice にした判断を前提に、

- handoff transport family row の minimal field core をどこまでに留めるか
- handoff transport carrier detail / payload / receipt line をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-transport-family-ready minimal-handoff-transport-family threshold**
であり、

- final handoff transport carrier detail
- final handoff transport payload
- final handoff transport receipt row

はまだ固定しない。

## 比較観点

1. handoff transport family を source-backed に見せる最小 row になっているか
2. handoff carrier detail と自然に接続しているか
3. transport carrier / payload / receipt row を premature に混ぜないか
4. theorem-line retained bridge の detail cut を保てるか

## 比較対象

### 案 1. `handoff_carrier_detail_ref + next_transport_family` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_transport_family = {
  handoff_carrier_detail_ref =
    retained_payload_body_materialization_theorem_export_handoff_carrier_detail,
  next_transport_family = authority_handoff_transport
}
```

#### 利点

- handoff transport family を source-backed に見せる最小 row である
- transport carrier / payload / receipt row を still later に残せる
- carrier detail と transport family の役割分担が明瞭である

#### 欠点

- actual transport row attachment は still prose 依存である

### 案 2. transport family row に carrier detail / payload / receipt row を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- family first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_carrier_detail_ref + next_transport_family` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_carrier_detail_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_transport_family = {
    handoff_carrier_detail_ref =
      retained_payload_body_materialization_theorem_export_handoff_carrier_detail,
    next_transport_family = authority_handoff_transport
  }
}
```

### この shape でまだ入れないもの

- `handoff_transport_carrier_detail`
- `handoff_transport_payload`
- `handoff_transport_receipt_row`

これらは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
carrier authority_handoff_carrier
transport family authority_handoff_transport
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_carrier -> authority_handoff_transport`

のような handoff transport family row である。

transport carrier / payload / receipt row は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-transport-family-ready handoff-transport-carrier-detail comparison**
に置く。

## open questions

- `next_transport_family` を canonical field name として十分に読めるか
- transport carrier detail を next reopen に置くべきか
- receipt row を transport payload より後段に残す順序でよいか
