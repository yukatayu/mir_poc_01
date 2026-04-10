# 190 — current L2 theorem line retained-payload-bless-update-row-pair-ready retained-payload-bless-update-row-ref-family threshold

## 目的

`specs/examples/189-current-l2-theorem-line-retained-payload-bless-update-split-ready-retained-payload-bless-update-row-pair-threshold.md`
までを前提に、

- retained-payload-bless-update-row-pair-ready retained bridge に actual bless-side row ref / update-side row ref family をどこまで足すか
- bless-side row ref / update-side row ref 自体をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-bless-update-row-pair-ready retained-payload-bless-update-row-ref-family threshold** であり、

- bless-side row ref / update-side row ref

の concrete dual ref shape は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-bless-update-row-pair-ready retained bridge を起点にする。
- actual bless-side row ref / update-side row ref の concrete dual ref shape は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual bless/update row pair までは current first choice に入っている。
2. actual bless-side row ref / update-side row ref family は still 後段に残している。

したがって current 問いは、
**retained-payload-bless-update-row-pair-ready retained bridge に `retained_payload_body_materialization_bless_update_row_ref_family_ref` だけを先に足してよいか、それとも bless-side row ref / update-side row ref とまとめて still bridge 外へ残すべきか**
である。

## 比較観点

1. actual bless/update row pair と actual bless/update row ref family の line を narrow に切れるか
2. actual bless/update row ref family を symbolic ref 1 本で留め、bless-side row ref / update-side row ref の concrete dual ref shape を still 後段に残せるか
3. bless-side row ref / update-side row ref を theorem-line bridge に premature に混ぜないか
4. next later reopen を actual bless-side row ref / update-side row dual-ref comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-payload-bless-update-row-pair-ready retained bridge を terminal cut にし、actual bless/update row ref family も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- row ref family と dual ref shape の混線をさらに避けられる

#### 欠点

- row pair の次段が prose 依存に残りやすい
- theorem-line bridge の bless/update row ref family return path が弱い

### 案 2. actual bless/update row ref family ref だけを持つ retained-payload-bless-update-row-ref-family-ready retained bridge にする

#### 読み

retained-payload-bless-update-row-pair-ready retained bridge に、
bless-side row ref / update-side row ref の concrete dual ref shape 自体を入れずに
**`retained_payload_body_materialization_bless_update_row_ref_family_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_pair_ready_sketch,
  retained_payload_body_materialization_bless_update_row_ref_family_ref
}
```

#### 利点

- bless/update row pair と bless/update row ref family の line を narrow に橋渡しできる
- bless-side row ref / update-side row ref の concrete dual ref shape を still 後段に残せる
- next later reopen を actual bless-side row ref / update-side row dual-ref comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- row ref family ref と actual dual ref shape を誤読されない説明が要る

### 案 3. bless-side row ref / update-side row ref を current phase で同時に入れる

#### 利点

- notebook consumer が row ref family 次段の concrete dual ref shape まで一気に追える
- dual-ref specific pressure に早く近づける

#### 欠点

- row ref family と dual ref shape を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific dual ref shape を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. actual bless/update row ref family ref だけを持つ retained-payload-bless-update-row-ref-family-ready retained bridge にする**
である。

理由は次の通り。

1. bless/update row pair の次段を symbolic ref で narrow に橋渡しできる
2. bless-side row ref / update-side row ref の concrete dual ref shape を still 後段に残せる
3. next later reopen を actual bless-side row ref / update-side row dual-ref comparison へ狭く進めやすい

## minimal retained-payload-bless-update-row-ref-family-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_pair_ready_sketch,
  retained_payload_body_materialization_bless_update_row_ref_family_ref
}
```

### `retained_payload_body_materialization_bless_update_row_ref_family_ref`

`retained_payload_body_materialization_bless_update_row_ref_family_ref` は、

- actual bless/update row pair の次段で
- notebook consumer が **actual bless-side row ref / update-side row ref family** に戻れる
  symbolic row-ref-family ref

だけを置く field である。

current task では、

- bless-side row ref / update-side row ref の concrete dual ref shape

をここに入れない。

## なぜ bless-side row ref / update-side row ref の concrete dual ref shape をまだ入れないか

bless-side row ref / update-side row ref の concrete dual ref shape を current phase で入れると、

- actual bless/update row ref family
- bless-side row ref / update-side row ref の concrete dual ref shape

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず actual bless-side row ref / update-side row ref family へ戻る symbolic ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-bless-update-row-ref-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_pair_ready_sketch = bridge_sketch:e12.retained_payload_bless_update_row_pair_ready,
  retained_payload_body_materialization_bless_update_row_ref_family_ref = retained_payload_bless_update_row_ref_family:e12.latest
}
```

ここで notebook bridge が知るのは
actual bless/update row ref family に戻る ref までであり、
bless-side row ref / update-side row ref の concrete dual ref shape まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-bless-update-row-ref-family-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_pair_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_bless_update_row_pair_ready,
  retained_payload_body_materialization_bless_update_row_ref_family_ref = retained_payload_bless_update_row_ref_family:sugoroku.draw.latest
}
```

draw actual bless/update row ref family へ戻る ref は notebook bridge で追えるが、
bless-side row ref / update-side row ref の concrete dual ref shape までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual bless-side row ref / update-side row ref family comparison を切り、current first choice は `retained_payload_body_materialization_bless_update_row_ref_family_ref` だけを足す retained-payload-bless-update-row-ref-family-ready retained bridge である
- bless-side row ref / update-side row ref の concrete dual ref shape は still 後段に残す
- next later reopen は actual bless-side row ref / update-side row dual-ref comparison である

### not decided

- bless-side row ref と update-side row ref の concrete dual-ref shape をどう切るか
- dual ref を named pair bundle に戻すか strict dual field にするか
- typed symbolic ref family を boundary-specific handoff artifact へ昇格させる concrete pressure を何とみなすか
