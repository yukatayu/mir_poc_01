# 252 — current L2 theorem line handoff-transport-receipt-ready minimal-handoff-transport-receipt threshold

## 目的

`specs/examples/251-current-l2-theorem-line-minimal-handoff-transport-payload-ready-handoff-transport-receipt-comparison.md`
で `handoff_transport_payload_ref + handoff_transport_receipt_row` row を current first choice にした判断を前提に、

- handoff transport receipt row の minimal field core をどこまでに留めるか
- handoff transport channel body をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-transport-receipt-ready minimal-handoff-transport-receipt threshold**
であり、

- final handoff transport channel body

はまだ固定しない。

## 比較観点

1. handoff transport receipt row を source-backed に見せる最小 row になっているか
2. handoff transport payload と自然に接続しているか
3. transport channel body を premature に混ぜないか
4. theorem-line retained bridge の detail cut を保てるか

## 比較対象

### 案 1. `handoff_transport_payload_ref + handoff_transport_receipt_row` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row = {
  handoff_transport_payload_ref =
    retained_payload_body_materialization_theorem_export_handoff_transport_payload,
  handoff_transport_receipt_row = authority_handoff_transport_receipt
}
```

#### 利点

- handoff transport receipt row を source-backed に見せる最小 row である
- transport channel body を still later に残せる
- payload と receipt の役割分担が明瞭である

#### 欠点

- actual channel body attachment は still prose 依存である

### 案 2. transport receipt row に channel body を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- receipt first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_transport_payload_ref + handoff_transport_receipt_row` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_receipt_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_payload_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row = {
    handoff_transport_payload_ref =
      retained_payload_body_materialization_theorem_export_handoff_transport_payload,
    handoff_transport_receipt_row = authority_handoff_transport_receipt
  }
}
```

### この shape でまだ入れないもの

- `handoff_transport_channel_body`

これは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
carrier authority_handoff_carrier
transport family authority_handoff_transport
transport carrier authority_handoff_transport_carrier
transport payload authority_handoff_transport_payload
transport receipt authority_handoff_transport_receipt
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_transport_payload -> authority_handoff_transport_receipt`

のような handoff transport receipt row である。

channel body は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison**
に置く。

## open questions

- `handoff_transport_receipt_row` を canonical field name として十分に読めるか
- transport channel body を next reopen に置く順序でよいか
