# W4-C supplied observer label preservation — bounded LAB repair

Direct consumer: `M8ObserverRuntime::export_observer_view`; R09, OB01/OB04,
U06/U11, PT15/PT18, SC16. This is a local prerequisite of W4-C, not acceptance of
R09 overall, source confidentiality, W4, or Canon/Plan250 state.

## Contract and alternatives

A policy has a base class and final maps of relation input/override classes.
The existing generic relation row represents one latest relation observation;
it contains no relation name. Candidate A conservatively joins the base and all
supplied map classes on that row, then checks one actual matching grant against
all actual returned rows. Other row kinds retain the base class. Existing typed
policy and per-relation nonweakening checks remain independent prerequisites.

Alternative B rejects any above-base declaration. It rejects legitimate stronger
relation rows even with sufficient clearance, and can reject a public retained
prefix containing no relation row. A preserves these useful positives. Unrelated
constraints may overclassify a relation row; no inferred source identity is used.
The label string remains opaque; its separate security class determines ordering.

## Proof and implementation correspondence

`MirroreaProofFirstObserverLabels.lean` imports the existing `LabelTheory`.
Ten general lemmas prove fold clearance exactness/coverage, independent binding,
existential whole-grant admission exactness, empty-output/no-binding behavior,
retained-row coverage and the absent-relation case. These characterize the row
checker, not all export prerequisites: a zero retention limit accepts an empty
row list in the checker but fails the runtime typed-policy check. `RowsPermitted` is the separate
declarative predicate; `admitRows` is the list checker. No condition is an authority
issuer. The generic model also has an incomparable-label example that rejects
pooling grants even though each row has some individual grant. That generic
checker input is not reachable from the current `rowClass` constructor, whose
outputs are base or one common effective join. It is not an M8 mutation test.

The concrete Public < Restricted < Private instance is proved by exhausting its
finite constructors. This establishes the finite algebra only. The Rust unit test
checks the actual class implementation against the full join table and all triples;
it is not a proof of Rust compilation. All ten general lemmas use Lean4.29.1 kernel
checking, `--trust=0 -j1`. No `sorry`, `admit` or Mir-specific axiom is introduced.
Standard Lean logic axioms, the kernel, compiler and honest correspondence of
Rust inputs with model parameters remain in the trusted computing base.

Rust builds its latest-per-kind/designated observation vector, truncates the
constructed prefix, checks that exact vector and returns it without regeneration.
`selected.take limit` models that prefix, not `Passive.feed` retention. Source
references, occurrence IDs, dependencies and redaction metadata are unchanged.
The base policy is not an upper bound on row classes: forwarding consumers must
retain each row label. Even an empty output still requires a matching base grant.

## Limits and evidence

Supplied labels and already-admitted grants are trusted inputs here. Grant epoch
is not checked by this export. Authentication of source labels, current revocation,
release-policy adequacy, occurrence/error/timing confidentiality, resource isolation
and physical network observer authorization remain separate obligations. The final
map values do not prove policy-update history. Constant-false legacy raw-field
predicates do not establish secrecy; regression checks inspect actual structures
and their debug representation as well as event provenance.

Actual integration tests pass through normal source checking and M8 execution.
They cover sufficient/insufficient clearance, wrong principal/reference grants,
retention, empty rows, diagnostic order, state preservation and exact trace identity.
Both splitting binding from clearance and omitting the output check fail targeted
tests; final restoration passes all twelve. First-entry-only folding and an
extra low row beyond the retention limit also fail targeted mutations. The existing focused M10 observer check also
passes. This is local evidence, not a network or general noninterference proof.

Reproduce with the external-copy command in `samples/lean/README.md`, or compile
Passive, ProducerFlow, GeneralLabels and ObserverLabels in that order with the
pinned Lean version. Then run:

```sh
cargo test --locked --offline -j 1 -p mir-runtime --test m8_runtime_observer -- --test-threads=1
cargo test --locked --offline -j 1 -p mir-runtime --lib proof_first_label_algebra_tests -- --test-threads=1
```

Report2614 and W4_CHECK retain exact hashes, original failures, mutation/restoration
receipts and advisory review dispositions. This file is outside frozen W4-B's
206-module manifest. Final narrow Oracle review is recovered and locally disposed;
no in-contract defect found, two regression gaps closed, no signed acceptance.
