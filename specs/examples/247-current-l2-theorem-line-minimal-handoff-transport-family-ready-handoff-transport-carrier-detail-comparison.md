# 247 — current L2 theorem line minimal-handoff-transport-family-ready handoff-transport-carrier-detail comparison

## 目的

`specs/examples/246-current-l2-theorem-line-handoff-transport-family-ready-minimal-handoff-transport-family-threshold.md`
で minimal handoff transport family row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff transport carrier detail をどこまで足してよいか
- symbolic handoff transport carrier detail に留めるか、transport payload / receipt row まで足すか
- handoff transport family line の次に何を canonical transport carrier marker として置くか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-transport-family-ready handoff-transport-carrier-detail comparison**
であり、

- final handoff transport payload
- final handoff transport receipt row

はまだ固定しない。

## 比較観点

1. handoff transport family の次段として自然な symbolic transport carrier detail になっているか
2. transport payload / receipt row を premature に混ぜないか
3. theorem-side retained bridge の symbolic-to-detail ratchet を保てるか
4. runtime handoff transport line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff transport family で止める

#### 利点

- retained bridge は最も軽い
- transport carrier detail 自体をまだ固定しない

#### 欠点

- `transport family authority_handoff_transport` の次にある carrier detail surface が still prose 依存である

### 案 2. `handoff_transport_family_ref + handoff_transport_carrier_detail` の row を足す

#### 読み

minimal handoff transport family row の次段として、

- `handoff_transport_family_ref`
- `handoff_transport_carrier_detail`

だけを持つ symbolic handoff transport carrier detail row を足す。

#### 利点

- transport carrier line を source-backed に見せられる
- transport family と carrier detail の接点を narrow に固定できる
- transport payload / receipt row を still later に残せる

#### 欠点

- `authority_handoff_transport_carrier` のような detail token を canonical transport carrier marker として読む必要がある

### 案 3. transport carrier row に payload / receipt row を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- carrier detail row と actual payload / receipt row を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_transport_family_ref + handoff_transport_carrier_detail` の row を足す**
である。

理由は次の通り。

1. handoff transport family の次に必要なのは symbolic transport carrier detail であって payload / receipt row ではない
2. `handoff_transport_family_ref + handoff_transport_carrier_detail` なら transport carrier line を source-backed に見せつつ payload / receipt row を still later に残せる
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
```

この line で handoff transport family の次に欲しいのは、たとえば

- `authority_handoff_transport -> authority_handoff_transport_carrier`

のような symbolic transport carrier detail row である。

payload / receipt row は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-transport-carrier-detail-ready minimal-handoff-transport-carrier-detail threshold**
に置く。

## open questions

- `authority_handoff_transport_carrier` のような detail token を canonical transport carrier marker として十分に読めるか
- transport payload を next reopen に置く順序でよいか
- receipt row を transport payload より後段に残す順序でよいか
