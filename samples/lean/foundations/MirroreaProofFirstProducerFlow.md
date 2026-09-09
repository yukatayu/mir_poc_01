# Ordinary expression producer dependency (task-local LAB)

This extends W1-passive-erasure at PL1 S1/S3, serving U11, OB-02/03,
PT-15/18, SC-16/17/24 and Q-26. It supplies a mathematical producer for the
existing collector, not a Mir source/runtime refinement, authority grant, Canon
THM/OBL update or alpha acceptance.

`Typed` and `infer` describe and compute expression typing for Int/Bool literals,
reads, addition, subtraction and conditional expressions. Their equivalence and
successful typed evaluation are general proofs. `Flows` independently describes
all operand and guard dependencies; `flows_exact` characterizes computational
`rank`. `Safe` is independent of `check`, but its branch rule uses this separately
characterized guard rank. Writes must satisfy value typing, expression flow to
the target and control-context flow. The checker is sound and complete relative
to these syntax-directed rules, not all semantically noninterfering programs.

For an **accepted** command, the same schema and static label map, and two
welltyped initial stores agreeing at low keys, `run_typed` and
`run_noninterference` prove typed final stores, low final-state agreement, and
exact equality of ordered projected write lists including multiplicity. High
branch choices, private values and raw trace lengths may differ. `confinement`
proves high control cannot change low cells or emit low writes. The actual
mathematical evaluator emits writes when they occur; the proof does not assume
input-trace equality or replace execution with expected rows. Raw `run` remains
callable on rejected syntax for counterexamples and has no unconditional safety
claim.

`checked_retention_noninterference` composes that derived write equality with
`Passive.feed`, under a common capacity and initial collector. Equality does not
validate initial rows or bound their size. The exact retention/capacity theorem
additionally needs `initial.length ≤ cap`; fixed-policy visibility of every
retained row needs initial-row visibility too. Neither establishes current
revocation, provenance or already-delivered information protection.

Controls include nonempty public arithmetic amid unequal secret branch traces,
rejection of explicit/control/expression-guard leaks and a wrong value type.
High-minus-itself cancellation is rejected despite the displayed equal outputs,
showing the limits of syntactic completeness. Oracle's occurrence-only control
keeps the final public value unchanged but writes it in one run and skips it in
the other: the checker rejects, and the unchecked visible lists and collectors
differ. Accepted completed traces can differ at the third raw-write cut. An
initial high row survives `skip` at capacity zero, without contradicting the
conditional equality theorem.

TCB: Lean4.29.1 kernel, printed standard `propext`/`Quot.sound`; no Mir-specific
axiom or proof admission. Keys require decidable equality, not a finite universe.
Commands and executions are finite. Values are mathematical integers, stores
are total and labels form a static Nat chain. Arbitrary local theories,
resources, FFI, loops, higher-order values, dynamic address/presence/incarnation,
concurrency, bounded integer failures, current authority, restoration and
interactive/timing observations remain outside this result.

Oracle source review completed on the frozen scratch proof. Main checked its
premise, occurrence, initial-state and response-cut findings and added the
controls above; no general theorem body changed. The first integration check
rejected a module doc placed before imports; moving it after imports produced a
fresh coherent Passive/ProducerFlow trust0 pass. This is recorded as a failed
attempt followed by a checked correction, not an uninterrupted green run.

The source probe uses the actual M7 checker: `atk` has no visibility annotation,
`hp` has `observer_safe`, the expression retains `atk`, and the generated effect
row includes `ObserverPublish` with no residual. Canon spec/02 and spec/08
explicitly make unlisted fields private by default, so `atk` is not merely an
unspecified visibility case. `observer_safe` still grants no implicit public
release. The proposed high/low mapping needs a selected observer, label algebra
and separate current release authority; it is not an automatic Nat-class mapping. A separate
M8 arithmetic probe shows differing success/overflow outcomes when varying a
modeled secret input. Neither is a public delivery or network exploit proof.

A source adapter must preserve the expression tree's reads (the effect row alone
is not a complete operand read-set), key resolution/aliases, types, accepted
command and entry context. Compared source invocations need explicit matched
binding/selection inputs or modeled dependencies. Prove per-run final-state and
actual observation correspondence before deriving two-run claims. Failure
outcomes, effective labels, current authorization, every admission/restore path,
lossy collection and metadata must remain in that boundary. Merely suppressing
publication cannot repair a low cell that already contains secret data.

Commands and exact evidence are in `samples/lean/README.md` and
`docs/proof-first/FOUNDATION_CHECK.json`. The separate fallible-assignment scratch
has a reviewed base and reviewed type/range/collector delta; neither is silently included in this mathematical-total producer contract.
