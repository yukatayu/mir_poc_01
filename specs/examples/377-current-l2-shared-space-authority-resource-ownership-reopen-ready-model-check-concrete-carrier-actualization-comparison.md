# 377 — current L2 shared-space-authority-resource-ownership-reopen-ready model-check-concrete-carrier-actualization comparison

## 目的

`specs/examples/376-current-l2-shared-space-authority-resource-ownership-reopen-ready-minimal-shared-space-authority-resource-ownership-reopen-threshold.md`
で shared-space authority / resource ownership reopen の minimum を fixed した次段として、

- model-check side の first actualization を、actual emitted carrier、source-sample emitted verification artifact wiring、sample-facing theorem/model-check evidence summary のどれから reopen するのが自然か
- `proof_notebook_review_unit` current first concrete pilot、tool-neutral formal hook、compare-ready docs-only bridge sketch、public-checker docs-only reserve chain をどの順で保つのが current first choice か
- concrete theorem/model-check tool binding、actual public-checker migration、actual emitted verifier handoff artifact、sample-facing bless/review session metadata をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 shared-space-authority-resource-ownership-reopen-ready model-check-concrete-carrier-actualization comparison**
であり、

- model-check concrete carrier first actualization
- source-sample emitted verification artifact wiring
- sample-facing theorem/model-check evidence summary and bless/review flow

の順序と split だけを固定する。

## scope

- current package は docs-only sequencing package に留める。
- entry criteria は `specs/examples/327...328`、`341...342`、`359...360`、`367...368`、`375...376` に置く。
- `proof_notebook_review_unit` は current first concrete pilot のまま維持する。
- public-checker side は docs-only reserve chain のまま保持し、actual migration へ昇格させない。
- current package では actual carrier code、sample runner wiring、sample-facing bless/review flow を同時に actualize しない。

## current 前提

current repo では次が成立している。

1. `specs/examples/327...328` により、tool-neutral formal hook artifact を入力にする row-local `proof_notebook_review_unit` が theorem-side current first concrete pilot に fixed 済みである。
2. `specs/examples/341...342` により、compare-ready docs-only bridge sketch は `comparison_basis_refs` を持つ current second reopen として fixed 済みである。
3. `specs/examples/359...360` により、machine-facing later line は model-check second reserve refs と public-checker second reserve refs に分けて inventory 済みである。
4. `specs/examples/367...368` により、model-check side は `tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow first actualization gate まで fixed 済みである。
5. `specs/examples/375...376` により、shared-space docs-first follow-up は一旦 checkpoint close と読めるため、repo-level current line は sample-visible theorem/model-check line へ clean に handoff できる。

したがって current 問いは、
**first actualization gate fixed 後の next reopen を、actual model-check carrier first / emitted verification artifact wiring second / sample-facing summary third に narrow に切るのが自然か**
である。

## 比較観点

1. `proof_notebook_review_unit` current first concrete pilot を current carrier のまま保てるか
2. actual model-check carrier の actualization を先に行っても、sample runner wiring や sample-facing summary を premature に混ぜないで済むか
3. public-checker docs-only reserve chain を parallel reserve として保てるか
4. repo-level next line を model-check concrete carrier first actualization へ clean に handoff できるか

## 比較対象

### 案 1. actual model-check carrier を first actualization とし、artifact wiring と sample-facing summary を separate package に送る

#### 読み

```text
model_check_concrete_carrier_actualization_comparison = {
  comparison_kind = current_l2_model_check_concrete_carrier_actualization_comparison,
  entry_criteria_refs = [
    model_check_concrete_carrier_first_actualization_gate,
    shared_space_authority_resource_ownership_reopen
  ],
  current_first_pilot_refs = [
    proof_notebook_review_unit_first_concrete_pilot
  ],
  actualization_entry_refs = [
    tool_neutral_formal_hook_only_input,
    compare_ready_docs_only_bridge_sketch
  ],
  first_actualization_target_refs = [
    model_check_concrete_carrier
  ],
  next_line_refs = [
    source_sample_emitted_verification_artifact_wiring,
    sample_facing_theorem_model_check_evidence_summary_and_bless_review_flow
  ],
  parallel_reserve_refs = [
    public_checker_docs_only_chain
  ],
  guard_refs = [
    keep_proof_notebook_as_current_first_pilot,
    keep_public_checker_chain_docs_only,
    avoid_concrete_tool_binding,
    avoid_emitted_verification_artifact_premature_merge,
    avoid_sample_facing_summary_premature_merge
  ],
  kept_later_refs = [
    actual_public_checker_migration,
    actual_emitted_verifier_handoff_artifact,
    bless_review_session_metadata,
    concrete_theorem_model_check_tool_binding,
    docs_first_host_facing_port_boundary_comparison
  ]
}
```

#### 利点

- `proof_notebook_review_unit` current first pilot を巻き戻さず、machine-facing sibling artifact だけを narrow に reopen できる。
- source-sample runner / regression helper 側へ premature に踏み込まずに済む。
- sample-facing bless/review flow を actual carrier shape の後段に残せる。

#### 欠点

- sample-visible path は 1 段ずつ進むので、artifact wiring までの距離はまだ残る。

### 案 2. source-sample emitted verification artifact wiring を先に reopen する

#### 利点

- sample runner / ladder と theorem/model-check 側の距離は早く見える。

#### 欠点

- actual carrier shape が未確定のまま wiring を始めるので drift しやすい。
- `mir-runtime` / regression helper を Package 2 で触る pressure が強すぎる。

### 案 3. sample-facing theorem/model-check evidence summary を先に reopen する

#### 利点

- 人間向け説明は早く厚くできる。

#### 欠点

- actual machine-facing carrier が無いまま bless/review flow を語ることになり、current repo の ratchet 順序と逆流する。
- current pilot と later concrete tool binding の境界が曖昧になる。

## current judgment

current L2 で最も自然なのは、
**案 1. actual model-check carrier を first actualization とし、artifact wiring と sample-facing summary を separate package に送る**
である。

理由は次の通り。

1. `specs/examples/367...368` が既に first actualization gate まで narrow に fixed しているため、次は gate そのものではなく actual carrier shape を最小に actualize するのが自然である。
2. `source-sample emitted verification artifact wiring` は actual carrier shape が無いと安定しにくく、current `e3` guarded line を bypass しやすい。
3. sample-facing theorem/model-check evidence summary は actual carrier と emitted artifact route の後段に置く方が、proof-notebook pilot と concrete tool binding later line の間を clean に保てる。

## current first choice details

- current first concrete pilot は `proof_notebook_review_unit` のまま維持する。
- first actualization target は `model_check_concrete_carrier` symbolic family に留める。
- actualization entry は `tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` に留める。
- source-sample emitted verification artifact wiring と sample-facing theorem/model-check evidence summary and bless/review flow は separate package として next line に送る。
- public-checker docs-only chain は parallel reserve に留め、actual public-checker migration は still later に残す。

## next promoted line

next promoted line は、
**model-check-concrete-carrier-actualization-comparison-ready model-check-concrete-carrier-first-actualization**
に置く。

## open questions

- model-check concrete carrier の actual shape を row-local case family と multi-row bundle family のどちらに寄せるのが最小か
- actual emitted carrier の schema / artifact kind 名を proof-notebook pilot とどう分けるべきか
- source-sample emitted verification artifact wiring で `mir-runtime` helper と regression helper のどちらを first integration surface に置くべきか
