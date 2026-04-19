# Report 0822 — Package 69 IFC public-checker entry-criteria ratchet

- Date: 2026-04-19 21:04 JST
- Author / agent: Codex
- Scope: Package 69 として source-side IFC trio `p10 / p11 / p12` の public checker API sketch threshold を public-checker entry criteria threshold まで ratchet し、helper-local operational CLI summary / specs / snapshot / traceability を同期する。
- Decision levels touched: L2

## 1. Objective

- `specs/examples/277` / `278` の docs-first public-checker entry criteria line を compare-floor のまま放置せず、current IFC trio に対して helper-local operational summary に narrow に mirror する。
- actual public checker command surface、shared output contract、parser-front public checker boundary、final public verifier contractは still later に残す。
- queue drift を起こさず、next self-driven package を public-checker command-surface ratchet として再構成する。

## 2. Scope and assumptions

- current package は helper-local / sample-local actualization のみを扱う。
- actualization 対象 sample は `p10 / p11 / p12` に限定する。
- public-checker entry criteria minimum は docs-first `277` / `278` の current first line に従い、
  `minimal API fixed + command-surface pressure present + comparison target narrowed + heavier boundary deferred`
  に留める。
- actual public checker command surface / shared output contract / parser-front public checker boundary / emitted verifier handoff surface / final public verifier contract はこの package では固定しない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
- `specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
- `specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
- `specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`
- `specs/examples/540-current-l2-ifc-public-checker-api-sketch-threshold-helper-mirror.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`
- `scripts/current_l2_detached_loop.py`

## 4. Actions taken

1. `crates/mir-runtime/tests/current_l2_operational_cli.rs` に先行して assertion を追加し、`actual_public_checker_entry_criteria_threshold` が未実装で赤になることを確認した。
2. `crates/mir-runtime/src/current_l2_cli.rs` に `actual_public_checker_entry_criteria_threshold` summary を追加し、`p10 / p11 / p12` だけで `reached`、それ以外は `guarded_not_reached` になる helper-local cut を実装した。
3. current minimum を
   `public_checker_api_ref + entry_criteria_refs + family_facade_support_ref + family_facade_script_refs + smoke_command_refs + next_comparison_target_ref + deferred_boundary_refs`
   に固定した。
4. `specs/examples/541` を追加し、Package 69 の current first line / stop line / retained later を明文化した。
5. `Documentation.md`、`tasks.md`、`progress.md`、`plan/`、`specs/`、`plan/90-source-traceability.md` を更新し、Package 69 close と Package 70 next を snapshot に同期した。

## 5. Files changed

- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/541-current-l2-ifc-public-checker-entry-criteria-threshold-helper-mirror.md`
- `docs/reports/0822-package69-ifc-public-checker-entry-criteria-ratchet.md`
- `Documentation.md`
- `tasks.md`
- `progress.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

## 6. Evidence / outputs / test results

1. `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact`
   - red phase: assertion failed because `value["actual_public_checker_entry_criteria_threshold"]["status"]` was `Null`
   - green phase: passed after implementation
2. `cargo test -p mir-runtime --test current_l2_operational_cli`
   - `16 passed`
3. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format json`
   - `actual_public_checker_entry_criteria_threshold.status == "reached"`
   - `public_checker_api_ref == "public_checker_api_ready_sketch"`
   - `smoke_command_refs` includes `smoke-same-lineage-checker`
   - `next_comparison_target_ref == "public_checker_command_surface_comparison"`
4. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format pretty`
   - pretty output includes `actual_public_checker_entry_criteria_threshold`
   - `public_checker_api_ref: public_checker_api_ready_sketch`
   - `smoke-same-lineage-checker`
   - `next_comparison_target_ref: public_checker_command_surface_comparison`
5. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt --format json`
   - `actual_public_checker_entry_criteria_threshold.status == "reached"`
   - `public_checker_api_ref == "public_checker_api_ready_sketch"`
   - `next_comparison_target_ref == "public_checker_command_surface_comparison"`
6. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json`
   - `actual_public_checker_entry_criteria_threshold.status == "guarded_not_reached"`
7. `python3 scripts/validate_docs.py`
   - `Documentation scaffold looks complete.`
   - `Found 821 numbered report(s).`
8. `git diff --check`
   - no output

## 7. What changed in understanding

- `actual_public_checker_api_sketch_threshold` の次段として `actual_public_checker_entry_criteria_threshold` を operational CLI helper summary に切っても、actual command surface や shared output contract を premature に固定せずに済む。
- current first line は `public_checker_api_ref + entry_criteria_refs + family_facade_support_ref + family_facade_script_refs + smoke_command_refs + next_comparison_target_ref + deferred_boundary_refs` で十分であり、family facade bundle 自体や detached-loop wrapper 全体を command surface ready sketch と同一視しない方が `specs/examples/278` と整合する。
- current helper-local actualization 対象は `p10 / p11 / p12` に絞るのが自然であり、IFC trio の outside を同一 threshold に広げない方が stop line が明瞭である。
- next self-driven package は `specs/examples/279` / `280` に anchored した public-checker command-surface ratchet へ narrow に移せる。

## 8. Open questions

- public checker command surface ready sketch を helper-local threshold に actualize してよいか。
- detached-loop `smoke-*` wrapper を current command-surface ready sketch に含めるべきか、それとも evidence 側に残すべきか。
- shared output contract / parser-front public checker boundary / emitted verifier handoff surface を later mixed gate のどの reopen order に置くか。

## 9. Suggested next prompt

- Package 70 として public-checker command-surface ratchet を進め、`specs/examples/279` / `280` の current first line を `run-source-sample` helper summary に narrow に mirror してください。family facade command bundle を current cut に留め、detached-loop `smoke-*` wrapper / shared output contract / parser-front public checker boundary は still later に残してください。
