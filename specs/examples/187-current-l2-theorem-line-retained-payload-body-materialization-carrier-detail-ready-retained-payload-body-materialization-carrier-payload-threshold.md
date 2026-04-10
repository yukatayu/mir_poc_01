# 187 — current L2 theorem line retained-payload-body-materialization-carrier-detail-ready retained-payload-body-materialization-carrier-payload threshold

## 目的

`specs/examples/186-current-l2-theorem-line-retained-payload-body-materialization-payload-ready-retained-payload-body-materialization-carrier-detail-threshold.md`
までを前提に、

- retained-payload-body-materialization-carrier-detail-ready retained bridge に actual retained payload body materialization carrier payload をどこまで足すか
- actual bless-side / update-side row split をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-body-materialization-carrier-detail-ready retained-payload-body-materialization-carrier-payload threshold** であり、

- actual bless-side row / update-side row split

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-body-materialization-carrier-detail-ready retained bridge を起点にする。
- actual bless-side / update-side row split は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual retained payload body materialization carrier detail までは current first choice に入っている。
2. actual retained payload body materialization carrier payload は still 後段に残している。

したがって current 問いは、
**retained-payload-body-materialization-carrier-detail-ready retained bridge に `retained_payload_body_materialization_carrier_payload_ref` だけを先に足してよいか、それとも bless-side / update-side row split とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual retained payload body materialization carrier detail と actual retained payload body materialization carrier payload の line を narrow に切れるか
2. actual retained payload body materialization carrier payload を symbolic ref 1 本で留め、actual bless-side / update-side row split を still 後段に残せるか
3. actual bless-side / update-side row split を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual bless-side / update-side row split comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-payload-body-materialization-carrier-detail-ready retained bridge を terminal cut にし、actual retained payload body materialization carrier payload も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- carrier payload と bless/update split の混線をさらに避けられる

#### 欠点

- carrier detail の次段が prose 依存に残りやすい
- theorem-line bridge の carrier payload return path が弱い

### 案 2. actual retained payload body materialization carrier payload ref だけを持つ retained-payload-body-materialization-carrier-payload-ready retained bridge にする

#### 読み

retained-payload-body-materialization-carrier-detail-ready retained bridge に、
actual bless-side / update-side row split を入れずに
**`retained_payload_body_materialization_carrier_payload_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch,
  retained_payload_body_materialization_carrier_payload_ref
}
```

#### 利点

- actual retained payload body materialization carrier detail と actual retained payload body materialization carrier payload の line を narrow に橋渡しできる
- bless-side / update-side row split を still 後段に残せる
- next later reopen を actual bless-side / update-side row split comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- carrier payload ref と bless/update split を誤読されない説明が要る

### 案 3. carrier payload と bless-side / update-side row split をまとめて入れる

#### 利点

- notebook consumer が actual carrier payload の次段まで一気に追える
- bless/update 具体圧力に早く近づける

#### 欠点

- carrier payload と bless/update split を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific split policy を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual retained payload body materialization carrier payload ref だけを持つ retained-payload-body-materialization-carrier-payload-ready retained bridge にする**
である。

理由は次の通り。

1. carrier detail の次段を symbolic ref で narrow に橋渡しできる
2. bless-side / update-side row split を still 後段に残せる
3. next later reopen を actual bless-side / update-side row split comparison へ狭く進めやすい

## minimal retained-payload-body-materialization-carrier-payload-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch,
  retained_payload_body_materialization_carrier_payload_ref
}
```

### `retained_payload_body_materialization_carrier_payload_ref`

`retained_payload_body_materialization_carrier_payload_ref` は、

- actual retained payload body materialization carrier detail の次段で
- notebook consumer が **actual retained payload body materialization carrier payload** に戻れる
  symbolic body-materialization-carrier-payload ref

だけを置く field である。

current task では、

- actual bless-side row / update-side row split

をここに入れない。

## なぜ actual bless-side / update-side row split をまだ入れないか

actual bless-side / update-side row split を current phase で入れると、

- actual retained payload body materialization carrier payload
- actual bless-side / update-side row split

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual retained payload body materialization carrier payload へ戻る symbolic ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-body-materialization-carrier-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch = bridge_sketch:e12.retained_payload_body_materialization_carrier_detail_ready,
  retained_payload_body_materialization_carrier_payload_ref = retained_payload_body_materialization_carrier_payload:e12.latest
}
```

ここで notebook bridge が知るのは
actual retained payload body materialization carrier payload に戻る ref までであり、
bless-side / update-side row split まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-body-materialization-carrier-payload-ready retained bridge

```text
proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_detail_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_body_materialization_carrier_detail_ready,
  retained_payload_body_materialization_carrier_payload_ref = retained_payload_body_materialization_carrier_payload:sugoroku.draw.latest
}
```

draw actual retained payload body materialization carrier payload へ戻る ref は notebook bridge で追えるが、
bless-side / update-side row split までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual retained payload body materialization carrier payload comparison を切り、current first choice は `retained_payload_body_materialization_carrier_payload_ref` だけを足す retained-payload-body-materialization-carrier-payload-ready retained bridge である
- actual bless-side row / update-side row split は still 後段に残す
- next later reopen は actual bless-side / update-side row split comparison である

### not decided

- actual bless-side row と update-side row の最小 split shape
- split を carrier payload line の直下で切るか、still bridge 外の policy row として切るか
- typed symbolic ref family を boundary-specific handoff artifact へ昇格させる concrete pressure を何とみなすか
