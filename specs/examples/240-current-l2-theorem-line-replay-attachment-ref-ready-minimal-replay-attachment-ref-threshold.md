# 240 — current L2 theorem line replay-attachment-ref-ready minimal-replay-attachment-ref threshold

## 目的

`specs/examples/239-current-l2-theorem-line-minimal-handoff-witness-row-detail-ready-replay-attachment-ref-comparison.md`
で `handoff_witness_row_ref + replay_attachment_ref` row を current first choice にした判断を前提に、

- replay attachment ref row の minimal field core をどこまでに留めるか
- handoff payload / carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
replay-attachment-ref-ready minimal-replay-attachment-ref threshold**
であり、

- final handoff payload row
- final handoff carrier detail

はまだ固定しない。

## 比較観点

1. replay attachment を source-backed に見せる最小 row になっているか
2. handoff witness row と自然に接続しているか
3. payload / carrier detail を premature に混ぜないか
4. theorem-line retained bridge の symbolic-ref cut を保てるか

## 比較対象

### 案 1. `handoff_witness_row_ref + replay_attachment_ref` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_replay_attachment_ref = {
  handoff_witness_row_ref =
    retained_payload_body_materialization_theorem_export_handoff_witness_row,
  replay_attachment_ref = authority_handoff_replay
}
```

#### 利点

- replay attachment を source-backed に見せる最小 row である
- payload / carrier detail を still later に残せる
- handoff witness row と replay ref の役割分担が明瞭である

#### 欠点

- payload / carrier detail の actual attachment は still prose 依存である

### 案 2. replay row に payload ref を同時に足す

#### 利点

- actual handoff payload との接続が見えやすい

#### 欠点

- current phase では premature である
- symbolic-ref first cut が重くなる

### 案 3. replay row に carrier detail を同時に足す

#### 利点

- runtime transport line に近い shape になる

#### 欠点

- theorem-line retained bridge の current phase では強すぎる
- runtime carrier と proof boundary が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_witness_row_ref + replay_attachment_ref` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_replay_attachment_ref_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_witness_row_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_replay_attachment_ref = {
    handoff_witness_row_ref =
      retained_payload_body_materialization_theorem_export_handoff_witness_row,
    replay_attachment_ref = authority_handoff_replay
  }
}
```

### この shape でまだ入れないもの

- `handoff_payload_ref`
- `handoff_carrier_ref`

これらは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `audit_sink -> authority_epoch_witness`
- `authority_epoch_witness -> authority_handoff_replay`

のような symbolic replay attachment ref である。

payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-replay-attachment-ref-ready handoff-payload-ref comparison**
に置く。

## open questions

- `replay_attachment_ref` を symbolic token field として十分に読めるか
- payload ref を next reopen に置くべきか
- handoff carrier detail を payload line より後段に残す順序でよいか
