# 225 — current L2 theorem line authority-owner-ref-ready authority-transition-stage-family comparison

## 目的

`specs/examples/224-current-l2-theorem-line-authority-serial-row-detail-ready-authority-owner-ref-threshold.md`
で owner-slot detail row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として transition stage family をどこまで足してよいか
- symbolic stage family marker に留めるか、explicit stage sequence row まで足すか
- handoff / witness / replay attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
authority-owner-ref-ready authority-transition-stage-family comparison**
であり、

- final stage sequence schema
- final authority handoff epoch
- final witness / replay attachment

はまだ固定しない。

## 比較観点

1. owner-slot detail の次段として自然な ordering detail になっているか
2. explicit stage list を premature に固定しないか
3. handoff / witness / replay を still later に残せるか
4. theorem-line retained bridge の symbolic row bundle を保てるか

## 比較対象

### 案 1. owner-slot detail で止める

#### 利点

- bridge は最も軽い
- stage family をまだ固定しない

#### 欠点

- `serial_commit` の内部 ordering が prose 依存のまま残る
- next promoted line が still 抽象的になりやすい

### 案 2. symbolic `authority_transition_stage_family` row を足す

#### 読み

owner-slot detail の次段として、

- `stage_family_kind`

だけを持つ symbolic row を足す。

#### 利点

- explicit stage sequence row を入れずに ordering cluster を source-backed にできる
- handoff / witness / replay をまだ入れなくてよい
- theorem-line retained bridge の symbolic style を保てる

#### 欠点

- actual stage list 自体は still 見えない

### 案 3. explicit stage sequence row を足す

#### 利点

- `lock -> draw -> commit -> publish` を row として見せられる

#### 欠点

- stage list を premature に既成事実化しやすい
- `witness_aware_commit_family` や replay line と近づきすぎる

### 案 4. handoff epoch / witness attachment を先に足す

#### 利点

- room lifecycle / fairness line へ直接つながりやすい

#### 欠点

- current phase では強すぎる
- Phase 4 control-plane threshold と premature に混ざりやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic `authority_transition_stage_family` row を足す**
である。

理由は次の通り。

1. owner-slot detail の次に必要なのは、actual stage list ではなく ordering cluster の symbolic surface である
2. explicit stage sequence row は current phase では強すぎる
3. symbolic family marker なら handoff / witness / replay line を still later に残せる

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
```

この line で owner-slot detail の次に欲しいのは、
`room_authority_slot` が持つ serial ordering cluster が
`authoritative_serial_commit_sequence`
である、という family-level bridge である。

actual stage list 自体や handoff / witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-transition-stage-family-ready minimal-authority-transition-stage-family threshold**
に置く。

## open questions

- `authoritative_serial_commit_sequence` を stage family canonical label として十分に読めるか
- explicit stage sequence row を reopen する前に authority handoff epoch pressure が入る可能性があるか
- witness / replay family と stage family をどこで合流させるか
