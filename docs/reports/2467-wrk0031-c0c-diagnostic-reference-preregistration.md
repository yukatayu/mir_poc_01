# Report 2467 — WRK-0031 C0-C Diagnostic-reference preregistration

- Date: 2026-07-28 09:47 JST
- Author / agent: Codex
- Scope: Pre-register the selected C0-C source-local Diagnostic-reference
  audit as a reversible literal-transcription L3 record.
- Decision levels touched: Canon `working/` L3 boundary and required metadata
  only, under ADR-0014.

## Objective

Create a narrow, reproducible record before relying on any claimed relation
between front-end source spans and Diagnostic carriers or families.

## Scope and assumptions

The audit records only literal terminal/reject/`Diagnostic` wording and explicit
source cross-references. A source-local reference or nonreference is not a
stage, coverage, rejection, Diagnostic-assignment, or totality result.

## Start state / dirty state

Started clean at `f2da0b1ede4f437ba022865809411a02aa4a0bf0`, equal to
`origin/main`, after the common-cut candidate screen selected C0-C only.

## Documents consulted

- Canon README/MAP, ADR-0014, working annex, specs 01/02/03/07, theory/03,
  theory/10, P008, and current index metadata.
- WRK-0028, Plans 199/200, and Report 2466.

## Actions taken

1. Re-checked ADR-0014 standing eligibility and reserved-boundary exclusions.
2. Pinned current source and LAB-input SHA-256 values.
3. Registered a source-local query rather than Plan 200's broader “stage”
   wording, with explicit duplicate and semantic-selection falsifiers.
4. Added WRK-0031, its MAP row, and regenerated Canon index metadata.

## Files changed

- `mirrorea_canon/working/WRK-0031-c0c-source-local-diagnostic-reference-audit.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2467-wrk0031-c0c-diagnostic-reference-preregistration.md`

## Commands run

- Ordered Canon/LAB reads, clean-state/parity checks, pre-registration digest
  collection, Canon index regeneration/check, and source-hierarchy validation.

## Evidence / outputs / test results

The record uses an existing LAB lane, literal-transcription result class,
alternative, falsifier, non-effects, and forward-only rollback trigger. No
registered outcome command or LAB result has run yet.

## What changed in understanding

Diagnostic-reference presence can be audited without treating a source span as
a semantic “stage” or a named error as evidence of complete rejection coverage.

## Open questions

- Does removing WRK-0028's repeated facts leave an independent source-local
  observation?
- Can every retained row avoid Diagnostic assignment, coverage, and totality?

## Suggested next prompt

After registration is pushed, run only the registered commands and retain the
source-tagged query result or freeze WRK-0031 at the first falsifier.

## Plan update status

更新不要: Plan 199/200 already record the C0-C selection; this task only
creates its required Canon L3 pre-registration.

## Documentation.md update status

更新不要: reader navigation does not change.

## docs/project-status.md update status

更新不要: pre-registration adds no maturity or gate result.

## progress.md update status

更新不要: no semantic, runtime, or workflow milestone exists yet.

## tasks.md update status

更新不要: it already names C0-C pre-registration as the current package.

## samples_progress.md update status

更新不要: no sample or runnable workflow changed.

## Reviewer findings and follow-up

The completed temporary Oracle review required the narrower source-reference
form and prohibited combining C0-C with C0-D. Local reading of R0, theory/10,
and spec/07 supports the explicit duplicate and coverage stop conditions. The
external answer is advisory, not repository evidence.

## Skipped validations and reasons

No Lean, parser, runtime, or sample execution is appropriate before this
pre-registration is committed and pushed. Registered source checks and full
documentation validation run only after that boundary is durable.

## Commit / push status

Pending at report write. The registration will be committed with
`--no-gpg-sign`, pushed, and checked against `origin/main` before outcome
evidence is created.

## Sub-agent session close status

No callable sub-agent session is available. The temporary Oracle review
completed; its locally checked scope controls are distilled above.
