# Passive observation candidate (LAB; scoped source reviews completed)

W1-passive-erasure supplies one observation dependency, under U11 / OB-02/03 /
PT-15/18 / SC-16/17/24 / Q-26. It does not promote requirement proposals, THM-005,
OBL-017/018, official phases or production contracts. Active debug is excluded.

The finite instrumented evaluator takes a domain transition `S → I → S` and an
observer update `S → O → R → O`. Domain inputs include the selected environmental
action. The observer cannot alter S through this mathematical interface, and the
domain transition cannot inspect O. `erasure` proves that removing observer
requests preserves the final domain state for every finite interleaving; applying
it to any prefix gives the same prefix correspondence. `lift_erases` supplies a
domain-only witness for every input list. These are interface-level composition
proofs, not a claim that readonly Rust methods enforce scheduling, input selection,
latency, memory isolation or host purity. An infinite observer-only run can still
starve domain work. Physical fairness and budget enforcement remain obligations.

For a fixed event projection `Event → Option Row`, rejected events do not occupy
visible retention slots. `filter_before_retention` proves the streaming evaluator
agrees with filtering first and retaining visible rows. `retained_bound` bounds
the retained list when its initial list is bounded. `no_invented_rows` establishes
membership origin in the initial rows or a projected input event; it alone is not
an exact causal/source-provenance or duplicate/order theorem. The concrete evaluator
retains newest rows first. This differs from existing M8 export's truncation order;
no refinement or new runtime retention policy is adopted.

`Aligned` separately describes pairs of traces with equal projected visible rows
and arbitrary rejected events on either side. `two_trace_retention` proves their
retained outputs equal. This is conditional trace composition, not proof that an
arbitrary source program produces aligned traces. The independent concrete domain
fragment strengthens non-vacuity: public additions use public state/input; private
updates can read both public and private state and produce private events. By
induction, `concrete_projection` derives its visible arithmetic trace, and
`concrete_two_run` proves retained output equality for arbitrary initial private
values and arbitrary private steps, with the same initial public state, capacity,
empty initial collectors and complete public-addition list. This is a
finite mathematical fragment, not a Mir parser/checker/FFI typing theorem.

Negative controls retain the separation of claims: retaining mixed events before
redaction lets one private event evict a public row at capacity1; global indices
reveal private insertion count; a readonly observer can leak all of S while still
satisfying domain erasure. Nonempty positive outputs demonstrate ordinary public
arithmetic despite differing private data. Fixed guards are tests, not general
proofs. No theorem makes a graph-validation Boolean public merely because its
calculation is sound.

TCB: Lean4.29.1 kernel and standard propext/Quot.sound as printed; the erasure proof
is axiom-free. No Mir-specific axiom, proof admission, SMT or fixed-example decision
is substituted for these general theorems. Abstract functions are explicit
parameters, not authenticated authorities. Projection correctness/currentness,
principal/request binding, redaction of occurrence/dependency metadata, dynamic
revocation/reclassification, prior-delivered data, resource consumption during
projection/retention, source-relative completeness and implementation simulation
remain open. Row count bounds do not imply byte, allocation or time bounds.

Current baseline: the existing four M8 observer tests passed at the pinned HEAD.
They show selected readonly and authority/provenance behavior, not two-run secrecy
or resource isolation. Oracle source review of the base candidate completed; main reproduced its
interactive-cut, initial-row, latest-before-filter and public-step mutation
controls in Lean. The exact retention delta review also completed without a
mathematical counterexample; main checked the reported boundary cases.


`retained_exact` additionally characterizes the entire retained list as the
list `take cap (reverse (filterMap project events) ++ rows)`, under the
initial bound. New rows have reverse processing order, not authenticated causal
order; initial rows keep their supplied order. It exposes order and multiplicity; it does not authenticate
causal/source provenance. `Aligned` is independently defined and implies projected-trace equality; a
converse is not proved or needed here. It is not a separate reason for source
noninterference.
Arbitrary private updates mean pure total functions in this command fragment;
source generation of public command occurrence and arguments remains unverified.

The erasure evaluator and collector have no proved runtime interaction bridge.
Equal completed visible traces do not imply equal snapshots at arbitrary paired
response cuts: inserting a private step before the public step changes the first
command's snapshot from `[7]` to `[]`. Compared request histories, effective
authority, initial retained states and visible prefixes need their own relation.
Policy changes must address already retained rows and future releases separately
from information already delivered. Empty/hidden input preserves existing rows.

Main reproduced an M8 effective-label mismatch using the current Rust libraries
and the real checked fixture/runtime: Public base policy, Private relation
input/override, and the existing Restricted grant yield an accepted real
relation-lineage row labeled Public. The grant cannot cover Private. This is a
trusted M8 setup/API countermodel, not a demonstrated network/M9 exploit; no
production fix or accepted-profile reopening is inferred without its scope and
consumer analysis. Complete input-label binding and consistent checked/authorized/
emitted effective labels are required before claiming refinement.

M8 is a per-kind/latest summary followed by category-order truncation, not merely
an opposite list order. All lossy selection (including latest, grouping and joins)
needs to factor through the authorized projection or prove hidden independence.
A row limit does not bound returned Vec capacity, input work or payload bytes.
Baseline constant-false raw-field helpers do not establish metadata secrecy.

Command-selection controls compare already selected public command lists; they do
not implement a high-dependent source selector. The initial Nat row `[99]` control
shows persistence, not an authenticated history of sensitive data or a revocation
proof. Neither control weakens the open source and current-authority obligations.
