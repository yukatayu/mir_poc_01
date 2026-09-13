# W3 dynamic composition dependencies (LAB candidate)

W3 is closed as a finite theory/proof/reference candidate. It is not an adopted
production contract, public identity format, alpha or Canon acceptance. W2 remains
closed at its finite scope. The user requested W3 alone, and work stops here;
Plan250/I3-4 and W4+ are not resumed. Report2613 retains the W3 evidence and limits.

## Current definitions and general results

| File suffix | Meaning and independent result |
|---|---|
| `DynamicSupport` | Structural formula mapping; arbitrary same-round and independent Grounded equivalence; actual finite n→n+m append, injective old slots and exact full-domain certificate checking |
| `DynamicGraphs` | One-way downstream extension of each graph kind; acyclic iff old and new induced graphs are acyclic; all these graph proofs use no axioms |
| `DynamicIdentity` | Actual W1 World/Record/Handle/UseRequest mapping; old record associations, code/contract, policy, authority and current context retained; checkHandle/checkUse exactly unchanged |
| `SupportImpact` | Every syntactic all/any branch contributes to dependency reachability; executable affected set agrees with an independent Path predicate; covered input changes preserve Grounded/live outside that conservative set |
| `CurrentChoice` | Current authorization production plus current handle checks; independent least-current-option judgment versus actual search, soundness and relative completeness; bounded monotone cursor and lineage preserved, including pure growth |
| `NamedCatalog` | Instance-scoped names, exact finite lookup and uniqueness; independent named-form elaboration versus executable compilation; full catalog checker correspondence and named Grounded meaning; actual catalog growth preserves old compiled dependencies/live membership |
| `DynamicScopeControls` | Finite reproduced entry-review counterexamples; these are not general proofs |

Every suffix names a `MirroreaProofFirst*.lean` file in this directory. These
definitions extend existing Support/CurrentUse/GraphValidation/TrackedValidation
only through imports; the accepted earlier sources remain unchanged.

The support embedding theorem needs neither finiteness nor injection. Actual
slot insertion proves injection separately and uses bounded exactness at the
different old/new domain sizes. The append keeps old formulas and eligibility
unchanged. Reparenting, retirement, authority changes and code exchange do not
automatically meet those hypotheses.

Graph edges in `DynamicGraphs` point from dependent to prerequisite. There is
no old-to-new dependency edge in its pure insertion case; new-to-old edges need
no extra restriction. Each kind is checked separately. Rooted positive support
may contain cross-kind cycles; per-kind acyclicity neither supplies roots nor
authorization. Acyclicity alone does not make an endpoint usable.

`DynamicIdentity` maps all fields of the modeled old Record, the four request
handles and their associations. Its locator retains instance/slot/identity.
Authority is identical before and after insertion. A previously issued claim may
already mention a future numeric slot; no-issuance does not imply unconditional
denial of every new target. This is not yet transport
of every captured frame, stored reference, pending request or result in a whole
lifecycle machine. A full catalog generation must not silently become every
unaffected handle's validity epoch.

`NamedCatalog.Name` separates an instance namespace from local spelling.
Uniqueness is checked over every slot. Missing references reject compilation;
absent names have no eligible record in its named meaning. `Elaborates` has
structural rules and equality-based reference resolution, without calling the
compiler. Relative completeness requires unique names. `check_exact` relates
the checker to unique naming and existence of elaboration derivations.
The generated snapshot has one entry per represented catalog record, and its
Grounded meaning agrees with the named dependency declarations. Insertion's
old formula mapping is proved from the actual compiler and preserved names,
not supplied as a conclusion about live sets.

The named catalog above remains a separate model input. The source consumer below
now constructs actual definitions/instances and projects their own metadata through
WorldProjection. It does not establish an imported-image or alternate-executor
boundary, a final wire format, an authentic namespace allocator or a global catalog.

## Reference choice component and fallback floor

`CurrentChoice.First` independently requires a currently usable selected request
and unusability of every preceding candidate. The producer searches authority
and checks under the same logical World. Selection gives an actual checked
witness; it issues no claim. A cursor resolves only from its current suffix.
Exhaustion retains the terminal position; later availability does not rewind it.
Pure insertion maps candidates and retains the selected position.

This component alone does not establish the full Canon fallback contract. Static
declared access targets, edge-local same-lineage annotations, capability/contract
degradation, owner-bound semantic occurrences, fresh witness/epoch reacquire and
current result consumption are composed in the maintained-reference extension below.
Presentation sample loss may not invoke semantic resolution or mutate a binding.
No generalized M4 relation or final THM-002 acceptance is claimed here.

## Controls, review and TCB

The current kernel run includes positive new rooted support, rejected rootless
support claims, a lawful checked dead new node, disjoint graph kinds with a cyclic
union, old use after growth, no authority for a new live target, revoked old use,
separate instance namespaces, duplicate/missing names and choice exhaustion.
Entry-review controls reproduce dead-node rank changes, nonidentity slot
permutation with forgotten handle transport, a new self-loop omitted by an old
catalog scan, stale inflationary iteration after retirement, and enabling an
alternative which had no successful branch witness before the change.

Nineteen mutations alter formulas, eligibility, graph edges, identities, code,
authority, principal, impact branches/direction, choice checking/index/cursor,
name uniqueness, reference compilation, disjunction and absent eligibility.
Each must fail in the unchanged proof script at a designated general theorem,
not merely at any syntax error. These checks do not themselves establish that
every mutated theorem statement becomes false. The absent-eligibility mutation
now targets the discriminating missing_eligibility_false property.
The unchanged source baseline must pass first. Failed development drafts and
Lean's error-recovery `sorryAx` outputs are recorded as failures, never proofs.

Oracle `mirrorea-w3-entry-growth` completed a neutral entry/dependency review.
Main checked its advice and reproduced the listed counterexamples. That packet
did not contain these later proof files or the complete W2/source import cone.
It is not their final code review, a kernel run, owner approval or signature.
The later material dependency review mirrorea-w3-growth-kernel found the
non-discriminating mutation and the printed-root audit limitation. Main corrected
both and reproduced its fresh-target, witness-search and module-revision controls.
The narrow correction review is also recovered. Its low-severity control-result
predicate finding was fixed: the expected private declaration and choice dependency
are required, a positive choice-allowed audit passes, and wrong-root/axiom controls
reject. These earlier reviews did not contain the source/lifecycle consumer;
its later validation and reviews are recorded in the integrated sections below.

The trusted computing base is Lean4.29.1's kernel/toolchain. Printed logical
axioms are propext, Classical.choice and Quot.sound where needed; there is no
Mir-specific axiom, `sorry` or `admit`. The harness also uses Lean collectAxioms on all1716 declarations in the eleven
imported proof modules, including private and unused declarations. Only the three
logical axioms pass its allowlist. A private unused standard-choice-dependent
definition is rejected when choice is deliberately excluded; this introduces no
proof hole or custom axiom. The harness, filesystem and invocation
logs are additional execution/evidence trust, not independent attestation.
Finiteness is per represented state, with mathematical Nat and no fixed roster.
Physical atomicity, authentic current-head acquisition, grant issuance, durable
recovery, secrecy, availability and machine-capacity bounds remain separate.

## Reproduction and direct next consumer

Run from repository root, using an existing external directory:

```bash
python3 scripts/proof_first_dynamic_composition_check.py --work-root /tmp
```

The script prints a new small work directory, copies all eleven dependencies,
uses `lean --trust=0`, audits axioms and executes nineteen theorem-targeted
mutations. Exact commands, hashes and results are in its `RESULT.json`;
the pinned invocation is `docs/proof-first/W3_DYNAMIC_CHECK.json`.
This cut is outside the generated Lean manifest; no `.olean` is committed.

The checked source consumer below creates and mutates those records. The maintained
binding extension now adds owner-authorized semantic fallback and explicit fresh
reacquire to the same source/result path; integration validation and review are complete.
The old surface source-patch helper remains report evidence rather than actual
lifecycle mutation or authority. Q18 commit reauthorization and prepared reservation
remain distinct.

## Actual source/lifecycle consumer (validated W3 finite candidate)

`MirroreaProofFirst*.lean` additions in the same foundation directory:

| Suffixes | Actual definitions and general results |
|---|---|
| InstancePrograms, InstanceState, CompositionCore | Checked Int64 definition and retained contract; distinct growing instances, parent/definition graphs and support; seven declarative mutation rules versus the exact elaborator/checker |
| WorldProjection, ManagementEntry | The same instance records generate support and four-handle identities; structural payload/current-cut authorization with separate current actor and no duplicated request application |
| InvocationBoundary, InvocationContract, CompositionMachine, SavedInvocation | Exact saved argument/code/evidence and current result use; actual delivered values satisfy current and retained contracts; pending/used preservation; unrelated growth preserves saved use, selected change/revocation rejects |
| CatalogHistory, CatalogCounterexamples | All selected mutations preserve predecessor refinement and inhabited interfaces; old immutable definitions persist; fully drained admission and unrelated-fork boundaries stay explicit |
| SourceAuthoring, SourceTypes, SourcePreservation, SourceTyping | Named statements execute the actual machine; independent declarative typing versus checker; type/environment and machine preservation |
| SourcePureLowering, SourceFunction | General strict named AST lowering with rejection preserved, including unused overflowing lets; used by the actual source adapter |
| SourceExecution, SourceFrame, SourceWriteHistory, SourceReadNames | Actual successful source writes, last successful rejection prefix, unchanged other bindings, latest value alignment and exactly one matching earlier producer per source-local read |
| SourceCompletion, SourceAllocation, SourceLocators, SourceHistory, SourceHistoryControls | Fresh generated IDs, no late bind loss after accepted typed control/start, retained record bounds and Int64 bounds, actual execution-to-catalog history; injected malformed histories/locators/counters rejected |
| FallbackStatic, FallbackStaticControls | Arbitrary finite flat-chain static floor, exact checker/declarative correspondence, actual parent dominance and late malformed-edge preservation; consumed by the maintained-reference extension below |

Run the source consumer with:

```bash
python3 scripts/proof_first_composition_source_check.py --work-root /tmp
```

Input: `samples/clean-near-end/mirrorea-proof-first-composition/main.mir`.
The runner builds the actual Rust parser, structurally serializes its AST, invokes
Lean's proved strict function compiler, and runs checked statements against the
same growing state. Definitions and instances are not supplied by expected JSON.
The typed provider declarations are the private lifecycle boundary; normal calls,
lets and assignment do not ask the programmer for witnesses, receipts or IDs.
The explicit genesis provides logical authority; source declarations do not issue it.

Additional TCB: the Rust lexer/parser and Python structural AST serialization are
not mechanically refined to source text; their concrete counterexamples/regressions
are tested. Lean's kernel/toolchain, all three standard logical axioms where used,
local honest/current authority head, execution harness/filesystem and sequential
publication are explicit assumptions. There are no Mir-specific axioms or admitted
proofs. The whole imported declaration audit also includes unused/private declarations
and generated proof applications to the actual sample. Declaration counts are audit
coverage, not completion metrics.

Finite contract inputs are explicit profile annotations. The pure compiler proves
value/rejection equivalence, not cost, effect or time equivalence. Source histories
cover transition-local named reads/writes; they do not supply public observations,
all internal support/auth reads, confidentiality or resource noninterference.
Record existence is distinct from current usability. The current actor must remain
admitted; a fully drained realm has no implicit rejoin authority. Logical cut identity
is proved along one reached history, not across arbitrary forks. External authority
head authenticity, physical atomicity, durable images and restart remain later gates.

The static fallback candidate uses equal exported contracts as a reversible private
profile; compatible unequal contracts are outside that profile. Saved invocation is
not a maintained reference. CurrentChoice may produce new evidence when searched,
so it cannot by itself implement a saved selected-option guard. Owner-binding
mutation authority, selected-option permission and actual-argument call permission
remain distinct in the extension below. This component checkpoint is consumed by the
reviewed finite W3 candidate below; it is not Canon or alpha acceptance.

Diagnostic correction: the whole-chain checker combines static outcomes so an
early outsideProfile does not conceal a later malformed/underdeclared edge.
An independent MalformedLinks judgment and checkShape_malformed theorem preserve
a later complete-floor malformed edge. The old masking behavior is rejected at
combine_malformed_right in a targeted proof-script mutation; it is not an additional
general proof. Mixed-error priority remains a private diagnostic convention.

## Maintained reference and admitted source sessions

The additional `Reference*.lean` modules compose the static floor with the same
composition machine and the ordinary-source consumer. They are LAB research
definitions and proofs. W3 is closed as a finite candidate after current-source
kernel validation, discriminating controls, twelve recovered advisory reviews and
source/evidence integration81f82a0b. This does not change Canon acceptance.

| Suffixes after Reference | Meaning / checked general property |
|---|---|
| Access, Selection, AccessHistory | Exact saved selected-option evidence and finite least-current search; continuous actual historical validity, separate from finding fresh evidence |
| Holding, HoldingTrace, Owner | Separate continuing H permission, owner-bound full change and exact producer/checker; same-epoch degradation retains original H origin |
| Store, Coherence, Mutation, Trace | Current retained static floors, canonical bindings, lazy committed fallback, explicit new-epoch reacquire and release; every admitted mutation preserves these invariants |
| Result, Execution, Chronology | Saved complete classified pending entries; protected result use, no reference-to-plain bypass, exact core/event/history correspondence |
| CancellationBoundary, Cancellation, CancellationTrace | Independently current C authorization bound to complete saved pending data; exact terminal occurrence, original and cancel IDs consumed; every future admitted extension rejects repeat result/cancel/use |
| Origins, SourceOrigins | Binding/hold activation points to its actual owner occurrence; source origins correspond one-to-one, in order and kind, to actual engine events |
| SourceData, SourceElaboration, Source, SourceTrace | Independent source typing/elaboration and exact checker; ordinary aliases/calls/assignment execute the same guarded engine; failures retain their committed prefix |
| SourceLocators, SourceProvenance, SourceReads | Existing record bounds, actual previous value producer, and successful elaboration's dependency existence; no inference of existence from an optional absent read alone |
| Authority | Independent finite successor relation/checker, retained revocations/issued claims, monotone generation/member epochs; old invocation result cannot return after an admitted newer head |
| Continuation, Session | Parsed place stays with source body; checked fresh launch, failure repair and completed-block continuation; full source partition, archive retention and saved pending statement agree for all admitted session steps |

The implemented source profile adds `reference`, `reacquire` and `release` at the
typed lifecycle provider boundary. The programmer supplies the relation reader,
ordered targets, adjacent lineage/access declarations and logical deadlines.
Ordinary reads, assignment and calls require no handwritten event, receipt,
witness or retry ID. Main source execution and three separately parsed continuation
cases use the admitted session relation. Adversarial controls also exercise broader
raw source helpers; their unsafe authority-head rollback is explicitly a negative
comparison, not admitted-profile behavior.

Each `ReferenceSession.Rooted` starts with a checked source block and fresh empty
machine. `Step` consists of source tick, cancellation, checked head installation,
real externally authorized management control, failed-block replacement and
completed-block continuation. There is no arbitrary populated-state relaunch or
restore entry. Source construction evidence is generated by source statements,
not by the external control helper. Accepted replacements check against actual
current values and retain the complete old program/completed/stopped/residual
context. The stored parsed place is used by execution; a replacement declared at B
cannot silently execute at A. Successful cancellation emits no source result and
does not make the failed result name available to repair code.

The provisional H profile separates continuing ownership from one-shot mutation
and selected access. Its smallest alternative H2 retains a precisely scoped
acquisition entitlement. C requires a current separate cancellation claim; C2
would retain a nontransferable originating continuation abort right. These are
different authority/revocation policies and are not Canon-adopted or proved
equivalent. Q-18 current reauthorization and prepared reservation stay separate.

The selected finite authority successor keeps the same realm, strictly increases
generation, retains full issued records and revocation tombstones, prohibits claim
ID reinterpretation, and preserves known-issuer/member monotonicity. Authentic
issuer publication, initial namespace ownership and current head agreement are
environment obligations. Checking a successor does not grant authority. Unrelated
head updates conservatively invalidate old invocation/H evidence. Locus-only rejoin
may reuse a still-current member claim with a fresh current envelope; a changed
member incarnation requires a newly matching claim.

Cancellation preserves unrelated pending payloads and current binding records,
not their future usability: advancing the logical serial can expire a lease.
Reacquire changes lineage but does not renew absolute chain deadlines. Retained
bindings continue to constrain compatible changes until authorized release.
An expired repair is demonstrated to retain its failed statement/residual and
produce no fabricated result. No availability, external-effect compensation or
remote nonexecution property is inferred.

Source identity is full decoded content plus UTF8 byte position. Independent byte
offset controls cover CRLF and multibyte comments. Identical contents in different
file instances have the same content identity; no global file-ID scheme is adopted.
Read/write provenance is transition-local and private, not an authorized public
observer, information-flow proof or snapshot/multi-owner transaction semantics.

Reproduce in an existing external directory:

```bash
python3 scripts/proof_first_reference_source_check.py --work-root /tmp
```

The command builds the actual locked/offline Rust parser, freezes the full Lean
import cone and source adapters, checks each module with Lean4.29.1 `--trust=0`,
executes the main source and all source cases, and audits all owned declarations
(including private/unused). The original/repair/addition/expired-repair proof applications are
actual generated consumers; namespace wrapping is recorded, not fabricated trace
composition. Five deliberate weakening controls must fail at designated general
proofs. Producers record generated bytes before execution; the parent verifies
that digest and freezes the same captured bytes. Generated originals, selected
frozen consumers, audit source and manifest file are checked again before success.
Required-name checks reject the weaker component branch's missing Session names;
they do not independently check a required type or declaration owner. The actual
generated, freshly checked declarations supply those concrete witness types.
Thirteen integrity/consumer controls use copies of actual generated files;
they are harness negatives, not additional semantic proofs. Lean/Cargo children
run serially under a 4 GiB address-space cap.

A passing run verifies its captured candidate. To reproduce this exact source cut,
compare the printed work directory's `MANIFEST.json` SHA256 with `frozen_manifest`
in `docs/proof-first/W3_REFERENCE_CHECK.json`; a different candidate can also pass.
Receipt honesty and exclusive work-directory ownership remain assumptions, including
generation-to-first-capture, compiler artifacts and publication/use after each last
verification. Digest checks verify identity at the recorded comparison points;
they are not an atomic filesystem snapshot or signed attestation. Before-capture
or after-last-read writes, and hostile changes restored between checks, are not
excluded by the comparisons alone. The helper controls do not fault-inject a whole
parent run; two consumer negatives reuse actual stored Rust parser reports.

The theory TCB remains Lean's kernel/toolchain with only `propext`,
`Classical.choice`, `Quot.sound` where used; no Mir-specific axiom or proof hole.
The actual parser/adapter and honest harness/filesystem are additional refinement
and execution trust. Fresh roots require externally isolated namespaces; atomic
sequential publication, complete finite retained histories, authentic head/claims,
physical limits, durability, distributed ordering and confidentiality remain
explicit implementation obligations. This does not freeze public ABI/wire or
promote Canon THM/OBL/I3 lifecycle state.
