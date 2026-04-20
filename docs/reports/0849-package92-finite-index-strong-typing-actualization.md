# 0849 — Package 92 finite-index strong typing actualization

## Objective

Package 92 として、first strong typing layer の current default を
IFC trio だけの reading で止めず、
capture / lifetime negative と simple cost negative を
current runnable prototype / checker-adjacent helper summary / docs まで actualize する。

## Scope and assumptions

- `specs/` を規範正本とし、今回の実装は final typed calculus ではない。
- stronger typed source principal は昇格しない。
- final public verifier contract は採らない。
- Package 93 で Lean-first formal skeleton hardening を続ける前提で、
  Package 92 は source-side finite-index first layer を閉じる。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/557-current-l2-first-strong-typing-layer-finite-index-spine-default.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/prototype/README.md`

## Actions taken

1. TDD で先に failing test を追加した。
   - source runner
   - verifier preview alignment
   - model-check projection pre-floor
   - operational CLI helper summary
2. `p15-typed-capture-escape-rejected` と
   `p16-typed-remote-call-budget-exceeded` を prototype として追加した。
3. `crates/mir-runtime/src/current_l2_cli.rs` の IFC-trio 固定 helper を、
   first strong typing sample set
   `p10 / p11 / p12 / p15 / p16`
   に widen した。
4. payload row-detail / row-body の sample-specific metadata を
   manifest helper に寄せ直した。
5. sample inventory / roadmap / progress / tasks / traceability を
   Package 92 close / Package 93 next に同期した。

## Evidence / outputs / test results

- failing test red
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_capture_and_cost_typed_prototype_paths`
    - sample file missing で fail
  - `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment verifier_preview_alignment_matches_emitted_route_for_capture_escape_typed_runtime_prototype`
    - source sample not found で fail
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_capture_escape_checker_hint_preview`
    - source sample not found で fail
- green after implementation
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_capture_and_cost_typed_prototype_paths`
  - `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment verifier_preview_alignment_matches_emitted_route_for_capture_escape_typed_runtime_prototype`
  - `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment verifier_preview_alignment_matches_emitted_route_for_cost_bound_typed_runtime_prototype`
  - `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor model_check_projection_prefloor_reaches_capture_escape_typed_runtime_prototype`
  - `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor model_check_projection_prefloor_reaches_cost_bound_typed_runtime_prototype`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_capture_escape_checker_hint_preview`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_cost_bound_checker_hint_preview`

## What changed in understanding

- Package 92 の最小 close に必要なのは、
  final typed surface ではなく
  finite-index first layer の sample-visible negative を
  current helper summary まで widen することだった。
- capture / lifetime と simple cost は、
  IFC trio の checker-adjacent route を壊さず、
  sample-local manifest widening で自然に actualize できた。
- Lean generated corpus への即時反映は不要であり、
  それは Package 93 の explanation hardening と一緒に進める方が歪みが少ない。

## Open questions

- `samples/lean/current-l2/` に `p15 / p16` をいつ追加するか。
  current recommendation は Package 93 で explanation と一緒に扱う。
- typed checker payload threshold 群の public-facing naming を
  final public surface にどう持ち上げるか。
  current cut では still helper-local。

## Suggested next prompt

Package 93 として、`samples/lean/foundations/` と `samples/lean/current-l2/` の役割差を保ちつつ、
`p15 / p16` を含む finite-index first layer の日本語 explanation と Lean-first skeleton drift を閉じてください。
