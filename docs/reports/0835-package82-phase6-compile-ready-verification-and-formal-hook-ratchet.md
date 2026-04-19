# 0835 Package 82 phase6-compile-ready-verification-and-formal-hook ratchet

## Objective

Phase 6 の current next line である compile-ready verification / formal hook を、
`mir-runtime` の narrow manifest と `run-source-sample` helper summary の両方で
source-backed に actualize し、queue を `phase6_next_reopen_sequencing` へ進める。

## Scope and assumptions

- 規範判断の正本は `specs/` を使う。
- Package 82 では selected cargo / smoke gate、tool-neutral formal hook shape、
  detached artifact validation guard の narrow actualization だけを扱う。
- `concrete_theorem_tool_binding`、`concrete_model_check_tool_binding`、
  `parser_second_tranche_widen`、`final_public_surface` は still later に残す。
- compile-ready checkpoint close は repo-local CLI / tests / emitted artifact / smoke floor の意味に留め、
  final public parser/checker/runtime surface には上げない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
- `specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-semantics/examples/current_l2_emit_formal_hook.rs`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
- `crates/mir-semantics/tests/current_l2_formal_hook_support.rs`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`

## Actions taken

1. `mir-runtime::current_l2` に
   `current_l2_compile_ready_verification_and_formal_hook_manifest()` と
   `CURRENT_L2_COMPILE_READY_VERIFICATION_AND_FORMAL_HOOK_MANIFEST`
   を追加し、
   Phase 6 compile-ready checkpoint の current cut
   `verification_gate_refs + smoke_gate_refs + formal_hook_shape + source_artifact_refs + validation_refs + retained_later_refs`
   を crate 本体で inspectable にした。
2. `crates/mir-runtime/tests/current_l2_compile_ready_formal_hook_manifest.rs`
   を追加し、selected cargo gate、smoke gate、formal-hook row core、
   validation guard、retained-later line が drift しないことを固定した。
3. `crates/mir-runtime/src/current_l2_cli.rs` に
   `actual_phase6_compile_ready_verification_and_formal_hook_threshold`
   を追加し、`p07 / p08 / p09` で reached、その他では guard-only になる helper mirror を actualize した。
4. `crates/mir-runtime/tests/current_l2_operational_cli.rs` を更新し、
   p06 guard-only、p07 pretty、p08/p09 json の reached/next-line を固定した。
5. compile-ready gate の repo-local evidence として、
   `cargo test -p mir-ast`、
   `cargo test -p mir-runtime ...`、
   `cargo test -p mir-semantics ...`、
   `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop`、
   `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static ...`、
   `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime ...`
   を通した。
6. `specs/examples/554`、decision register、snapshot docs、traceability を
   Package 82 close / Package 83 next 読みに更新した。
7. `specs/examples/303` / `304` の next promoted line wording drift を修正し、
   `phase6_next_reopen_sequencing` へ揃えた。

## Files changed

- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_compile_ready_formal_hook_manifest.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
- `specs/examples/554-current-l2-phase6-compile-ready-verification-and-formal-hook-threshold-helper-mirror.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Commands run

- `cargo fmt --all`
- `cargo test -p mir-ast`
- `cargo test -p mir-runtime --test current_l2_compile_ready_formal_hook_manifest current_l2_compile_ready_formal_hook_manifest_keeps_minimum_cut -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
- `cargo test -p mir-runtime --test current_l2_compile_ready_formal_hook_manifest --test current_l2_checker_runtime_first_tranche_manifest --test current_l2_runtime_skeleton --test current_l2_operational_cli`
- `cargo test -p mir-semantics --test current_l2_minimal_interpreter --test current_l2_static_gate_support --test current_l2_detached_bundle_support --test current_l2_formal_hook_support`
- `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e5-underdeclared-lineage --artifact-root target/current-l2-detached --run-label package82-static --overwrite`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root target/current-l2-detached --run-label package82-runtime --overwrite`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format pretty`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- RED
  - `cargo test -p mir-runtime --test current_l2_compile_ready_formal_hook_manifest ... --exact`
    - `no current_l2_compile_ready_verification_and_formal_hook_manifest in current_l2`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - `actual_phase6_compile_ready_verification_and_formal_hook_threshold` が `Null`
- GREEN
  - `cargo test -p mir-runtime --test current_l2_compile_ready_formal_hook_manifest ... --exact`
    - 1 passed
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_stale_reconnect_refresh_prototype -- --exact`
    - 1 passed
  - `cargo test -p mir-ast`
    - 56 passed
  - `cargo test -p mir-runtime --test current_l2_compile_ready_formal_hook_manifest --test current_l2_checker_runtime_first_tranche_manifest --test current_l2_runtime_skeleton --test current_l2_operational_cli`
    - 24 passed
  - `cargo test -p mir-semantics --test current_l2_minimal_interpreter --test current_l2_static_gate_support --test current_l2_detached_bundle_support --test current_l2_formal_hook_support`
    - 61 passed
  - `python3 -m unittest scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_detached_loop`
    - 28 passed
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
  - `git diff --check`
    - no output
  - `smoke-formal-hook-static`
    - `target/current-l2-detached/formal-hooks/package82-static/e5-underdeclared-lineage.formal-hook.json`
  - `smoke-formal-hook-runtime`
    - `target/current-l2-detached/formal-hooks/package82-runtime/e2-try-fallback.formal-hook.json`
- Reached helper mirror expectations
  - `p07` pretty:
    - `actual_phase6_compile_ready_verification_and_formal_hook_threshold:`
    - `formal_hook_artifact_kind_ref: current_l2_tool_neutral_formal_hook`
    - `verification_gate_refs` に selected cargo / unittest gate が並ぶ
    - `smoke_gate_refs = [smoke_formal_hook_static, smoke_formal_hook_runtime]`
    - `next_comparison_target_ref: phase6_next_reopen_sequencing_comparison`
  - `p09` json:
    - `status = reached`
    - `formal_hook_subject_kind_refs = [fixture_static_cluster, runtime_try_cut_cluster]`
    - `formal_hook_obligation_kind_refs = [canonical_normalization_law, no_re_promotion, rollback_cut_non_interference]`
    - `validation_refs = [input_schema_version_guard, input_artifact_kind_guard]`
  - `p06` guard-only
    - `status = guarded_not_reached`

## What changed in understanding

- Phase 6 compile-ready checkpoint の本質は、
  theorem/model-check concrete binding を先回りで入れることではなく、
  selected gate と tool-neutral formal hook entry criteria を source-backed helper mirror に束ねることだった。
- したがって Package 82 の最小実装は、
  new public verifier contract を作ることでも parser second tranche を reopen することでもなく、
  compile-ready gate / smoke / emitted hook row core / detached validation guard を
  narrow manifest と sample-local summary に切り出すことだった。
- この整理により、next line は `phase6_next_reopen_sequencing` へ自然に送れるようになった。

## Open questions

- Package 83 で `phase6_next_reopen_sequencing` をどこまで helper-local threshold に actualize するか。
- parser second tranche reopen の first package を attached-slot / predicate fragment にどこまで絞るか。
- theorem-first concrete tool binding reserve path を snapshot でどこまで mirror するか。

## Suggested next prompt

Package 83 として `specs/examples/305...306` を anchor に
phase6-next-reopen-sequencing ratchet を進め、
parser second tranche first / theorem-first reserve / model-check second reserve の sequencing minimum を helper-local threshold と snapshot docs に actualize してください。
