# Proof-first finite support foundation (LAB)

This is task-authorized W1 research evidence for current handles and dynamic
composition. It changes no Canon theorem status, Plan250 milestone, authority
rule or production contract. The accumulating record is
[Report2611](../../../docs/reports/2611-mirrorea-proof-first-w1-foundations.md).

## Meaning and executable boundary

`MirroreaProofFirst.Support.Proof` is an inductive finite derivation independent
of the closure algorithm: a referenced record requires its eligibility and a
derivation of its support formula. `Grounded` also requires the subject's own
eligibility. Formulas contain top, bottom, reference, conjunction and disjunction;
list conjunction/disjunction are related to binary folds by general proofs.
Empty conjunction is true; empty disjunction is false.

`rounds` starts empty and adds eligible records whose formulas hold in the
previous round. `derive` filters a supplied enumeration after its length in
rounds. The quantified finite results require `complete : ∀ k, k ∈ nodes`.
Thus every typed key, including every reference, belongs to the enumeration.
Duplicate enumeration entries only loosen the bound; the theorem asserts
membership, not uniqueness of list representation. A decoder must separately
establish finite identity, reference validity and canonical representation.
Duplicates do remain in the raw returned list; its entries are not occurrences.

The raw checker can accept a forged certificate if its supplied enumeration is
incomplete. The consumer-facing LAB `Snapshot n` instead uses `Fin n` keys and
internally constructs `List.finRange n`; callers supply no enumeration.
`snapshot_live_exact`, `snapshot_certificate_accepted` and
`snapshot_checked_sound` discharge coverage internally for this typed boundary.
An external decoder must still provide a faithful identity/incarnation mapping;
finite indices do not establish that mapping or authorize a handle.

`checkMembership` independently checks eligibility, strictly earlier support,
and saturation. It does not compare with a recomputed answer. `checkCanonical`
adds exact formula-height equations. A membership certificate need not have
minimal ranks; a canonical certificate must have the unique earliest live ranks.
Ranks of inactive records are not constrained.

## Mechanically checked claims and direct obligations

| Lean declarations | Checked claim | Obligation before implementation consumption |
|---|---|---|
| `eval_correct`, `allOf_correct`, `anyOf_correct` | Boolean formula evaluation agrees with declarative satisfaction | Preserve typed formula decoding and fold semantics; reject unsupported inputs explicitly |
| `grounded_least`, `rounds_iff_derivation` | Finite derivations coincide with eventual positive closure, which is least closed | Do not adopt a larger self-supporting fixed point |
| `stable_exists`, `bounded_rounds_exact`, `derive_exact` | Finite complete enumeration gives exact membership within its length in rounds | Account for actual formula traversal, allocation and input limits; this is not a wall-clock or memory bound |
| `checker_exact`, `checked_membership_exact` | Actual membership checker is equivalent to rank rules and accepts only exact grounded membership | Freeze forms and eligibility during checking; preserve all three checks |
| `canonical_generated_accepted`, `canonical_checker_sound` | Actual producer passes the canonical checker, which implies exact membership and earliest live ranks | Canonical ranks are internal certificate data, with no public release decision |
| `retirement_mono` | Shrinking eligibility with fixed formulas cannot add grounded members | Recompute or prove incremental maintenance; old survivor ranks may change |
| `normalization_idempotent` | Replacing eligibility by its grounded subset does not change grounding | This is a pure normalization theorem, not a state-transition invariant |
| `no_root_no_ground` | A reference-only graph has no grounded record | A rootless cycle cannot survive merely because its records existed before |
| `low_rounds_independent`, `low_grounded_independent` | Equal low formulas/eligibility yield equal low rounds/membership if every potential low reference is low | Check all branches, not only the selected branch. Prove observation, metadata, resource and two-run execution properties separately |

The results quantify over formulas and inputs; executable `#guard` examples and
the differential enumeration are controls, not general proofs. Generated
certificate acceptance and lawful roots prevent a safety-only all-reject reading.
The inductive derivation is not an axiom asserting the checker conclusion.

## Reproduction and trusted computing base

```bash
lean --trust=0 samples/lean/foundations/MirroreaProofFirstSupport.lean
python3 scripts/proof_first_support_check.py --work-root /existing/work/directory
```

The second command makes a fresh small work directory, copies proof/reference
inputs and leaves the handoff unchanged. It checks the exact Lean version and
the proof, driver, runner and reference hashes against
`docs/proof-first/SUPPORT_CUT.json`. The proof, driver and reference execute
from verified copies. The invoking runner is separately trusted: hashing and
copying the canonical runner path does not attest the active entry point, and
the copied runner is not executed. An alternate selected manifest can describe
a different valid cut. Consumers of this reviewed cut must compare the recorded
manifest digest with `c7ceb771cbf286db81aebae3251693bcaa3691543cf3814866fedb33ff7cd89f`;
a PASS alone does not select it. The manifest is neither a signature nor
acceptance. Exact version output is checked under the trusted-toolchain premise.
It compiles Lean and compares actual
Lean output with the hash-pinned F0.3 Python implementation for 2,744 cases. It
also runs root-withdrawal, stale-rank, low-rank-leak, four-node eligibility and
depth-limit countermodels. It rejects Python `-O`, which would remove assertions.
The differential driver is executed through this command with its compiled
import; it is not a standalone foundation theorem file.

Checked toolchain: Lean4.29.1, commit
`f72c35b3f637c8c6571d353742168ab66cc22c00`, kernel checking with `--trust=0`.
The file prints axiom dependencies for its major claims. The checked cut uses
only standard `propext`, `Quot.sound`, and for some results `Classical.choice`;
some results use none. There is no Mir-specific axiom, `sorry` or `admit`.
Lean's kernel/toolchain is trusted for theorem checking. Native execution,
Python and operating environment additionally enter the finite-comparison TCB.
The comparison does not prove general Python or Rust refinement.

## Counterexamples and unresolved connection

Deleting only locally unsupported records after a root disappears can leave a
rootless residual cycle. A surviving record's earliest rank can increase after
withdrawal. A secret shortcut can change an exported rank while leaving low
membership unchanged. None is repaired by a claim about a separate per-kind DAG.

The first rooted top-OR cycle is logically redundant. A four-node uniform
eligibility example distinguishes cyclic support from an acyclic support graph
under fixed identities and own-eligibility gates; it does not establish that
ordinary Mir source requires cyclic support. A justification DAG plus saturation
is the smaller certificate alternative; source necessity remains a consumer issue.

F0.3 additionally permits a started pure task to continue after its locus is
retired when its module/member remain independently live. This executable
counterexample is recorded in Report2611. Support correctness alone says nothing
about which records each execution step must currently require. Current-use,
policy revocation, exact content/context binding, recovery and all mutator paths
need separate proofs before the next production increment. No authority is
created by a proof of membership.

Snapshot normalization is not permission to erase eligibility. With `a = ref b`
and `b = top`, initially eligible `a` is unsupported. Later eligibility of `b`
grounds both only if `a`'s original eligibility was retained. Replacing eligibility
permanently by the initial empty live set loses this revival. The executable
Lean controls include this case, incomplete enumeration and duplicate output.

The final boundary Oracle review found no snapshot-theorem counterexample and
permitted scoped use in the next LAB proof. It reviewed supplied proof logs and
execution summaries, not independent Lean execution or complete negative-test
transcripts. Main-agent local execution remains the source of those results.
The recorded finite differential compares generated membership/live ranks; it
does not enumerate arbitrary certificates or establish checker refinement.
