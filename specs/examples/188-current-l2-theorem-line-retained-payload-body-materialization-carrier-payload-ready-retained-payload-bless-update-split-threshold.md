# 188 — current L2 theorem line retained-payload-body-materialization-carrier-payload-ready retained-payload-bless-update-split threshold

## 目的

`specs/examples/187-current-l2-theorem-line-retained-payload-body-materialization-carrier-detail-ready-retained-payload-body-materialization-carrier-payload-threshold.md`
までを前提に、

- retained-payload-body-materialization-carrier-payload-ready retained bridge に actual bless-side / update-side row split をどこまで足すか
- bless-side row / update-side row pair 自体をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-body-materialization-carrier-payload-ready retained-payload-bless-update-split threshold** であり、

- bless-side row / update-side row pair

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-body-materialization-carrier-payload-ready retained bridge を起点にする。
- actual bless-side row / update-side row pair は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual retained payload body materialization carrier payload までは current first choice に入っている。
2. actual bless-side / update-side row split は still 後段に残している。

したがって current 問いは、
**retained-payload-body-materialization-carrier-payload-ready retained bridge に `retained_payload_body_materialization_bless_update_split_ref` だけを先に足してよいか、それとも bless-side row / update-side row pair とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual retained payload body materialization carrier payload と actual bless-side / update-side row split の line を narrow に切れるか
2. actual bless-side / update-side row split を symbolic ref 1 本で留め、bless-side row / update-side row pair を still 後段に残せるか
3. bless-side row / update-side row pair を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual bless-side / update-side row pair comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-payload-body-materialization-carrier-payload-ready retained bridge を terminal cut にし、actual bless-side / update-side row split も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- split と row pair の混線をさらに避けられる

#### 欠点

- carrier payload の次段が prose 依存に残りやすい
- theorem-line bridge の bless/update split return path が弱い

### 案 2. actual bless/update split ref だけを持つ retained-payload-bless-update-split-ready retained bridge にする

#### 読み

retained-payload-body-materialization-carrier-payload-ready retained bridge に、
bless-side row / update-side row pair 自体を入れずに
**`retained_payload_body_materialization_bless_update_split_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_bless_update_split_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch,
  retained_payload_body_materialization_bless_update_split_ref
}
```

#### 利点

- carrier payload と bless/update split の line を narrow に橋渡しできる
- bless-side row / update-side row pair を still 後段に残せる
- next later reopen を actual bless-side / update-side row pair comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- split ref と actual row pair を誤読されない説明が要る

### 案 3. bless-side row ref / update-side row ref を current phase で同時に入れる

#### 利点

- notebook consumer が split 次段の row pair まで一気に追える
- row-pair specific pressure に早く近づける

#### 欠点

- split と row pair を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific row pairing を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual bless/update split ref だけを持つ retained-payload-bless-update-split-ready retained bridge にする**
である。

理由は次の通り。

1. carrier payload の次段を symbolic ref で narrow に橋渡しできる
2. bless-side row / update-side row pair を still 後段に残せる
3. next later reopen を actual bless-side / update-side row pair comparison へ狭く進めやすい

## minimal retained-payload-bless-update-split-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_bless_update_split_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch,
  retained_payload_body_materialization_bless_update_split_ref
}
```

### `retained_payload_body_materialization_bless_update_split_ref`

`retained_payload_body_materialization_bless_update_split_ref` は、

- actual retained payload body materialization carrier payload の次段で
- notebook consumer が **actual bless-side / update-side row split** に戻れる
  symbolic split ref

だけを置く field である。

current task では、

- bless-side row / update-side row pair

をここに入れない。

## なぜ bless-side row / update-side row pair をまだ入れないか

bless-side row / update-side row pair を current phase で入れると、

- actual bless-side / update-side row split
- bless-side row / update-side row pair

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual bless-side / update-side row split へ戻る symbolic ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-bless-update-split-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_split_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch = bridge_sketch:e12.retained_payload_body_materialization_carrier_payload_ready,
  retained_payload_body_materialization_bless_update_split_ref = retained_payload_bless_update_split:e12.latest
}
```

ここで notebook bridge が知るのは
actual bless/update split に戻る ref までであり、
bless-side row / update-side row pair まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-bless-update-split-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_split_ready_sketch = {
  proof_notebook_bridge_retained_payload_body_materialization_carrier_payload_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_body_materialization_carrier_payload_ready,
  retained_payload_body_materialization_bless_update_split_ref = retained_payload_bless_update_split:sugoroku.draw.latest
}
```

draw actual bless/update split へ戻る ref は notebook bridge で追えるが、
bless-side row / update-side row pair までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual bless-side / update-side row split comparison を切り、current first choice は `retained_payload_body_materialization_bless_update_split_ref` だけを足す retained-payload-bless-update-split-ready retained bridge である
- bless-side row / update-side row pair は still 後段に残す
- next later reopen は actual bless-side / update-side row pair comparison である

### not decided

- bless-side row と update-side row の最小 pair shape
- row pair を 2 ref で切るか named pair bundle で切るか
- typed symbolic ref family を boundary-specific handoff artifact へ昇格させる concrete pressure を何とみなすか
