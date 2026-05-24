# 2105 P-SURF-05 Role Admission Capability Grant

- Date: 2026-05-24
- Author / agent: Codex
- Scope: Surface Mir P-SURF-05 role admission capability grant
- Decision levels touched: L2 alpha implementation evidence, L3 runtime identity non-claim

## Objective

- Identifier: `P-SURF-05 role admission capability grant`
- Package: Surface Mir brace complete autonomous implementation
- Report path: `docs/reports/2105-p-surf-05-role-admission-capability-grant.md`

Implement the narrow Surface Mir role-admission evidence floor after P-SURF-04: role claims must be recorded separately from authority, join admission must produce request / verdict / witness / capability-grant rows, stale membership must not resurrect authority, and optional package/runtime hash binding must remain metadata rather than semantic safety proof.

## Scope and assumptions

- Scope is report-level Surface evidence only. This package does not claim production identity provider, hardware attestation, WAN/federation admission, runtime membership lifecycle, leave/rejoin execution, grant retirement, final auth schema, runtime dispatch, or source patch activation.
- `.mir` files remain semantic source authority; `package.mir.json` remains an alpha artifact.
- `BrowserClient[self] { join World as BrowserClient via WorldAdmission }` is accepted as an admission request in this alpha lane.
- Authority-bearing writes require an active admission-derived `WriteState(<place>)` grant. The role string and indexed-state key are not authority.
- Stale membership rows are modeled as report-level fences for the same principal/target place inside the checked role-instance block.

## Start state / dirty state

Started from the pushed P-SURF-04 closeout on branch `main` with untracked `sub-agent-pro/surface-mir-brace-completion-001/` present. That handoff directory was intentionally left untracked and unstaged. P-SURF-05 began with clean committed P-SURF-04 sources plus local P-SURF-05 edits made in this task.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`
- `sub-agent-pro/surface-mir-brace-completion-001/*.md`
- `sub-agent-pro/surface-mir-brace-completion-001/sample-blueprints/*.md`
- Reviewer findings from sub-agent `019e5954-fe95-7481-b65d-c7842591c8d0`

## Actions taken

- Added `crates/mir-semantics::surface_role_admission` with report structs for role claims, admission requests, accepted verdicts, capability grants, admission witnesses, authority checks, stale membership rejections, optional hash bindings, diagnostics, and obligations.
- Added the `surface_role_admission_check` example for JSON/pretty source checks.
- Added `crates/mir-semantics/tests/role_admission_capability_grant.rs`.
- Added `samples/full-system-v1-surface/role-admission/` with `ROLE-01..04`.
- Extended `ROLE-01` to prove a grant-backed World-owned indexed-state write is accepted.
- Extended `ROLE-02` to prove a role claim without grant cannot write World-owned indexed state.
- Extended `ROLE-03` after review so stale membership fences the prior grant and rejects a post-stale write.
- Added `ROLE-04` to keep package/runtime hash binding as metadata with `semantic_safety_proof=false`.
- Updated `scripts/surface_mir_samples.py`, `scripts/surface_mir_release_check.py`, and script tests for the 28-row Surface matrix.
- Updated documentation snapshots, specs, and plans to mark P-SURF-05 closed and `P-SURF-06 source patch hot-plug` next.

## Files changed

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/surface_role_admission.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/examples/surface_role_admission_check.rs`
- `crates/mir-semantics/tests/role_admission_capability_grant.rs`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `samples/full-system-v1-surface/role-admission/**`
- `samples/full-system-v1-surface/elaboration/*/expected/elaboration.json`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/full-system-v1-surface/README.md`
- `scripts/README.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `specs/00-document-map.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/66-role-admission-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt --check`
- `cargo fmt`
- `git diff --check`
- `cargo test -p mir-ast --test surface_mir_parser -- --nocapture`
- `cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-05`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-05`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`

## Evidence / outputs / test results

- `scripts.tests.test_validate_docs`: 18 tests passed.
- `scripts/check_source_hierarchy.py`: 480 required paths present, 0 missing.
- `scripts/validate_docs.py`: documentation scaffold complete after report creation; 1257 numbered reports found.
- Initial `cargo fmt --check` found one formatting diff in `surface_role_admission.rs`; `cargo fmt` was applied and `cargo fmt --check` then passed.
- `git diff --check`: passed.
- `mir-ast surface_mir_parser`: 13 tests passed.
- `mir-semantics indexed_state_semantics`: 7 tests passed.
- `mir-semantics surface_to_core_elaboration`: 13 tests passed.
- `mir-semantics role_admission_capability_grant`: 4 tests passed.
- Initial Surface helper unit test run failed because `ROLE-03` still expected only `stale_membership_message_rejected`; the test was updated to also expect `stale_membership_authority_rejected`, and the rerun passed 30 tests.
- `scripts/surface_mir_samples.py check-all`: 28 rows passed, failed `[]`, workflow-ready `false`.
- `scripts/surface_mir_authoring_check.py check-all`: 28 sources accepted as `.mir` authority.
- `scripts/surface_mir_release_check.py`: ready `true`, 13 results, failed commands `[]`.
- `scripts/product_alpha1_release_check.py`: status `accepted`, 29 commands passed, failed commands `[]`.
- `scripts/operational_product_samples.py`: status `accepted`, failed commands `[]`.
- `scripts/minimal_alpha1_patterns.py`: status `accepted`, strict family count 4, failed `[]`.

## What changed in understanding

P-SURF-05 can close as a report-level evidence floor only if the positive and negative authority paths are both explicit: the accepted write must be tied to an admission grant, and stale membership must fence a prior grant before any later write check. Hash binding remains useful metadata, but it cannot be treated as semantic safety proof without a later attestation spec.

## Open questions

- Runtime identity and membership lifecycle remain later work.
- Leave/rejoin execution, grant retirement, and durable membership history are not implemented.
- There is no sample for an admitted member with a deliberately withheld narrower capability; current negative coverage is claim-without-grant and stale-grant fence.
- Production identity provider, hardware attestation, and WAN/federation admission remain outside this package.
- Source patch activation remains pending for P-SURF-06.

## Suggested next prompt

`P-SURF-06 source patch hot-plug`

## Plan update status

Updated `plan/00-index.md`, `plan/66-role-admission-roadmap.md`, and `plan/68-surface-full-system-v1-roadmap.md` to mark P-SURF-05 as closed report-level role admission evidence and P-SURF-06 as the next promoted package.

## Documentation.md update status

Updated. `Documentation.md` now identifies P-SURF-05 as the role admission / capability grant evidence floor and keeps runtime dispatch, production identity, hardware attestation, WAN admission, and source patch hot-plug as non-claims.

## progress.md update status

Updated. `progress.md` records P-SURF-05 closure, the 28-row Surface matrix, current runnable commands, implemented samples, current non-claims, and next gap `P-SURF-06`.

## tasks.md update status

Updated. `tasks.md` now makes `P-SURF-06 source patch hot-plug` the current promoted autonomous package while preserving P-SURF-05 as report-level evidence only.

## samples_progress.md update status

Updated. `samples_progress.md` records the role-admission evidence root, the 28-row Surface sample matrix, grant-backed accepted write, missing-grant rejection, stale post-grant write fence, and current validation commands.

## Reviewer findings and follow-up

- Code mapper sub-agent `019e5941-763c-7962-929d-e7baef53c5ee` found the expected role admission surfaces and advised keeping ROLE-03 report-level unless runtime membership lifecycle was explicitly implemented.
- Reviewer sub-agent `019e5954-fe95-7481-b65d-c7842591c8d0` reported:
  - stale membership rejection was logged but did not fence later authority checks;
  - `plan/66-role-admission-roadmap.md` did not make P-SURF-05 closed / P-SURF-06 next explicit enough and omitted a ROLE-04 validation anchor.
- Follow-up implemented:
  - stale membership now clears active grants and inserts a same-principal/same-place fence;
  - stale post-grant writes produce `stale_membership_authority_rejected` with authority check reason `stale_membership`;
  - `ROLE-03` source, expected JSON, Rust test, and Python test now cover the post-stale write rejection;
  - `plan/66-role-admission-roadmap.md` now states P-SURF-05 closed, P-SURF-06 next, and includes a ROLE-04 helper anchor.

## Skipped validations and reasons

No requested P-SURF-05 validations were skipped. P-SURF-06 source patch CLI/runtime commands are not run because that package remains next in sequence.

## Commit / push status

Pending at report creation. The intended commit message is `p-surf-05: add role admission evidence`; final commit hash and push status are reported in the package close response.

## Sub-agent session close status

- `019e5941-763c-7962-929d-e7baef53c5ee`: completed code-mapper pass and was closed after package validation.
- `019e5954-fe95-7481-b65d-c7842591c8d0`: completed reviewer pass and was closed after follow-up fixes.
