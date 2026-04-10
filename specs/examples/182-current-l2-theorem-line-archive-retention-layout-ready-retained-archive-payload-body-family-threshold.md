# 182 — current L2 theorem line archive-retention-layout-ready retained-archive-payload-body-family threshold

## 目的

`specs/examples/181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md`
までを前提に、

- archive-retention-layout-ready retained bridge に actual retained archive payload body family をどこまで足すか
- actual bless-side / update-side row split と actual retained payload materialization family をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-retention-layout-ready retained-archive-payload-body-family threshold** であり、

- actual bless-side row / update-side row split
- actual retained payload materialization family

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-retention-layout-ready retained bridge を起点にする。
- actual bless-side row / update-side row split、actual retained payload materialization family、actual retained payload body materialization detail は巻き込まない。

## current 前提

current repo では次が成立している。

1. archive retention layout family までは current first choice に入っている。
2. actual retained archive payload body family は still 後段に残している。

したがって current 問いは、
**archive-retention-layout-ready retained bridge に `retained_archive_payload_body_family_ref` だけを先に足してよいか、それとも bless-side / update-side row split や actual retained payload materialization family とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. archive retention layout family と actual retained archive payload body family の line を narrow に切れるか
2. actual retained archive payload body family を symbolic ref 1 本で留め、bless-side / update-side row split を still 後段に残せるか
3. actual retained payload materialization family を theorem-line bridge に premature に混ぜないか
4. next later reopen を retained payload materialization family comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-retention-layout-ready retained bridge を terminal cut にし、actual retained archive payload body family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- payload body family と materialization family の混線をさらに避けられる

#### 欠点

- archive retention layout family の次段が prose 依存に残りやすい
- theorem-line bridge の payload body family return path が弱い

### 案 2. actual retained archive payload body family ref だけを持つ retained-archive-payload-body-family-ready retained bridge にする

#### 読み

archive-retention-layout-ready retained bridge に、
actual bless-side / update-side row split や actual retained payload materialization family を入れずに
**`retained_archive_payload_body_family_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch = {
  proof_notebook_bridge_archive_retention_layout_ready_sketch,
  retained_archive_payload_body_family_ref
}
```

#### 利点

- archive retention layout family と actual retained archive payload body family の line を narrow に橋渡しできる
- bless-side / update-side row split を still 後段に残せる
- actual retained payload materialization family を premature に混ぜずに済む
- next later reopen を retained payload materialization family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- payload body family ref と materialization family ref を誤読されない説明が要る

### 案 3. payload body family と bless-side / update-side row split をまとめて入れる

#### 利点

- notebook consumer が payload body line をさらに細かく追える
- bless/update 実体 pressure に早く近づける

#### 欠点

- payload body family、bless-side / update-side row split、materialization family を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific retention body lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual retained archive payload body family ref だけを持つ retained-archive-payload-body-family-ready retained bridge にする**
である。

理由は次の通り。

1. archive retention layout family の次段を symbolic ref で narrow に橋渡しできる
2. bless-side / update-side row split を still 後段に残せる
3. actual retained payload materialization family を still 後段に残せる
4. next later reopen を retained payload materialization family comparison へ狭く進めやすい

## minimal retained-archive-payload-body-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch = {
  proof_notebook_bridge_archive_retention_layout_ready_sketch,
  retained_archive_payload_body_family_ref
}
```

### `retained_archive_payload_body_family_ref`

`retained_archive_payload_body_family_ref` は、

- archive retention layout family の次段で
- notebook consumer が **actual retained archive payload body family** に戻れる
  symbolic body-family ref

だけを置く field である。

current task では、

- actual bless-side row / update-side row split
- actual retained payload materialization family
- actual retained payload body materialization detail

をここに入れない。

## なぜ bless-side / update-side row split をまだ入れないか

actual bless-side row / update-side row split を current phase で入れると、

- actual retained archive payload body family
- bless-side / update-side row split
- actual retained payload materialization family

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual retained archive payload body family へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ actual retained payload materialization family をまだ入れないか

actual retained payload materialization family を current phase で入れると、

- archive retention layout family
- actual retained archive payload body family
- actual retained payload materialization family

の切り分けが同じ reopen で崩れやすい。

current pressure は payload body family ref を 1 本だけ bridge に足すところまでで十分である。

## practical examples

### example A — fallback chain retained-archive-payload-body-family-ready retained bridge

```text
proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch = {
  proof_notebook_bridge_archive_retention_layout_ready_sketch = bridge_sketch:e12.archive_retention_layout_ready,
  retained_archive_payload_body_family_ref = retained_archive_payload_body_family:e12.latest
}
```

ここで notebook bridge が知るのは
actual retained archive payload body family に戻る ref までであり、
bless-side / update-side row split や materialization family まではまだ bridge に入れない。

### example B — witnessed draw retained-archive-payload-body-family-ready retained bridge

```text
proof_notebook_bridge_retained_archive_payload_body_family_ready_sketch = {
  proof_notebook_bridge_archive_retention_layout_ready_sketch = bridge_sketch:sugoroku.draw.archive_retention_layout_ready,
  retained_archive_payload_body_family_ref = retained_archive_payload_body_family:sugoroku.draw.latest
}
```

draw actual retained archive payload body family へ戻る ref は notebook bridge で追えるが、
bless-side / update-side row split や materialization family までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual retained archive payload body family comparison を切り、current first choice は `retained_archive_payload_body_family_ref` だけを足す retained-archive-payload-body-family-ready retained bridge である
- actual bless-side row / update-side row split と retained payload materialization family は still 後段に残す
- next later reopen は retained payload materialization family comparison である

### not decided

- retained payload materialization family の最小 shape
- actual bless-side row と update-side row をどこで分けるか
- actual retained archive payload body family を body materialization detail とどこで接続するか
