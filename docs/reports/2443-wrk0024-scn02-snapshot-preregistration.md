# Report 2443 — WRK-0024 SCN-02 snapshot countermodel preregistration

- Date: 2026-07-28 05:35 JST
- Author / agent: Codex
- Scope: ADR-0014 L3 pre-registration only. No outcome evidence is executed or
  relied on in this package.
- Decision levels touched: L3 working-annex registration and its required
  operational metadata only.

## Objective

Pre-register a narrow countermodel for whether owner-serial mutation alone
excludes stale read/write behavior in SCN-02's cross-locus dependent assignment.

## Scope and assumptions

The record examines only a finite model satisfying selected *displayed*
premises. It must not turn that model into a Canon execution or choose a
snapshot, evaluation locus, pending carrier, request identity, Core primitive,
or SCN result.

## Start state / dirty state

Started clean at `fcf5ea613c2153667e1c4a887589fb939692c7a5`, which had just
recorded the owner directions and Plan 199. This package creates no scratch
source; the registered post-push marker check will establish its prior absence.

## Documents consulted

- ADR-0014 and `mirrorea_canon/working/README.md`.
- `theory/01-mircore-v0.md`, `theory/03-elaboration.md`,
  `spec/05-runtime-semantics.md`, SCN-02, and PROPOSAL-012.
- LAB Plan 187, Plan 192, Plan 193, and Plan 199.

## Actions taken

1. Registered WRK-0024 with exact Canon/LAB anchors, alternative, expected
   falsifiers, rollback trigger, permitted locations, commands, and non-claims.
2. Registered it in the Canon MAP and regenerated the Canon index.
3. Deliberately did not create or execute the registered scratch countermodel;
   ADR-0014 requires a committed pre-registration first.

## Files changed

- `mirrorea_canon/working/WRK-0024-scn02-read-write-snapshot-ambiguity.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2443-wrk0024-scn02-snapshot-preregistration.md`

## Commands run

- `git status --short`, `git rev-parse HEAD`, and `sha256sum` for the anchored
  source cut.
- Read-only source audits with `sed`, `rg`, and `find`.
- Canon index generation/check and document/source-hierarchy validation will
  run before this registration commit. Registered Lean commands are deferred.

## Evidence / outputs / test results

No outcome evidence exists yet. The only evidence in this package is the
pre-registration text and its pinned source digests. The registered marker
check and Lean countermodel run must occur only after this package is committed
and pushed.

## What changed in understanding

The selected V1/R1 direction permits a bounded investigation of the *absence*
of a snapshot/fusion rule without selecting the missing carrier. This is the
smallest C1 slice: it tests whether serializing writes alone entails an atomic
read-dependent update. It does not answer how Mir should prevent the trace.

## Open questions

- Whether the exact eventual rule is owner-side evaluation, an explicit
  snapshot relation, another restricted V1/R1 presentation, or a diagnostic
  rejection remains reserved.
- C0, C2, C3--C7 remain separate Plan 199 work.

## Suggested next prompt

Execute the registered WRK-0024 countermodel once the registration is pushed,
then retain only the literal result and escalate any required semantic choice.

## Plan update status

更新不要: Plan 199 already defines C1; pre-registration adds no result or
workstream change.

## Documentation.md update status

更新不要: reader-facing project orientation is unchanged by a pre-registration.

## docs/project-status.md update status

更新不要: no research result or project-status claim has changed.

## progress.md update status

更新不要: no outcome, readiness, or blocker classification has changed.

## tasks.md update status

更新不要: Plan 199 C1 was already the current autonomous package.

## samples_progress.md update status

更新不要: no runnable sample or workflow changed.

## Reviewer findings and follow-up

The prior temporary Oracle review identifies the SCN-02 read/snapshot boundary
as composition-critical. This pre-registration limits the next evidence to a
finite countermodel of the displayed minimal premises; it does not treat the
advisory review as a semantic authority.

## Skipped validations and reasons

The registered `lean --trust=0` and marker-check commands are intentionally
skipped until the pre-registration is committed and pushed. Runtime/sample
commands are out of scope because no executable repository artifact changes.

## Commit / push status

Pending at report write. This registration will be committed with
`--no-gpg-sign` and pushed before the registered countermodel is created.

## Sub-agent session close status

No callable sub-agent session was available. The existing Oracle review is
advisory and its relevant concern is captured as a bounded falsifier here.
