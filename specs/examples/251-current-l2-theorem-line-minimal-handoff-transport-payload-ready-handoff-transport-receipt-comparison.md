# 251 — current L2 theorem line minimal-handoff-transport-payload-ready handoff-transport-receipt comparison

## 目的

`specs/examples/250-current-l2-theorem-line-handoff-transport-payload-ready-minimal-handoff-transport-payload-threshold.md`
で minimal handoff transport payload row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff transport receipt row をどこまで足してよいか
- symbolic handoff transport receipt row に留めるか、transport channel body まで足すか
- handoff transport payload line の次に何を canonical transport receipt marker として置くか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-transport-payload-ready handoff-transport-receipt comparison**
であり、

- final handoff transport channel body

はまだ固定しない。

## 比較観点

1. handoff transport payload の次段として自然な symbolic transport receipt row になっているか
2. transport channel body を premature に混ぜないか
3. theorem-side retained bridge の symbolic-to-detail ratchet を保てるか
4. runtime handoff transport receipt line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff transport payload で止める

#### 利点

- retained bridge は最も軽い
- transport receipt row 自体をまだ固定しない

#### 欠点

- `transport payload authority_handoff_transport_payload` の次にある transport receipt surface が still prose 依存である

### 案 2. `handoff_transport_payload_ref + handoff_transport_receipt_row` の row を足す

#### 読み

minimal handoff transport payload row の次段として、

- `handoff_transport_payload_ref`
- `handoff_transport_receipt_row`

だけを持つ symbolic handoff transport receipt row を足す。

#### 利点

- transport receipt line を source-backed に見せられる
- payload と receipt の接点を narrow に固定できる
- transport channel body を still later に残せる

#### 欠点

- `authority_handoff_transport_receipt` のような receipt token を canonical transport receipt marker として読む必要がある

### 案 3. transport receipt row に channel body を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- receipt row と channel body を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_transport_payload_ref + handoff_transport_receipt_row` の row を足す**
である。

理由は次の通り。

1. handoff transport payload の次に必要なのは symbolic transport receipt row であって channel body ではない
2. `handoff_transport_payload_ref + handoff_transport_receipt_row` なら transport receipt line を source-backed に見せつつ channel body を still later に残せる
3. theorem-line retained bridge の symbolic-to-detail ratchet と整合する

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
carrier authority_handoff_carrier
transport family authority_handoff_transport
transport carrier authority_handoff_transport_carrier
transport payload authority_handoff_transport_payload
transport receipt authority_handoff_transport_receipt
```

この line で handoff transport payload の次に欲しいのは、たとえば

- `authority_handoff_transport_payload -> authority_handoff_transport_receipt`

のような symbolic transport receipt row である。

channel body は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-transport-receipt-ready minimal-handoff-transport-receipt threshold**
に置く。

## open questions

- `authority_handoff_transport_receipt` のような receipt token を canonical transport receipt marker として十分に読めるか
- transport channel body を next reopen に置く順序でよいか
