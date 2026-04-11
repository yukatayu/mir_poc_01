# 238 — current L2 theorem line handoff-witness-row-detail-ready minimal-handoff-witness-row-detail threshold

## 目的

`specs/examples/237-current-l2-theorem-line-minimal-witness-aware-handoff-family-ready-handoff-witness-row-detail-comparison.md`
で `witness_actor_ref + handoff_witness_kind` row を current first choice にした判断を前提に、

- handoff witness row detail の minimal field core をどこまでに留めるか
- replay attachment、handoff payload / carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
handoff-witness-row-detail-ready minimal-handoff-witness-row-detail threshold**
であり、

- final replay attachment
- final handoff payload / carrier detail

はまだ固定しない。

## 比較観点

1. handoff witness を source-backed に見せる最小 row になっているか
2. witness-aware handoff family と自然に接続しているか
3. replay / payload / carrier detail を premature に混ぜないか
4. theorem-line retained bridge の row-detail first cut を保てるか

## 比較対象

### 案 1. `witness_aware_handoff_family_ref + witness_actor_ref + handoff_witness_kind` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_handoff_witness_row = {
  witness_aware_handoff_family_ref =
    retained_payload_body_materialization_theorem_export_witness_aware_handoff_family,
  witness_actor_ref = audit_sink,
  handoff_witness_kind = authority_epoch_witness
}
```

#### 利点

- handoff witness を source-backed に見せる最小 row である
- replay / payload / carrier detail を still later に残せる
- family row と row detail の役割分担が明瞭である

#### 欠点

- replay / payload の actual attachment は still prose 依存である

### 案 2. row detail に replay attachment を同時に足す

#### 利点

- audit / replay line に近い shape になる

#### 欠点

- current phase では premature である
- row-detail first cut が重くなる

### 案 3. row detail に payload / carrier detail を同時に足す

#### 利点

- actual operational handoff payload との接続が見えやすい

#### 欠点

- theorem-line retained bridge の current phase では強すぎる
- runtime carrier と proof boundary が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 1. `witness_aware_handoff_family_ref + witness_actor_ref + handoff_witness_kind` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_witness_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_witness_aware_handoff_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_handoff_witness_row = {
    witness_aware_handoff_family_ref =
      retained_payload_body_materialization_theorem_export_witness_aware_handoff_family,
    witness_actor_ref = audit_sink,
    handoff_witness_kind = authority_epoch_witness
  }
}
```

### この shape でまだ入れないもの

- `replay_ref`
- `handoff_payload_ref`
- `handoff_carrier_ref`

これらは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `audit_sink -> authority_epoch_witness`

のような handoff witness row である。

replay attachment や payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-handoff-witness-row-detail-ready replay-attachment-ref comparison**
に置く。

## open questions

- `handoff_witness_kind` の canonical token set をどこまで current phase で固定してよいか
- replay attachment を next reopen に置くべきか
- handoff payload / carrier detail を replay line より後段に残す順序でよいか
