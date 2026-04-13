# 378 — current L2 model-check-concrete-carrier-actualization-comparison-ready minimal-model-check-concrete-carrier-actualization threshold

## 目的

`specs/examples/377-current-l2-shared-space-authority-resource-ownership-reopen-ready-model-check-concrete-carrier-actualization-comparison.md`
で model-check concrete carrier actualization comparison の current first choice を fixed した次段として、

- actualization sequencing comparison の minimum をどこまでに留めるか
- current first pilot、first actualization target、next line、parallel reserve、guard を minimum にどう残すか
- repo-level next line と kept-later machine-facing line をどう handoff するか

を比較する。

ここで固定するのは
**current L2 model-check-concrete-carrier-actualization-comparison-ready minimal-model-check-concrete-carrier-actualization threshold**
であり、

- model-check concrete carrier first actualization
- source-sample emitted verification artifact wiring
- sample-facing theorem/model-check evidence summary and bless/review flow

の順序と split だけを minimum に残す。

## 比較観点

1. actualization sequencing の current cut を lossless に minimum へ残せるか
2. `proof_notebook_review_unit` current first pilot と `model_check_concrete_carrier` first actualization target を minimum に区別できるか
3. public-checker docs-only reserve chain を parallel reserve として保持できるか
4. repo-level next line を model-check concrete carrier first actualization へ handoff できるか

## 比較対象

### 案 1. comparison 名と next promoted line だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- first actualization target、next line、parallel reserve、guard が見えない。
- sample-visible theorem/model-check line の staged reopen order が drift しやすい。

### 案 2. `comparison_kind + entry_criteria_refs + current_first_pilot_refs + actualization_entry_refs + first_actualization_target_refs + next_line_refs + parallel_reserve_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- actualization sequencing の current cut を lossless に残せる。
- theorem-side current first pilot と machine-facing first actualization target を minimum に区別できる。
- emitted artifact wiring と sample-facing summary を separate package に送る guard を minimum に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. actual emitted carrier schema や sample-facing bless/review metadata まで threshold に含める

#### 利点

- later actualization との接続は見えやすい。

#### 欠点

- threshold ではなく Package 2 / Package 4 の actualization 内容を先取りする。
- current sequencing package の narrow cut を重くしすぎる。

## current judgment

current L2 で最も自然なのは、
**案 2. `comparison_kind + entry_criteria_refs + current_first_pilot_refs + actualization_entry_refs + first_actualization_target_refs + next_line_refs + parallel_reserve_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は actualization order の narrowing であり、actual carrier schema や sample-facing metadata は next package へ送るべきである。
2. `proof_notebook_review_unit` first concrete pilot を current carrier として minimum に残す必要がある。
3. source-sample emitted verification artifact wiring と sample-facing evidence summary を separate package に保つ guard が minimum にないと snapshot drift を起こしやすい。

## current first choice shape

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

## practical reading

current minimal model-check concrete carrier actualization comparison が示すのは、

- current first concrete pilot は `proof_notebook_review_unit` のままである
- first actualization target は `model_check_concrete_carrier` である
- source-sample emitted verification artifact wiring と sample-facing theorem/model-check evidence summary は次 package に分ける
- public-checker side は docs-only reserve chain のまま parallel reserve に留める

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-model-check-concrete-carrier-actualization-threshold-ready model-check-concrete-carrier-first-actualization**
に置く。

## open questions

- first actual carrier を row-local case list へ留めるか、subject-bundle shape を同時に持たせるか
- source-sample emitted verification artifact wiring の first route を runtime helper から開くか、regression helper から開くか
- sample-facing theorem/model-check evidence summary で compare-ready bridge sketch と proof-notebook review-unit をどう並べるか
