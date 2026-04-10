# 181 — current L2 theorem line retained-archive-payload-ready archive-retention-layout threshold

## 目的

`specs/examples/180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md`
までを前提に、

- retained-archive-payload-ready retained bridge に archive retention layout をどこまで足すか
- actual retained archive payload body family と actual bless-side / update-side row split をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-archive-payload-ready archive-retention-layout threshold** であり、

- actual retained archive payload body family
- actual bless-side row / update-side row split

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-archive-payload-ready retained bridge を起点にする。
- actual retained archive payload body family、actual bless-side row / update-side row split、actual retained payload materialization family は巻き込まない。

## current 前提

current repo では次が成立している。

1. retained archive payload family までは current first choice に入っている。
2. archive retention layout は still 後段に残している。

したがって current 問いは、
**retained-archive-payload-ready retained bridge に `archive_retention_layout_ref` だけを先に足してよいか、それとも actual retained payload body family / bless-side / update-side split とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. retained archive payload family と archive retention layout family の line を narrow に切れるか
2. archive retention layout family を symbolic ref 1 本で留め、actual retained payload body family を still 後段に残せるか
3. actual bless-side / update-side row split を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual retained archive payload body family comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-archive-payload-ready retained bridge を terminal cut にし、archive retention layout も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- layout family と actual retained payload body family の混線をさらに避けられる

#### 欠点

- retained payload family の次段が prose 依存に残りやすい
- theorem-line bridge の retention layout return path が弱い

### 案 2. archive retention layout family ref だけを持つ archive-retention-layout-ready retained bridge にする

#### 読み

retained-archive-payload-ready retained bridge に、
actual retained payload body family や bless-side / update-side split を入れずに
**`archive_retention_layout_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_archive_retention_layout_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_ready_sketch,
  archive_retention_layout_ref
}
```

#### 利点

- retained archive payload family と archive retention layout family の line を narrow に橋渡しできる
- actual retained payload body family を still 後段に残せる
- bless-side / update-side split を premature に混ぜずに済む
- next later reopen を actual retained archive payload body family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- layout family ref と actual payload body family ref を誤読されない説明が要る

### 案 3. archive retention layout と actual retained payload body family をまとめて入れる

#### 利点

- notebook consumer が retained payload line をさらに具体的に追える
- actual retained payload body family pressure に早く近づける

#### 欠点

- archive retention layout、actual retained payload body family、bless-side / update-side row split を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific retention body lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. archive retention layout family ref だけを持つ archive-retention-layout-ready retained bridge にする**
である。

理由は次の通り。

1. retained archive payload family の次段を symbolic ref で narrow に橋渡しできる
2. actual retained payload body family を still 後段に残せる
3. bless-side / update-side row split を still 後段に残せる
4. next later reopen を actual retained archive payload body family comparison へ狭く進めやすい

## minimal archive-retention-layout-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_archive_retention_layout_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_ready_sketch,
  archive_retention_layout_ref
}
```

### `archive_retention_layout_ref`

`archive_retention_layout_ref` は、

- actual retained archive payload family の次段で
- notebook consumer が **archive retention layout family** に戻れる
  symbolic layout-family ref

だけを置く field である。

current task では、

- actual retained archive payload body family
- actual bless-side row / update-side row split
- actual retained payload materialization family

をここに入れない。

## なぜ actual retained payload body family をまだ入れないか

actual retained payload body family を current phase で入れると、

- archive retention layout family
- actual retained payload body family
- bless-side / update-side row split

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず archive retention layout family へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ bless-side / update-side row split をまだしないか

actual bless-side row / update-side row split を current phase で入れると、

- retained archive payload family
- archive retention layout family
- actual retained payload body family

の切り分けが同じ reopen で崩れやすい。

current pressure は layout family ref を 1 本だけ bridge に足すところまでで十分である。

## practical examples

### example A — fallback chain archive-retention-layout-ready retained bridge

```text
proof_notebook_bridge_archive_retention_layout_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_ready_sketch = bridge_sketch:e12.retained_archive_payload_ready,
  archive_retention_layout_ref = archive_retention_layout:e12.latest
}
```

ここで notebook bridge が知るのは
archive retention layout family に戻る ref までであり、
actual payload body family や bless-side / update-side split まではまだ bridge に入れない。

### example B — witnessed draw archive-retention-layout-ready retained bridge

```text
proof_notebook_bridge_archive_retention_layout_ready_sketch = {
  proof_notebook_bridge_retained_archive_payload_ready_sketch = bridge_sketch:sugoroku.draw.retained_archive_payload_ready,
  archive_retention_layout_ref = archive_retention_layout:sugoroku.draw.latest
}
```

draw archive retention layout family へ戻る ref は notebook bridge で追えるが、
actual payload body family や bless-side / update-side split までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で archive retention layout comparison を切り、current first choice は `archive_retention_layout_ref` だけを足す archive-retention-layout-ready retained bridge である
- actual retained payload body family と actual bless-side / update-side row split は still 後段に残す
- next later reopen は actual retained archive payload body family comparison である

### not decided

- actual retained archive payload body family の最小 shape
- actual bless-side row と update-side row をどこで分けるか
- archive retention layout family を actual retained payload materialization family とどこで接続するか
