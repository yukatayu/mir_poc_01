# 226 — current L2 theorem line authority-transition-stage-family-ready minimal-authority-transition-stage-family threshold

## 目的

`specs/examples/225-current-l2-theorem-line-authority-owner-ref-ready-authority-transition-stage-family-comparison.md`
で symbolic `authority_transition_stage_family` row を current first choice にした判断を前提に、

- stage family row の minimal field core をどこまでに留めるか
- explicit stage list、handoff epoch、witness attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-transition-stage-family-ready minimal-authority-transition-stage-family threshold**
であり、

- final stage sequence schema
- final authority handoff epoch
- final witness / replay attachment

はまだ固定しない。

## 比較観点

1. symbolic family row として最小になっているか
2. explicit stage list を unnecessary に持ち込まないか
3. owner-slot detail と stage family の接点が source-backed になっているか
4. theorem-line retained bridge の symbolic row bundle を保てるか

## 比較対象

### 案 1. `authority_owner_detail_ref + stage_family_kind` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_transition_stage_family = {
  authority_owner_detail_ref =
    retained_payload_body_materialization_theorem_export_authority_owner_ref,
  stage_family_kind = authoritative_serial_commit_sequence
}
```

#### 利点

- owner-slot detail の次段として最小である
- explicit stage list を still later に残せる
- handoff / witness / replay を混ぜない

#### 欠点

- actual `lock -> draw -> commit -> publish` sequence 自体は row に出ない

### 案 2. explicit stage list を同じ row に持つ

#### 利点

- actual ordering を 1 row で読める

#### 欠点

- stage family と explicit sequence を同時に固定しやすい
- current phase では強すぎる

### 案 3. stage family に handoff / witness attachment を同時に足す

#### 利点

- later lifecycle / fairness line に直結しやすい

#### 欠点

- current phase では premature である
- Phase 4 control-plane line と witness line を早く混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_owner_detail_ref + stage_family_kind` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_transition_stage_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_owner_ref_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_transition_stage_family = {
    authority_owner_detail_ref =
      retained_payload_body_materialization_theorem_export_authority_owner_ref,
    stage_family_kind = authoritative_serial_commit_sequence
  }
}
```

### この shape でまだ入れないもの

- explicit `transition_stage_sequence`
- `authority_handoff_epoch_ref`
- `witness_ref`
- `replay_ref`

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

current theorem-line retained bridge に今ほしいのは、
`room_authority_slot` に結び付いた stage family が
`authoritative_serial_commit_sequence`
である、という minimal family row である。

actual stage sequence list や handoff / witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-authority-transition-stage-family-ready authority-transition-stage-sequence-shape comparison**
に置く。

## open questions

- `authoritative_serial_commit_sequence` を family kind canonical label として十分に読めるか
- explicit stage sequence row を first reopen に置くべきか
- authority handoff epoch を stage sequence より先に reopen すべき pressure があるか
