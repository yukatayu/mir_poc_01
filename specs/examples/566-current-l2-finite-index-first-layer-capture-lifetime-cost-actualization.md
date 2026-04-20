# 566. current L2 finite-index first layer capture / lifetime / cost actualization

## 目的

`specs/examples/557` で source-backed に選んだ
first strong typing layer の current default を、
IFC trio だけの docs reading で止めず、
**capture / lifetime negative と simple cost negative を
current runnable prototype と checker-adjacent helper summary まで actualize する**。

ここで actualize するのは final typed calculus でも
final typed source principal でも final public verifier contract でもない。

## current first line

- principal は checker-adjacent first layer に置く。
- `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` は conceptual spine として維持する。
- first-class target は
  - IFC / taint
  - capture / lifetime
  - simple cost
  と読む。
- stronger typed surface は current source principal に早期昇格しない。

## 今回 actualize したもの

### corrected prototype

- `p15-typed-capture-escape-rejected`
  - ephemeral token を capture scope の外へ publish しようとして止まる。
  - current reading は `capture_escape_negative`。
- `p16-typed-remote-call-budget-exceeded`
  - remote call budget が尽きた後の追加 call を止める。
  - current reading は `remote_call_budget_negative`。

### helper-local checker summary widening

`run-source-sample` の helper-local operational summary は、
従来の IFC trio `p10 / p11 / p12` だけでなく、
次の **first strong typing sample set** を current actualization floor として読める。

- `p10-typed-authorized-fingerprint-declassification`
- `p11-typed-unauthorized-fingerprint-release`
- `p12-typed-classified-fingerprint-publication-block`
- `p15-typed-capture-escape-rejected`
- `p16-typed-remote-call-budget-exceeded`

current cut では、次を helper-local threshold に留めて actualize してよい。

- `typed_checker_hint_preview`
- `actual_checker_payload_family_threshold`
- `actual_checker_payload_row_family_threshold`
- `actual_checker_payload_row_detail_threshold`
- `actual_checker_payload_row_body_threshold`
- `actual_checker_payload_supported_kind_summary_threshold`
- `actual_checker_payload_public_schema_sketch_threshold`
- `actual_public_checker_api_sketch_threshold`
- `actual_public_checker_entry_criteria_threshold`
- `actual_public_checker_command_surface_threshold`
- `actual_shared_output_contract_threshold`
- `actual_public_checker_boundary_threshold`
- `actual_verifier_handoff_surface_threshold`
- `actual_minimal_parser_subset_freeze_threshold`
- `actual_parser_to_checker_reconnect_freeze_threshold`
- `actual_phase1_semantics_closeout_threshold`
- `actual_phase2_parser_free_poc_closeout_threshold`

## evidence

- source runner
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- verifier preview alignment
  - `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
- model-check projection pre-floor
  - `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
- operational CLI helper summary
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- prototype samples
  - `samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt`
  - `samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt`

## retained alternatives

- stronger typed source principal
- final capture / lifetime source syntax
- final simple cost source syntax
- theorem-side richer cost proof
- full dependent core

## stop line

今回ここで止めてよいもの:

- final typed source principal
- final typed calculus
- final public checker payload schema
- final public verifier contract
- Lean generated corpus への即時昇格

## next package

次の current package は Package 93 である。

- `samples/lean/foundations/` の actual small proof fragment
- `samples/lean/current-l2/` の generated stub corpus
- 日本語 explanation

の役割差を保ったまま、
first strong typing layer と theorem-side placeholder 群の drift を閉じる。
