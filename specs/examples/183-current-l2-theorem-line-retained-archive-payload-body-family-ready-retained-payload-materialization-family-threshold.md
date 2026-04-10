# 183 — current L2 theorem line retained-archive-payload-body-family-ready retained-payload-materialization-family threshold

## 目的

`specs/examples/182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md`
までを前提に、

- retained-archive-payload-body-family-ready retained bridge に retained payload materialization family をどこまで足すか
- actual bless-side / update-side row split と actual retained payload body materialization detail をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-archive-payload-body-family-ready retained-payload-materialization-family threshold** であり、

- actual bless-side row / update-side row split
- actual retained payload body materialization detail

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-archive-payload-body-family-ready retained bridge を起点にする。
- actual bless-side / update-side row split、actual retained payload body materialization detail、actual retained payload body materialization payload は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual retained archive payload body family までは current first choice に入っている。
2. retained payload materialization family は still 後段に残している。

したがって current 問いは、
**retained-archive-payload-body-family-ready retained bridge に `retained_payload_materialization_family_ref` だけを先に足してよいか、それとも bless-side / update-side row split や actual retained payload body materialization detail とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual retained archive payload body family と retained payload materialization family の line を narrow に切れるか
2. retained payload materialization family を symbolic ref 1 本で留め、bless-side / update-side row split を still 後段に残せるか
3. actual retained payload body materialization detail を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual retained payload body materialization detail comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-archive-payload-body-family-ready retained bridge を terminal cut にし、retained payload materialization family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- materialization family と body materialization detail の混線をさらに避けられる

#### 欠点

- payload body family の次段が prose 依存に残りやすい
- theorem-line bridge の materialization family return path が弱い

### 案 2. retained payload materialization family ref だけを持つ retained-payload-materialization-family-ready retained bridge にする

#### 読み

retained-archive-payload-body-family-ready retained bridge に、
actual bless-side / update-side row split や actual retained payload body materialization detail を入れずに
**`retained_payload_materialization_family_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch,
  retained_payload_materialization_family_ref
}
```

#### 利点

- actual retained archive payload body family と retained payload materialization family の line を narrow に橋渡しできる
- bless-side / update-side row split を still 後段に残せる
- actual retained payload body materialization detail を premature に混ぜずに済む
- next later reopen を actual retained payload body materialization detail comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- materialization family ref と body materialization detail ref を誤読されない説明が要る

### 案 3. materialization family と bless-side / update-side row split をまとめて入れる

#### 利点

- notebook consumer が materialization line をさらに細かく追える
- bless/update 実体 pressure に早く近づける

#### 欠点

- materialization family、bless-side / update-side row split、body materialization detail を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific materialization lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. retained payload materialization family ref だけを持つ retained-payload-materialization-family-ready retained bridge にする**
である。

理由は次の通り。

1. payload body family の次段を symbolic ref で narrow に橋渡しできる
2. bless-side / update-side row split を still 後段に残せる
3. actual retained payload body materialization detail を still 後段に残せる
4. next later reopen を actual retained payload body materialization detail comparison へ狭く進めやすい

## minimal retained-payload-materialization-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch,
  retained_payload_materialization_family_ref
}
```

### `retained_payload_materialization_family_ref`

`retained_payload_materialization_family_ref` は、

- actual retained archive payload body family の次段で
- notebook consumer が **retained payload materialization family** に戻れる
  symbolic materialization-family ref

だけを置く field である。

current task では、

- actual bless-side row / update-side row split
- actual retained payload body materialization detail
- actual retained payload body materialization payload

をここに入れない。

## なぜ bless-side / update-side row split をまだ入れないか

actual bless-side row / update-side row split を current phase で入れると、

- retained payload materialization family
- bless-side / update-side row split
- actual retained payload body materialization detail

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず retained payload materialization family へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ actual retained payload body materialization detail をまだ入れないか

actual retained payload body materialization detail を current phase で入れると、

- actual retained archive payload body family
- retained payload materialization family
- actual retained payload body materialization detail

の切り分けが同じ reopen で崩れやすい。

current pressure は materialization family ref を 1 本だけ bridge に足すところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-materialization-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch = bridge_sketch:e12.retained_archive_payload_body_family_ready,
  retained_payload_materialization_family_ref = retained_payload_materialization_family:e12.latest
}
```

ここで notebook bridge が知るのは
retained payload materialization family に戻る ref までであり、
bless-side / update-side row split や body materialization detail まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-materialization-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_materialization_family_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch = bridge_sketch:sugoroku.draw.retained_archive_payload_body_family_ready,
  retained_payload_materialization_family_ref = retained_payload_materialization_family:sugoroku.draw.latest
}
```

draw retained payload materialization family へ戻る ref は notebook bridge で追えるが、
bless-side / update-side row split や body materialization detail までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で retained payload materialization family comparison を切り、current first choice は `retained_payload_materialization_family_ref` だけを足す retained-payload-materialization-family-ready retained bridge である
- actual bless-side row / update-side row split と actual retained payload body materialization detail は still 後段に残す
- next later reopen は actual retained payload body materialization detail comparison である

### not decided

- actual retained payload body materialization detail の最小 shape
- actual bless-side row と update-side row をどこで分けるか
- actual retained payload body materialization payload をどこで接続するか
