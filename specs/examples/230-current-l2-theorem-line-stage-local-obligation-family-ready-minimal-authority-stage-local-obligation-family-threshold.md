# 230 — current L2 theorem line stage-local-obligation-family-ready minimal-authority-stage-local-obligation-family threshold

## 目的

`specs/examples/229-current-l2-theorem-line-minimal-authority-transition-stage-sequence-ready-stage-local-obligation-family-comparison.md`
で symbolic `authority_transition_stage_local_obligation_family` row を current first choice にした判断を前提に、

- stage-local obligation family row の minimal field core をどこまでに留めるか
- actual stage-local obligation row、authority handoff epoch、witness / replay attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
stage-local-obligation-family-ready minimal-authority-stage-local-obligation-family threshold**
であり、

- final stage-local obligation row schema
- final authority handoff epoch
- final witness / replay attachment

はまだ固定しない。

## 比較観点

1. stage-local obligation cluster を source-backed に見せる最小 row になっているか
2. actual stage sequence row と自然に接続しているか
3. actual row detail や handoff / witness を premature に混ぜないか
4. theorem-line retained bridge の family-first ratchet を保てるか

## 比較対象

### 案 1. `authority_transition_stage_sequence_ref + stage_local_obligation_family_kind` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_family = {
  authority_transition_stage_sequence_ref =
    retained_payload_body_materialization_theorem_export_authority_transition_stage_sequence,
  stage_local_obligation_family_kind = authoritative_serial_commit_stage_obligations
}
```

#### 利点

- stage-local obligation cluster を source-backed に見せる最小 row である
- actual stage-local obligation row を still later に残せる
- authority handoff / witness / replay を混ぜない

#### 欠点

- each stage の actual local obligation は still prose 依存である

### 案 2. actual stage-local obligation row を同じ row bundle に持つ

#### 利点

- family marker と row detail を同時に読める

#### 欠点

- current phase では強すぎる
- stage-local obligation family と row detail を premature に結合しやすい

### 案 3. family row に handoff / witness attachment を同時に足す

#### 利点

- lifecycle / fairness line に直結しやすい

#### 欠点

- current phase では premature である
- Phase 4 control-plane line と witness line を早く混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_transition_stage_sequence_ref + stage_local_obligation_family_kind` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_stage_local_obligation_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_transition_stage_sequence_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_transition_stage_local_obligation_family = {
    authority_transition_stage_sequence_ref =
      retained_payload_body_materialization_theorem_export_authority_transition_stage_sequence,
    stage_local_obligation_family_kind = authoritative_serial_commit_stage_obligations
  }
}
```

### この shape でまだ入れないもの

- `stage_local_obligation_rows`
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
actual ordered stage sequence
`[lock, draw, commit, publish]`
に結び付いた stage-local obligation family が
`authoritative_serial_commit_stage_obligations`
である、という minimal family row である。

各 stage の actual obligation row や handoff / witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-authority-stage-local-obligation-family-ready stage-local-obligation-row-detail comparison**
に置く。

## open questions

- `authoritative_serial_commit_stage_obligations` を family kind canonical label として十分に読めるか
- actual stage-local obligation row を first reopen に置くべきか
- authority handoff epoch を stage-local obligation row より先に reopen すべき pressure があるか
