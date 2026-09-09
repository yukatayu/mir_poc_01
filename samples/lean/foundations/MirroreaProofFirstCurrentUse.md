# Proof-first current-use candidate (LAB)

This is the active W1-current-use research candidate. It is not accepted Canon,
production behavior, a public identity format, a cryptographic authority scheme,
or resolution of Q-18. The existing Plan250 remains paused. The direct consumers
are typed current handles and, later, every actual execution/release boundary.
Report2611 accumulates evidence and review findings.

## Independent policy meaning and concrete algorithms

`Authorized` is an inductive judgment with leaf, conjunction and explicit left/
right disjunction rules. There is no empty authority branch. `ValidClaim` requires
an exact issued record, nonrevocation, a current issuer epoch, expected issuer/
predicate, matching principal/member/incarnation/instance, allowed action/target
and bounded release label. Issuer trust is an input, not a theorem consequence.

`produce` searches the finite current issued inventory and constructs a branch
witness. `checkWitness` independently validates the submitted branch without
calling the producer or substituting a different branch. General Lean proofs
establish producer acceptance, checker soundness and relative completeness for
this finite policy language and inventory. There is no theorem for arbitrary
delegation, authentication protocols or imported local axioms.

`revalidate` checks exact policy ID/version and complete typed context, then
checks the retained witness under current authority. Context retains principal,
member/locus/module/operation coordinates and incarnations, operation revision,
action, instance, request, tagged arguments, code, contract and generation.
Integer and Boolean arguments are distinct constructors. Checking a proof does
not mutate the authority store.

## Current state and handles

`World n` contains a finite support snapshot and indexed records. Handle checks
compare instance, kind, incarnation and revision with the current record, then
compute support from that World's current eligibility and formulas. The proven
support lemma relates this computation to independent grounded derivations.

`currentContext` reconstructs code, contract, instance, generations and current
record incarnations from World, not from submitted evidence. The selected
reference `checkUse` requires member, execution locus, module and operation
handles; exact member-principal association; and the operation's current
module/locus associations. The locus rejection is an abstract analogue of the copied F0.3 counterexample.
The claim-free pure task has no positive translation into this operation profile;
no repair or refinement of that execution path is established.
It is not a decision that every possible future Mir step has these four handles.

General `checkUse_sound` and `checkUse_complete` connect the executable checker
to independent current handle/support and authority judgments. The latter means
some fresh evidence exists for a lawful use; it does not promise an old evidence
tree remains reusable. Currentness of the supplied World itself, concurrency,
the exact decoder/index mapping and every concrete entry point remain obligations.
A valid old pure support certificate remains valid about old inputs.

The candidate recomputes support at use. The smallest alternative caches a
certificate only under a separately verified binding to the current snapshot.
The alternative needs an additional state/certificate correspondence proof and
an atomic check/use mechanism; this candidate does not establish that mechanism.

## Admission ledger and transition limits

`tryAdmit` records only a current-use admission decision. It performs no body
evaluation, owner write, network send, reply or success receipt. Its unique key
is instance/principal/request; operation and arguments do not create retry rights
under a reused key. This conservative reference key requires a concrete request
allocator and finite capacity/exhaustion policy before implementation use.

The executable finite-action machine includes admit, record retirement and claim
revocation. General proofs show each new admission satisfied current use, the
admission operation leaves World unchanged, all these actions retain historical
decisions, and arbitrary finite runs preserve unique decision keys. Retirement
prevents subsequent use at that locus. A decision before retirement remains a
historical admission, not an unconditional later execution permit.

These are not all-mutator state invariant, actual execution, result release,
patch, migration, connection, restore, or durable at-most-once proofs. Retire and
revoke model an already-authorized control transition; their authorization is
not derived here. They do not select prepared-patch authority reservations or
change existing owner/provider permit semantics.

## Actual controls and trust boundary

The executable controls cover two-layer success, explicit OR success, mixed
subjects, missing/revoked/old-epoch claims, changed policy/generation/instance,
request and tagged-argument changes, wrong handle kind, malformed witness,
tampered claim, retirement, reused index with a fresh incarnation, lawful fresh
current-use validation, changed code and duplicate admission. They are fixed controls;
the quantified results are the separate theorem declarations.

Reproduction, using a small existing work directory:

```bash
lean --trust=0 -o "$WORK/MirroreaProofFirstSupport.olean" samples/lean/foundations/MirroreaProofFirstSupport.lean
LEAN_PATH="$WORK" lean --trust=0 samples/lean/foundations/MirroreaProofFirstCurrentUse.lean
```

The imported support cut is pinned by `docs/proof-first/SUPPORT_CUT.json`.
The actual current-use frozen run and hashes are in
`docs/proof-first/CURRENT_USE_CHECK.json`. Trust includes Lean4.29.1's kernel/
toolchain and standard propext/Quot.sound/Classical.choice where printed. There
is no Mir-specific axiom or admitted proof. Native execution, filesystem and
invoking commands are additional trust for the controls and logs, not independent
attestation. Compiler errors during development were failures, not proof evidence.

Remaining representation obligations include faithful external identity and
incarnation mapping, claim-ID coherence, issuer trust and namespaces,
policy version discipline, source-bound code/contract/label interpretation,
record admission, reference validity, finite numeric/resource bounds and current
authority serialization. Static label comparison does not establish information
flow, observer authorization or two-run secrecy. CurrentUse includes no local
theory contract proof; that separate judgment must be composed without minting
authority. No Python or Rust refinement has been proved here.

## Oracle review and reproduced countermodels

The single read-only `mirrorea-current-use-review` completed successfully. Its
source review found no contradiction of the scoped checker/producer theorems;
it did not execute Lean. Main-agent Lean execution of
`MirroreaProofFirstCurrentUseReview.lean` reproduced the following limits:

- Fresh revised member/locus/module handles accept old evidence; operation
  revision does not. Context equality does not bind omitted revisions.
- Fresh evidence with unchanged claims accepts replacement locus/module/target
  incarnations and changed operation code; member incarnation is claim-scoped.
  Bare home/module coordinates do not preserve an association's lifetime.
- Tagged arguments are not an operation-signature typing judgment.
- Issued-inventory search is not F0.3 supplied-claim search. Candidate target is
  the operation coordinate, whereas F0.3 uses its module. Translation is OPEN.
- A valid nonpreferred OR branch is accepted. Revoking the selected left branch
  rejects old evidence; newly produced right evidence succeeds. An unchanged
  version does not establish immutable unused policy branches.
- Aliased claim IDs alias revocation. Exact-record checking does not prove a
  unique issuer/record namespace.
- Rejoined current-use validation with the retained request number does not add
  another ledger entry; a fresh request number does. Uniqueness of a fabricated
  singleton initial ledger proves no authorization origin.
- Retirement changes support without changing generation. Generation alone is
  insufficient to bind a cached support certificate to current state.

These are fixed countermodels, not general proofs or adopted lifetime policy.
The admission ledger supplies unique decision keys, not linear consumption of
execution permission. History origin, actual caller binding, temporal identity,
all-mutator preservation and source translation remain explicit obligations.
