# 234 — current L2 theorem line authority-handoff-epoch-ref-ready minimal-authority-handoff-epoch-ref threshold

## 目的

`specs/examples/233-current-l2-theorem-line-minimal-authority-stage-local-obligation-row-detail-ready-authority-handoff-epoch-ref-comparison.md`
で symbolic `authority_handoff_epoch_ref` row を current first choice にした判断を前提に、

- authority handoff epoch ref row の minimal field core をどこまでに留めるか
- witness / replay attachment、handoff payload / carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-handoff-epoch-ref-ready minimal-authority-handoff-epoch-ref threshold**
であり、

- final witness / replay attachment
- final handoff payload / carrier detail

はまだ固定しない。

## 比較観点

1. authority turnover line を source-backed に見せる最小 row になっているか
2. stage-local obligation row detail と自然に接続しているか
3. witness / replay / payload / carrier detail を premature に混ぜないか
4. theorem-line retained bridge の ref-first ratchet を保てるか

## 比較対象

### 案 1. `authority_transition_stage_local_obligation_row_ref + authority_handoff_epoch_ref` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_handoff_epoch_ref = {
  authority_transition_stage_local_obligation_row_ref =
    retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_row,
  authority_handoff_epoch_ref = room_authority_epoch
}
```

#### 利点

- authority turnover line を source-backed に見せる最小 row である
- witness / replay / payload / carrier detail を still later に残せる
- stage-local obligation row detail と handoff ref の役割分担が明瞭である

#### 欠点

- actual handoff payload は still prose 依存である

### 案 2. handoff ref に witness / replay attachment を同時に足す

#### 利点

- fairness / replay line に近い shape になる

#### 欠点

- current phase では premature である
- ref-first cut が重くなる

### 案 3. handoff ref に payload / carrier detail を同時に足す

#### 利点

- actual operational handoff payload との接続が見えやすい

#### 欠点

- theorem-line retained bridge の current phase では強すぎる
- runtime carrier と proof boundary が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_transition_stage_local_obligation_row_ref + authority_handoff_epoch_ref` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_handoff_epoch_ref_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_stage_local_obligation_row_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_handoff_epoch_ref = {
    authority_transition_stage_local_obligation_row_ref =
      retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_row,
    authority_handoff_epoch_ref = room_authority_epoch
  }
}
```

### この shape でまだ入れないもの

- `witness_ref`
- `replay_ref`
- `handoff_payload_ref`
- `handoff_carrier_ref`

これらは still later である。

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
handoff to next_authority
```

current theorem-line retained bridge に今ほしいのは、
current authority turnover line が
`room_authority_epoch`
に結び付く、という minimal handoff ref row である。

witness / replay や actual handoff payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-authority-handoff-epoch-ref-ready witness-aware-handoff-family comparison**
に置く。

## open questions

- `authority_handoff_epoch_ref` を canonical ref surface として十分に読めるか
- witness-aware handoff family を next reopen に置くべきか
- handoff payload / carrier detail を witness line より後段に残す順序でよいか
