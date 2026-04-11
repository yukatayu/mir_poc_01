# 232 — current L2 theorem line stage-local-obligation-row-detail-ready minimal-authority-stage-local-obligation-row-detail threshold

## 目的

`specs/examples/231-current-l2-theorem-line-minimal-authority-stage-local-obligation-family-ready-stage-local-obligation-row-detail-comparison.md`
で `stage_label + stage_local_obligation_kind` row を current first choice にした判断を前提に、

- stage-local obligation row detail の minimal field core をどこまでに留めるか
- authority handoff epoch、witness / replay attachment、payload / carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
stage-local-obligation-row-detail-ready minimal-authority-stage-local-obligation-row-detail threshold**
であり、

- final authority handoff epoch
- final witness / replay attachment
- final stage-local payload / carrier detail

はまだ固定しない。

## 比較観点

1. stage-local obligation を source-backed に見せる最小 row になっているか
2. family row と自然に接続しているか
3. handoff / witness / payload / carrier detail を premature に混ぜないか
4. theorem-line retained bridge の row-detail first cut を保てるか

## 比較対象

### 案 1. `authority_transition_stage_local_obligation_family_ref + stage_label + stage_local_obligation_kind` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_row = {
  authority_transition_stage_local_obligation_family_ref =
    retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_family,
  stage_label = lock,
  stage_local_obligation_kind = authority_slot_lock
}
```

#### 利点

- stage-local obligation を source-backed に見せる最小 row である
- handoff / witness / payload / carrier detail を still later に残せる
- family row と row detail の役割分担が明瞭である

#### 欠点

- actual payload / carrier detail は still prose 依存である

### 案 2. row detail に handoff / witness attachment を同時に足す

#### 利点

- lifecycle / fairness line に近い shape になる

#### 欠点

- current phase では premature である
- row detail first cut が重くなる

### 案 3. row detail に payload / carrier detail を同時に足す

#### 利点

- actual operational payload との接続が見えやすい

#### 欠点

- theorem-line retained bridge の current phase では強すぎる
- runtime carrier と proof boundary が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_transition_stage_local_obligation_family_ref + stage_label + stage_local_obligation_kind` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_stage_local_obligation_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_stage_local_obligation_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_row = {
    authority_transition_stage_local_obligation_family_ref =
      retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_family,
    stage_label = lock,
    stage_local_obligation_kind = authority_slot_lock
  }
}
```

### この shape でまだ入れないもの

- `authority_handoff_epoch_ref`
- `witness_ref`
- `replay_ref`
- `stage_local_payload_ref`
- `stage_local_carrier_ref`

これらは still later である。

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
```

current theorem-line retained bridge に今ほしいのは、たとえば

- `lock -> authority_slot_lock`
- `draw -> draw_result_capture`
- `commit -> committed_state_write`
- `publish -> visibility_publish`

のような stage-local obligation row である。

handoff / witness / replay や payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-authority-stage-local-obligation-row-detail-ready authority-handoff-epoch-ref comparison**
に置く。

## open questions

- `stage_local_obligation_kind` の canonical token set をどこまで current phase で固定してよいか
- authority handoff epoch を next reopen に置くべきか
- payload / carrier detail を witness line より後段に残す順序でよいか
