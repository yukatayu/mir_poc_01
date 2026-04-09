# 149 — current L2 theorem line retention-ready path-policy threshold

## 目的

`specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md`
と
`specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md`
までを前提に、

- retention-ready retained bridge に actual retained path policy をどこまで足すか
- emitted artifact pressure をどこまで後段に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の retention-ready path-policy threshold** であり、

- actual emitted notebook artifact
- `proof_assistant_adapter` specific schema

は固定しない。

## scope

- `proof_notebook` first bridge だけを扱う。
- current retention-ready retained bridge を起点にする。
- detached validation loop の retained path policy は巻き込まない。

## current 前提

current repo では次が成立している。

1. compare basis refs / bless decision state / review-note refs / retained-notebook ref / review-session ref までは current first choice に入っている。
2. review actor / timestamp も observed-session bridge までは足してよい。
3. review session lifecycle state も lifecycle-ready bridge までは足してよい。
4. retention state も retention-ready bridge までは足してよい。
5. actual retained path policy / emitted artifact pressure は still 後段に残している。

したがって current 問いは、
**retention-ready retained bridge に retained path policy だけを先に足してよいか、それとも concrete path handling も still bridge 外へ残すべきか**
である。

## 比較観点

1. current retention-ready retained bridge の最小性を壊さないか
2. retention progression と retained path policy を、emitted artifact pressure から分離できるか
3. detached validation loop の retained path candidate を premature に actual emitter line へ混ぜないか
4. later reopen で emitted artifact threshold へ narrow に進めるか
5. actual file / blob management を theorem-line bridge に誤昇格しないか

## 比較対象

### 案 1. retention-ready retained bridge を terminal cut にし、path policy も bridge 外へ残す

#### 読み

bridge sketch には

- `review_session_state`
- `retention_state`

までを置き、`retained_path_policy_ref` は external note / prose に残す。

#### 利点

- current bridge を最も動かさない
- emitted artifact pressure と混ざる余地をさらに減らせる

#### 欠点

- retention progression と path policy の対応が prose 依存に残りやすい
- next later reopen の出発点が弱い

### 案 2. retained path policy ref だけを持つ path-ready retained bridge にする

#### 読み

retention-ready retained bridge に、
actual emitted artifact を入れずに
**`retained_path_policy_ref`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_path_ready_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref,
  reviewed_by_ref,
  reviewed_at_ref,
  review_session_state,
  retention_state,
  retained_path_policy_ref
}
```

#### 利点

- retention progression と path policy anchor を narrow に橋渡しできる
- actual emitted artifact を still 後段に残せる
- next later reopen を emitted-artifact threshold へ狭く進めやすい
- path policy 自体は symbolic ref に留められる

#### 欠点

- bridge-level field が 1 つ増える
- `retained_path_policy_ref` を actual path string / emitter id と誤読されない説明が要る

### 案 3. path policy と emitted artifact をまとめて入れる

#### 読み

bridge sketch に

- `retained_path_policy_ref`
- `emitted_artifact_ref`

をまとめて足し、retained notebook handoff bundle に近い shape へ進める。

#### 利点

- path policy と actual handoff artifact を一度に語れる
- concrete notebook exchange に近づく

#### 欠点

- actual emitted notebook artifact pressure を premature に混ぜやすい
- theorem-line bridge の docs-first discipline と still 距離がある

## current judgment

current L2 で最も自然なのは、
**案 2. retained path policy ref だけを持つ path-ready retained bridge にする**
である。

理由は次の通り。

1. retention progression と path policy anchor を narrow に接続できる
2. actual emitted notebook artifact を still 後段に残せる
3. `proof_notebook` first bridge の current docs-first discipline を壊さない
4. next later reopen を emitted-artifact threshold へ狭く進めやすい

## minimal path-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_path_ready_sketch = {
  bridge_subject_ref,
  review_units,
  bridge_goal_text,
  comparison_basis_refs,
  bless_decision_state,
  review_note_refs,
  retained_notebook_ref,
  review_session_ref,
  reviewed_by_ref,
  reviewed_at_ref,
  review_session_state,
  retention_state,
  retained_path_policy_ref
}
```

### `retained_path_policy_ref`

`retained_path_policy_ref` は、

- retained notebook linkage に対する concrete path handling rule を指す
  **symbolic policy ref**

だけを置く field である。

current task では、

- actual retained path string / URI
- emitted notebook artifact id
- actual file exchange rule

をここに入れない。

## なぜ emitted artifact をまだ入れないか

actual emitted artifact を current phase で入れると、

- notebook export / handoff file exchange
- emitted artifact id / blob policy
- concrete consumer adapter surface

が theorem-line bridge に premature に混ざりやすい。

current pressure はまず retained path policy ref を stable に切るところまでで十分である。

## practical examples

### example A — fallback chain path-ready retained bridge

```text
proof_notebook_bridge_path_ready_sketch = {
  bridge_subject_ref = fallback_cluster:e12,
  review_units = [
    review_unit:e12.canonical_normalization,
    review_unit:e12.no_repromotion_boundary
  ],
  bridge_goal_text = "fallback chain の normalization family を walkthrough する",
  comparison_basis_refs = [
    bridge_sketch:e12.previous
  ],
  bless_decision_state = accepted,
  review_note_refs = [
    review_note:e12.normalization.observation
  ],
  retained_notebook_ref = retained_notebook:e12.latest,
  review_session_ref = review_session:e12.walkthrough,
  reviewed_by_ref = actor:maintainer_1,
  reviewed_at_ref = timestamp:2026-04-09T23:59,
  review_session_state = blessed,
  retention_state = retained,
  retained_path_policy_ref = retained_path_policy:phase5.default
}
```

### example B — `try` / `atomic_cut` path-ready retained bridge

```text
proof_notebook_bridge_path_ready_sketch = {
  bridge_subject_ref = runtime_cluster:e6,
  review_units = [
    review_unit:e6.rollback_cut_non_interference,
    review_unit:e6.rollback_frontier_reading
  ],
  bridge_goal_text = "rollback family を side-by-side で見直す",
  comparison_basis_refs = [
    runtime_cluster:e6,
    static_gate:e6
  ],
  bless_decision_state = revise_requested,
  review_note_refs = [
    review_note:e6.rollback.followup
  ],
  retained_notebook_ref = retained_notebook:e6.followup,
  review_session_ref = review_session:e6.rollback_walkthrough,
  reviewed_by_ref = actor:maintainer_2,
  reviewed_at_ref = timestamp:2026-04-10T00:05,
  review_session_state = followup_pending,
  retention_state = provisional,
  retained_path_policy_ref = retained_path_policy:phase5.followup
}
```

## current docs-only cut

current task で fixed にしてよいのは次である。

1. retention-ready retained bridge の次段では、`retained_path_policy_ref` までは足してよい
2. `retained_path_policy_ref` は symbolic policy ref に留め、actual path string / emitter id / file exchange rule を actual contract にしない
3. actual emitted notebook artifact は still 後段に残す
4. next later reopen は、emitted notebook artifact threshold の comparison に置く

## この段階でまだ固定しないもの

- `emitted_artifact_ref`
- actual retained path string / URI
- concrete notebook file exchange
- `proof_assistant_adapter` specific schema
