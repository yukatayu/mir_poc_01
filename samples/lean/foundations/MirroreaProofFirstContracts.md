# W2 local contract and resource boundary — unreviewed LAB candidate

These six sources are task-local mathematical research, not production meaning,
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

| `MirroreaProofFirstModuleContractBoundary.lean` | CurrentUse plus exact descriptor/code/contract/arity/typed-argument linking; independent Successful/call soundness; both profiles’ relative completeness; accepted actual execution; absence of authority and changed context/stamps reject |

The import order is ResourceBoundary, LocalContract, ContractExport,
PureFunctions, FunctionContractBridge, then ModuleContractBoundary (also importing
CurrentUse and its Support dependency). Namespaces remain
`MirroreaProofFirst.ResourceBoundary`, `LocalContract`, `ContractExport`,
`PureFunctions`, `FunctionContractBridge`, and `ModuleContractBoundary`. The repo mirror changes only import
module names from the frozen scratch cut; no claim depends on filename identity.

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
other record revision. No registry install/update/retire protocol, caller proof
possession, current-head acquisition, all-entry preservation, serialization,
restore/import or concurrent publication is established. A proof cannot issue auth.

Mathematical regions are not malloc or a Core primitive. Machine bounds,
decoders, time/memory consumption and failure payloads remain open. `None` is a
reference rejection, not a final public error or its disclosure classification.
There is no confidentiality theorem for these exports: secret-derived lengths
and arguments cannot become public merely because a contract is proved.

No existing Mir parser, checked Core, real network, persistence, source-level
configuration creation, or first-class current module handle is refined by these
six files. Ordinary parent assignment and explicit other-owner snapshots remain
separate W2 consumers. Candidate A uses dynamic opaque-handle checks; smallest
viable B adds affine caller checking while retaining currentness/auth. Neither is
adopted as a final source contract here.

## Review and reproduction

The main launched read-only Oracle `mirrorea-w2-resource-contract` once with the
frozen five-file cut. The invocation failed before submission because the shared
profile lock remained held by the W1 job; no answer was received and no retry has
been sent. No independent Lean execution or signed reviewer is claimed. The earlier W1 abort/alias/source consultation is
separate and its pending extensions are not premises of this candidate. The sixth
module boundary was added afterwards and was not in that frozen five-file packet.

Use the fresh-copy command in `samples/lean/README.md`; it writes compiler outputs
outside the source tree. Exact commands, source hashes and axiom output are in
`docs/proof-first/W2_INTEGRATED_CHECK.json` and the subsequent mirror record.
