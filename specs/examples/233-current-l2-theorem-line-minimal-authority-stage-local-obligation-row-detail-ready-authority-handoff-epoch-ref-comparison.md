# 233 — current L2 theorem line minimal-authority-stage-local-obligation-row-detail-ready authority-handoff-epoch-ref comparison

## 目的

`specs/examples/232-current-l2-theorem-line-stage-local-obligation-row-detail-ready-minimal-authority-stage-local-obligation-row-detail-threshold.md`
で actual stage-local obligation row detail を current first choice にした判断を前提に、

- theorem-line retained bridge の次段として authority handoff epoch ref をどこまで足してよいか
- symbolic handoff epoch ref に留めるか、witness / replay attachment まで足すか
- stage-local row detail と authority turnover line をどこで接続するか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-authority-stage-local-obligation-row-detail-ready authority-handoff-epoch-ref comparison**
であり、

- final witness / replay attachment
- final handoff payload / carrier detail

はまだ固定しない。

## 比較観点

1. stage-local obligation row detail の次段として自然な authority turnover bridge になっているか
2. witness / replay を premature に混ぜないか
3. payload / carrier detail を premature に混ぜないか
4. theorem-line retained bridge の symbolic-ref first cut を保てるか

## 比較対象

### 案 1. stage-local obligation row detail で止める

#### 利点

- retained bridge は最も軽い
- authority turnover line をまだ固定しない

#### 欠点

- authority turnover / handoff lifecycle が prose 依存のまま残る

### 案 2. symbolic `authority_handoff_epoch_ref` row を足す

#### 読み

stage-local obligation row detail の次段として、

- `authority_handoff_epoch_ref`

だけを持つ symbolic ref row を足す。

#### 利点

- authority turnover line を source-backed に接続できる
- witness / replay / payload / carrier detail をまだ入れなくてよい
- theorem-line retained bridge の ref-first ratchet を保てる

#### 欠点

- handoff epoch の actual payload は still 見えない

### 案 3. handoff ref に witness / replay attachment を同時に足す

#### 利点

- fairness / replay line に直結しやすい

#### 欠点

- current phase では premature である
- witness line と authority turnover line を早く混ぜやすい

### 案 4. handoff ref に payload / carrier detail を同時に足す

#### 利点

- actual operational turnover payload との接続が見えやすい

#### 欠点

- theorem-line retained bridge の current phase では強すぎる
- runtime carrier と proof boundary が早く混ざる

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic `authority_handoff_epoch_ref` row を足す**
である。

理由は次の通り。

1. stage-local row detail の次に必要なのは actual payload ではなく authority turnover line の symbolic ref である
2. symbolic ref row なら witness / replay / payload / carrier detail を still later に残せる
3. theorem-line retained bridge の ref-first ratchet と整合する

## practical example

```text
transition by room_authority {
  lock
  draw
  commit
  publish
}
handoff to next_authority
```

この line で stage-local obligation row detail の次に欲しいのは、
current authority turnover line が
`room_authority_epoch`
に結び付いている、という symbolic ref row である。

witness / replay や actual handoff payload / carrier detail は、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-handoff-epoch-ref-ready minimal-authority-handoff-epoch-ref threshold**
に置く。

## open questions

- `room_authority_epoch` のような epoch ref token を canonical ref surface として十分に読めるか
- witness-aware handoff family を handoff ref の直後に reopen する順序でよいか
- payload / carrier detail を witness line より後段に残す順序でよいか
