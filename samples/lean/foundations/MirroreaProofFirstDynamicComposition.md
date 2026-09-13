# W3 dynamic composition dependencies (LAB candidate)

This is an active W3 theory/proof/reference cut, not W3 completion, an adopted
production contract, a public identity format or Canon acceptance. W2 remains
closed at its finite scope. The user requested W3 alone, with a stop after W3;
Plan250/I3-4 and W4+ are not automatically resumed. Report2613 accumulates W3.

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

## Reference choice and remaining fallback floor

`CurrentChoice.First` independently requires a currently usable selected request
and unusability of every preceding candidate. The producer searches authority
and checks under the same logical World. Selection gives an actual checked
witness; it issues no claim. A cursor resolves only from its current suffix.
Exhaustion retains the terminal position; later availability does not rewind it.
Pure insertion maps candidates and retains the selected position.

This component does not establish the full Canon fallback contract. Static
declared access targets, edge-local same-lineage annotations, capability/contract
degradation, owner-bound semantic occurrences, fresh witness/epoch reacquire and
current result consumption must be composed into the same source/lifecycle path.
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
reject. These reviews did not contain the new source/lifecycle consumer; it remains open.

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

The checked source consumer below now creates and mutates those records. The
remaining direct consumer is a maintained binding with owner-authorized semantic
fallback and explicit fresh reacquire, consumed by the same source/result path.
The old surface source-patch helper remains report evidence rather than actual
lifecycle mutation or authority. Q18 commit reauthorization and prepared reservation
remain distinct.

## Actual source/lifecycle consumer (W3 candidate, final review pending)

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
| FallbackStatic, FallbackStaticControls | Arbitrary finite flat-chain static floor, exact checker/declarative correspondence, actual parent dominance and late malformed-edge preservation; dynamic owner binding, invalidation and reacquire remain open |

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
must remain distinct. Final fallback/source integration and material review remain
open; this source checkpoint is not W3 acceptance or completion.

Diagnostic correction: the whole-chain checker combines static outcomes so an
early outsideProfile does not conceal a later malformed/underdeclared edge.
An independent MalformedLinks judgment and checkShape_malformed theorem preserve
a later complete-floor malformed edge. The old masking behavior is rejected at
combine_malformed_right in a targeted proof-script mutation; it is not an additional
general proof. Mixed-error priority remains a private diagnostic convention.
