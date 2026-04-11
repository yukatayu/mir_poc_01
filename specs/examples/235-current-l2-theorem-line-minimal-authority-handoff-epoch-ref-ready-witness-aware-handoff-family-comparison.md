# 235 — current L2 theorem line minimal-authority-handoff-epoch-ref-ready witness-aware-handoff-family comparison

## 目的

`specs/examples/234-current-l2-theorem-line-authority-handoff-epoch-ref-ready-minimal-authority-handoff-epoch-ref-threshold.md`
で symbolic authority handoff epoch ref row を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として witness-aware handoff family をどこまで足してよいか
- symbolic family marker に留めるか、actual witness row detail まで足すか
- replay attachment / payload / carrier detail をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-authority-handoff-epoch-ref-ready witness-aware-handoff-family comparison**
であり、

- final handoff witness row schema
- final replay attachment
- final handoff payload / carrier detail

はまだ固定しない。

## 比較観点

1. authority handoff epoch ref の次段として自然な fairness / witness bridge になっているか
2. actual witness row detail を premature に固定しないか
3. replay / payload / carrier detail を still later に残せるか
4. theorem-line retained bridge の family-first ratchet を保てるか

## 比較対象

### 案 1. authority handoff epoch ref で止める

#### 利点

- bridge は最も軽い
- witness line をまだ固定しない

#### 欠点

- handoff witness cluster が prose 依存のまま残る

### 案 2. symbolic `witness_aware_handoff_family` row を足す

#### 読み

authority handoff epoch ref の次段として、

- `witness_aware_handoff_family_kind`

だけを持つ symbolic family row を足す。

#### 利点

- fairness / witness line を source-backed に接続できる
- actual witness row / replay / payload / carrier detail をまだ入れなくてよい
- theorem-line retained bridge の family-first ratchet を保てる

#### 欠点

- actual witness row 自体は still 見えない

### 案 3. actual handoff witness row を足す

#### 利点

- handoff witness detail を直接見せられる

#### 欠点

- current phase では強すぎる
- replay / payload line と premature に結び付きやすい

### 案 4. replay attachment を先に足す

#### 利点

- replay / audit line に直結しやすい

#### 欠点

- current phase では premature である
- witness line と replay line を早く混ぜやすい

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic `witness_aware_handoff_family` row を足す**
である。

理由は次の通り。

1. authority handoff epoch ref の次に必要なのは actual witness row ではなく witness cluster の family marker である
2. symbolic family marker なら replay / payload / carrier detail を still later に残せる
3. theorem-line retained bridge の family-first ratchet と整合する

## practical example

```text
handoff to next_authority
witness by audit_sink
```

この line で authority handoff epoch ref の次に欲しいのは、
current handoff witness cluster が
`authority_epoch_witnessed_handoff`
である、という family-level bridge である。

actual witness row や replay / payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**witness-aware-handoff-family-ready minimal-witness-aware-handoff-family threshold**
に置く。

## open questions

- `authority_epoch_witnessed_handoff` を family kind canonical label として十分に読めるか
- actual witness row detail を first reopen に置く順序でよいか
- replay attachment を witness row detail より先に reopen すべき pressure があるか
