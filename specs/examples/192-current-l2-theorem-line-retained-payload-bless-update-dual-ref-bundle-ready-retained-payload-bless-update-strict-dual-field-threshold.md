# 192 — current L2 theorem line retained-payload-bless-update-dual-ref-bundle-ready retained-payload-bless-update-strict-dual-field threshold

## 目的

`specs/examples/191-current-l2-theorem-line-retained-payload-bless-update-row-ref-family-ready-retained-payload-bless-update-dual-ref-bundle-threshold.md`
までを前提に、

- retained-payload-bless-update-dual-ref-bundle-ready retained bridge に strict dual field をどこまで近づけるか
- strict dual field を current bridge では internal-only split に留めるか
- consumer-visible pair まで current bridge に持ち込むか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-bless-update-dual-ref-bundle-ready retained-payload-bless-update-strict-dual-field threshold** であり、
consumer-visible pair はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-bless-update-dual-ref-bundle-ready retained bridge を起点にする。
- consumer-visible pair と boundary-specific handoff artifact は巻き込まない。

## current 前提

current repo では次が成立している。

1. concrete dual-ref bundle に戻る symbolic ref までは current first choice に入っている。
2. bless-side row ref / update-side row ref の strict dual-field shape は still 後段に残している。

したがって current 問いは、
**dual-ref bundle ref の次段として strict dual field 自体を current bridge に近づけるなら、internal-only split を first cut にするべきか、それとも consumer-visible pair まで同時に切るべきか**
である。

## 比較観点

1. dual-ref bundle と strict dual field の line を narrow に切れるか
2. strict dual field を current bridge では internal-only split に留め、consumer-visible pair を still 後段に残せるか
3. theorem-side retained bridge に consumer-facing field shape を premature に押し込まないか
4. next later reopen を consumer-visible pair comparison へ狭く進められるか

## 比較対象

### 案 1. dual-ref bundle ref を terminal cut にし、strict dual field も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- concrete strict dual field の premature actualization を避けられる

#### 欠点

- strict dual-field comparison の結果が prose 依存に残りやすい
- bless-side row ref / update-side row ref の最小 field discipline が bridge に現れない

### 案 2. internal-only strict dual field だけを持つ retained-payload-bless-update-strict-dual-field-ready retained bridge にする

#### 読み

retained-payload-bless-update-dual-ref-bundle-ready retained bridge に、
consumer-visible pair を導入せず
**`retained_payload_body_materialization_bless_side_row_ref`** と
**`retained_payload_body_materialization_update_side_row_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch,
  retained_payload_body_materialization_bless_side_row_ref,
  retained_payload_body_materialization_update_side_row_ref
}
```

#### 利点

- strict dual field 自体は current bridge で見える
- consumer-visible pair を still 後段に残せる
- next later reopen を consumer-visible pair comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 2 本増える
- consumer が見てよい pair shape だと誤読されない説明が要る

### 案 3. consumer-visible pair を current bridge へ同時に入れる

#### 利点

- notebook consumer が pair surface まで一気に追える
- later reopen の一部を早めに片付けられる

#### 欠点

- strict dual field と consumer-facing pair を同じ reopen で混ぜやすい
- internal-only split と consumer-visible shape の境界が曖昧になる
- theorem-line retained bridge に consumer-specific surface を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. internal-only strict dual field だけを持つ retained-payload-bless-update-strict-dual-field-ready retained bridge にする**
である。

理由は次の通り。

1. dual-ref bundle の次段として strict dual field 自体は narrow に橋渡しできる
2. consumer-visible pair を still 後段に残せる
3. next later reopen を pair comparison へ狭く進めやすい

## minimal retained-payload-bless-update-strict-dual-field-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch,
  retained_payload_body_materialization_bless_side_row_ref,
  retained_payload_body_materialization_update_side_row_ref
}
```

### `retained_payload_body_materialization_bless_side_row_ref`

`retained_payload_body_materialization_bless_side_row_ref` は、
retained payload body materialization の bless-side row を指す
internal theorem-side field である。

### `retained_payload_body_materialization_update_side_row_ref`

`retained_payload_body_materialization_update_side_row_ref` は、
retained payload body materialization の update-side row を指す
internal theorem-side field である。

current task では、この 2 field を consumer-visible pair には昇格させない。

## なぜ consumer-visible pair をまだ入れないか

consumer-visible pair を current phase で入れると、

- strict dual field
- pair surface
- boundary-specific handoff artifact

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず strict dual field 自体を internal-only split として stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-bless-update-strict-dual-field-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch = bridge_sketch:e12.retained_payload_bless_update_dual_ref_bundle_ready,
  retained_payload_body_materialization_bless_side_row_ref = retained_payload_bless_side_row:e12.latest,
  retained_payload_body_materialization_update_side_row_ref = retained_payload_update_side_row:e12.latest
}
```

ここで notebook bridge が知るのは strict dual field までであり、
consumer-visible pair surface まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-bless-update-strict-dual-field-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_strict_dual_field_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_bless_update_dual_ref_bundle_ready,
  retained_payload_body_materialization_bless_side_row_ref = retained_payload_bless_side_row:sugoroku.draw.latest,
  retained_payload_body_materialization_update_side_row_ref = retained_payload_update_side_row:sugoroku.draw.latest
}
```

draw strict dual field までは notebook bridge で追えるが、
consumer-visible pair までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で strict dual-field comparison を切り、current first choice は internal-only strict dual field だけを足す retained-payload-bless-update-strict-dual-field-ready retained bridge である
- consumer-visible pair は still 後段に残す
- next later reopen は retained-payload-bless-update strict dual-field-ready consumer-visible pair comparison である

### not decided

- bless-side row ref と update-side row ref を consumer-visible pair にどう束ねるか
- pair shape を internal notebook consumer にだけ見せるか、boundary-specific handoff artifact まで上げるか
- strict dual field / pair surface を actual external contract へ actualize する concrete pressure を何とみなすか
