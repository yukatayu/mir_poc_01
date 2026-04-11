# 229 — current L2 theorem line minimal-authority-transition-stage-sequence-ready stage-local-obligation-family comparison

## 目的

`specs/examples/228-current-l2-theorem-line-authority-transition-stage-sequence-shape-ready-minimal-authority-transition-stage-sequence-threshold.md`
で actual ordered stage sequence row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として stage-local obligation family をどこまで足してよいか
- symbolic family marker に留めるか、actual stage-local obligation row まで足すか
- authority handoff / witness / replay line をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-authority-transition-stage-sequence-ready stage-local-obligation-family comparison**
であり、

- final stage-local obligation row schema
- final authority handoff epoch
- final witness / replay attachment

はまだ固定しない。

## 比較観点

1. actual ordered stage sequence の次段として自然な theorem-side retained bridge になっているか
2. stage-local obligation row を premature に固定しないか
3. authority handoff / witness / replay を still later に残せるか
4. authority transition line の narrow ratchet を保てるか

## 比較対象

### 案 1. stage sequence row で止める

#### 利点

- retained bridge は最も軽い
- stage-local obligation family をまだ固定しない

#### 欠点

- `lock` / `draw` / `commit` / `publish` ごとの local obligation cluster が prose 依存のまま残る

### 案 2. symbolic `authority_transition_stage_local_obligation_family` row を足す

#### 読み

stage sequence row の次段として、

- `stage_local_obligation_family_kind`

だけを持つ symbolic row を足す。

#### 利点

- actual per-stage row に踏み込まず、local obligation cluster を source-backed にできる
- authority handoff / witness / replay をまだ入れなくてよい
- theorem-line retained bridge の family-first ratchet を保てる

#### 欠点

- each stage の local obligation 自体は still row に出ない

### 案 3. actual stage-local obligation row を足す

#### 利点

- `lock` / `draw` / `commit` / `publish` ごとの local obligation を直接見せられる

#### 欠点

- current phase では強すぎる
- witness / replay / handoff line と premature に結び付きやすい

### 案 4. handoff / witness attachment を先に足す

#### 利点

- lifecycle / fairness line に直結しやすい

#### 欠点

- current phase では premature である
- Phase 4 control-plane line と witness line を早く混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic `authority_transition_stage_local_obligation_family` row を足す**
である。

理由は次の通り。

1. actual ordered stage sequence の次に必要なのは actual row detail ではなく local obligation cluster の family marker である
2. symbolic family marker なら actual stage-local obligation row を still later に残せる
3. authority handoff / witness / replay line も still later に保てる

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
```

この line で actual ordered stage sequence の次に欲しいのは、
`[lock, draw, commit, publish]`
に結び付いた local obligation cluster が
`authoritative_serial_commit_stage_obligations`
である、という family-level bridge である。

各 stage の actual obligation row や handoff / witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**stage-local-obligation-family-ready minimal-authority-stage-local-obligation-family threshold**
に置く。

## open questions

- `authoritative_serial_commit_stage_obligations` を family kind canonical label として十分に読めるか
- actual stage-local obligation row を first reopen に置く順序でよいか
- authority handoff epoch を stage-local obligation row より先に reopen すべき pressure があるか
