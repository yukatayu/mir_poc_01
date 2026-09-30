# W4-C — finite producer and custody inventory

This is LAB implementation correspondence for W4-C's frozen reference, with
W4-D as its direct consumer. It does not adopt the reference, widen Canon,
resume Plan250/I3-4, prove all Rust executions, or guarantee delivery after a
legitimate revocation or process death. The input hashes and searchable call
sites are in the W4_CHECK-linked `producer-inventory-v4/PINS.json` and
`CALLS.json`. The latter is a lexical audit aid, not a compiler call-graph proof.

## Selected boundary and physical ownership

The reference's `Invocation::new` derives an ordered, contiguous full original
owner manifest from checked projected fragments, checks every admitted Core and
argument map, actually reserves retained result Vec capacity for the full finite
activation, constructs one source control, boots a real LocalFabric, and binds
each actual M8 session to that control. `next` selects only that original cursor;
`report` collects the same retained request; `accept` independently checks original
receipt dependencies, current original use, authority floor and exact pending
Committed custody before publishing the one source-consume event. Result storage
for another activation is checked by `invoke_again` before restarting the cursor.

LocalFabric owns its private backend. ST owns a separate boxed M8LocalRuntime for
each admitted locus. OW1 owns one M8LocalRuntime on its worker; commands are
serialized by a zero-capacity synchronous channel. Logical E/C/source roles in
OW1 share that local projection pool. The lower owner and designated counters are
separate fields; the budget does not infer physical separation from a role name.

The live M8 fields are not exported by LocalFabric or Invocation. The worker
snapshot command returns an inert clone: lower source binding loses live executor
custody. Worker shutdown_extract consumes the handle and terminates its loop;
LocalFabric exposes no route to it, and its Drop discards the extracted runtime.
Its separate semantic-kernel caller is a different owning runtime. M8PatchRuntime,
M8 observer wrappers and M10 helpers construct/own their own M8 sessions; their
public noncontextual helpers do not obtain this Invocation's private live backend.
This exclusion depends on the pinned constructors and call sites, not on saying
that a method is merely old, private, test-only, or outside a sample.

## Numeric resources and actual producers

All costs below are upper bounds for admission; counters advance only by the
actual producer calls. A successful source body has at most four lower owner rows
and their four local projections. An already terminal result still owes every
actual unprojected row; terminalization releases only unused future capacity.

| Resource / producer | Actual route and maximum cost | Admission / preservation boundary |
| --- | --- | --- |
| Original result slots | Invocation new / invoke_again; one retained result per original statement | try_reserve_exact before first effect; checked remaining finite manifest capacity before another activation; outcome replacement does not append another slot |
| Source request ID | submit_source_action or local owner prototype: one SYS4 request ID before arm | checked_add1; no new request ID is needed to resume/collect/accept this original |
| Source lower occurrence / trace | source permit → M8 enqueue and same queued service: occurrence1, trace≤4 | pure source_initial_resources_available before arm; raw legacy entry refuses a bound source before allocating; genuine source work uses its own owed capacity |
| Source local projection | project_source_owner_trace from retained actual lower IDs, at most4 | actual projection map discharges once; missing/duplicate/foreign rows cannot clear debt; exact trace cursor addition checked |
| Contextual owner read | backend read_owner_int_with_context → actual M8 owner value and row: local1 | source_foreign_trace_available1 before read; newest exact lower write must agree with actual current value and have an actual projected origin |
| Contextual designated evaluation | backend ST or worker E command: designated occurrence1, lower trace≤3, local≤3 | local debt guard3 plus designated_resources_available3,1 before lower evaluation |
| Contextual designated consumption | ST or worker C command: lower designated trace≤2, local≤2 | local debt guard2 plus designated_resources_available2,0 before consumption |
| Exact designated import | actual checked delivered publication: local1 only when a new contextual import row is required | preeffect local guard1; no numeric designated lower row; Unavailable is distinct from identity/domain rejection |
| Cached non-consuming validation | cloned lower designated engine consumes speculatively (trace≤2), live local validation row1 | lower trace headroom2 checked before cloning/consuming; live local debt guard1; cloned overflow cannot kill worker |
| Relation publication / import | direct live backend publisher and consumer import: local1 each; relation lower trace has no numeric counter | private backend guard1 before either lower call; actual direct relation calls are included, not assumed all staged |
| Relation invalidation / reacquire | ST candidate transition: local≤2 / ≤1 including rejection row | backend guard2 / guard1 before lower call; final whole candidate checks all retained source debt; OW1 relation lifecycle is explicitly BackendIneligible |
| Bootstrap / lease / relation observation | bootstrap and fresh lease installation, publication commit, shadow qualification/projection | no local or lower numeric trace increment; source state/authority/frame compatibility is checked separately |
| M9 authority refresh | genuine lifecycle issuer → backend refresh (OW1 acknowledged) → canonical floor commit | no local or lower trace ID allocation; no source use/result relabeling; historical result and current acknowledgment are separate |
| Checked patch installation | cloned ST backend installs designated-only plans / actual patch lifecycle string | no local numeric trace allocation; full original owner/relation contracts preserved; final source/counter/floor check precedes custody transfer |
| Whole-fabric save | backend save_local_cuts → capture_for_sys4_local_cut(false) | zero M8 local save rows; actual raw save_local_cut(true) is a distinct method with no call from this backend |
| Whole-fabric restore | validates cut, bootstraps fresh candidate, then restore_local_cuts | source-bearing cuts refuse; any RestoreRejected row is in the fresh candidate, never the retained live owner; this is not source recovery |

`advance_anchor_to_frozen`, `note_primary_available_same_lineage`,
`synchronize_entity_presence`, `initialize_patch_declared_int`, noncontextual
M8 evaluate/consume, and raw save/restore are real trace-producing methods.
Their callers in the pinned inventory own separate M10/patch/observer sessions,
or operate on fresh candidates. They are not called by M8ExecutionBackend or by
its worker command loop on this source-bound live session. The raw worker Enqueue/Serve commands exist. The prior inventory's blanket
raw-enqueue exclusion was false: unknown evaluation appended a lower rejection
row before the plan-specific invocation guard, and the wrapper projected it.
Actual direct ST/OW1 controls with four owed slots reproduce this mutation.
The successor now checks source binding before unknown-operation handling;
any raw entry into a bound session refuses without occurrence, trace or queue
mutation. Unbound unknown entry still produces its genuine UnknownEvaluation
row. Original source enqueue alone reaches the private statement-custody branch
through the exact checked source permit. This is a lower-entry guard; the tests
do not establish a reachable ordinary Invocation attack using the raw command.

The exact ownership exclusion also remains: LocalFabric boot constructs each
ST runtime or the sole OW1 worker from its own sealed admitted instance. Its
M8ExecutionBackend methods call the contextual/source commands; they never call
worker.enqueue or M8LocalRuntime.enqueue_owner. SemanticRuntimeKernel's separate
from-admitted constructor allocates its own runtime and moves it into its own
OwnerExecutionBackend; its enqueue_owner_request is the real raw-command caller.
Neither constructor takes a retained source-fabric backend or exports it. An
inert worker snapshot and consuming shutdown do not transfer live source custody.
SourceInitialResources checks the legacy FIFO, but FIFO emptiness alone never
excluded the old unknown-operation producer, because its rejection is not queued.
The same-name methods are not treated as interchangeable.

## Aggregate and qualification counters

Two further u64 counters have a separate disposition. They are not protected
merely by the owner-local budget:

| Counter | Pinned invariant on this retained source fabric | Evidence and limits |
| --- | --- | --- |
| M8LocalTrace observer aggregate next_node_index | Starts at zero; one increment for each new distinct retained qualified row; never retires a published source-history row in the selected path, hence equals aggregate entries.len | Actual ST/OW1 repeated full snapshot reconciliation and two real source bodies preserve count/index equality; clone-and-publish ST positive preserves it |
| m8_locus_trace_sequences[locus] | Equals the count of distinct qualified identities issued with that locus prefix; each maps to a retained actual raw row of the fixed physical session | ST locus has one session; OW1 loci share the same session, and each prefix counts a subset of its raw rows; provisional outcomes use actual returned rows and repeated qualification does not allocate again |

The finite argument uses these exact constructors and mutators. LocalFabric
starts with empty aggregate/registries. The backend returns the complete actual
local trace, never M8LocalTrace.prefix/suffix; the raw traces append monotonically
in this source path. Refresh only qualifies previously unregistered raw IDs.
Direct provisional qualification consumes actual backend outcome observations,
actual relation publish/import rows, or actual non-consuming validation rows.
Relation import first installs its newly observed raw local row before SYS4
qualification; a stored already-qualified shadow is not a new raw producer.

reconcile_fabric_qualified_session contains retain, but its removal condition
never holds here: every previously registered actual raw row is still present in
the next complete snapshot, the physical session name is stable, and source
candidate publication preserves raw trace extension. Existing aggregate rows are
updated at the same index, new rows append, and no source API installs a prefix,
suffix, fresh session, or source-bearing restored trace into this live aggregate.
The source report journal restores aggregate, registry, sequences and graph
views together to the prior unpublished reporting state; later refresh projects
the retained actual backend rows again. It does not roll back committed M8 state.
The aggregate and qualification counters can decrease during this coherent
rollback of unpublished reporting state; they are not globally monotone through
failed transactions. The count/vector and injective-registry invariants are
restored together. Those rolled-back qualification products have no independently
retained published escape in this local owner/callback boundary. A new D process
transfer must recheck that no such identity escapes before publication; this is
not a durable distributed identity-reuse protocol.

The lowest source candidate publisher now separately checks aggregate count,
consecutive indices, distinct IDs, retained row-ID/index prefix, exact qualified
prefix sequences, preservation of existing raw-to-qualified mappings, and that
aggregate row IDs belong to that registry. Two controls showed that unexplained
MAX jumps in either candidate counter were previously accepted. Both now refuse
without changing the live aggregate or source acknowledgment; the authentic
candidate positive remains. These private-counter mutation controls do not prove
an ordinary caller can forge a candidate. They make the publication invariant
explicit. They also do not establish raw-row authenticity by string shape: that
comes from the preceding finite producer and custody argument.

On the checked x86_64 Rust1.94.1 profile, M8LocalTraceEntry is nonzero-sized.
A Vec of these entries cannot have more than isize::MAX bytes, so its retained
row count is strictly below u64::MAX. Thus aggregate count+1 fits before the next
push, and each per-locus count+1 fits because it counts a subset of one retained
raw trace. This is a bound on numeric overflow under the stated ordinary-memory
premise; it is not pre-reserved storage or recovery from allocation failure.
The Vec size rule is from the [Rust standard library documentation](https://doc.rust-lang.org/std/vec/).
A new target wider than64bits, trace pruning, partial snapshot, session replacement,
new raw-ID producer, counter setter, or source restore invalidates this reuse and
requires a new argument or explicit budget guards. Arbitrary M8LocalTrace values
and non-source restore histories are outside this cardinality claim.

## Endpoint production and discharge identity

A remote original's remaining work is request enqueue4/move3/dequeue1, reply
enqueue4/move3/dequeue1, then source acceptance1. A local original uses only the
acceptance1 SYS4 ID. Initial admission includes the actual owner and origin inbox
prefixes. Every actual foreign inbox head ahead of the original owes one dequeue;
new movement/inbox insertion ahead adds one, insertion behind adds zero.

| Allocator | Whole increment | Callers / discharge |
| --- | --- | --- |
| enqueue_outbox | 4 | original source submission and original owner report may discharge4; relation publication gets its own request ID, designated input reply/result delivery retains its distinct foreign request ID |
| enqueue_local_inbox | 6 | cached delivery retry uses a fresh foreign request ID; never discharges original work, and adds a predecessor obligation when appropriate |
| move_envelope_body | 3 | step_transport uses actual retained outbox envelope; exact original movement is recorded once and repeated collection returns that record; foreign arrival adds prefix debt when needed |
| dequeue_locus | 1 | removes the actual inbox head; a genuine original or its genuine FIFO predecessor discharges1, with no arbitrary skip |
| next_mailbox_token / next_endpoint_occurrence | 1 | relation serve, participant leave/reacquire and I3 local-cut admission retain every original obligation; labels never grant discharge |
| source accept | 1 | checks full original current-use/receipt dependencies, then consume_publish checks exact Committed pending custody and publishes cursor/graph/counter together |

`source_owns_endpoint_request` additionally requires live source_control; an inert
candidate with a remembered request string cannot discharge original work.
Request strings are not capabilities. Their meaning here follows the finite
private caller chain: source submission uses its freshly allocated retained ID;
source reply is built from the actual dequeued original and M8 result; movement
and dequeue use actual mailbox records. Relation/foreign source submissions use
the checked monotone request allocator. Future direct callers of these bulk
helpers must re-establish this mapping; adding an arbitrary same-ID caller is a
reopen trigger. Candidate publication cannot substitute the original carrier,
report, FIFO prefix, protected terminal lookup, graph predecessor or actual local
trace prefix. It preserves debt using the inert saved stage before custody moves.

The relation endpoint's older aggregate preflight checks its own maximum cost;
actual constituent allocations still pass the shared source-debt guards. A late
relation refusal may retain its own prior effects/rows according to that route,
but cannot spend the original debt. Source report transactions restore only
unpublished source reporting views on failure; they do not undo committed M8
state or genuine unrelated effects.

## Current authority versus retained history

The genuine M9 lifecycle apply route now accepts a valid successor while a source
is held, as prescribed by ADR-0028 / theory18 §4. It refreshes ST or receives the
OW1 refresh acknowledgment before publishing the successor. The original use,
request, generation labels and result remain unchanged.

The four actual ST/OW1 controls separate: (1) an old unserved ordinary carrier
refuses because owner_lineage_ref includes its issuing generation; (2) queued
original revoked capability gives actual MissingCapability/no write; (3) a
previously committed write remains historical while current acknowledgment
refuses; (4) an unrelated revocation leaves a previously queued M8 use valid, so
its original body executes once, but its old-generation source acknowledgment
still refuses. The earlier test expecting case1 to become a G2 request was an
incorrect positive assumption, retained as failed evidence. No carrier/use is
renewed to make that test pass. The independent existing theorem
OwnerStatementAcknowledgment.current_generation_rejected matches the ack bound;
M9 generation issuance/retranslation and Rust call correspondence remain separate.

## Memory, progress and assurance boundary

Only the retained outcome Vec has an actual capacity reservation. Checked u64
headroom is not a reservation of Vec/BTreeMap/string storage, allocator success,
OS memory, worker lifetime, or network reliability. Other dynamic allocations
remain standard Rust allocations under the selected continuing-process profile;
allocator abort, process death and a permanently disconnected worker do not have
a source recovery guarantee. A retained actual result never licenses reexecution.
The tests distinguish unobserved result/worker loss from declared semantic failure.
No claim of total termination or completion after every allowed interference follows.

A current original can complete conditionally on valid unchanged authority,
healthy original carriers, adequate admitted numeric headroom and result slots,
available ordinary working memory, and finite predecessor traffic being stepped.
The actual mixed scheduler scans finite actual outboxes and then the actual
front of each nonempty inbox. It never flattens inbox contents or skips a
same-mailbox FIFO predecessor. A requested envelope is eligible for an ordinary
typed attempt, including resource/route refusal. An unrelated outbox is selected
only if the shared movement capacity check and the known route/locus conditions
hold; an unrelated inbox head is selected only if its actual one-token dequeue
fits. The latter uses source_dequeue_discharge shared with the lowest dequeue:
only a live original or actual required FIFO prefix discharges that token.
The requested-ID exception grants selection, never capacity discharge or authority.

The earlier outbox-only repair still stranded the original behind an unrelated
E inbox. A genuine initial32-token schedule (before either submission) uses15 to
bring a foreign input receipt to E, admits the original with17, then strands its
actual S head with10 owed tokens. That foreign E dequeue needs11; the original S
dequeue needs10. New ST/OW1 controls reproduce the failure without post-admission
counter mutation. Current selection leaves E retained, completes/acknowledges the
original once, and explicitly collecting E still reports IdentifierExhausted.
Ample-capacity controls preserve genuine foreign11. Same-S-inbox controls require
actual foreign refusal/terminal record with body0 before original service; spare
positives serve both. The higher Invocation::next may internally collect twice,
so the discriminator observes one report call, not an assumed public-step count.

Ten FiniteScan general results now include exact actual-head coverage, selected
head FIFO preservation, enabled-head selection and blocked-outbox/head fallback.
The independent predicate is eligibility for a transition attempt, not eventual
body success. Check equivalence and the actual finite candidate list remain
physical correspondence premises. An original route fault still refuses until
that same route is available; it cannot fabricate completion or permit replay.
The 32-pass budget may require another collection. Finite repeated-collection
controls do not prove fairness or finite completion under arbitrary arrivals.
A completion handle can be successfully delivered once plus one redelivery; failed
collection does not spend that delivery count. Losing both successful handles is
outside this bounded profile. Capacity is admitted per original statement for lower/endpoint pools;
only result slots cover the entire finite activation up front. Later statements
may refuse before effect if their own capacity or authority does not fit.

The ReportReservation and EndpointBudget general Lean results prove numeric
admission/discharge facts and independent checker/Ready equivalence. Actual IDs,
phases, producers, costs, aliases and caller discipline are this bounded physical
correspondence obligation, not facts created by Lean. The completed producer reviews identified concrete outbox/inbox and diagnostic
deltas. The actual-head review resolves the prior two findings and bounded scan
correspondence. Its additional exact-foreign-fault finding is reproduced and
repaired below; final acceptance is recorded separately in W4_CHECK and Report2614. New producers, custody escape, resource sharing,
authority policy or a broader completion promise reopen this inventory.

## Exact foreign transformation retention (2026-09-30 forward repair)

The explicit-request selection exception correctly reaches a typed ordinary
resource refusal, but the earlier transport took an admitted exact transformation
before reserving movement tokens. With initial headroom4, a genuine E.result
submission spends4; its admitted CorruptSourceRef then disappeared on the failed
explicit collection while the envelope stayed in the outbox. An initial21 case
also admits/completes/acknowledges the original once before the explicit foreign
refusal. Both schedules reproduce in ST and OW1 without post-admission mutation.

Selection now clones that descriptor without consuming it. The unchanged ordinary
move is attempted, and only a successful move consumes the matching descriptor.
The existing Retarget terminal branch still consumes the fault and the carrier
with explicit refusal evidence. Exclusive mutable fabric access prevents a
concurrent key/association change during this attempt. The original report guard
still handles unpublished original failures. No resource cost, authority, source
phase, public contract or broader panic/abort recovery is added. Typed failed
moves preserve envelope/endpoints/causality/counter and exact fault; successful
moves actually apply the selected transformation. Separate ordinary-spare
controls produce genuine foreign11, so preservation is not blanket refusal.

Controls red-v2 has4 discriminating failures plus4 positives; green-v1 passes8.
Red-v1 additionally expected a full designated collection with only8 tokens;
that was invalid. The corrected positive observes actual movement and transformed
provenance, without claiming downstream provenance validation at transport. All
failed inputs/logs remain retained. The final full profile receipts, source pins,
and C judgment live in W4_CHECK; this inventory itself grants no acceptance.
