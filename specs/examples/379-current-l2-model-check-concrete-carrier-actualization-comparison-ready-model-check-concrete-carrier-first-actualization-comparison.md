# 379 — current L2 model-check-concrete-carrier-actualization-comparison-ready model-check-concrete-carrier-first-actualization comparison

## 目的

`specs/examples/378-current-l2-model-check-concrete-carrier-actualization-comparison-ready-minimal-model-check-concrete-carrier-actualization-threshold.md`
で model-check concrete carrier actualization sequencing の minimum を fixed した次段として、

- first actual carrier を row-local machine-facing sibling artifact に留めるのが自然か
- tool-neutral formal hook を唯一の hard input にしつつ、compare-ready docs-only bridge sketch を docs-only context に留める cut が自然か
- source-sample emitted verification artifact wiring、sample-facing theorem/model-check evidence summary、public-checker actual migration、concrete theorem/model-check tool binding をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 model-check-concrete-carrier-actualization-comparison-ready model-check-concrete-carrier-first-actualization comparison**
であり、

- row-local machine-facing carrier の actual shape
- hard input / docs-only context / fail-closed condition

だけを固定する。

## scope

- current package は `mir-semantics` helper-local actualization package に留める。
- entry criteria は `specs/examples/327...328`、`341...342`、`359...360`、`367...368`、`377...378` に置く。
- `proof_notebook_review_unit` は current first theorem-side pilot のまま維持する。
- current package では `mir-runtime` sample runner / ladder / regression helper には触れず、source-sample emitted verification artifact wiring は next package に送る。
- public-checker docs-only reserve chain は actual migration へ昇格させない。

## current 前提

current repo では次が成立している。

1. `specs/examples/327...328` により、tool-neutral formal hook artifact を入力にする row-local `proof_notebook_review_unit` が theorem-side current first concrete pilot に fixed 済みである。
2. `specs/examples/341...342` により、compare-ready docs-only bridge sketch は current second reopen として fixed 済みである。
3. `specs/examples/359...360` により、machine-facing later line は model-check second reserve refs と public-checker second reserve refs に分けて inventory 済みである。
4. `specs/examples/367...368` により、model-check side は `tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow first actualization gate まで fixed 済みである。
5. `specs/examples/377...378` により、sample-visible theorem/model-check line の reopen order は actual carrier first / emitted artifact wiring second / sample-facing summary third に固定済みである。

したがって current 問いは、
**first actual carrier を、tool-neutral formal hook を唯一の hard input にする row-local machine-facing sibling artifact として `mir-semantics` helper-local に留めるのが自然か**
である。

## 比較観点

1. `proof_notebook_review_unit` current first theorem-side pilot を巻き戻さず、machine-facing sibling artifact を足せるか
2. current `e3` guarded line を bypass せずに、formal hook 境界だけを hard input にできるか
3. source-sample emitted verification artifact wiring と sample-facing summary を premature に混ぜずに済むか
4. public-checker actual migration と concrete tool binding を still later に残せるか

## 比較対象

### 案 1. tool-neutral formal hook only input から row-local machine-facing sibling artifact を actualize する

#### 読み

```text
model_check_concrete_carrier_first_actualization = {
  actualization_kind = current_l2_model_check_concrete_carrier_first_actualization,
  entry_criteria_refs = [
    model_check_concrete_carrier_actualization_comparison
  ],
  hard_input_refs = [
    tool_neutral_formal_hook_artifact
  ],
  docs_only_context_refs = [
    compare_ready_docs_only_bridge_sketch
  ],
  actual_artifact_refs = [
    model_check_concrete_carrier_artifact(
      schema_version,
      artifact_kind,
      subject_kind,
      subject_ref,
      row_local_case(obligation_kind, evidence_refs)
    )
  ],
  current_pilot_refs = [
    proof_notebook_review_unit_first_concrete_pilot
  ],
  guard_refs = [
    keep_row_local_case_split,
    keep_public_checker_chain_docs_only,
    avoid_sample_runner_wiring,
    avoid_concrete_tool_binding,
    fail_closed_on_schema_kind_pair_or_empty_evidence
  ],
  kept_later_refs = [
    source_sample_emitted_verification_artifact_wiring,
    sample_facing_theorem_model_check_evidence_summary_and_bless_review_flow,
    actual_public_checker_migration,
    actual_emitted_verifier_handoff_artifact,
    concrete_theorem_model_check_tool_binding
  ]
}
```

#### 利点

- `proof_notebook_review_unit` current first pilot を維持したまま、machine-facing sibling artifact を最小に actualize できる。
- hard input を formal hook に限定できるので、`e3` guarded line を bypass しない。
- runtime helper や regression helper をまだ触らずに済む。

#### 欠点

- compare-ready bridge sketch は docs-only context に留まり、code hard input には入らない。

### 案 2. compare-ready bridge sketch shape や subject-bundle shape を同時に code 化する

#### 利点

- later bridge sketch との接続は見えやすい。

#### 欠点

- current repo に code anchor が無い bridge sketch shape を premature に actualize する。
- row-local cutを越えて bundle shape へ drift しやすい。

### 案 3. sample runner wiring や public-checker actual migration まで current package に含める

#### 利点

- sample-visible path や public path を早く見せやすい。

#### 欠点

- `mir-runtime` / regression helper / public-checker docs-only reserve line を同時に触ることになり、package 境界が崩れる。
- current guarded line や public reserve line を壊しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. tool-neutral formal hook only input から row-local machine-facing sibling artifact を actualize する**
である。

理由は次の通り。

1. current repo の hard input として source-backed なのは tool-neutral formal hook artifact であり、compare-ready bridge sketch はまだ docs-only bridge である。
2. first actual carrier を row-local case に留めると、`proof_notebook_review_unit` pilot と sibling relation を保ったまま machine-facing cut を最小にできる。
3. source-sample emitted verification artifact wiring と sample-facing theorem/model-check evidence summary はその次 package に送る方が drift が少ない。

## current first choice details

- hard input は `ToolNeutralFormalHookArtifact` だけに留める。
- actual output は `schema_version + artifact_kind + subject_kind + subject_ref + case(obligation_kind + evidence_refs)` の row-local list に留める。
- supported pair は current formal hook line の runtime 1 件 + static 2 件だけを受け付ける。
- schema / artifact kind mismatch、unsupported pair、empty `subject_ref`、empty `contract_rows`、empty `evidence_refs` は fail-closed に止める。
- `compare_ready_docs_only_bridge_sketch` は docs-only context ref に留め、code hard input へは上げない。

## next promoted line

next promoted line は、
**model-check-concrete-carrier-first-actualization-ready source-sample-emitted-verification-artifact-wiring**
に置く。

## open questions

- source-sample emitted verification artifact wiring の first route を runtime helper と regression helper のどちらへ置くか
- sample-facing summary package で review-unit と model-check carrier をどの order で見せるか
- model-check carrier の later bundle shape を必要に応じてどこで reopen するか
