# Report 0713 — Macro 0-4 closeout via duplicate-cluster actualization

- Date: 2026-04-17T09:12:56+09:00
- Author / agent: Codex
- Scope: duplicate-cluster source-authored static-stop pair `e14/e15` を current fixed-subset sample corpus に actualize し、execution lane を `Macro 0〜4 closeout fixed` に更新する
- Decision levels touched: L2 / L3

## 1. Objective

`specs/examples/438` で comparison として残していた duplicate-cluster widening のうち、current fixed-subset widening closeout に必要な最小 pair-first cut を source-authored actualization まで進める。あわせて runner / lowering / verification ladder / emitted artifact wiring / regression helper と docs mirror を同期し、execution lane を `Macro 0〜4 closeout fixed` に揃える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/417-current-l2-stable-malformed-capability-second-source-backed-widening-actualization.md`
- `specs/examples/430-current-l2-malformed-duplicate-cluster-later-reopen-comparison.md`
- `specs/examples/438-current-l2-malformed-duplicate-cluster-source-sample-widening-comparison.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

前提:

- `atomic_cut` / malformed family / typed promotion の broader reopen は今回の task で事実化しない。
- duplicate cluster の current cut は `e14/e15` pair-first widening に限定し、duplicate reason-code promotion や nested try/fallback variants は kept-later に残す。

## 3. Actions taken

1. `samples/current-l2/` に duplicate option-side / chain-side malformed pair として
   - `e14-malformed-duplicate-option-declaration.txt`
   - `e15-malformed-duplicate-chain-declaration.txt`
   を追加した。
2. `crates/mir-runtime/src/current_l2.rs` の accepted sample set に `e14/e15` を追加し、current-L2 runner が source-authored malformed pair を明示的に扱えるようにした。
3. `mir-runtime` tests に
   - lowering exact-match
   - source sample runner exact acceptance
   - verification ladder static-stop / formal-hook reachability
   - emitted artifact wiring to review-unit / model-check carrier
   の各 focused test を追加した。
4. 途中で lowering の expectation drift を発見し、`stage1_reconnect_clusters` は `None` ではなく `Some` で、3 floor がすべて `false` になる current behavior を正として ratchet した。
5. `scripts/current_l2_source_sample_regression.py` と `scripts/tests/test_current_l2_source_sample_regression.py` を更新し、authored corpus / smoke bundle / regression order を authored fourteen に widen した。
6. `specs/examples/443...` を追加して、duplicate-cluster source-authored static-stop pair actualization を current cut として記録した。
7. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、research abstract、sample docs を更新し、execution lane を `Macro 0〜4 closeout fixed` に揃えた。

## 4. Files changed

### Production / sample / helper

- `samples/current-l2/e14-malformed-duplicate-option-declaration.txt`
- `samples/current-l2/e15-malformed-duplicate-chain-declaration.txt`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`

### Normative / memory / snapshot

- `specs/examples/443-current-l2-malformed-duplicate-cluster-source-authored-static-stop-pair-actualization.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_005.md`
- `docs/reports/0713-macro0-4-closeout-via-duplicate-cluster-actualization.md`

更新不要:

- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `AGENTS.md`

## 5. Commands run and exact outputs

### Resource / baseline

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `/dev/vda2 99G 77G 18G 82% /`
- `free -h`
  - `Mem: 960Mi total / 840Mi used / 72Mi free / 119Mi available / 19Gi swap`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-17 08:51:56 JST`

### Focused red/green while implementing

- `cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_matches_e14_fixture_and_duplicate_option_static_stop -- --exact`
  - initial red: `No such file or directory`
  - final green: `ok`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_named_e14_sample -- --exact`
  - initial red: `source sample not found: e14-malformed-duplicate-option-declaration`
  - final green: `ok`
- `cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_matches_e15_fixture_and_duplicate_chain_static_stop -- --exact`
  - initial red on expectation drift, then final green after aligning `stage1_reconnect_clusters`
- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionInventoryTests.test_inventory_rows_cover_authored_and_deferred_entries scripts.tests.test_current_l2_source_sample_regression.SourceSampleRegressionPlanningTests.test_plan_regression_commands_uses_expected_order_and_smoke_plumbing`
  - initial red while script still assumed authored twelve
  - final green: `OK`

### Final verification

- `cargo test -p mir-runtime --test current_l2_source_lowering`
  - `14 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `16 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder`
  - `14 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring`
  - `7 passed`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - `5 passed`
- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `Ran 13 tests in 0.010s`
  - `OK`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - authored inventory rows `14`
  - duplicate pair `e14/e15` both `source-authored / malformed / not_evaluated / fixture_static_cluster / present`
- `python3 scripts/current_l2_source_sample_regression.py regression --run-label macro0-4-closeout --artifact-root target/current-l2-source-sample-regression-macro0-4-closeout`
  - `all regression commands passed`
  - runtime/static formal-hook smoke bundle completed for all `18` planned commands
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 712 numbered report(s).`
- `git diff --check`
  - 無出力

## 6. Evidence / findings

- duplicate-cluster widening can be closed for the current fixed-subset scope with exactly two malformed static-stop rows:
  - `e14`: duplicate option declaration
  - `e15`: duplicate chain declaration
- current runtime / ladder behavior is:
  - accepted by sample runner
  - lowered to static malformed form
  - not evaluated
  - no terminal runtime outcome
  - `formal_hook` reaches `fixture_static_cluster`
  - emitted artifact wiring can still produce review units and model-check carriers
- the current lowering does not suppress `stage1_reconnect_clusters`; instead it keeps the structure with all three floor booleans false. This is now documented by tests.
- authored sample corpus now widens from twelve to fourteen without reopening broader try-rollback malformed-static family.
- with `e14/e15` actualized, the execution lane can be read as `Macro 0〜4 closeout fixed` in the repo’s current scoped sense.

## 7. Changes in understanding

- `Macro 4` closeout should not be read as “all malformed family resolved”; it means the **current fixed-subset widening line** is complete enough to stop promoting new malformed rows on the execution lane.
- source-authored malformed rows do not need to be runtime-success cases to count as execution-lane progress; static-stop rows that traverse runner / ladder / emitted artifact paths are first-class current evidence.
- duplicate-cluster line now matches the same ratchet pattern used earlier for missing-option and capability families: pair-first authored widening now, broader family later.

## 8. Open questions

- broader try-rollback malformed-static family reopen criteria
- duplicate reason-code promotion or typed promotion
- nested-scope / request / predicate / try-connected duplicate variants
- whether `stage1_reconnect_clusters` should later gain a dedicated duplicate-family summary bit
- final malformed catalog and public presentation

## 9. Suggested next prompt

`では引き続き、tasks.md 先頭の modality internalization trigger note と、その後ろの theory-lab / reserve-integration queue を自走でお願いします。`
