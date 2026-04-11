# 227 — current L2 theorem line minimal-authority-transition-stage-family-ready authority-transition-stage-sequence-shape comparison

## 目的

`specs/examples/226-current-l2-theorem-line-authority-transition-stage-family-ready-minimal-authority-transition-stage-family-threshold.md`
で symbolic stage family row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として actual stage sequence shape をどこまで足してよいか
- ordered stage label sequence に留めるか、per-stage obligation row まで足すか
- handoff / witness / replay attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-authority-transition-stage-family-ready authority-transition-stage-sequence-shape comparison**
であり、

- final per-stage obligation schema
- final authority handoff epoch
- final witness / replay attachment

はまだ固定しない。

## 比較観点

1. symbolic stage family の次段として自然な concrete shape になっているか
2. per-stage obligation を premature に固定しないか
3. handoff / witness / replay を still later に残せるか
4. theorem-line retained bridge の narrow lineを保てるか

## 比較対象

### 案 1. symbolic stage family で止める

#### 利点

- bridge は最も軽い
- actual stage shape をまだ固定しない

#### 欠点

- `authoritative_serial_commit_sequence` の中身が prose 依存のまま残る

### 案 2. ordered stage label sequence を足す

#### 読み

symbolic stage family の次段として、

- ordered `transition_stage_sequence`

だけを持つ row を足す。

#### 利点

- actual ordering を source-backed に見せられる
- per-stage obligation / witness / replay をまだ入れなくてよい
- current bridge の narrow line を保ちやすい

#### 欠点

- per-stage semantics は still 見えない

### 案 3. per-stage obligation row を足す

#### 利点

- `lock` / `draw` / `commit` / `publish` ごとの役割が見える

#### 欠点

- current phase では強すぎる
- witness / replay / handoff line と結び付きやすい

### 案 4. handoff / witness attachment を先に足す

#### 利点

- lifecycle / fairness line に直結しやすい

#### 欠点

- current phase では premature である
- Phase 4 control-plane line と早く混ざりやすい

## current judgment

current L2 で最も自然なのは、
**案 2. ordered stage label sequence を足す**
である。

理由は次の通り。

1. symbolic family の次にほしいのは、actual ordering であって per-stage obligation ではない
2. ordered stage label sequence なら sequence を source-backed にできる
3. per-stage obligation / handoff / witness を still later に残せる

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
```

この line で symbolic stage family の次に欲しいのは、
actual stage sequence が
`[lock, draw, commit, publish]`
である、という narrow row である。

各 stage の local obligation や handoff / witness / replay は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-transition-stage-sequence-shape-ready minimal-authority-transition-stage-sequence threshold**
に置く。

## open questions

- stage label sequence を canonical ordered list として十分に読めるか
- per-stage obligation row を stage sequence の直後に reopen する順序でよいか
- authority handoff epoch を stage sequence より先に reopen すべき pressure があるか
