# 179 — current L2 theorem line archive-member-body-compare-ready archive-bless-update-policy threshold

## 目的

`specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`
までを前提に、

- archive-member-body-compare-ready retained bridge に actual archive bless / update policy をどこまで足すか
- retained archive payload をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-member-body-compare-ready archive-bless-update-policy threshold** であり、

- retained archive payload
- archive retention layout

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-member-body-compare-ready retained bridge を起点にする。
- retained archive payload、archive retention layout、actual archive bless / update row split は巻き込まない。

## current 前提

current repo では次が成立している。

1. archive member body compare までは current first choice に入っている。
2. actual archive bless / update policy は still 後段に残している。

したがって current 問いは、
**archive-member-body-compare-ready retained bridge に `archive_bless_update_policy_ref` だけを先に足してよいか、それとも actual retained archive payload とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual archive member body compare と actual archive bless / update policy の line を narrow に切れるか
2. bless / update policy を family ref 1 本で留め、actual bless-side / update-side split を still 後段に残せるか
3. retained archive payload を theorem-line bridge に premature に混ぜないか
4. next later reopen を retained archive payload comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-member-body-compare-ready retained bridge を terminal cut にし、actual archive bless / update policy も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- retained archive payload と bless / update policy の混線をさらに避けられる

#### 欠点

- archive member body compare の次段が prose 依存に残りやすい
- theorem-line bridge の actual policy return path が弱い

### 案 2. actual archive bless/update policy family ref だけを持つ archive-bless-update-policy-ready retained bridge にする

#### 読み

archive-member-body-compare-ready retained bridge に、
retained archive payload を入れずに
**`archive_bless_update_policy_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_archive_bless_update_policy_ready_sketch = {
  proof_notebook_bridge_archive_member_body_compare_ready_sketch,
  archive_bless_update_policy_ref
}
```

#### 利点

- archive member body compare と actual archive bless / update policy の line を narrow に橋渡しできる
- bless / update split 自体は still 後段に残せる
- retained archive payload を premature に混ぜずに済む
- next later reopen を retained archive payload comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- policy family ref と retained payload ref を誤読されない説明が要る

### 案 3. bless-side ref / update-side ref を分けるか、retained archive payload までまとめて入れる

#### 利点

- notebook consumer が actual bless / update flow をさらに細かく追える
- retained archive payload pressure に早く近づける

#### 欠点

- bless / update split と retained archive payload を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual archive bless/update policy family ref だけを持つ archive-bless-update-policy-ready retained bridge にする**
である。

理由は次の通り。

1. archive member body compare の次段を symbolic ref で narrow に橋渡しできる
2. bless / update split 自体は still 後段に残せる
3. retained archive payload と archive retention layout を still 後段に残せる
4. next later reopen を retained archive payload comparison へ狭く進めやすい

## minimal archive-bless-update-policy-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_archive_bless_update_policy_ready_sketch = {
  proof_notebook_bridge_archive_member_body_compare_ready_sketch,
  archive_bless_update_policy_ref
}
```

### `archive_bless_update_policy_ref`

`archive_bless_update_policy_ref` は、

- actual archive member body compare の次段で
- notebook consumer が **actual archive bless / update policy family** に戻れる
  symbolic policy-family ref

だけを置く field である。

current task では、

- actual bless-side row / update-side row
- retained archive payload
- archive retention layout

をここに入れない。

## なぜ bless / update split をまだしないか

actual bless-side ref と update-side ref を current phase で分けると、

- bless-side lifecycle
- update-side lifecycle
- retained archive payload の attach point

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual archive bless / update policy family へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ retained archive payload をまだ入れないか

retained archive payload を current phase で入れると、

- actual archive bless / update policy
- retained archive payload
- archive retention layout

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual archive bless / update policy family を symbolic ref で stable に切るところまでで十分である。

## practical examples

### example A — fallback chain archive-bless-update-policy-ready retained bridge

```text
proof_notebook_bridge_archive_bless_update_policy_ready_sketch = {
  proof_notebook_bridge_archive_member_body_compare_ready_sketch = bridge_sketch:e12.archive_member_body_compare_ready,
  archive_bless_update_policy_ref = archive_policy:e12.latest
}
```

ここで notebook bridge が知るのは
archive bless / update policy family に戻る ref までであり、
retained archive payload まではまだ bridge に入れない。

### example B — witnessed draw archive-bless-update-policy-ready retained bridge

```text
proof_notebook_bridge_archive_bless_update_policy_ready_sketch = {
  proof_notebook_bridge_archive_member_body_compare_ready_sketch = bridge_sketch:sugoroku.draw.archive_member_body_compare_ready,
  archive_bless_update_policy_ref = archive_policy:sugoroku.draw.latest
}
```

draw archive bless / update policy family へ戻る ref は notebook bridge で追えるが、
retained archive payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で archive bless / update policy comparison を切り、current first choice は `archive_bless_update_policy_ref` だけを足す archive-bless-update-policy-ready retained bridge である
- bless / update split と retained archive payload は still 後段に残す
- next later reopen は retained archive payload comparison である

### not decided

- actual bless-side row と update-side row をどこで分けるか
- retained archive payload の最小 shape
- archive retention layout への接続
