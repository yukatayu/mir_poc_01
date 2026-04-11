# 242 — current L2 theorem line handoff-payload-ref-ready minimal-handoff-payload-ref threshold

## 目的

`specs/examples/241-current-l2-theorem-line-minimal-replay-attachment-ref-ready-handoff-payload-ref-comparison.md`
で `handoff_replay_attachment_ref + handoff_payload_ref` row を current first choice にした判断を前提に、

- handoff payload ref row の minimal field core をどこまでに留めるか
- handoff carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-payload-ref-ready minimal-handoff-payload-ref threshold**
であり、

- final handoff carrier detail
- final handoff transport / receipt line

はまだ固定しない。

## 比較観点

1. handoff payload を source-backed に見せる最小 row になっているか
2. replay attachment ref と自然に接続しているか
3. carrier detail を premature に混ぜないか
4. theorem-line retained bridge の symbolic-ref cut を保てるか

## 比較対象

### 案 1. `handoff_replay_attachment_ref + handoff_payload_ref` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_payload_ref = {
  handoff_replay_attachment_ref =
    retained_payload_body_materialization_theorem_export_handoff_replay_attachment_ref,
  handoff_payload_ref = authority_handoff_payload
}
```

#### 利点

- handoff payload を source-backed に見せる最小 row である
- carrier detail を still later に残せる
- replay attachment ref と payload ref の役割分担が明瞭である

#### 欠点

- carrier detail の actual attachment は still prose 依存である

### 案 2. payload row に carrier detail を同時に足す

#### 利点

- actual transport / receipt line との接続が見えやすい

#### 欠点

- current phase では premature である
- symbolic-ref first cut が重くなる

## current judgment

current L2 で最も自然なのは、
**案 1. `handoff_replay_attachment_ref + handoff_payload_ref` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_payload_ref_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_handoff_replay_attachment_ref_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_payload_ref = {
    handoff_replay_attachment_ref =
      retained_payload_body_materialization_theorem_export_handoff_replay_attachment_ref,
    handoff_payload_ref = authority_handoff_payload
  }
}
```

### この shape でまだ入れないもの

- `handoff_carrier_detail`

これは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `authority_handoff_replay -> authority_handoff_payload`

のような symbolic handoff payload ref である。

carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-payload-ref-ready handoff-carrier-detail comparison**
に置く。

## open questions

- `handoff_payload_ref` を symbolic token field として十分に読めるか
- carrier detail を next reopen に置くべきか
- transport / receipt line を carrier detail より後段に残す順序でよいか
