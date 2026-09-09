# W2 local contract and resource boundary — unreviewed LAB candidate

These ten sources are task-local mathematical research, not production meaning,
formal THM/OBL acceptance, a Plan250 resume, or a verified α candidate. The current
consumer is reusable arithmetic producing the actual positive length consumed by
an exclusive-region allocator. Report2612 records the work and review status.

## Sources and dependency boundary

| Source | Defined objects and actual general evidence |
|---|---|
| `MirroreaProofFirstResourceBoundary.lean` | Nonempty, separated regions; independent Allowed/check equivalence; checked allocate/release/move/split preserve WF; arbitrary finite policy/action schedules preserve WF and consumed-handle absence; exact split cover/disjointness, move geometry, old nonconsumed frame, returned-handle currentness |
| `MirroreaProofFirstLocalContract.lean` | Independent arithmetic Denotes/evaluator correspondence; explicit nonnegative derivations and certificates; inference soundness and relative completeness; checked assumptions at invocation |
| `MirroreaProofFirstContractExport.lean` | Structural binding and scope checks; independent Common/check correspondence; positive-result and exact Int-to-Nat export; two evidence profiles' scoped completeness; same allocator consumes the accepted result and cannot grant denied authority |
| `MirroreaProofFirstPureFunctions.lean` | Annotated Int/Nat/lambda/application/iteration calculus; independent typing/inference equivalence; closure/environment typing; successful-evaluation preservation; separate finite execution derivations and fuel evaluator correspondence |
| `MirroreaProofFirstFunctionContractBridge.lean` | Structural arithmetic lowering into that calculus; typing and actual execution correspondence; closed unary function execution and accepted export value correspondence |
| `MirroreaProofFirstModuleContractBoundary.lean` | CurrentUse plus exact descriptor/code/contract/arity/typed-argument linking; independent Successful/call soundness; both profiles’ relative completeness; accepted actual execution; absence of authority and changed context/stamps reject; later same-proof descriptor uniqueness and immutable bounded catalog preservation/checking |
| `MirroreaProofFirstOwnerAssignment.lean` | Separate live/frozen environments, independent locality/admission/elaboration checkers; frame/type/conditional two-run preservation; exact owner routing and value-preserving label-erasure counterexample; later capture/check correspondence, per-slot captured typing/low equivalence, and single-capture failure-aware continuation preservation |
| `MirroreaProofFirstProfileGuarantees.lean` | Symbolic acceptance implies positivity for every hypothesis-satisfying valuation; actual accepted inputs witness satisfiable hypotheses; checked-value positivity alone is weaker |
| `MirroreaProofFirstHandleValues.lean` | Finite interface binding, independent scoped/evaluation relations, scoped termination, no new references; carried stamps retain current-use and authority checks |
| `MirroreaProofFirstPureHandleFunctions.lean` | Separate pure higher-order handle extension at arbitrary finite size; rechecked type and execution correspondence; general unchanged-handle carriage and actual current invocation |

The import order is ResourceBoundary, LocalContract, ContractExport,
PureFunctions, FunctionContractBridge, then ModuleContractBoundary (also importing
CurrentUse and its Support dependency). OwnerAssignment additionally imports
FallibleFlow/ProducerFlow/Passive; ProfileGuarantees follows ContractExport;
HandleValues follows ModuleContractBoundary; PureHandleFunctions follows
HandleValues. Original candidates and their frozen review cuts are retained. Namespaces remain
`MirroreaProofFirst.ResourceBoundary`, `LocalContract`, `ContractExport`,
`PureFunctions`, `FunctionContractBridge`, `ModuleContractBoundary`,
`OwnerAssignment`, `ProfileGuarantees`, `HandleValues` and `PureHandleFunctions`. Original mirrors adapt import module names; later capture/catalog extensions
are separate unreviewed deltas. No claim depends on filename identity.

## Meaning and nonvacuity

Resource state has monotone fresh IDs/block IDs and a live partial map. A handle
contains its exact region and identity. Move consumes the old ID, issues a fresh
ID for the same interval and changes its holder. Split takes an absolute interior
cut, consumes the original handle and returns the two disjoint subregions.
Release removes a handle. Failure in the reference driver leaves state unchanged.
Current ownership is distinct from the independently supplied operation policy.

The raw helpers are intentionally unsafe entries. Their presence is not an
all-entry invariant proof: `execute` guards them, and its transition language is
the quantified scope. Per-step policy changes are inputs; their authenticity is
not proved. Old-image restoration is explicitly outside the monotone schedule.

Allowed/check equivalence and execution existence admit useful positive actions.
Exact geometry and frame properties supplement safety preservation, which alone
would not rule out losing resources. The returned-handle theorem establishes
currentness at its holder, not permission under a separate policy.

Local arithmetic uses mathematical Int and explicit assumptions. Its derivation
language includes hypotheses, nonnegative literals, sum/product and square.
Certificate completeness is relative to these derivations, not arbitrary true
arithmetic. Missing arguments are rejected by scope checking; evaluator defaults
do not define an author-facing absent-value semantics.

The common export binds actual code, arguments, assumptions, profile/version,
contract version, instance/generation, principal and request. One profile checks
the actual value positive; another checks a certificate for a base whose code is
base+1. Both re-evaluate code and validate assumptions. They share the same export
contract and allocator, but do not establish an arbitrary local-theory loader or
the whole heterogeneous-theory requirement.

Pure closures retain their environment. They contain no effects or linear
continuations. Finite execution completeness says every derivation has a fuel
threshold above which evaluation succeeds. It is not normalization of all typed
terms, a fixed global fuel bound, or an execution-time guarantee. The general
counter-family theorem covers every finite iteration count and integer
initial/delta. The bridge duplicates only pure expressions when lowering square.

Controls include allocate12/split5/move, stale and unauthorized consumption,
invalid splits, old-image resurrection, square(-3)+1 allocating10, input4
allocating4, swapped results/versions/generation/request/arguments rejected,
twice(increment)(40)=42, and iterate7(+3)(0)=21. Fixed `decide`/`rfl` examples are
controls; they are not the general theorems listed above.

## Trust and implementation obligations

Lean4.29.1 `--trust=0` checks the sources. `#print axioms` exposes only standard
`propext`, `Quot.sound`, and where used `Classical.choice`; some lemmas are
axiom-free. No Mir-specific axiom, `sorry`, or `admit` is introduced. The Lean
kernel, standard definitions, source/import integrity and local execution tools
are the checking TCB (trusted computing base). A logged successful command is
not a signed external attestation.

Expected Binding, current State and policy are trusted inputs in this model.
Equality does not authenticate a caller or establish current code provenance,
instance generations, policy or head. ModuleContractBoundary now composes the W1 CurrentUse checker with independent
Linked/Successful propositions: exact module/operation stamps, record code/contract
IDs, typed actual arguments, arity and payload binding. Both guarantee profiles
retain relative completeness, and successful calls execute the actual arithmetic
code in PureFunctions. An independently valid boolean-tagged CurrentUse request
is rejected by the integer contract boundary. CurrentUse alone is not an operation
signature checker. Removing authorization or argument linking is rejected by
mutant checks.

The supplied World and Registry remain trusted. Nominal IDs do not authenticate
the actual descriptor mapping; full module/operation stamps do not cover every
other record revision. The later catalog candidate adds only bounded no-overwrite descriptor insertion
and an explicitly selected catalogCall entry. General finite insertion sequences
preserve old bindings and well-formedness. For a fixed current World/use/registry,
these insertions preserve an already successful call. Any catalog still rejects
a module that is not current in the supplied World; a missing current-code entry
rejects rather than substituting another descriptor. This is not World rollback
protection or authenticated recovery. Current catalog lookup rejects a
different descriptor even with a fresh envelope. Positive registered calls agree
with the original boundary. The retained raw call and original invoke entries still use the weaker path.
Later explicit invokeRegistered entries reuse each handle evaluator and select
catalogCall. General soundness returns actual handle execution, current use,
contract result and current-code catalog binding; stale/no-authority rejection
is preserved. PureHandleFunctions additionally offers invokeClosedRegistered,
which checks a closed expression has handle type before evaluation/invocation.
Its soundness and relative completeness from independent finite execution
derivations are checked. An ill-typed ignored argument can execute in the raw
evaluator but is rejected by this checked entry. No raw entry is relabeled safe.
A concrete raw-call control changes actual code while
keeping nominal IDs and rebuilding its envelope, and succeeds with the supplied
trusted registry. This exposes the missing integrity premise; it is not a
production exploit or authorization bypass theorem. A catalog snapshot supplied
by an attacker or restored from an old head remains outside the protection.
No authenticated registry install/update/retire protocol, caller proof
possession, current-head acquisition, all-entry preservation, serialization,
restore/import or concurrent publication is established. A proof cannot issue auth.

Mathematical regions are not malloc or a Core primitive. Machine bounds,
decoders, time/memory consumption and failure payloads remain open. `None` is a
reference rejection, not a final public error or its disclosure classification.
There is no confidentiality theorem for these exports: secret-derived lengths
and arguments cannot become public merely because a contract is proved.

No existing Mir parser, checked Core, real network, persistence, source-level
configuration creation, or first-class current module handle is refined by these
ten files. OwnerAssignment now models ordinary writes to imported base state
and explicitly captured values, with separate value/type/label environments.
Capture authentication, authorization and metadata provenance remain unmet premises.
The later Capture namespace checks expression type, data flow and capture control
label. Separate per-slot read valuations avoid imposing a shared capture cut.
A single explicit capture followed by assignment includes actual capture failure;
its checker additionally bounds the completion dependency by the target label.
General frame, type and two-run projected-outcome preservation hold for that
composition. Ignoring this completion dependency allows secret-dependent overflow
to suppress an otherwise public constant write; the concrete counterexample and
two rejected checker mutations are retained. Arbitrary event selection, capture
presence/timing, source metadata authenticity and concurrent capture realization
are still open. This does not silently add snapshot semantics to ordinary reads.
Its literalization counterexample preserves values but
changes the generated program with secret input and leaks through a low write;
it does not contradict a fixed-program noninterference theorem.
HandleValues/PureHandleFunctions model carrying interface references, not
source-level lifecycle construction or effectful closure shipping. They do not
extend duplication permission to owned regions or effect continuations. The
separate extended calculus is research, with no claimed refinement of the old
parser or conservative implementation replacement.

Candidate A uses dynamic opaque-handle checks; smallest
viable B adds affine caller checking while retaining currentness/auth. Neither is
adopted as a final source contract here.

## Review and reproduction

The first W2 Oracle invocation failed before submission on the shared-profile
launch lock. A revised packet freshly kernel-checked by the main agent is running as
`mirrorea-w2-integrated-review` using a dedicated attached tab, preserving the
original W1 job. Neither job has a collected result. That revised cut contains
OwnerAssignment and the six earlier W2 modules; ProfileGuarantees, HandleValues
and PureHandleFunctions, plus the later capture, catalog and registered/closed-entry extensions, are outside that frozen cut and cannot inherit its eventual review.
No independent Lean execution or signed reviewer is claimed. All ten sources
remain unreviewed candidates. W1's pending abort/alias/source extensions are not
premises of these new modules.

Use the fresh-copy command in `samples/lean/README.md`; it writes compiler outputs
outside the source tree. Exact commands, source hashes and axiom output are in
`docs/proof-first/W2_INTEGRATED_CHECK.json`, the subsequent mirror record and
`docs/proof-first/W2_EXTENSION_CHECK.json`. The extension record preserves the
later cut separately; fixed controls and rejected mutations are not general
implementation proofs.

The higher-order evaluator now has a general reference-preservation candidate:
ExprWithin/ValueWithin/EnvWithin retain exact interface references in syntax,
closure bodies and captured environments. Every independent finite execution
preserves any such reference bound; the actual evaluator cannot manufacture a
reference outside it. A singleton bound entails exact original stamps; multiple
existing references may still be selected. registered_uses_existing
connects this property to the existing catalog/current-use/contract entry.
This neither authenticates source literals nor grants authority or proves handle
selection confidentiality. Omitting closure code or captured environments loses
the property; typed hidden-reference controls and actual iteration/application
remain nontrivial witnesses. This extension is outside the pending Oracle cut.

A finite fixed-expression counterexample now uses a natural count to select the
current token at zero and a noncurrent-stamp token at one. Both runs typecheck
and preserve the same reference bound, but actual registered calls return42
versus rejection. The noncurrent token has a mismatching revision; no revocation
history is constructed. If the input is secret and call success is publicly
observable, a future information-flow checker must retain this count dependency.
This is an unreviewed conditional obligation, not a deployed disclosure, adopted
observer policy, or general noninterference result.

A separate identity-iteration example returns the same token for every natural
count in the independent finite execution relation (general Lean proof), while
fixed fuel4 completes at count0 and fails at count3. This distinguishes reference
selection from budget-dependent completion. Neither semantic value equality nor
existence of sufficient fuel proves fixed-budget success, timing, resource
noninterference or a public failure policy.


The later unreviewed Normalization namespace supplies a general totality result
for closed, typed expressions of the pure higher-order handle calculus, including
arbitrary finite natural iteration. A type-indexed logical relation is proved
by induction on the independent typing derivation; it is not assumed for every
expression. The fundamental/finite-iteration construction uses propext only.
Existing execution completeness yields one result and a sufficient minimum fuel,
with propext/Classical.choice/Quot.sound. This is big-step termination for this
calculus, not strong normalization of an unspecified reduction relation.
For a checked closed handle expression, a finite selection derivation now exists
without a separate termination premise. At all sufficiently large fuel values,
the registered entry equals the catalog/current-authority/contract call on that
selected handle, including rejection. This creates no authority or successful
call guarantee. An unbound variable never returns; closed typing cannot be
omitted. Existing fixed-budget and secret-count counterexamples remain valid.
No runtime cost bound, fixed-budget completion, timing/resource noninterference,
source refinement, effectful/general recursion or module lifecycle is proved.
Original PureFunctions is unchanged; its analogous scratch proof is supporting
research only. All these additions are outside the pending Oracle packet.


The same extension also proves every independently HasType-typed value computable
using the existing mutual value/environment recursor. EnvTyped then entails the
logical environment relation, so any Typed expression in an EnvTyped environment
has an actual result of the declared type at every sufficiently large fuel.
This discharges the semantic environment premise from structural typing; it does
not add an external closure decoder, environment-admission checker, authority,
serialization rule or information-flow policy. The selected registered entry
remains closed. These general proofs have the same standard axiom boundary.


Later unreviewed ContractExport extensions address two existing W2 consumer
obligations. RequestAllocation passes the current reference State, the whole
existing Binding and the exact allocation Action to an independently supplied
authorization callback. General exactness exposes accepted positive actual length,
the callback decision and the same raw allocation; WF is inherited. The old unary
principal policy cannot distinguish any two positive lengths for that principal.
10-unit acceptance, valid17-unit rejection, changed-request rejection and current
deny controls separate arithmetic evidence from authorization. This callback is
trusted input, not an issuer, authenticated current head, full CurrentUse context,
all-mutator authorization, physical atomicity or a selected production contract.
The other resource mutators retain their original explicit reference boundary.

CheckedArithmetic adds a finite-interval evaluator for existing local-contract
Term inputs/literals/add/mul/square and independent declarative rules. General
exactness and soundness preserve the mathematical value, input scope and range.
Successful evaluation is equivalent to scoped inputs and mathematical bounds at
every syntax node. Accepted contract length plus these bounds yields the actual
checked result Int.ofNat(length). A positive in-range final value alone is
insufficient: an intermediate product may overflow before cancellation. Missing
inputs reject instead of using the mathematical evaluator's default. General
proofs use only propext/Classical.choice/Quot.sound (some subsets); no Mir axioms.
Signed64 controls motivate one finite candidate; no final numeric/failure/privacy
policy or existing-Rust implementation refinement is adopted. Current source
checker accepts square-plus-one and cancellation programs, but the current debug
interpreter panics on their overflowing products. This is retained counterevidence,
not a fixed runtime or an alpha acceptance claim. Both extensions are outside the
pending Oracle packets, and request-bound callback authenticity/actual source
admission and machine failure information flow remain open.

The checked-arithmetic extension additionally proves exact local flow checking and
two-run equality of the complete Option result, including missing-input/overflow
failure, when both inputs agree on all referenced low-classified argument options
under the same supplied labels and bounds. Erasing successful payloads alone is
insufficient: a secret square can still select some()/none. This conditional local
property does not authenticate source labels, release a private result, constrain
resource/time effects, or cover authorization callbacks/allocation geometry.


The later unreviewed BoundedIdentifiers extension retains the same resource
operations and fresh-ID history with finite handle/block identifier ceilings.
Independent per-operation capacity rules and both addition-based and
remaining-capacity checks are exact; the latter checks current bounds before
subtracting. Successful operations preserve spatial WF and finite bounds, and
arbitrary finite changing-policy schedules preserve old-handle absence. Allocation,
move and split reject when fresh identifiers are exhausted; release can still
succeed without resetting history. Given representable ceilings, successful next
counters remain representable. This is not a native-arithmetic/refinement proof,
physical memory budget, chosen ABI, availability or authenticated recovery policy.
Resetting counters after release can restore spatial WF yet resurrect an old
handle; the history restriction cannot be replaced by spatial WF alone.

CheckedAllocation composes actual finite-interval Term evaluation, equality to the
claimed contract result, contract checking, the independent exact-allocation
callback and bounded resource admission. Its general exactness/soundness and
relative completeness expose each obligation; a valid positive mathematical
contract with overflowing intermediates does not allocate. This remains a
nonproduction reference composition, not ordinary-source/parser/IR or network E2E
refinement. Callback authenticity/currentness, allocation cost/geometry information
flow, actual memory failure, physical atomicity and recovery remain open. Neither
unbounded IDs nor recycled finite slots are silently adopted for production.


The later unreviewed CurrentAllocation reference entry keeps current module use
and resource authorization independent. Both see the same supplied World/request,
complete carried stamps, actual descriptor/arguments and exact allocation action;
catalog integrity, checked arithmetic, contract and finite capacity remain checked.
General exactness, soundness and completeness relative to accepted local contracts
and intermediate bounds are kernel-checked; denial at either authorization layer
prevents allocation. This does not issue authority or authenticate the supplied
joint state. Controls accept42, reject revoked/retired use, absent catalog, exhausted
IDs and resource denial; refreshing module evidence for request10 does not reuse
the resource grant for request9. Four skipped/substituted checks fail general
proofs and concrete controls. A repeat of the same admitted request can allocate
fresh handles0 and1: request deduplication is NOT provided by current authorization
or fresh IDs. Atomic coupling of current checks, allocation and durable decision
history is still an implementation obligation, not a resolved Q-18 protocol.
No ordinary-source/E2E, network, save/restore, all-mutator, resource/authorization
information-flow or alpha acceptance follows from this reference composition.


The further unreviewed Once entry couples successful CurrentAllocation effects to
an append-only request-key history in one abstract atomic step. For arbitrary
finite invocation sequences (including changing supplied Worlds, registries and
grants), general proofs preserve resource WF and history uniqueness and exclude
a second effect for an already committed key. Failed attempts leave both parts
unchanged; fresh successful requests can allocate. Keys reuse the existing
CurrentUse (instance, principal, request) candidate; they are not regenerated from
changing argument/code/generation fields. A conflicting reuse is rejected, not
served from a cached result. No public retry/error or request-ID policy is frozen.
The raw CurrentAllocation entry still admits repeats; only this added reference
step includes effect/history coupling. It does not acquire an authentic World or
make two independent physical writes atomic. Resource-only crash survival permits
repetition; history-only survival blocks the missing effect. These two concrete
prefix counterexamples require an actual durable mechanism. History compaction,
finite request namespace/storage exhaustion, concurrent ownership of the decision,
recovery/fresh-import identity and history/failure information flow remain open.
Neither Q-18 reservation versus reauthorization nor physical exactly-once delivery
is selected or proved. Duplicate rejection returns no cached handle or authority.


Reservation is a later unreviewed reference boundary around the same actual
CurrentAllocation allocator. It separates retained identity, actual effect history
and response visibility. A pre-effect stop retains identity with no effect; a
lost response after checked allocation retains both allocation and effect history.
Arbitrary finite attempt sequences preserve resource WF, distinct reserved/effect
keys and effect-to-reservation inclusion. A repeated reserved key rejects without
another allocation, even after other invocations with changed supplied context.
A newly recorded effect entails actual checked allocation. Useful success returns
the real length42 handle; response loss retains that same effect. Three mutations
(ignore reservation, invent a pre-effect fact, erase effect on response loss) fail
general proofs and concrete controls. General proofs use only propext, Quot.sound
and inherited Classical.choice; no new Mir axiom or accepted sorry is introduced.

This is a conservative nonproduction state machine, not a refinement proof for
all SYS5/SYS4/native transitions. Its pre-effect stop is an explicit model input,
not a conclusion available from an arbitrary network error. Smallest viable
alternative: clear only an independently established no-effect/no-outstanding-action
reservation; the model accepts a retry after such a pre-effect stop. Clearing on
mere missing response instead repeats the actual allocation. No public outcome
or Q18 authorization-reservation semantics is adopted. Reservation exhaustion,
compaction, authenticated heads, crash persistence, concurrent ownership,
availability and outcome/history information flow remain open. In particular,
the atomic Once.failure_unchanged theorem does not describe every runtime error.

Full reads of current sys5_i3_process_runtime.rs and sys5_i3_private_quic.rs
confirm distinct real mechanisms: owner reservation before SYS4 handoff; retained
Ambiguous/ServeReserved state after uncertainty; current authority revalidation;
requester attempt reservation before the first awaited frame write; completed
write versus accepted receipt; retained ingress reservation before a cancellable
read. Provider occurrence references avoid raw-value-derived frame hashes, unlike
the ordinary restricted carrier profile. These are code facts, not general privacy
or durable-recovery proofs. The ordinary process start initializes pending,
tombstone and effect-occurrence maps empty: its image bootstrap must not be counted
as same-instance post-effect recovery. Runtime-only retry2 and ledger1 tests pass
at this cut; no new actual QUIC or persistence test was executed in this increment.

Allocation observation is a further UNREVIEWED reference consequence. Shared
checked allocation exposes a private prior allocation through the next public
handle ID (0 versus 1), and through success versus failure at capacity one even
when handle payloads are erased. Two independent resource/reservation stores
admit general low-state and complete low-outcome-trace projection proofs for
arbitrary finite interleavings, conditional on equal initial low stores and equal
low input subsequences. Actual outcomes include allocation failure and identifiers;
this is not merely equality of erased returned values. Current low-handle lookup
is likewise independent of the high store. Resource WF is preserved in both stores.

The conditional candidate does not supply source classification or authority.
Scoped addresses separate mathematical address domains, but a concrete control
executes the SAME unscoped request once in each pool. Caller-chosen classification
therefore cannot establish global request uniqueness or authorize a namespace.
Binding source scope, request identity and grants remains an explicit consumer
obligation; this candidate is not an adopted allocator or Q18 solution. Merely
hiding IDs is an inadequate alternative because the capacity failure still leaks.
An opaque-ID design with independently reserved low capacity is a possible smallest
alternative, not yet established or selected.

The two-run theorem assumes equal low inputs including current worlds, grants,
arguments and resource limits. Source secret-dependent presence, globally changed
authority generations, physical resource interference, timing, scheduling cost,
divergence and active-debug authorization are not proved. The reference low trace
omits high actions; an actual observer must justify that release policy separately.
Three final mutations share the low state, reveal high action presence, or erase
lookup scope: each breaks a general proof and a concrete control. Final scratch
and fresh fifteen-module actual-source mirror pass Lean --trust=0; standard axioms
are propext, Quot.sound and inherited Classical.choice, with no accepted sorry or
new Mir axiom. No production increment or alpha/readiness claim follows.

A follow-on finite F0.3 recovery control (95db2f, copied original model, no source
edits) separates three cases. A newly constructed same-instance Kernel rejects a
checkpoint as UntrustedCheckpoint because trusted snapshots/head have no restart
loader. Injecting the latest retained trusted state restores stock7 and rejects
duplicate serve; a subsequent consume after revocation is ReleaseDenied while
stock remains7. Injecting BOTH the old data and old trusted head instead admits
the old pending request and executes it again (stock10→7). This is the stated
trusted-head assumption's counterexample, not an exploit against an honest retained
head, a physical crash test, or proof that fresh import is same-instance recovery.
A hash chain alone does not establish that its supplied head is current. No
current authority is derived from a historical successful effect. Reproduction
script, exact output and copied Python-source hashes are retained in
W2_EXTENSION_CHECK.recovery_anchor_controls; work pointer RECOVERY_ANCHOR_CONTROLS.
