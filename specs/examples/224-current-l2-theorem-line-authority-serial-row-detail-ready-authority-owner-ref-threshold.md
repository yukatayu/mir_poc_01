# 224 — current L2 theorem line authority-serial-row-detail-ready authority-owner-ref threshold

## 目的

`specs/examples/223-current-l2-theorem-line-minimal-authority-serial-contract-ready-authority-serial-row-detail-comparison.md`
で owner slot detail を current first choice にした判断を前提に、

- owner slot detail row の minimal field core をどこまでに留めるか
- subject field の重複、stage / handoff detail、witness attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-serial-row-detail-ready authority-owner-ref threshold**
であり、

- final authority handoff schema
- final transition stage family
- final witness / replay / fairness attachment

はまだ固定しない。

## 比較観点

1. minimal contract row の補助 detail として最小になっているか
2. `authority_subject_ref` を unnecessary に二重化しないか
3. owner slot detail と handoff / stage / witness を premature に混ぜないか
4. theorem-line retained bridge の symbolic row bundle を保てるか

## 比較対象

### 案 1. `authority_owner_ref` だけを足す minimal detail row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_owner_ref = {
  authority_serial_transition_contract_ref =
    retained_payload_body_materialization_theorem_export_authority_serial_transition_contract,
  authority_owner_ref = room_authority_slot
}
```

#### 利点

- contract row にある `authority_subject_ref` を再記述せずに済む
- owner slot detail だけを narrow に補える
- stage / witness / replay を still later に残せる

#### 欠点

- owner と subject の関係は contract row 参照を辿って読む必要がある

### 案 2. subject + owner を同じ detail row に並べる

#### 利点

- 1 row だけ見れば split が読める

#### 欠点

- `authority_subject_ref` を contract row と detail row で二重化しやすい
- retained bridge の narrow lineに対して冗長になりやすい

### 案 3. owner slot detail と handoff / stage detail を同時に足す

#### 利点

- later authority-handoff line へつながりやすい

#### 欠点

- current phase では強すぎる
- transition stage family / witness family の reopen 順序を崩しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_owner_ref` だけを足す minimal detail row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_owner_ref_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_serial_transition_contract_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_owner_ref = {
    authority_serial_transition_contract_ref =
      retained_payload_body_materialization_theorem_export_authority_serial_transition_contract,
    authority_owner_ref = room_authority_slot
  }
}
```

### この shape でまだ入れないもの

- repeated `authority_subject_ref`
- `transition_stage_family_ref`
- `authority_handoff_epoch_ref`
- `witness_ref`
- `replay_ref`

これらは still later である。

## practical example

```text
transition by room_authority {
  draw
  commit
  publish
}
```

current theorem-line retained bridge に今ほしいのは、
`room_authority` が参照する room-critical owner slot が
`room_authority_slot` である、という minimal detail である。

`draw` / `commit` / `publish` の stage family、handoff epoch、witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-owner-ref-ready authority-transition-stage-family comparison**
に置く。

## open questions

- `room_authority_slot` を owner slot canonical name として十分に読めるか
- transition stage family を owner slot detail の後に置く順序でよいか
- authority handoff epoch を stage family の前に reopen すべき pressure があるか
