# Report 2384 - WRK-0018 falsifier rollback

- Date: 2026-07-23 03:00 JST
- Author / agent: Codex
- Scope: registered-falsifier correction and source restoration before frozen manifestation
- Decision levels touched: none; no Canon or OBL result

## Objective

Correct the direct-outcome handling after independent review found that the
registered marked-tail compile failure had already triggered WRK-0018's freeze
condition, restore the exact pinned source inputs, and preserve only the
failure history for later append-only manifestation.

## Scope and assumptions

WRK-0018 says without exception that failure to compile the marked tail is an
expected falsifier and that any falsifier freezes reliance without repairing the
tail.  Report 2383 records an intentionally incomplete marked-tail compile
failure and subsequent proof-form changes.  Regardless of its testing intent,
the literal pre-registration therefore controls: the green tail cannot be
retained as positive evidence.  Commit history is not rewritten.

## Start state / dirty state

Started clean at pushed direct-evidence commit
`5b33d915bab2741e7b1fa72e627a7ed5f916da38`.  That commit added the marked
tail, explanation, and Report 2383.  The source input digests pinned by
WRK-0018 remain available from the registration base.

## Documents consulted

Read WRK-0018's question, expected falsifier, and rollback trigger; Report
2383; plan 177; the pinned foundation and companion explanation; the
working-record lifecycle rules; and the independent review of commit
`5b33d915`.

## Actions taken

1. Accepted the review finding that the first marked-tail compiler failure is
   a registered falsifier rather than an exempt red test.
2. Removed the later green tail and matching explanation rather than revising
   the registered question or preserving a repaired result.
3. Added this correction report.  The separate frozen manifest will append the
   evidence commit and explain that Report 2383 is failure history, not
   positive evidence.

## Files changed

- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean` (restored)
- `samples/lean/foundations/CurrentL2IfcSecretExamples.md` (restored)
- this report

## Commands run

- inspected the immutable direct-outcome diff and exact WRK-0018 falsifier text
- compared both original source inputs with the registration-base SHA-256
  digests
- post-review source-restoration compile and absence/digest checks (pending at
  report write)

## Evidence / outputs / test results

The review found one blocking process issue: the direct report explicitly
records that a `WRK0018` marked tail first failed to compile because its model
identifiers were missing, then was changed to compile.  WRK-0018 lists marked-
tail compilation failure as a falsifier and prohibits repairing the tail.
Therefore the green theorem source in `5b33d915` is excluded from retained
positive evidence.

The same review found no semantic-scope violation in the discarded toy tail:
it was concrete `Nat`/`Bool`, had the named positive/adverse theorems, and
introduced no THM/OBL/BND-008 claim.  That does not overcome the process
falsifier.  It also noted that post-edit documentation, hierarchy, and Canon
index checks passed at the discarded state, but Report 2383 described them
incompletely; those checks do not rehabilitate the route.

## What changed in understanding

An intentionally failing scaffold belongs outside a pre-registered marked-tail
experiment unless the record explicitly permits it.  For this record, a marked
tail must be first introduced only in its complete registered form.  The
failure route is frozen; a future question would need a distinct fresh
pre-registration and cannot repair WRK-0018 in place.

## Open questions

- Is a future direct dependency experiment decision-relevant enough to justify
  a fresh record with an explicit pre-source validation procedure?
- Would such a future experiment require a real selected provenance or
  low-equivalence relation, and therefore owner/canon action?

## Suggested next prompt

Manifest WRK-0018 as frozen at the marked-tail compile falsifier, then return
to candidate selection without reusing or repairing its discarded toy tail.

## Plan update status

`plan/` 更新不要: plan 177 remains the immutable selection/pre-registration
input.  The rollback freezes its route and does not prescribe a successor.

## Documentation.md update status

`Documentation.md` 更新不要: reader-facing status changes only in the frozen
working-record manifestation.

## docs/project-status.md update status

更新不要: this rollback is unmanifested direct evidence; the published snapshot
changes with the subsequent frozen record.

## progress.md update status

`progress.md` 更新不要: no current-state classification is changed until the
working record append-only records the falsifier.

## tasks.md update status

`tasks.md` 更新不要: package 48 remains pending frozen manifestation.

## samples_progress.md update status

`samples_progress.md` 更新不要: the active foundation and its command are
restored, and no workflow readiness or dashboard validation result changes.

## Reviewer findings and follow-up

The voluntary independent reviewer found the blocking literal-falsifier issue,
one report wording inconsistency about review necessity, and incomplete
post-edit validation accounting.  Its scope review also confirmed that the
discarded toy tail itself did not make a semantic promotion.  The first finding
is accepted and determines the freeze; the latter two are recorded here and in
the manifest.

## Skipped validations and reasons

The full current-L2 sync remains skipped because it builds into the repo-root
Cargo target with only 6.9 GiB free.  Broad runtime/distributed suites do not
exercise this restored helper-local source.  No new Lean tail is tried after
the falsifier, because that would be an impermissible repair.

## Commit / push status

Pending at report write.  The source restoration and this report will be
committed with `--no-gpg-sign` and pushed before WRK-0018 manifests the frozen
history.

## Sub-agent session close status

The independent reviewer is closed after returning its finding.  No subagent
modified files.
