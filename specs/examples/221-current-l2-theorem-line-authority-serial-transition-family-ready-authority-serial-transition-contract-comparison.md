# 221 — current L2 theorem line authority-serial-transition-family-ready authority-serial-transition-contract comparison

## 目的

`specs/examples/220-current-l2-theorem-line-higher-level-async-control-family-ready-authority-serial-transition-family-threshold.md`
を前提に、

- theorem-line retained bridge の次段として
  `authority_serial_transition_contract` をどこまで actualize してよいか
- family marker に留めるか、minimal contract row を足すか、stronger row まで足すか
- witness / replay / owner-slot detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-serial-transition-family-ready authority-serial-transition-contract comparison**
であり、

- final authority protocol schema
- final replay / witness contract
- final shared-space authority catalog
- public checker / theorem export API

はまだ固定しない。

## scope

- theorem-line retained bridge の next step としてだけ `authority_serial_transition_contract` を比較する。
- authoritative room baseline を参照するが、shared-space final catalog は固定しない。
- `witness_aware_commit_family` と `event_tree_execution_view` は still later candidate に残す。

## 比較観点

1. family marker から minimal contract row へ narrow に進めるか
2. actual authority protocol / owner-slot detail / witness attachment を premature に混ぜないか
3. theorem-line retained bridge を symbolic / docs-only row bundle のまま維持できるか
4. Phase 4 authoritative room baseline と整合するか

## 比較対象

### 案 1. `authority_serial_transition_family` marker で止める

#### 利点

- retained bridge は最も軽い
- actual contract row を早く既成事実化しない

#### 欠点

- next promoted line が prose 依存になりやすい
- theorem-side retained bridge から authority ordering obligation が見えにくい

### 案 2. minimal `authority_serial_transition_contract` row を足す

#### 読み

family marker の次段として、

- `obligation_kind`
- `authority_subject_ref`
- `transition_kind`

だけを持つ minimal contract row を足す。

#### 利点

- family marker より一段 concrete だが、actual protocol row までは入れない
- theorem-line retained bridge から authority ordering obligation を source-backed に見せられる
- witness / replay / owner-slot detail を still later に残せる

#### 欠点

- row が増える分、family marker only より bridge は少し重い

### 案 3. stronger authority row を足す

#### 読み

minimal contract row に加えて、

- `authority_owner_ref`
- `transition_stage_refs`
- witness / replay attachment

まで同時に足す。

#### 利点

- later protocol / fairness compare に直接つながりやすい

#### 欠点

- actual authority protocol / witness / replay を early に押し込みやすい
- theorem-line retained bridge の narrow line を崩しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. minimal `authority_serial_transition_contract` row を足す**
である。

理由は次の通り。

1. family marker only だと next promoted line が still prose 依存になる
2. stronger row は actual authority protocol / witness / replay を早く持ち込みすぎる
3. minimal row なら source-of-truth ordering obligation を見せつつ、later detail を still later に残せる

## practical example

```text
authoritative room:
  authority-ack
  single room authority
  authoritative serial transition

request roll(player = P) {
  transition by room_authority {
    lock
    draw
    commit
    publish
  }
}
```

この line で theorem-line retained bridge に次に欲しいのは、
authority ordering obligation を表す **minimal contract row** である。

`room_authority` の owner-slot detail や `draw` / `publish` の stage ref、
さらに witness / replay attachment までは、まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-serial-transition-contract-ready minimal-authority-serial-contract threshold**
に置く。

## open questions

- minimal row core を `obligation_kind + authority_subject_ref + transition_kind` に留めるか
- `transition_kind` の enum をどこまで narrow に切るか
- owner-slot detail / witness / replay をどの順で later reopen するか
