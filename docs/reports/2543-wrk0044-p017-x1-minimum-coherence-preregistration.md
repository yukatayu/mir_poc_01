# Report 2543: WRK-0044 P017 X1 Minimum Coherence Preregistration

- Date: 2026-07-30 00:56 JST
- Author: Codex
- Scope: ADR-0014 L3 preregistration only; no outcome source, implementation,
  or Canon semantic amendment.
- Decision level: L3-open reversible research boundary; no promotion.

## Objective

Register the one selected P017 X1 minimum relation-envelope coherence experiment
before any Lean source is materialized, and make its hypotheses, adverse cases,
and stop rule reproducible.

## Scope and assumptions

P017 X1 is the owner-recorded V1/R1 cross-locus-read direction. The record is
limited to one existing-LAB-lane experiment at the pinned Git/document
authority-and-evidence cut. External rejection and no observation are
candidate-local scopes, not adopted Canon semantics. A passing later result may
say only conditionally compatible at the pinned cut.

## Start state / dirty state

The selection package was committed and pushed at
`b15eb514c1f2c9223c35336e3f398d94ff06bd1b` with a clean worktree.
Plan 228 selected one candidate but no WRK, Lean source, helper, schema,
runtime change, or evidence artifact existed. The worktree became dirty only
with this registration package.

## Documents consulted

Read Canon root/Map, ADR-0014, working annex rules, P012, P013, P017,
theory/01--05 and theory/07, Core/runtime boundary, Plans 225--228,
current snapshots, historical WRK registration shape, and documentation
validator. A temporary GPT-5.6 Sol Pro Oracle review was attached to
ADR-0014, working rules, P017, and Plans 227--228; it was advisory and checked
against local sources.

## Actions taken

Created WRK-0044 with exact Canon/LAB pins, an eight-row P017 ledger, an
aggregate stop rule, `C + H_K + D_K` hypothesis discipline, anti-vacuity
control, restore non-identity stop, and a narrow external-rejection/no-
observation scope. Registered it in Canon Map and regenerated the Canon index.
Synchronized current LAB status snapshots without adding source evidence.

## Files changed

- `mirrorea_canon/working/WRK-0044-p017-x1-minimum-relation-envelope-coherence.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2543-wrk0044-p017-x1-minimum-coherence-preregistration.md`

## Commands run

Ran parent-cut digest checks; inspected historical registration deltas; ran the
temporary Oracle review; regenerated the Canon index; ran `make docs` and the
working-annex validation after committing; inspected staged diffs, ran
`git diff --check`, and scanned staged content for concrete Discord webhook
URLs. Commit/push and equality checks are recorded below.

## Evidence / outputs / test results

No Lean source was created or run. The registration pins all inputs to
`b15eb514c1f2c9223c35336e3f398d94ff06bd1b` and declares
`Evidence commits: none`. The post-commit documentation, source-hierarchy,
Canon index, and authoritative working-annex validators passed. The record's
only evidence is a pre-registered procedure; no semantic, proof, runtime, or
readiness result exists.

## What changed in understanding

The selected question remains eligible, but not as an informal “minimum model.”
The experiment must treat external rejection as `H_rejection-external`, use a
Git/document rather than semantic “cut,” forbid vacuity and silent helper
assumptions, preserve restore correspondence without occurrence equality, keep
observation as a negative scope, and defeat any positive aggregate result when
one required row stops or remains open.

## Open questions

Whether one disposable presentation can cover every ledger row without a
reserved surface is untested. Candidate-local relation definitions, if needed,
must not turn into a reusable schema or lifecycle. Canon branch/failure,
validation, causal, persistence, source, observation, runtime, and public
design decisions remain open.

## Suggested next prompt

After confirming this registration is pushed, materialize exactly one Markdown-
held Lean block in the declared `plan/` lane, run its registered audits,
and either retain only a bounded conditional result or freeze it at the first
falsifier.

## Plan update status

更新不要: Plan 228 already records the selection; this registration intentionally
adds no LAB source or new plan record.

## Documentation.md update status

更新不要: the strict registration delta excludes `Documentation.md`. Its Plan
228 entry remains an accurate reader link to the selected candidate; current
registration status is in the working Map and status snapshots.

## docs/project-status.md update status

更新済み: recorded WRK-0044 as registered but unexecuted and preserved all
non-claims.

## progress.md update status

更新済み: updated the logical-specification boundary, current research action,
timestamp, and recent log.

## tasks.md update status

更新済み: replaced the unregistered-candidate action with the pushed-registration
then one-experiment path.

## samples_progress.md update status

更新不要: no runnable sample, command, debug surface, or sample blocker changed.

## Reviewer findings and follow-up

The temporary Oracle review found ADR-0014 eligibility in substance but required
corrections before `Standing eligibility: pass`: classify external rejection as
a hypothesis; distinguish Git/document and semantic cuts; prohibit vacuity and
pre/post-load identity; retain observation and source boundaries separately;
expand all rows with per-row falsifiers and an aggregate stop; pin sources and
the exact expected source path; and cover the full reserved-boundary stop list.
WRK-0044 incorporates those corrections. The review remains advisory; P017 and
ADR-0014 are the authority.

## Skipped validations and reasons

Skipped Lean execution intentionally: ADR-0014 and the working record prohibit
outcome source before this record is committed and pushed. No callable
sub-agent tool exists in this environment. The Oracle review completed; no
external work remains active.

## Commit / push status

Pending at report creation. This registration package will be committed with
`--no-gpg-sign`, pushed, and checked so `HEAD == origin/main` before any
outcome source is created.

## Sub-agent session close status

No callable sub-agent session exists. The temporary Oracle session
`p017-x1-wrk0044-registrati-review` completed successfully and its raw output
remains outside the repository.
