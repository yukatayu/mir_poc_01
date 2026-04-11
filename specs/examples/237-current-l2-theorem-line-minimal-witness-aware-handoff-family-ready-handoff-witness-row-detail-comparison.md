# 237 — current L2 theorem line minimal-witness-aware-handoff-family-ready handoff-witness-row-detail comparison

## 目的

`specs/examples/236-current-l2-theorem-line-witness-aware-handoff-family-ready-minimal-witness-aware-handoff-family-threshold.md`
で symbolic `witness_aware_handoff_family` row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として actual handoff witness row detail をどこまで足してよいか
- `witness_actor_ref + handoff_witness_kind` row に留めるか、replay attachment / payload / carrier detail まで足すか
- witness family line をどこで actual witness semantics に接続するか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-witness-aware-handoff-family-ready handoff-witness-row-detail comparison**
であり、

- final replay attachment
- final handoff payload / carrier detail

はまだ固定しない。

## 比較観点

1. symbolic witness family の次段として自然な actual row detail になっているか
2. replay attachment / payload / carrier detail を premature に混ぜないか
3. handoff witness line を low-level transport detail へ早く近づけないか
4. theorem-line retained bridge の row-detail first cut を保てるか

## 比較対象

### 案 1. witness-aware handoff family で止める

#### 利点

- bridge は最も軽い
- actual witness row detail をまだ固定しない

#### 欠点

- `witness by audit_sink` の actual witness surface が still prose 依存である

### 案 2. `witness_actor_ref + handoff_witness_kind` row を足す

#### 読み

family row の次段として、

- `witness_actor_ref`
- `handoff_witness_kind`

だけを持つ actual row detail を足す。

#### 利点

- actual handoff witness surface を source-backed に見せられる
- replay attachment / payload / carrier detail をまだ入れなくてよい
- theorem-line retained bridge の row-detail first cut として自然である

#### 欠点

- witness kind label を canonical token として読む必要がある

### 案 3. row detail に replay attachment を同時に足す

#### 利点

- audit / replay line に直結しやすい

#### 欠点

- current phase では premature である
- witness row detail と replay line を早く混ぜやすい

### 案 4. row detail に payload / carrier detail を同時に足す

#### 利点

- actual operational handoff payload との接続が見えやすい

#### 欠点

- theorem-line retained bridge の narrow ratchet を壊しやすい
- proof boundary と runtime carrier が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 2. `witness_actor_ref + handoff_witness_kind` row を足す**
である。

理由は次の通り。

1. symbolic family の次に必要なのは actual witness row detail であって replay attachment ではない
2. `witness_actor_ref + handoff_witness_kind` なら handoff witness semantics を narrow に source-backed にできる
3. replay / payload / carrier detail は still later に保てる

## practical example

```text
handoff to next_authority
witness by audit_sink
```

この line で family row の次に欲しいのは、たとえば

- `audit_sink -> authority_epoch_witness`

のような actual handoff witness row である。

replay attachment や payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**handoff-witness-row-detail-ready minimal-handoff-witness-row-detail threshold**
に置く。

## open questions

- `authority_epoch_witness` のような handoff witness kind label を canonical token として十分に読めるか
- replay attachment を handoff witness row detail の直後に reopen する順序でよいか
- payload / carrier detail を replay line より後段に残す順序でよいか
