# 250 — current L2 theorem line handoff-transport-payload-ready minimal-handoff-transport-payload threshold

## 目的

`specs/examples/249-current-l2-theorem-line-minimal-handoff-transport-carrier-detail-ready-handoff-transport-payload-comparison.md`
で `handoff_transport_carrier_detail_ref + handoff_transport_payload` row を current first choice にした判断を前提に、

- handoff transport payload row の minimal field core をどこまでに留めるか
- handoff transport receipt row をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-transport-payload-ready minimal-handoff-transport-payload threshold**
であり、

- final handoff transport receipt row

はまだ固定しない。

## 比較観点

1. handoff transport payload を source-backed に見せる最小 row になっているか
2. handoff transport carrier detail と自然に接続しているか
3. transport receipt row を premature に混ぜないか
4. theorem-line retained bridge の detail cut を保てるか

## 比較対象

### 案 1. `handoff_transport_carrier_detail_ref + handoff_transport_payload` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_transport_payload = {
  handoff_transport_carrier_detail_ref =
    retained_payload_body_materialization_theorem_export_handoff_transport_carrier_detail,
  handoff_transport_payload = authority_handoff_transport_payload
}
```

#### 利点

- handoff transport payload を source-backed に見せる最小 row である
- transport receipt row を still later に残せる
- carrier detail と payload の役割分担が明瞭である

#### 欠点

- actual receipt row attachment は still prose 依存である

### 案 2. transport payload row に receipt row を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- payload first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_transport_carrier_detail_ref + handoff_transport_payload` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_carrier_detail_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_transport_payload = {
    handoff_transport_carrier_detail_ref =
      retained_payload_body_materialization_theorem_export_handoff_transport_carrier_detail,
    handoff_transport_payload = authority_handoff_transport_payload
  }
}
```

### この shape でまだ入れないもの

- `handoff_transport_receipt_row`

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
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_transport_carrier -> authority_handoff_transport_payload`

のような handoff transport payload row である。

receipt row は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-transport-payload-ready handoff-transport-receipt comparison**
に置く。

## open questions

- `handoff_transport_payload` を canonical field name として十分に読めるか
- transport receipt row を next reopen に置く順序でよいか
