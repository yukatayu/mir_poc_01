# 194 — current L2 theorem line consumer-visible-pair-ready boundary-specific-handoff-pair threshold

## 目的

`specs/examples/193-current-l2-theorem-line-retained-payload-bless-update-strict-dual-field-ready-consumer-visible-pair-threshold.md`
までを前提に、

- notebook consumer pair の次段として handoff-facing pair をどこまで current bridge に足してよいか
- handoff-facing pair を symbolic handoff pair ref に留めるべきか
- actual boundary-specific pair shape まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の consumer-visible-pair-ready boundary-specific-handoff-pair threshold** であり、
actual boundary-specific pair shape はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current consumer-visible-pair-ready retained bridge を起点にする。
- actual boundary-specific pair shape と actual external contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. notebook consumer 向け pair surface までは current first choice に入っている。
2. handoff-facing pair は still 後段に残している。

したがって current 問いは、
**consumer-visible pair の次段として handoff-facing pair を current bridge に近づけるなら、symbolic handoff pair ref を first cut にするべきか、それとも actual boundary-specific pair shape まで同時に切るべきか**
である。

## 比較観点

1. notebook consumer pair と handoff-facing pair の line を narrow に切れるか
2. handoff-facing pair を current bridge では symbolic ref に留め、actual boundary-specific pair shape を still 後段に残せるか
3. theorem-side retained bridge に external contract specific field shape を premature に押し込まないか
4. next later reopen を actual boundary-specific pair shape comparison へ狭く進められるか

## 比較対象

### 案 1. consumer-visible pair を terminal cut にし、handoff-facing pair も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- handoff-facing pair の premature actualization を避けられる

#### 欠点

- handoff-facing pair comparison の結果が prose 依存に残りやすい
- notebook consumer pair と handoff-facing pair の接続が bridge に現れない

### 案 2. symbolic handoff pair ref だけを持つ boundary-specific-handoff-pair-ready retained bridge にする

#### 読み

consumer-visible-pair-ready retained bridge に、
actual boundary-specific pair shape を導入せず
**`retained_payload_body_materialization_boundary_handoff_pair_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch,
  retained_payload_body_materialization_boundary_handoff_pair_ref
}
```

#### 利点

- handoff-facing pair の戻り先を theorem-side retained bridge に narrow に橋渡しできる
- actual boundary-specific pair shape を still 後段に残せる
- next later reopen を actual handoff pair shape comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- notebook consumer pair と handoff-facing pair ref を誤読されない説明が要る

### 案 3. actual boundary-specific pair shape を current bridge へ同時に入れる

#### 利点

- notebook consumer と handoff consumer が current bridge から pair shape を直接追える
- later reopen の一部を早めに片付けられる

#### 欠点

- symbolic handoff pair ref と actual pair shape を同じ reopen で混ぜやすい
- theorem-line retained bridge に external contract specific shape を premature に押し込みやすい
- handoff pair shape と actual external contract pressure の line が曖昧になる

## current judgment

current L2 で最も自然なのは、
**案 2. symbolic handoff pair ref だけを持つ boundary-specific-handoff-pair-ready retained bridge にする**
である。

理由は次の通り。

1. notebook consumer pair の次段として handoff-facing pair の戻り先を narrow に橋渡しできる
2. actual boundary-specific pair shape を still 後段に残せる
3. next later reopen を actual handoff pair shape comparison へ狭く進めやすい

## minimal boundary-specific-handoff-pair-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch,
  retained_payload_body_materialization_boundary_handoff_pair_ref
}
```

### `retained_payload_body_materialization_boundary_handoff_pair_ref`

`retained_payload_body_materialization_boundary_handoff_pair_ref` は、
boundary-specific handoff consumer が使う pair shape に戻るための
symbolic retained ref である。

current task では、この ref の先にある actual pair shape 自体は bridge に入れない。

## なぜ actual boundary-specific pair shape をまだ入れないか

actual boundary-specific pair shape を current phase で入れると、

- notebook consumer pair
- symbolic handoff pair ref
- actual handoff pair shape
- actual external contract

が theorem-line bridge の同じ reopen で混ざりやすい。

current pressure はまず handoff-facing pair の戻り先を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain boundary-specific-handoff-pair-ready retained bridge

```text
proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch = bridge_sketch:e12.retained_payload_bless_update_consumer_pair_ready,
  retained_payload_body_materialization_boundary_handoff_pair_ref = retained_payload_boundary_handoff_pair:e12.latest
}
```

ここで notebook bridge が知るのは handoff-facing pair に戻る ref までであり、
actual handoff pair shape まではまだ bridge に入れない。

### example B — witnessed draw boundary-specific-handoff-pair-ready retained bridge

```text
proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_bless_update_consumer_pair_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_bless_update_consumer_pair_ready,
  retained_payload_body_materialization_boundary_handoff_pair_ref = retained_payload_boundary_handoff_pair:sugoroku.draw.latest
}
```

draw handoff pair への戻り先は notebook bridge で追えるが、
actual handoff pair shape までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で boundary-specific handoff pair comparison を切り、current first choice は symbolic handoff pair ref だけを足す retained bridge である
- actual boundary-specific pair shape は still 後段に残す
- next later reopen は boundary-specific-handoff-pair-ready actual handoff pair shape comparison である

### not decided

- actual boundary-specific pair shape をどの field で切るか
- symbolic handoff pair ref を retained bridge のまま維持するか、boundary-specific artifact row へ actualize するか
- handoff pair shape を actual external contract へ actualize する concrete pressure を何とみなすか
