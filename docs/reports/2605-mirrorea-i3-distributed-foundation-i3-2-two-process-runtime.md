# Mirrorea I3-2 two-process runtime — bounded close report

## Title and identifier

I3-2 / Report 2605. Final source/evidence cut: `19c5b386613d6adb1f0b934e6ced81acb327d245`.

## Objective

Run checked, generated per-locus artifacts across two actual operating-system processes through the selected private QUIC reliable bidirectional stream, preserving owner, authority, provenance, redaction, ordering fields, and deterministic cleanup.

## Scope and assumptions

This is finite localhost evidence (FM-5), not public or production infrastructure. The source is built once supervisor-side; children receive generated images/control only. Process A hosts `ParticipantA` and `ViewerC`; process B hosts `WorldAuthority` and `ParticipantB`.

## Start state / dirty state

Source base was `4e7302582f6be8e5ab6339777602cf0cd386939b`. The final source cut is the clean pushed revision above; no user changes were discarded.

## Documents consulted

Canon README/MAP, DESIGN-CONSTITUTION, architecture 01–10, plan/00-gates, plan/01-phases, plan/05-i3-entry-contract, ADR-0034, ADR-0037, ADR-0038, ADR-0039/PROPOSAL-042, Plan 250, status snapshots, and the I3-1 report.

## Actions taken

Added the private QUIC process seam, generated-image bootstrap, mTLS and exact-SPKI peer checks, generated request/remote owner serve/write/reply/local receipt, bounded startup/reaping, exact joined delivery records, fail-closed image/preface/SPKI/bootstrap/cleanup falsifiers, and observer-safe lineage checks.

## Files changed

Runtime and probe source/Cargo files are recorded by the source commit. This report and the synchronized LAB snapshots are the documentation changes. Canon acceptance files are recorded separately by the parent integration.

- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `Documentation.md`
- `README.md`
- `AGENTS.md`
- `CANON.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py`
- `plan/00-index.md`
- `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/adr/ADR-0039.md`
- `mirrorea_canon/architecture/README.md`
- `mirrorea_canon/architecture/04-runtime-carriers.md`
- `mirrorea_canon/architecture/06-project-product-layers.md`
- `mirrorea_canon/architecture/10-i3-multi-process-runtime.md`
- `mirrorea_canon/meta/agent-instructions.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/meta/style-guide.md`
- `mirrorea_canon/meta/proposals/PROPOSAL-042-mirrorea-i3-2-multi-process-runtime.md`
- `mirrorea_canon/plan/README.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/plan/05-i3-entry-contract.md`

## Commands run

Focused runtime/probe tests, default and seam variants, default image tests, documentation compile-fail tests, format, warnings-denied Clippy, docs checks, and `git diff --check` were run as applicable. Exact command/output inventory is retained in the closeout status and source review notes.

The accepted regression floor was rerun at close with locked dependencies and
single-threaded tests: `mir-runtime` library filter
`sys6_i2_conformance_tests`, integration test `sys6_i2_cli`, integration test
`m10_conformance`, and integration test `m10_cli`.

## Evidence / outputs / test results

Final bounded evidence: localnet `12/12` repeated; full probe `62/62`; runtime default `29/29`; runtime test-seam `47/47`; runtime library `281/281`; documentation compile-fail `1/1` for default and private configurations. Format, focused/all-feature Clippy, and diff checks passed. No public-workflow claim is made.

The close regression rerun passed SYS-6 conformance `25/25`, SYS-6 CLI `8/8`,
M10 conformance `67/67`, and M10 CLI `4/4`. Canon index freshness covered
208 files, source hierarchy covered 800 required paths, the overview suite
passed `12/12`, and both direct docs validation and `make docs` passed.

## What changed in understanding

The process boundary is meaningful only when the live QUIC connection and stream perform peer/admission checks; a supervisor-issued token or prefabricated control message alone is insufficient. Exact delivery records, rather than non-empty counters, are required for source/Core/artifact/edge/carrier/request/network/runtime correspondence.

## Open questions

I3-3 must still define the complete failure, retry, reconnect, ambiguity, and ordering refinement matrix. Generic post-admission unknown failure semantics, real-time behavior under OS suspension, and durable restart semantics remain open.

## Suggested next prompt

Resume the owner-paused Plan 250 program at I3-3 only after explicit owner/user resume; consume this runtime seam for typed fault and ordering evidence.

## Plan update status

Plan 250 and plan/00-index were synchronized to the accepted I3-2 close and pause state.

## Documentation.md update status

Synchronized to the accepted I3-2 bounded capability and owner pause.

## docs/project-status.md update status

更新済み: accepted I3-2 current status, exact bounded evidence, owner pause, and
official lifecycle non-claims were synchronized.

## progress.md update status

Synchronized axes, milestone map, macro phase, readiness, recent log, and exact evidence class.

## tasks.md update status

Rewritten as a current snapshot: no active semantic milestone while paused; I3-3 remains next and inactive.

## samples_progress.md update status

Updated the I3-2 bounded runnable evidence row; this is not workflow/product completion.

## Reviewer findings and follow-up

Independent review closed with P0=0 and P1=0. Earlier P1 findings (forgeable ingress, weak unauthenticated-result falsifier, unbounded lifecycle cleanup, inferred lineage, and skipped default coverage) and later deadline/observation races were corrected and re-reviewed. Remaining P2/non-claims are explicitly retained below.

The final Canon-first planner review initially found three P1 status/control
contradictions and one P2 stale provenance label. Plan 250's owner-pause
exception and next action, the no-active-package status snapshots, accepted
local two-process wording, explicit resume gate, and ADR-0039 reader-view
authority were corrected. Re-review returned ACCEPT with P0=0 and P1=0; its
last P2 stale overview audit label was also synchronized before integration.

## Skipped validations and reasons

Lean/general theorem, full I3 failure matrix, fuzz campaign, WAN/OS-suspension behavior, production security, durability, browser/provider integration, public compatibility, and official I3 lifecycle checks were not I3-2 requirements and were not represented as passes.

A full workspace test was not rerun at close because the root filesystem had
only 14 GiB free and the existing `target/` occupied 33 GiB after earlier disk
pressure. Focused direct-consumer suites and the accepted SYS-6/M10 regression
floor were run instead. The full workspace is explicitly not counted as a
pass, and no disposable directory was removed without owner confirmation.

## Commit / push status

Source/evidence cut `19c5b386613d6adb1f0b934e6ced81acb327d245` and
Canon/status integration cut `ab038f277d3cfd5ae1db2ad3e1fbc0147dfe0180`
were pushed. The immediate pointer-only closeout commit records the integration
hash and intentionally does not self-reference its own future hash. Final remote
parity is checked after that pointer commit.

## Sub-agent session close status

Implementation, test, evaluation, planning, and independent review assignments closed or handed back. No I3-3 source/report was created or activated. The owner has paused execution after I3-2; this is neither blocked nor stale, and the bounded program is not closed.

## Exact bounded non-claims

The evidence does not claim retry/reconnect/order refinement, the full network failure matrix, C-distributed SCN-01/02/03/06, public or production transport/API/wire, durability, Browser/Host product realization, general proof, exactly-once, WAN liveness, or official I3 lifecycle entry. Preflight source/build/admission/cohort/credential work is outside the child lifecycle deadline; Rustls configuration is finite and names/interfaces remain private/provisional.
