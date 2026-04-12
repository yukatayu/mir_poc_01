# 348 — current L2 proof-model-check-first-concrete-tool-pilot-ready minimal-proof-model-check-first-concrete-tool-pilot threshold

## 目的

`specs/examples/347-current-l2-minimal-actual-e3-authored-row-ready-proof-model-check-first-concrete-tool-pilot-comparison.md`
で proof/model-check first concrete tool pilot の current first choice を fixed した次段として、

- current first concrete pilot の minimum をどこまでに留めるか
- actualized code anchor と current input/guard を minimum にどう残すか
- second source-sample cluster sequencing を next line にどう narrow handoff するか

を比較する。

ここで固定するのは
**current L2 proof-model-check-first-concrete-tool-pilot-ready minimal-proof-model-check-first-concrete-tool-pilot threshold**
であり、

- second source-sample cluster sequencing の具体順
- model-check concrete carrier
- public checker migration
- bless / review-session metadata

はまだ固定しない。

## 比較観点

1. current first concrete pilot を minimum に残せるか
2. proof notebook review-unit code anchor と guarded input reading を minimum に残せるか
3. next line を second source-sample cluster sequencing に narrow handoff できるか

## 比較対象

### 案 1. pilot 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- actualized code anchor と guarded input reading が弱い。

### 案 2. `pilot_kind + actualized_code_anchor_refs + current_input_refs + kept_later_refs` を持つ

#### 利点

- current concrete pilot の cut を lossless に残せる。
- existing helper/example/tests を current first concrete carrier として repository memory に残せる。
- next line の second source-sample cluster sequencing へ handoff しやすい。

#### 欠点

- 案 1 より fields は増える。

### 案 3. second source-sample cluster candidate まで threshold に含める

#### 利点

- 次段との接続は見えやすい。

#### 欠点

- threshold ではなく next reopen を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `pilot_kind + actualized_code_anchor_refs + current_input_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package は proof notebook current cut を first concrete pilot として閉じることが本体である。
2. actual code path を already 持つこと自体が current evidence なので、code anchor を minimum に残す必要がある。
3. second source-sample cluster sequencing は next mainline として明示すれば十分である。

## current first choice shape

```text
proof_model_check_first_concrete_tool_pilot = {
  pilot_kind = current_l2_proof_notebook_first_concrete_pilot,
  actualized_code_anchor_refs = [
    current_l2_formal_hook_support,
    current_l2_proof_notebook_review_unit_support,
    current_l2_proof_notebook_review_unit_support_tests,
    current_l2_emit_proof_notebook_review_unit
  ],
  current_input_refs = [
    tool_neutral_formal_hook_only_input,
    row_local_review_unit_shape,
    keep_e3_formal_hook_guarded
  ],
  kept_later_refs = [
    model_check_concrete_carrier,
    public_checker_migration,
    second_source_sample_cluster_sequencing,
    bless_review_session_metadata
  ]
}
```

## practical reading

current minimal threshold が示すのは、

- first concrete tool pilot は proof notebook review-unit current cut に留める
- current code anchor は helper / example / focused tests に already ある
- model-check side と public checker migration は still later に残す
- next promoted line は second source-sample cluster sequencing に置く

という最小 cut である。

## next promoted line

next promoted line は、
**proof-model-check-first-concrete-tool-pilot-ready second-source-sample-cluster-sequencing**
に置く。

## open questions

- second source-sample cluster の first candidate をどの family に置くか
- model-check side を proof notebook のどの row family から reopen するか
- public checker migration を later retained line としてどこで inventory 化するか
