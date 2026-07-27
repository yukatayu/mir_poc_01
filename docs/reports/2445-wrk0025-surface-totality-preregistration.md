# Report 2445 — WRK-0025 Surface totality-domain preregistration

- Date: 2026-07-28 05:44 JST
- Author / agent: Codex
- Scope: ADR-0014 L3 literal-inventory pre-registration only; no outcome audit
  is executed or relied on in this package.
- Decision levels touched: L3 working-annex registration and required metadata
  only.

## Objective

Pre-register the minimal C0 audit: distinguish forms merely admitted by the
displayed grammar from forms whose total Core-or-Diagnostic outcome is already
specified.

## Scope and assumptions

The audit may report a source-boundary inventory only. It must not define
`WellScoped`, rewrite the grammar, choose a Core operation or diagnostic, or
treat P004/P008/P015 direction records as applied rules.

## Start state / dirty state

Started clean at pushed `616d291cb181c4d71352df0ef8bc1ce4b569c1cd`, after
WRK-0024 evidence linkage. No outcome command has run for WRK-0025.

## Documents consulted

- ADR-0014, working annex rules, spec/01, spec/02, theory/01, theory/03.
- P004, P008, P015, and Plan 199.

## Actions taken

1. Registered the exact source cut, narrow question, alternative, falsifier,
   rollback trigger, commands, permitted locations, and non-claims.
2. Registered WRK-0025 in the Canon MAP and regenerated the Canon index.
3. Deferred the registered source inventory until this registration is pushed.

## Files changed

- `mirrorea_canon/working/WRK-0025-surface-totality-domain-inventory.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2445-wrk0025-surface-totality-preregistration.md`

## Commands run

- `git status --short`, `git rev-parse HEAD`, and `sha256sum` for the source cut.
- Read-only `sed` source audit.
- Canon index/document/source-hierarchy validation will run before the
  registration commit. Registered outcome commands are intentionally deferred.

## Evidence / outputs / test results

No inventory outcome exists yet. The registration and pinned digest list are
the only evidence. The registered source commands run only after commit/push.

## What changed in understanding

P008 A narrows the problem to totality over an exact domain, while P004/P015
remove two broad directions without yet publishing that domain. A literal
inventory can expose the remaining classification boundary without deciding it.

## Open questions

- The exact v0 accepted/rejected source classes and diagnostic coverage remain
  open.
- C2 and C6 remain separate Plan 199 packages.

## Suggested next prompt

Push the registration, execute its source inventory, and retain only the
literal unclassified-form result.

## Plan update status

更新不要: Plan 199 already defines C0; registration creates no result.

## Documentation.md update status

更新不要: reader orientation is unchanged by pre-registration.

## docs/project-status.md update status

更新不要: no project-status claim changes before the audit result.

## progress.md update status

更新不要: no readiness or blocker classification changes before the result.

## tasks.md update status

更新不要: C0 was already listed as active autonomous research.

## samples_progress.md update status

更新不要: no sample or runnable workflow changed.

## Reviewer findings and follow-up

The prior advisory Oracle review required an exact domain and total
Diagnostic coverage before shared-model claims. This registration tests only
the literal source inventory supporting that requirement.

## Skipped validations and reasons

Registered `rg`/Python audit commands are deferred until the pre-registration
is committed/pushed. No runtime/sample/Lean validation is relevant.

## Commit / push status

Pending at report write. The registration will be committed with
`--no-gpg-sign` and pushed before the inventory is executed.

## Sub-agent session close status

No callable sub-agent session was available. Oracle material is advisory only
and is not treated as Canon state.
