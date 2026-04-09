# 175 — current L2 theorem line archive-body-ready archive-bundle threshold

## 目的

`specs/examples/174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md`
までを前提に、

- archive-body-ready retained bridge に actual archive bundle family をどこまで足すか
- actual archive bundle manifest / member family をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-body-ready archive-bundle threshold** であり、

- actual archive bundle manifest / member family

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-body-ready retained bridge を起点にする。
- archive retention layout、bundle retention layout、bless/update policy、actual archive member body compare は巻き込まない。

## current 前提

current repo では次が成立している。

1. archive body family までは current first choice に入っている。
2. actual archive bundle manifest / member family は still 後段に残している。

したがって current 問いは、
**archive-body-ready retained bridge に `archive_bundle_ref` だけを先に足してよいか、それとも actual archive bundle manifest / member family とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual archive body family と actual archive bundle family の line を narrow に切れるか
2. actual archive bundle family と actual archive bundle manifest / member family を分離できるか
3. actual archive bundle manifest / member family を theorem-line bridge に premature に混ぜないか
4. next later reopen を archive bundle manifest / member family comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-body-ready retained bridge を terminal cut にし、actual archive bundle も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual archive bundle manifest / member family との混線をさらに避けられる

#### 欠点

- actual archive body family と actual archive bundle family の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. actual archive bundle ref だけを持つ archive-bundle-ready retained bridge にする

#### 読み

archive-body-ready retained bridge に、
actual archive bundle manifest / member family を入れずに
**`archive_bundle_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_archive_bundle_ready_sketch = {
  proof_notebook_bridge_archive_body_ready_sketch,
  archive_bundle_ref
}
```

#### 利点

- actual archive body family と actual archive bundle family の line を narrow に橋渡しできる
- actual archive bundle manifest / member family を still 後段に残せる
- next later reopen を archive bundle manifest / member family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- archive bundle family と archive bundle manifest / member family を誤読されない説明が要る

### 案 3. actual archive bundle と actual archive bundle manifest / member family をまとめて入れる

#### 利点

- notebook consumer が actual archive bundle family を一度に追える
- bundle retrieval / member walk / bless policy pressure に近づく

#### 欠点

- actual archive bundle family と actual archive bundle manifest / member family を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. actual archive bundle ref だけを持つ archive-bundle-ready retained bridge にする**
である。

理由は次の通り。

1. actual archive body family と actual archive bundle family の line を narrow に接続できる
2. actual archive bundle manifest / member family を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を archive bundle manifest / member family comparison へ狭く進めやすい

## minimal archive-bundle-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_archive_bundle_ready_sketch = {
  proof_notebook_bridge_archive_body_ready_sketch,
  archive_bundle_ref
}
```

### `archive_bundle_ref`

`archive_bundle_ref` は、

- actual archive body family の次段で
- notebook consumer が **actual archive bundle family** に戻れる
  symbolic archive-bundle ref

だけを置く field である。

current task では、

- actual archive bundle manifest / member family

をここに入れない。

## なぜ actual archive bundle manifest / member family をまだ入れないか

actual archive bundle manifest / member family を current phase で入れると、

- actual archive bundle family
- actual archive bundle manifest / member family
- bundle retention layout / bless policy / archive member body compare

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual archive bundle family を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain archive-bundle-ready retained bridge

```text
proof_notebook_bridge_archive_bundle_ready_sketch = {
  proof_notebook_bridge_archive_body_ready_sketch = bridge_sketch:e12.archive_body_ready,
  archive_bundle_ref = archive_bundle:e12.latest
}
```

ここで notebook bridge が知るのは
archive bundle に戻る ref までであり、
bundle manifest や bundle member family まではまだ bridge に入れない。

### example B — witnessed draw archive-bundle-ready retained bridge

```text
proof_notebook_bridge_archive_bundle_ready_sketch = {
  proof_notebook_bridge_archive_body_ready_sketch = bridge_sketch:sugoroku.draw.archive_body_ready,
  archive_bundle_ref = archive_bundle:sugoroku.draw.latest
}
```

draw archive bundle へ戻る ref は notebook bridge で追えるが、
bundle manifest や member family までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で archive bundle comparison を切り、current first choice は `archive_bundle_ref` のみを足す archive-bundle-ready retained bridge である
- next later reopen は actual archive bundle manifest / member family comparison である
- actual archive bundle manifest / member family は still 後段に残す

### not decided

- actual archive bundle manifest / member family の最小 shape
- archive bundle manifest と archive bundle member family を同じ reopen で扱うかどうか
- archive retention layout / bless/update policy への接続
