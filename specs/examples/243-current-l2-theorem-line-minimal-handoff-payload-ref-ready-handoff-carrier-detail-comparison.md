# 243 — current L2 theorem line minimal-handoff-payload-ref-ready handoff-carrier-detail comparison

## 目的

`specs/examples/242-current-l2-theorem-line-handoff-payload-ref-ready-minimal-handoff-payload-ref-threshold.md`
で symbolic handoff payload ref を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff carrier detail をどこまで足してよいか
- symbolic handoff carrier detail に留めるか、transport / receipt line まで足すか
- symbolic payload line をどこで actual handoff carrier semantics に接続するか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-payload-ref-ready handoff-carrier-detail comparison**
であり、

- final handoff transport / receipt family
- final handoff transport / receipt row

はまだ固定しない。

## 比較観点

1. handoff payload ref の次段として自然な symbolic handoff carrier detail になっているか
2. transport / receipt line を premature に混ぜないか
3. theorem-side retained bridge の symbolic-to-detail ratchet を保てるか
4. runtime transport line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff payload ref で止める

#### 利点

- retained bridge は最も軽い
- handoff carrier detail 自体をまだ固定しない

#### 欠点

- `payload authority_handoff_payload` の次にある carrier detail surface が still prose 依存である

### 案 2. `handoff_payload_ref + handoff_carrier_detail` の row を足す

#### 読み

symbolic handoff payload ref の次段として、

- `handoff_payload_ref`
- `handoff_carrier_detail`

だけを持つ handoff carrier detail row を足す。

#### 利点

- handoff carrier line を source-backed に見せられる
- transport / receipt line をまだ入れなくてよい
- theorem-line retained bridge の symbolic-to-detail ratchet と整合する

#### 欠点

- `handoff_carrier_detail` を canonical detail token として読む必要がある

### 案 3. carrier row に transport / receipt line を同時に足す

#### 利点

- actual runtime handoff line との接続が見えやすい

#### 欠点

- current phase では premature である
- carrier line と transport / receipt line を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_payload_ref + handoff_carrier_detail` の row を足す**
である。

理由は次の通り。

1. symbolic handoff payload ref の次に必要なのは handoff carrier detail であって transport / receipt line ではない
2. `handoff_payload_ref + handoff_carrier_detail` なら carrier line を source-backed に見せつつ transport / receipt line を still later に残せる
3. theorem-line retained bridge の symbolic-to-detail ratchet と整合する

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
carrier authority_handoff_carrier
```

この line で handoff payload ref の次に欲しいのは、たとえば

- `authority_handoff_payload -> authority_handoff_carrier`

のような handoff carrier detail row である。

transport / receipt line は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-carrier-detail-ready minimal-handoff-carrier-detail threshold**
に置く。

## open questions

- `authority_handoff_carrier` のような carrier detail token を canonical label として十分に読めるか
- transport / receipt family を handoff carrier detail の直後に reopen する順序でよいか
- runtime receipt line を transport family より後段に残す順序でよいか
