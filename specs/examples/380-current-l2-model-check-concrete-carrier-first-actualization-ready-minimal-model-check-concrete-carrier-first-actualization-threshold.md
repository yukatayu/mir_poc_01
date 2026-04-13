# 380 — current L2 model-check-concrete-carrier-first-actualization-ready minimal-model-check-concrete-carrier-first-actualization threshold

## 目的

`specs/examples/379-current-l2-model-check-concrete-carrier-actualization-comparison-ready-model-check-concrete-carrier-first-actualization-comparison.md`
で model-check concrete carrier first actualization の current first choice を fixed した次段として、

- first actual carrier package の minimum をどこまでに留めるか
- hard input、docs-only context、actual artifact、current pilot、guard を minimum にどう残すか
- repo-level next line と kept-later machine-facing line をどう handoff するか

を比較する。

ここで固定するのは
**current L2 model-check-concrete-carrier-first-actualization-ready minimal-model-check-concrete-carrier-first-actualization threshold**
であり、

- tool-neutral formal hook only hard input
- row-local machine-facing carrier list
- fail-closed guard

だけを minimum に残す。

## 比較観点

1. first actual carrier package の current cut を lossless に minimum へ残せるか
2. `proof_notebook_review_unit` current first pilot と machine-facing actual carrier を minimum に区別できるか
3. source-sample emitted verification artifact wiring と public-checker actual migration を still later に残せるか
4. repo-level next line を source-sample emitted verification artifact wiring へ handoff できるか

## 比較対象

### 案 1. actualization 名と artifact kind 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- hard input、docs-only context、row-local case shape、guard が見えない。
- fail-closed 条件と kept-later line が drift しやすい。

### 案 2. `actualization_kind + entry_criteria_refs + hard_input_refs + docs_only_context_refs + actual_artifact_refs + current_pilot_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- first actual carrier package の current cut を lossless に残せる。
- theorem-side current pilot と machine-facing actual carrier を minimum に区別できる。
- wiring / sample-facing summary / public-checker migration / concrete tool binding を later に残す guard を minimum に持てる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. source-sample emitted verification artifact wiring や sample-facing summary まで threshold に含める

#### 利点

- sample-visible path との接続は見えやすい。

#### 欠点

- threshold ではなく次 package の内容を先取りする。
- actual carrier package の narrow cut を重くしすぎる。

## current judgment

current L2 で最も自然なのは、
**案 2. `actualization_kind + entry_criteria_refs + hard_input_refs + docs_only_context_refs + actual_artifact_refs + current_pilot_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は formal hook から row-local machine-facing carrier を actualize することであり、そこに wiring や sample-facing summary を混ぜるべきではない。
2. `proof_notebook_review_unit` current first pilot を minimum に残すことで theorem-side pilot と machine-facing carrier の sibling relation を保てる。
3. public-checker docs-only reserve chain を崩さず、concrete tool binding を later に残すには guard と kept-later refs が必要である。

## current first choice shape

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

## practical reading

current minimal model-check concrete carrier first actualization が示すのは、

- hard input は tool-neutral formal hook artifact だけである
- actual output は row-local machine-facing carrier list である
- compare-ready bridge sketch は docs-only context に留める
- source-sample emitted verification artifact wiring と sample-facing summary は次 package に送る

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-model-check-concrete-carrier-first-actualization-ready source-sample-emitted-verification-artifact-wiring**
に置く。

## open questions

- source-sample emitted verification artifact wiring の first integration surface を runtime helper と regression helper のどちらに置くか
- sample-facing summary で review-unit と model-check carrier をどう並べるか
- later public-checker migration と actual emitted verifier handoff artifact の順序をどこで比較するか
