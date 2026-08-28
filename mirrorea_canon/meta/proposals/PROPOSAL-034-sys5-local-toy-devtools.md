---
id: meta/proposal-034
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0026, adr/ADR-0029, adr/ADR-0030, arch/03-toolchain, arch/04-runtime-carriers, theory/05-authority, theory/14-maintained-relation-projection, spec/12-sys3-per-locus-projection, spec/13-sys4-in-process-generated-dispatch]
summary: SYS-5の四locus local toy fabric、source-bound leave/fresh lifecycle、observer-safe joined devtoolsをcut 53a21e64で有限受理する提案。
open_items: []
---

# PROPOSAL-034 — SYS-5 local toy fabric and typed devtools

## Owner disposition and selected capability

Under ADR-0026, accept the smallest headless SYS-5 vertical slice that lets a
person see one ordinary `.mir` source become four generated locus programs and
actual generated dispatch:

```text
ordinary source
  -> checked Core and deterministic per-locus projection
  -> WorldAuthority + ParticipantA + ParticipantB + ViewerC runtimes
  -> owner RMW, designated publication/consume, maintained relation lifecycle
  -> observer-safe source/Core/artifact/edge/occurrence joined report
```

The selected provisional commands are `mir project-loci`, `mir run-local`, and
`mir inspect`. They are one finite local workflow and are not a public CLI,
JSON, API, ABI, artifact, or wire compatibility promise. Runtime execution
consumes the already checked/projected project and sealed admission; it does
not reparse source, choose a plan from a fixture name, consult expected JSON,
or accept caller-supplied route, Core, authority, semantic state, or result.

World, participant, viewer, avatar, and bird are sample/library names only.
They add no Mir Core primitive.

## Explicit relation-anchor locus refinement

Accept the provisional internal relation-anchor form
`primary A at L epoch E transform T` and the corresponding fallback form. The
explicit locus is retained through AST, M6 classification, M7 checked Core,
projection, and source map. An explicit unknown locus is rejected before
projection. The accepted local toy relation is owned by `ParticipantB`, while
its primary existence anchor is at `ParticipantA`, fallback anchor at
`ParticipantB`, and consumer-local projection at `ViewerC`; these roles must
not be collapsed.

The earlier form without `at L` remains a bounded compatibility input and
retains no invented anchor locus. The SYS-5 leave/fresh lifecycle requires an
explicit checked primary anchor locus and fails closed when it is absent. This
is an internal reference-grammar refinement, not final/public syntax or a
compatibility guarantee.

## Source-bound leave, fallback, and fresh lineage

The external lifecycle action names only the checked relation. The runtime
derives the primary participant locus from that relation's checked explicit
anchor, then M9 derives the exact admitted principal, membership, incarnation,
capability, and witness lineages. A successful ParticipantA leave:

1. records a distinct external request/enqueue occurrence;
2. retires the exact M9 membership and its capability/witness lineage into a
   monotone successor/tombstone;
3. only afterward lets the independent ParticipantB relation owner invalidate
   the primary and publish the fallback through its generated endpoint; and
4. returns typed observer-safe receipt/evidence without mutating ViewerC or
   transferring ParticipantB owner authority.

The action is source-bound because its only semantic operand and participant
placement come from checked relation Core. There is no new ordinary Surface
`leave` production, no caller-selected membership reference, and no transport
authority. Repeating the same leave fails as a typed duplicate without a
partial M9 successor, M8 state/relation/designated mutation, or authority mint.

Fresh reacquire consumes the exact retained tombstone lineage of that leave.
M9 issues a distinct membership epoch and incarnation and fresh checked
capability/witness lineages before ParticipantB republishes the primary. The
caller cannot supply an epoch, incarnation, membership reference, grant,
witness, or authority. The fresh receipt's retired-membership and retired-
epoch references equal the preceding leave's tombstone and successor-epoch
references; a merely similar but unjoined new admission is not accepted.

## Failure-atomic candidate and cut continuity

The selected leave, primary invalidation, and fresh-reacquire transitions are
ST-only failure-atomic candidates. Identifier capacity and all required
source/anchor/M9/endpoint facts are checked before commit; execution is
prepared on a detached candidate and replaces the live fabric only after the
complete transition succeeds. Route failure, stale/duplicate membership,
missing explicit anchor, identifier exhaustion, or endpoint failure leaves the
live M9 generation, M8 state/relation/result, queues, causality, and observer
state unchanged, so the same valid action can be retried after the injected
fault is cleared. This is a finite implementation contract, not a general
transaction theorem or a hidden multi-owner transaction.

The ST local cut additionally retains completed leave evidence, exact retired
lineage, relation shadow, M9 lifecycle/live floor, and lifecycle occurrence
counter. An exact post-leave cut restores that state and permits the same exact
tombstone to be consumed by fresh reacquire. Corrupt leave evidence or an
inconsistent cut rejects before partial restore. This is process-local memory,
not durable save/load; OW1 leave/fresh/cut remains outside this acceptance.

## Observer-safe joined devtools and toy workflow

The accepted report presents, in one deterministic JSON document, source
span, checked Core reference, source/target locus fragments, generated edge,
request identity, distinct enqueue/dispatch/receive/serve/observe occurrences,
owner mutation, designated result/cache version, relation selected anchor and
lineage, presentation gap, leave/fresh lifecycle, typed authority failure,
save/restore branch, patch lifecycle, and verification discharge.

Request identity is not an occurrence or queue position. Rows after the saved
cut are labeled as active prefix, discarded post-cut, or active restored; a
reader need not infer a false linear history. Patch input retains its safe
logical source label, checked-program identity, exact patch reference, and
actual patch occurrence. Raw source text, credentials, capability/witness
material, private state/payload, and raw M8/M9 identity do not enter the
observer-safe report.

The canonical workflow exercises actual owner-side attack RMW, designated
tick/publication and named consume, B-owned bird relation with A-primary and
B-fallback, local cut/restore, one accepted designated-only patch, one rejected
owner-RMW-changing patch, A leave and duplicate rejection, ViewerC
presentation gap without semantic mutation, exact fresh reacquire, consumer
capability revocation and typed failed consume, and one finite optional
verification discharge. Patch activation precedes leave/fresh in this selected
trace; commutation of patch activation with an already advanced membership
lineage is not accepted here.

## Falsifiers, evidence, and stop condition

The contract is falsified if anchor placement is inferred from topology; leave
accepts a caller-supplied membership/grant; M8 mutation precedes M9 retirement;
fallback mutates the consumer; fresh reacquire is not joined to the exact
tombstone; a failed candidate partially changes live state; post-leave restore
loses its retired lineage; report rows invent causal links or merge forked
branches; observer output leaks raw protected material; or the workflow chooses
semantics from a filename/expected JSON.

The accepted implementation/evidence cut is
`53a21e64b5a17e24b522f720db10b6e539c058e0`. Fresh evidence includes AST 10/10,
M7 pipeline 27/27, SYS-3 28/28, local workflow 8/8, relation dispatch 17/17,
cut/patch 12/12, CLI 3/3, M9 leave lifecycle 4/4, the complete `mir-runtime`
all-target test run (245 library tests plus all integrations), preserved M10
source/CLI/conformance 2/4/67, formatting, `mir-runtime` all-targets
warnings-denied Clippy, diff validation, and manual CLI/redaction inspection.
Independent M9 authority/concurrency, semantics, and usability/security reviews
of the final equal-generation shared-floor cut returned ACCEPT with no P0/P1.
OBL-062 classifies only this finite executable/devtools correspondence as
`runtime-monitored`. Repository-wide closeout validation remains SYS-5 report
evidence and does not enlarge this proposal's claims.

```text
Direct consumer: SYS-6 finite I2 assurance/conformance profile
Blocker reduced: SYS-4 had typed endpoint evidence but no human-runnable
  four-locus source-first workflow or one joined observer-safe causal view
Acceptance use: source -> Core -> locus artifact -> generated edge -> actual
  occurrence conformance rows and selected lifecycle/failure falsifiers
```

Close SYS-5 at this cut. Advance the sole active goal to SYS-6. Reopen only for
one of the falsifiers above or if SYS-6 cannot consume these typed rows without
reconstructing semantics from source, fixture names, or expected output.

## Non-effects

This proposal does not accept broad PHASE-I1, official I2 entry/exit, public
CLI/API/ABI/wire/JSON, final grammar, browser/View/renderer, real transport,
multi-process execution, durable persistence, OW1 lifecycle/cut/patch,
arbitrary relation DAG, general failure-atomicity/save/reacquire/projection/
noninterference theorem, production deployment, or I3 implementation. Theory
remains T1 and M10 remains the unchanged regression baseline.
