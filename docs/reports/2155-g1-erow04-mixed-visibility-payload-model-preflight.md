# Report 2155 - G1 ELAB-04 mixed visibility payload-model preflight

- Date: 2026-07-04 06:31 JST
- Author / agent: Codex
- Scope: LAB-only docs preflight for future `ELAB-04` mixed visibility repair
  payload decisions.
- Decision levels touched: `L3` LAB evidence / repository memory only.

## Objective

Record a docs-only preflight for future `ELAB-04` mixed visibility payload
decisions without changing executable output. The package names a conceptual
mixed wrapper, base remote-request branch, visibility branch, association
vocabulary, and ordering / ranking deferrals while keeping `ELAB-04`
no-repair.

## Scope and assumptions

The scope is limited to repository memory and status documentation. Rust
emission logic, expected JSON, sample row count, canon text, and Lean statement
files remain unchanged.

Working assumptions:

- canon in `mirrorea_canon/` remains normative;
- `ELAB-04` currently represents one rejected generated cross-locus read
  request with both base remote-request missing failures and `VisibilityDenied`;
- `E-ROW-001` is the current top-level LAB carrier for this detail, not a final
  ownership model for the mixed row;
- branch-local guidance must not be treated as a whole-row local-premise
  witness unless every branch is covered by an explicit grouped relation;
- child branches classify missing evidence only; they are not independent
  failed premises or emitted repair objects;
- any branch ordering in docs is editorial only and carries no ranking meaning;
- `ELAB-07` set-insertion evidence remains exact to the current `plan/102`
  fact pattern and must not be generalized to `ELAB-04`.

This package does not claim an `ELAB-04` executable payload, `ELAB-04` mixed
set-insertion support, general set-insertion support, bundle semantics,
partial-guidance output, visibility-repair ranking, repair ranking, multi-edit
support, final Diagnostic / repair ABI, OBL-024/025 proof, conformance, canon
movement, or G1 exit.

## Start state / dirty state

Package start:

- `HEAD = origin/main = 7977d2313e6faf3627086d55803da6ff90a1a4cf`
- notifier task baseline was recorded before package work;
- working tree was clean.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md`
- `mirrorea_canon/scenarios/SCN-02-attack.md`
- `mirrorea_canon/scenarios/SCN-05-portal.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md`
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md`
- `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md`
- `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/README.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Added `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated `plan/95`, `plan/96`, and `plan/98` to reference the new
  `ELAB-04` preflight and the completed `plan/103..106` `ELAB-07` guard
  chain.
- Updated `README.md`, `Documentation.md`,
  `docs/research_abstract/surface_mir_alpha_01.md`, `progress.md`,
  `tasks.md`, `samples_progress.md`, and the Surface elaboration sample
  README.
- Ran a sub-agent mapping pass for current `ELAB-04` executable facts and
  overclaim risks.
- Completed a ChatGPT Pro Oracle advisory review for the preflight. The first
  browser attempt failed before submission because attachment upload timed out;
  the second attempt used inline text delivery and returned advisory critique.

## Files changed

- `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/full-system-v1-surface/elaboration/README.md`
- `docs/reports/2155-g1-erow04-mixed-visibility-payload-model-preflight.md`

## Commands run

Commands run so far:

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M %Z'
ask-chatgpt-pro ... --file plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md --file plan/96-g1-erow-set-insertion-bundle-payload-inventory.md --file plan/98-g1-erow04-mixed-visibility-branch-inventory.md --file mirrorea_canon/theory/03-elaboration.md --file mirrorea_canon/theory/10-diagnostics.md --file mirrorea_canon/spec/07-diagnostics-format.md --file samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json
ask-chatgpt-pro --browser-attachments never --timeout 75m --slug "erow04 mixed payload" --write-output /tmp/oracle-erow04-mixed-payload-2155.md ...
cargo fmt --check
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_remote_request_when_failure_row_is_underdeclared -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_generated_write_request_when_failure_row_is_underdeclared -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples.SurfaceMirSamplesTests.test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic
python3 -m unittest scripts.tests.test_surface_mir_samples
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2155.json
jq '{sample_count, failed_count:(.failed|length), validation_error_count:(.validation_errors|length), elab04_has_repair: ([.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[]? | has("suggested_repair")] | any), elab04_missing: ((.results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[0].missing_evidence) // null), elab07_repair_shape: ((.results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair[0].repair_shape) // null), elab10_repair_count: ((.results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null), elab13_repair_count: ((.results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair | length) // null)}' /tmp/mirrorea-surface-check-all-2155.json
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
git ls-files --modified --others --exclude-standard | xargs -r rg -n --pcre2 '<endpoint-form pattern>' || true
git status --short --branch
git add Documentation.md README.md docs/research_abstract/surface_mir_alpha_01.md plan/00-index.md plan/90-source-traceability.md plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md plan/96-g1-erow-set-insertion-bundle-payload-inventory.md plan/98-g1-erow04-mixed-visibility-branch-inventory.md progress.md samples/full-system-v1-surface/elaboration/README.md samples_progress.md tasks.md docs/reports/2155-g1-erow04-mixed-visibility-payload-model-preflight.md plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md
git diff --cached --stat
git commit --no-gpg-sign -m "Add ELAB-04 mixed preflight"
git push
git rev-parse HEAD
```

The endpoint-form scan above records the redacted command shape. The local
command used the real endpoint-form regular expression without writing it into
this report. Final status commands are run after this bookkeeping update is
committed and pushed.

## Evidence / outputs / test results

Evidence so far:

- Sub-agent mapping confirmed the current `ELAB-04` detail is one rejected
  generated read request with `request_id = req-0001`,
  `request_kind = read`, `generated_from = cross_locus_read_expression`,
  `target_ref = when_fails_row|locus=role:BrowserClient|event=render`,
  declared `MissingCapability`, and missing `MissingWitness`,
  `RouteUnavailable`, `StaleMembership`, and `VisibilityDenied`.
- The same mapping confirmed current `ELAB-04` output omits
  `suggested_repair`; the omission is intentional because the current repair
  branches are mixed and under-specified, not because the row lacks a
  `VisibilityDenied` branch.
- The first Oracle browser attempt failed before a message was submitted:
  attachments did not reach a clickable send button before the browser
  attachment timeout.
- Oracle session `erow04-mixed-payload` returned advisory confirmation that a
  docs-only conceptual association model is sound if it does not leak into ABI,
  JSON shape, repair object, branch IDs, or OBL-024/025 witness language.
  Follow-up edits incorporated its main cautions:
  child branches classify missing evidence only, branch ordering is editorial,
  `ELAB-07` is the only current non-singleton set-insertion exception, and
  current `ELAB-04` no-repair omission must not be called final canon-format
  behavior.
- `cargo fmt --check` exited 0.
- Focused `ELAB-04` Rust test
  `rejects_generated_remote_request_when_failure_row_is_underdeclared`: 1
  passed / 0 failed.
- Existing exact `ELAB-07` positive test
  `rejects_generated_write_request_when_failure_row_is_underdeclared`: 1
  passed / 0 failed.
- Existing visibility singleton boundary test
  `rejects_visibility_only_failure_row_underdeclaration_with_erow_002_detail`:
  1 passed / 0 failed.
- Focused Python sample test:
  `test_elaboration_underdeclared_failure_row_negative_reports_expected_diagnostic`
  1 passed / 0 failed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples`: 45 tests OK.
- Surface helper `check-all` JSON summary:
  `sample_count = 52`, `failed_count = 0`, `validation_error_count = 0`,
  `elab04_has_repair = false`, `elab04_missing = ["MissingWitness",
  "RouteUnavailable", "StaleMembership", "VisibilityDenied"]`,
  `elab07_repair_shape = "set_insertion"`, `elab10_repair_count = 1`,
  `elab13_repair_count = 1`.
- `python3 scripts/validate_docs.py`: documentation scaffold complete; 1307
  numbered reports found.
- `python3 scripts/check_source_hierarchy.py`: required 602 / present 602 /
  missing 0.
- `git diff --check` exited 0.
- Endpoint-form scan over changed and untracked files returned no matches.

## What changed in understanding

`ELAB-04` should be documented as a mixed row whose current carrier is lossy at
the branch-ownership level. The safe repository-memory model is to separate
the conceptual wrapper that owns the associated request and whole failed
row-containment premise from child concepts that classify the base
remote-request missing failures and the visibility branch. This still does not
authorize an executable payload.

## Open questions

- Should future `ELAB-04` executable widening use a mixed wrapper with branch
  children or separate associated diagnostics?
- What exact association key is needed to prevent separate diagnostics from
  double-counting one generated request?
- Should OBL-025 remain single-edit only, or gain a separate grouped
  multi-edit / whole-gap relation?
- Should visibility repair guidance live in `suggested_repair[]` or a separate
  guidance family?
- What ranking domain is intended: complete whole-row repairs, branch-local
  guidance, or human-facing alternatives?

## Suggested next prompt

「OBL-025 の whole-gap coverage / branch-local guidance non-coverage /
grouped multi-edit relationを、Lean compile-check-only statement draft 側で
どこまで抽象化できるかレビューしてください。」

## Plan update status

更新済み:

- Added `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated `plan/95`, `plan/96`, and `plan/98`.

## Documentation.md update status

更新済み:

- Added `ELAB-04` mixed visibility payload-model preflight status and
  `plan/107` to the concise Surface Mir current summary.

## progress.md update status

更新済み:

- Added current `ELAB-04` mixed visibility payload-model preflight status.
- Updated E-ROW mixed / set-insertion notes to mention `plan/107`.
- Added a 2026-07-04 06:31 JST recent-log row.

## tasks.md update status

更新済み:

- Moved `ELAB-04` mixed visibility payload-model preflight into current
  holding state.
- Replaced the completed preflight candidate with a later reserve executable
  payload-model candidate that remains blocked on a wrapper or associated
  diagnostics decision.
- Added a reserve `OBL-025 whole-gap relation refinement` candidate to mirror
  the package's suggested next proof-side prompt.

## samples_progress.md update status

更新済み:

- Updated dashboard wording for `plan/107`.
- Added a recent validation log row.
- Kept sample row count at 52.

## Reviewer findings and follow-up

- Semantic reviewer `019f29f0-f925-72f0-bbde-3c73b110ec87` reported no
  blocking findings. It checked that `plan/107` remains LAB-only, does not
  adopt a Diagnostic ABI / JSON shape / emitted payload / repair object /
  branch IDs / independent failed premises / ranking / OBL-024/025 proof, and
  keeps the `ELAB-04` no-repair / exact `ELAB-07` exception boundary accurate.
- Bookkeeping reviewer `019f29f1-13a0-73a2-8cfc-9dc0e0b93341` reported no
  blocking findings. It verified required report section order, Markdown-only
  changed/untracked files, unchanged expected JSON / sample matrix, 52 Surface
  rows, no endpoint-form leak, and passing docs/source-hierarchy/diff checks.
  It noted pre-final pending wording and the absence of an OBL-025 refinement
  candidate in `tasks.md`; this report revision replaced the pending wording,
  and `tasks.md` now includes a reserve `OBL-025 whole-gap relation
  refinement` candidate.

## Skipped validations and reasons

No required local validation was skipped for this docs-only package. Full
workspace Rust tests were not rerun because no Rust source, expected JSON, or
sample helper code changed; the package instead reran the focused elaboration
tests, the full Surface sample unittest module, the Surface helper `check-all`,
and docs/source hierarchy checks.

## Commit / push status

- Content commit `619a267214431bfa0c5ad61eb4ef2ccf2c2ee83e`
  (`Add ELAB-04 mixed preflight`) was pushed to `origin/main`.
- This report-status section is the follow-up bookkeeping update for the
  package. Final `HEAD == origin/main` verification is performed after this
  bookkeeping update is committed and pushed, because a commit cannot record
  its own hash before it exists.

## Sub-agent session close status

The mapping sub-agent completed and its findings were incorporated; a close was
requested for that session. Reviewer sub-agents
`019f29f0-f925-72f0-bbde-3c73b110ec87` and
`019f29f1-13a0-73a2-8cfc-9dc0e0b93341` completed and were closed.
