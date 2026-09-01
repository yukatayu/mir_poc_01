---
id: arch/08-browser-host-security-invariants
status: L1-fixed
maturity: reviewed
depends_on: [root/design-constitution, arch/02-boundary-contracts, arch/07-browser-host-trust-boundaries, theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, theory/09-two-layer-time, adr/ADR-0036]
summary: Browser/Host trust edgeに共通するbinding、freshness、role分離、revocation、redaction、resource/TCB invariant。
open_items: []
---

# 08 — Browser/Host cross-edge security invariants

## Direct consumer and scope

```text
Direct consumer: every BND-007/010--016 implementation; I3-1/3/5; NEXT-0 I5 entry
Blocker reduced: an individually typed edge can still admit substitution, stale
  authority, confused-deputy dispatch, replay, ambiguous external effect, metadata
  leakage or pre-limit resource exhaustion unless the crossings share these rules
Acceptance use: mandatory negative cases for future I3/I5 contracts and tests
```

This fixes semantic responsibilities, not field encodings. Concrete identifiers,
epoch representations, quota units, timeout values, schemas, APIs, ABIs, wire,
storage and sandbox mechanisms remain private/provisional or unresolved.

## Required crossing record

Every implementation of BND-007 or BND-010--016 must preserve enough typed,
content-bound information to decide the following. A single field may safely carry
several items; omission may not be replaced by ambient process/session state.

1. **Parties and scope:** acting principal, package/provider/plugin instance,
   target locus/owner/handler/operation, named consumer and storage/resource scope.
2. **Semantic binding:** exact content/source/Core/artifact or effect-request
   identity, policy/grant epoch, instance incarnation, presentation/semantic
   frontier, authority/witness lineage and freshness/replay domain where applicable.
3. **Admitted intent:** requested capability/effect/failure/resource/data/device
   access and observer/information-flow labels needed by the crossing.
4. **Roles:** semantic decision owner, policy/admission owner, validator, physical
   enforcement agent, evidence reporter and direct consumer. One component may
   fill multiple roles, but the roles and consequences remain distinct.
5. **Order and consequence:** check/admit, grant, allocate and activate are separate
   verdicts. State which verdict authorizes which next step and where use-time
   revalidation or semantic/external-effect linearization occurs.
6. **Failure state:** typed rejection/failure, occurrence identity, replay/duplicate
   rule, ambiguity after possible admission/linearization, and whether any mutation
   or external effect could have occurred.
7. **Lifecycle:** update, revoke, terminate, restart and cleanup consequences for
   queued, in-flight and late result/callback work, cached handles and namespaces.
8. **Observation:** payload and metadata labels, redaction, retention and resource
   budget, including identifiers, presence, counts, timing, size, reason/failure,
   metrics, cache and crash artifacts.
9. **Containment:** accounting principal(s), limit checked before attacker-controlled
   proportional allocation/effect/logging, cancellation boundary and deterministic
   cleanup evidence.
10. **Non-freeze/evidence:** exact provisional surfaces plus positive and primary
    denial evidence required by the direct consumer.

## Time-of-check/use and confused-deputy rules

- `checked`, `admitted`, `granted`, `allocated` and `activated` are distinct. A
  verdict cannot be reused after relevant content, policy epoch, grant lineage,
  instance incarnation, target or resource scope changes.
- Before dispatch/use, the semantic owner revalidates the acting principal,
  instance, target, authority lineage, policy epoch and requested scope. A Browser,
  adapter or provider never substitutes its ambient authority for the caller.
- Restart/reconnect creates new occurrence/session evidence, not fresh authority.
  Queued work, handles and late results bound to an old incarnation/epoch cannot
  cross a revoke/update/terminate boundary without a new explicit verdict.
- A callback is a fresh typed crossing. It inherits no caller/provider authority.

Primary falsifier: swapping content or target after admission, or reusing a verdict,
handle, queue entry, callback or late result after epoch/incarnation revocation,
causes activation, mutation, disclosure or effect.

## Input, presentation and external-effect rules

- Presentation output is a non-authoritative derivative. Derived input has a
  distinct event and command/request identity and binds its target/handler,
  presentation frontier, freshness and replay/duplicate policy. It is revalidated
  against current semantic state and authority; stale presentation never becomes fact.
- Provider results bind the exact effect request, caller/package instance, provider
  incarnation, policy/resource scope, labels, failure row, freshness and designated
  consumer. Provider provenance is evidence, not authority.
- External-effect admission/linearization is explicit. Crash, cancellation or lost
  result after possible effect produces a typed ambiguous state; neither blind retry
  nor exactly-once is inferred. A retry creates new occurrences joined to the
  original semantic request identity under an explicit policy.

Primary falsifier: replay duplicates a semantic/external effect, presentation state
becomes semantic truth, or an ambiguous provider occurrence is reported as success,
safe failure or an automatic retry.

## Redaction, resource and trusted-computing-base consequences

- Redaction covers values and metadata: identity/presence, topology, row/count,
  timing/size, failure/reason, metrics, caches, diagnostic references and crash
  artifacts. Reference existence itself may be private. Retention and rate limits
  apply after redaction; debug mode is not an authority bypass.
- CPU/time, memory, storage, network/effect, device/data and observation cost is
  accounted to the relevant principal, package, provider/plugin, observer and
  operation. Required limits are checked before attacker-controlled allocation,
  logging or external work. Enforcement unavailable means no activation.
- T3 raw-native access enlarges the trusted computing base and may compromise the
  process/host resources it can reach. Process isolation is preferred; in-process
  use must state the expanded T0/TCB and narrower containment claim. T3 has no
  legitimate authority-minting path, but an in-process T3 compromise invalidates
  affected T0 integrity rather than proving the forged action safely contained.
  T1/T2 never obtain raw-native access.
- Termination/cancellation is typed failure/cleanup evidence, not semantic success.
  Cleanup must invalidate handles, queues and namespaces without stale resurrection.

Primary falsifier: metadata or crash artifacts bypass visibility, work begins before
its limit is enforceable, or T3 compromise is described as contained beyond its
actual process/data/resource boundary.

### Per-tier positive/denial and compromise matrix

| Tier | Permitted positive path | Denied path | Trusted assumption / protected scope | Compromise consequence |
|---|---|---|---|---|
| T0 | check, admit, enforce policy, run Mir semantics, validate adapters | own identity becomes owner/grant; missing binding defaults from ambient state | checker/policy/runtime integrity protects semantic state, grant lineage and admitted crossings | affected semantic/authority guarantee is invalid; fail stop/recover from trusted evidence, do not claim preservation |
| T1 | checked package logic within exact declared capability/effect/resource envelope | raw FFI, ambient host/data access, direct store, grant mint | T0 check plus T4 enforcement contains a malicious package to its admitted scope | violation is typed denial/termination; if containment fails, T1 safety claim is invalid for the affected scope |
| T2 | typed, sandboxed provider request/result within policy and resource scope | ambient Mir state/authority, undeclared host access, hidden retry/direct mutation | adapter validation and T4 isolation protect Mir state and out-of-scope host data/resources | provider scope may be compromised; result is untrusted/revalidated and provider is revoked/killed, with ambiguity explicit |
| T3 | separately admitted least-privilege native integration through a trusted adapter | ordinary-package route, undeclared access, legitimate authority mint | declared process/data/resource/TCB boundary only; no T1/T2 equivalence | all reachable host/process/data may be compromised; co-resident T0 integrity is invalid, and actions are breaches, not valid grants |
| T4 | supply OS/browser/engine/device/storage/isolation substrate | process/session/certificate/device identity becomes Mir authority | claimed host isolation and resource/data boundary protect only the stated deployment scope | affected isolation/resources/data—and co-resident T0 if reachable—are untrusted; non-authority wording alone proves no containment |

The positive path is accepted only while the stated assumption and exact crossing
record hold. A denied path is a typed rejection or security breach; it is never
relabelled as an authorized semantic transition.

## Reopen triggers

Reopen ALIGN-2 only if a direct consumer cannot preserve these bindings without
semantic defaulting, safety/redaction weakening or irreversible public freeze; if
ordinary T1/T2 requires raw-native access; or if termination cannot prevent stale
queued/late work from regaining effect. Otherwise concrete mechanisms belong to I5.
