# 228 — current L2 theorem line authority-transition-stage-sequence-shape-ready minimal-authority-transition-stage-sequence threshold

## 目的

`specs/examples/227-current-l2-theorem-line-minimal-authority-transition-stage-family-ready-authority-transition-stage-sequence-shape-comparison.md`
で ordered stage label sequence を current first choice にした判断を前提に、

- stage sequence row の minimal field core をどこまでに留めるか
- per-stage obligation、handoff epoch、witness attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-transition-stage-sequence-shape-ready minimal-authority-transition-stage-sequence threshold**
であり、

- final per-stage obligation schema
- final authority handoff epoch
- final witness / replay attachment

はまだ固定しない。

## 比較観点

1. actual ordering を見せる最小 row になっているか
2. stage family row と自然に接続しているか
3. per-stage obligation や handoff / witness を premature に混ぜないか
4. theorem-line retained bridge の symbolic / narrow lineを保てるか

## 比較対象

### 案 1. `authority_transition_stage_family_ref + transition_stage_sequence` だけを持つ minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_transition_stage_sequence = {
  authority_transition_stage_family_ref =
    retained_payload_body_materialization_theorem_export_authority_transition_stage_family,
  transition_stage_sequence = [lock, draw, commit, publish]
}
```

#### 利点

- actual ordering を source-backed に見せる最小 row になる
- per-stage obligation / handoff / witness を still later に残せる
- stage family row と sequence row の役割分担が明瞭である

#### 欠点

- each stage の meaning は still prose 依存である

### 案 2. stage sequence と per-stage obligation を同じ row に持つ

#### 利点

- actual ordering と local obligation を 1 row で読める

#### 欠点

- current phase では強すぎる
- stage sequence row と obligation line を premature に結合しやすい

### 案 3. stage sequence に handoff / witness attachment を同時に足す

#### 利点

- lifecycle / fairness line に直結しやすい

#### 欠点

- current phase では premature である
- Phase 4 control-plane line と witness line を早く混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 1. `authority_transition_stage_family_ref + transition_stage_sequence` だけを持つ minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_transition_stage_sequence_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_transition_stage_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_transition_stage_sequence = {
    authority_transition_stage_family_ref =
      retained_payload_body_materialization_theorem_export_authority_transition_stage_family,
    transition_stage_sequence = [lock, draw, commit, publish]
  }
}
```

### この shape でまだ入れないもの

- `per_stage_obligation_rows`
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
actual stage sequence が
`[lock, draw, commit, publish]`
である、という minimal row である。

各 stage の finer obligation や handoff / witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**minimal-authority-transition-stage-sequence-ready stage-local-obligation-family comparison**
に置く。

## open questions

- `transition_stage_sequence` を stage label enum list として十分に読めるか
- per-stage obligation family を first reopen に置くべきか
- authority handoff epoch を stage-local obligation より先に reopen すべき pressure があるか
