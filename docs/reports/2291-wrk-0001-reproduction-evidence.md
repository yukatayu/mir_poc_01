# Report 2291 - WRK-0001 finite-index reproduction evidence

- Date: 2026-07-21 17:17 JST
- Author / agent: Codex
- Scope: Run the committed WRK-0001 commands and retain the narrow result in an existing LAB lane before manifesting it in the WRK record.
- Decision levels touched: L3 evidence only. No L0/L1, theory/11, contract, SCN, Gate, Phase, proof, implementation, or public-state movement.

## Objective

Reproduce the committed finite-index Lean fragment and record only the positive and rejecting evidence declared by WRK-0001.

## Scope and assumptions

The commands run only after `3e263f72` committed and pushed the pre-registration. The existing fragment is helper-local. It is not treated as a final calculus, source-language design, runtime, authority model, or proof discharge.

## Start state / dirty state

Started from clean, pushed `main` at `3e263f72731a32e2ca0ed549a873da5bb33d92ad`, containing the committed WRK-0001 record and no unmanifested result evidence.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014, theory/02, `working/README.md`, and WRK-0001.
- `plan/156`, `plan/158`, `plan/159`, `plan/00-index.md`, `tasks.md`, `progress.md`, and `samples/lean/README.md` as LAB evidence and operating guidance.
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean` and `samples/lean/manifest.json`.

## Actions taken

- Ran the exact positive Lean compile and source-level negative audit committed in WRK-0001.
- Preserved the observation in `plan/wrk-0001-finite-index-reproduction.md`, an existing permitted LAB root.
- Kept the WRK record itself unchanged so that the evidence commit is distinguishable from the later append-only manifest commit.

## Files changed

- `plan/wrk-0001-finite-index-reproduction.md`
- `plan/00-index.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2291-wrk-0001-reproduction-evidence.md`

## Commands run

- `lean --version`
- `lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- The exact Python source audit pre-registered by WRK-0001.
- `python3 scripts/validate_docs.py`
- `make docs`

## Evidence / outputs / test results

- Lean 4.29.1 reported release commit `f72c35b3f637c8c6571d353742168ab66cc22c00`.
- The direct Lean file check passed with exit status 0 and no stdout/stderr.
- The source audit passed with exit status 0: all four named lemmas were present and no `sorry`, `admit`, `axiom`, `unsafe`, `partial`, or `implemented_by` token occurred.
- Documentation validation and `make docs` passed after the registration commit. The current evidence document is intentionally not yet listed in `Evidence commits:`; that manifest action occurs in a later commit.

## What changed in understanding

The standing L3 route can retain a real, narrow mechanization result without requiring either a new helper surface or a semantic decision. The result is useful only as a reproducibility and scope-control signal until a separate reviewed promotion path exists.

## Open questions

- The completed browser-backed Oracle candidate-selection advisory recommends a distinct, future OBL-021 single-projection countermodel. It is not a dependency of this result and cannot amend the committed pre-registration.
- Whether this finite fragment generalizes to any final Mir index discipline remains intentionally unanswered.
- The evidence must still be append-only manifested in WRK-0001 and validated from a clean detached worktree.

## Suggested next prompt

Manifest this exact evidence commit in WRK-0001, validate it authoritatively in a clean detached worktree, then complete a cross-cut checkpoint review without promoting L3 evidence.

## Plan update status

更新済み: the direct reproduction evidence and index reference are retained in the pre-registered permitted `plan/` lane.

## Documentation.md update status

`Documentation.md` 更新不要: the reader's top-level route and current boundaries did not change.

## docs/project-status.md update status

更新済み: the human view now distinguishes completed reproduction from the separate WRK manifest action.

## progress.md update status

更新済み: the recent log records the narrow passed commands and their non-claims.

## tasks.md update status

更新済み: evidence capture is closed; manifest and checkpoint validation are now current.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample path, runner, command catalog, or readiness classification changed.

## Reviewer findings and follow-up

The completed Oracle advisory ranked a future OBL-021 single-projection adequacy countermodel above this reproduction as a next candidate. This commit does not adopt it: its conclusion is deferred to the pilot checkpoint, and the advisory cannot rewrite the pre-registration or upgrade this result.

## Skipped validations and reasons

The authoritative working-annex validation is deferred until the next commit manifests this evidence. Running it now would correctly see evidence that is not yet listed in the WRK record.

## Commit / push status

Pending focused diff review, commit, and push of the retained evidence commit.

## Sub-agent session close status

No new sub-agent result was used. The Oracle consultation completed and will be handled as non-normative advisory input at the checkpoint.
