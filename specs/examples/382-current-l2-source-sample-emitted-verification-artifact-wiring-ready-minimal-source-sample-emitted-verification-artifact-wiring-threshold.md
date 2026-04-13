# 382 — current L2 source-sample-emitted-verification-artifact-wiring-ready minimal-source-sample-emitted-verification-artifact-wiring threshold

## 目的

`specs/examples/381-current-l2-model-check-concrete-carrier-first-actualization-ready-source-sample-emitted-verification-artifact-wiring-comparison.md`
で source-sample emitted verification artifact wiring の current first choice を fixed した次段として、

- emitted route package の minimum をどこまでに留めるか
- source entry、emitted route、guard、kept-later refs をどう残すか
- repo-level next line を sample-facing summary package にどう handoff するか

を比較する。

ここで固定するのは
**current L2 source-sample-emitted-verification-artifact-wiring-ready minimal-source-sample-emitted-verification-artifact-wiring threshold**
であり、

- source sample runner entry
- emitted route payload
- reached/guarded split

だけを minimum に残す。

## 比較観点

1. helper-local emitted route actualization の current cut を lossless に minimum へ残せるか
2. runner report shape を widen せずに downstream artifact route だけを記述できるか
3. sample-facing bless/review flow と docs-first I/O boundary を still later に残せるか
4. `e3` guarded line と underdeclared fail-closed を minimum に残せるか

## 比較対象

### 案 1. `wiring_kind + entry_criteria_refs + source_entry_refs + emitted_route_refs + route_rule_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- helper-local emitted route package の current cut を lossless に残せる。
- source sample runner と downstream artifact route の境界を minimum に区別できる。
- guarded row / kept-later line を minimum に持てる。

#### 欠点

- 最小 field 数としてはやや多い。

### 案 2. emitted artifact 名と reached/guarded verdict だけを残す

#### 利点

- 軽い。

#### 欠点

- source entry、route rule、guard、kept-later line が見えない。
- `run_current_l2_source_sample` public/report shape を保つ current judgmentが drift しやすい。

### 案 3. bless/review flow や later public gate まで threshold に含める

#### 利点

- human-facing path は見えやすい。

#### 欠点

- next package と later gate を先取りし、current package の narrow cut を壊す。

## current judgment

current L2 で最も自然なのは、
**案 1. `wiring_kind + entry_criteria_refs + source_entry_refs + emitted_route_refs + route_rule_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は source sample runner から downstream verification artifact へ届く helper-local route を actualize することであり、public/report widen や bless/review flow はまだ混ぜるべきではない。
2. reached/guarded split と underdeclared fail-closed を minimum に残すことで、current formal-hook top と sample-visible line の境界を維持できる。
3. sample-facing summary と docs-first I/O boundary を後段へ clean に handoff できる。

## current first choice shape

```text
source_sample_emitted_verification_artifact_wiring = {
  wiring_kind = current_l2_source_sample_emitted_verification_artifact_wiring,
  entry_criteria_refs = [
    model_check_concrete_carrier_first_actualization
  ],
  source_entry_refs = [
    run_current_l2_source_sample,
    current_l2_source_sample_run_report
  ],
  emitted_route_refs = [
    emitted_route(
      source_report,
      formal_hook_status,
      formal_hook_guard_reason,
      formal_hook_artifact?,
      proof_notebook_review_units[],
      model_check_concrete_carriers[]
    )
  ],
  route_rule_refs = [
    valid_row_uses_fixture_aligned_detached_bundle_to_formal_hook,
    malformed_row_uses_fixture_static_gate_to_formal_hook,
    guarded_row_returns_not_reached_without_followup_artifacts
  ],
  guard_refs = [
    keep_runner_public_shape_unchanged,
    keep_formal_hook_top_unchanged,
    keep_e3_guarded,
    fail_closed_on_underdeclared_or_missing_fixture_path,
    keep_public_checker_chain_docs_only,
    avoid_regression_helper_contract_widening
  ],
  kept_later_refs = [
    sample_facing_theorem_model_check_evidence_summary_and_bless_review_flow,
    docs_first_io_host_facing_port_boundary,
    final_public_parser_checker_runtime_api,
    public_operational_cli
  ]
}
```

## practical reading

current minimal source-sample emitted verification artifact wiring が示すのは、

- source entry は `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` に留める
- emitted route は formal hook reached/guarded split と review-unit / model-check carrier fan-out を持つ
- runner 本体の public/report shape は変えない
- sample-facing summary は次 package へ送る

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-source-sample-emitted-verification-artifact-wiring-ready sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow**
に置く。

## open questions

- sample-facing summary package で review-unit / model-check carrier / compare-ready bridge sketch をどう見せ分けるか
- current bless/review flow を repo-local sync / regression success / human review のどこまでで minimum に切るか
- emitted route を later public contract や regression helperへいつ widen するか
