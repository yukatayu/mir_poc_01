# Report 0817 — Package 64 IFC checker payload row-detail ratchet

- Date: 2026-04-19 19:49 JST
- Author / agent: Codex
- Scope: Package 64 として source-side IFC trio `p10 / p11 / p12` の checker payload row-family threshold を checker payload row-detail threshold まで ratchet し、helper-local operational CLI summary / specs / snapshot / traceability を同期する。
- Decision levels touched: L2

## 1. Objective

- `specs/examples/267` / `268` の docs-first checker payload row-detail line を compare-floor のまま放置せず、current IFC trio に対して helper-local operational summary に narrow に mirror する。
- actual checker row body、supported kind detail、final public checker artifact、final public verifier contractは still later に残す。
- queue drift を起こさず、next self-driven package を checker payload row-body ratchet として再構成する。

## 2. Inputs consulted

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
- `specs/examples/267-current-l2-minimal-checker-payload-row-family-ready-checker-payload-row-detail-comparison.md`
- `specs/examples/268-current-l2-checker-payload-row-detail-ready-minimal-checker-payload-row-detail-threshold.md`
- `specs/examples/269-current-l2-minimal-checker-payload-row-detail-ready-checker-payload-row-body-comparison.md`
- `specs/examples/270-current-l2-checker-payload-row-body-ready-minimal-checker-payload-row-body-threshold.md`
- `specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md`
- `specs/examples/524-current-l2-ifc-label-flow-negative-prototype-closeout-and-representative-lean-sample-set-widening.md`
- `specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md`
- `specs/examples/535-current-l2-ifc-checker-payload-row-family-threshold-helper-mirror.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. `crates/mir-runtime/tests/current_l2_operational_cli.rs` に先行して assertion を追加し、`actual_checker_payload_row_detail_threshold` が未実装で赤になることを確認した。
2. `crates/mir-runtime/src/current_l2_cli.rs` に `actual_checker_payload_row_detail_threshold` summary を追加し、`p10 / p11 / p12` だけで `reached`、それ以外は `guarded_not_reached` になる helper-local cut を実装した。
3. `payload_row_family_ref = actual_checker_payload_row_family`、`row_source_ref = fixture_checked_reason_codes`、`row_reason_kind = case_label reused token` を current minimal checker payload row-detail threshold として固定した。
4. `specs/examples/536` を追加し、Package 64 の current first line / stop line / retained later を明文化した。
5. `Documentation.md`、`tasks.md`、`progress.md`、`plan/`、`specs/`、`plan/90-source-traceability.md` を更新し、Package 64 close と Package 65 next を snapshot に同期した。

## 4. Files changed

- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `specs/examples/536-current-l2-ifc-checker-payload-row-detail-threshold-helper-mirror.md`
- `docs/reports/0817-package64-ifc-checker-payload-row-detail-ratchet.md`
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

## 5. Commands run and exact outputs

1. `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact`
   - red phase: assertion failed because `value["actual_checker_payload_row_detail_threshold"]["status"]` was `Null`
   - green phase: passed after implementation
2. `cargo test -p mir-runtime --test current_l2_operational_cli`
   - `16 passed`
3. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format json`
   - `actual_checker_payload_row_detail_threshold.status == "reached"`
   - `coverage_state == "partial_cluster"`
   - `payload_row_family_ref == "actual_checker_payload_row_family"`
   - `row_source_ref == "fixture_checked_reason_codes"`
   - `row_reason_kind == "authority_sensitive_success"`
4. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format pretty`
   - pretty output includes `actual_checker_payload_row_detail_threshold`
   - `payload_row_family_ref: actual_checker_payload_row_family`
   - `row_source_ref: fixture_checked_reason_codes`
   - `row_reason_kind: authority_miss_negative`
5. `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt --format json`
   - `actual_checker_payload_row_detail_threshold.status == "reached"`
   - `coverage_state == "full_cluster"`
   - `row_reason_kind == "classified_to_public_negative"`
6. `python3 scripts/validate_docs.py`
   - `Documentation scaffold looks complete.`
   - `Found 816 numbered report(s).`
7. `git diff --check`
   - no output

## 6. Evidence / findings

- `actual_checker_payload_row_family_threshold` の次段として `actual_checker_payload_row_detail_threshold` を operational CLI helper summary に切っても、actual checker row body や final public checker artifact を premature に固定せずに済む。
- current first line は `payload_row_family_ref + row_source_ref + row_reason_kind` で十分であり、actual row body や supported kind detail を同時投入しない方が `specs/examples/268` と整合する。
- current helper-local actualization 対象は `p10 / p11 / p12` に絞るのが自然であり、IFC trio の outside を同一 threshold に広げない方が stop line が明瞭である。
- current IFC trio では generic checker row enum を新設するより、already actualized な `case_label` token を `row_reason_kind` として再利用する方が source-backed であり、偽の row serializer を早期に固定しないで済む。
- next self-driven package は `specs/examples/269` / `270` に anchored した checker payload row-body ratchet へ narrow に移せる。

## 7. Changes in understanding

- Package 64 を閉じるのに actual checker row body や supported kind detail は不要である。
- `actual checker payload row detail` は docs-only comparison から helper-local actualization へ 1 段進められるが、それでも still `source + discriminator` bridge であって actual row body ではない。
- current live queue は zero ではなく、Package 65 checker payload row-body ratchet を source-backed に再投入できる。

## 8. Open questions

- checker payload row body を `row_body` variant-local slot bundle で helper-local actualization してよいか。
- `authority_sensitive_success` / `authority_miss_negative` / `classified_to_public_negative` を current row-detail token としてどこまで later checker payload side へ引き継げるか。
- actual row body を current phase でどこまで stable token として扱うか。
- final public checker artifact / final public verifier contract を later mixed gate のどの reopen order に置くか。

## 9. Suggested next prompt

- Package 65 として checker payload row-body ratchet を進め、`specs/examples/269` / `270` の current first line を `run-source-sample` helper summary に narrow に mirror してください。`p10 / p11 / p12` を reached、その他 sample を guard-only に保ち、supported kind detail / final public checker artifact / final public verifier contract は still later に残してください。
