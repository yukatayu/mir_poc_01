# 180 — current L2 theorem line archive-bless-update-policy-ready retained-archive-payload threshold

## 目的

`specs/examples/179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md`
までを前提に、

- archive-bless-update-policy-ready retained bridge に retained archive payload をどこまで足すか
- archive retention layout と actual bless-side / update-side row split をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の archive-bless-update-policy-ready retained-archive-payload threshold** であり、

- archive retention layout
- actual bless-side row / update-side row split

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current archive-bless-update-policy-ready retained bridge を起点にする。
- archive retention layout、actual bless-side row / update-side row split、actual retained payload body family は巻き込まない。

## current 前提

current repo では次が成立している。

1. archive bless / update policy family までは current first choice に入っている。
2. retained archive payload は still 後段に残している。

したがって current 問いは、
**archive-bless-update-policy-ready retained bridge に `retained_archive_payload_ref` だけを先に足してよいか、それとも actual retained payload family / archive retention layout とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual archive bless / update policy family と retained archive payload family の line を narrow に切れるか
2. retained archive payload family を symbolic ref 1 本で留め、archive retention layout を still 後段に残せるか
3. actual retained payload body family を theorem-line bridge に premature に混ぜないか
4. next later reopen を archive retention layout comparison へ narrow に進めるか

## 比較対象

### 案 1. archive-bless-update-policy-ready retained bridge を terminal cut にし、retained archive payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- archive retention layout と retained payload body family の混線をさらに避けられる

#### 欠点

- archive bless / update policy family の次段が prose 依存に残りやすい
- theorem-line bridge の retained payload return path が弱い

### 案 2. retained archive payload family ref だけを持つ retained-archive-payload-ready retained bridge にする

#### 読み

archive-bless-update-policy-ready retained bridge に、
archive retention layout や actual retained payload body family を入れずに
**`retained_archive_payload_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_archive_payload_ready_sketch = {
  proof_notebook_bridge_archive_bless_update_policy_ready_sketch,
  retained_archive_payload_ref
}
```

#### 利点

- actual archive bless / update policy family と retained archive payload family の line を narrow に橋渡しできる
- archive retention layout を still 後段に残せる
- actual retained payload body family を premature に混ぜずに済む
- next later reopen を archive retention layout comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- payload family ref と retention layout ref を誤読されない説明が要る

### 案 3. retained archive payload の body row / bless-side payload / update-side payload をまとめて入れる

#### 利点

- notebook consumer が retained payload line をさらに細かく追える
- archive retention layout pressure に早く近づける

#### 欠点

- actual retained payload body family、archive retention layout、bless-side / update-side row split を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific retention lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. retained archive payload family ref だけを持つ retained-archive-payload-ready retained bridge にする**
である。

理由は次の通り。

1. archive bless / update policy family の次段を symbolic ref で narrow に橋渡しできる
2. archive retention layout を still 後段に残せる
3. actual retained payload body family を still 後段に残せる
4. next later reopen を archive retention layout comparison へ狭く進めやすい

## minimal retained-archive-payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_archive_payload_ready_sketch = {
  proof_notebook_bridge_archive_bless_update_policy_ready_sketch,
  retained_archive_payload_ref
}
```

### `retained_archive_payload_ref`

`retained_archive_payload_ref` は、

- actual archive bless / update policy family の次段で
- notebook consumer が **actual retained archive payload family** に戻れる
  symbolic payload-family ref

だけを置く field である。

current task では、

- archive retention layout
- actual bless-side row / update-side row split
- actual retained payload body family

をここに入れない。

## なぜ archive retention layout をまだ入れないか

archive retention layout を current phase で入れると、

- retained archive payload family
- archive retention layout
- actual retained payload body family

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual retained archive payload family へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ bless-side / update-side row split をまだしないか

actual bless-side row / update-side row split を current phase で入れると、

- actual archive bless / update policy family
- retained archive payload family
- archive retention layout

の切り分けが同じ reopen で崩れやすい。

current pressure は retained payload family ref を 1 本だけ bridge に足すところまでで十分である。

## practical examples

### example A — fallback chain retained-archive-payload-ready retained bridge

```text
proof_notebook_bridge_retained_archive_payload_ready_sketch = {
  proof_notebook_bridge_archive_bless_update_policy_ready_sketch = bridge_sketch:e12.archive_bless_update_policy_ready,
  retained_archive_payload_ref = retained_archive_payload:e12.latest
}
```

ここで notebook bridge が知るのは
retained archive payload family に戻る ref までであり、
archive retention layout や actual payload body family まではまだ bridge に入れない。

### example B — witnessed draw retained-archive-payload-ready retained bridge

```text
proof_notebook_bridge_retained_archive_payload_ready_sketch = {
  proof_notebook_bridge_archive_bless_update_policy_ready_sketch = bridge_sketch:sugoroku.draw.archive_bless_update_policy_ready,
  retained_archive_payload_ref = retained_archive_payload:sugoroku.draw.latest
}
```

draw retained archive payload family へ戻る ref は notebook bridge で追えるが、
archive retention layout や actual payload body family までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で retained archive payload comparison を切り、current first choice は `retained_archive_payload_ref` だけを足す retained-archive-payload-ready retained bridge である
- archive retention layout と actual bless-side row / update-side row split は still 後段に残す
- next later reopen は archive retention layout comparison である

### not decided

- archive retention layout の最小 shape
- actual bless-side row と update-side row をどこで分けるか
- actual retained archive payload body family の最小 shape
