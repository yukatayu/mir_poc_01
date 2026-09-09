---
id: spec/17-i3-read-only-provider-effect
status: L1-fixed
maturity: reviewed
depends_on: [root/design-constitution, theory/02-types-effects-failures, theory/05-authority, theory/07-observation, spec/08-m7-checked-elaboration, spec/10-m9-auth-verification, spec/12-sys3-per-locus-projection, arch/07-browser-host-trust-boundaries, arch/08-browser-host-security-invariants, arch/09-i3-private-adapter, arch/10-i3-multi-process-runtime, adr/ADR-0034, adr/ADR-0042, meta/proposal-045, adr/ADR-0043]
summary: I3-3 provider failureへsource-derived read-only effectと有限T0/T4境界を選択する。実装受理ではない。
open_items: []
---

# 17 — Finite read-only external effect

ADR-0042 selects this contract, not implementation acceptance.
Direct consumer: I3-3 provider failure/ordering. Blocker reduced: missing
source-derived invocation. Acceptance use: checked network invocation,
actual host read, typed result/failure consumption.

## Source, Core and placement

At most one external-effect declaration is admitted per checked program;
old forms are unaffected. This zero-argument, read-only invocation
returning an `Int`, with an explicitly named requesting principal/locus,
execution locus, logical resource slot, result consumer, observation policy,
declared failure row and finite resource allowance. The consumer is the
requesting locus in this profile. These are checked source facts, not deployment
inputs. The fixed consumer rule becomes an explicit checked Core coordinate,
not a separate freely chosen syntax clause. Provisional notation:

```text
effect sample_value by self at ParticipantA
  invoke read_int at WorldAuthority resource sample_input
  returns Int visible observer_safe
  limit bytes 32 calls 64
  fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied,
         RouteUnavailable, ProviderResourceNotFound, AdapterUnavailable,
         ProviderInvalidResult, ProviderPolicyDenied, ResourceExhausted)
```

`read_int` names a finite adapter profile, not Core standard I/O or a public
library. Core represents external invocation, not RMW, pure evaluation, state
read or authentication. No domain/file/native-path primitive enters Core.
Consume the typed value once, without owner state write, receipt or grant.

M6 retains typed invocation metadata and exact source spans; M7 checks the
zero-argument signature, declared loci/principal, unique operation/resource
binding, result type, failure containment, allowance and observation policy.
The finite allowance is exactly 32 input bytes per call, at most 64 invocation
reservations per admitted operation, one executing call at a time, and at
most 64 retained request decisions per runtime. Other allowances reject in
this profile rather than being silently clamped. Source declarations request
these limits; they do not authorize them. Calls count reservations, including
every post-reservation failure or uncertain outcome, without refund in the live
admitted context. Only fresh admission creates a new allowance; it also creates
a new incarnation and does not authorize retry of an old request.

Checked identity and private snapshots retain every coordinate. Generated
projection contains distinct effect requester/service/result-consumer
fragments and invocation/result edges, with exact source/Core/artifact/contract
references. A source action selects only an existing projected effect; it
cannot supply a new edge, provider identity, grant, path, result or failure.
Unsupported legacy execution or executable export rejects the new form before
any invocation, rather than erasing it into accepted owner/pure semantics.
Existing sources and M5/M6/M7 proof claims are not widened by this extension.

## Admission and authority

Initial checked admission and a separate effect policy decision are both
required. An effect-use capability is bound to the exact admitted instance,
principal/membership epoch and incarnation, operation, source/Core/contracts,
requester, executor, consumer, logical resource, allowance, policy generation,
provider incarnation and release labels. In this no-extra-provider-process
profile, provider incarnation is the exact admitted runtime/resource-binding
incarnation, not the filesystem name or a QUIC session. Bind it in request,
reservation, outcome and consumption checks. Fresh setup/rebinding/restart
invalidates old queued handles and late results; cleanup follows invalidation.
Its issuer is the trusted Mir authority/policy boundary,
not the provider. Ordinary owner-write permission, a declaration, valid QUIC,
certificate, endpoint, provider identity or successful read cannot substitute.

The finite trusted policy admits only this named read-only profile and its
exact declared slot/allowance after the existing membership/authentication
and verification verdict. An opaque effect authorization is issued only from
that decision. Missing, wrong, retired or revoked grants reject before the
host call. Arbitrary caller-created tokens and public authentication-provider
claims cannot issue it. Revalidate current authority at invocation and again
at result release/consumption; initialization is not a permanent use permit.

Invocation reservation, call-start commitment and revocation serialize in the
effect-owning runtime. A revocation before `CallStarted` prevents invocation,
including when a capacity reservation already exists. Revocation after
`CallStarted` preserves possible/actual invocation and prevents unauthorized
release or consumption; it cannot manufacture a no-call fact. Checked
provider/resource rebinding is not supported while the
admitted instance is live; require quiescent termination and fresh admission.

## Bounded host crossing and exact outcomes

Roles remain distinct: Mir source defines meaning; T0 policy issues permission;
the executor locus performs current checks and linearization; the trusted
adapter validates the host crossing/result; T4 filesystem performs physical
I/O only; the requester is the typed consumer; an observer projection is a
separately authorized information release. No physical provider owns Mir state.

Use a fixed trusted adapter (T0) to a host filesystem service (T4) over a
dedicated, nonsecret, task-local fixture namespace. This is not a T2 sandbox,
T3 native-plugin admission or a claim that untrusted host code is contained.
The fixture is outside source samples and provisioned by trusted local setup.
Setup binds only the already declared logical slot to that namespace; neither
source nor network input carries a native path. No arbitrary user file, path
traversal, symlink, device, FIFO, socket or remote filesystem is admitted.
The private namespace and its binding remain under T0 control for the entire
run; concurrent hostile T4 replacement is outside this finite trust profile,
not claimed safe by a preliminary path check.

Each authorized first invocation attempts an actual regular-file read. Enforce
the byte limit with a bounded read of at most 33 bytes (the extra byte detects
overflow); do not read an unbounded file and check afterward. The accepted
payload is ASCII `0`, `[1-9][0-9]*` or `-[1-9][0-9]*` within the i64 range,
optionally followed by exactly one LF. No plus sign, leading zero, negative
zero, spaces, CR or trailing bytes are accepted. The optional LF counts in the
32-byte limit. A second fresh admitted run with changed fixture content must return
the changed value. No request, action or fault tag supplies the returned value.

The result sum is `Value(Int) | DeclaredFailure(atom)`, not FabricReceipt:

| Actual reached boundary | Outcome |
| --- | --- |
| Valid bounded integer read and current release permission | `Value(Int)` |
| Actual resource lookup/open/read returns `NotFound` | `ProviderResourceNotFound` |
| Actual host service/open/read operational error other than missing resource or access denial | `AdapterUnavailable` |
| Invalid integer, overflow, noncanonical encoding or excess bytes | `ProviderInvalidResult` |
| Denied binding/resource policy, unsupported kind, or actual host `PermissionDenied` | `ProviderPolicyDenied` |
| Allowance or retention capacity unavailable before invocation | `ResourceExhausted`, no host invocation |

Authority, visibility, malformed-carrier and transport failures retain their
own typed boundaries; they are not renamed provider failures. A source omitting
any generated failure is rejected statically. Error diagnostics contain only
the typed class and permitted provenance, never an OS error string, path,
fixture contents, private source, credentials or witness payload. The `Int`
is released only to the designated consumer under its current release label;
`observer_safe` is not permission to include raw results in diagnostics.
Observer projection separately validates label, redaction, retention and rate
for opaque identity/presence, counts, timing, size and reason as information
effects. Unpermitted fields are redacted or the observation is rejected.
Observer records contain only permitted opaque provenance and typed classes,
never native paths, raw results, grants or witnesses.

The finite adapter executes no supplied code and allocates only bounded input,
result and ledger storage. Charge fixed input (33 bytes), result, reservation,
decision, permitted observation and generated message capacity to the admitted
principal/instance/operation before the corresponding work. Existing frame,
queue and child-event bounds remain in force; no unbounded allocation or log
precedes the capacity check. At most one invocation runs, at most 64 request
and result decisions remain live, and rejected duplicates do not bypass bounded
network/observer admission. No proportional-to-host-file allocation is allowed.

Host-call activation requires the private admitted resource envelope from the
actual trusted finite supervisor: a positive whole-run deadline of at most
15 seconds and a reaper allowance of at most 1 second, with the existing
termination/reaping enforcement. The executor must fail closed if this
enforcement/binding is unavailable. This is inherited operational CPU/time
containment for this fixed non-programmable adapter, not a CPU scheduler quota
or Browser sandbox. Trusted local regular-file service is a T4
assumption, not a hard real-time or arbitrary-filesystem termination theorem.
The existing bounded supervisor can terminate an unresponsive process; this
is operational containment, not a generated provider failure or proof that a
started call did not occur. No new semantic deadline, lease or hidden retry is
introduced. Quiescent fixture setup/cleanup must not touch user resources.

## Request lifecycle and correspondence

The monotone executor states are `Reserved -> CallStarted -> OutcomeRetained`
or `Reserved -> RejectedBeforeCall`; post-start rejection retains its start
fact. Reserve decision capacity and the call allowance before a one-use
invocation reservation. Bind it to the exact current request, runtime, provider
incarnation and grant. Revalidate grant and admitted resource binding and
atomically commit `CallStarted` immediately before entering the adapter. The
physical call follows in the same serialized action without another
revocation/cancellation interleave. Any host lookup/metadata/open/read used by
the invocation belongs after that commitment; setup's trusted resource
provisioning is not a request effect. Cancellation after `CallStarted` may leave
physical execution unknown, never `RejectedBeforeCall`. Actual returned host
evidence separately establishes that an invocation occurred. A duplicate never invokes
again, including after uncertain reply delivery. No live-context decision is
evicted or reset by reconnect. The finite policy is typed duplicate rejection,
not transparent provider retry, stored-result retry or global exactly-once.

Validate the actual result's type, resource/adapter association, labels and
current request/grant lineage. Retain the validated outcome (or a post-call
rejection preserving the invocation fact) before reply release. The private
carrier uses a distinct effect discriminant and preserves request and result
identity; transport sharing does not turn it into an OwnerRequest/RMW reply.
Requester preflights capacity and current binding before first consume, retains
its decision before pending removal, and rejects duplicate/stale results with
no second consume. An external failure creates no success receipt.

Lost evidence after possible invocation leaves the requester pending with
remote disposition unknown, bound to the original request. It is not a local
terminal provider failure, nonexecution proof or permission to retry. Existing
current-binding admission may accept a later genuine result while the request
is still pending. Supervisor timeout does not change this accepted rule.

Preserve source eligibility -> semantic request -> adapter send -> complete
receive -> effect admission/revalidation -> invocation reservation -> actual
host call -> validated retained outcome -> result send/receive -> current
checked consumption. Separate semantic identity, per-session network occurrence,
provider invocation occurrence and local consumption. Use actual producer
references; a reference hash alone is not cryptographic attestation.

Cut/save admission must include effect pending/reserved and retained ingress
state. In-flight work rejects cut/rebinding; a new session cannot clear this
obligation. A permitted process-local cut retains completed request tombstones,
consumption decisions, spent allowance and retired authority; an unsupported
cut/export path rejects before producing a supposedly complete image. No cut
or fresh connection can make a consumed live-context request eligible again.
No durable restore, distributed transaction or live patch protocol
is introduced. The later I3-3 cut consumer integrates this rule with the other
existing process-local obligations before milestone acceptance.

## Finite acceptance and non-claims

Required evidence: source/check/snapshot/projection preservation and unsupported
entry negatives; source-derived grant issuance and missing/wrong/revoked use
falsifiers with zero calls; actual two-process QUIC success with two different
fixture values in fresh runs; actual missing-resource failure with zero owner
writes; malformed/oversized/type/resource rejection; duplicate request/result,
reconnect, lost reply and post-call revocation retention; bounded capacity;
observer-safe joined provenance and exact effect-order evidence. Local variants
must be labelled local; representative actual I/O success/failure use the
selected real transport and normal checked receiver, not a second fake harness.

The required evidence is finite runtime-monitored, not a Lean/general theorem.
Stop expansion after those consumers and independent review pass. Public
contracts, provider registries, Browser/I5 and durability are not selected.
ADR-0043 accepts the finite I3-3 profile; execution is owner-paused with no
active semantic milestone. I3-4 remains inactive until explicit resume.
