# 248 — current L2 theorem line handoff-transport-carrier-detail-ready minimal-handoff-transport-carrier-detail threshold

## 目的

`specs/examples/247-current-l2-theorem-line-minimal-handoff-transport-family-ready-handoff-transport-carrier-detail-comparison.md`
で `handoff_transport_family_ref + handoff_transport_carrier_detail` row を current first choice にした判断を前提に、

- handoff transport carrier detail row の minimal field core をどこまでに留めるか
- handoff transport payload / receipt line をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-transport-carrier-detail-ready minimal-handoff-transport-carrier-detail threshold**
であり、

- final handoff transport payload
- final handoff transport receipt row

はまだ固定しない。

## 比較観点

1. handoff transport carrier detail を source-backed に見せる最小 row になっているか
2. handoff transport family と自然に接続しているか
3. transport payload / receipt row を premature に混ぜないか
4. theorem-line retained bridge の detail cut を保てるか

## 比較対象

### 案 1. `handoff_transport_family_ref + handoff_transport_carrier_detail` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_transport_carrier_detail = {
  handoff_transport_family_ref =
    retained_payload_body_materialization_theorem_export_handoff_transport_family,
  handoff_transport_carrier_detail = authority_handoff_transport_carrier
}
```

#### 利点

- handoff transport carrier detail を source-backed に見せる最小 row である
- transport payload / receipt row を still later に残せる
- transport family と carrier detail の役割分担が明瞭である

#### 欠点

- actual payload / receipt row attachment は still prose 依存である

### 案 2. transport carrier detail row に payload / receipt row を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- carrier-detail first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_transport_family_ref + handoff_transport_carrier_detail` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_transport_carrier_detail = {
    handoff_transport_family_ref =
      retained_payload_body_materialization_theorem_export_handoff_transport_family,
    handoff_transport_carrier_detail = authority_handoff_transport_carrier
  }
}
```

### この shape でまだ入れないもの

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
transport carrier authority_handoff_transport_carrier
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_transport -> authority_handoff_transport_carrier`

のような handoff transport carrier detail row である。

payload / receipt row は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-transport-carrier-detail-ready handoff-transport-payload comparison**
に置く。

## open questions

- `handoff_transport_carrier_detail` を canonical field name として十分に読めるか
- transport payload を next reopen に置くべきか
- receipt row を transport payload より後段に残す順序でよいか
