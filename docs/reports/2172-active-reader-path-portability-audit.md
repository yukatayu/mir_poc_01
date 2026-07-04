# Report 2172 — Active reader path portability audit

- Date: 2026-07-04 12:13 JST
- Author / agent: Codex
- Scope: active reader-facing path-portability cleanup after Product Alpha / Full System V1 path hardening
- Decision levels touched: LAB maintenance only; no normative decision changed

## Objective

Audit tracked host-specific absolute paths after the Product Alpha and Full
System V1 path-portability packages, remove the remaining active
reader-facing `/home/yukatayu/...` paths without rewriting historical evidence,
and harden the active generators whose documented `actual output` snippets
still depended on the host checkout path.

## Scope and assumptions

Scope was limited to active reader-facing docs, active sample README text, and
the generators that produce the affected clean-near-end / Lean manifest path
surfaces.

Out of scope:

- `docs/reports/` historical reports
- `docs/research_abstract/old/` archived research abstracts
- `samples/lean/old/` archived generated bundles
- `tmp_faq/` historical scratch/FAQ material
- `/home/codex/.codex/...` Oracle operation paths in `AGENTS.md` / `.docs/`,
  because those are current environment operation references, not repo source
  paths
- test strings such as `/Users/` that intentionally exercise path scrub guards

## Start state / dirty state

Package 34 started from clean `HEAD == origin/main == a6a7e90f`.

`git ls-files` host-path inventory showed historical report/archive hits plus a
small active set:

- `docs/research_abstract/clean_near_end_lean_01_detail.md`
- `docs/research_abstract/clean_near_end_modal_01_detail.md`
- `docs/research_abstract/clean_near_end_order_model_01_detail.md`
- `docs/research_abstract/clean_near_end_typing_01_detail.md`
- `samples/current-l2/README.md`

Reviewer follow-up then identified two emitting surfaces behind those snippets:

- `crates/mir-runtime/src/clean_near_end.rs`
- `scripts/current_l2_lean_sample_sync.py`

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `docs/research_abstract/clean_near_end_lean_01_detail.md`
- `docs/research_abstract/clean_near_end_modal_01_detail.md`
- `docs/research_abstract/clean_near_end_order_model_01_detail.md`
- `docs/research_abstract/clean_near_end_typing_01_detail.md`
- `samples/current-l2/README.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Actions taken

- Replaced active clean-near-end research abstract sample `source_path` values
  from host-specific `/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/...`
  form to repo-relative `samples/...` form.
- Replaced the clean-near-end Lean manifest output path with
  `samples/lean/manifest.json`.
- Replaced the `samples/current-l2/README.md` authoring-policy link with a
  repository-relative `../../.docs/current-l2-source-sample-authoring-policy.md`
  link.
- Updated `mir-clean-near-end` clean sample reports / closeout roots to display
  repo-owned paths relative to the repository root.
- Updated `scripts/current_l2_lean_sample_sync.py` to print
  `samples/lean/manifest.json` instead of the host absolute manifest path.
- Added an active reader-facing host-path lint to `scripts/validate_docs.py`
  with tests. The lint rejects `/home/<user>/dev/mir_poc_01` and matching
  `/Users/.../dev/mir_poc_01` repo paths in active reader docs/samples while
  preserving the historical/archive/external-operation exclusions. The active
  sample scan is intentionally limited to active sample roots rather than all
  `samples/**`.
- Updated current snapshots to record this as maintenance only.

## Files changed

- `docs/research_abstract/clean_near_end_lean_01_detail.md`
- `docs/research_abstract/clean_near_end_modal_01_detail.md`
- `docs/research_abstract/clean_near_end_order_model_01_detail.md`
- `docs/research_abstract/clean_near_end_typing_01_detail.md`
- `samples/current-l2/README.md`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2172-active-reader-path-portability-audit.md`

## Commands run

- `git ls-files | xargs rg -n --pcre2 '/home/(codex|yukatayu)|/Users/'`
- `git ls-files | rg -v '^docs/reports/' | xargs rg -n --pcre2 '/home/(codex|yukatayu)|/Users/'`
- `rg -n '/home/yukatayu/dev/mir_poc_01|/home/codex/dev/mir_poc_01|/Users/' docs/research_abstract/clean_near_end_*_detail.md samples/current-l2/README.md`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_active_reader_host_absolute_repo_paths scripts.tests.test_validate_docs.ValidateDocsTests.test_main_allows_historical_host_paths_outside_active_reader_lint` (RED)
- `cargo test -p mir-runtime --test clean_near_end_samples clean_sample_authorized_declassification_passes -- --nocapture` (RED for repo-relative `source_path`)
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_main_prints_repo_relative_manifest_path` (RED)
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_active_reader_host_absolute_repo_paths scripts.tests.test_validate_docs.ValidateDocsTests.test_main_allows_historical_host_paths_outside_active_reader_lint` (GREEN)
- `cargo test -p mir-runtime --test clean_near_end_samples clean_sample_authorized_declassification_passes -- --nocapture` (GREEN)
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync.CurrentL2LeanSampleSyncTests.test_main_prints_repo_relative_manifest_path` (GREEN)
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_declassification --format json`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `python3 -m py_compile scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- `python3 -m py_compile scripts/validate_docs.py scripts/tests/test_validate_docs.py scripts/current_l2_lean_sample_sync.py scripts/tests/test_current_l2_lean_sample_sync.py`
- `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_current_l2_lean_sample_sync`
- `cargo test -p mir-runtime --test clean_near_end_samples -- --nocapture`
- `cargo fmt --check`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 -m unittest discover -s scripts/tests`
- `git diff --check`
- `rg -n '/home/yukatayu/dev/mir_poc_01|/home/codex/dev/mir_poc_01|/Users/' docs/research_abstract/clean_near_end_*_detail.md samples/current-l2/README.md`

## Evidence / outputs / test results

- Active target scan returned no remaining
  `/home/yukatayu/dev/mir_poc_01`, `/home/codex/dev/mir_poc_01`, or `/Users/`
  hits in the five touched active reader files.
- Spot checks showed:
  - `samples/lean/manifest.json`
  - `samples/clean-near-end/modal/01_stage_stable_later_minimal.mir`
  - `samples/clean-near-end/order-handoff/01_authorized_roll_publish_handoff.mir`
  - `samples/clean-near-end/typing/01_authorized_declassification.mir`
  - `../../.docs/current-l2-source-sample-authoring-policy.md`
- `mir-clean-near-end run-sample 01_authorized_declassification --format json`
  emitted `source_path:
  "samples/clean-near-end/typing/01_authorized_declassification.mir"`.
- `scripts/current_l2_lean_sample_sync.py` printed
  `samples/lean/manifest.json` and did not change committed generated files.
- TDD guard evidence:
  - The active reader-host-path test failed before implementation because
    `validate_docs.py` did not reject the active `/home/yukatayu/dev/mir_poc_01`
    sample path. After reviewer follow-up, the same test was broadened to
    `/home/alice/dev/mir_poc_01` and failed until the Linux pattern was
    generalized.
  - The historical exclusion test was broadened with `samples/old/` evidence;
    it stayed outside the active sample lint after the sample root list was
    narrowed.
  - The clean-near-end Rust test failed before implementation because
    `source_path` still included the host checkout path; it passed after the
    runtime display helper was added.
  - The Lean sync stdout test failed before implementation because `main()`
    printed the absolute manifest path; it passed after printing through
    `repo_relative_source_path`.
  - After adding the lint, the active rejection test and historical exclusion
    test both passed.
- Final validation:
  - Targeted combined Python unit tests: 44 tests passed.
  - `cargo test -p mir-runtime --test clean_near_end_samples -- --nocapture`:
    27 tests passed.
  - `cargo fmt --check`: passed.
  - `scripts/check_source_hierarchy.py`: required/present 602/602.
  - `python3 scripts/validate_docs.py` initially caught this package's own
    exact lint-pattern wording in `progress.md`; the wording was changed to a
    non-exact description and the validator then passed, finding 1324 numbered
    reports.
  - `python3 -m unittest discover -s scripts/tests`: 676 tests passed.
  - `git diff --check`: passed after the final report/timestamp edits.
  - Active target host-path scan returned no hits.

## What changed in understanding

Remaining host-specific absolute paths are mostly historical evidence or
external operation references. The active portability cleanup should stay
narrow: fix active generators and reader-facing active sample paths, but do not
rewrite historical reports or archived generated bundles unless a later
migration package explicitly reclassifies them.

## Open questions

- Whether archived `samples/lean/old/` bundles should be regenerated or left as
  historical byte-for-byte evidence remains intentionally open.
- Whether `tmp_faq/` should remain tracked historical scratch material is a
  separate repository-organization question.

## Suggested next prompt

Continue autonomous maintenance from `tasks.md`; if staying on portability,
prefer adding a targeted lint for active reader-facing paths rather than
rewriting historical archives.

## Plan update status

`plan/` 更新不要:

- No roadmap, semantics, or repository-memory decision changed.

## Documentation.md update status

`Documentation.md` 更新不要:

- Root reader navigation did not change.

## progress.md update status

`progress.md` 更新済み:

- Added a recent log entry for active clean-near-end/current-L2 reader path
  portability.

## tasks.md update status

`tasks.md` 更新済み:

- Added a current holding-state maintenance note and explicit out-of-scope
  classification.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- Added a docs-only recent validation log row; sample workflow status did not
  change.

## Reviewer findings and follow-up

Reviewer sub-agent findings and follow-up:

- High: active docs described snippets as `actual output`, but the
  clean-near-end runtime and Lean sync command still emitted host absolute
  paths. Fixed the emitters and added Rust/Python regression tests plus actual
  command checks.
- Medium: the lint/report claimed `/home/*/dev/mir_poc_01`, but the first
  regex only matched selected usernames. Broadened the regex and added an
  `/home/alice/dev/mir_poc_01` regression case.
- Medium: the first lint shape scanned all `samples/**/*.md|json` except
  `samples/lean/old/`, which could reject archived sample material. Narrowed
  the active sample root list and added historical `samples/old/` coverage.
- Low: this report initially said reviewer pending. Updated this section.

## Skipped validations and reasons

No skipped validations at this checkpoint. Heavy workspace-wide Cargo tests
were not rerun because this package touched one mir-runtime test target and
script validators, not shared Rust semantics beyond clean-near-end path
display. The affected Rust target, affected Python tests, docs validators, and
full `scripts/tests` discovery are run before commit.

## Commit / push status

Pending at report update; commit and push will be recorded in a follow-up report
update after validation.

## Sub-agent session close status

Reviewer sub-agent completed, findings were processed, and the session was
closed after the final local validation pass.
