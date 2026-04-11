# 244 — current L2 theorem line handoff-carrier-detail-ready minimal-handoff-carrier-detail threshold

## 目的

`specs/examples/243-current-l2-theorem-line-minimal-handoff-payload-ref-ready-handoff-carrier-detail-comparison.md`
で `handoff_payload_ref + handoff_carrier_detail` row を current first choice にした判断を前提に、

- handoff carrier detail row の minimal field core をどこまでに留めるか
- handoff transport / receipt line をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-carrier-detail-ready minimal-handoff-carrier-detail threshold**
であり、

- final handoff transport / receipt family
- final handoff transport / receipt row

はまだ固定しない。

## 比較観点

1. handoff carrier を source-backed に見せる最小 row になっているか
2. handoff payload ref と自然に接続しているか
3. transport / receipt line を premature に混ぜないか
4. theorem-line retained bridge の detail cut を保てるか

## 比較対象

### 案 1. `handoff_payload_ref + handoff_carrier_detail` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_carrier_detail = {
  handoff_payload_ref =
    retained_payload_body_materialization_theorem_export_handoff_payload_ref,
  handoff_carrier_detail = authority_handoff_carrier
}
```

#### 利点

- handoff carrier を source-backed に見せる最小 row である
- transport / receipt line を still later に残せる
- payload ref と carrier detail の役割分担が明瞭である

#### 欠点

- transport / receipt line の actual attachment は still prose 依存である

### 案 2. carrier row に transport / receipt line を同時に足す

#### 利点

- actual runtime handoff line との接続が見えやすい

#### 欠点

- current phase では premature である
- detail first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_payload_ref + handoff_carrier_detail` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_payload_ref_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_carrier_detail = {
    handoff_payload_ref =
      retained_payload_body_materialization_theorem_export_handoff_payload_ref,
    handoff_carrier_detail = authority_handoff_carrier
  }
}
```

### この shape でまだ入れないもの

- `handoff_transport_family`
- `handoff_receipt_row`

これらは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
carrier authority_handoff_carrier
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_payload -> authority_handoff_carrier`

のような handoff carrier detail row である。

transport / receipt line は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-carrier-detail-ready handoff-transport-family comparison**
に置く。

## open questions

- `handoff_carrier_detail` を canonical detail token として十分に読めるか
- transport family を next reopen に置くべきか
- receipt row を transport family より後段に残す順序でよいか
