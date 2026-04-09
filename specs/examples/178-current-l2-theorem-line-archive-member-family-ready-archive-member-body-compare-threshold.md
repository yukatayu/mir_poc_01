# 178 — current L2 theorem line archive-member-family-ready archive-member-body-compare threshold

## 目的

`specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`
までを前提に、

- archive-member-family-ready retained bridge に actual archive member body compare をどこまで足すか
- actual archive bless / update policy をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-member-family-ready archive-member-body-compare threshold** であり、

- actual archive bless / update policy
- retained archive payload

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-member-family-ready retained bridge を起点にする。
- bless / update policy、retained archive payload、archive retention layout は巻き込まない。

## current 前提

current repo では次が成立している。

1. archive bundle member family までは current first choice に入っている。
2. actual archive member body compare は still 後段に残している。

したがって current 問いは、
**archive-member-family-ready retained bridge に `archive_member_body_compare_ref` だけを先に足してよいか、それとも actual archive bless / update policy とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual archive bundle member family と actual archive member body compare の line を narrow に切れるか
2. actual archive member body compare と actual archive bless / update policy を分離できるか
3. actual archive bless / update policy を theorem-line bridge に premature に混ぜないか
4. next later reopen を archive bless / update policy comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-member-family-ready retained bridge を terminal cut にし、actual archive member body compare も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- bless / update policy との混線をさらに避けられる

#### 欠点

- actual archive bundle member family と actual archive member body compare の line が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. actual archive member-body-compare ref だけを持つ archive-member-body-compare-ready retained bridge にする

#### 読み

archive-member-family-ready retained bridge に、
actual archive bless / update policy を入れずに
**`archive_member_body_compare_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_archive_member_body_compare_ready_sketch = {
  proof_notebook_bridge_archive_member_family_ready_sketch,
  archive_member_body_compare_ref
}
```

#### 利点

- actual archive bundle member family と actual archive member body compare の line を narrow に橋渡しできる
- actual archive bless / update policy を still 後段に残せる
- next later reopen を archive bless / update policy comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- archive member body compare と bless / update policy を誤読されない説明が要る

### 案 3. actual archive member body compare と bless / update policy をまとめて入れる

#### 利点

- notebook consumer が actual archive member compare flow を一度に追える
- bless / update decision pressure に近づく

#### 欠点

- actual archive member body compare と bless / update policy を premature に混ぜやすい
- narrow reopen を stage ごとに切りにくい

## current judgment

current L2 で最も自然なのは、
**案 2. actual archive member-body-compare ref だけを持つ archive-member-body-compare-ready retained bridge にする**
である。

理由は次の通り。

1. actual archive bundle member family と actual archive member body compare の line を narrow に接続できる
2. actual archive bless / update policy を still 後段に残せる
3. current docs-first discipline を壊さない
4. next later reopen を archive bless / update policy comparison へ狭く進めやすい

## minimal archive-member-body-compare-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_archive_member_body_compare_ready_sketch = {
  proof_notebook_bridge_archive_member_family_ready_sketch,
  archive_member_body_compare_ref
}
```

### `archive_member_body_compare_ref`

`archive_member_body_compare_ref` は、

- actual archive bundle member family の次段で
- notebook consumer が **actual archive member body compare family** に戻れる
  symbolic archive-member-body-compare ref

だけを置く field である。

current task では、

- actual archive bless / update policy
- retained archive payload

をここに入れない。

## なぜ actual archive bless / update policy をまだ入れないか

actual archive bless / update policy を current phase で入れると、

- actual archive member body compare
- actual archive bless / update policy
- retained archive payload
- archive retention layout

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual archive member body compare を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain archive-member-body-compare-ready retained bridge

```text
proof_notebook_bridge_archive_member_body_compare_ready_sketch = {
  proof_notebook_bridge_archive_member_family_ready_sketch = bridge_sketch:e12.archive_member_family_ready,
  archive_member_body_compare_ref = archive_member_body_compare:e12.latest
}
```

ここで notebook bridge が知るのは
archive member body compare に戻る ref までであり、
bless / update policy まではまだ bridge に入れない。

### example B — witnessed draw archive-member-body-compare-ready retained bridge

```text
proof_notebook_bridge_archive_member_body_compare_ready_sketch = {
  proof_notebook_bridge_archive_member_family_ready_sketch = bridge_sketch:sugoroku.draw.archive_member_family_ready,
  archive_member_body_compare_ref = archive_member_body_compare:sugoroku.draw.latest
}
```

draw archive member body compare へ戻る ref は notebook bridge で追えるが、
bless / update policy までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で archive member body compare comparison を切り、current first choice は `archive_member_body_compare_ref` のみを足す archive-member-body-compare-ready retained bridge である
- next later reopen は actual archive bless / update policy comparison である
- actual archive bless / update policy は still 後段に残す

### not decided

- actual archive bless / update policy の最小 shape
- bless / update policy を bless-side ref と update-side ref に分けるか、policy family ref 1 本に留めるか
- retained archive payload / archive retention layout への接続
