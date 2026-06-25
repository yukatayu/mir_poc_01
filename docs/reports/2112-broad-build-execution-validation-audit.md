# Report 2112 — broad build execution validation audit

- Date: 2026-06-25T08:13:26.402374Z
- Author / agent: Codex
- Scope: repository-wide readthrough-informed build / execution audit, clippy hardening, focused sample reruns, and snapshot report
- Decision levels touched: no normative `specs/` decision changed; implementation/test/report/dashboard maintenance only

## Objective

Run and understand as much of the repository's buildable / executable surface as practical, using sub-agent inventory to avoid missing major command families. Fix validation blockers found during that audit, keep generated / local-only artifacts out of the commit, and leave repository status evidence in the required report/dashboard locations.

## Scope and assumptions

- Scope includes Rust workspace build/test/clippy, Python helper/unit tests, documented sample helpers, release-check helpers, direct `mirrorea-alpha` CLI entrypoints, resource/toolchain availability, Docker availability, and Lean/Lean-stub boundary checks.
- Scope does not include changing normative semantics in `specs/`.
- Lean compiler validation is only possible if `lean` / `lake` / `elan` are installed on `PATH`; they were not.
- Heavy/generated release outputs were written under `/tmp/mir-validation-20260625-164803`, not repo root.
- `.codex-discord/` was treated as local secret config and not inspected or committed.

## Start state / dirty state

- Branch: `main`.
- Remote: `origin git@github.com:yukatayu/mir_poc_01.git`.
- Task baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .` recorded the Discord diff baseline without sending a progress notification.
- Initial tracked dirty state for this audit was clean after the prior skill-install task; ignored/local-only items included `.codex-discord/`, `Cargo.lock`, `target/`, and Python `__pycache__/`.
- Resource audit:
  - `df -h .`: root filesystem had about 60G available at audit start.
  - `free -h`: about 9.3Gi available memory.
  - `lsblk -f` / `findmnt`: no mounted `/mnt/mirrorea-work`; root ext4 was the active filesystem.
  - `target/` grew from about 1.4G to about 6.9G after broad Cargo and release checks.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `samples_progress.md`
- `tasks.md`
- `scripts/README.md`
- `samples/README.md`
- `Cargo.toml`
- `Makefile`
- relevant Rust source/tests/examples under `crates/`
- relevant sample/helper scripts under `scripts/`
- skill instructions read for this task: `discord-report`, `superpowers:using-superpowers`, `superpowers:dispatching-parallel-agents`, `superpowers:subagent-driven-development`, `superpowers:systematic-debugging`, `superpowers:verification-before-completion`

## Actions taken

- Used three sub-agents to inventory runnable surfaces and risk:
  - Rust/Cargo workspace, examples, and expected outputs.
  - Python/helper/Docker/Lean availability and side-effect risk.
  - sample taxonomy and documented validation commands.
- Established `/tmp/mir-validation-20260625-164803` as the validation log/output root.
- Ran docs validators, Python helper suites, Sugoroku helper, practical alpha helpers, Product Alpha helpers, Full System V1 helpers, Surface Mir helpers, release checks, workspace Cargo build/test/clippy, Lean-stub pipeline, and direct CLI commands.
- Found that `cargo clippy --workspace --all-targets -- -D warnings` failed on existing warnings across `mirrorea-core`, `mir-ast`, `mir-semantics`, `mir-runtime`, and `mirrorea-cli`.
- Fixed clippy blockers without changing normative specs:
  - collapsed nested `if` statements where clippy required it;
  - removed redundant closures / clones / conversions;
  - corrected bool assertions in tests;
  - boxed the large `SurfacePlaceItem::State` enum variant;
  - added narrow `#[allow(clippy::too_many_arguments)]` / `#[allow(clippy::type_complexity)]` where helper builders intentionally carry evidence-row parameters.
- Restored generated provider-admission JSON absolute-path churn after commands rewrote committed `/home/yukatayu/...` paths to local `/home/codex/...`; the diffs were path-only.
- Updated `progress.md` recent log and `samples_progress.md` recent validation log.
- Created this report.

## Files changed

- Rust clippy hardening:
  - `crates/mirrorea-core/src/runtime.rs`
  - `crates/mir-ast/src/surface_alpha.rs`
  - `crates/mir-ast/examples/surface_mir_alpha_parse.rs`
  - `crates/mir-ast/examples/textual_mir_alpha_parse.rs`
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
  - `crates/mir-ast/tests/support/practical_alpha1_checker_support.rs`
  - `crates/mir-semantics/examples/full_system_v1_check.rs`
  - `crates/mir-semantics/examples/surface_indexed_state_check.rs`
  - `crates/mir-semantics/examples/surface_role_admission_check.rs`
  - `crates/mir-semantics/examples/surface_to_core_elaborate.rs`
  - `crates/mir-semantics/src/full_system_v1/checker.rs`
  - `crates/mir-semantics/src/full_system_v1/projection.rs`
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/src/surface_indexed_state.rs`
  - `crates/mir-semantics/src/surface_to_core_elaboration.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `crates/mir-runtime/examples/posegraph_runtime_session.rs`
  - `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`
  - `crates/mir-runtime/src/clean_near_end.rs`
  - `crates/mir-runtime/src/current_l2.rs`
  - `crates/mir-runtime/src/full_system_v1_local_split.rs`
  - `crates/mir-runtime/src/full_system_v1_provider_admission.rs`
  - `crates/mir-runtime/src/full_system_v1_renderer_pose_backend.rs`
  - `crates/mir-runtime/src/posegraph_runtime.rs`
  - `crates/mir-runtime/src/practical_alpha09_devtools.rs`
  - `crates/mir-runtime/src/practical_alpha1_transport.rs`
  - `crates/mir-runtime/src/product_alpha1_transport.rs`
  - `crates/mir-runtime/tests/full_system_v1_session.rs`
  - `crates/mir-runtime/tests/posegraph_runtime.rs`
  - `crates/mir-runtime/tests/practical_alpha09_devtools.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
  - `crates/mirrorea-cli/src/main.rs`
  - `crates/mirrorea-cli/tests/alpha_cli.rs`
- Snapshot/report:
  - `progress.md`
  - `samples_progress.md`
  - `docs/reports/2112-broad-build-execution-validation-audit.md`
- Not committed / restored:
  - three `samples/full-system-v1/provider-adapter/renderer-pose-*/generated/provider-admission-report.json` files were rewritten by helpers with local absolute paths, then restored because diffs were path-only.

## Commands run

Representative command groups and logs:

- Discord baseline:
  - `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- Resource/toolchain:
  - `df -h .`
  - `free -h`
  - `lsblk -f`
  - `findmnt`
  - `du -sh . target .git .cargo .lake`
  - `python3 --version`
  - `cargo --version`
  - `rustc --version`
  - `rustfmt --version`
  - `docker ps`
  - `docker compose version`
  - `lean --version` / `lake --version` / `elan --version`
- Docs:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `python3 -m unittest scripts.tests.test_validate_docs`
- Cargo:
  - `cargo fmt --check`
  - `cargo check --workspace --all-targets`
  - `cargo build --workspace --all-targets`
  - `cargo test --workspace --all-targets --no-fail-fast`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - focused package clippy/test reruns for `mir-ast`, `mir-semantics`, `mir-runtime`, `mirrorea-core`, and `mirrorea-cli` while fixing warnings.
- Python unit tests:
  - `python3 -m unittest discover -s scripts/tests`
- Current-L2 / clean-near-end / helper families:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - `python3 scripts/clean_near_end_samples.py smoke-all --format json`
  - `python3 scripts/clean_near_end_samples.py run typing --format json`
  - `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  - `python3 scripts/clean_near_end_samples.py run model-check --format json`
  - `python3 scripts/clean_near_end_samples.py run modal --format json`
  - `python3 scripts/clean_near_end_samples.py matrix --format json`
  - `python3 scripts/avatar_follow_samples.py check-all --format json`
  - `python3 scripts/typed_external_boundary_samples.py check-all --format json`
  - `python3 scripts/network_transport_samples.py check-all --format json`
  - `python3 scripts/projection_codegen_samples.py check-all --format json`
  - `python3 scripts/visual_debugger_viewer_samples.py check-all --format json`
  - `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
- Sugoroku:
  - `python3 scripts/sugoroku_world_samples.py list`
  - `python3 scripts/sugoroku_world_samples.py check-all`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes`
  - `python3 scripts/sugoroku_world_samples.py model-check`
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
- Practical / Product Alpha:
  - `python3 scripts/practical_alpha1_check.py check-all --format json`
  - `python3 scripts/practical_alpha1_run_local.py check-all --format json`
  - `python3 scripts/practical_alpha05_session.py check-all --format json`
  - `python3 scripts/practical_alpha08_session_hotplug.py check-all --format json`
  - `python3 scripts/practical_alpha09_devtools.py check-all --format json`
  - `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`
  - `python3 scripts/practical_alpha1_attach.py check-all --format json`
  - `python3 scripts/practical_alpha1_transport.py check-all --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
  - `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - `python3 scripts/practical_alpha1_avatar.py check-all --format json`
  - `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
  - `python3 scripts/operational_product_samples.py list --format json`
  - `python3 scripts/operational_product_samples.py check-all --format json`
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mir-validation-20260625-164803/outputs-after-clippy-fixes/mirrorea-alpha1-release`
  - `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mir-validation-20260625-164803/outputs-after-clippy-fixes/mirrorea-alpha1-installed-binary-check`
- Full System V1 / Surface:
  - `python3 scripts/textual_mir_samples.py check-all --format json`
  - `python3 scripts/full_system_v1_samples.py matrix --format json`
  - `python3 scripts/full_system_v1_samples.py check-all --format json`
  - `python3 scripts/full_system_v1_samples.py operational-matrix --format json`
  - `python3 scripts/full_system_v1_samples.py check-operational-all --format json`
  - `python3 scripts/posegraph_runtime_samples.py check-all --format json`
  - `python3 scripts/projection_v1_samples.py check-all --format json`
  - `python3 scripts/provider_admission_samples.py check-all --format json`
  - `python3 scripts/renderer_pose_backend_samples.py check-all --format json`
  - `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mir-validation-20260625-164803/outputs-after-clippy-fixes/mirrorea-full-v1-release`
  - `python3 scripts/surface_mir_samples.py matrix --format json`
  - `python3 scripts/surface_mir_samples.py check-all --format json`
  - `python3 scripts/surface_mir_authoring_check.py check-all --format json`
  - `python3 scripts/surface_mir_release_check.py --format json plan --out /tmp/mir-validation-20260625-164803/outputs/mirrorea-surface-release`
  - `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mir-validation-20260625-164803/outputs-after-clippy-fixes/mirrorea-surface-release`
- Lean / formal hook:
  - `python3 scripts/current_l2_lean_sample_sync.py --help`
  - `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e1-place-atomic-cut --plan-only --artifact-root /tmp/mir-validation-20260625-164803/lean/artifacts`
  - `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e1-place-atomic-cut --artifact-root /tmp/mir-validation-20260625-164803/lean/artifacts --run-label theorem-lean-stub-e1-validation`
- Direct CLI:
  - `cargo run -q -p mirrorea-cli -- --help`
  - `cargo run -q -p mirrorea-cli -- --format json check samples/product-alpha1/demo/package.mir.json`
  - `cargo run -q -p mirrorea-cli -- --format json demo --skip-docker --out /tmp/mir-validation-20260625-164803/cli/mirrorea-alpha-demo-skip-docker`
  - `cargo run -q -p mirrorea-cli -- --format json view /tmp/mir-validation-20260625-164803/cli/mirrorea-alpha-demo-skip-docker/viewer --check`
  - `cargo run -q -p mirrorea-cli -- --format json view /tmp/mir-validation-20260625-164803/cli/mirrorea-alpha-demo-skip-docker/devtools --check`
  - `/tmp/mir-validation-20260625-164803/cli/mirrorea-alpha-demo-skip-docker/native-bundle/run.sh`
- Final closeout after reviewer fixes:
  - `make check`
  - `git diff --check`
  - redacted tracked-diff webhook marker scan

## Evidence / outputs / test results

- Log/output root: `/tmp/mir-validation-20260625-164803`.
- Cargo final post-fix pass:
  - `cargo_fmt_check_after_clippy_fixes.log`: pass.
  - `cargo_check_workspace_all_targets_after_clippy_fixes.log`: pass.
  - `cargo_build_workspace_all_targets_after_clippy_fixes.log`: pass.
  - `cargo_test_workspace_all_targets_after_clippy_fixes.log`: pass.
  - `cargo_clippy_workspace_all_targets_after_final_cargo_loop.log`: pass.
- Python final post-fix pass:
  - `python_unittest_discover_scripts_tests_after_clippy_fixes.log`: `Ran 638 tests ... OK`.
- Post-fix sample/release pass logs:
  - `post-fix-samples/current_l2_smoke_all.log`
  - `post-fix-samples/current_l2_closeout.log`
  - `post-fix-samples/clean_near_end_smoke_all.log`
  - `post-fix-samples/clean_near_end_matrix.log`
  - `post-fix-samples/practical_alpha09_devtools_check_all.log`
  - `post-fix-samples/practical_alpha1_transport_check_all.log`
  - `post-fix-samples/practical_alpha1_product_preview_check_all.log`
  - `post-fix-samples/full_system_v1_check_all.log`
  - `post-fix-samples/full_system_v1_check_operational_all.log`
  - `post-fix-samples/posegraph_runtime_check_all.log`
  - `post-fix-samples/projection_v1_check_all.log`
  - `post-fix-samples/provider_admission_check_all.log`
  - `post-fix-samples/renderer_pose_backend_check_all.log`
  - `post-fix-samples/surface_mir_check_all.log`
  - `post-fix-samples/surface_mir_authoring_check_all.log`
  - `post-fix-samples/product_alpha1_release_check.log`
  - `post-fix-samples/product_alpha1_installed_binary_check.log`
  - `post-fix-samples/full_system_v1_release_check.log`
  - `post-fix-samples/surface_mir_release_check.log`
- Direct CLI evidence:
  - `cli/mirrorea_alpha_check_demo.log`: pass.
  - `cli/mirrorea_alpha_demo_skip_docker.log`: pass; payload status `partial`, as expected for bounded alpha/non-final product.
  - `cli/mirrorea_alpha_view_demo_devtools.log`: pass.
  - `cli/mirrorea_alpha_native_bundle_run_script.log`: pass.
  - `cli/mirrorea_alpha_help.log`: unsupported command, exit 2; recorded as expected current CLI limitation.
  - `cli/mirrorea_alpha_view_demo.log`: failed because the guessed path `viewer/` was wrong; corrected to `devtools/` and passed.
- Lean evidence:
  - `lean/current_l2_theorem_lean_stub_pipeline_e1_plan_only.log`: pass.
  - `lean/current_l2_theorem_lean_stub_pipeline_e1_execute.log`: pass.
  - `lean/lean_toolchain_path_check.log`: `lean`, `lake`, and `elan` not found.
- Final after-review closeout evidence:
  - `final-after-review/make_check_after_review.log`: pass.
  - `final-after-review/git_diff_check_after_review.log`: pass.
  - `final-after-review/secret_diff_scan_after_review.log`: pass; no forbidden webhook marker in tracked diff.
- Generated output sizes:
  - `/tmp/mir-validation-20260625-164803`: about 735M after full audit.
  - `/tmp/mir-validation-20260625-164803/outputs-after-clippy-fixes`: contains Product Alpha, installed-binary, Full System V1, and Surface release-check outputs.
- Tracked generated provider JSON path churn was inspected and restored; no content change was committed there.

## What changed in understanding

- The repository has a broad runnable surface that is already heavily helper-driven. The active executable roots align with the docs: `samples/clean-near-end/`, `samples/current-l2/`, `samples/lean/`, Product Alpha, Full System V1, and Surface alpha helper families.
- `cargo clippy --workspace --all-targets -- -D warnings` was not previously clean; it is now clean after implementation/test helper hardening.
- `mirrorea-alpha --help` is not implemented as a human help surface; unknown commands return structured JSON with `diagnostic_code: unknown_command`.
- Product Alpha direct `demo --skip-docker` writes viewer assets under `devtools/`, not `viewer/`.
- Several helper commands rewrite committed generated reports with machine-local absolute paths. This is path-only churn and should be treated as generated artifact hygiene, not semantic evidence.
- Lean stub artifact generation can run without the Lean compiler, but actual Lean compiler validation cannot run in this environment because the Lean toolchain is absent.

## Open questions

- Should generated Full System V1 provider-admission reports stop embedding absolute workstation paths, or should committed generated reports be regenerated with repo-relative paths in a dedicated task?
- Should `mirrorea-alpha --help` become a supported command, or is structured `unknown_command` output intentionally sufficient for the current alpha CLI?
- Should `cargo clippy --workspace --all-targets -- -D warnings` be added to the documented default validation floor?

## Suggested next prompt

Run a focused follow-up to make generated provider-admission reports path-stable and decide whether workspace clippy should become an official required gate.

## Plan update status

`plan/` 更新不要: no semantics, roadmap, open-question, syntax, or long-term repository-memory decision changed. The audit only added validation evidence and clippy hardening.

## Documentation.md update status

`Documentation.md` 更新不要: no reader-facing architecture/status entrypoint changed. The runnable surface evidence was recorded in `progress.md`, `samples_progress.md`, and this report.

## progress.md update status

`progress.md` 更新済み: added a 2026-06-25 recent-log entry for broad build/execution audit and Lean compiler skip reason.

## tasks.md update status

`tasks.md` 更新不要: no current task map, blocker, or next self-driven package changed. Follow-up ideas are recorded as open questions here, not promoted as task-map changes.

## samples_progress.md update status

`samples_progress.md` 更新済み: added a Recent Validation Log row for the broad executable audit / clippy hardening.

## Reviewer findings and follow-up

- Final read-only reviewer: `019efdd9-a707-7e53-89a1-28641e5ccce0`.
- Finding: report listed non-existent shorthand spec names in `Documents consulted`.
  Follow-up: corrected to the actual ordered filenames under `specs/00-document-map.md`, `01-charter-and-decision-levels.md`, `02-system-overview.md`, `03-layer-model.md`, and `09-invariants-and-constraints.md`.
- Finding: report still had pending reviewer / sub-agent closeout sections.
  Follow-up: this section and the sub-agent closeout section were completed after review.
- Finding: `progress.md` and `samples_progress.md` top-level update timestamps were stale relative to the new 2026-06-25 validation rows.
  Follow-up: both top-level timestamps were updated to `2026-06-25 17:12 JST`.
- Reviewer did not find a blocking Rust behavior regression, webhook/token-like strings in the patch, or remaining provider-generated path churn.

## Skipped validations and reasons

- Lean compiler mechanization (`lean` / `lake` / `elan`) was skipped because none of those commands are on `PATH`.
- `current_l2_lean_sample_sync.py --help` could not show help in this environment because it attempts to call `lean` before completing.
- `mirrorea-alpha --help` exited 2 with structured `unknown_command`; this is recorded as current CLI behavior, not treated as a passing help validation.
- A guessed `mirrorea-alpha view .../viewer --check` path failed because the generated demo viewer assets are actually under `devtools/`; the corrected `devtools/` path passed.
- Full production WAN/federation, final public API/ABI, and arbitrary native provider execution are outside the current repo stage and were not claimed.

## Commit / push status

Pre-commit status at report close: final local validation has passed, and this report is intentionally included before the commit. Commit will use `git commit --no-gpg-sign`; push and post-push commit漏れ verification will be recorded in the final response.

## Sub-agent session close status

- Explorer `019efdbe-04ac-7281-a530-5804a38d031b`: completed runnable command inventory; closed.
- Explorer `019efdbe-1c18-7bd3-abe2-cce1517fa613`: completed Rust/Cargo inventory; closed.
- Explorer `019efdbe-352a-7940-ba33-cf13967694dc`: completed non-Rust/tooling inventory; closed.
- Reviewer `019efdd9-a707-7e53-89a1-28641e5ccce0`: completed final read-only review; findings addressed; closed.
