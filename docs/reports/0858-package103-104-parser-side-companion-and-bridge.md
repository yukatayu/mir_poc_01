# 0858 — Package 103 / 104 parser-side companion and bridge

## Objective

Package 103 parser-side companion surface bundle と Package 104 parser-side bundle-to-helper bridge を、
representative companion sample、`mir-ast` parse test、guided bundle helper、snapshot sync まで含めて actualize する。

## Scope and assumptions

- parser-side companion sample は helper-local / non-production の reader aid に留める。
- representative slice は `p06`、`p07`、`p08` に限定する。
- final grammar、final public parser/checker/runtime API、final public verifier contract は still later に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/472-current-l2-minimal-companion-experimental-order-handoff-surface.md`
- `specs/examples/564-current-l2-phase6-perform-head-request-clause-bundle-attachment-comparison.md`
- `specs/examples/565-current-l2-phase6-perform-head-request-clause-bundle-thin-wrapper-threshold-helper-mirror.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## Actions taken

1. `samples/prototype/current-l2-parser-companion/` を追加し、`p06 / p07 / p08` representative slice の parser-side companion sample と日本語 README を置いた。
2. `crates/mir-ast/tests/current_l2_stage3_request_head_clause_bundle_sample_bundle.rs` を追加し、companion sample が `Stage3RequestHeadClauseBundle` へ parse できることを machine-check した。
3. `scripts/current_l2_guided_samples.py bundle problem1|problem2` に `parser_companion_path` を追加し、prototype / parser companion / Lean artifact を 1 画面で辿れる bridge を actualize した。
4. `scripts/tests/test_current_l2_guided_samples.py` を更新し、bundle helper が parser companion path を落とさないことを RED/GREEN で確認した。
5. `specs/examples/577` と `578` を追加し、snapshot / roadmap / traceability / queue を Package 105 / 106 へ同期した。

## Files changed

- `crates/mir-ast/tests/current_l2_stage3_request_head_clause_bundle_sample_bundle.rs`
- `samples/prototype/current-l2-parser-companion/README.md`
- `samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt`
- `samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt`
- `samples/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt`
- `samples/prototype/current-l2-typed-proof-model-check/README.md`
- `samples/prototype/current-l2-order-handoff/README.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md`
- `specs/examples/578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## Commands run

- `cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_sample_bundle`
- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- `python3 scripts/current_l2_guided_samples.py bundle problem1`
- `python3 scripts/current_l2_guided_samples.py bundle problem2`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- `cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_sample_bundle`
  - 2 tests passed
- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - 12 tests passed
- `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `p06` representative slice の parser companion path を含む bundle を確認
- `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - `p07 / p08` representative pair の parser companion path を含む bundle を確認
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - whitespace / conflict marker なし

## What changed in understanding

- parser-side carrier は既にあったが、representative sample と直接結び付く companion surface sample が無かったため、Package 103 の first slice は sample file と test で閉じるのが自然だった。
- `bundle` helper に parser companion path を入れることで、Package 104 は新しい public API を増やさずに close できた。
- 次の parser-side line は、companion sample の parse result 自体を JSON / CLI summary として inspectable にする small inspector package が自然である。

## Open questions

- parser companion inspector を `mir-ast` example として置くか、Python helper として置くか。
- reserve / negative line の parser companion widening をどの順で reopen するか。

## Suggested next prompt

Package 105 / 106 として、parser-side companion sample の parse result inspector と、companion sample / original prototype / bundle helper の mapping matrix を self-driven に actualize してください。 final grammar / final public parser API へは上げないでください。
