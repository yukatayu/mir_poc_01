# 241 — current L2 theorem line minimal-replay-attachment-ref-ready handoff-payload-ref comparison

## 目的

`specs/examples/240-current-l2-theorem-line-replay-attachment-ref-ready-minimal-replay-attachment-ref-threshold.md`
で symbolic replay attachment ref を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff payload ref をどこまで足してよいか
- symbolic handoff payload ref に留めるか、handoff carrier detail まで足すか
- symbolic replay line をどこで actual handoff payload semantics に接続するか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-replay-attachment-ref-ready handoff-payload-ref comparison**
であり、

- final handoff carrier detail
- final handoff transport / receipt line

はまだ固定しない。

## 比較観点

1. replay attachment ref の次段として自然な symbolic handoff payload になっているか
2. carrier detail を premature に混ぜないか
3. theorem-side retained bridge の symbolic-ref ratchet を保てるか
4. runtime handoff transport line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. replay attachment ref で止める

#### 利点

- retained bridge は最も軽い
- handoff payload 自体をまだ固定しない

#### 欠点

- `replay attachment authority_handoff_replay` の次にある handoff payload surface が still prose 依存である

### 案 2. `handoff_replay_attachment_ref + handoff_payload_ref` の symbolic row を足す

#### 読み

symbolic replay attachment ref の次段として、

- `handoff_replay_attachment_ref`
- `handoff_payload_ref`

だけを持つ symbolic handoff payload row を足す。

#### 利点

- handoff payload line を source-backed に見せられる
- carrier detail をまだ入れなくてよい
- theorem-line retained bridge の symbolic-ref ratchet と整合する

#### 欠点

- `handoff_payload_ref` を symbolic token として読む必要がある

### 案 3. payload row に carrier detail を同時に足す

#### 利点

- actual transport / receipt line との接続が見えやすい

#### 欠点

- current phase では premature である
- payload line と carrier line を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_replay_attachment_ref + handoff_payload_ref` の symbolic row を足す**
である。

理由は次の通り。

1. symbolic replay attachment ref の次に必要なのは symbolic handoff payload であって carrier detail ではない
2. `handoff_replay_attachment_ref + handoff_payload_ref` なら payload line を source-backed に見せつつ carrier detail を still later に残せる
3. theorem-line retained bridge の symbolic-ref ratchet と整合する

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
```

この line で replay attachment ref の次に欲しいのは、たとえば

- `authority_handoff_replay -> authority_handoff_payload`

のような symbolic handoff payload ref である。

carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-payload-ref-ready minimal-handoff-payload-ref threshold**
に置く。

## open questions

- `authority_handoff_payload` のような payload token を canonical label として十分に読めるか
- carrier detail を handoff payload ref の直後に reopen する順序でよいか
- receipt / transport line を carrier detail より後段に残す順序でよいか
