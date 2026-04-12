# 328 — current L2 theorem-first-concrete-tool-pilot-ready minimal-theorem-first-concrete-tool-pilot threshold

## 目的

`specs/examples/327-current-l2-source-sample-authoring-bless-regression-policy-ready-theorem-first-concrete-tool-pilot-comparison.md`
で theorem-first concrete tool pilot の current first choice を fixed した次段として、

- concrete pilot minimum をどこまでに留めるか
- review unit current cut と code anchor を minimum にどう残すか
- bridge sketch / compare-bless metadata / proof-assistant adapter をどこまで minimum から外すか

を比較する。

ここで固定するのは
**current L2 theorem-first-concrete-tool-pilot-ready minimal-theorem-first-concrete-tool-pilot threshold**
であり、

- notebook bridge sketch
- compare / bless metadata
- proof assistant adapter / model-check side
- public checker migration

はまだ固定しない。

## 比較観点

1. tool-neutral formal hook current cut を保てるか
2. proof notebook review unit current cut を minimum に残せるか
3. still-later line を guard として明示できるか

## 比較対象

### 案 1. review unit payload だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- tool-neutral formal hook input と code anchor の guard が弱い。

### 案 2. `pilot_kind + input_artifact_ref + review_unit_shape + code_anchor_refs + guard_refs` を持つ

#### 利点

- concrete pilot の actual cut と still-later guard を同時に残せる。
- review unit を bridge sketch や compare-bless metadata と混ぜずに済む。

#### 欠点

- 案 1 より fields は増える。

### 案 3. bridge sketch / compare-bless metadata まで minimum に含める

#### 利点

- 後段との接続は見えやすい。

#### 欠点

- current threshold ではなく later reopen を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `pilot_kind + input_artifact_ref + review_unit_shape + code_anchor_refs + guard_refs` を持つ**
である。

理由は次の通り。

1. current task の本体は review unit concrete pilot を narrow actualize することであり、tool-neutral formal hook input と still-later guard を minimum に残す必要がある。
2. bridge sketch / compare-bless metadata / proof assistant adapter は still later に残すべきである。
3. code anchor を minimum に含めることで、non-production helper/example/test cut を repo memory に残せる。

## current first choice shape

```text
theorem_first_concrete_tool_pilot = {
  pilot_kind = current_l2_proof_notebook_review_unit_first_pilot,
  input_artifact_ref = current_l2_tool_neutral_formal_hook,
  review_unit_shape = {
    notebook_kind = proof_notebook_review_unit,
    source_artifact_ref = tool_neutral_formal_hook,
    subject_kind,
    subject_ref,
    row = {
      obligation_kind,
      evidence_refs,
      goal_text
    },
    checklist
  },
  code_anchor_refs = [
    current_l2_formal_hook_support,
    current_l2_proof_notebook_review_unit_support,
    current_l2_emit_proof_notebook_review_unit,
    current_l2_proof_notebook_review_unit_support_tests
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_only_input,
    avoid_bridge_sketch_premature_merge,
    avoid_compare_bless_metadata_premature_merge,
    avoid_proof_assistant_adapter_binding,
    avoid_model_check_side_binding
  ]
}
```

## practical reading

current minimal theorem-first concrete tool pilot が示すのは、

- concrete pilot は tool-neutral formal hook を input にした proof notebook review unit に留める
- review unit は `subject_kind + subject_ref + row(obligation_kind + evidence_refs + goal_text) + checklist` に留める
- bridge sketch / compare-bless metadata / proof assistant adapter / model-check side は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**theorem-first-concrete-tool-pilot-ready post-checkpoint-drift-suppression comparison**
に置く。

## open questions

- bridge sketch comparison をどの package で reopen するか
- runtime/static formal hook coverage を proof notebook review unit examplesにどう広げるか
- wider authored-row sequencing が theorem-first concrete pilot の next evidence に何を要求するか
