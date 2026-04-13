# 381 — current L2 model-check-concrete-carrier-first-actualization-ready source-sample-emitted-verification-artifact-wiring comparison

## 目的

`specs/examples/380-current-l2-model-check-concrete-carrier-first-actualization-ready-minimal-model-check-concrete-carrier-first-actualization-threshold.md`
で first actual machine-facing carrier の minimum を fixed した次段として、

- current authored source sample octet と verification ladder reached row を theorem/model-check side helper output にどう接続するのが最小か
- `run_current_l2_source_sample` の public/report shape を変えずに emitted route を actualize できるか
- `e3` guarded line、public-checker docs-only reserve、sample-facing bless/review flow を premature に混ぜずに済むか

を比較する。

ここで固定するのは
**current L2 model-check-concrete-carrier-first-actualization-ready source-sample-emitted-verification-artifact-wiring comparison**
であり、

- source sample runner から downstream verification artifact へ届く narrow route
- reached/guarded split
- kept-later sample-facing flow

だけを固定する。

## scope

- current package は `mir-runtime` test/support helper-local actualization package に留める。
- entry criteria は `specs/examples/321...324`、`325...326`、`327...328`、`377...380` に置く。
- `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` の public/report shape は変えない。
- current package では public operational surface や regression helper contract は widen せず、sample-facing theorem/model-check evidence summary and bless/review flow は next package に送る。
- compare-ready bridge sketch は docs-only context に留める。

## current 前提

current repo では次が成立している。

1. syntax-backed sample runner / verification ladder は `specs/examples/321...324` により fixed 済みであり、current authored source sample octet は `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` に揃っている。
2. source-sample authoring / bless / regression policy は `specs/examples/325...326` により fixed 済みであり、current `bless` は repo-local sync + regression success の意味に留める。
3. theorem-side current first pilot は `specs/examples/327...328` により `proof_notebook_review_unit` に fixed 済みである。
4. `specs/examples/377...378` により、sample-visible theorem/model-check line の reopen order は actual model-check carrier first / emitted artifact wiring second / sample-facing summary third に固定済みである。
5. `specs/examples/379...380` により、tool-neutral formal hook only hard input から row-local model-check carrier list を actualize 済みである。

したがって current 問いは、
**current authored source sample と verification ladder reached row を、`run_current_l2_source_sample` の public/report shapeを変えずに review-unit / model-check carrier helper output へどう狭く接続するか**
である。

## 比較観点

1. `run_current_l2_source_sample` current cut を保ったまま emitted route を actualize できるか
2. runtime row / static row / guarded row の split を current formal-hook top と整合的に保てるか
3. `proof_notebook_review_unit` current first pilot と model-check carrier sibling line を同じ route で narrow に束ねられるか
4. sample-facing bless/review flow、public operational surface、public-checker chain を still later に残せるか

## 比較対象

### 案 1. runtime test/support helper-local route で source sample runner から formal hook / review-unit / model-check carrier へ fan-out する

#### 読み

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

#### 利点

- `run_current_l2_source_sample` の public/report shape を変えずに、sample-visible theorem/model-check side へ narrow route を actualize できる。
- reached runtime row、reached static row、`e3` guarded row の split を current formal-hook top と整合的に保てる。
- review-unit pilot と model-check carrier sibling line を同じ helper-local route で fan-out できる。

#### 欠点

- emitted route は test/support helper-local evidence に留まり、public contract にはまだ上がらない。

### 案 2. `run_current_l2_source_sample` 自体の report shape を widen して downstream artifact を直に持たせる

#### 利点

- sample-facing surface は直観的に見えやすい。

#### 欠点

- helper-local ratchet を越えて public/report shape を早く凍らせやすい。
- public operational surface later gate を崩しやすい。

### 案 3. regression helper / Python orchestration から first integration を始める

#### 利点

- bless/review flow と近い位置に置ける。

#### 欠点

- sample runner / ladder / formal hook / review-unit / model-check carrier の call chain が code 上で分散しやすい。
- repo-local orchestration contract を premature に太らせやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. runtime test/support helper-local route で source sample runner から formal hook / review-unit / model-check carrier へ fan-out する**
である。

理由は次の通り。

1. current public/runtime surface を変えずに source-backed emitted route evidence を足せる。
2. valid/malformed/guarded split を current formal-hook top と一致させたまま、sample-visible theorem/model-check line を 1 package 進められる。
3. bless/review flow は docs-first next package に分ける方が drift が少ない。

## current first choice details

- actual route は `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs` の helper-local transform に留める。
- entry は `run_current_l2_source_sample(sample_argument, host_plan)` と `CurrentL2SourceSampleRunReport` に留める。
- `static gate = valid` row は fixture-aligned detached bundle artifact を経由して formal hook を作る。
- `static gate = malformed` row は fixture-static gate artifact を経由して formal hook を作る。
- `static gate = underdeclared` は current wiring floor の外として fail-closed に止める。
- current formal hook build が guard で止まる row は `GuardedNotReached` とし、followup artifact は空 list に留める。
- `e3` は current `runtime_try_cut_cluster` family の外にあるため、guarded row の代表としてそのまま維持する。

## next promoted line

next promoted line は、
**source-sample-emitted-verification-artifact-wiring-ready sample-facing-theorem-model-check-evidence-summary-and-bless-review-flow**
に置く。

## open questions

- sample-facing summary で review-unit と model-check carrier をどの順で見せるか
- current bless/review flow で compare-ready bridge sketch を docs-only context としてどこまで見せるか
- emitted route を regression helper や later public contract へいつ widen するか
