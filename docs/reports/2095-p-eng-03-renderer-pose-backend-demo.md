# 2095 — P-ENG-03 renderer pose backend demo

## Objective

Close `P-ENG-03 renderer pose backend demo` by actualizing a bounded Full System V1 renderer lane above provider admission plus PoseGraph runtime, proving 1 accepted renderer row and 2 blocked rows, synchronizing runtime/CLI/helper/sample/doc surfaces, and promoting the roadmap snapshot to `P-FSV1-01 source operational suite`.

## Scope and assumptions

- Scope is limited to `P-ENG-03` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- Semantic source of truth remains `.mir` source plus typed IR; `package.mir.json` remains alpha compatibility/package artifact only.
- This package is allowed to actualize:
  - bounded renderer pose delivery above admitted provider manifests and bounded PoseGraph runtime evidence
  - 1 accepted renderer row
  - 2 blocked renderer rows for split-frame violation and explicit reacquire requirement
  - runtime/example/CLI/helper/generated-report evidence for the renderer lane
  - snapshot/roadmap promotion to `P-FSV1-01`
- Safe-side narrowing used in this package:
  - accepted renderer evidence is structural `binding_context` plus `pose_snapshot_frontier` agreement, not attested package provenance
  - PoseGraph remains semantic owner of pose acceptance/rejection
  - generic same-process session execution must not silently admit renderer boundaries without local-split/provider context
  - renderer execution remains bounded delivery evidence only, not arbitrary native/WASM/provider execution
- This package does not claim final provider ABI, arbitrary native/WASM execution, renderer-owned world semantics, attested PoseGraph package provenance, final packet/FFI transport semantics, final server/client binary split, LLVM/native codegen completion, WAN/federation, distributed durable save/load R3/R4, Unity/Unreal provider execution, or final public ABI/SDK.

## Start state / dirty state

- Branch: `main`
- Start point: after `P-ENG-02` closeout (`03eea411`) had been committed and pushed.
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-ENG-03` draft edits in runtime/CLI/doc files
  - new in-scope renderer files and sample roots already existed as draft package work
  - those edits were treated as package work and were not reverted
- During closeout, reviewer findings showed the first draft over-claimed source/PoseGraph binding and widened generic runtime gating too far; the package was recut before final report/commit.

## Documents consulted

- Repository policy/context:
  - `AGENTS.md`
  - `.docs/progress-task-axes.md`
- Core repo docs:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Full System V1 specs:
  - `specs/33-full-system-v1-scope.md`
  - `specs/34-textual-mir-alpha-grammar.md`
  - `specs/35-mir-typed-ir-and-interpreter.md`
  - `specs/36-projection-ir-and-boundary-preservation.md`
  - `specs/37-posegraph-runtime-semantics.md`
  - `specs/38-engine-provider-admission.md`
- Full System V1 plans:
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/59-textual-mir-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `plan/62-projection-backend-roadmap.md`
  - `plan/63-engine-provider-roadmap.md`
- Handoff package:
  - `sub-agent-pro/full-system-completion-001/*.md`

## Actions taken

1. Added and exported `crates/mir-runtime::full_system_v1_renderer_pose_backend` as a bounded renderer wrapper over `run_full_system_v1_provider_admission_path` plus `run_posegraph_runtime_package_path`.
2. Added renderer samples, matrix, expected/generated reports, runtime example, CLI command, runtime tests, CLI tests, helper script, and helper tests for:
  - `eng-03-renderer-pose-positive`
  - `eng-03-renderer-pose-split-frame-negative`
  - `eng-03-renderer-pose-reacquire-negative`
3. Widened provider admission for `provider_kind = renderer` with explicit input/output schema names and inventory-only target policy.
4. Added the initial renderer boundary interpreter receipt path, then recut it after reviewer findings:
  - renderer boundaries were removed from unconditional generic admission
  - same-process runtime now admits renderer boundaries only when local split passes explicit outbound boundary refs into the session
5. Added red-first regression coverage for the reviewer findings:
  - unrelated PoseGraph package without binding context must be blocked
  - direct session execution of `renderer_frame_packet` without admission context must be rejected
  - CLI must cover the `reacquire_required` blocked row
  - helper must execute the `mirrorea-alpha render-pose-backend-v1` CLI surface rather than only the runtime example
6. Repaired the semantic gap by introducing structural `binding_context` matching between:
  - source/provider/local-split-derived expected context
  - PoseGraph package declared context
  - accepted snapshot frontier agreement
7. Added `posegraph_binding_attestation_deferred` as an explicit residual obligation so docs/reporting do not over-claim attested provenance.
8. Recut `README.md`, `Documentation.md`, `progress.md`, `samples_progress.md`, reader summaries, sample/script READMEs, and plan files to:
  - remove stale “renderer remains later” wording
  - describe the accepted row as structural `binding_context` + frontier evidence
  - keep package provenance and broader execution as non-claims
9. Re-ran package-specific validations, docs/source validators, and the required existing major anchors. `cargo fmt` was required once before rerunning the broad alpha workflows because both `product_alpha1_release_check.py` and `operational_product_samples.py` surface `cargo fmt --check`.

## Files changed

- Rust source/runtime exports:
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/full_system_v1_renderer_pose_backend.rs`
  - `crates/mir-runtime/src/full_system_v1_provider_admission.rs`
  - `crates/mir-runtime/src/full_system_v1_local_split.rs`
  - `crates/mir-runtime/src/full_system_v1_session.rs`
  - `crates/mir-runtime/examples/mir_full_system_v1_renderer_pose_backend.rs`
  - `crates/mir-semantics/src/full_system_v1/interpreter.rs`
  - `crates/mir-semantics/src/full_system_v1/mod.rs`
  - `crates/mirrorea-cli/src/main.rs`
- Rust tests:
  - `crates/mir-runtime/tests/renderer_pose_backend.rs`
  - `crates/mir-runtime/tests/full_system_v1_session.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
- Scripts/tests:
  - `scripts/renderer_pose_backend_samples.py`
  - `scripts/tests/test_renderer_pose_backend_samples.py`
- Samples:
  - `samples/full-system-v1/provider-adapter/renderer-pose-matrix.json`
  - `samples/full-system-v1/provider-adapter/renderer-pose-positive/*`
  - `samples/full-system-v1/provider-adapter/renderer-pose-split-frame-negative/*`
  - `samples/full-system-v1/provider-adapter/renderer-pose-reacquire-negative/*`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/full-system-v1/README.md`
  - `scripts/README.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `plan/63-engine-provider-roadmap.md`
  - this report

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M %Z'
cargo test -p mir-runtime --test renderer_pose_backend renderer_pose_backend_blocks_posegraph_package_without_binding_context -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session runtime_session_rejects_renderer_boundary_without_admission_context -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli render_pose_backend_v1_blocks_reacquire_rejection -- --nocapture
python3 -m unittest scripts.tests.test_renderer_pose_backend_samples.RendererPoseBackendSamplesTests.test_helper_executes_cli_surface
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 -m unittest scripts.tests.test_renderer_pose_backend_samples
python3 scripts/renderer_pose_backend_samples.py run eng-03-renderer-pose-positive --format json
python3 scripts/renderer_pose_backend_samples.py run eng-03-renderer-pose-split-frame-negative --format json
python3 scripts/renderer_pose_backend_samples.py run eng-03-renderer-pose-reacquire-negative --format json
python3 scripts/renderer_pose_backend_samples.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo test -p mir-runtime --test projection_ir -- --nocapture
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"
python3 scripts/operational_product_samples.py check-all --format json
cargo fmt
cargo fmt --check
git diff --check
git status --short
git add <package files>
git commit --no-gpg-sign -m "P-ENG-03: renderer pose backend demo"
git push
```

## Evidence / outputs / test results

- Red-first fix evidence:
  - unrelated PoseGraph package without `binding_context` initially reproduced the reviewer’s false accept path; the new runtime test now passes with terminal outcome `blocked_posegraph_binding_context_missing`
  - direct same-process session execution of `renderer_frame_packet` without admission context initially accepted incorrectly; the new session test now passes with `unsupported_effect_runtime`
  - CLI `reacquire_required` coverage now exists and passes
  - helper CLI-surface test now passes after switching from the runtime example to `mirrorea-alpha render-pose-backend-v1`
- Package-specific renderer evidence:
  - `cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture`: passed, 4 tests
  - `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`: passed, 5 tests
  - `cargo test -p mir-runtime --test provider_admission -- --nocapture`: passed, 6 tests
  - `cargo test -p mir-runtime --test projection_ir -- --nocapture`: passed, 9 tests
  - `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`: passed, 10 tests
  - `python3 -m unittest scripts.tests.test_renderer_pose_backend_samples`: passed, 9 tests
  - `python3 scripts/renderer_pose_backend_samples.py check-all --format json`: passed, 3 executable rows matched expected summaries
  - `python3 scripts/provider_admission_samples.py check-all --format json`: passed, 5 executable rows
  - `python3 scripts/projection_v1_samples.py check-all --format json`: passed, 6 executable rows
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: passed after `cargo fmt`
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-XnVBZH`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted

## What changed in understanding

- A bounded renderer lane cannot honestly claim source-owned PoseGraph delivery from a free-standing package path; the safe close condition is narrower:
  - admitted provider/local-split/source context must be retained explicitly
  - PoseGraph package must declare a matching structural `binding_context`
  - snapshot frontier agreement must still hold
  - attested provenance remains later work
- Renderer provider boundaries should not be widened into generic same-process runtime semantics. They need explicit admission context propagated from the local-split/provider layer.
- CLI/helper wording matters here because the package claim is about a public-ish execution surface (`mirrorea-alpha render-pose-backend-v1`), not only an internal example binary.

## Open questions

- No blocker remains for `P-ENG-03`.
- Remaining later work for this lane is explicit rather than implicit:
  - attested PoseGraph package provenance
  - broader source-first operational suites
  - arbitrary native/WASM/provider execution
  - final provider ABI / SDK

## Suggested next prompt

```text
P-FSV1-01 source operational suite
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/61-posegraph-runtime-roadmap.md`
- `plan/63-engine-provider-roadmap.md`

## Documentation.md update status

Updated for the recut closeout so `P-ENG-03` is described as structural binding-context plus frontier evidence, not source-derived PoseGraph ownership transfer.

## progress.md update status

Updated to show:

- `P-ENG-03` re-closeout with reviewer-driven binding-context/gating fixes
- current promoted package `P-FSV1-01`
- next promoted package after current closeout `P-FSV1-02`
- `FS-08` now described with structural binding evidence and explicit provenance non-claim

## tasks.md update status

Retained the earlier `P-ENG-03` package promotion updates:

- current promoted package remains `P-FSV1-01 source operational suite`
- next promoted package remains `P-FSV1-02 portal/shard source samples`

No additional task-map rewrite was required during the re-closeout.

## samples_progress.md update status

Updated to:

- remove stale “renderer remains later” wording from adjacent status rows
- describe the accepted renderer row as binding-context plus frontier evidence
- refresh the recent validation log entry for the final `P-ENG-03` closeout

## Reviewer findings and follow-up

- Explorer `Bacon` completed earlier and its direction was incorporated:
  - keep `samples/product-alpha1/engine-adapter/` inventory-only
  - actualize renderer evidence under `samples/full-system-v1/provider-adapter/`
  - keep the lane bounded and non-owning
- Reviewer `Carson` completed and found 3 issues:
  - missing PoseGraph/source/provider binding check
  - generic interpreter admission regression for renderer boundaries
  - incomplete CLI evidence claim
  - all 3 were addressed before closeout
- Reviewer `Nash` completed and found status/doc drift:
  - stale “renderer remains later” wording
  - missing renderer commands in some command blocks
  - reader summary undercount
  - all drift findings were addressed before closeout

## Skipped validations and reasons

- None.

## Commit / push status

- Report written before commit/push finalization.
- Commit hash: pending
- Push status: pending

## Sub-agent session close status

- Explorer `Bacon`: already completed and closed before final closeout.
- Reviewer `Carson`: completed; findings incorporated; agent closed.
- Reviewer `Nash`: completed; findings incorporated; agent closed.
