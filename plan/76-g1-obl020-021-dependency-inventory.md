# plan/76 - G1 OBL-020/021 dependency inventory

## Purpose

This file is LAB repository memory. It inventories the dependencies and
separation boundaries for:

- `OBL-020`: well-formedness preservation of step rules;
- `OBL-021`: elaboration determinism.

Both obligations are required for G1 exit by canon, but this file does not edit
canon, does not create Lean statement files, does not prove either obligation,
does not move `mirrorea_canon/theory/11-metatheory-ledger.md`, and does not
claim G1 / T1 / T2 exit.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB G1 target memory: `plan/71-g1-ordinary-assignment-target.md`
- LAB SCN consequence memory:
  `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- LAB OBL-001 statement memory:
  `plan/73-g1-obl001-lean-statement-inventory.md` and
  `plan/74-g1-obl001-lean-statement-draft.md`
- LAB dependency evidence:
  `plan/75-g1-scn-rhs-dependency-gap-evidence.md`

If this LAB inventory conflicts with canon, canon wins.

## Canon anchors

| Anchor | Reading for this inventory |
|---|---|
| `mirrorea_canon/plan/00-gates.md` | G1 exit requires OBL-001 plus OBL-020/021; this file closes none of them. |
| `mirrorea_canon/plan/01-phases.md` | T2 later expects OBL-020/021/002 proof skeletons; this file is pre-skeleton inventory only. |
| `mirrorea_canon/theory/01-mircore-v0.md` | Defines runtime configuration, well-formedness, step-rule sketches, and states that well-formedness preservation is OBL-020. |
| `mirrorea_canon/theory/03-elaboration.md` | BND-001 requires determinism and states that elaboration determinism is OBL-021. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | OBL-020 target is `MirCore.Step.WF`; OBL-021 target is `MirCore.Elab.Det`; both remain open. |
| `mirrorea_canon/spec/06-conformance.md` | C-static / C-runtime / C-distributed pass claims require SCN-level conformance; this inventory makes no pass claim. |
| `mirrorea_canon/scenarios/SCN-01-sugoroku-roll.md` | Assignment-triggered request / publish / dependency evidence pressures both WF and determinism, but does not prove them. |
| `mirrorea_canon/scenarios/SCN-02-attack.md` | Cross-locus read/write, failure containment, and nested-locus non-authority are G1 pressure cases, not completed proofs. |

## OBL-020 inventory: well-formedness preservation

OBL-020 is a runtime step obligation. Its core shape is:

```text
WF(config) and config --step--> config' implies WF(config')
```

The configuration components are the canon runtime state
`H / Q / S / M / G / W / L / P`: occurrence DAG, request queues, stores,
membership, capability store, witness store, lease/chain store, and patch
lifecycle store. OBL-020 ranges over step rules for the calculus, not only the
ordinary-assignment cases.

The canon well-formedness clauses currently named in `theory/01` are:

| WF clause | Canon reading | G1-relevant pressure |
|---|---|---|
| `WF-H-ACYCLIC` | occurrence DAG `H` is acyclic | generated request / serve / publish order must not create hidden cycles |
| `WF-GRANT-LINEAGE` | every `use(ρ)` has a matching grant ancestor | owner-directed write requests must carry capability obligations; validity is later authority proof |
| `WF-OBS-PUBLISH` | every `observe` has a `publish` ancestor | visible write consequences must keep publish before observe evidence |
| `WF-ACTIVE-KEY` | store entries use active keys at recorded epoch or explicit tombstones | assignment steps must not silently write stale keys |
| `WF-CHAIN-MONOTONE` | chain positions are monotone | mostly G2, but OBL-020 must not break it through unrelated step rules |

### Step-rule families to inventory later

| Step family | WF preservation dependency | Boundary |
|---|---|---|
| `E-WRITE` | owner-local write preserves active-key store discipline and appends an occurrence without creating a cycle | no remote store write claim |
| `E-REQ` | request emission appends/enqueues an owner-directed request with source/owner/failure/authority carriers | no request-serving success claim |
| `E-SERVE` pass | owner validation precedes store mutation/read reply and preserves grant/witness/epoch assumptions | authority soundness remains THM-004 |
| `E-SERVE` fail | explicit failure occurrence is in the request failure row and leaves store unchanged | no generic runtime reject collapse |
| `E-PUB` / `E-OBS` | observe requires publish ancestry and observer authority/redaction conditions | observer-safe noninterference remains THM-005 |
| `E-ADMIT` | membership epoch/incarnation and grant/witness stores update coherently | production identity is out of scope |
| `E-CUT` | cut occurrence preserves acyclicity and later rollback boundary | distributed commit is out of scope |
| `E-DEGRADE` / `E-REACQ` | chain position monotonicity and new lineage discipline are preserved | main proof belongs to G2 |
| `E-PATCH` | activation depends on the admitted frontier and does not mutate on stale frontier | main proof belongs to G7 |

### G1-limited OBL-020 dependency slice

For the ordinary-assignment gate, the minimum useful dependency inventory is:

1. owner-local write preservation of active-key store WF;
2. cross-locus request emission preservation of acyclic request occurrence and
   queue WF;
3. successful serve preservation for owner-directed writes;
4. failure serve preservation with no store mutation;
5. visible publish / observe ancestry preservation for SCN-01;
6. capability/witness carrier presence as a premise, without proving grant
   lineage soundness;
7. source-span and generated-edge metadata treated as evidence, not runtime WF.

This slice is only the G1 pressure view. It must not silently narrow OBL-020
itself, whose canon scope remains well-formedness preservation of the step
rules as a family. This is an inventory for future statement/proof work. It is
not an OBL-020 statement and not a proof skeleton.

## OBL-021 inventory: elaboration determinism

OBL-021 is an elaboration obligation. Its core shape is:

```text
If the same well-scoped input elaborates twice, the result is the same
semantic elaboration result, or the same diagnostic class/span on rejection.
```

The relevant input is the canon unified judgment input:

```text
Σ ; Ψ ; Γ ; Δ ; L ⊢ s
```

and the relevant output is either:

```text
(c, A, μ, ε, φ, C, O, G_e)
```

or a diagnostic. OBL-021 is not runtime scheduling determinism.

### Determinism dependencies

| Dependency | Why it matters | Boundary |
|---|---|---|
| finite declared environments | name / locus / state / role lookup must not depend on traversal accidents | does not freeze parser grammar |
| unique owner resolution | `owner(x)` and indexed-state declarations must be unique or produce deterministic diagnostics | no ambient authority from nested locus blocks |
| fixed current locus and owner hint semantics | `O { ... }` must produce the same owner-directed interpretation for the same input | no authority creation |
| fixed failure-generation function | generated failures for read/write/publish/observe must be stable | diagnostic ABI remains separate |
| fixed visibility lookup | visible field consequences are deterministic for a fixed declaration | no runtime telemetry ABI |
| dependency coverage rule | RHS reads produce the same dependency/read-consequence set | OPEN-014 materialization policy remains parameterized or later-fixed |
| source-span mapping | generated rows point back to deterministic source spans | no final JSON source-map ABI freeze |
| obligation carrier construction | discharged constraints and residual obligations are stable for the same input | no proof of obligation validity |
| rule selection | local/cross read, local/cross write, locus block, handler failure containment, visible publish, and no-ambient-authority cases choose the same elaboration rule for the same input | no runtime execution claim |

### Equality granularity

The future OBL-021 statement should avoid overfitting helper-local identifiers.
Candidate equality levels, all subject to canon confirmation:

| Output family | Equality target |
|---|---|
| Core semantic consequences | structural/semantic equality modulo helper row IDs if canon does not freeze IDs |
| Type / mode / effect rows | exact equality for the canon judgment output `(A, μ, ε)` after any canon-normalization |
| Generated failures | exact finite-set equality |
| Constraints / residual obligations | exact family equality for `C` and `O`, with payload equality only where canon fixes payloads |
| Generated obligations | exact obligation-family equality, with payload equality only where canon fixes payloads |
| Source spans | equality of source origin/span relation, not necessarily final JSON shape |
| Diagnostics | canon diagnostic family and span, not LAB helper string freeze |
| Dependency rows | equality of owner/state/key/field/read-role/write-link semantics, not LAB JSON key names |

This inventory does not decide whether future determinism should use syntactic
equality, normalized equality, alpha-equivalence, definitional equality, or a
canon-specific equivalence relation. If canon later fixes row IDs or
exchange-form JSON, this inventory must be revisited.

## SCN pressure and open gaps

SCN-01 and SCN-02 are pressure cases for the inventory, not conformance passes.

| Pressure | Current reading |
|---|---|
| SCN-01 request / publish / dependency | pressures OBL-020 through request / serve / publish / observe WF preservation and OBL-021 through deterministic generation of those consequences |
| SCN-02 target/self RHS dependencies | pressures OBL-021 dependency-output equality and THM-001 dependency coverage |
| SCN-02 canon wording unevenness | `theory/03` worked shape currently names `atk` dependency, while `scenarios/SCN-02` expects both `player[target].hp` and `player[self].atk`; LAB `plan/75` records both as evidence, but canon wording remains the higher source and may need later clarification |
| conformance | C-static requires all SCN expectations, including negatives, at the claimed level; this inventory does not claim any SCN pass |

## Shared dependencies with OBL-001

| Shared vocabulary | Used by OBL-001 | Used by OBL-020 | Used by OBL-021 |
|---|---|---|---|
| environment / ownership | postcondition premise | step premises for store/request owner | deterministic owner lookup |
| current locus | owner-local vs owner-directed write | request source / serve owner split | deterministic locus-block interpretation |
| assignment target / RHS reads | write and dependency postconditions | runtime write/read consequences after service | deterministic generated consequences |
| failure rows | generated failure containment | fail-step explicit failure WF | deterministic generated failure set |
| authority carriers | obligation representation | `use(ρ)` / witness WF premises | deterministic obligation construction |
| source spans | THM-001 span preservation | evidence only, not runtime WF | deterministic source-span relation |

## Separation from OBL-001 / OBL-002

| Item | Separation rule |
|---|---|
| OBL-001 | States assignment elaboration soundness postconditions for successful elaboration. It may assume enough WF/determinism context but should not prove either OBL-020 or OBL-021. |
| OBL-002 | Proves THM-001 after the statement. It should use OBL-020/021 only if those statements/proofs are actually available; this inventory does not make them available. |
| OBL-020 | Proves preservation over runtime step rules. It should not become a hidden premise that lets OBL-001 claim runtime execution. |
| OBL-021 | Proves uniqueness/stability of elaboration output. It should not be smuggled into OBL-001 as if successful elaboration already implies uniqueness. |
| OBL-004 | No-undeclared-communication is a corollary target after THM-001; it is not part of this dependency inventory. |

## Future Lean statement-shape implications

No Lean file was added in this inventory package. A later package actualized
the OBL-021 statement-shape draft under `samples/lean/lab-statements/obl021/`.
Future LAB-only statement drafts should remain separated by obligation family
unless a file is explicitly about shared vocabulary.

Possible future shapes, with names and file paths intentionally undecided:

| Possible shape | Safe content | Non-claim |
|---|---|---|
| OBL-020 statement-shape draft | abstract `WF`, `Step`, and `PreservesWF` proposition shape | no step-rule proof, no theorem name decision |
| OBL-021 statement-shape draft | actualized later as abstract `Elaborates` / `Rejects` relation and deterministic result/diagnostic proposition | no parser/checker implementation proof, no equality relation or diagnostic ABI decision |
| shared vocabulary draft | opaque carriers for env/locus/result/diagnostic/span | no final MirCore datatype freeze |

Do not update the canon ledger unless a human-approved canon process accepts a
statement/proof status change.

## Snapshot update checklist

This inventory changes repository memory and roadmap status, not runnable sample
status.

| File | Expected status |
|---|---|
| `plan/00-index.md` | update required |
| `plan/90-source-traceability.md` | update required |
| `Documentation.md` | update required |
| `progress.md` | update required, with recent log |
| `tasks.md` | update required |
| `samples_progress.md` | update unnecessary unless a Lean/sample artifact is added |
| `docs/reports/` | new report required |

## Overclaim guardrails

Do not claim:

- OBL-020 completion;
- OBL-021 completion;
- either obligation is `complete` in the gate sense;
- proof skeleton completion;
- Lean statement completion;
- G1 exit;
- T1 or T2 transition;
- C-static, C-runtime, or C-distributed conformance;
- deterministic runtime scheduling;
- request serving or store mutation correctness;
- authority soundness;
- observer-safe noninterference;
- final diagnostic ABI;
- final Core IR JSON/API;
- OPEN-014 resolution.

## Close condition for this package

This package closes when the inventory, `plan/` index, source traceability,
snapshot docs, validators, report, and local validations are synchronized.

Close condition is inventory-only: no canon edit, no Lean file, no proof, no
OBL status movement, no conformance claim, no runtime implementation claim.

## Next safe packages

1. OBL-020 LAB Lean statement-shape draft, only if a statement skeleton is
   useful before proof work.
2. OBL-001 statement refinement, if review finds it should reference OBL-020
   / OBL-021 only as explicit assumptions or adjacent obligations.
3. OBL-021 statement refinement, only if review finds a concrete gap such as
   projection-totality wording or diagnostic-equivalence granularity.
4. E-ROW diagnostic alignment package for canon E-ROW-001 / E-ROW-002 versus
   LAB diagnostics.
