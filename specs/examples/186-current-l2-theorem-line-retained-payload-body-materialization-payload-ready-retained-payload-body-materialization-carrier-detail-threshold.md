# 186 — current L2 theorem line retained-payload-body-materialization-payload-ready retained-payload-body-materialization-carrier-detail threshold

## 目的

`specs/examples/185-current-l2-theorem-line-retained-payload-body-materialization-detail-ready-retained-payload-body-materialization-payload-threshold.md`
までを前提に、

- retained-payload-body-materialization-payload-ready retained bridge に actual retained payload body materialization carrier detail をどこまで足すか
- actual bless-side / update-side row split と actual retained payload body materialization carrier payload をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-body-materialization-payload-ready retained-payload-body-materialization-carrier-detail threshold** であり、

- actual bless-side row / update-side row split
- actual retained payload body materialization carrier payload

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-body-materialization-payload-ready retained bridge を起点にする。
- actual bless-side / update-side row split、actual retained payload body materialization carrier payload は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual retained payload body materialization payload までは current first choice に入っている。
2. actual retained payload body materialization carrier detail は still 後段に残している。

したがって current 問いは、
**retained-payload-body-materialization-payload-ready retained bridge に `retained_payload_body_materialization_carrier_detail_ref` だけを先に足してよいか、それとも bless-side / update-side row split や actual retained payload body materialization carrier payload とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual retained payload body materialization payload と actual retained payload body materialization carrier detail の line を narrow に切れるか
2. actual retained payload body materialization carrier detail を symbolic ref 1 本で留め、bless-side / update-side row split を still 後段に残せるか
3. actual retained payload body materialization carrier payload を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual retained payload body materialization carrier payload comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-payload-body-materialization-payload-ready retained bridge を terminal cut にし、actual retained payload body materialization carrier detail も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- carrier detail と carrier payload の混線をさらに避けられる

#### 欠点

- body materialization payload の次段が prose 依存に残りやすい
- theorem-line bridge の carrier detail return path が弱い

### 案 2. actual retained payload body materialization carrier detail ref だけを持つ retained-payload-body-materialization-carrier-detail-ready retained bridge にする

#### 読み

retained-payload-body-materialization-payload-ready retained bridge に、
actual bless-side / update-side row split や actual retained payload body materialization carrier payload を入れずに
**`retained_payload_body_materialization_carrier_detail_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_payload_ready_sketch,
  retained_payload_body_materialization_carrier_detail_ref
}
```

#### 利点

- actual retained payload body materialization payload と actual retained payload body materialization carrier detail の line を narrow に橋渡しできる
- bless-side / update-side row split を still 後段に残せる
- actual retained payload body materialization carrier payload を premature に混ぜずに済む
- next later reopen を actual retained payload body materialization carrier payload comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- carrier detail ref と carrier payload ref を誤読されない説明が要る

### 案 3. carrier detail と bless-side / update-side row split をまとめて入れる

#### 利点

- notebook consumer が carrier line をさらに細かく追える
- bless/update 実体 pressure に早く近づける

#### 欠点

- carrier detail、bless-side / update-side row split、carrier payload を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific carrier lifecycle を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual retained payload body materialization carrier detail ref だけを持つ retained-payload-body-materialization-carrier-detail-ready retained bridge にする**
である。

理由は次の通り。

1. body materialization payload の次段を symbolic ref で narrow に橋渡しできる
2. bless-side / update-side row split を still 後段に残せる
3. actual retained payload body materialization carrier payload を still 後段に残せる
4. next later reopen を actual retained payload body materialization carrier payload comparison へ狭く進めやすい

## minimal retained-payload-body-materialization-carrier-detail-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_payload_ready_sketch,
  retained_payload_body_materialization_carrier_detail_ref
}
```

### `retained_payload_body_materialization_carrier_detail_ref`

`retained_payload_body_materialization_carrier_detail_ref` は、

- actual retained payload body materialization payload の次段で
- notebook consumer が **actual retained payload body materialization carrier detail** に戻れる
  symbolic body-materialization-carrier-detail ref

だけを置く field である。

current task では、

- actual bless-side row / update-side row split
- actual retained payload body materialization carrier payload

をここに入れない。

## なぜ bless-side / update-side row split をまだ入れないか

actual bless-side / update-side row split を current phase で入れると、

- actual retained payload body materialization carrier detail
- bless-side / update-side row split
- actual retained payload body materialization carrier payload

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual retained payload body materialization carrier detail へ戻る symbolic ref を stable に切るところまでで十分である。

## なぜ actual retained payload body materialization carrier payload をまだ入れないか

actual retained payload body materialization carrier payload を current phase で入れると、

- actual retained payload body materialization payload
- actual retained payload body materialization carrier detail
- actual retained payload body materialization carrier payload

の切り分けが同じ reopen で崩れやすい。

current pressure は carrier detail ref を 1 本だけ bridge に足すところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-body-materialization-carrier-detail-ready retained bridge

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_payload_ready_sketch = bridge_sketch:e12.retained_payload_body_materialization_payload_ready,
  retained_payload_body_materialization_carrier_detail_ref = retained_payload_body_materialization_carrier_detail:e12.latest
}
```

ここで notebook bridge が知るのは
actual retained payload body materialization carrier detail に戻る ref までであり、
bless-side / update-side row split や carrier payload まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-body-materialization-carrier-detail-ready retained bridge

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_payload_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_body_materialization_payload_ready,
  retained_payload_body_materialization_carrier_detail_ref = retained_payload_body_materialization_carrier_detail:sugoroku.draw.latest
}
```

draw actual retained payload body materialization carrier detail へ戻る ref は notebook bridge で追えるが、
bless-side / update-side row split や carrier payload までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual retained payload body materialization carrier detail comparison を切り、current first choice は `retained_payload_body_materialization_carrier_detail_ref` だけを足す retained-payload-body-materialization-carrier-detail-ready retained bridge である
- actual bless-side row / update-side row split と actual retained payload body materialization carrier payload は still 後段に残す
- next later reopen は actual retained payload body materialization carrier payload comparison である

### not decided

- actual retained payload body materialization carrier payload の最小 shape
- actual bless-side row と update-side row をどこで分けるか
- actual retained payload body materialization carrier payload を bless/update split の前後どちらで接続するか
