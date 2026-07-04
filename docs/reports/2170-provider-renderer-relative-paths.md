# Report 2170 - provider / renderer generated-report relative paths

- Date: 2026-07-04 11:47 JST
- Author / agent: Codex
- Scope: Full System V1 provider / renderer / release-check path portability hardening
- Decision levels touched: no canon decision changed; LAB helper and generated evidence only

## Objective

Stop Full System V1 provider / renderer helper execution and representative
release-check CLI planning from rewriting generated evidence with host-specific
absolute repository paths. Preserve the existing bounded provider-admission,
renderer-pose, and release-check evidence while making subprocess inputs
portable across checkouts.

## Scope and assumptions

Scope is limited to:

- `scripts/provider_admission_samples.py`
- `scripts/renderer_pose_backend_samples.py`
- `scripts/full_system_v1_release_check.py`
- their focused unit tests
- renderer-pose generated nested `provider-admission-report.json` artifacts
- repository-memory / snapshot docs that describe the maintenance hardening

Working assumption: the Rust report surfaces intentionally echo the path text
they receive. Because the Python helpers run subprocesses with `cwd=REPO_ROOT`,
the smallest portable fix is to pass repo-relative `samples/...` argv for
in-repo paths and preserve absolute strings only for external paths.

Stop line: no canon edit, no normative spec edit, no runtime semantics change,
no new provider execution admission, no final ABI / SDK claim, no sample count
or workflow status change, and no renderer-owned world semantics claim.

## Start state / dirty state

Package 32 started from a clean `main` at `a4cd1ab4`, matching `origin/main`,
after report 2169 had been committed and pushed. The triggering observation was
that running `python3 -m unittest discover -s scripts/tests` in package 31
regenerated three renderer-pose nested provider-admission reports with local
absolute `/home/codex/...` paths.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `plan/58-full-system-v1-roadmap.md`
- `docs/reports/2169-source-hierarchy-wording-lint.md`
- `docs/reports/TEMPLATE.md`
- `scripts/provider_admission_samples.py`
- `scripts/renderer_pose_backend_samples.py`
- `scripts/tests/test_provider_admission_samples.py`
- `scripts/tests/test_renderer_pose_backend_samples.py`
- `scripts/full_system_v1_release_check.py`
- `scripts/tests/test_full_system_v1_release_check.py`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `crates/mir-runtime/src/full_system_v1_provider_admission.rs`
- `crates/mir-runtime/src/full_system_v1_renderer_pose_backend.rs`

## Actions taken

- Reproduced the renderer-pose generated-file side effect and confirmed the
  changed files were the three renderer-pose nested
  `generated/provider-admission-report.json` artifacts.
- Traced the path values from Python helper row paths through subprocess argv
  into Rust report serialization.
- Added a red renderer test that runs the positive renderer row and asserts all
  generated nested provider-admission `*_path` fields are non-absolute.
- Added a red renderer command-shape assertion that expects the source argv to
  be repo-relative.
- Implemented helper-local `_repo_relative_arg()` in
  `scripts/renderer_pose_backend_samples.py` and used it for source, request,
  provider manifest, and PoseGraph package inputs.
- Incorporated sub-agent tracing, which showed the provider helper had the same
  subprocess-boundary portability issue even though its tracked generated JSON
  summary does not expose full path fields.
- Added a red provider command-shape assertion that expects the source, request,
  and provider manifest argv to be repo-relative.
- Implemented the same helper-local `_repo_relative_arg()` pattern in
  `scripts/provider_admission_samples.py`.
- Added a red release-check plan test showing that representative Full System
  V1 CLI commands were still planned with host absolute sample paths.
- Added `repo_relative_arg()` to `scripts/full_system_v1_release_check.py` and
  used it for the representative `project-full-v1`, `run-full-v1-split`,
  `admit-provider-v1`, and `render-pose-backend-v1` sample inputs.
- After final review, added output-root-relative serialization for Full System
  V1 release-check report / bundle / viewer path fields and command argv
  entries under the release output root.
- Added release-check unit coverage proving a home-shaped output root does not
  serialize `/home/` or `/Users/` in the release-check's own `reports/`,
  `bundle.json`, or `index.html`.
- Added focused external-path fallback tests for provider, renderer, and
  release-check repo-relative path helpers.
- Regenerated renderer-pose nested provider-admission reports so committed path
  fields use `samples/...` rather than host absolute paths.
- Updated `plan/58`, `progress.md`, `tasks.md`, and `samples_progress.md` with
  the maintenance hardening note.

## Files changed

- `scripts/provider_admission_samples.py`
- `scripts/renderer_pose_backend_samples.py`
- `scripts/full_system_v1_release_check.py`
- `scripts/tests/test_full_system_v1_release_check.py`
- `scripts/tests/test_provider_admission_samples.py`
- `scripts/tests/test_renderer_pose_backend_samples.py`
- `samples/full-system-v1/provider-adapter/renderer-pose-positive/generated/provider-admission-report.json`
- `samples/full-system-v1/provider-adapter/renderer-pose-reacquire-negative/generated/provider-admission-report.json`
- `samples/full-system-v1/provider-adapter/renderer-pose-split-frame-negative/generated/provider-admission-report.json`
- `plan/58-full-system-v1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2170-provider-renderer-relative-paths.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- `git diff -- scripts/renderer_pose_backend_samples.py scripts/tests/test_renderer_pose_backend_samples.py`
- `sed -n ... scripts/provider_admission_samples.py`
- `sed -n ... scripts/tests/test_provider_admission_samples.py`
- `find samples/full-system-v1/provider-adapter -path '*generated/provider-admission-report.json' -maxdepth 4 -type f | sort`
- `rg -n 'source_path|request_path|provider_manifest_path' ...`
- `python3 -m unittest scripts.tests.test_provider_admission_samples.ProviderAdmissionSamplesTests.test_helper_executes_cli_surface_with_repo_relative_paths`
- `python3 -m unittest scripts.tests.test_provider_admission_samples.ProviderAdmissionSamplesTests.test_helper_executes_cli_surface_with_repo_relative_paths`
- `python3 -m unittest scripts.tests.test_renderer_pose_backend_samples.RendererPoseBackendSamplesTests.test_positive_row_writes_portable_provider_report_paths scripts.tests.test_renderer_pose_backend_samples.RendererPoseBackendSamplesTests.test_helper_executes_cli_surface`
- `python3 -m unittest scripts.tests.test_provider_admission_samples`
- `python3 -m unittest scripts.tests.test_renderer_pose_backend_samples`
- `python3 scripts/provider_admission_samples.py check-all --format json`
- `python3 scripts/renderer_pose_backend_samples.py check-all --format json`
- `rg -n '"[^"\n]*(/home/|/Users/)' samples/full-system-v1/provider-adapter/*/generated/provider-admission-report.json`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-relative-paths`
- `python3 -m unittest scripts.tests.test_full_system_v1_release_check.FullSystemV1ReleaseCheckTests.test_plan_cli_commands_use_repo_relative_sample_paths`
- `python3 -m unittest scripts.tests.test_full_system_v1_release_check.FullSystemV1ReleaseCheckTests.test_plan_cli_commands_use_repo_relative_sample_paths`
- `python3 -m py_compile scripts/full_system_v1_release_check.py scripts/tests/test_full_system_v1_release_check.py`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-relative-paths-2`
- `rg -n '"[^"\n]*(/home/|/Users/)' /tmp/mirrorea-full-v1-release-relative-paths-2`
- `rg -n '"[^"\n]*(/home/|/Users/)' /tmp/mirrorea-full-v1-release-relative-paths-2/reports /tmp/mirrorea-full-v1-release-relative-paths-2/bundle.json /tmp/mirrorea-full-v1-release-relative-paths-2/index.html`
- `rg -n 'samples/full-system-v1/.+\.(mir|json)' /tmp/mirrorea-full-v1-release-relative-paths-2/reports/cli__*.json /tmp/mirrorea-full-v1-release-relative-paths-2/bundle.json`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 -m py_compile scripts/full_system_v1_release_check.py scripts/provider_admission_samples.py scripts/renderer_pose_backend_samples.py scripts/tests/test_full_system_v1_release_check.py scripts/tests/test_provider_admission_samples.py scripts/tests/test_renderer_pose_backend_samples.py`
- `git diff --check`
- `python3 -m unittest scripts.tests.test_full_system_v1_release_check scripts.tests.test_provider_admission_samples scripts.tests.test_renderer_pose_backend_samples`
- `python3 -m unittest discover -s scripts/tests`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-relative-paths-final`
- `rg -n '"[^"\n]*(/home/|/Users/)' /tmp/mirrorea-full-v1-release-relative-paths-final/reports /tmp/mirrorea-full-v1-release-relative-paths-final/bundle.json /tmp/mirrorea-full-v1-release-relative-paths-final/index.html`
- `rg -n '"[^"\n]*(/home/|/Users/)' samples/full-system-v1/provider-adapter/*/generated/provider-admission-report.json`
- `python3 -m unittest scripts.tests.test_full_system_v1_release_check.FullSystemV1ReleaseCheckTests.test_release_outputs_do_not_serialize_home_shaped_output_paths`
- `python3 -m unittest scripts.tests.test_full_system_v1_release_check.FullSystemV1ReleaseCheckTests.test_release_outputs_do_not_serialize_home_shaped_output_paths scripts.tests.test_full_system_v1_release_check.FullSystemV1ReleaseCheckTests.test_repo_relative_helpers_preserve_external_paths`
- `python3 -m unittest scripts.tests.test_provider_admission_samples.ProviderAdmissionSamplesTests.test_repo_relative_arg_preserves_external_paths scripts.tests.test_renderer_pose_backend_samples.RendererPoseBackendSamplesTests.test_repo_relative_arg_preserves_external_paths`
- `python3 -m py_compile scripts/full_system_v1_release_check.py scripts/tests/test_full_system_v1_release_check.py scripts/tests/test_provider_admission_samples.py scripts/tests/test_renderer_pose_backend_samples.py`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 -m py_compile scripts/full_system_v1_release_check.py scripts/provider_admission_samples.py scripts/renderer_pose_backend_samples.py scripts/tests/test_full_system_v1_release_check.py scripts/tests/test_provider_admission_samples.py scripts/tests/test_renderer_pose_backend_samples.py`
- `git diff --check`
- `python3 -m unittest scripts.tests.test_full_system_v1_release_check scripts.tests.test_provider_admission_samples scripts.tests.test_renderer_pose_backend_samples`
- `python3 -m unittest discover -s scripts/tests`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-relative-paths-reviewfix`
- `rg -n '"[^"\n]*(/home/|/Users/)' /tmp/mirrorea-full-v1-release-relative-paths-reviewfix/reports /tmp/mirrorea-full-v1-release-relative-paths-reviewfix/bundle.json /tmp/mirrorea-full-v1-release-relative-paths-reviewfix/index.html`
- `rg -n '"[^"\n]*(/home/|/Users/)' samples/full-system-v1/provider-adapter/*/generated/provider-admission-report.json`
- `python3 scripts/provider_admission_samples.py check-all --format json`
- `python3 scripts/renderer_pose_backend_samples.py check-all --format json`

## Evidence / outputs / test results

Red / green evidence so far:

- Provider command-shape red test before implementation:
  `python3 -m unittest scripts.tests.test_provider_admission_samples.ProviderAdmissionSamplesTests.test_helper_executes_cli_surface_with_repo_relative_paths`
  failed because the command used
  `/home/codex/dev/mir_poc_01/samples/.../viewer-diagnostic-positive.mir`
  instead of `samples/.../viewer-diagnostic-positive.mir`.
- Provider command-shape test after implementation: 1 test passed.
- Renderer path-field and command-shape red tests before implementation failed
  because generated nested provider-admission reports and subprocess argv used
  `/home/codex/...` paths.
- Renderer targeted tests after implementation: 2 tests passed.
- `python3 -m unittest scripts.tests.test_provider_admission_samples`: 11 tests
  passed.
- `python3 -m unittest scripts.tests.test_renderer_pose_backend_samples`: 10
  tests passed.
- `python3 scripts/provider_admission_samples.py check-all --format json`:
  5 passed, 0 failed, no validation errors.
- `python3 scripts/renderer_pose_backend_samples.py check-all --format json`:
  3 passed, 0 failed, no validation errors.
- Absolute-path scan over provider-adapter generated provider-admission reports
  returned no matches.
- Release-check representative CLI command red test before implementation:
  `python3 -m unittest scripts.tests.test_full_system_v1_release_check.FullSystemV1ReleaseCheckTests.test_plan_cli_commands_use_repo_relative_sample_paths`
  failed because `cli:project-full-v1` used
  `/home/codex/dev/mir_poc_01/samples/.../effectful-sugoroku-positive.mir`.
- Release-check representative CLI command test after implementation: 1 test
  passed.
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-relative-paths-2`:
  status `accepted`, all 29 planned commands passed, and the `cli:*` command
  argv in the release bundle used `samples/full-system-v1/...` paths.
- Absolute-path scan over the Full System V1 release-check `reports/`,
  `bundle.json`, and `index.html` returned no `/home` or `/Users` matches.
- A broader scan over the whole release-check output found Product Alpha
  compatibility-floor reports with `/home/codex/...` paths under
  `compat-product-alpha1-release/`; those are outside this package's Full
  System V1 provider / renderer / release-check surface and are recorded as a
  follow-up candidate, not silently fixed here.
- Final reviewer found that the release-check's own `out_dir`, `report_path`,
  `bundle_path`, and `html_path` fields still serialized raw output-root paths
  when `--out` itself was home-shaped. A new red test reproduced that with a
  `/tmp/.../home/codex/release` output root, failing on `/home/` in
  `bundle.json`.
- After adding output-root-relative display serialization, the release output
  serialization test passed. External fallback tests for provider, renderer,
  and release-check helpers also passed.
- Final validation:
  - `python3 scripts/validate_docs.py`: `Documentation scaffold looks complete.`
    and `Found 1322 numbered report(s).`
  - `python3 scripts/check_source_hierarchy.py`: required 602, present 602,
    missing 0.
  - `python3 -m py_compile ...`: passed with no output.
  - `git diff --check`: passed with no output.
  - `python3 -m unittest scripts.tests.test_full_system_v1_release_check scripts.tests.test_provider_admission_samples scripts.tests.test_renderer_pose_backend_samples`:
    33 tests passed.
  - `python3 -m unittest discover -s scripts/tests`: 670 tests passed.
  - `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-relative-paths-reviewfix`:
    status `accepted`, 29 planned commands passed, `failed_commands: []`.
  - Absolute-path scan over
    `/tmp/mirrorea-full-v1-release-relative-paths-reviewfix/reports`,
    `bundle.json`, and `index.html`: no `/home` or `/Users` matches.
  - Absolute-path scan over committed provider-adapter generated
    provider-admission reports: no `/home` or `/Users` matches.
  - `python3 scripts/provider_admission_samples.py check-all --format json`:
    5 passed, 0 failed, no validation errors.
  - `python3 scripts/renderer_pose_backend_samples.py check-all --format json`:
    3 passed, 0 failed, no validation errors.

## What changed in understanding

The portability issue was not a Rust report-constructor bug. The Rust surfaces
echo the path strings they are given, which is useful for report traceability.
The helper scripts were the unstable layer because they built absolute in-repo
`Path` objects and passed `str(path)` to subprocesses running with
`cwd=REPO_ROOT`.

Provider admission and renderer pose share the same subprocess-boundary pattern.
Only renderer-pose exposed the issue in committed generated JSON because it
writes the full nested provider-admission report, including nested
`source_path`, `request_path`, and `provider_manifest_path` fields. Provider
admission's own generated summary does not expose those fields, but the command
shape should still be portable for consistency and future report widening.

The same principle applies to Full System V1 release-check planning. Its
representative CLI commands and output-root paths are stored in report and
bundle JSON, so planning sample inputs with repo-relative paths and displaying
output-root paths relatively avoids host-specific evidence in the bounded Full
System V1 release lane. The embedded Product Alpha compatibility release-check
is a separate compatibility floor with its own output path policy.

## Open questions

- Whether later helper families that currently pass absolute in-repo paths but
  do not commit full path-bearing reports should be audited in a broader
  portability package.
- Whether a shared Python utility for repo-relative subprocess argv should be
  introduced after one or two more helper families need the same behavior.
- Whether Product Alpha compatibility-floor release outputs should receive a
  separate portability pass for `package_path`, Docker Compose file, and native
  bundle provenance fields.

## Suggested next prompt

Continue the autonomous maintenance chain by auditing the remaining sample
helpers for host-specific generated evidence only after this package is
validated, reviewed, committed, and pushed. Do not widen runtime semantics or
provider execution scope by default.

## Plan update status

`plan/58-full-system-v1-roadmap.md` updated with a 2026-07-04 maintenance note
for repo-relative provider / renderer helper input paths, release-check
representative CLI paths, output-root-relative release-check report / bundle /
viewer path display, and renderer nested provider report path fields.

## Documentation.md update status

`Documentation.md` update not needed: the source hierarchy and Full System V1
summary remain current, and this package only hardens helper portability for
existing evidence.

## progress.md update status

`progress.md` updated with a timestamped recent log entry. No feature status,
macro phase, OBL status, conformance, ABI, or runtime-semantics claim changed.

## tasks.md update status

`tasks.md` updated with a holding-state note and timestamp. The current
recommended next workstream remains unchanged.

## samples_progress.md update status

`samples_progress.md` updated with a timestamped validation-log row and
provider-adapter / renderer-pose / release-check evidence wording. No sample
count or workflow status changed.

## Reviewer findings and follow-up

Sub-agent tracing found that the same absolute-argv boundary exists in both
provider and renderer helpers. The package incorporated that finding by adding
provider-side command-shape coverage and the same helper-local fix. Local
inspection then found the same pattern in Full System V1 release-check
representative CLI planning, and this package added focused coverage there too.

Final reviewer findings and follow-up:

- Medium: release-check still serialized home-shaped output-root paths in its
  own reports / bundle / viewer. Fixed by output-root-relative display
  serialization and a red/green test with `/tmp/.../home/codex/release`.
- Low: `progress.md` header timestamp was stale. Fixed to
  `2026-07-04 11:47 JST`.
- Low / test gap: external-path fallback for repo-relative helpers was
  unguarded. Fixed with focused tests for provider, renderer, and release-check
  helpers.

## Skipped validations and reasons

No relevant validation was intentionally skipped for this package. Broader
Product Alpha compatibility-output path normalization was not attempted because
it is a separate compatibility-floor surface and the only remaining host paths
found by the broad `/tmp` scan were under
`compat-product-alpha1-release/`.

## Commit / push status

Pending commit and push.

## Sub-agent session close status

Descartes returned a focused execution-path trace covering provider and
renderer helper mutation paths, Rust serialization points, smallest fix, and
existing relative-path precedent. Darwin returned final read-only review
findings; all three findings were addressed locally. No sub-agent file edits
were made.
