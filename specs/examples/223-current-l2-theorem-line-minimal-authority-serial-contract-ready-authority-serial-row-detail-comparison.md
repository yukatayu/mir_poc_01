# 223 — current L2 theorem line minimal-authority-serial-contract-ready authority-serial-row-detail comparison

## 目的

`specs/examples/222-current-l2-theorem-line-authority-serial-transition-contract-ready-minimal-authority-serial-contract-threshold.md`
で three-field minimal contract row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として authority-serial row detail をどこまで足してよいか
- owner slot detail を先に出すか、transition stage family を先に出すか
- witness / replay attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-authority-serial-contract-ready authority-serial-row-detail comparison**
であり、

- final authority slot schema
- final transition stage schema
- final witness / replay carrier
- final public checker / theorem export API

はまだ固定しない。

## 比較観点

1. minimal contract row の次段として最も narrow な detail になっているか
2. Phase 4 authoritative room baseline と premature に密結合しないか
3. stage / witness / replay detail を still later に残せるか
4. theorem-line retained bridge の symbolic row bundle を保てるか

## 比較対象

### 案 1. minimal contract row で止める

#### 利点

- retained bridge は最も軽い
- row detail を急いで既成事実化しない

#### 欠点

- `authority_subject_ref` と room owner slot の関係が prose 依存のまま残る
- next promoted line が still 抽象的になりやすい

### 案 2. owner slot detail を next row detail として足す

#### 読み

minimal contract row の次段として、

- `authority_owner_ref`

だけを足す narrow detail row を置く。

#### 利点

- `authority_subject_ref` と room owner slot の関係を source-backed にできる
- stage / witness / replay をまだ入れなくてよい
- Phase 4 の `single room authority` baseline と接続しやすい

#### 欠点

- minimal row only より retained bridge は一段重くなる

### 案 3. transition stage family を先に足す

#### 利点

- `lock -> draw -> commit -> publish` の serial ordering へ近づきやすい

#### 欠点

- room owner slot detailが曖昧なまま stage family へ進むことになる
- witness / replay family と結びつきやすく、current phase では強すぎる

### 案 4. witness / replay attachment を先に足す

#### 利点

- fairness / replay pressure に直結しやすい

#### 欠点

- current Phase 5 theorem-line では強すぎる
- `witness_aware_commit_family` の later line を premature に再導入しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. owner slot detail を next row detail として足す**
である。

理由は次の通り。

1. minimal contract row の次段でまず切りたいのは、subject と room owner slot の接続である
2. transition stage family を先に入れると、stage / witness / replay line が近づきすぎる
3. owner slot detail なら Phase 4 baseline と接続しつつ、still later を広く残せる

## practical example

```text
room sugoroku_room {
  authority_model   = single_room_authority
  consistency_mode  = authoritative_serial_transition

  resource board_state owner = room_authority_slot
  resource roll_lock   owner = room_authority_slot
}

transition by room_authority {
  lock
  draw
  commit
  publish
}
```

この line で minimal contract row の次に欲しいのは、
`room_authority` が参照する owner slot が `room_authority_slot` である、
という narrow detail である。

`lock` / `draw` / `publish` の stage family や witness / replay attachment は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-serial-row-detail-ready authority-owner-ref threshold**
に置く。

## open questions

- `authority_owner_ref` だけを足せば十分か
- `authority_subject_ref` を detail row 側で重ねて書くべきか
- transition stage family と owner slot detail をどの順で later reopen するか
