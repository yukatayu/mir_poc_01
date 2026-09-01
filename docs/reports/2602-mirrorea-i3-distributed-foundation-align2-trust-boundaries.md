# Report 2602 — ALIGN-2 Browser/Host trust boundaries

- Date: 2026-09-01
- Author / agent: Codex parent/orchestrator with bounded planner, mapper, test,
  status, reviewer, and ChatGPT Pro Oracle advisory sessions
- Scope: `ALIGN-2` / Mirrorea I3 Distributed Foundation
- Decision levels touched: L1 architecture and boundary clarification under the
  owner-authorized PROPOSAL-037 / ADR-0034 program

## Objective

Give future Browser/Host participation an exact non-freezing trust boundary so
third-party Mir packages can be admitted, connected to Mirrorea, projected to a
View/renderer and connected to typed inputs/providers without package, host,
transport, renderer or native integration becoming semantic authority.

Direct consumers are I3-0 transport non-authority, I3-1 encoding/admission/log
limits, I3-5 observer-safe network views, and NEXT-0's inactive I5 entry contract.

## Scope and assumptions

This milestone defines responsibilities and security invariants only. It does not
implement I5 or choose a package/origin/signature/storage format, sandbox, browser
UI, renderer/engine, Unity/Unreal plugin ABI, public FFI/API/ABI/wire, transport or
production deployment. The owner direction is the authorization for this L1
clarification; the North Star and safety/authority/privacy guarantees are preserved.

## Start state / dirty state

- Pinned start revision: `3a7b3c7192d72f8b6f6c94214a9ce335f878cd65`.
- `HEAD`, local `main`, and `origin/main` matched and the worktree was clean.
- ALIGN-0/1 were completed; ALIGN-2 was sole active, I3-0 next/not active,
  official I3 lifecycle unentered, both transports UNSELECTED, OPEN-032 unresolved.
- Root storage remained near the earlier audited 89% use with about 21 GiB free.
  No build/cache or generated-artifact tree was added.
- No git worktree was created.

## Documents consulted

Canon-first review covered NORTH-STAR, DESIGN-CONSTITUTION, MAP, architecture
01/02/05/06, theory 04/05/07/09/13, plan 01/02/05, ADR-0005/0011/0034/0035,
PROPOSAL-037/038, and the exact ALIGN-2 contract in LAB Plan 250. Only directly
relevant LAB evidence in specs/31, specs/38, plan/63 and Report 2601 was read;
`docs/reports/` was not scanned in bulk.

The independent Oracle received a context packet and returned advisory output
at `/tmp/align2-oracle-review.md` (SHA-256
`5b0197f57cbfba278da631c8a6626c4c1414454aad47d2ea7f39f05ce28a25c7`). Its model
selection metadata did not independently verify Pro selection, so no stronger
claim is made.

## Actions taken

- Added PROPOSAL-039 and ADR-0036 through the forward-only Canon process.
- Clarified BND-007 from the ambiguous “View has no logic” shorthand to: View
  owns no authoritative domain semantics or direct mutation path, but may perform
  presentation-local computation over an observer-safe projection.
- Added BND-010--016 for package admission, Browser-to-fabric participation,
  View-to-renderer, typed input, typed effect/provider, privileged raw FFI and
  resource/sandbox enforcement.
- Defined trust tiers T0--T4 as local trust labels distinct from Theory T0--T2,
  semantic strata, PL layers and a numeric privilege lattice.
- Added the cross-edge security record for exact content/request, instance,
  target, epoch/lineage/frontier/freshness and scope binding; role separation;
  use-time revalidation; queued/in-flight revocation; ambiguous external effects;
  metadata redaction; pre-limit accounting; and T3 trusted-computing-base effects.
- Separated package admission from semantic grant, T1/T2 from raw FFI, forward
  projection from typed reverse input, and typed providers from direct mutation.
- Updated current Canon/LAB/status/reader pointers to ALIGN-0--2 completed,
  I3-0 sole active, I3-1 next/not active, with lifecycle/selection unchanged.
- Added a reader regression that was first RED on the missing contracts/status
  and then GREEN after integration.

## Files changed

Normative decisions and architecture:

- `mirrorea_canon/meta/proposals/PROPOSAL-039-align2-browser-host-trust-boundaries.md`
- `mirrorea_canon/adr/ADR-0036.md`, `mirrorea_canon/adr/README.md`
- `mirrorea_canon/architecture/02-boundary-contracts.md`
- `mirrorea_canon/architecture/05-satellites.md`
- `mirrorea_canon/architecture/06-project-product-layers.md`
- `mirrorea_canon/architecture/07-browser-host-trust-boundaries.md`
- `mirrorea_canon/architecture/08-browser-host-security-invariants.md`
- `mirrorea_canon/architecture/README.md`
- `mirrorea_canon/NORTH-STAR.md`, `mirrorea_canon/GLOSSARY.md`
- Canon navigation, changelog, index, plan and meta current pointers.

Derived LAB/current readers and evidence:

- Plan 250 and `plan/00-index.md`
- root `AGENTS.md`, `CANON.md`, `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`, `tasks.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py`
- this sole ALIGN-2 report

No Rust, Lean, model, scenario, sample, runtime or generated evidence artifact was
changed. `samples_progress.md` was not changed.

## Commands run

- Revision/parity/dirty-state and focused `rg`, `sed`, file-size and diff scans.
- Canon index generation and check.
- Canon index unit tests, source hierarchy, reader HTML regression, full docs
  validation and `make docs`.
- `git diff --check` and scoped secret-pattern scan.
- Browser-backed Oracle consult plus independent local planner/security review.

## Evidence / outputs / test results

- Test-author RED evidence: 11 reader tests with four expected new failures before
  the Canon/reader/status integration; the later root-CANON frontier guard also
  failed once against the stale pointer before its fix.
- Integrated reader result: 12/12 passed; combined reader/index unit result 17/17.
- Canon index currently includes 200 files; generation/check passes.
- Canon architecture files 07 and 08 are below the 15,000-byte limit; phase plan
  remains 14,999 bytes.
- Source hierarchy has 800 required paths, 800 present, 0 missing.
- Full docs validation passed: scaffold complete, 1,756 numbered reports.
- `make docs` passed after agent config, 200-file Canon index, 800/800 hierarchy
  and the same full docs validation.
- `git diff --check`, agent config validation and scoped secret-pattern scan passed.
- Independent exact-diff security/semantic review returned ACCEPT with no P0/P1.

## What changed in understanding

An individually typed boundary is insufficient when its verdict can be reused
after content, target, policy, authority lineage or instance changes. The accepted
cut therefore treats checked/admitted/granted/allocated/activated as separate
states and makes use-time binding/freshness, revocation of queued/late work,
metadata redaction and ambiguous external-effect state cross-edge requirements.

View “logic” was the wrong dividing line. The stable line is authority: presentation
may compute locally, but semantic meaning, mutation, persistent truth, grant,
fallback lineage, patch admission and information-flow policy remain in Mir.

## Open questions

- Package/origin/signature/storage formats, sandbox technology, quota units,
  Browser UI, renderer/engine and public plugin/FFI/API/ABI remain UNRESOLVED for I5.
- OPEN-032 and both reliable-stream candidates remain unresolved/UNSELECTED for I3-0.
- Concrete Shared-Space mechanisms and Reversed Library product design remain later
  owner-reserved programs.

None is an ALIGN-2 blocker.

## Suggested next prompt

Continue autonomously with I3-0 only: implement the same finite two-process canary
for TLS-over-TCP framed reliable stream and QUIC reliable stream, compare both by
the fixed criteria, obtain security/network/semantic review, and select at most one
without freezing a public wire or beginning I3-1 early.

## Plan update status

`plan/` 更新済み: Plan 250 remains the sole roadmap, marks ALIGN-2 completed and
I3-0 sole active, keeps I3-1 inactive, and records the accepted cross-edge security
requirements. `plan/00-index.md` mirrors the transition without becoming normative.

## Documentation.md update status

`Documentation.md` 更新済み: the accepted trust edges, cross-edge invariants,
I5 non-implementation and I3-0 current frontier are synchronized.

## docs/project-status.md update status

更新済み: ALIGN-2 completion, I3-0 active state, trust/security Canon sources,
unchanged lifecycle and transport non-selection are synchronized.

## progress.md update status

`progress.md` 更新済み: snapshot tables and a command-derived timestamped ALIGN-2
close log record the boundary acceptance without claiming implementation progress.

## tasks.md update status

`tasks.md` 更新済み: it is a full current snapshot with I3-0 as the only active
self-driven package and OPEN-032 as a delegated research decision, not user blocker.

## samples_progress.md update status

`samples_progress.md` 更新不要: ALIGN-2 changes no runnable sample, command,
debug surface, validation workflow or sample blocker.

## Reviewer findings and follow-up

The pre-edit planner returned GO with no P0 and required the exact BND split,
trust-tier distinction, old View wording reconciliation, non-freeze clauses and
I3-0 transition. All were incorporated.

The Oracle accepted the eight-edge decomposition but initially blocked the common
schema for time-of-check/use, confused-deputy, late-work, metadata and external-
effect ambiguity gaps. Those findings were checked against Canon authority and
incorporated into architecture/08 and ADR-0036 without adopting concrete schemas.

Final independent review initially found three P1s: stale root CANON frontier,
fabric-as-authority-owner wording, and incomplete per-tier compromise semantics.
All were fixed with a new regression guard, Mir semantic-owner wording and the
T0--T4 permitted/denied/assumption/compromise matrix. It also found one invalid
project-status path label. After that mechanical fix, final disposition was
ACCEPT with no remaining P0/P1; all required validators passed.

## Skipped validations and reasons

Rust format/Clippy/workspace, accepted I2 runtime suites, Lean, bounded models,
transport canaries, fuzzing and multi-process tests were not rerun because this
architecture-only milestone changes no production, formal, scenario, sample or
runtime surface. ALIGN-0 freshly preserved the accepted I2/M10 regression floor.
None of these skipped commands is reported as passing for ALIGN-2.

## Commit / push status

Exact-diff review and final validation are complete. The report cannot embed its
own future commit hash. The parent commits with `--no-gpg-sign`, pushes to
`origin/main`, verifies `HEAD == main == origin/main` and a clean worktree, then
report the accepted cut at the milestone checkpoint.

## Sub-agent session close status

Completed bounded sessions: pre-edit planner, boundary mapper, HTML test author,
status writer, Oracle operator and final independent reviewer. Every advisory
finding was compared by the parent against the exact Canon and diff before use.
