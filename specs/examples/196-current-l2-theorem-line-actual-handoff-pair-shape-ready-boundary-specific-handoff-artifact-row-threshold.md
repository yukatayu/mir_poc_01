# 196 — current L2 theorem line actual-handoff-pair-shape-ready boundary-specific-handoff-artifact-row threshold

## 目的

`specs/examples/195-current-l2-theorem-line-boundary-specific-handoff-pair-ready-actual-handoff-pair-shape-threshold.md`
までを前提に、

- actual-handoff-pair-shape-ready retained bridge に boundary-specific handoff artifact row をどこまで近づけるか
- artifact row を theorem-side retained bridge では minimal row に留めるべきか
- external-contract-facing handoff row まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の actual-handoff-pair-shape-ready boundary-specific-handoff-artifact-row threshold** であり、
external-contract-facing handoff row はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current actual-handoff-pair-shape-ready retained bridge を起点にする。
- external-contract-facing handoff row と actual external contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. actual `retained_payload_body_materialization_boundary_handoff_pair` までは current first choice に入っている。
2. boundary-specific handoff artifact row は still 後段に残している。

したがって current 問いは、
**actual handoff pair shape の次段として boundary-specific handoff artifact row を current bridge に近づけるなら、minimal artifact row を first cut にするべきか、それとも external-contract-facing handoff row まで同時に切るべきか**
である。

## 比較観点

1. actual handoff pair shape と boundary-specific handoff artifact row の line を narrow に切れるか
2. artifact row を current bridge では minimal row に留め、external-contract-facing handoff row を still 後段に残せるか
3. theorem-side retained bridge に external-contract-facing row を premature に押し込まないか
4. next later reopen を external-contract-facing handoff row comparison へ狭く進められるか

## 比較対象

### 案 1. actual handoff pair shape を terminal cut にし、artifact row も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- artifact row の premature actualization を避けられる

#### 欠点

- artifact row comparison の結果が prose 依存に残りやすい
- theorem-side retained bridge で boundary-specific row の最小 discipline が見えない

### 案 2. minimal boundary-specific handoff artifact row だけを持つ retained bridge にする

#### 読み

actual-handoff-pair-shape-ready retained bridge に、
external-contract-facing handoff row を導入せず
**`retained_payload_body_materialization_boundary_handoff_artifact_row`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch,
  retained_payload_body_materialization_boundary_handoff_artifact_row = {
    boundary = retained_payload_body_materialization_boundary_handoff_pair.boundary,
    pair_ref = retained_payload_body_materialization_boundary_handoff_pair
  }
}
```

#### 利点

- boundary-specific handoff artifact row 自体は current bridge で見える
- external-contract-facing handoff row を still 後段に残せる
- next later reopen を external-contract-facing handoff row comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- artifact row と external-contract-facing row を誤読されない説明が要る

### 案 3. external-contract-facing handoff row を current bridge へ同時に入れる

#### 利点

- external consumer が必要とする row に一気に近づく
- later reopen の一部を早めに片付けられる

#### 欠点

- boundary-specific artifact row と external-contract-facing row を同じ reopen で混ぜやすい
- theorem-line retained bridge に actual external contract を premature に押し込みやすい
- external contract pressure の line が曖昧になる

## current judgment

current L2 で最も自然なのは、
**案 2. minimal boundary-specific handoff artifact row だけを持つ retained bridge にする**
である。

理由は次の通り。

1. actual handoff pair shape の次段として artifact row 自体は narrow に橋渡しできる
2. external-contract-facing handoff row を still 後段に残せる
3. next later reopen を external-contract-facing handoff row comparison へ狭く進めやすい

## minimal boundary-specific-handoff-artifact-row-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch,
  retained_payload_body_materialization_boundary_handoff_artifact_row = {
    boundary = retained_payload_body_materialization_boundary_handoff_pair.boundary,
    pair_ref = retained_payload_body_materialization_boundary_handoff_pair
  }
}
```

### `retained_payload_body_materialization_boundary_handoff_artifact_row`

`retained_payload_body_materialization_boundary_handoff_artifact_row` は、
boundary-specific handoff consumer 向け artifact row を
theorem-side retained bridge で最小限に表す field である。

current task では、この row を external-contract-facing handoff row には昇格させない。

## なぜ external-contract-facing handoff row をまだ入れないか

external-contract-facing handoff row を current phase で入れると、

- actual handoff pair shape
- boundary-specific handoff artifact row
- external-contract-facing handoff row
- actual external contract

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず boundary-specific handoff artifact row 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain boundary-specific-handoff-artifact-row-ready retained bridge

```text
proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch = bridge_sketch:e12.retained_payload_actual_boundary_handoff_pair_ready,
  retained_payload_body_materialization_boundary_handoff_artifact_row = {
    boundary = theorem_prover,
    pair_ref = retained_payload_boundary_handoff_pair:e12.latest
  }
}
```

ここで notebook bridge が知るのは boundary-specific artifact row までであり、
external-contract-facing row まではまだ bridge に入れない。

### example B — witnessed draw boundary-specific-handoff-artifact-row-ready retained bridge

```text
proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_actual_boundary_handoff_pair_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_actual_boundary_handoff_pair_ready,
  retained_payload_body_materialization_boundary_handoff_artifact_row = {
    boundary = protocol_verifier,
    pair_ref = retained_payload_boundary_handoff_pair:sugoroku.draw.latest
  }
}
```

draw artifact row は notebook bridge で追えるが、
external-contract-facing row までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で boundary-specific handoff artifact row comparison を切り、current first choice は `retained_payload_body_materialization_boundary_handoff_artifact_row` だけを足す retained bridge である
- external-contract-facing handoff row は still 後段に残す
- next later reopen は boundary-specific-handoff-artifact-row-ready external-contract-facing handoff row comparison である

### not decided

- external-contract-facing handoff row をどの field で切るか
- boundary-specific handoff artifact row を retained bridge のまま維持するか external contract row へ actualize するか
- actual external contract へ actualize する concrete pressure を何とみなすか
