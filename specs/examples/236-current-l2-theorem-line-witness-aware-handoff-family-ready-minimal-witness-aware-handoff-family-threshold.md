# 236 — current L2 theorem line witness-aware-handoff-family-ready minimal-witness-aware-handoff-family threshold

## 目的

`specs/examples/235-current-l2-theorem-line-minimal-authority-handoff-epoch-ref-ready-witness-aware-handoff-family-comparison.md`
で symbolic `witness_aware_handoff_family` row を current first choice にした判断を前提に、

- witness-aware handoff family row の minimal field core をどこまでに留めるか
- actual witness row、replay attachment、handoff payload / carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
witness-aware-handoff-family-ready minimal-witness-aware-handoff-family threshold**
であり、

- final handoff witness row schema
- final replay attachment
- final handoff payload / carrier detail

はまだ固定しない。

## 比較観点

1. handoff witness cluster を source-backed に見せる最小 row になっているか
2. authority handoff epoch ref と自然に接続しているか
3. actual witness row / replay / payload / carrier detail を premature に混ぜないか
4. theorem-line retained bridge の family-first ratchet を保てるか

## 比較対象

### 案 1. `authority_handoff_epoch_detail_ref + witness_aware_handoff_family_kind` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_witness_aware_handoff_family = {
  authority_handoff_epoch_detail_ref =
    retained_payload_body_materialization_theorem_export_authority_handoff_epoch_ref,
  witness_aware_handoff_family_kind = authority_epoch_witnessed_handoff
}
```

#### 利点

- handoff witness cluster を source-backed に見せる最小 row である
- actual witness row / replay / payload / carrier detail を still later に残せる
- authority handoff ref と witness family の役割分担が明瞭である

#### 欠点

- actual witness row 自体は still prose 依存である

### 案 2. actual witness row を同じ row bundle に持つ

#### 利点

- family marker と witness row detail を同時に読める

#### 欠点

- current phase では強すぎる
- witness family と row detail を premature に結合しやすい

### 案 3. family row に replay attachment を同時に足す

#### 利点

- audit / replay line に直結しやすい

#### 欠点

- current phase では premature である
- handoff witness line と replay line を早く混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_handoff_epoch_detail_ref + witness_aware_handoff_family_kind` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_witness_aware_handoff_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_handoff_epoch_ref_ready_sketch,
  retained_payload_body_materialization_theorem_export_witness_aware_handoff_family = {
    authority_handoff_epoch_detail_ref =
      retained_payload_body_materialization_theorem_export_authority_handoff_epoch_ref,
    witness_aware_handoff_family_kind = authority_epoch_witnessed_handoff
  }
}
```

### この shape でまだ入れないもの

- `handoff_witness_row`
- `replay_ref`
- `handoff_payload_ref`
- `handoff_carrier_ref`

これらは still later である。

## practical example

```text
handoff to next_authority
witness by audit_sink
```

current theorem-line retained bridge に今ほしいのは、
current handoff witness cluster が
`authority_epoch_witnessed_handoff`
である、という minimal family row である。

actual witness row や replay / payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-witness-aware-handoff-family-ready handoff-witness-row-detail comparison**
に置く。

## open questions

- `authority_epoch_witnessed_handoff` を family kind canonical label として十分に読めるか
- actual witness row detail を next reopen に置くべきか
- replay attachment を witness row detail より先に reopen すべき pressure があるか
