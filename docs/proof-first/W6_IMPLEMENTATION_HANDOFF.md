# W6 Astra implementation handoff — LAB candidate

Status: bounded Astra research/conditions handoff complete as a conditional LAB
candidate. The exact frozen proof, independent advisory scopes, final conditions
and validation are indexed in `W6_CHECK.json`. Whole W6, production assurance,
Q26/Q27,119system requirements and public interfaces are not accepted. Canon
remains normative; Plan250 is owner-paused and W7 is inactive.

## Scope and direct consumer

The owner requested the Astra portion of W6: theory, mechanical proofs,
counterexamples, concrete implementation conditions and remaining tests. The
next consumer is a separately authorized implementation of useful observation
over actual W3/W4/W5 source, runtime and recovery state. The implementation must
preserve private authorized observations while establishing the named conditions.

W5's native-v11 source/binary and complete recovery receipts remain unchanged.
Restored cohort state is quarantined DATA, not renewed transport, prepared ACK,
observation or debugger authority. Existing finite profiles remain distinct.

## Guarantees established in the finite model

| Obligation | Proof source in the frozen W6 cut | Exact scope |
|---|---|---|
| Actual observation fidelity | `W6Fidelity`, `W6Knowledge` | Rendered rows have supplied occurrence/projection witnesses; actual truth requires independent source accuracy. Absence additionally requires a complete authorized closed scope. Inferred records cannot acquire an observed tag through the defined serializer. |
| Passive semantic erasure | `W6PassiveCapture` | Optional capture/query actions preserve domain state and intrinsic emitted history under the same erased input schedule. Arbitrary finite observer interleaving is covered; physical fairness/progress is not. |
| Adaptive authorized observation | `W6Interactive`, `W6OwnerResponse` | Existing checked typed producers derive response equality, including useful private values. The real finite owner-body model includes partial failure, missing data, bounded arithmetic and classified completion. Arbitrary source cursor loops are not proved. |
| Distributed composition | `W6Distributed` | Checked nodes compose under a common strategy over complete prior responses. Node identity/order are retained. Secret-dependent arrival, timeout and response existence are separate obligations. |
| Full-response refinement | `W6Refinement` | Deterministic forward simulation preserves the entire output and transfers local two-run unwinding to adaptive sessions. Equal nondeterministic trace sets alone are insufficient. |
| Current observer release | `W6Release` | Independent current/live facts, authentic purpose and full binding gate one-shot release; restore/retirement refuse and previous knowledge persists. The primitive's supplied projection requires a separate authorization argument. |
| Current policy and every output row | `W6AuthorizedRelease` | The registered interface fixes the label theory, policy identity and clearance. Accepted output rows satisfy this scope. Its composition with budgeted release proves currentness, accounting bounds, actual payload/disclosed-row-history bounds and row authorization together, under explicit funded initial-history and capacity-versus-charge premises. A completed checked producer turn is connected to this entry with derived two-run output equality. The name `RegisteredPolicy` is not evidence of authentic registration. |
| Historical policy and hidden structural changes | `W6PolicyEvolution` | Historical policy/taint is retained. Exact order embedding preserves visibility under translated clearance. Hidden storage-identity attach/detach preserves the defined low relation; public aliases must already be represented. |
| Secret-dependent command selection | `W6DynamicAdmission` | Different commands checked under their hidden activation label preserve low store equality and emit equal visible rows. Checking each body at bottom is insufficient. This finite result does not prove actual source admission, status, timing or general hot-plug. |
| Resource accounting | `W6Resources`, `W6BudgetedRelease` | Declared row/byte/work/temporary accounts; actual payload/disclosed-row-history bounds under explicit funding invariants. Pre-funded checks charge even refused authorization. Actual costs, input admission, ledger storage and physical isolation are separate premises. |
| Active control | `W6ActiveControl` | Current command-specific control authority, semantic validation and one-shot consumption; useful local step and domain DATA rewind preserve current external/spent ledgers. This does not prove general debug confidentiality, resource bounds, distributed pause or whole-world rewind. |
| Falsification | `W6Counterexamples` and frozen `false-controls.json` | Raw IDs/dependencies, latest-before-filter, reason/presence, cached grants, clearance widening, weak refinement, shared quota, raw polling and restoration are distinguished by explicit counterexamples. Counts are inventory, not system coverage. |

`CheckedProducer.captured` is a fresh one-completed-turn witness, not a
persistent-session constructor. One grant's equal gate behavior does not cover
future grants; the spent lists [20] and [] can agree on grant10 and diverge on
grant20. `Release.knowledge` stores released payload values only: refusal and a
successful empty response remain distinct observations. A persistent claim needs
a concrete history-indexed relation over future authority, accounting, complete
responses and policy transitions. This and concrete dynamic-policy/history
transfer remain model-integration gates, in addition to physical implementation.

Detached restore preserves the saved history, not later independent history.
Reattachment must independently acquire current authority and retain or reconcile
live spent, external-outcome, uncertainty and disclosure ledgers. Merely restoring
an old snapshot and setting attached=true can revive a consumed serial; a fresh
grant alone does not repair a rolled-back external ledger.

The coherent dependency cut is not one all-runtime theorem. Adaptive total-producer, finite fallible owner, distributed scheduling and current-funded-release claims retain their stated individual scopes; the actual combined source/OS/network refinement remains open.

The finite typed producer and owner proofs are substantive dependencies, not
assumed final response equality. Generic composition lemmas still expose their
local unwinding, authentic classification, current registry and physical
refinement premises. A supplied Boolean/class string/certificate-shaped record
does not establish the corresponding real-world fact.

The current-funded positive fixture uses independent private A and B inputs:
an observer authorized for A receives its actual varying value and public11.
Changing A changes the response; changing incomparable B does not. This
strengthens the older private-labeled constant examples. Its two output rows now
reserve two rows. The actual row-history theorem concerns stored released
payloads; byte/work costs for arbitrary mathematical Int, the complete request/
refusal transcript, other persistent records and physical heap remain separate
correspondence obligations.

Exact direct-edge fidelity and hidden-insertion secrecy can conflict: a→b and
a→hidden→b have different direct adjacency even after visible IDs are renumbered.
Recorded predecessor, source dependency, causal reachability and display
inference require separate meanings and authorization. Acyclicity is insufficient.
The ordinal-to-payload law applies within one response; cross-response or restored
identity requires an additional authentic, authorized binding.

Narrowing authority cannot reconstruct rows already evicted under wider authority.
The capacity-one A+B-to-A counterexample requires honest coverage/loss handling
or independently retained evidence before a complete historical query is claimed.
Accumulated observer knowledge also survives narrowing. The generic history-indexed
unwinding theorem requires concrete policy-transfer proofs; readable-key inclusion
alone is insufficient. Suppress-all-if-private is rejected for both presence
leakage and loss of useful observation.

## Concrete predecessor mechanisms and gaps

The older root `crates/mir-runtime/src/m8_runtime_observer.rs` takes a supplied
grant. Its `admits` does not independently check the stored epoch; its exporter
selects latest rows before projection and copies raw node/dependency/source
identities. These are source correspondence obligations, not a claim that an
accepted whole-channel confidentiality guarantee has been falsified.

The frozen W4/W5 route is stronger. Its
`sys5_i3_original_source_observation.rs` checks genuine original source, native
occurrences and actual transport custody. `ActualOriginalProjection` is sealed;
registered Report authority is checked under the current M9 floor. The control
path pre-funds statement storage and checks a 4MiB encoded body limit before body
allocation. It does not bound earlier DTO parsing, Vec/String construction or
cloning. These mechanisms must be preserved when extending observation. The
native sender writes Finish before the observation header/body. Finish-write completion with
body-write completion unrecorded is a partial/unknown-delivery case: some body
bytes may already have reached the peer. Even recorded complete body writing
is sender-local evidence, not peer receipt, validation or acceptance.

The current generation guard spans projection, encoding and IO. A slow observer
can therefore make a contending update wait. Each Finish/header write_encoded starts a five-second
deadline shared by prefix and header writes; body writing starts another. Inner
read/write loops reuse their supplied deadline. Neither five nor fifteen seconds
is a whole-report upper bound. This is a
source-derived progress obligation; no measured denial-of-service claim is made.
Hashed occurrence identities remain information-bearing fields; authenticity
and fixed length do not prove confidentiality or unlinkability. The current
digest helper concatenates four fields without lengths/separators: fixed kind
with(request=a,source=bc,admission=d) and(request=ab,source=c,admission=d) has the
same input encoding. Digest equality therefore needs an unambiguous authentic
input-domain argument or independent full-tuple validation before identity,
custody or join use. This helper-level example is not a demonstrated admitted
Report exploit. A future format option would need separate versioning/adoption.

The guard guarantee follows from the inspected Report call paths; the projection
type itself does not carry its lifetime. New paths must use the canonical shared
live floor and synchronize all relevant authority invalidations through it with
complete fact coverage. Independent review did not recursively inspect every
called custody/authority validator or prove all future paths guarded.

The exact source ranges and hashes are in `READ_LEDGER.json`. Detailed conditions
and test designs are the versioned `implementation_obligations` and
`implementation_tests` artifacts referenced by `W6_CHECK.json`.

## Required implementation evidence

| Family | Concrete check before the stronger claim is used |
|---|---|
| T01–T02 projection and metadata | Pair actual producers with hidden insertions/branches and equal authorized inputs; compare complete status/identity/count/order/source/dependency/length output, preserving useful private data. Authenticate labels for existence and metadata as well as values. |
| T03–T04 release and contention | Revoke/change scope after queueing and before drain; test actual lock/linearization/in-flight behavior under a stalled observer and failure after Finish but before header/body. Preserve currentness and explicitly measure domain/lifecycle progress. |
| T05–T07 producer and distributed correspondence | Include private missing/overflow/completion, adaptive two-node requests, delivery selection, wrong source/epoch and actual direct-edge joins. Test edge existence/absence confidentiality as well as fidelity. Every discarded channel needs an explicit authorized abstraction. |
| T08–T09 knowledge and policy | Distinguish actual/inferred/certified absence; require real closed-scope completeness and an empty complete query-relevant projection or independently certified lossless equivalent, not an empty retained page. Private/unknown reasons can share one scheduled unavailable result; per-hidden-event placeholders are forbidden. Retain historical taint and accumulated knowledge; test narrowing after eviction and secret-dependent command activation. Independently authorize policy translation and coverage/loss metadata. |
| T10–T11 accounting and passive behavior | Charge validation, projection, temporary buffers, output, queues and persistent knowledge/spent/control histories before work. Compare observation off/on/drop/slow with domain state and intrinsic effects/witnesses retained. Include per-principal isolation, repeated refusals, sufficient row charge, valid initial account under the current limit, and funded initial/restored disclosed-history. Refusal does not repair invalid initial state; successful empty output can still grow spent history. A bounded nominal account alone is insufficient. |
| T12–T13 active control | Test wrong purpose/target/payload/current epoch/replay/restore and validator rejection; preserve and independently reconcile live external outcome, uncertainty, spent and disclosure histories on rewind/reattachment. Pre-fund control work and preserve ordinary effect authorization. |
| T14–T15 refinement and host channels | Preserve complete wrapper outputs and scheduler coupling across layers. Inventory logs, filenames, crash/save artifacts, stdout/stderr, framing, EOF and timing. Define the honest host and excluded-channel assumptions explicitly. |

The three existing native observer/source baseline cases pass with the retained
W5 binary: absent optional observer index, wrong/noncurrent Report authority and
typed declared expiry. They provide limited observed wall/RSS data. The new
physical campaigns above have **not** run. No actual new observer or debugger
implementation is supplied by this research task.

## Policy choices and implementation order

Q26's allowed channels, response cuts, timing/progress guarantees and Q27's
local/global debugger behavior remain unadopted. Reversible finite prototypes
may proceed under named LAB assumptions; final policy cannot be chosen silently.
In particular, a common response-cut schedule is an explicit conditional model,
not permission to erase actual availability or timing from a stronger claim.

Begin with one actual source/observer profile, authentic source classification
and the complete outward field inventory. Connect current registered scope to
projection and accounting; do not expose the generic supplied-projection
primitive as an authority boundary. Then establish producer/output and
distribution/refinement correspondence, run physical resource/progress tests,
and add independently scoped active control. Re-run affected W5 currentness and
recovery baselines whenever those paths change.

Freeze reliance on an affected claim if a concrete counterexample appears.
Preserve the failed cut; strengthen the missing premise/mechanism or reject the
candidate explicitly. Removing an inconvenient field from the comparison does
not establish secrecy. No whole-W6 or W7 acceptance follows automatically.

## Evidence and reproduction

`W6_CHECK.json` indexes the frozen proof receipt, inherited dependency pins,
native baseline, source obligations, test matrix, review dispositions and final
validation. `REQUIREMENT_DISPOSITIONS.json` preserves all 119 IDs, U/D origins and
approval/adoption flags. `READ_LEDGER.json` separates new reads from hash-bound
reuse. Report2616 is the single task report.

The external workroot contains the unadopted model source and failed precursors.
Use the final frozen replay command recorded in `W6_CHECK.json`; it reconstructs
the copied dependency closure with no reused `.olean`, reruns the owned-axiom
audit and rejects the deliberately false controls. It does not rebuild Rust or
turn a source hash into a physical refinement proof.

Lean's kernel and permitted standard logical axioms establish these mathematical
results. Authentic source/policy acquisition, real synchronization, complete
output correspondence, checked machine arithmetic, physical cost bounds,
scheduler coupling and host isolation remain additional implementation premises.
