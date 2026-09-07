---
id: spec/16-i3-owner-admission-budget
status: L1-fixed
maturity: reviewed
depends_on: [root/design-constitution, theory/02-types-effects-failures, theory/04-ordering-and-cuts, theory/05-authority, theory/07-observation, spec/02-surface-grammar, spec/04-core-ir, spec/08-m7-checked-elaboration, spec/09-m8-deterministic-runtime, spec/10-m9-auth-verification, spec/12-sys3-per-locus-projection, spec/13-sys4-in-process-generated-dispatch, arch/09-i3-private-adapter, arch/10-i3-multi-process-runtime, adr/ADR-0041]
summary: ADR-0041が選択するbounded source owner-admission budget、owner直列gate、declared terminal failureと非迂回契約。
open_items: []
---

# 16 — Finite owner admission budget and terminal failure

ADR-0041 selects this bounded extension under PROPOSAL-044.
It is not implementation evidence, a final language/API/wire, or a change to
the previously accepted source profiles. Its direct consumer is I3-3 external
time/failure ordering; no provider operation is introduced here.

## Source and checked meaning

An optional provisional handler clause follows the declared failure row:

```ebnf
OwnerAdmissionBudget ::= "within" "owner_ticks" PositiveInt
```

The finite range is 1 through 65535. The clause is admitted only for a handler
containing exactly one lowerable owner assignment; other shapes reject at the
clause span. This is not a handler-wide multi-owner deadline or transaction.
Without the clause, every previously accepted meaning and generated failure
row remains unchanged. A padded `fails(DeadlineExpired, ...)` name alone does
not activate a time condition.

For an opted-in request, the generated failure row contains the distinct typed
system atom `DeadlineExpired` in addition to all ordinary owner failures.
Omitting it is a static failure-row diagnostic, not a dynamic rejection.
The annotation declares only owner-admission waiting: it is not an end-to-end
network deadline, request-creation deadline or synchronized wall-clock promise.

M6 retains a typed admission condition and its exact source span in the owner
CoreTemplate; M7 consumes that retained template without reconstructing the
condition from an AST or a name. The M7 checked owner Core and contract retain
budget, owner clock coordinate and source reference. Checked-program identity,
projection contract identity and both request/reply carrier contracts include
the condition. Different budgets, absence, owner or source provenance must not
alias. Deployment, sessions, test schedules and failure-name padding cannot
provide missing metadata.

The bare M5 RMW computation remains unchanged. This typed M6/M7 admission
extension is not erased into that bare computation and is not covered by its
old proof/evidence claims. Any M8, M9, SYS4, local CLI, projection or alternate
execution entry that cannot enforce the condition must fail closed before
owner evaluation/mutation. Initial artifact admission is not a reusable permit
to bypass the per-request time gate. A trusted internal bridge may invoke the
unchanged computation only after the exact request's gate authorizes serve;
it must not expose an unbound boolean or caller-created permit.
Annotated execution consumes a private, non-forgeable, non-Clone, one-use
admission permit before semantic handoff. Bind it to the runtime, exact ledger
occurrence, owner, source/Core/contracts/budget and current M9 generation.
Only serialized gate resolution constructs it. An absent optional field,
legacy default or public source action cannot supply it. The invariant applies
at the actual SYS4 owner-serve boundary, SYS5/local workflow/CLI, M10 source and
typed-schedule execution, and direct M8 execution; a Kernel-only or CLI-only
check is insufficient. Every private M9-to-execution translation retains the
condition, including paths that bootstrap M8 directly. Unsupported paths
reject before mutation. A failed or cancelled handoff retains the reservation;
the permit cannot be reacquired or transferred to another request.
An export claiming a complete executable bare-M5 projection must reject an
annotated template or retain its guard explicitly; returning the unguarded
computation as the complete program is not a meaning-preserving conversion.
Inspecting the unchanged computation as a non-executable subcomponent is a
separate operation and does not discharge the admission contract.

## Clock boundary

`OwnerTick` is exactly an unsigned 64-bit integer, range 0 through 2^64-1.
The private clock-domain tag `OwnerAdmissionTicksU64` and owner locus are part
of the checked contract identity; they do not name a global clock. One private
owner-runtime clock-control input supplies monotonically ordered ticks.
An equal-tick input is an accepted no-op. This is an explicit finite host
input, not a Core builtin or an
arbitrary provider API. The request, carrier, requester, certificate, session
and deployment map cannot set or advance it. A clock input neither names a
request nor constructs a terminal failure, grant or semantic state value.
Only the owner gate derives a disposition from it and the checked condition.

Under one owner serialization, validate the first complete frame, exact
source/contract and current authority, reject duplicate or capacity conflicts,
read start tick `s`, and compute `d = checked_add(s, budget)` in that exact
u64 domain. Only then atomically commit `AwaitingAdmission(s,d)`; no clock or
authority update interleaves with those steps. A partial frame, unvalidated
candidate or requester timestamp supplies no start tick. Overflow rejects
before reservation with a
distinct typed clock/preflight diagnostic, never `DeadlineExpired`.
Backward or overflowing clock advance changes neither clock nor request
ledgers. This finite profile supplies no physical clock accuracy, fairness,
lease, global synchronization or restart guarantee.

## Owner-local state and order

The owner serializes clock advance, authority-generation installation,
request staging and resolution. The request-bound ledger is bounded and does
not evict records to admit another request. Staging releases the owner
execution slot: a held request must not prevent another eligible action.

```text
Absent -> AwaitingAdmission(exact request, source condition, s, d)
       -> exactly one retained resolution:
          DeclaredOwnerFailure(DeadlineExpired)
          / RejectedBeforeServe(current authority diagnostic)
          / ServeReserved(one-use admission permit)
              -> actual Served / ordinary owner failure / ambiguous handoff
```

Resolution first revalidates the exact runtime-bound handle and immutable
request/source/contract binding; a wrong handle or binding leaves the genuine
awaiting record untouched. It then revalidates current M9 authority. A genuine
current-authority rejection retains `RejectedBeforeServe`, with its typed local
diagnostic and no future serve; it does not fabricate a transported expiry.
Time cannot replace
capability/witness/membership validation or revive a retired binding.
At that one serialized transition, admission to serve is eligible iff `resolve_tick < d`;
expiry wins iff `resolve_tick >= d`. An expired request performs no owner
evaluation, mutation, publication or successful result production. A rejected
authority/binding check does not fabricate a valid expiry decision.

Commit the appropriate reservation/terminal state before handing off work or
sending an outcome. `ServeReserved` is admission authorization, not proof of
an M8 serve, successful evaluation or mutation. Only actual downstream records
establish those facts, including a failure or ambiguous handoff. Once admission
to serve wins, later clock advance cannot change it to
expiry, cancel it retroactively, erase an ambiguous post-handoff state or
allow another serve. Once expiry wins, failed send, late traffic, explicit
retry or reconnect cannot erase that decision or restart its budget. Existing
ambiguous reserved/handoff-failure behavior remains conservative. "Retained"
means this runtime ledger's lifetime, not durability or crash recovery.

## Typed reply and requester knowledge

The generated owner reply has a typed sum of successful outcome and declared
owner failure. A failure is not a `FabricReceipt`, test `fault_id`, transport
error, or unstructured exception. This extension admits only a gate-produced
`DeadlineExpired` case; adding other failure producers requires their own
checked operation/contract and actual implementation evidence.

The outcome preserves exact source/Core/artifact/operation/owner/request and
reply-edge identity, declared typed failure, and owner decision provenance.
Only the trusted checked owner gate constructs its success-or-expiry decision.
Raw strings, a matching transport peer, observer JSON or an expected result
cannot substitute for that producer. The private codec remains bounded and
fail-closed, including unknown discriminants and malformed outcome bindings;
no public wire/version compatibility is promised.

The requester validates its exact pending request, current admission generation,
reply contract and typed failure containment before consuming the failure.
In one requester-local serialized transition, consumption first retains a
bounded `TerminalFailureConsumed` decision with the exact outcome
occurrence/commitment, then removes the pending entry. It increments
neither successful reply/receipt nor owner-serve/write counts. Replay or stale
failure rejects without changing the stored decision or creating retry rights.
Capacity failure must not delete pending state before a terminal record can be
retained. A genuine G1 outcome presented against G2 is conservatively rejected
under the current exact-binding rule; no historical-authority reuse is selected.

If no validated terminal outcome arrives, the requester remains pending with
remote disposition unknown. Its local wait-budget expiry never establishes
remote nonexecution, cancellation, `DeadlineExpired`, `RouteUnavailable`, or
permission to retry. In particular, serve followed by lost reply followed by
local waiting expiry remains uncertainty, not confirmed unserved.

Observer output is a typed permitted projection of these actual decisions and
source/reason references. It must not expose raw capability/witness payloads,
private source/state, unvalidated records or unrestricted clock-control data.
Source/request identity and network occurrences remain separate.

Admission reservation, expiry decision, failure send/receive/consume,
successful reply/receipt and actual owner serve/write are distinct
record-derived observations. For a validated delivered expiry, the owner
expiry decision is one and its serve/write/success receipt counts are zero;
send, receive and consume require their actual producer records. A lost
outcome or missing/rejected actor evidence is unknown/`None`, not an inferred
zero. Counter projection must reject malformed or contradictory evidence.

## Finite acceptance matrix

| Positive/control | Representative falsifier |
|---|---|
| old sources retain their prior row and behavior | failure-name padding enables a budget, or an old source acquires expiry |
| annotated source retains exact M6/M7/contract identity | annotation lost at M6, changed budget aliases, underdeclared typed failure checks successfully |
| every execution entry enforces or rejects the condition | direct M8/SYS4/local execution bypasses the gate |
| exact one-use permit admits only its bound request | reused, foreign-runtime/request permit reaches M8; wrong handle damages the genuine awaiting request |
| admission reservation followed by actual downstream outcome | `ServeReserved` followed by M8 failure is reported or counted as `Served` |
| budget 1 serves at start tick and expires at start+1 | overflow, backward advance, requester-selected tick, or concurrent tick/revoke permits a false decision |
| live-route expiry transports a declared failure with zero owner mutation | failure is consumed as success, omitted, source-free, wrong-request or wrong-generation evidence |
| serve wins, reply is lost, requester wait expires | local timeout asserts remote nonexecution or rewrites served state |
| expiry persists through lost outcome and explicit duplicate/reconnect | budget restarts, record evicts, or the request later serves |
| exact failure consumes once with a retained terminal decision | replay/tampered failure mutates terminal/pending state or creates retry rights |
| bounded awaiting/terminal capacity and an independent later action | 65th pending record evicts a live record, held request blocks all owner progress, or a reaper supplies the outcome |

Acceptance requires actual source-first two-process selected-transport evidence
for network-dependent rows plus focused producer/binder/capacity falsifiers,
the accepted I2 regression floor and independent review. A bounded model or
litmus may enumerate owner-clock/revoke/serve interleavings; it is not a
general theorem or an extension of existing Lean proofs. Full I3-3 matrix,
I3-4 scenarios, official lifecycle, provider execution, durability, arbitrary
clock/lease models and public compatibility remain separate.
