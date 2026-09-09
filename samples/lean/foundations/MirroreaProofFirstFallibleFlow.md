# Fallible assignment dependency (task-local LAB)

One fixed invocation evaluates an Int/Bool expression under common pure total
arithmetic functions returning Option. The actual step either leaves the store
unchanged and emits failed(target), or writes the evaluated value and emits
wrote(target,value). Success/failure and store lemmas characterize those cases.
None covers this evaluator's failure only, not every runtime error category.

Under equal low initial cells and expression rank no greater than the target
label, step_noninterference proves equal low final cells and projected actual
outcomes. Target-label projection is conservative, not a least classification.
The singleton collector theorem uses Passive.feed and common initial rows and
capacity; it does not sanitize or bound arbitrary initial rows.

`signed bits` bounds each arithmetic result to [-2^bits,2^bits); bits=63 is the
selected i64 result interval. Conditional successful evaluation and step typing
are checked separately. Range preservation requires every initial value and
every syntactic literal to be in range. Typed arithmetic can fail; no total
progress or general Rust arithmetic refinement is claimed.

Controls include nonempty low successes and failures, private overflow,
out-of-range literals/operands and intermediate overflow. Omitting flow permits
secret-dependent low outcomes. Checking assignments separately does not make
an aborting sequence safe: private failure can suppress a later public write.
The separate AbortFlow reference investigates that counterexample; it is not
part of this single-assignment contract.

Lean 4.29.1 --trust=0 checked a coherent dependency cut. Printed standard axioms
are propext/Quot.sound, with Classical.choice additionally in bounded_failure;
there are no Mir axioms or admissions. Base and delta Oracle reviews completed,
most recently Q2421976456e3b4e704364a9973cf8f77d1e159f50274db83ebe5b348d6192311.
Main retained the conditional premises and recorded the initial failed range
proof followed by the checked equality-transport correction. Exact commands,
hashes and outputs are in docs/proof-first/FOUNDATION_CHECK.json.

Actual M7/M8 finite controls exercise source, checked expression and owner
execution, but are not general refinement or observer/network evidence.
Canon makes unlisted fields private by default; observer_safe is not an implicit
public release or authority grant. Source/admission binding, presence, authority
failure dependencies, metadata, sequential control, current policy, restore and
resource/time behavior remain separate obligations. GeneralLabels is a separate
mathematical extension, not silently combined with this Nat-label profile.
