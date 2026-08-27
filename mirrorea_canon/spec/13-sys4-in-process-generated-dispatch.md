---
id: spec/13-sys4-in-process-generated-dispatch
status: L1-fixed
maturity: draft
depends_on: [spec/05-runtime-semantics, spec/09-m8-deterministic-runtime, spec/10-m9-auth-verification, spec/12-sys3-per-locus-projection, arch/03-toolchain, arch/04-runtime-carriers, theory/13-evaluation-materialization, adr/ADR-0030]
summary: cut 22196f93で受理したSYS-4 generated-plan-only locus runtime、endpoint dispatch、typed faults、ST local cut、designated-only checked patchの有限仕様。
open_items: []
---

# 13 — SYS-4 in-process generated dispatch

## Admission and runtime boundary

The selected finite route is:

```text
GlobalProjectionResult
+ exact complete sealed M9 admission
+ typed initial values for projected local schemas
  -> LocalFabric::bootstrap
  -> LocusRuntime[locus] + generated endpoints
  | typed Sys4DispatchDiagnostics
```

Bootstrap requires exact checked-program and projection identity, complete M9
residual discharge and authority inventory, exact projected locus inventory,
and schema/owner/index/field-valid initial values. Failure creates no live
partial fabric. Initial values populate existing owner-local projected state;
they cannot add an operation, locus, edge, schema, authority, capability,
witness, membership, Core node, or expected result.

Each `LocusRuntime` retains its exact `LocusProgram`, local store, incoming and
outgoing endpoint state, local trace, and runtime-admission evidence. Endpoint
routes are the complete set produced by the SYS-3 `CommunicationPlan`. There is
no fallback to source parsing, fixture-name dispatch, expected-output lookup,
manual interfaces, or a global mutable semantic store.

Internal Rust names in this chapter identify the accepted finite evidence and
are not a public API, ABI, artifact format, JSON schema, or wire contract.

## Dispatch state machine

An external action may invoke a source-derived handler with typed ordinary
arguments, advance a declared designated tick, request an already-defined
save/load/patch boundary, or arm a bounded fault. It cannot name a replacement
target, create an edge, provide authority, inject Core/state/result, or bypass
the generated plan.

For a generated endpoint, the accepted runtime separates:

```text
source handler invocation
  -> source-local carrier creation and outbox send
  -> generated-route transport step
  -> target inbox receive and dequeue
  -> exact carrier + current M9 revalidation
  -> target-local M8 serve/evaluate/consume or typed failure
  -> reply/publication/receipt or terminal quarantine
```

Every actual row binds its checked program, source/Core reference, source and
target fragment refs, generated edge ref, envelope/request identity, exact
occurrence identity, route, effect/failure row, visibility/redaction, current
M9 generation, and applicable publication/frontier/consumption identity.
Queue position, endpoint, worker, transport step, receipt, and observation do
not grant authority.

Owner requests preserve the whole same-owner RMW at the owner locus. An origin
locus gets no owner-private read or direct store access. Designated remote
input is served at its source owner and consumed at the named evaluator. A
designated result is published by the evaluator and delivered only to the
source-named consumer fragment. Relations remain owner publication plus
consumer-local projection; this runtime does not create an absolute-value
stream.

## Semantic-consumption retry

For one accepted `DesignatedResultDelivery`, define the finite identity:

```text
checked program + designated operation/result + named consumer
+ source/Core/edge provenance + M8 publication identity
+ input/result frontier + version + policy/visibility/redaction binding
```

The first exact delivery imports the accepted M8 publication into the consumer
partition and performs one M8 semantic consume. The runtime stores the typed
decision and exact binding. A retry with that identical identity returns the
stored value and appends no second semantic-consumption row. A different
consumer, publication, frontier, version, policy, provenance, visibility,
redaction, or binding digest is not a retry and fails before cache success or
M8 consumption.

This rule implements `[E-CONSUME]` for one bounded source-named consumer. It is
not transport retry, exactly-once delivery, implicit callback, hidden
transaction, multi-consumer semantics, or permission to reinterpret the M10
same-delivery `AlreadyConsumed` regression.

## ST and OW1 profiles

ST is the reference process-local profile. Each semantic locus has an
independently keyed M8 session and owner-local store. The accepted four-locus,
two-owner pressure case crosses generated endpoints and mutates only the
corresponding owner partitions.

OW1 is eligible only under ADR-0028's exactly-one combined semantic owner/
source-owner constraint. It runs the same projected artifacts and generated
plans while its worker exclusively owns the M8 session. Its selected owner and
designated results, failure class, and source/Core/artifact/occurrence
correspondence must agree with ST. An ineligible topology fails typed before
state is shared or duplicated.

An OW1 observer snapshot is clone-only observation. A worker failure returns
typed `ObserverSnapshotUnavailable`; genuine absence remains `Ok(None)`. The
runtime must not substitute a stale cached snapshot, turn a committed semantic
success into failure, or replay the operation. Recovery obtains the latest
exact worker snapshot. This observer contract transfers no authority.

## Failure and quarantine rules

The finite diagnostic/failure surface includes:

- route unavailable, unavailable envelope, wrong target, or unknown projected
  edge;
- stale membership or authority generation, missing capability/witness,
  missing producer release/evaluator/consumer authority;
- duplicate, stale, mismatched, or missing result, receipt, publication, tick,
  or semantic-consumption identity;
- split-frame frontier/version/policy/publication mismatch;
- missing payload and provenance/visibility/redaction/policy/cache corruption;
- post-dequeue owner/evaluator/consumer M8 rejection; and
- backend ineligibility or observer snapshot unavailability.

Validation occurs at the earliest boundary that owns the relevant fact.
Pre-dispatch failure creates no endpoint carrier. In-transit or target-side
failure retains observer-safe attempted route/provenance and terminally
quarantines the carrier when necessary. No failure may mutate an unrelated
store, mint authority, consume a failed cache entry, fabricate an M8 success,
or prevent a later independent accepted carrier from advancing.

## Bounded ST local cut

`Sys4LocalCut` is a process-local finite consistent cut, not a durable/public
serialization format. For ST it retains and validates:

```text
exact projected program and M9 authority generation/live floor
M9 admitted-validation counters and observer-safe validation audit maps
per-locus M8 cuts, local state, M8/raw/qualified/observer-safe trace
incoming/outgoing mailboxes and symmetric send/receive endpoint records
pending carriers, typed route/in-transit faults, completed receipts
designated publication/import/consumption/cache state
request and endpoint occurrence counters, causality and dependencies
patch generation, frontier, and accepted/rejected lifecycle rows
```

Restore preflights the complete cut and installs nothing on failure. It rejects
identity/projection drift, old M9 generation or changed authority live floor,
missing/duplicate/asymmetric endpoint records, invalid M8 dependency or
causality, counter rollback, stale consumption/publication state, and forged
patch frontier/lifecycle. An accepted in-flight carrier resumes once. OW1
save/restore is typed `BackendIneligible` until an acknowledged worker-cut
protocol is specified and implemented.

## Bounded checked patch

The runtime patch input is only:

```text
Sys4CheckedPatchCandidate {
  patch identity,
  exact base Sys4PatchFrontier,
  already checked/projected FabricProgram,
  complete sealed M9 admission,
  computed bounded compatibility
}
```

It contains no raw source, AST, manual edge, grant, or expected result. The
accepted profile requires ST quiescence, exact base program/projection/
activation frontier, exact M9 authority lineage and current shared live floor,
complete candidate admission, unchanged locus/fragment/edge/state-schema and
owner-route shape, unchanged owner RMW and all non-designated Core/edges/
handlers/relations, and a designated-only material change.

The candidate installs on a clone first. Only after all checks pass does M9
atomically rebind the same authority live floor to the normally admitted
patched program. The fabric swaps the prepared state, advances one patch
generation, retains authority lineages and tombstones, and clears old
designated publication/consumption caches. Rejection appends one typed patch
lifecycle row and changes no semantic or authority state. OW1 patch is typed
`BackendIneligible`. This profile defines neither arbitrary patch
compatibility nor general hot-plug.

## Evidence and non-claims

The accepted finite implementation cut is
`22196f93b0112b8fd2987ec078021c8865b71651`. The focused SYS-4 suite has 99
passing tests and the complete runtime library suite has 179 passing tests at
that cut. Preserved M10 source/CLI/conformance regression, format,
warnings-denied Clippy, diff validation, and independent review also accepted
the bounded scope. OBL-061 records only `runtime-monitored` evidence.

This specification does not claim a CLI, public API/ABI/wire/JSON/artifact
format, real transport, deployment mapping, durable recovery, OW1 cut/patch,
multi-owner OW, arbitrary relation DAG, arbitrary patch, general
projection/dispatch/retry/save/refinement theorem, browser/View, final
devtools, broad PHASE-I1, official I2 entry, or I2 exit.
