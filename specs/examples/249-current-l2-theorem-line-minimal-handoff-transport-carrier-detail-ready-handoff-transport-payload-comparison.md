# 249 — current L2 theorem line minimal-handoff-transport-carrier-detail-ready handoff-transport-payload comparison

## 目的

`specs/examples/248-current-l2-theorem-line-handoff-transport-carrier-detail-ready-minimal-handoff-transport-carrier-detail-threshold.md`
で minimal handoff transport carrier detail row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として handoff transport payload をどこまで足してよいか
- symbolic handoff transport payload に留めるか、transport receipt row まで足すか
- handoff transport carrier detail line の次に何を canonical transport payload marker として置くか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-transport-carrier-detail-ready handoff-transport-payload comparison**
であり、

- final handoff transport receipt row

はまだ固定しない。

## 比較観点

1. handoff transport carrier detail の次段として自然な symbolic transport payload になっているか
2. transport receipt row を premature に混ぜないか
3. theorem-side retained bridge の symbolic-to-detail ratchet を保てるか
4. runtime handoff transport payload line を早く proof boundary に混ぜないか

## 比較対象

### 案 1. handoff transport carrier detail で止める

#### 利点

- retained bridge は最も軽い
- transport payload 自体をまだ固定しない

#### 欠点

- `transport carrier authority_handoff_transport_carrier` の次にある transport payload surface が still prose 依存である

### 案 2. `handoff_transport_carrier_detail_ref + handoff_transport_payload` の row を足す

#### 読み

minimal handoff transport carrier detail row の次段として、

- `handoff_transport_carrier_detail_ref`
- `handoff_transport_payload`

だけを持つ symbolic handoff transport payload row を足す。

#### 利点

- transport payload line を source-backed に見せられる
- carrier detail と payload の接点を narrow に固定できる
- transport receipt row を still later に残せる

#### 欠点

- `authority_handoff_transport_payload` のような payload token を canonical transport payload marker として読む必要がある

### 案 3. transport payload row に receipt row を同時に足す

#### 利点

- actual runtime handoff transport line との接続が見えやすい

#### 欠点

- current phase では premature である
- payload row と actual receipt row を早く結合しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `handoff_transport_carrier_detail_ref + handoff_transport_payload` の row を足す**
である。

理由は次の通り。

1. handoff transport carrier detail の次に必要なのは symbolic transport payload であって receipt row ではない
2. `handoff_transport_carrier_detail_ref + handoff_transport_payload` なら transport payload line を source-backed に見せつつ receipt row を still later に残せる
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
```

この line で handoff transport carrier detail の次に欲しいのは、たとえば

- `authority_handoff_transport_carrier -> authority_handoff_transport_payload`

のような symbolic transport payload row である。

receipt row は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-transport-payload-ready minimal-handoff-transport-payload threshold**
に置く。

## open questions

- `authority_handoff_transport_payload` のような payload token を canonical transport payload marker として十分に読めるか
- transport receipt row を next reopen に置く順序でよいか
