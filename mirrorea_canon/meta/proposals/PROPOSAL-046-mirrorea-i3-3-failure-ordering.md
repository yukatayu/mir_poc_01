---
id: meta/proposal-046
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-037, adr/ADR-0034, adr/ADR-0039, adr/ADR-0040, adr/ADR-0041, adr/ADR-0042, plan/05-i3-entry-contract, arch/10-i3-multi-process-runtime, spec/16-i3-owner-admission-budget, spec/17-i3-read-only-provider-effect]
summary: I3-3の有限network failure・ordering evidenceを受理し、I3-4をinactiveに保ってowner pauseする。
open_items: []
---

# PROPOSAL-046 — Mirrorea I3-3 failure and ordering acceptance

Direct consumer: I3-4 source-first C-distributed scenarios and relation/designated
pressure after explicit owner resume.

Blocker reduced: the accepted two-process fabric lacked a completed finite
failure/ordering profile for loss, reconnect, duplicate delivery, current
authority, declared time/provider failures and local cut admission with traffic.

Acceptance use: I3-4 can consume a reviewed source-derived process/transport
boundary with explicit retained uncertainty and observer-safe fault evidence,
without manufacturing communication edges, grants, state or expected results.

## Disposition and authority

The owner-delegated ADR-0034 bounded program accepts I3-3 at exact
source/evidence cut `fe5dd972e2ddb3a513c785458a07702e4d4d99fa`, under the
original fixed program and the subsequent explicit direction to stop after
I3-3. ADR-0043 records this acceptance. Plan 250 remains the sole authorized
current roadmap, but execution is owner-paused with no active semantic
milestone. I3-4 is next and inactive until explicit resume; I3-5, I3-6 and
NEXT-0 remain ordered and inactive. This is neither blocked/stale nor program
close, and is not official I3 lifecycle entry or exit.

## Accepted finite contract

Ordinary checked source still determines Core, per-locus artifacts, generated
communication, owner operations, declared effects/failures and provenance.
Deployment supplies only logical-locus-to-process/endpoint mapping. Actual
supervised OS children communicate through the selected private QUIC reliable
stream, retain disjoint stores and perform owner-side evaluation. Session,
certificate, address, control purpose and transport occurrence are not grants.

The twenty families in plan/05 have explicit positive/falsifier mappings in
LAB:docs/reports/2606. Network-dependent claims use actual processes/QUIC;
finite decoder, admission-policy, ledger, witness and graph checks remain
explicitly LOCAL. Bounded abstract model evidence is a third class, not a
substitute for network execution. Acceptance is of these finite representatives,
not every network schedule or every variant in each family.

- Before-admission unavailability differs from post-admission request-bound
  ambiguity. Reconnect retains request identity, tombstones and pending state;
  duplicate rejection or previously accepted operation-specific stored-result
  behavior does not become global exactly-once or hidden retry.
- Complete-frame decoding and current receiver binding precede semantic use.
  Genuine capability and source-declared membership successors reject stale
  traffic. Qualified installation/ACK/publication and rejection are kept as
  their actual causal branches, without invented publication-before-rejection
  ordering or session-derived authority.
- spec/16's source opt-in owner ticks govern bounded admission waiting and
  typed expiry. Host supervision deadlines do not become semantic clocks.
  spec/17's separately authorized read-only effect crosses the real host
  boundary and returns typed results/failures; call, retained outcome, delivery,
  current-authority consume and separately authorized observation stay distinct.
- The finite cut profile admits an A-local continuing-runtime boundary after
  the genuine first receipt and before a genuine later source action. It
  checks actual local queues/backend, pending/reserved work, session/ingress
  custody and retained asynchronous obligations. Whole-cohort eligibility and
  a private role-bound purpose prevent ordinary-path escape; the purpose is
  custody provenance, not authority. No snapshot or restore material is issued.
- Actual late-first-reply and header/body-read cancellation with physical close
  preserve pending/history obligations. A genuine LOCAL retained-graph test
  checks receipt → cut admission → later enqueue; separately removing either
  causal record is detected and restored behavior passes.

Observer-safe normal joins remain distinct from fieldless/slot-only private
fault-conformance completion. Private in-child assertions are not advertised
as exported fault traces. Provider `AdapterUnavailable` injected-read/codec
evidence remains LOCAL rather than an actual OS operational-error claim.

## Assurance and validation boundary

Process, adapter, source/runtime correspondence, authority, custody and decoder
evidence is `runtime-monitored`. The dependency-free lifecycle model is
`model-checked-bounded`: one owner, two source request identities, two attempts
per identity, one withdrawal, one disconnect/reconnect cycle and abstract ledger
capacity one; its fixed exploration is 432 states/2136 transitions with faulty
dedup, reconnect-clear and authority-renewal variants detected. The abstract
capacity is not the runtime's concrete capacity or a general Mir limit.

The accepted source passes workspace all-target regression (1573 tests), runtime
doctests (four), formatting and warnings-denied workspace/selected-feature
Clippy. Independent final semantic/security/correctness review has P0/P1/P2=0.
Exact commands, bounds, mutation results, historical failed attempts and
evidence classes remain in LAB:docs/reports/2606. No new Lean result or theory/11
state is asserted. Existing proof/model/runtime classifications are preserved;
general obligations remain intentionally deferred.

## Explicit non-claims and reopen conditions

This does not accept C-distributed SCN-01/02/03/06, cross-process relation or
designated pressure, I3-5 joined network-devtools workflow, finite I3 conformance
or official I3 entry/exit. Those are later fixed milestones. It does not accept
WAN fairness, production security/deployment, arbitrary-network proofs, public
wire/API/ABI/package/FFI compatibility, general leases, Browser/Host package
execution, upper Shared-Space semantics or a renderer/provider owner.

The cut profile is not save/restore, restart, checked patch installation,
distributed quiescence/durability, live provider cut or successful cut reconnect.
The cancellation case does not claim RESET_STREAM, received body bytes or an
observed body-write failure. Generic cut-reconnect refusal and provider-factory
exclusion include source/type-path inspection; an unexecuted defensive branch
is not relabeled as runtime-tested preservation.

Reopen for a reproducible source-free operation/grant, stale authority
resurrection, duplicate mutation/consume, hidden retry or false certainty,
provenance/redaction loss, custody/causal-edge loss, false clean shutdown or an
I3-4 direct-consumer counterexample. Generalization without a direct consumer
does not reopen this accepted finite profile.
