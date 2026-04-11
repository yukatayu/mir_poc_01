# 239 — current L2 theorem line minimal-handoff-witness-row-detail-ready replay-attachment-ref comparison

## 目的

`specs/examples/238-current-l2-theorem-line-handoff-witness-row-detail-ready-minimal-handoff-witness-row-detail-threshold.md`
で actual handoff witness row detail を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として replay attachment ref をどこまで足してよいか
- symbolic replay attachment ref に留めるか、handoff payload / carrier detail まで足すか
- actual handoff witness row と replay line をどこで接続するか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-witness-row-detail-ready replay-attachment-ref comparison**
であり、

- final handoff payload row
- final handoff carrier detail

はまだ固定しない。

## 比較観点

1. handoff witness row の次段として自然な symbolic replay attachment になっているか
2. payload / carrier detail を premature に混ぜないか
3. theorem-side retained bridge の symbolic-ref ratchet を保てるか
4. runtime handoff / transport line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff witness row detail で止める

#### 利点

- retained bridge は最も軽い
- replay attachment 自体をまだ固定しない

#### 欠点

- `witness by audit_sink` の次にある replay attachment surface が still prose 依存である

### 案 2. `handoff_witness_row_ref + replay_attachment_ref` の symbolic row を足す

#### 読み

actual handoff witness row の次段として、

- `handoff_witness_row_ref`
- `replay_attachment_ref`

だけを持つ symbolic replay attachment row を足す。

#### 利点

- replay attachment line を source-backed に見せられる
- payload / carrier detail をまだ入れなくてよい
- theorem-line retained bridge の symbolic-ref ratchet と整合する

#### 欠点

- `replay_attachment_ref` を symbolic token として読む必要がある

### 案 3. replay row に payload ref を同時に足す

#### 利点

- actual handoff payload との接続が見えやすい

#### 欠点

- current phase では premature である
- replay line と payload line を早く結合しやすい

### 案 4. replay row に carrier detail を同時に足す

#### 利点

- actual transport / receipt line に近い shape になる

#### 欠点

- runtime carrier と proof boundary が早く混ざる
- theorem-line retained bridge の narrow ratchet を壊しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_witness_row_ref + replay_attachment_ref` の symbolic row を足す**
である。

理由は次の通り。

1. actual handoff witness row の次に必要なのは replay attachment の symbolic reference であって payload / carrier detail ではない
2. `handoff_witness_row_ref + replay_attachment_ref` なら replay line を source-backed に見せつつ payload / carrier detail を still later に残せる
3. theorem-line retained bridge の symbolic-ref ratchet と整合する

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
```

この line で handoff witness row の次に欲しいのは、たとえば

- `audit_sink -> authority_epoch_witness`
- `authority_epoch_witness -> authority_handoff_replay`

のような symbolic replay attachment ref である。

handoff payload や carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**replay-attachment-ref-ready minimal-replay-attachment-ref threshold**
に置く。

## open questions

- `authority_handoff_replay` のような replay attachment token を canonical label として十分に読めるか
- payload ref を replay attachment ref の直後に reopen する順序でよいか
- carrier detail を payload line より後段に残す順序でよいか
