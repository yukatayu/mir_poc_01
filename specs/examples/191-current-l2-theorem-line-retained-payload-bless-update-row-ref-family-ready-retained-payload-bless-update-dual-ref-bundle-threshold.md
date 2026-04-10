# 191 — current L2 theorem line retained-payload-bless-update-row-ref-family-ready retained-payload-bless-update-dual-ref-bundle threshold

## 目的

`specs/examples/190-current-l2-theorem-line-retained-payload-bless-update-row-pair-ready-retained-payload-bless-update-row-ref-family-threshold.md`
までを前提に、

- retained-payload-bless-update-row-ref-family-ready retained bridge に concrete dual-ref shape をどこまで足すか
- strict dual field をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retained-payload-bless-update-row-ref-family-ready retained-payload-bless-update-dual-ref-bundle threshold** であり、

- actual bless-side row ref
- actual update-side row ref

の strict dual-field shape は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retained-payload-bless-update-row-ref-family-ready retained bridge を起点にする。
- actual bless-side row ref / update-side row ref の strict dual-field shape は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual bless/update row ref family までは current first choice に入っている。
2. bless-side row ref / update-side row ref の concrete dual-ref shape は still 後段に残している。

したがって current 問いは、
**retained-payload-bless-update-row-ref-family-ready retained bridge に `retained_payload_body_materialization_bless_update_dual_ref_bundle_ref` だけを先に足してよいか、それとも strict dual field まで同時に bridge に入れるべきか**
である。

## 比較観点

1. row-ref family と concrete dual-ref shape の line を narrow に切れるか
2. dual-ref shape を named bundle ref 1 本に留め、strict dual field を still 後段に残せるか
3. bless-side row ref / update-side row ref を theorem-line bridge に premature に混ぜないか
4. next later reopen を strict dual-field comparison へ narrow に進めるか

## 比較対象

### 案 1. retained-payload-bless-update-row-ref-family-ready retained bridge を terminal cut にし、concrete dual-ref shape も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- concrete dual-ref shape の premature actualization をさらに避けられる

#### 欠点

- row-ref family の次段が prose 依存に残りやすい
- theorem-line bridge の concrete dual-ref return path が弱い

### 案 2. dual-ref bundle ref だけを持つ retained-payload-bless-update-dual-ref-bundle-ready retained bridge にする

#### 読み

retained-payload-bless-update-row-ref-family-ready retained bridge に、
strict dual field 自体を入れずに
**`retained_payload_body_materialization_bless_update_dual_ref_bundle_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch,
  retained_payload_body_materialization_bless_update_dual_ref_bundle_ref
}
```

#### 利点

- row-ref family と concrete dual-ref shape の line を narrow に橋渡しできる
- strict dual field を still 後段に残せる
- next later reopen を actual bless-side row ref / update-side row strict dual-field comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 つ増える
- named bundle ref と strict dual field を誤読されない説明が要る

### 案 3. bless-side row ref / update-side row ref の strict dual field を current phase で同時に入れる

#### 利点

- notebook consumer が concrete dual-ref shape まで一気に追える
- strict dual-field specific pressure に早く近づける

#### 欠点

- dual-ref bundle と strict dual field を同じ reopen で混ぜやすい
- narrow reopen を stage ごとに切りにくい
- theorem-line bridge に consumer-specific field shape を premature に押し込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. dual-ref bundle ref だけを持つ retained-payload-bless-update-dual-ref-bundle-ready retained bridge にする**
である。

理由は次の通り。

1. row-ref family の次段を symbolic bundle ref で narrow に橋渡しできる
2. strict dual field を still 後段に残せる
3. next later reopen を actual bless-side row ref / update-side row strict dual-field comparison へ狭く進めやすい

## minimal retained-payload-bless-update-dual-ref-bundle-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch,
  retained_payload_body_materialization_bless_update_dual_ref_bundle_ref
}
```

### `retained_payload_body_materialization_bless_update_dual_ref_bundle_ref`

`retained_payload_body_materialization_bless_update_dual_ref_bundle_ref` は、

- actual bless/update row ref family の次段で
- notebook consumer が **actual bless-side row ref / update-side row ref の concrete dual-ref bundle** に戻れる
  symbolic dual-ref-bundle ref

だけを置く field である。

current task では、

- bless-side row ref
- update-side row ref

の strict dual field 自体をここに入れない。

## なぜ strict dual field をまだ入れないか

strict dual field を current phase で入れると、

- actual bless/update row ref family
- concrete dual-ref bundle
- strict dual field

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず concrete dual-ref bundle へ戻る symbolic ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain retained-payload-bless-update-dual-ref-bundle-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch = bridge_sketch:e12.retained_payload_bless_update_row_ref_family_ready,
  retained_payload_body_materialization_bless_update_dual_ref_bundle_ref = retained_payload_bless_update_dual_ref_bundle:e12.latest
}
```

ここで notebook bridge が知るのは
concrete dual-ref bundle に戻る ref までであり、
bless-side row ref / update-side row ref の strict dual field まではまだ bridge に入れない。

### example B — witnessed draw retained-payload-bless-update-dual-ref-bundle-ready retained bridge

```text
proof_notebook_bridge_retained_payload_bless_update_dual_ref_bundle_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_row_ref_family_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_bless_update_row_ref_family_ready,
  retained_payload_body_materialization_bless_update_dual_ref_bundle_ref = retained_payload_bless_update_dual_ref_bundle:sugoroku.draw.latest
}
```

draw concrete dual-ref bundle へ戻る ref は notebook bridge で追えるが、
bless-side row ref / update-side row ref の strict dual field までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で concrete dual-ref shape comparison を切り、current first choice は `retained_payload_body_materialization_bless_update_dual_ref_bundle_ref` だけを足す retained-payload-bless-update-dual-ref-bundle-ready retained bridge である
- bless-side row ref / update-side row ref の strict dual-field shape は still 後段に残す
- next later reopen は actual bless-side row ref / update-side row strict dual-field comparison である

### not decided

- bless-side row ref と update-side row ref の strict dual-field shape をどう切るか
- strict dual field を consumer-visible pair にするか internal-only split にするか
- concrete dual-ref bundle を boundary-specific handoff artifact へ昇格させる concrete pressure を何とみなすか
