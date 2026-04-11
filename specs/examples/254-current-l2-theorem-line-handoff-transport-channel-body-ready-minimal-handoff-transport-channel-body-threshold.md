# 254 — current L2 theorem line handoff-transport-channel-body-ready minimal-handoff-transport-channel-body threshold

## 目的

`specs/examples/253-current-l2-theorem-line-minimal-handoff-transport-receipt-ready-handoff-transport-channel-body-comparison.md`
で `handoff_transport_receipt_ref + handoff_transport_channel_body` row を current first choice にした判断を前提に、

- handoff transport channel body row の minimal field core をどこまでに留めるか
- low-level memory-order family をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-transport-channel-body-ready minimal-handoff-transport-channel-body threshold**
であり、

- final low-level memory-order family

はまだ固定しない。

## 比較観点

1. handoff transport channel body を source-backed に見せる最小 row になっているか
2. handoff transport receipt row と自然に接続しているか
3. low-level memory-order family を premature に混ぜないか
4. theorem-line retained bridge の detail cut を保てるか

## 比較対象

### 案 1. `handoff_transport_receipt_ref + handoff_transport_channel_body` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_transport_channel_body = {
  handoff_transport_receipt_ref =
    retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row,
  handoff_transport_channel_body = authority_handoff_transport_channel_body
}
```

#### 利点

- handoff transport channel body を source-backed に見せる最小 row である
- low-level memory-order family を still later に残せる
- receipt と channel body の役割分担が明瞭である

#### 欠点

- actual low-level ordering attachment は still prose 依存である

### 案 2. transport channel body row に low-level memory-order family を同時に足す

#### 利点

- actual runtime handoff transport line と external verifier vocabulary との接続が見えやすい

#### 欠点

- current phase では premature である
- channel-body first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_transport_receipt_ref + handoff_transport_channel_body` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_channel_body_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_receipt_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_transport_channel_body = {
    handoff_transport_receipt_ref =
      retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row,
    handoff_transport_channel_body = authority_handoff_transport_channel_body
  }
}
```

### この shape でまだ入れないもの

- `low_level_memory_order_family`

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
transport channel body authority_handoff_transport_channel_body
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_transport_receipt -> authority_handoff_transport_channel_body`

のような handoff transport channel body row である。

low-level memory-order family は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-transport-channel-body-ready low-level-memory-order-family comparison**
に置く。

## open questions

- `handoff_transport_channel_body` を canonical field name として十分に読めるか
- low-level memory-order family を next reopen に置く順序でよいか
