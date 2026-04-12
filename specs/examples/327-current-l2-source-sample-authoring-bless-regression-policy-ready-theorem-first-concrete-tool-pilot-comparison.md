# 327 — current L2 source-sample-authoring-bless-regression-policy-ready theorem-first-concrete-tool-pilot comparison

## 目的

`specs/examples/325-current-l2-verification-ladder-wiring-ready-source-sample-authoring-bless-regression-policy-comparison.md`
と
`specs/examples/326-current-l2-source-sample-authoring-bless-regression-policy-ready-minimal-source-sample-authoring-bless-regression-policy-threshold.md`
で source-sample authoring / bless / regression policy を fixed した次段として、

- tool-neutral formal hook 後段の theorem-first concrete consumer を何に置くか
- concrete pilot をどこまで actual code path に寄せてよいか
- bridge sketch / compare-bless metadata / proof-assistant adapter をどこまで後段に残すか

を比較する。

ここで固定するのは
**current L2 source-sample-authoring-bless-regression-policy-ready theorem-first-concrete-tool-pilot comparison**
であり、

- notebook bridge sketch
- compare / bless metadata
- proof assistant adapter / model-check side
- public checker migration

はまだ固定しない。

## scope

- entry criteria は `specs/examples/303...304` の tool-neutral formal hook と `specs/examples/325...326` の source-sample policy に置く。
- concrete pilot は theorem-side consumer だけを扱う。
- source sample runner / ladder / repo-local policy を巻き戻さない。

## current 前提

current repo では次が成立している。

1. theorem-side first practical consumer class は `proof_notebook` に置く。
2. concrete notebook workflow pressure の first threshold は human review checklist / walkthrough pressure に置く。
3. current named review unit bundle は `subject_ref + row(obligation_kind + evidence_refs + goal_text) + checklist` に留めるのが natural である。
4. tool-neutral formal hook artifact は `subject_kind + subject_ref + contract_rows(obligation_kind + typed symbolic evidence_refs)` で current code path に actualize 済みである。
5. source-sample path は repo-local policy helper まで fixed しており、theorem-first pilot はその後段 reserve として扱える。

したがって current 問いは、
**tool-neutral formal hook artifact を入力にした non-production proof notebook review unit を concrete pilot に置くのが最小か**
である。

## 比較観点

1. tool-neutral formal hook current cut を壊さないか
2. proof_notebook first bridge と連続的か
3. bridge sketch / compare-bless metadata / retained path policy を premature に混ぜないか
4. source-sample policy と proof consumer pressure の handoff を明確に保てるか

## 比較対象

### 案 1. tool-neutral formal hook から `proof_notebook_review_unit` を直接導く

#### shape

```text
theorem_first_concrete_tool_pilot = {
  pilot_kind = current_l2_proof_notebook_review_unit_first_pilot,
  input_artifact_kind = current_l2_tool_neutral_formal_hook,
  review_unit = {
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
    current_l2_emit_proof_notebook_review_unit
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_input,
    avoid_bridge_sketch_premature_merge,
    avoid_compare_bless_metadata_premature_merge,
    avoid_proof_assistant_adapter_binding
  ]
}
```

#### 利点

- existing tool-neutral formal hook row core をそのまま concrete consumer に渡しやすい。
- `proof_notebook` first bridge と最も連続的である。
- bridge sketch / compare-bless metadata / retained path policy を後段に残せる。

#### 欠点

- review unit の `goal_text` / `checklist` を current cut で narrow derivation する必要がある。

### 案 2. formal hook から notebook bridge sketch を直接導く

#### 利点

- bridge-level walkthrough まで一気に見える。

#### 欠点

- row-local review unit と bridge sketch を同時に固定しやすい。
- compare / bless metadata pressure と近づきやすい。

### 案 3. proof assistant adapter / model-check side を first concrete pilot にする

#### 利点

- machine-facing consumer pressure は見えやすい。

#### 欠点

- tool-neutral formal hook current cut と source-sample policy current cut からの跳躍が大きい。
- solver-specific / machine-facing schema を premature に要求しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. tool-neutral formal hook から `proof_notebook_review_unit` を直接導く**
である。

理由は次の通り。

1. `proof_notebook` first consumer と `review_unit` current cut に最も連続的である。
2. source-sample policy fixed 後の next pressure として、human-facing theorem consumer を narrow actualize できる。
3. bridge sketch / compare-bless metadata / proof assistant adapter を still later に残せる。

## current first choice details

- concrete pilot は non-production helper / example / focused tests に留める。
- review unit の `goal_text` と `checklist` は `subject_kind + obligation_kind` から narrow に導き、solver-specific hint や retained metadata は持ち込まない。
- input は tool-neutral formal hook artifact に固定し、detached bundle / static gate artifact からの direct branch は持たない。

## open questions

- bridge sketch comparison へ進むとき、review unit の named bundle をどこまで再利用するか
- `proof_assistant_adapter` second candidate をいつ concrete にするか
- source-sample widened authored rows が theorem-first pilot の example coverageに何を要求するか
