# 195 — current L2 theorem line boundary-specific-handoff-pair-ready actual handoff pair shape threshold

## 目的

`specs/examples/194-current-l2-theorem-line-consumer-visible-pair-ready-boundary-specific-handoff-pair-threshold.md`
までを前提に、

- boundary-specific-handoff-pair-ready retained bridge に actual handoff pair shape をどこまで近づけるか
- actual handoff pair shape を theorem-side retained bridge では minimal shape に留めるべきか
- boundary-specific artifact row まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の boundary-specific-handoff-pair-ready actual handoff pair shape threshold** であり、
boundary-specific handoff artifact row はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current boundary-specific-handoff-pair-ready retained bridge を起点にする。
- boundary-specific handoff artifact row と actual external contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. symbolic `retained_payload_body_materialization_boundary_handoff_pair_ref` までは current first choice に入っている。
2. actual boundary-specific handoff pair shape は still 後段に残している。

したがって current 問いは、
**symbolic handoff pair ref の次段として actual handoff pair shape を current bridge に近づけるなら、minimal actual pair shape を first cut にするべきか、それとも boundary-specific artifact row まで同時に切るべきか**
である。

## 比較観点

1. symbolic handoff pair ref と actual handoff pair shape の line を narrow に切れるか
2. actual handoff pair shape を current bridge では minimal shape に留め、boundary-specific handoff artifact row を still 後段に残せるか
3. theorem-side retained bridge に boundary-specific artifact row を premature に押し込まないか
4. next later reopen を boundary-specific handoff artifact row comparison へ狭く進められるか

## 比較対象

### 案 1. symbolic handoff pair ref を terminal cut にし、actual handoff pair shape も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- actual handoff pair shape の premature actualization を避けられる

#### 欠点

- actual handoff pair shape comparison の結果が prose 依存に残りやすい
- theorem-side retained bridge で handoff-facing pair の最小 field discipline が見えない

### 案 2. minimal actual handoff pair shape だけを持つ actual-handoff-pair-shape-ready retained bridge にする

#### 読み

boundary-specific-handoff-pair-ready retained bridge に、
boundary-specific handoff artifact row を導入せず
**`retained_payload_body_materialization_boundary_handoff_pair`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch,
  retained_payload_body_materialization_boundary_handoff_pair = {
    boundary = theorem_prover,
    bless_side_row_ref = retained_payload_body_materialization_bless_side_row_ref,
    update_side_row_ref = retained_payload_body_materialization_update_side_row_ref
  }
}
```

#### 利点

- actual handoff pair shape 自体は current bridge で見える
- boundary-specific artifact row を still 後段に残せる
- next later reopen を boundary-specific handoff artifact row comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- actual handoff pair shape と boundary-specific artifact row を誤読されない説明が要る

### 案 3. boundary-specific handoff artifact row を current bridge へ同時に入れる

#### 利点

- handoff-facing consumer が artifact row に一気に近づく
- later reopen の一部を早めに片付けられる

#### 欠点

- actual handoff pair shape と artifact row を同じ reopen で混ぜやすい
- theorem-line retained bridge に boundary-specific row contract を premature に押し込みやすい
- actual external contract pressure の line が曖昧になる

## current judgment

current L2 で最も自然なのは、
**案 2. minimal actual handoff pair shape だけを持つ actual-handoff-pair-shape-ready retained bridge にする**
である。

理由は次の通り。

1. symbolic handoff pair ref の次段として actual handoff pair shape 自体は narrow に橋渡しできる
2. boundary-specific handoff artifact row を still 後段に残せる
3. next later reopen を artifact row comparison へ狭く進めやすい

## minimal actual-handoff-pair-shape-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch,
  retained_payload_body_materialization_boundary_handoff_pair = {
    boundary = theorem_prover,
    bless_side_row_ref = retained_payload_body_materialization_bless_side_row_ref,
    update_side_row_ref = retained_payload_body_materialization_update_side_row_ref
  }
}
```

### `retained_payload_body_materialization_boundary_handoff_pair`

`retained_payload_body_materialization_boundary_handoff_pair` は、
boundary-specific handoff consumer に向けた actual pair shape を
theorem-side retained bridge で最小限に表す field である。

current task では、この shape を boundary-specific artifact row には昇格させない。

## なぜ boundary-specific handoff artifact row をまだ入れないか

boundary-specific handoff artifact row を current phase で入れると、

- symbolic handoff pair ref
- actual handoff pair shape
- boundary-specific artifact row
- actual external contract

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず actual handoff pair shape 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain actual-handoff-pair-shape-ready retained bridge

```text
proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch = bridge_sketch:e12.retained_payload_boundary_handoff_pair_ready,
  retained_payload_body_materialization_boundary_handoff_pair = {
    boundary = theorem_prover,
    bless_side_row_ref = retained_payload_bless_side_row:e12.latest,
    update_side_row_ref = retained_payload_update_side_row:e12.latest
  }
}
```

ここで notebook bridge が知るのは actual handoff pair shape までであり、
boundary-specific artifact row まではまだ bridge に入れない。

### example B — witnessed draw actual-handoff-pair-shape-ready retained bridge

```text
proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_pair_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_boundary_handoff_pair_ready,
  retained_payload_body_materialization_boundary_handoff_pair = {
    boundary = protocol_verifier,
    bless_side_row_ref = retained_payload_bless_side_row:sugoroku.draw.latest,
    update_side_row_ref = retained_payload_update_side_row:sugoroku.draw.latest
  }
}
```

draw handoff pair の actual shape は notebook bridge で追えるが、
boundary-specific artifact row までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で actual handoff pair shape comparison を切り、current first choice は `retained_payload_body_materialization_boundary_handoff_pair` だけを足す actual-handoff-pair-shape-ready retained bridge である
- boundary-specific handoff artifact row は still 後段に残す
- next later reopen は actual-handoff-pair-shape-ready boundary-specific handoff artifact row comparison である

### not decided

- boundary-specific handoff artifact row をどの field で切るか
- actual handoff pair shape を retained bridge のまま維持するか artifact row へ actualize するか
- artifact row を actual external contract へ actualize する concrete pressure を何とみなすか
