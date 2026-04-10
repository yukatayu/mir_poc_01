# 193 — current L2 theorem line retained-payload-bless-update-strict-dual-field-ready consumer-visible-pair threshold

## 目的

`specs/examples/192-current-l2-theorem-line-retained-payload-bless-update-dual-ref-bundle-ready-retained-payload-bless-update-strict-dual-field-threshold.md`
までを前提に、

- strict dual field の次段として pair surface をどこまで current bridge に足してよいか
- pair surface を internal notebook consumer にだけ見せるべきか
- boundary-specific handoff pair まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-bless-update-strict-dual-field-ready consumer-visible-pair threshold** であり、
boundary-specific handoff pair はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-bless-update-strict-dual-field-ready retained bridge を起点にする。
- boundary-specific handoff pair と actual external contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. strict dual field 自体までは current first choice に入っている。
2. pair surface は still 後段に残している。

したがって current 問いは、
**strict dual field の次段として consumer-visible pair を current bridge に近づけるなら、internal notebook consumer pair を first cut にするべきか、それとも boundary-specific handoff pair まで同時に切るべきか**
である。

## 比較観点

1. strict dual field と pair surface の line を narrow に切れるか
2. pair surface を current bridge では internal notebook consumer pair に留め、boundary-specific handoff pair を still 後段に残せるか
3. theorem-side retained bridge に external-contract-facing pair shape を premature に押し込まないか
4. next later reopen を boundary-specific handoff pair comparison へ狭く進められるか

## 比較対象

### 案 1. strict dual field を terminal cut にし、pair surface も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- consumer-visible pair の premature actualization を避けられる

#### 欠点

- pair surface 比較の結果が prose 依存に残りやすい
- notebook consumer が pair 単位で読む current最小 shape が bridge に現れない

### 案 2. internal notebook consumer pair だけを持つ consumer-visible-pair-ready retained bridge にする

#### 読み

strict dual-field-ready retained bridge に、
boundary-specific handoff pair を導入せず
**`retained_payload_body_materialization_bless_update_pair`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch,
  retained_payload_body_materialization_bless_update_pair = {
    bless_side_row_ref = retained_payload_body_materialization_bless_side_row_ref,
    update_side_row_ref = retained_payload_body_materialization_update_side_row_ref
  }
}
```

#### 利点

- strict dual field の次段として pair surface 自体は current bridge で見える
- boundary-specific handoff pair を still 後段に残せる
- next later reopen を handoff pair comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- notebook-internal pair と external-facing pair を誤読されない説明が要る

### 案 3. boundary-specific handoff pair を current bridge へ同時に入れる

#### 利点

- notebook consumer と handoff consumer が同じ pair surface を一気に追える
- later reopen の一部を早めに片付けられる

#### 欠点

- notebook-internal pair と handoff-facing pair を同じ reopen で混ぜやすい
- theorem-line retained bridge に external contract specific shape を premature に押し込みやすい
- pair surface と actual external contract pressure の line が曖昧になる

## current judgment

current L2 で最も自然なのは、
**案 2. internal notebook consumer pair だけを持つ consumer-visible-pair-ready retained bridge にする**
である。

理由は次の通り。

1. strict dual field の次段として pair surface 自体は narrow に橋渡しできる
2. boundary-specific handoff pair を still 後段に残せる
3. next later reopen を handoff pair comparison へ狭く進めやすい

## minimal consumer-visible-pair-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch,
  retained_payload_body_materialization_bless_update_pair = {
    bless_side_row_ref = retained_payload_body_materialization_bless_side_row_ref,
    update_side_row_ref = retained_payload_body_materialization_update_side_row_ref
  }
}
```

### `retained_payload_body_materialization_bless_update_pair`

`retained_payload_body_materialization_bless_update_pair` は、
strict dual field を notebook consumer が pair 単位で読むための
consumer-visible pair field である。

current task では、この pair を boundary-specific handoff pair には昇格させない。

## なぜ handoff pair をまだ入れないか

boundary-specific handoff pair を current phase で入れると、

- notebook consumer pair
- handoff-facing pair
- actual external contract

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず notebook consumer が pair surface を読む current最小 shape を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain consumer-visible-pair-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch = bridge_sketch:e12.retained_payload_bless_update_strict_dual_field_ready,
  retained_payload_body_materialization_bless_update_pair = {
    bless_side_row_ref = retained_payload_bless_side_row:e12.latest,
    update_side_row_ref = retained_payload_update_side_row:e12.latest
  }
}
```

ここで notebook bridge が知るのは notebook consumer 用の pair surface までであり、
handoff-facing pair まではまだ bridge に入れない。

### example B — witnessed draw consumer-visible-pair-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_bless_update_strict_dual_field_ready,
  retained_payload_body_materialization_bless_update_pair = {
    bless_side_row_ref = retained_payload_bless_side_row:sugoroku.draw.latest,
    update_side_row_ref = retained_payload_update_side_row:sugoroku.draw.latest
  }
}
```

draw pair surface までは notebook bridge で追えるが、
handoff-facing pair までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で consumer-visible pair comparison を切り、current first choice は internal notebook consumer pair だけを足す retained bridge である
- boundary-specific handoff pair は still 後段に残す
- next later reopen は consumer-visible-pair-ready boundary-specific handoff pair comparison である

### not decided

- notebook consumer pair を handoff-facing pair にどう昇格させるか
- pair surface を symbolic retained bridge のまま維持するか、boundary-specific artifact row へ actualize するか
- pair surface を actual external contract へ actualize する concrete pressure を何とみなすか
