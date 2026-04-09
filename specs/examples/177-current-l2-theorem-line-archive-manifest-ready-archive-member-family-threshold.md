# 177 — current L2 theorem line archive-manifest-ready archive-member-family threshold

## 目的

`specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`
までを前提に、

- archive-manifest-ready retained bridge に actual archive bundle member family をどこまで足すか
- actual archive member body compare をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-manifest-ready archive-member-family threshold** であり、

- actual archive member body compare

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-manifest-ready retained bridge を起点にする。
- archive member body compare、bless / update policy、retained archive payload、archive retention layout は巻き込まない。

## current 前提

current repo では次が成立している。

1. archive bundle manifest family までは current first choice に入っている。
2. actual archive bundle member family は still 後段に残している。

したがって current 問いは、
**archive-manifest-ready retained bridge に `archive_bundle_member_family_ref` だけを先に足してよいか、それとも actual archive member body compare とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual archive bundle manifest family と actual archive bundle member family の line を narrow に切れるか
2. actual archive bundle member family と actual archive member body compare を分離できるか
3. actual archive member body compare を theorem-line bridge に premature に混ぜないか
4. next later reopen を archive member body compare comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-manifest-ready retained bridge を terminal cut にし、actual archive bundle member family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- archive member body compare との混線をさらに避けられる

#### 欠点

- actual archive bundle manifest family と actual archive bundle member family の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. actual archive bundle member-family ref だけを持つ archive-member-family-ready retained bridge にする

#### 読み

archive-manifest-ready retained bridge に、
actual archive member body compare を入れずに
**`archive_bundle_member_family_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_archive_member_family_ready_sketch = {
  proof_notebook_bridge_archive_manifest_ready_sketch,
  archive_bundle_member_family_ref
}
```

#### 利点

- actual archive bundle manifest family と actual archive bundle member family の line を narrow に橋渡しできる
- actual archive member body compare を still 後段に残せる
- next later reopen を archive member body compare comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- archive bundle member family と archive member body compare を誤読されない説明が要る

### 案 3. actual archive bundle member family と archive member body compare をまとめて入れる

#### 利点

- notebook consumer が actual archive member walk を一度に追える
- archive member body compare / bless-update pressure に近づく

#### 欠点

- actual archive bundle member family と archive member body compare を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. actual archive bundle member-family ref だけを持つ archive-member-family-ready retained bridge にする**
である。

理由は次の通り。

1. actual archive bundle manifest family と actual archive bundle member family の line を narrow に接続できる
2. actual archive member body compare を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を archive member body compare comparison へ狭く進めやすい

## minimal archive-member-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_archive_member_family_ready_sketch = {
  proof_notebook_bridge_archive_manifest_ready_sketch,
  archive_bundle_member_family_ref
}
```

### `archive_bundle_member_family_ref`

`archive_bundle_member_family_ref` は、

- actual archive bundle manifest family の次段で
- notebook consumer が **actual archive bundle member family** に戻れる
  symbolic archive-member-family ref

だけを置く field である。

current task では、

- actual archive member body compare

をここに入れない。

## なぜ actual archive member body compare をまだ入れないか

actual archive member body compare を current phase で入れると、

- actual archive bundle member family
- actual archive member body compare
- bless / update policy
- retained archive payload

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual archive bundle member family を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain archive-member-family-ready retained bridge

```text
proof_notebook_bridge_archive_member_family_ready_sketch = {
  proof_notebook_bridge_archive_manifest_ready_sketch = bridge_sketch:e12.archive_manifest_ready,
  archive_bundle_member_family_ref = archive_member_family:e12.latest
}
```

ここで notebook bridge が知るのは
archive member family に戻る ref までであり、
member body compare まではまだ bridge に入れない。

### example B — witnessed draw archive-member-family-ready retained bridge

```text
proof_notebook_bridge_archive_member_family_ready_sketch = {
  proof_notebook_bridge_archive_manifest_ready_sketch = bridge_sketch:sugoroku.draw.archive_manifest_ready,
  archive_bundle_member_family_ref = archive_member_family:sugoroku.draw.latest
}
```

draw archive member family へ戻る ref は notebook bridge で追えるが、
member body compare までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で archive member family comparison を切り、current first choice は `archive_bundle_member_family_ref` のみを足す archive-member-family-ready retained bridge である
- next later reopen は actual archive member body compare comparison である
- actual archive member body compare は still 後段に残す

### not decided

- actual archive member body compare の最小 shape
- actual archive member body compare を member-body ref 1 本に留めるか、bless / update policy まで further split するか
- bless / update policy / retained archive payload への接続
