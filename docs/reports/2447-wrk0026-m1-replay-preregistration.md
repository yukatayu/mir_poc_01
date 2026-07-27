# Report 2447 — WRK-0026 M1 replay-discrimination preregistration

- Date: 2026-07-28 05:52 JST
- Author / agent: Codex
- Scope: ADR-0014 L3 literal-inventory pre-registration only; no replay-policy
  outcome has been executed or relied on.
- Decision levels touched: L3 working-annex registration and required metadata
  only.

## Objective

Pre-register the C2 source question: whether M1 validation claims already give
a semantic way to distinguish a replay from a separately intended equal-looking
request.

## Scope and assumptions

The audit must not impose exactly-once, retry, deduplication, or a request
identifier. It can only report whether the pinned texts already supply such a
relation.

## Start state / dirty state

Started clean at pushed `884b20c4f381bdcf7e042bf6eef30d1eca49f700`, following
the WRK-0025 freeze/report package. No WRK-0026 outcome command has run.

## Documents consulted

- ADR-0014 and working-annex rules.
- Theory/01, theory/04, theory/05, spec/05, P012, P013, Plan 193, and Plan 199.

## Actions taken

1. Registered a source-bound question, alternative, falsifier, rollback, and
   non-effects that exclude a request/replay design choice.
2. Registered WRK-0026 in the Canon MAP and regenerated the Canon index.
3. Deferred every registered outcome command until registration commit/push.

## Files changed

- `mirrorea_canon/working/WRK-0026-m1-replay-discrimination-inventory.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2447-wrk0026-m1-replay-preregistration.md`

## Commands run

- `git status --short`, `git rev-parse HEAD`, `sha256sum`, and read-only source
  audits before registration.
- Canon index/document/source-hierarchy validation will run before commit.
  Registered outcome commands are intentionally deferred.

## Evidence / outputs / test results

No replay-discrimination result exists yet. The pinned pre-registration is the
only evidence; the source inventory must wait for commit/push.

## What changed in understanding

M1 selects where validation claims may be formalized, not whether those claims
are a unique request/action identity. This separates anti-spoofing validation
from duplicate/retry semantics without deciding either.

## Open questions

- Whether an additional semantic identity/correlation/policy relation is
  needed remains open until the registered literal inventory runs.
- C0 restart and C6 scalar terminal remain separate work.

## Suggested next prompt

Push this registration, execute the source inventory, and retain only a
literal statement about supplied or missing replay discrimination.

## Plan update status

更新不要: Plan 199 already defines C2; no outcome exists.

## Documentation.md update status

更新不要: pre-registration changes no reader-facing project state.

## docs/project-status.md update status

更新不要: no current-status claim changes before the result.

## progress.md update status

更新不要: no readiness/blocker classification changes before the result.

## tasks.md update status

更新不要: C2 is already an active autonomous package.

## samples_progress.md update status

更新不要: no sample/workflow changed.

## Reviewer findings and follow-up

The prior advisory Oracle review warned that replay/identity cannot be a hidden
side table. This registration tests the narrower source question without
adopting that advisory wording as Canon.

## Skipped validations and reasons

The registered `rg`/Python audit is deferred until this registration is pushed.
No runtime/sample/Lean command is relevant to this source inventory.

## Commit / push status

Pending at report write. Commit/push will precede every outcome command.

## Sub-agent session close status

No callable sub-agent session was available. Oracle evidence remains advisory.
