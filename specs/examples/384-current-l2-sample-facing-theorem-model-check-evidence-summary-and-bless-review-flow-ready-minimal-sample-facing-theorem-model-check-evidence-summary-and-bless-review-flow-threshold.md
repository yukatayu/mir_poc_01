# 384 — current L2 sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready minimal-sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow threshold

## 目的

`specs/examples/383-current-l2-source-sample-emitted-verification-artifact-wiring-ready-sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-comparison.md`
で sample-facing theorem/model-check evidence summary と bless/review flow の current first choice を fixed した次段として、

- current docs-first package の minimum をどこまでに留めるか
- sample surface、evidence route、bless/review refs、guard をどう残すか
- repo-level next line を docs-first I/O / host-facing port boundary へどう handoff するか

を比較する。

ここで固定するのは
**current L2 sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready minimal-sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow threshold**
であり、

- sample-facing surface
- repo-local bless/review flow
- kept-later externalization gate

だけを minimum に残す。

## 比較観点

1. docs-first summary package の current cut を lossless に minimum へ残せるか
2. README / `.docs` / snapshot docs の役割分担を minimum に残せるか
3. compare-ready bridge sketch を docs-only context に留める current judgmentを残せるか
4. docs-first I/O / host-facing port boundary を next line に clean に handoff できるか

## 比較対象

### 案 1. `summary_kind + entry_criteria_refs + sample_surface_refs + evidence_route_refs + docs_only_context_refs + bless_review_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- current docs-first package の cut を lossless に残せる。
- sample-facing surface と repo-local bless/review flow の境界を minimum に分けて持てる。
- current bless が public CLI / retained archive / concrete tool bindingではないことを guard に残せる。

#### 欠点

- fields はやや多い。

### 案 2. bless の prose summary だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- evidence route、sample surface、docs-only context、kept-later line が見えない。
- later externalization gate との境界が drift しやすい。

### 案 3. docs-first I/O / host-facing port boundary まで threshold に含める

#### 利点

- next line との接続は見えやすい。

#### 欠点

- current package と次 package を混ぜ、current docs-first summary cut を崩す。

## current judgment

current L2 で最も自然なのは、
**案 1. `summary_kind + entry_criteria_refs + sample_surface_refs + evidence_route_refs + docs_only_context_refs + bless_review_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は human-facing summary と repo-local bless/review flow の整理であり、surface / route / guard を minimum に分けて持つ方が drift が少ない。
2. compare-ready bridge sketch や external tool binding を current bless に混ぜないためには docs-only context と kept-later refs が必要である。
3. next line を docs-first I/O / host-facing port boundary comparison に clean に handoff できる。

## current first choice shape

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

## practical reading

current minimal sample-facing theorem/model-check evidence summary and bless/review flow が示すのは、

- sample-facing evidence は README / `.docs` / snapshot docs で見る
- current bless は repo-local sync + inventory/regression success である
- compare-ready bridge sketch は docs-only context に留める
- public CLI / retained archive / concrete tool binding はまだ current bless に含めない

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow-ready docs-first-io-host-facing-port-boundary-comparison**
に置く。

## open questions

- docs-first I/O / host-facing port boundary の working label と first comparison axes をどう切るか
- compare-ready bridge metadata を later に actualize する pressure がどこから来るか
- retained archive / public CLI / concrete tool binding の first reopen order をどこで比較するか
