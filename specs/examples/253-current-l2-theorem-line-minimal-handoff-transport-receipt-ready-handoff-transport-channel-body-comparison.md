# 253 — current L2 theorem line minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison

## 目的

`specs/examples/252-current-l2-theorem-line-handoff-transport-receipt-ready-minimal-handoff-transport-receipt-threshold.md`
で minimal handoff transport receipt row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff transport channel body をどこまで足してよいか
- symbolic handoff transport channel body に留めるか、low-level memory-order family まで足すか
- handoff transport receipt row の次に何を canonical transport channel body marker として置くか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison**
であり、

- final low-level memory-order family

はまだ固定しない。

## 比較観点

1. handoff transport receipt row の次段として自然な symbolic transport channel body になっているか
2. low-level memory-order family を premature に混ぜないか
3. theorem-side retained bridge の symbolic-to-detail ratchet を保てるか
4. runtime handoff transport channel body line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff transport receipt row で止める

#### 利点

- retained bridge は最も軽い
- transport channel body 自体をまだ固定しない

#### 欠点

- `transport receipt authority_handoff_transport_receipt` の次にある channel body surface が still prose 依存である

### 案 2. `handoff_transport_receipt_ref + handoff_transport_channel_body` の row を足す

#### 読み

minimal handoff transport receipt row の次段として、

- `handoff_transport_receipt_ref`
- `handoff_transport_channel_body`

だけを持つ symbolic handoff transport channel body row を足す。

#### 利点

- transport channel body line を source-backed に見せられる
- receipt と channel body の接点を narrow に固定できる
- low-level memory-order family を still later に残せる

#### 欠点

- `authority_handoff_transport_channel_body` のような channel-body token を canonical transport channel body marker として読む必要がある

### 案 3. transport channel body row に low-level memory-order family を同時に足す

#### 利点

- actual runtime handoff transport line と external verifier vocabulary との接続が見えやすい

#### 欠点

- current phase では premature である
- channel body row と low-level memory-order family を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_transport_receipt_ref + handoff_transport_channel_body` の row を足す**
である。

理由は次の通り。

1. handoff transport receipt row の次に必要なのは symbolic transport channel body であって low-level memory-order family ではない
2. `handoff_transport_receipt_ref + handoff_transport_channel_body` なら transport channel body line を source-backed に見せつつ low-level memory-order family を still later に残せる
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
transport channel body authority_handoff_transport_channel_body
```

この line で handoff transport receipt row の次に欲しいのは、たとえば

- `authority_handoff_transport_receipt -> authority_handoff_transport_channel_body`

のような symbolic transport channel body row である。

low-level memory-order family は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-transport-channel-body-ready minimal-handoff-transport-channel-body threshold**
に置く。

## open questions

- `authority_handoff_transport_channel_body` のような channel-body token を canonical transport channel body marker として十分に読めるか
- low-level memory-order family を next reopen に置く順序でよいか
