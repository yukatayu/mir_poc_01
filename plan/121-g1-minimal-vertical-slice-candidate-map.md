# plan/121 - G1 minimal vertical slice candidate map

## Purpose

This file is LAB repository memory.

It maps the first minimal source-first vertical slice candidate after
`plan/120-repo-triage-recut-matrix.md`. The slice feeds G1 ordinary assignment
work by selecting only the source / static elaboration / statement-boundary
evidence that can pressure THM-001 without widening into runtime, product,
transport, projection, or hot-plug completion.

This file does not edit canon, does not promote a package, does not close G0 or
G1, does not move proof-obligation status, does not claim conformance, and does
not change runnable sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB planning and evidence: legacy `specs/`, `plan/`, samples, helpers,
  reports, Rust code, and Lean statement drafts outside `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If any LAB evidence conflicts with canon, canon wins. The labels in this file
are scheduling labels, not truth labels.

## Current phase reading

The overall plan is still in `Macro 0 / T0-G0 rebaseline` when judged by the
canon phase plan. Existing Surface / Full System / Product Alpha artifacts reach
ahead as LAB evidence floors, but they do not move the canon implementation
state out of T0.

Within that current rebaseline, the nearest next theoretical line remains G1
ordinary assignment. This file narrows the next source-first vertical slice
candidate so future work can cite the right evidence without reopening broad
runtime or product claims.

## Candidate label

Use this temporary label for planning only:

```text
G1-MVS-ASSIGNMENT-STATIC
```

Descriptive name:

```text
source-first static assignment spine
```

Meaning:

- `G1`: the slice pressures ordinary assignment.
- `MVS`: minimal vertical slice, not final product scope.
- `ASSIGNMENT`: simple Surface assignment before compound assignment.
- `STATIC`: parse / check / elaborate / statement-boundary evidence only.

Do not use the label as a sample status, conformance status, package name, gate
status, or public API name.

## Vertical path

The slice is a static source-first consequence map:

```text
.mir source fixture
-> Surface parser / AST evidence
-> indexed-state and role-context static check evidence
-> Surface-to-Core elaboration evidence
-> explicit static consequence inventory
-> LAB-only Lean statement-shape / guard references
```

It is not an end-to-end runtime slice, conformance slice, release slice,
implementation slice, or product slice.

## Slice spine

| Layer | Include in the candidate | Evidence class | Stop line |
|---|---|---|---|
| Source syntax | Canon `S { ... }` locus-block syntax, simple assignment, and explicit rejection of old bracket place syntax as legacy pressure. | `keep-core-idea` / `useful-floor` from Surface syntax rows | No final public grammar freeze and no `S[ ... ]` sugar revival. |
| Indexed state | S-owned indexed state, owner/keyspace separation, key-not-authority and nested-locus non-authority pressure. | `keep-core-idea` / `useful-floor` from indexed-state rows | No G3 authority theorem, production membership, or runtime stale-lifecycle claim. |
| Owner-directed write | Cross-locus assignment lowers to explicit owner-directed request rather than direct remote store. | `keep-core-idea` / `useful-floor` from `ELAB-02`, `ELAB-09`, `ELAB-11`, `ELAB-12` | No runtime request serving, transport delivery, or distributed store mutation claim. |
| RHS dependency | RHS indexed reads are recorded as dependency rows for SCN-01 same-field and SCN-02 target/self reads. | `useful-floor` from `ELAB-11`, `ELAB-12`, and `plan/75` | No OPEN-014 read materialization decision; no cache / observe / reply policy freeze. |
| Visible write consequence | Visible writes expose publish / observe consequences where canon requires visibility effects. | `keep-core-idea` / `useful-floor` from `ELAB-09` and `ELAB-11` | No final viewer / telemetry ABI and no runtime `MessageEnvelope` dispatch. |
| Failure-row containment | Generated failures remain explicit and underdeclared rows stay static-boundary failures. | `keep-core-idea` / `useful-floor` from `ELAB-07`, `ELAB-10`, `ELAB-13..16`, and E-ROW plans | No final diagnostic ID / repair ABI freeze and no generic runtime reject collapse. |
| Authority obligation carrier | Owner-directed requests carry capability / witness obligation representation. | `keep-core-idea` from THM-001 / OBL-001 statement boundary and selected LAB request rows | No authority soundness, grant-lineage proof, production auth, or G3 completion. |
| Source span mapping | Generated consequences map back to the assignment source span. | `useful-floor` from `ELAB-05`, `ELAB-09`, and later diagnostic projection evidence | No final Core JSON `source_map` or diagnostic exchange ABI freeze. |
| Statement boundary | OBL-001 / OBL-020 / OBL-021 statement-shape drafts and sync guards remain compile-check-only evidence. | `useful-floor` from `plan/73..78`, `plan/117`, and Lean lab statements | No proof discharge, proof skeleton completion, or `theory/11` status movement. |

## Minimal source scenario shape

The slice should stay at the level of abstract source pressure, not a new domain
catalog.

Required source pressure:

- two loci: actor/current locus and state owner locus;
- S-owned indexed state with one key parameter and one field;
- one simple assignment in a nested owner block;
- one RHS dependency case where the assigned field is also read;
- one RHS dependency case with two reads;
- one visible-field write case that requires publish / observe consequence;
- one underdeclared generated-failure negative row;
- one nested-locus non-authority guard.

Scenario names such as `World`, `Participant`, `player`, `position`, `hp`, and
`atk` may appear only as SCN examples. They are not Mir core vocabulary and
should not be required by the slice.

## Evidence map

| Evidence | Use for | Do not use for |
|---|---|---|
| `mirrorea_canon/plan/00-gates.md` | G1 exit criteria and non-claim boundary. | Claiming exit. |
| `mirrorea_canon/plan/01-phases.md` | T0/T1/T2 phase reading. | Treating LAB artifacts as implementation-state movement. |
| `mirrorea_canon/theory/03-elaboration.md` | BND-001 / THM-001 contract wording. | Freezing helper JSON or runtime dispatch. |
| `mirrorea_canon/theory/11-metatheory-ledger.md` | Proof status authority. | Moving OBL status from LAB evidence. |
| `plan/71-g1-ordinary-assignment-target.md` | G1 target and proof-boundary split. | Broad Surface program soundness. |
| `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` | SCN-01/02 static consequence rows. | C-static conformance pass. |
| `plan/75-g1-scn-rhs-dependency-gap-evidence.md` | RHS dependency evidence for SCN-01/02 pressure. | OPEN-014 resolution or read transport policy. |
| `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md` | Ordinary-assignment claim traceability. | Canon source or gate closeout. |
| `plan/119-g0-remaining-claim-family-drilldown-priority.md` | Stop lines for non-ordinary rows. | Opening later-gate rows by default. |
| `plan/120-repo-triage-recut-matrix.md` | keep-core / useful-floor / archive / postpone scheduling vocabulary. | Relabeling runnable workflow status or moving files. |
| `samples/full-system-v1-surface/{syntax,indexed-state,elaboration}/` | Runnable LAB evidence floor. | Final sample status, final grammar, final runtime, or conformance. |
| `samples/lean/lab-statements/obl001`, `obl020`, `obl021` | Compile-check-only statement-shape evidence. | Proof, proof skeleton completion, or canon ledger movement. |

## Narrow executable evidence set

The current candidate should cite only the narrow rows below when it needs
runnable LAB evidence:

| Row | Use in this slice | Boundary |
|---|---|---|
| `ELAB-02` | Nested foreign place write becomes an owner-directed write request. | Structural support only; not SCN pass evidence. |
| `ELAB-05` | Generated Core IR carries source-span pressure. | No span proof or final JSON ABI. |
| `ELAB-07` | Generated write request rejects underdeclared failure rows. | Diagnostic / repair details remain LAB-only and non-final. |
| `ELAB-09` | Visible write generates explicit publish / observe pressure. | LAB `MessageEnvelope` is not runtime dispatch or canon vocabulary. |
| `ELAB-10` | Visible communication rejects underdeclared `VisibilityDenied`. | Does not prove the exact SCN-01 visible-write negative row. |
| `ELAB-11` | SCN-01-shaped visible same-field assignment records owner-directed write, RHS dependency, publish / observe, and spans. | No runtime dispatch, C-static pass, or proof discharge. |
| `ELAB-12` | SCN-02-shaped attack assignment records owner-directed write and target/self RHS dependencies. | OPEN-014 read materialization remains unresolved. |

Implementation-path citations for these rows should stay LAB-only:
`scripts/surface_mir_samples.py`,
`crates/mir-semantics/examples/surface_to_core_elaborate.rs`,
`crates/mir-semantics/src/surface_to_core_elaboration.rs`, and
`crates/mir-semantics/tests/surface_to_core_elaboration.rs`.

## Explicitly out of scope

- Role admission, membership lifecycle, capability grant-lineage soundness, and
  witness freshness beyond obligation representation.
- Fallback, lease, stale non-resurrection, load/rollback, and monotone
  degradation.
- Runtime request serving, store mutation, occurrence ordering, process split,
  transport, WAN/federation, Docker-as-final-runtime, and distributed durable
  save/load.
- Projection/backend packet schema, FFI, provider adapter, renderer semantics,
  native/WASM execution, and provider-owned semantics.
- Source patch hot-plug, activation cut, migration, rollback, and final patch
  ABI.
- Final devtools/viewer/telemetry ABI, redaction/retention completion, and
  public diagnostic / repair ABI.
- Product Alpha release packaging, `package.mir.json`, installer/native bundle,
  public CLI/API/SDK freeze, and final shared-space catalog.
- Domain vocabulary such as `World`, `Room`, `Avatar`, `Game`, `Portal`, or
  sample role names as core primitives.

## Hidden failure modes to guard

| Failure mode | Guard |
|---|---|
| Calling the slice "vertical" and then importing runtime/product scope. | Keep the label `STATIC`; require every row to cite parse/check/elaborate/statement-boundary evidence only. |
| Treating `keep-core-idea` as canon promotion. | Always cite canon first and call the label a LAB scheduling overlay. |
| Treating `useful-floor` as workflow completion or conformance. | Cite commands/fixtures as reproducible evidence only. |
| Freezing helper JSON / `MessageEnvelope` / `rhs_indexed_read` as final Core ABI. | Phrase semantic consequences abstractly and keep helper names in LAB evidence columns. |
| Resolving OPEN-014 by accident. | Say "dependency/read consequence is explicit" and leave materialization open. |
| Smuggling G3 authority into G1. | Require obligation representation only; keep grant-lineage proof separate. |
| Collapsing publish / observe into debug output. | Treat visible-write consequence as an information effect boundary, not a devtools panel. |
| Turning E-ROW repair evidence into final diagnostic/repair ABI. | Keep E-ROW rows as diagnostic pressure and OBL-024/025 support only. |
| Reopening all `plan/70` rows. | Follow `plan/119`: only ordinary-assignment support is immediate by default. |
| Reusing the broad `plan/69` minimal slice literally. | Treat `plan/69` as pre-`plan/120` management synthesis; this file narrows the current slice to G1 static assignment only. |
| Treating Lean compile-check as theorem status. | Keep `theory/11` unchanged and say compile-check-only. |

## Next safe package options

| Option | When to choose it | Close condition |
|---|---|---|
| `OBL-001 statement refinement` | Review finds the current statement draft misses an abstract predicate from the slice spine. | Lean compile-check and guard tests pass; no proof or ledger movement. |
| `OBL-020/021 boundary review` | The slice needs sharper separation between WF preservation and determinism before OBL-001 wording changes. | Statement-shape docs and guards remain compile-check-only. |
| `SCN exact static slice manifest` | A future package needs one concise reader-facing map from SCN-01/02 rows to current LAB fixtures before touching Lean. | No new sample status; exact fixture references only. |
| `canon mental-model clarification proposal` | A real canon wording gap is found around ordinary assignment, dependency, or nested-locus non-authority. | Proposal only; no canon edit without human/canon process. |

## Required non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No proof-obligation status movement.
- No proof discharge.
- No proof skeleton completion.
- No C-static, C-runtime, or C-distributed conformance claim.
- No implementation-state completion.
- No runnable sample status relabel.
- No final grammar/API/Core IR/diagnostic/repair/runtime/transport/projection
  ABI freeze.
- No promotion of helper/sample/report/Lean compile-check evidence to canon.

## Open questions

- Should the next concrete package refine `THM001StatementDraft.lean`, or should
  it first write a SCN exact static slice manifest that names the minimal fixture
  rows without touching Lean?
- Does the current OBL-001 statement need a more explicit abstract predicate for
  visible-write publish / observe consequences, or is the existing target shape
  sufficient until proof work starts?
- Should a future canon proposal introduce a short non-domain mental model for
  ordinary assignment before T1, or can current canon wording carry G1 work?

## Close condition

This file is closed when `plan/00-index.md`, the docs validators, current
snapshot docs, and the package report are synchronized.

Close condition is candidate-map-only: no canon edit, no gate exit, no proof,
no conformance claim, no implementation change, and no runnable sample status
change.
