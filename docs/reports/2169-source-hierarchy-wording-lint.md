# Report 2169 - source-hierarchy wording lint

- Date: 2026-07-04 11:07 JST
- Author / agent: Codex
- Scope: validator guardrail plus reader-facing LAB wording cleanup
- Decision levels touched: no canon decision changed; LAB wording and validator only

## Objective

Close the open follow-up from report 2168 by adding a small mechanical lint that
prevents active reader-facing docs and repository memory from re-promoting
legacy `specs/` as the normative source after the `mirrorea_canon/` adoption.

## Scope and assumptions

Scope is limited to `scripts/validate_docs.py`, its unit tests, active
reader-facing LAB docs under `docs/hands_on/` and `docs/research_abstract/`,
the affected LAB roadmap notes in `plan/19`, `plan/50`, and `plan/58`, and
snapshot / repository-memory updates that describe the guardrail.

Working assumption: historical reports, legacy `specs/` content, and archived
research material may preserve older wording as evidence/history. The lint
therefore targets `CANON.md`, root/snapshot docs, `samples/README.md`,
`.docs/`, `docs/hands_on/`, `docs/research_abstract/`, and `plan/`, while
excluding `docs/research_abstract/old/`.

Stop line: no canon edit, no G0 or G1..G7 exit, no proof-obligation status
movement, no conformance claim, no implementation semantics change, no runnable
sample status change, and no broad historical rewrite.

## Start state / dirty state

Package 31 started clean on `main` at `8fe3a7e6`, matching `origin/main`, after
the source-hierarchy wording audit package and report-status update were
committed and pushed.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `docs/reports/2168-g0-source-hierarchy-stale-wording-audit.md`
- `docs/reports/TEMPLATE.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/58-full-system-v1-roadmap.md`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/repository_layer_structure_01.md`
- Relevant `docs/research_abstract/*.md` files reported by the new lint
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`

## Actions taken

- Added a red unit test showing that `validate_docs.py` must reject
  reader-facing `規範判断の正本は specs/` wording.
- Confirmed the test failed before implementation with `0 != 1`.
- Added source-hierarchy wording lint constants and scanner functions to
  `scripts/validate_docs.py`.
- Scoped the lint to active/root reader surfaces, `samples/README.md`, and
  `plan/`, excluding historical reports, legacy specs, and archived research
  material.
- Extended the lint after sidecar review so it also catches split-line
  `specs/` / source-of-truth bullets and the English
  `Normative source remains specs...` shape.
- Allowed the explicit rejected-pattern row in `plan/70` so the ledger can keep
  documenting the forbidden historical pattern.
- Reworded detected reader-facing docs so `mirrorea_canon/` is the normative
  source and legacy `specs/` links are LAB evidence / historical boundary
  references.
- Updated `plan/119`, `progress.md`, and `tasks.md` to record the new guardrail.

## Files changed

- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `docs/research_abstract/full_system_v1_roadmap_01.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/operational_backend_inventory_01.md`
- `docs/research_abstract/operational_package_authoring_01.md`
- `docs/research_abstract/operational_product_sample_01.md`
- `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
- `docs/research_abstract/post_p20_hotplug_next_package_inventory_01.md`
- `docs/research_abstract/post_p21_distributed_activation_ordering_family_01.md`
- `docs/research_abstract/post_p21_final_public_hotplug_abi_family_01.md`
- `docs/research_abstract/post_p21_rollback_durable_migration_family_01.md`
- `docs/research_abstract/product_alpha1_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `docs/research_abstract/repository_layer_structure_01.md`
- `docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2169-source-hierarchy-wording-lint.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short --branch`
- `wc -l scripts/validate_docs.py scripts/tests/test_validate_docs.py scripts/check_source_hierarchy.py`
- `rg -n "^def |^class |argparse|main\\(|unittest|report|Documentation scaffold|Found|ERROR|WARN|fail|violation" scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- `sed -n ... scripts/tests/test_validate_docs.py`
- `sed -n ... scripts/validate_docs.py`
- `rg -n "CANON_NOTICE|PROGRESS_REQUIRED|TASKS_REQUIRED|UNRESOLVED_TEMPLATE|REQUIRED_TEMPLATE" scripts/validate_docs.py`
- Multiple `rg` source-hierarchy / stale wording scans across root docs,
  snapshots, `plan/`, `.docs/`, `docs/hands_on/`, `docs/research_abstract/`,
  `scripts/README.md`, and `AGENTS.md`
- `find docs/reports -maxdepth 1 -name "[0-9][0-9][0-9][0-9]-*.md" | sort | tail -n 5`
- `sed -n ... docs/reports/2168-g0-source-hierarchy-stale-wording-audit.md`
- `sed -n ... docs/hands_on/README.md`
- `sed -n ... docs/research_abstract/README.md`
- `find docs/research_abstract -maxdepth 1 -type f -name "*.md" | sort`
- Focused `rg` stale `specs/`-as-normative scans over `docs/hands_on`,
  `docs/research_abstract`, and `plan`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_reader_facing_specs_as_normative_wording`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_stale_wording_in_canon_and_sample_entry_docs scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_split_line_and_english_specs_normative_wording`
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_japanese_specs_normative_variants scripts.tests.test_validate_docs.ValidateDocsTests.test_main_allows_negated_specs_as_normative_policy_wording`
- `python3 scripts/validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 -m unittest discover -s scripts/tests`
- `python3 scripts/check_source_hierarchy.py`
- `git diff --check`
- `python3 -m py_compile scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- `git diff --stat`
- `git diff -- scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- `git diff -- docs/hands_on/README.md docs/research_abstract/README.md docs/research_abstract/full_system_v1_roadmap_01.md plan/50-product-alpha1-public-boundary-roadmap.md`
- `date '+%Y-%m-%d %H:%M %Z'`
- `git diff -- progress.md tasks.md plan/119-g0-remaining-claim-family-drilldown-priority.md`
- `sed -n ... plan/19-repository-map-and-taxonomy.md`
- `sed -n ... docs/research_abstract/repository_layer_structure_01.md`
- `sed -n ... plan/58-full-system-v1-roadmap.md`
- `sed -n ... CANON.md`
- `sed -n ... samples/README.md`
- Changed-file endpoint leak scan over `git ls-files --modified --others --exclude-standard`
- `git add ...`
- Staged-file endpoint leak scan over `git diff --cached --name-only --diff-filter=ACM`
- `git commit --no-gpg-sign -m "Add source hierarchy wording lint"`
- `git push`

## Evidence / outputs / test results

Red / green evidence so far:

- Targeted red test before implementation:
  `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_main_rejects_reader_facing_specs_as_normative_wording`
  failed with `AssertionError: 0 != 1`.
- Targeted test after implementation:
  one test passed.
- `python3 scripts/validate_docs.py` initially reported 18 stale wording hits
  in `docs/hands_on/README.md`, selected `docs/research_abstract/*.md`, and
  `plan/50`.
- After wording cleanup, `python3 scripts/validate_docs.py` reported
  `Documentation scaffold looks complete.` and `Found 1320 numbered report(s).`
- `python3 -m unittest scripts.tests.test_validate_docs` reported 21 tests
  passing.
- Sidecar review identified missing lint scope for `CANON.md` and
  `samples/README.md`, plus pattern gaps for split-line `specs/` /
  source-of-truth bullets and `Normative source remains specs...` wording.
- Added two more red tests; before implementation both failed with `0 != 1`.
- After implementation, those two targeted tests passed.
- The widened lint then reported 3 additional active hits:
  `docs/research_abstract/repository_layer_structure_01.md`, `plan/19`, and
  `plan/58`.
- After the second wording cleanup, `python3 scripts/validate_docs.py` reported
  `Documentation scaffold looks complete.` and `Found 1321 numbered report(s).`
- Final reviewer then identified two precision issues:
  - false positive risk for negated policy wording such as
    `Do not treat specs as normative`;
  - false negative risk for Japanese variants without whitespace and
    `規範正本は specs`.
- Added two red tests for those cases; before implementation both failed with
  `0 != 1`.
- After implementation, those two targeted tests passed.
- Final validation:
  - `python3 scripts/check_source_hierarchy.py`: required 602, present 602,
    missing 0.
  - `python3 scripts/validate_docs.py`: `Documentation scaffold looks complete.`
    and `Found 1321 numbered report(s).`
  - `python3 -m unittest scripts.tests.test_validate_docs`: 25 tests passed.
  - `python3 -m unittest discover -s scripts/tests`: 663 tests passed.
  - `python3 -m py_compile scripts/validate_docs.py scripts/tests/test_validate_docs.py`:
    passed with no output.
  - `git diff --check`: passed with no output.
- `python3 -m unittest discover -s scripts/tests` regenerated three
  provider-admission report JSON files with local absolute `/home/codex/...`
  paths; those generated-output side effects were restored out of the task
  diff after validation.

## What changed in understanding

Report 2168 fixed the most visible root/snapshot and LAB-memory drift, but
reader-facing `docs/hands_on/` and `docs/research_abstract/` still had several
old source-hierarchy summaries. Sidecar review also showed that `CANON.md`,
`samples/README.md`, split-line source-hierarchy bullets, and the English
`Normative source remains specs...` wording are part of the useful guardrail
surface. Those pages are not all canon, but they are entry points for humans
and agents, so leaving them outside the lint would allow the same drift to
return.

The lint needs to be narrower than a global markdown grep because historical
reports and legacy specs intentionally preserve old wording. A scoped
reader-facing / active-memory guard gives useful protection without rewriting
evidence history.

## Open questions

- Whether future package lines should add more precise allowlist entries for
  intentionally quoted rejected patterns, or whether they should prefer
  paraphrases that avoid triggering the lint.
- Whether a later cleanup should shorten older research abstracts further so
  they depend less on dense source-hierarchy preambles.

## Suggested next prompt

Continue self-driven work by returning to the next safe G1 support package only
after this validator package is reviewed, validated, committed, and pushed. Do
not edit canon or widen runtime / observation behavior by default.

## Plan update status

`plan/` updated:

- Updated `plan/119` with the validator follow-up.
- Updated `plan/19` to list `mirrorea_canon/` as canon and legacy `specs/` as
  LAB evidence.
- Updated `plan/50` to demote `specs/25` from normative source wording to
  legacy LAB boundary evidence.
- Updated `plan/58` to demote `specs/33..38` from normative source wording to
  legacy LAB boundary evidence.

## Documentation.md update status

`Documentation.md` update not needed: the existing canon/LAB source-hierarchy
summary remains current, and this package only adds a validator guardrail plus
reader-facing LAB cleanup.

## progress.md update status

`progress.md` updated with the source-hierarchy wording lint note and a
timestamped recent log entry.

## tasks.md update status

`tasks.md` updated with the source-hierarchy wording lint holding-state note,
maintenance validation row, and timestamp.

## samples_progress.md update status

`samples_progress.md` update not needed: no runnable sample path, validation
command, blocker, workflow readiness row, or sample status changed.

## Reviewer findings and follow-up

Sub-agent `Hegel` completed a read-only scope review. Findings:

- Add `CANON.md` and `samples/README.md` to the lint scope.
- Keep `docs/reports/**`, `specs/**`, `docs/research_abstract/old/**`,
  `sub-agent-pro/**`, and `tmp_faq/**` out of scope.
- Catch split-line `specs/` / source-of-truth bullets and the English
  `Normative source remains specs...` wording.
- Correct active hits in `plan/19`,
  `docs/research_abstract/repository_layer_structure_01.md`, and `plan/58`.

Follow-up: all four actionable findings were implemented. The `progress.md`
self-reference false positive was handled by rewording the progress log rather
than adding a broad allowlist.

Sub-agent `Noether` then performed a final focused review and found no blocking
issues. Follow-up precision findings were implemented:

- Added a negative-case test and allowed explicitly negated policy wording such
  as `Do not treat specs as normative`.
- Added Japanese variant coverage for no-space `規範判断の正本は specs` wording
  and `規範正本は specs` wording.
- Updated this report's closeout sections.

## Skipped validations and reasons

- Cargo / Rust build and tests not run for this package because the
  changes are Python validator plus documentation wording only.
- Lean / Lake checks not run for this package because no Lean source,
  generated Lean artifact, theorem statement, or proof ledger changed.
- Sample helper / release-check suites not run for this package because
  no runnable sample status, helper output, expected JSON, or command surface
  changed.
- The full `scripts/tests` discovery was run and passed, but its generated JSON
  absolute-path side effects were not kept because this package does not change
  generated sample evidence.

## Commit / push status

Package commit pushed:

- `6935e80a Add source hierarchy wording lint`

Report-status commit pending at this report update.

## Sub-agent session close status

Sub-agent `Hegel` completed the read-only scope check and was closed.
Sub-agent `Noether` completed the final focused review and was closed.
