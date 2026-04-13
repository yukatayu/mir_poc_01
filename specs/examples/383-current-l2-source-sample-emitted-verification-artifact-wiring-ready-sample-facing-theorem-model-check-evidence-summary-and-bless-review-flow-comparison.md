# 383 — current L2 source-sample-emitted-verification-artifact-wiring-ready sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow comparison

## 目的

`specs/examples/382-current-l2-source-sample-emitted-verification-artifact-wiring-ready-minimal-source-sample-emitted-verification-artifact-wiring-threshold.md`
で helper-local emitted route の minimum を fixed した次段として、

- sample code を読む人間に theorem/model-check evidence をどこでどう見せるのが最小か
- current repo-local bless/review flow をどこまで明示してよいか
- compare-ready bridge sketch、concrete tool binding、public CLI、retained archive を still later に残せるか

を比較する。

ここで固定するのは
**current L2 source-sample-emitted-verification-artifact-wiring-ready sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow comparison**
であり、

- sample-facing evidence summary
- repo-local bless/review flow
- kept-later externalization gate

だけを固定する。

## scope

- current package は docs-first package に留める。
- entry criteria は `specs/examples/325...326`、`327...328`、`341...342`、`381...382` に置く。
- current package では new code、retained artifact archive、public CLI、concrete theorem/model-check tool binding は actualize しない。
- compare-ready bridge sketch は docs-only context に留める。

## current 前提

current repo では次が成立している。

1. source-sample authoring / bless / regression policy は `specs/examples/325...326` により fixed 済みである。
2. theorem-side current first pilot は `specs/examples/327...328` により `proof_notebook_review_unit` に fixed 済みである。
3. compare-ready bridge sketch second reopen は `specs/examples/341...342` により docs-only context として fixed 済みである。
4. model-check concrete carrier first actualization は `specs/examples/379...380` により fixed 済みである。
5. source-sample emitted verification artifact wiring は `specs/examples/381...382` により fixed 済みであり、runtime/static reached row は helper-local emitted route から review-unit / model-check carrier helper output に届く。

したがって current 問いは、
**sample code を読む人間へ、current authored sample octet と emitted route の evidence をどこで見せ、何を current bless/review flow と呼ぶのが最小か**
である。

## 比較観点

1. sample-facing surface を README / `.docs` / snapshot docs だけで閉じられるか
2. reached runtime row、reached static row、guarded row の差を human-facing に説明できるか
3. compare-ready bridge sketch を docs-only context に留めつつ review-unit / model-check carrier sibling line を説明できるか
4. public CLI、retained archive、concrete tool binding を still later に残せるか

## 比較対象

### 案 1. README + `.docs` + snapshot docs で sample-facing evidence summary と repo-local bless/review flow を docs-first に固定する

#### 読み

```text
sample_facing_theorem_model_check_evidence_summary_and_bless_review_flow = {
  summary_kind = current_l2_sample_facing_theorem_model_check_evidence_summary,
  entry_criteria_refs = [
    source_sample_emitted_verification_artifact_wiring
  ],
  sample_surface_refs = [
    source_sample_readme,
    source_sample_authoring_policy,
    current_snapshot_docs
  ],
  evidence_route_refs = [
    source_sample
      -> runner_and_verification_ladder
      -> formal_hook_reached_or_guarded
      -> proof_notebook_review_units_and_model_check_concrete_carriers
  ],
  docs_only_context_refs = [
    compare_ready_bridge_sketch
  ],
  bless_review_refs = [
    review_source_fixture_matrix_ladder_snapshot_sync,
    run_inventory,
    run_regression,
    optionally_inspect_emitted_review_unit_and_model_check_carrier_artifacts,
    accept_repo_local_sync_as_current_bless
  ],
  guard_refs = [
    keep_bridge_sketch_docs_only,
    keep_e3_guarded,
    avoid_retained_archive_policy,
    avoid_public_cli,
    avoid_concrete_tool_binding
  ],
  kept_later_refs = [
    docs_first_io_host_facing_port_boundary,
    actual_compare_ready_bridge_metadata,
    retained_artifact_bless_archive,
    public_operational_cli,
    concrete_theorem_model_check_tool_binding
  ]
}
```

#### 利点

- 人間が current sample-visible evidence を追う path を、existing docs surface だけで一貫して説明できる。
- `e3` guarded row の扱いを failure ではなく current guard として明示できる。
- public CLI や retained archive をまだ要件化せずに current bless/review flow を言語化できる。

#### 欠点

- bless/review flow は repo-local 実務フローの説明に留まり、external tool contract ではない。

### 案 2. compare-ready bridge sketch や bless/review metadata の actual carrier を先に作る

#### 利点

- bridge / bless の future contract は見えやすい。

#### 欠点

- docs-only context を premature に code/contract 化しやすい。
- current helper-local route と review flow を超えてしまう。

### 案 3. public CLI / retained archive / concrete tool binding を current bless と同時に進める

#### 利点

- operational path は一見分かりやすい。

#### 欠点

- public surface と concrete tool choice を早く凍らせやすい。
- current repo がまだ持っていない契約を作ってしまう。

## current judgment

current L2 で最も自然なのは、
**案 1. README + `.docs` + snapshot docs で sample-facing evidence summary と repo-local bless/review flow を docs-first に固定する**
である。

理由は次の通り。

1. current helper-local emitted route と authored sample octet は既に source-backed であり、人間向け説明を足すだけで sample-visible milestone を閉じられる。
2. compare-ready bridge sketch は docs-only context のままでも、review-unit / model-check carrier sibling line を理解するには十分である。
3. public CLI や retained archive を current bless に混ぜない方が later gate と整合する。

## current first choice details

- sample-facing evidence summary は `samples/current-l2/README.md` を current reader-facing anchor に置く。
- repo-local bless/review flow は `.docs/current-l2-source-sample-authoring-policy.md` を operational anchor に置く。
- reached runtime row / reached static row は helper-local emitted route の downstream artifact を current evidence として見てよい。
- `e3` は `formal hook = not reached (guarded)` と empty followup artifact list を current evidence として持つ。
- compare-ready bridge sketch は docs-only context として言及してよいが、actual bridge metadata や emitted bridge artifact は current bless の要件に入れない。
- current bless は reviewed repo-local sync + inventory/regression success を意味し、retained archive や public command を意味しない。

## next promoted line

next promoted line は、
**sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready docs-first-io-host-facing-port-boundary-comparison**
に置く。

## open questions

- docs-first I/O / host-facing port boundary の working label をどこで final term 候補へ寄せるか
- compare-ready bridge sketch を later に actualize するならどの metadata から narrow に reopen するか
- current bless/review flow を retained archive へ widen する pressure がいつ出るか
