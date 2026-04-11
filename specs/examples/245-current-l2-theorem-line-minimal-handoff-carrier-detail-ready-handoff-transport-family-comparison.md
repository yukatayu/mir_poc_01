# 245 — current L2 theorem line minimal-handoff-carrier-detail-ready handoff-transport-family comparison

## 目的

`specs/examples/244-current-l2-theorem-line-handoff-carrier-detail-ready-minimal-handoff-carrier-detail-threshold.md`
で minimal handoff carrier detail row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff transport family をどこまで足してよいか
- symbolic handoff transport family に留めるか、transport carrier detail / payload / receipt row まで足すか
- handoff carrier detail line の次に何を canonical transport marker として置くか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-carrier-detail-ready handoff-transport-family comparison**
であり、

- final handoff transport carrier detail
- final handoff transport payload
- final handoff transport receipt row

はまだ固定しない。

## 比較観点

1. handoff carrier detail の次段として自然な symbolic transport family になっているか
2. transport carrier / payload / receipt row を premature に混ぜないか
3. theorem-side retained bridge の symbolic-to-detail ratchet を保てるか
4. runtime handoff transport line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff carrier detail で止める

#### 利点

- retained bridge は最も軽い
- transport family 自体をまだ固定しない

#### 欠点

- `carrier authority_handoff_carrier` の次にある transport family surface が still prose 依存である

### 案 2. `handoff_carrier_detail_ref + handoff_transport_family` の row を足す

#### 読み

minimal handoff carrier detail row の次段として、

- `handoff_carrier_detail_ref`
- `next_transport_family`

だけを持つ symbolic handoff transport family row を足す。

#### 利点

- transport line を source-backed に見せられる
- carrier detail と transport family の接点を narrow に固定できる
- transport carrier / payload / receipt row を still later に残せる

#### 欠点

- `authority_handoff_transport` のような family token を canonical transport marker として読む必要がある

### 案 3. transport family row に carrier detail / payload / receipt row を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- family row と actual transport row を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_carrier_detail_ref + handoff_transport_family` の row を足す**
である。

理由は次の通り。

1. handoff carrier detail の次に必要なのは symbolic transport family であって actual transport row ではない
2. `handoff_carrier_detail_ref + next_transport_family` なら transport line を source-backed に見せつつ transport carrier / payload / receipt row を still later に残せる
3. theorem-line retained bridge の symbolic-to-detail ratchet と整合する

## practical example

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
carrier authority_handoff_carrier
transport family authority_handoff_transport
```

この line で handoff carrier detail の次に欲しいのは、たとえば

- `authority_handoff_carrier -> authority_handoff_transport`

のような symbolic transport family row である。

carrier detail / payload / receipt row は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-transport-family-ready minimal-handoff-transport-family threshold**
に置く。

## open questions

- `authority_handoff_transport` のような family token を canonical transport marker として十分に読めるか
- transport carrier detail を next reopen に置く順序でよいか
- receipt row を transport payload より後段に残す順序でよいか
