# 222 — current L2 theorem line authority-serial-transition-contract-ready minimal-authority-serial-contract threshold

## 目的

`specs/examples/221-current-l2-theorem-line-authority-serial-transition-family-ready-authority-serial-transition-contract-comparison.md`
で minimal `authority_serial_transition_contract` row を current first choice とした判断を前提に、

- minimal row の field core をどこまでに留めるか
- owner-slot detail / stage refs / witness attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-serial-transition-contract-ready minimal-authority-serial-contract threshold**
であり、

- final authority slot schema
- final transition stage family
- final witness / replay attachment
- final public checker carrier

はまだ固定しない。

## 比較観点

1. source-of-truth ordering obligation を見せる最小 row になっているか
2. room owner slot detail と contract row を premature に混ぜないか
3. witness / replay / fairness attachment を後段に残せるか
4. theorem-line retained bridge の symbolic row bundle を保てるか

## 比較対象

### 案 1. three-field minimal row

#### shape

```text
retained_payload_body_materialization_theorem_export_authority_serial_transition_contract = {
  authority_serial_transition_family_ref =
    retained_payload_body_materialization_theorem_export_authority_serial_transition_family,
  obligation_kind = authority_serial_transition_contract,
  authority_subject_ref = room_authority,
  transition_kind = serial_commit
}
```

#### 利点

- authority ordering obligation を示す最小 row になる
- owner-slot detail / stage refs / witness を still later に残せる
- theorem-line retained bridge の narrow line を保ちやすい

#### 欠点

- stage-level detail はまだ見えない

### 案 2. owner-slot detail 付き row

#### 利点

- authority placement と contract row を近くで見せられる

#### 欠点

- `authority_subject_ref` と `authority_owner_ref` を早く二重化しやすい
- Phase 4 authority baseline と theorem-line contract row を premature に密結合しやすい

### 案 3. stage / witness attachment 付き row

#### 利点

- later fairness / replay comparison に直結しやすい

#### 欠点

- current phase では強すぎる
- `witness_aware_commit_family` の reopen 順序を壊しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. three-field minimal row**
である。

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_serial_transition_contract_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_authority_serial_transition_family_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_serial_transition_contract = {
    authority_serial_transition_family_ref =
      retained_payload_body_materialization_theorem_export_authority_serial_transition_family,
    obligation_kind = authority_serial_transition_contract,
    authority_subject_ref = room_authority,
    transition_kind = serial_commit
  }
}
```

### この shape でまだ入れないもの

- `authority_owner_ref`
- `transition_stage_refs`
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
「`room_authority` が serial commit obligation を持つ」
という minimal row である。

`draw` / `publish` の stage sequence 自体や witness / replay は、
まだ current task では扱わない。

## next promoted line

next promoted line は、
**minimal-authority-serial-contract-ready authority-serial-row-detail comparison**
に置く。

## current status note

この threshold は、current Phase 5 theorem-line で
`authority_serial_transition_contract` の **minimum floor**
を固定するものである。

owner slot detail、transition stage family、witness / replay attachment は
この次段で比較する。

## open questions

- `transition_kind = serial_commit` を family enum に広げるべきか
- `authority_owner_ref` と `authority_subject_ref` をどこで分けるか
- `witness_aware_commit_family` と row detail line をどこで合流させるか
