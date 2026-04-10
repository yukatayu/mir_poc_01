# 184 — current L2 theorem line retained-payload-materialization-family-ready retained-payload-body-materialization-detail threshold

## 目的

`specs/examples/183-current-l2-theorem-line-retained-archive-payload-body-family-ready-retained-payload-materialization-family-threshold.md`
までを前提に、

- retained-payload-materialization-family-ready retained bridge に actual retained payload body materialization detail をどこまで足すか
- actual bless-side / update-side row split と actual retained payload body materialization payload をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-materialization-family-ready retained-payload-body-materialization-detail threshold** であり、

- actual bless-side row / update-side row split
- actual retained payload body materialization payload

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-materialization-family-ready retained bridge を起点にする。
- actual bless-side / update-side row split、actual retained payload body materialization payload、actual retained payload body materialization carrier detail は巻き込まない。

## current 前提

current repo では次が成立している。

1. retained payload materialization family までは current first choice に入っている。
2. actual retained payload body materialization detail は still 後段に残している。

したがって current 問いは、
**retained-payload-materialization-family-ready retained bridge に `retained_payload_body_materialization_detail_ref` だけを先に足してよいか、それとも bless-side / update-side row split や actual retained payload body materialization payload とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. retained payload materialization family と actual retained payload body materialization detail の line を narrow に切れるか
2. actual retained payload body materialization detail を symbolic ref 1 本で留め、bless-side / update-side row split を still 後段に残せるか
3. actual retained payload body materialization payload を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual retained payload body materialization payload comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-payload-materialization-family-ready retained bridge を terminal cut にし、actual retained payload body materialization detail も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- materialization detail と payload materialization payload の混線をさらに避けられる

#### 欠点

- materialization family の次段が prose 依存に残りやすい
- theorem-line bridge の body materialization detail return path が弱い

### 案 2. actual retained payload body materialization detail ref だけを持つ retained-payload-body-materialization-detail-ready retained bridge にする

#### 読み

retained-payload-materialization-family-ready retained bridge に、
actual bless-side / update-side row split や actual retained payload body materialization payload を入れずに
**`retained_payload_body_materialization_detail_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_body_materialization_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_materialization_family_ready_sketch,
  retained_payload_body_materialization_detail_ref
}
```

#### 利点

- retained payload materialization family と actual retained payload body materialization detail の line を narrow に橋渡しできる
- bless-side / update-side row split を still 後段に残せる
- actual retained payload body materialization payload を premature に混ぜずに済む
- next later reopen を actual retained payload body materialization payload comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- body materialization detail ref と payload materialization payload ref を誤読されない説明が要る

### 案 3. body materialization detail と bless-side / update-side row split をまとめて入れる

#### 利点

- notebook consumer が materialization body line をさらに細かく追える
- bless/update 実体 pressure に早く近づける

#### 欠点

- body materialization detail、bless-side / update-side row split、payload materialization payload を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific materialization payload lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual retained payload body materialization detail ref だけを持つ retained-payload-body-materialization-detail-ready retained bridge にする**
である。

理由は次の通り。

1. materialization family の次段を symbolic ref で narrow に橋渡しできる
2. bless-side / update-side row split を still 後段に残せる
3. actual retained payload body materialization payload を still 後段に残せる
4. next later reopen を actual retained payload body materialization payload comparison へ狭く進めやすい

## minimal retained-payload-body-materialization-detail-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_body_materialization_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_materialization_family_ready_sketch,
  retained_payload_body_materialization_detail_ref
}
```

### `retained_payload_body_materialization_detail_ref`

`retained_payload_body_materialization_detail_ref` は、

- retained payload materialization family の次段で
- notebook consumer が **actual retained payload body materialization detail** に戻れる
  symbolic body-materialization-detail ref

だけを置く field である。

current task では、

- actual bless-side row / update-side row split
- actual retained payload body materialization payload
- actual retained payload body materialization carrier detail

をここに入れない。

## なぜ bless-side / update-side row split をまだ入れないか

actual bless-side / update-side row split を current phase で入れると、

- actual retained payload body materialization detail
- bless-side / update-side row split
- actual retained payload body materialization payload

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual retained payload body materialization detail へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ actual retained payload body materialization payload をまだ入れないか

actual retained payload body materialization payload を current phase で入れると、

- retained payload materialization family
- actual retained payload body materialization detail
- actual retained payload body materialization payload

の切り分けが同じ reopen で崩れやすい。

current pressure は body materialization detail ref を 1 本だけ bridge に足すところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-body-materialization-detail-ready retained bridge

```text
proof_notebook_bridge_retained_payload_body_materialization_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_materialization_family_ready_sketch = bridge_sketch:e12.retained_payload_materialization_family_ready,
  retained_payload_body_materialization_detail_ref = retained_payload_body_materialization_detail:e12.latest
}
```

ここで notebook bridge が知るのは
actual retained payload body materialization detail に戻る ref までであり、
bless-side / update-side row split や materialization payload まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-body-materialization-detail-ready retained bridge

```text
proof_notebook_bridge_retained_payload_body_materialization_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_materialization_family_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_materialization_family_ready,
  retained_payload_body_materialization_detail_ref = retained_payload_body_materialization_detail:sugoroku.draw.latest
}
```

draw actual retained payload body materialization detail へ戻る ref は notebook bridge で追えるが、
bless-side / update-side row split や materialization payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual retained payload body materialization detail comparison を切り、current first choice は `retained_payload_body_materialization_detail_ref` だけを足す retained-payload-body-materialization-detail-ready retained bridge である
- actual bless-side row / update-side row split と actual retained payload body materialization payload は still 後段に残す
- next later reopen は actual retained payload body materialization payload comparison である

### not decided

- actual retained payload body materialization payload の最小 shape
- actual bless-side row と update-side row をどこで分けるか
- actual retained payload body materialization carrier detail をどこで接続するか
