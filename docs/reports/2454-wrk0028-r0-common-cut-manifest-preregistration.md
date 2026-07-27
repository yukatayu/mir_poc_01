# Report 2454 — WRK-0028 R0 common-cut manifest preregistration

- Date: 2026-07-28 08:24 JST
- Author / agent: Codex
- Scope: Pre-register a reversible L3 literal-transcription audit for
  pre-enumerated C0/C2 source spans. No semantic candidate is selected.
- Decision levels touched: Canon `working/` L3 boundary and operational
  metadata only, under ADR-0014.

## Objective

Create a bounded, reproducible current-cut provenance record before relying on
any consolidated C0/C2 source inventory.

## Scope and assumptions

`mirrorea_canon/` remains normative. Plan 199/200 are LAB navigation, not
authority for a Canon reading. R0 may transcribe source-local statements and
their explicit authority limits only; it may not reconcile or compose them.

## Start state / dirty state

Started clean at `4ee275507000b905e46c6b5389865f7c0985ab79`, equal to
`origin/main`, after Plan 200 and the report-lane validator repair. Root disk
had 61 GiB free; approximately 5.2 GiB memory was available.

## Documents consulted

- Canon README/MAP, ADR-0014, working annex, spec/01--03, theory/01/03/05/10,
  P004/P008/P012/P013/P015, and current WRK metadata.
- LAB Plans 199/200, current snapshots, and Oracle operating notes.
- Temporary Oracle review `r0-standing-eligibilit-review-20260728`.

## Actions taken

1. Re-read the standing-eligibility and reserved-boundary rules.
2. Pinned the current authority/input cut and recorded SHA-256 values for all
   intended Canon anchors and LAB inputs.
3. Asked Oracle to challenge the proposed R0 scope, then retained only its
   source-compatible governance controls.
4. Narrowed R0 from C0--C7 to pre-enumerated C0/C2 source spans and excluded
   historical WRK results from its manifest.
5. Created WRK-0028, added its MAP row, and regenerated Canon index metadata.

## Files changed

- `mirrorea_canon/working/WRK-0028-r0-common-cut-fact-manifest.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2454-wrk0028-r0-common-cut-manifest-preregistration.md`

## Commands run

- Ordered Canon/LAB source reads, source searches, and current-cut SHA-256
  collection.
- `df -h .`, `free -h`, `git status --short`, and Git parity checks.
- One temporary GPT-5.6 Sol Pro Oracle consultation.
- Canon index regeneration/check and source-hierarchy validation.

## Evidence / outputs / test results

Oracle conditionally approved an L3 literal-transcription record only when it
uses source-local rows, preserves proposal qualifiers/non-effects, separates
registration from result evidence, and freezes on any need for interpretation
or a new artifact contract. Local ADR-0014 and working-annex text support
those controls. No outcome command or manifest result has run yet.

## What changed in understanding

The safe first unit is not a C0/C2 compatibility conclusion. It is a
provenance-only record whose rows must be individually classifiable without
semantic interpretation. Historical WRK evidence remains outside R0 and must
be pinned by later records if needed.

## Open questions

- Whether every pre-enumerated C0/C2 span can retain its qualifier under the
  R0 literal-transcription boundary.
- Whether the actual manifest exposes an authority-class ambiguity and must
  freeze rather than synthesize a classification rule.

## Suggested next prompt

After this registration is pushed, run only its registered commands and retain
the source-local manifest or freeze WRK-0028 on the first registered falsifier.

## Plan update status

`plan/` 更新不要: Plan 200 already defines R0 and no result exists yet.

## Documentation.md update status

`Documentation.md` 更新不要: reader navigation does not change.

## docs/project-status.md update status

更新不要: a preregistered L3 audit does not change project maturity or a gate.

## progress.md update status

`progress.md` 更新不要: no runtime, theory, or workflow milestone changed.

## tasks.md update status

`tasks.md` 更新不要: its current autonomous C0/C2 package already names R0
as the next boundary.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample or runnable workflow changed.

## Reviewer findings and follow-up

Oracle conditionally approved R0 and required source-local classification,
qualifier retention, separate evidence commits, and immediate freeze on any
authority-normalization pressure. The external answer is advisory; the WRK
uses only controls confirmed against local Canon text.

## Skipped validations and reasons

No semantic, Lean, runtime, parser, or sample execution is appropriate before
the preregistration is committed and pushed. Registration validation remains
pending at report write.

## Commit / push status

Pending at report write. The preregistration will be committed with
`--no-gpg-sign`, pushed, and checked against `origin/main` before any outcome
evidence is created.

## Sub-agent session close status

No callable sub-agent session was available. The temporary Oracle session
completed; its raw transcript remains outside the repository and its advisory
controls are distilled above.
