# Report 2081 — P-POSE-02 no-split-frame helper evidence

- Date: 2026-05-21 22:02:49 +0900
- Author / agent: Codex
- Scope: `P-POSE-02` bounded PoseGraph helper evidence, sample actualization, helper/test sync, snapshot-doc sync
- Decision levels touched: `L1` existing PoseGraph direction, `L2` helper/sample realization

## Objective

Close `P-POSE-02` by actualizing the smallest honest PoseGraph evidence package:

- one accepted same-client same-observation-snapshot no-split-frame row
- one negative split-snapshot mismatch row exported as machine-readable `violation_export`
- seven residual PoseGraph rows kept explicitly `planned_only`

while preserving existing Product Alpha-1 operational evidence and avoiding overclaim about global simultaneity, full PoseGraph runtime completion, save/load admissibility, or devtools-panel completion.

## Scope and assumptions

- Keep `specs/29` / `plan/54` reading fixed:
  no-split-frame means same client session, same observation snapshot, same `pose_version`.
- Realize `P-POSE-02` through `scripts/posegraph_samples.py` plus helper-only `package.mir.json` inputs under `samples/product-alpha1/posegraph/`.
- Do not extend the direct Product Alpha-1 runtime/CLI surface in this package.
- Do not claim workflow-ready PoseGraph runtime, global cross-client simultaneity, renderer-owned semantics, pose-aware save/load completion, or Unity/VRM compatibility.

## Start state / dirty state

- Start point was `main` after `P-COMP-04` (`49a3e880`) had already been committed and pushed.
- Work resumed with a pre-written RED diff in `scripts/tests/test_posegraph_samples.py` that expected `pose-04` to become `accepted` and `pose-05` to become `violation_export`.
- No unrelated user changes were reverted.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/29-transform-posegraph-semantics.md`
- `specs/32-autonomous-execution-and-completion-contract.md`
- `plan/00-index.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/54-transform-posegraph-roadmap.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `docs/hands_on/README.md`
- `docs/hands_on/transform_posegraph_01.md`
- `docs/research_abstract/mir_computational_core_01.md`
- `sub-agent-pro/mirrorea_mir_computational_core_handoff.md`

## Actions taken

- Kept the RED-first test expectations in `scripts/tests/test_posegraph_samples.py` and verified they failed against the old scaffold-only helper.
- Promoted `samples/product-alpha1/posegraph/matrix.json` from all-planned to mixed status:
  `pose-04` and `pose-05` are now executable helper rows with explicit `execution_surface`, `package_input`, and `expected_outcome`.
- Added helper-only executable manifests:
  `samples/product-alpha1/posegraph/no-split-frame-positive/package.mir.json`
  and
  `samples/product-alpha1/posegraph/split-frame-negative/package.mir.json`.
- Extended `scripts/posegraph_samples.py` to:
  validate executable `package_input`,
  materialize package metadata,
  classify accepted rows and violation rows in `matrix`,
  execute a helper-only no-split-frame contract,
  return `accepted` for same-snapshot same-version rows,
  return `violation_export` with stable `violation_kind = no_split_frame` and machine-readable detail for mismatched rows,
  and synchronize `check-all`, `closeout`, and pretty output.
- Synchronized overview docs, sample catalog docs, hands-on docs, repository-memory docs, taxonomy docs, and dashboards to reflect:
  1 accepted PoseGraph row,
  1 `violation_export` row,
  7 planned rows,
  and the next reopen point `all-up closeout audit`.
- Preserved `P-POSE-01` as historical scaffold closeout rather than overwriting that history.

## Files changed

- PoseGraph sample/helper:
  `samples/product-alpha1/posegraph/matrix.json`
  `samples/product-alpha1/posegraph/README.md`
  `samples/product-alpha1/posegraph/no-split-frame-positive/README.md`
  `samples/product-alpha1/posegraph/no-split-frame-positive/package.mir.json`
  `samples/product-alpha1/posegraph/split-frame-negative/README.md`
  `samples/product-alpha1/posegraph/split-frame-negative/package.mir.json`
  `scripts/posegraph_samples.py`
  `scripts/tests/test_posegraph_samples.py`
- Snapshot/docs/indexes:
  `README.md`
  `Documentation.md`
  `progress.md`
  `tasks.md`
  `samples_progress.md`
  `samples/README.md`
  `samples/product-alpha1/README.md`
  `scripts/README.md`
  `docs/hands_on/README.md`
  `docs/hands_on/transform_posegraph_01.md`
  `docs/research_abstract/mir_computational_core_01.md`
  `plan/00-index.md`
  `plan/19-repository-map-and-taxonomy.md`
  `plan/54-transform-posegraph-roadmap.md`
  `plan/57-autonomous-computational-core-master-plan.md`
  `specs/00-document-map.md`
  `specs/29-transform-posegraph-semantics.md`

## Commands run

```bash
git status --short
git diff -- scripts/tests/test_posegraph_samples.py
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 -m py_compile scripts/posegraph_samples.py
date '+%Y-%m-%d %H:%M:%S %z'
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_posegraph_samples`
  passed 10/10 after the helper/matrix sync.
- `python3 scripts/posegraph_samples.py matrix --format json`
  reported:
  9 total rows,
  7 planned rows,
  2 executable rows,
  1 accepted row,
  1 violation row,
  `matrix_status = "mixed"`,
  `workflow_ready = false`.
- `python3 scripts/posegraph_samples.py check-all --format json`
  passed with:
  `accepted = ["pose-04-no-split-frame-positive"]`,
  `violations = ["pose-05-split-frame-negative"]`,
  `failed = []`.
- `python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json`
  returned:
  `terminal_outcome = "accepted"`,
  `target_pose_version = 17`,
  `anchored_pose_version = 17`,
  `pose_snapshot_ref = "snapshot#avatar-017"`,
  `outcome_matches_expected = true`.
- `python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json`
  returned:
  `terminal_outcome = "violation_export"`,
  `violation_kind = "no_split_frame"`,
  detail containing `snapshot mismatch` and the mismatched `pose_version`,
  `outcome_matches_expected = true`.
- `python3 -m unittest scripts.tests.test_validate_docs`
  passed 14/14.
- `python3 scripts/check_source_hierarchy.py`
  passed with `required = 235`, `missing = 0`.
- `python3 scripts/validate_docs.py`
  passed and reported documentation scaffold complete.
- `cargo fmt --check`
  passed.
- `git diff --check`
  passed.
- `python3 -m py_compile scripts/posegraph_samples.py`
  passed.

## What changed in understanding

- The honest first PoseGraph realization is not a broad runtime integration. It is a mixed helper evidence line with one accepted row, one violation row, and explicit residual planned rows.
- The negative carrier must stay named `violation_export` consistently; mixing it with `runtime_rejection` wording would blur the intended boundary.
- The repo already had the right same-snapshot equality semantics in representative `.mir` sketches and RED tests; the missing work was synchronizing helper, matrix, and docs around that boundary.

## Open questions

- Pose-aware save/load admissibility remains later work.
- PoseGraph devtools panel family and export surface remain later work.
- Anchor-switch / stale-anchor reacquire negative rows remain later work.
- The next promoted package is the all-up closeout audit.

## Suggested next prompt

all-up closeout audit を実施し、current package chain 全体の docs / samples / validators / reports / non-claims / commit-push 状態まで同期して閉じてください。

## Plan update status

`plan/` 更新済み:
`plan/00-index.md`, `plan/19-repository-map-and-taxonomy.md`, `plan/54-transform-posegraph-roadmap.md`, `plan/57-autonomous-computational-core-master-plan.md`

## Documentation.md update status

`Documentation.md` 更新済み:
`P-POSE-02` closeout, PoseGraph mixed/helper evidence wording, next reopen point `all-up closeout audit`

## progress.md update status

`progress.md` 更新済み:
latest closeout package, PoseGraph line status, current blockers, next reopen point, recent log

## tasks.md update status

`tasks.md` 更新済み:
`P-POSE-02` closed, ordered self-driven queue reduced to all-up closeout audit

## samples_progress.md update status

`samples_progress.md` 更新済み:
PoseGraph row status, root status, validation log, current repo-local focus

## Reviewer findings and follow-up

- Explorer `Pauli` identified the reusable contract shape already implicit in the RED tests and pointed to existing snapshot/devtools carrier patterns. The implementation reused the accepted/violation/planned matrix shape and kept future devtools export explicitly later.
- Explorer `Anscombe` identified the full doc-drift set that would become stale once `pose-04/05` were executable. Those docs were updated in the same package.
- Reviewer `Archimedes` was started for final diff review, but it did not return within two waits and was closed. Final closeout therefore relies on local diff inspection plus the focused validation evidence above.

## Skipped validations and reasons

- Full product alpha release check, installed-binary adoption probe, operational suite check-all, and Docker-backed flows were not rerun in this package because `P-POSE-02` only changed the helper-only PoseGraph line plus snapshot docs and did not touch those runtime paths.
- No Rust runtime test was added in this package because the bounded implementation choice deliberately stayed on the helper side instead of extending the direct Product Alpha-1 runtime.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `019e4a95-1475-7643-9b55-67aad8f5b845` (`Pauli`): completed and closed
- `019e4a98-99ff-7ff2-91d6-3adf086e0d8e` (`Anscombe`): completed and closed
- `019e4aa0-63fd-70e0-be11-e1e2a391282d` (`Archimedes`): closed after timeout without final review output
