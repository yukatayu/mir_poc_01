# 197 — current L2 theorem line boundary-specific-handoff-artifact-row-ready external-contract-facing-handoff-row threshold

## 目的

`specs/examples/196-current-l2-theorem-line-actual-handoff-pair-shape-ready-boundary-specific-handoff-artifact-row-threshold.md`
までを前提に、

- boundary-specific-handoff-artifact-row-ready retained bridge に external-contract-facing handoff row をどこまで近づけるか
- external-contract-facing row を theorem-side retained bridge では minimal row に留めるべきか
- actual external contract まで同時に持ち込むべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の boundary-specific-handoff-artifact-row-ready external-contract-facing-handoff-row threshold** であり、
actual external contract はまだ固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current boundary-specific-handoff-artifact-row-ready retained bridge を起点にする。
- actual external contract は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_boundary_handoff_artifact_row` までは current first choice に入っている。
2. external-contract-facing handoff row は still 後段に残している。

したがって current 問いは、
**boundary-specific handoff artifact row の次段として external-contract-facing handoff row を current bridge に近づけるなら、minimal external-facing row を first cut にするべきか、それとも actual external contract まで同時に切るべきか**
である。

## 比較観点

1. boundary-specific handoff artifact row と external-contract-facing handoff row の line を narrow に切れるか
2. external-facing row を current bridge では minimal row に留め、actual external contract を still 後段に残せるか
3. theorem-side retained bridge に actual external contract を premature に押し込まないか
4. next later reopen を actual external contract comparison へ狭く進められるか

## 比較対象

### 案 1. boundary-specific handoff artifact row を terminal cut にし、external-facing row も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- external-facing row の premature actualization を避けられる

#### 欠点

- external-facing row comparison の結果が prose 依存に残りやすい
- theorem-side retained bridge で external-facing row の最小 discipline が見えない

### 案 2. minimal external-contract-facing handoff row だけを持つ retained bridge にする

#### 読み

boundary-specific-handoff-artifact-row-ready retained bridge に、
actual external contract を導入せず
**`retained_payload_body_materialization_external_handoff_row`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch,
  retained_payload_body_materialization_external_handoff_row = {
    boundary = retained_payload_body_materialization_boundary_handoff_artifact_row.boundary,
    artifact_row_ref = retained_payload_body_materialization_boundary_handoff_artifact_row
  }
}
```

#### 利点

- external-facing handoff row 自体は current bridge で見える
- actual external contract を still 後段に残せる
- next later reopen を actual external contract comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- external-facing row と actual external contract を誤読されない説明が要る

### 案 3. actual external contract を current bridge へ同時に入れる

#### 利点

- actual external consumer が必要とする shape に一気に近づく
- later reopen の一部を早めに片付けられる

#### 欠点

- external-facing row と actual external contract を同じ reopen で混ぜやすい
- theorem-line retained bridge に actual external contract を premature に押し込みやすい
- actual external contract pressure の line が曖昧になる

## current judgment

current L2 で最も自然なのは、
**案 2. minimal external-contract-facing handoff row だけを持つ retained bridge にする**
である。

理由は次の通り。

1. boundary-specific handoff artifact row の次段として external-facing row 自体は narrow に橋渡しできる
2. actual external contract を still 後段に残せる
3. next later reopen を actual external contract comparison へ狭く進めやすい

## minimal external-contract-facing-handoff-row-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch,
  retained_payload_body_materialization_external_handoff_row = {
    boundary = retained_payload_body_materialization_boundary_handoff_artifact_row.boundary,
    artifact_row_ref = retained_payload_body_materialization_boundary_handoff_artifact_row
  }
}
```

### `retained_payload_body_materialization_external_handoff_row`

`retained_payload_body_materialization_external_handoff_row` は、
external-facing handoff row を theorem-side retained bridge で最小限に表す field である。

current task では、この row を actual external contract には昇格させない。

## なぜ actual external contract をまだ入れないか

actual external contract を current phase で入れると、

- boundary-specific handoff artifact row
- external-facing handoff row
- actual external contract

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず external-facing handoff row 自体を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain external-contract-facing-handoff-row-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch = bridge_sketch:e12.retained_payload_boundary_handoff_artifact_row_ready,
  retained_payload_body_materialization_external_handoff_row = {
    boundary = theorem_prover,
    artifact_row_ref = retained_payload_boundary_handoff_artifact_row:e12.latest
  }
}
```

ここで notebook bridge が知るのは external-facing row までであり、
actual external contract まではまだ bridge に入れない。

### example B — witnessed draw external-contract-facing-handoff-row-ready retained bridge

```text
proof_notebook_bridge_retained_payload_external_handoff_row_ready_sketch = {
  proof_notebook_bridge_retained_payload_boundary_handoff_artifact_row_ready_sketch = bridge_sketch:sugoroku.draw.retained_payload_boundary_handoff_artifact_row_ready,
  retained_payload_body_materialization_external_handoff_row = {
    boundary = protocol_verifier,
    artifact_row_ref = retained_payload_boundary_handoff_artifact_row:sugoroku.draw.latest
  }
}
```

draw external-facing row は notebook bridge で追えるが、
actual external contract までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で external-facing handoff row comparison を切り、current first choice は `retained_payload_body_materialization_external_handoff_row` だけを足す retained bridge である
- actual external contract は still 後段に残す
- next later reopen は external-contract-facing-handoff-row-ready actual external contract comparison である

### not decided

- actual external contract をどの field で切るか
- external-facing handoff row を retained bridge のまま維持するか actual external contract へ actualize するか
- actual external contract へ actualize する concrete pressure を何とみなすか
