# Report 2590 — Mir Theory v0 / I1+ Milestone 9: Auth / verification extension

- Date: 2026-08-04 to 2026-08-05 (started after M8 remote parity)
- Author / agent: parent orchestrator with independent planner, theory/formalization, production implementer, test-author, and reviewer roles
- Scope: ADR-0015 M9 only — typed auth Contract transformation and separate finite verification evidence over the fixed M8 runtime contract
- Start revision: `7f23a72ef1a20be27c5fa916ffe189fe0a44ba20`

## Objective

Resolve the M7/M8 `AuthDeferred` and `VerifyDeferred` boundaries without redefining M8 base semantics: implement MembershipAuth, CapabilityAuth, one non-transparent `ContractUpdate`, attach/remove/revocation, one finite refinement/model/Lean obligation, evidence provenance, dependent-artifact invalidation, and fail-closed diagnostics/traces.

## Scope and assumptions

M8 is fixed input. Authentication evidence identifies a principal but is not a grant. Runtime layers transform typed Contracts; verification modules separately map Judgment/ResidualObligation to Evidence, Diagnostic, or ResidualObligation. M9 may reject more, strengthen contracts, discharge a bounded obligation, and add evidence. It may not manufacture authority, permit undeclared effects, erase failures, reinterpret Core operations, change relation projection, or claim M10 conformance/public ABI/wire/production behavior.

## Start state / dirty state

M9 started clean with `HEAD == origin/main == 7f23a72ef1a20be27c5fa916ffe189fe0a44ba20`. Plan 247 names M9 as the sole active semantic frontier and M10 as next. The root filesystem had about 4.7 GiB available and memory availability was about 10 GiB; no heavy build or cleanup was started.

## Documents consulted

Canon-first reading includes the root README/MAP, Design Constitution C9/C11, ADR-0015, theories 02/05/07/11/17, specs 02/03/08/09, Plan 247, and M8 Report 2589. Current LAB snapshots are read only for the M9 frontier; older LAB auth/layer/verifier code is implementation evidence, not normative meaning.

## Actions taken

1. Verified M8 push parity and clean M9 start state.
2. Opened one M9 report and requested independent planner, implementation-seam, test-seam, and formal-seam audits.
3. Selected one source-bound outer M9 admission over an unchanged M8 deferral boundary.  The original checked identity, Core, residual rows, and source map remain unchanged; exact M9 resolution rows are a typed sidecar, and direct M8 admission remains `DeferredToM9`.
4. Selected one versioned extension frame with non-coercible runtime-policy and verifier lanes.  Runtime auth layers use explicit non-transparent `ContractUpdate`/admission/activation cut; verifier modules return only Evidence, Diagnostic, or ResidualObligation.  The per-request auth sidecar alternative was rejected because it cannot make removal, revocation, persistence, or dependent-evidence invalidation explicit.
5. Completed the M9 RED matrix and corresponding Canon/Lean finite tranche in parallel.  The four focused Rust suites first failed at the expected unresolved M9 module imports.
6. Implemented the fixed RED contracts through a production single writer without widening M8 direct admission or rewriting M7 identity/Core/residual/source-map data.  The resulting source-bound admission validates current membership/capability/witness lineage and finite-verifier evidence before exposing the M9-admitted runtime seam.
7. Implemented explicit non-transparent ContractUpdate attach/remove/revocation with prevalidated failure atomicity, activation-cut provenance, dependent-artifact invalidation, and typed observer-safe projections.  Provider, transport, session, locus, and principal strings alone never satisfy authority.
8. Removed shortcut constructors, cloneable authority state, duplicate-ref overwrite paths, and raw ContractRuntime evidence/snapshot projections discovered by the negative matrix.  Contract/runtime Debug output contains structural summaries rather than raw authority references.
9. Independent review found forgeable public provider proof metadata, a metadata-only finite verifier, missing epoch-currentness invalidation, unrelated layer-removal revocation, a raw authority-history projection, an overstated Lean `cost_bound` claim, and an overstated action-sequence model-check claim.  The correction sealed provider-issued attestation, added current-lineage tombstoning, bound removal to attachment lineage, made direct revocation observable through redacted structural rows, closed raw history, narrowed Canon to the actual Lean theorem, and renamed/validated the bounded reachable-state graph.
10. A narrow re-review kept the finite verifier finding open because a named residual still discharged itself.  The final correction replaced that route with an actual finite before/after Contract inclusion check over source-derived failures, capabilities, effects, and observations.  Its source-bound discharge records a normalized candidate and expected delta; the non-transparent ContractUpdate must present that exact delta before any activation cut or M8 mutation.  The final finite-only review closed the remaining P0.

## Files changed

- `docs/reports/2590-mir-theory-v0-i1plus-milestone-9-auth-verification.md`
- `mirrorea_canon/meta/proposals/PROPOSAL-027-m9-auth-verification.md`
- `mirrorea_canon/adr/ADR-0024.md`
- `mirrorea_canon/spec/10-m9-auth-verification.md`
- `mirrorea_canon/theory/18-m9-auth-verification.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- Canon navigation, changelog, and generated index files
- `samples/lean/foundations/MirTheoryV0M9AuthVerification.lean`
- `samples/lean/foundations/MirTheoryV0M9AuthVerification.md`
- `crates/mir-runtime/tests/m9_external_boundary.rs`
- `crates/mir-runtime/src/m9_auth_verification_unit_tests.rs`
- `crates/mir-runtime/src/m9_auth_verification.rs`
- `crates/mir-runtime/src/m8_runtime_admission.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-semantics/tests/m9_finite_refinement.rs`
- `crates/mir-semantics/tests/m9_model_check_auth.rs`
- `crates/mir-semantics/src/m9_finite_refinement.rs`
- `crates/mir-semantics/src/m9_model_check_auth.rs`
- `crates/mir-semantics/src/lib.rs`
- `plan/247-mir-theory-v0-i1plus-current-roadmap.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Commands run

- M9 start-state, disk, memory, Canon-first, roadmap, and implementation-seam inspection.
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M9AuthVerification.lean`
- Lean boundary `#check` and `#print axioms` checks for five selected M9 theorems.
- Canon index validation and scoped Canon whitespace checks.
- Focused Rust formatting and `cargo fmt --check`.
- Four focused Cargo RED invocations for the new runtime and semantics suites.
- `python3 scripts/check_source_hierarchy.py` and `python3 scripts/validate_docs.py` after correcting the report heading required by the validator.  One initial Canon-index invocation from the repository root failed with `canon root not found`; rerunning it from `mirrorea_canon/` passed.
- `cargo test -p mir-runtime --lib` and `cargo test -p mir-runtime --test m9_external_boundary`.
- `cargo test -p mir-semantics --test m9_finite_refinement --test m9_model_check_auth`.
- `cargo test -p mir-runtime`, `cargo test -p mir-semantics`, and `cargo test --workspace`.
- `cargo clippy -p mir-runtime --all-targets -- -D warnings` and `cargo clippy -p mir-semantics --all-targets -- -D warnings`.
- `cargo fmt --check`, `git diff --check`, agent-config validation, the corrected Lean forbidden-stub scan, and full documentation validation.  The first post-sync documentation run correctly rejected the report because its `Files changed` list omitted the six synchronized planning/status files; adding those exact paths resolved the deterministic schema failure.

## Evidence / outputs / test results

The finite M9 Lean file compiles with trusted settings; five selected theorems print no axioms, and the `sorry` / `admit` / user-axiom / stub scan has no hit.  OBL-026 is now `lean-proved` only for the exact finite source-resolution/no-M8-success/no-authority/two-transparent-overlay profile.  It does not cover non-transparent MembershipAuth/CapabilityAuth composition generally.  Canon indexing passes at 167 indexed files, source hierarchy validates 798/798 required paths, and the repository documentation validator accepts the current 1,744-report scaffold.

The M9 Rust suites began as deliberate RED evidence at unresolved M9 module imports and later reproduced each independent-review falsifier.  The finite verifier suite is green at 8/8: its positive candidate preserves all source-derived failures and effects while adding the exact MembershipAuth precondition, capability requirement, `AuthRejected` failure, and authority-private observation; negatives remove a baseline failure, add an undeclared external effect, weaken observation, omit required rows, replay public metadata, or mismatch identity/source/schema/contract.  The bounded auth-model suite is green at 9/9 over an input-sensitive finite reachable-state graph through depth four.  It covers monotone revocation, rejected-use no mutation, concrete replay/hidden-mutation counterexamples, insufficient-bound non-coverage, fresh-evidence re-acquire, and unrelated/malformed selector diagnostics.  This is bounded state-graph evidence, not action-sequence enumeration or a Lean/general proof.

The crate-private runtime suite has 11 M9 tests and the external public-boundary suite has one, for 29 focused M9 tests with the two semantics suites.  They cover exact source/residual identity, sealed provider attestation versus public forged claims, transport/provider non-authority, capability policy and duplicate-ref rejection, current epoch/incarnation tombstoning, exact finite-discharge/delta binding, final source-bound admission, duplicate/missing/atomic removal lifecycle, observer-safe attach/remove/direct-revocation evidence, observation weakening, concrete bounded counterexamples, and fresh-evidence re-acquire.  The original external-positive tests were migrated rather than disabled; no zero-test placeholder remains.

Both changed crates pass full crate tests and combined all-target `-D warnings` Clippy.  The full workspace test suite passes.  Formatting and diff checks pass.  Agent configuration, Canon index (167 files), source hierarchy (798/798), trusted Lean compilation, axiom output, and forbidden-stub scans pass.  Documentation validation, its unit suite, and `make docs` pass after close-document synchronization.  Independent review has closed every P0/P1/P2 finding.  The ledger now records OBL-028 as `model-checked-bounded` only for the exact graph described above, never as a proof claim.

## What changed in understanding

M9 must consume the exact deferred source-bound rows and canonical M8 plan derivation rather than attach a legacy untyped middleware stack. The auth and verification lanes share provenance/version/invalidation infrastructure but do not share semantic transformation meaning.  The ordinary `with auth MembershipAuth` and `verify finite_refinement` markers are module-contract selectors in this finite M9 profile; their external typed bindings must match program identity, kind, name, and SourceRef exactly.  M9 does not strip residuals or rewrite the M7 artifact.

## Open questions

No owner decision is required.  Reopen M8 only if a lossless crate-private plan derivation cannot preserve the checked identity/source map/residual descriptors or would change direct M8 `DeferredToM9` behavior.

## Suggested next prompt

No prompt is required. Continue autonomously into M10 release conformance after M9 push parity.

## Plan update status

更新済み: Plan 247 marks M9 complete and M10 as the sole active frontier.  It records the 29 focused tests, exact OBL-026/028 classifications, and the fresh M10 release-profile blocker without claiming a pass.

## Documentation.md update status

更新済み: the reader entry states M0--M9 closed, M10 active, and summarizes source-bound resolution, typed ContractUpdate/revocation/invalidation, the three assurance lines, and bounded-only nonclaims.

## docs/project-status.md update status

更新済み: the current project status names M10 as active and gives the exact OBL-026/028 evidence classes plus the M9 non-effect/M10 stop line.

## progress.md update status

更新済み: a command-derived timestamp, M9 close evidence, M10 active frontier, exact logical/user/implementation axes, bounded proof/model distinctions, and a recent-log row are recorded.

## tasks.md update status

更新済み: the current snapshot names M10 as the sole autonomous package; its direct blocker is fresh reproducible same-source conformance, SCN C-static/C-runtime evidence, fresh checkout reproduction, and independent review.  No post-M10 owner decision is inferred.

## samples_progress.md update status

更新済み: the M9 Lean/runtime/refinement/model commands are evidence-only rows.  The dashboard explicitly does not classify them as a public workflow or official M10 conformance.

## Reviewer findings and follow-up

The independent planner selected the versioned typed Contract-transformer over one minimal per-request sidecar alternative.  Implementation and formal audits selected an external source-bound M9 resolution judgment with exact residual keys, a lossless crate-private M8 plan derivation, append-only provenance/invalidation, OBL-026 as the exact finite Lean slice, and OBL-028 as a bounded revocation/no-mutation target.

Independent final review reported two P0s: publicly forgeable provider-proof strings and metadata-only finite-refinement discharge.  It also reported epoch drift without current-lineage invalidation, layer removal not tied to attachment lineage, missing direct-revocation audit, raw history/Debug exposure, a nonexistent Lean `cost_bound` claim, and an action-sequence overclaim.  One correction sealed provider attestation, enforced current lineage and exact removal provenance, added observer-safe revocation rows, closed raw history, narrowed Canon, and made model inputs control the bounded state graph.  Narrow re-review closed every item except finite refinement.  The second bounded correction added the actual source-derived Contract inclusion predicate and exact runtime delta binding.  Final finite-only re-review marked that last P0 CLOSED at `m9_finite_refinement.rs` `discharge_candidate` and the pre-cut runtime check.

## Skipped validations and reasons

Commit/push and remote parity remain pending.  Workspace-wide Clippy is not claimed because an unrelated pre-existing CLI boolean-assert lint remains outside the two changed crates; both changed crates pass combined all-target `-D warnings`, and the full workspace test suite passes.  M10 release conformance, fresh reproduction, and public CLI workflow checks are later work rather than M9 claims.

## Commit / push status

No M9 commit or push exists at report start. M8 closeout `7f23a72ef1a20be27c5fa916ffe189fe0a44ba20` is pushed with remote parity verified.

## Sub-agent session close status

- M9 pre-edit planner: complete, read-only.
- M9 implementation-seam audit: complete, read-only.
- M9 test-seam audit and RED authoring: complete; test-only files changed.
- M9 formalization-seam audit and bounded Canon/Lean tranche: complete; owned Canon/Lean files changed and scoped checks passed.
- M9 production implementation: complete; production writer returned ownership to the parent without committing.
- M9 independent final reviewer: complete, read-only; all findings closed after bounded corrections and narrow re-review.
