# Report 2536 - WRK-0043 M1 adverse/mutation countermodel execution

## Title and identifier

2536-wrk0043-m1-adverse-mutation-countermodel-execution: execute only the
registered finite source for WRK-0043 and retain or freeze the exact evidence
without selecting M1 validation or mutation semantics.

## Objective

Test whether the declared neutral, adverse-only, mutation-only, and seeded
adverse/mutation-overlap fixtures are distinguished by one predicate-only Lean
detector under the registered no-axiom and reserved-boundary conditions.

## Scope and assumptions

WRK-0043 was registered and pushed at
8ff73b23ab8d45c503852341b0f036b212082fd5. The authority/input cut remains
b07ea81d8d1a2117e1e5c861d99f51508764ecf7. The evidence lane is exactly one
Markdown-held Lean block under plan/ and this direct report; temporary
extraction and axiom harnesses are not retained sources.

## Start state / dirty state

HEAD and fetched origin/main were equal at
66cac4c28bbc030b81d5b8e04bb1ceb1b41b3fa2; the worktree was clean. WRK-0043
was a valid registered, unexecuted, non-promoted L3 record. The root filesystem
had 29G available (85% used), the repository was 6.0G, target was 5.9G, and
Lean was version 4.29.1.

## Documents consulted

ADR-0014, working/README.md, P013, P017, WRK-0040--0043, Plans 220, 221, 223,
and 224, the current LAB snapshots, Reports 2534 and 2535, the retained prior
Lean sources, and the test-driven-development and debugging operating guidance.

## Actions taken

Ran a RED harness that intentionally defined the overlap detector as False; Lean
rejected its claimed seeded-overlap theorem. Built a GREEN fixture model with one
Anchor, a finite AdverseTag family, four Fixture forms, inductive adverse and
mutation marks, and one existential overlap detector. Materialized only the
registered Markdown source, then compiled, inspected theorem axioms, checked the
matrix/theorem inventory, scanned forbidden vocabulary, enforced the evidence
allowlist, and ran diff checks.

## Files changed

- `plan/wrk-0043-p017-x1-m1-adverse-mutation-countermodel.md`
- `docs/reports/2536-wrk0043-m1-adverse-mutation-countermodel-execution.md`

## Commands run

- lean --version
- lean --trust=0 on the disposable RED harness
- lean --trust=0 on the disposable GREEN harness
- extracted the sole fenced Lean block and compiled it with lean --trust=0
- appended four disposable print-axioms commands and compiled that harness
- scanned the extracted source for placeholders, unsafe/classical/quotient/native
  facilities, selected-boundary vocabulary, and the four registered theorems
- checked the one-block source shape, theorem count, fixture matrix evidence,
  artifact SHA-256 values, evidence allowlist, and git diff --check

## Evidence / outputs / test results

RED failed as intended: True.intro could not prove the deliberately False
seeded-overlap detector. The retained source passed lean --trust=0.

The final Markdown SHA-256 is
0cc958ee31eb7d4ed07dda77372f4c8a4b118b88ba894b0f5d6520cdcfe53cd3.
The extracted Lean SHA-256 is
fbc1a5650076de59df44f5311be69a6d255cc70dd7f565072ebff94931c631e1.

neutral_is_clear, adverse_only_is_clear, mutation_only_is_clear, and
overlap_is_detected each reported no axioms. The source has one Lean block and
four theorem declarations. Placeholder, unsafe, partial, implemented_by,
Classical, Choice, Quotient, Quot.sound, native_decide, and axiom scans were
clean. The reserved-vocabulary scan over the Lean block was clean.

The first GREEN draft used a proposition-valued pattern match with tag equality.
The axiom harness exposed propext in every theorem. The uncommitted draft was
discarded; the retained source instead uses inductive marks that carry the
supplied tag as evidence. Its final axiom reports are clean.

## What changed in understanding

The no-axiom condition is material for this experiment: an apparently simple
proposition-valued fixture classifier can introduce propext through its generated
equations. The same finite control matrix can be expressed constructively with
inductive fixture evidence, without adding any Mir semantic notion.

## Open questions

The result does not determine whether an actual request has an adverse tag,
whether validation accepts or rejects it, whether it fails closed, whether a
mutation occurs or belongs to that request, or any branch, carrier, transition,
authority, persistence, runtime, or public behavior. Those remain unresolved.

## Suggested next prompt

Link this exact evidence into WRK-0043 Results and the Canon MAP, then
synchronize the reader/status views before any fresh candidate screen.

## Plan update status

plan/ updated: the declared WRK-0043 source now holds the sole retained
predicate-only Lean block. Plan 224 remains the candidate-selection and stop
record; no positive semantic plan was added.

## Documentation.md update status

Documentation.md unchanged: execution evidence is linked through the following
metadata package before reader snapshots are changed.

## docs/project-status.md update status

更新不要: the compact control view remains at registered/unexecuted until the
Results/MAP metadata link is committed.

## progress.md update status

progress.md unchanged: the next metadata and reader packages will record the
passed finite result and its non-effects.

## tasks.md update status

tasks.md unchanged: the next metadata and reader packages will move package 5
from registered execution to a fresh non-mechanical screen.

## samples_progress.md update status

samples_progress.md unchanged: no runnable sample, command, debug surface, or
sample-dashboard row changed.

## Reviewer findings and follow-up

The prior temporary Oracle review was advisory candidate selection only. No new
review was needed for the pre-registered execution. No callable sub-agent
execution interface is available.

## Skipped validations and reasons

No runtime or sample command applies: this is a finite Lean LAB artifact, not a
runtime, parser, adapter, or sample change. No positive Mir model can be run
because the registered source intentionally supplies only opaque fixture labels.

## Commit / push status

Evidence committed as `22d0f95c25500a1018f301ed9ebcc6f3b6d91354`
(`test: execute P017 X1 M1 countermodel`), pushed to `origin/main`, and verified
equal to fetched `origin/main`. The evidence is linked by the following
metadata package before the successor screen.

## Sub-agent session close status

No callable sub-agent session was opened or remains to close.
