# 231 — current L2 theorem line minimal-authority-stage-local-obligation-family-ready stage-local-obligation-row-detail comparison

## 目的

`specs/examples/230-current-l2-theorem-line-stage-local-obligation-family-ready-minimal-authority-stage-local-obligation-family-threshold.md`
で symbolic stage-local obligation family row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として actual stage-local obligation row detail をどこまで足してよいか
- `stage_label + stage_local_obligation_kind` row に留めるか、handoff / witness attachment まで足すか
- stage-local obligation row bundle をどこまで actual stage semantics に寄せるか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-authority-stage-local-obligation-family-ready stage-local-obligation-row-detail comparison**
であり、

- final authority handoff epoch
- final witness / replay attachment
- final stage-local payload / carrier detail

はまだ固定しない。

## 比較観点

1. symbolic family の次段として自然な actual row detail になっているか
2. authority handoff / witness / replay を premature に混ぜないか
3. stage-local obligation line を low-level memory-order 語彙へ早く近づけないか
4. theorem-line retained bridge の narrow ratchet を保てるか

## 比較対象

### 案 1. stage-local obligation family で止める

#### 利点

- bridge は最も軽い
- actual row detail をまだ固定しない

#### 欠点

- `lock` / `draw` / `commit` / `publish` ごとの local obligation が still prose 依存である

### 案 2. `stage_label + stage_local_obligation_kind` row を足す

#### 読み

family row の次段として、

- `stage_label`
- `stage_local_obligation_kind`

だけを持つ actual row detail を足す。

#### 利点

- per-stage local obligation を source-backed に見せられる
- authority handoff / witness / replay をまだ入れなくてよい
- theorem-line retained bridge の row-detail first cut として自然である

#### 欠点

- stage-local kind label を canonical token として読む必要がある

### 案 3. row detail に handoff / witness attachment を同時に足す

#### 利点

- lifecycle / fairness line に直結しやすい

#### 欠点

- current phase では premature である
- Phase 4 control-plane line と witness line を早く混ぜやすい

### 案 4. row detail に payload / carrier detail を同時に足す

#### 利点

- actual operational payload との接続が見えやすい

#### 欠点

- theorem-line retained bridge の narrow ratchet を壊しやすい
- proof boundary と runtime carrier が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 2. `stage_label + stage_local_obligation_kind` row を足す**
である。

理由は次の通り。

1. symbolic family の次に必要なのは actual row detail であって handoff / witness attachment ではない
2. `stage_label + stage_local_obligation_kind` なら stage-local semantics を narrow に source-backed にできる
3. payload / carrier detail や handoff / witness は still later に保てる

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
```

この line で family row の次に欲しいのは、たとえば

- `lock -> authority_slot_lock`
- `draw -> draw_result_capture`
- `commit -> committed_state_write`
- `publish -> visibility_publish`

のような per-stage local obligation row である。

handoff / witness / replay や payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**stage-local-obligation-row-detail-ready minimal-authority-stage-local-obligation-row-detail threshold**
に置く。

## open questions

- `authority_slot_lock` / `draw_result_capture` / `committed_state_write` / `visibility_publish` のような stage-local obligation kind label を canonical token として十分に読めるか
- authority handoff epoch を stage-local obligation row detail より先に reopen すべき pressure があるか
- payload / carrier detail を handoff / witness より後段に残す順序でよいか
