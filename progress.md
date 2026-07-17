# progress

最終更新: 2026-07-17 16:14 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the concise LAB snapshot of workflow readiness and evidence. It is not a canon decision record or a historical log. Read `docs/project-status.md` for the human control view, `tasks.md` for the current work map, and `plan/` for detailed repository memory.

## project axis

```text
Correct theory -> safe hot-plug -> execution, communication, verification,
and visualization across Places in a virtual-space system.
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform remain separable. A domain `World` or `Game` is user-defined on Mir; it is not a Mir core primitive.

## final ideal

```text
.mir -> Surface parse/elaboration -> Core typed IR and obligations
     -> checker/runtime -> projection/deployment -> host/provider/view evidence
```

The target carries source semantics through placement, explicit communication, contracts, evolution, and observation without folding authentication or visualization into an untyped transport/debug channel.

## current milestone position

| Axis | Status | Readiness / next boundary |
| --- | --- | --- |
| Logical specification | canon is `T0/G0 rebaseline`; G0 exit and T1 entry are unrecorded | T0-T2 evidence research is permitted; canon status remains controlled by `mirrorea_canon/plan/01-phases.md` and `mirrorea_canon/theory/11-metatheory-ledger.md` |
| User-facing specification | source-first direction, companion notation, and example guidance exist as LAB evidence | no final grammar, public API, or final user contract |
| Implementation / operation | Product Alpha, Full System V1, and Surface roots are runnable bounded LAB evidence | no C-static/C-runtime/C-distributed conformance or final runtime/product claim |

`T-RESEARCH-001` through `T-RESEARCH-003` are `research-complete`: finite
countermodels established the OBL-001/020/021 statement boundary, one
`[E-WRITE]` store-key clause was checked under a value-only/frame reading, and
one `[E-OBS]` graph kernel was proved under a fresh, prefix-preserving,
incoming-only extension. `T-RESEARCH-005` is also `research-complete`: an
`[E-DEGRADE]/[E-REACQ]` two-rule scratch kernel preserves only prior defined
map entries under explicit experiment-local support, framing, and lineage
assumptions. None is a canon append, lineage, or full-step definition. No canon
theorem, status, or lifecycle changed. The canonical lifecycle remains
`mirrorea_canon/plan/01-phases.md`; the LAB work-selection and stop boundary is
`plan/156-t0-t2-research-autonomy-envelope.md`.

`T-RESEARCH-006` is `research-complete` as a frozen source-adequacy audit,
not a proof: all 13 selected transition cases x five named WF clauses are
`missing` under a derivation-complete criterion (`0 direct / 65 missing`). The
audit localizes the missing premises to history extension, component frames,
state/membership coherence, authority records, and chain transitions. It
rejects using unproved THM-006 as an OBL-020 premise and records a two-state
Lean challenge showing why `[E-SERVE]/fail` no-store-change alone cannot frame
the membership epoch. No canon status, definition, or lifecycle changed.

`T-RESEARCH-007` is `decision-ready` for OBL-020 only. It prepared
PROPOSAL-003's A/B/C choice for organizing a future proof-facing formalization:
shared five-heading LAB-derived review checklist (advisory recommendation), no
required shared checklist with package-local organization, or defer. The
headings are not canon predicates or fixed Lean premises. The proposal is
non-self-executing and does not block unrelated existing-lane theory research;
no concrete transition, frame, or carrier is defined until the owner records a
disposition.

`T-RESEARCH-008` is `research-complete` for the OBL-021 BND-001 source cut:
all three abstract determinism postconditions are canonically intended but
`0 direct / 0 delegated / 3 missing` under a derivation-complete criterion.
The missing force is result/projection coherence, diagnostic equivalence, and
shared-outcome exclusion; no equality, datatype, statement, or proof status was
selected.

`T-RESEARCH-009` is `research-complete` as a bounded OBL-005
structural-flattening kernel. An opaque-leaf, experiment-local binary shape
with a free ordered-word output proves only that one reassociation preserves
structural output, and that a one-hole meta-context is an identity. An
order-reversed fold still satisfies reassociation but fails a two-leaf
left-to-right oracle; a separate empty-constructor mutation demonstrates that
a source-level unit needs a changed signature. This does not define canonical
applicability, chain validity, source-level unit, confluence, evaluation, or
OBL-005 status.

`T-RESEARCH-010` is `research-complete` as an OBL-006 source-adequacy audit:
`0 direct / 0 delegated / 1 missing` formalization boundary. Same ordered
output does not determine confluence; the source cut does not select a term
domain, guarded validity, equality/denotation, relation, or theorem shape.

`T-RESEARCH-011` is `research-complete` as a THM-002 / OBL-007
trace-formalization audit: `0 direct / 0 delegated / 1 missing` complete
statement boundary. The canon directly fixes monotone same-lineage selection
and explicit fresh reacquire policy, but does not yet give the trace, selection,
lineage, origin/reacquire, freshness, and transition/frame relations from which
a Lean statement could be derived. Two disposable finite models delimit those
missing bindings; no theorem, status, or lifecycle changed.

`T-RESEARCH-012` is `research-complete` as a THM-004 / OBL-015
mutation-origin audit: `0 direct / 0 delegated / 1 missing` coupled
formalization boundary. The canon directly fixes grant-lineage authority for
delegated capability use and separately permits owner-local mutation. It does
not yet define the proof-facing association from a particular mutation to a
validating use/request/capref, the owner-local/declared-transition branch, or
the complete trace/step link. A favorable-order three-event twin isolates only
the experiment-local association; no theorem, status, or lifecycle changed.

`T-RESEARCH-004` was not selected after its bounded preflight. A literal-RHS
foreign-locus source pair reproduced request shape, failure-row acceptance /
rejection, and source spans, but the current lane has no structured
capability/witness carrier or JSON-to-OBL-001 interpretation. This is an
evidence-route blocker, not a canon counterexample; it creates a decision-ready
owner item for the bridge only.

The owner-facing bundle records the authority cut, alternatives, evidence
level, non-claims, and reopen trigger for that item. The later direct theory
objective allows unrelated existing-lane research around the dormant bridge,
but is not recorded as an owner defer or bridge authorization.

## milestone map

| Phase | Primary aim | Current position | Autonomy |
| --- | --- | --- | --- |
| T0 | vocabulary and G0 | current; G0-D3 deferred | bounded LAB research only |
| T1 | calculus, G1-G3 statements, final SCN expectations | no official entry | T1-oriented research only |
| T2 | proof skeletons and G5 statements | later research target | Lean exploration; proof status stays in `theory/11` |
| I1 | reference implementation | later | blocked on theory exits |
| I2 | multi-locus | later | blocked on I1 |
| I3 | real transport | later; first real LAN point | blocked on I2 / transport ADR |
| I4-I6 | persistence, View, distributed federation | later | blocked on prior phases |

Exact exits are `mirrorea_canon/plan/00-gates.md` and `mirrorea_canon/plan/01-phases.md`, not this table.

## line snapshots

### Product Alpha line

`samples/product-alpha1/` provides bounded runnable product-alpha evidence, including operational and release-check workflows. It is not a final product, public compatibility promise, or canon implementation status. See `samples_progress.md`.

### Operational Suite line

The WorldCore through TwoShard/Gradient sample suite can be checked through its documented LAB commands. It exercises bounded same-session contracts and evidence, not real distributed durability or a final shared-space catalog. See `samples_progress.md`.

### Mir Language line

Surface parser, indexed-state checker, elaboration, role admission, source patch, and static devtools are runnable LAB evidence. The active theory focus is the G1 ordinary-assignment static bridge and its OBL statement boundary, not runtime widening. See `plan/121-g1-minimal-vertical-slice-candidate-map.md` and `plan/156-t0-t2-research-autonomy-envelope.md`.

### PoseGraph line

PoseGraph has bounded LAB sample evidence. Its performance-sensitive kernel remains separable from Mir runtime semantics and is not a current theory shortcut. See `samples_progress.md`.

### Projection/Backend line

Projection, deployment, and backend/provider artifacts remain bounded LAB evidence or planned boundaries. BND-006 and later implementation phases govern their realization. See `mirrorea_canon/plan/00-gates.md`.

### Engine/Provider line

Provider and engine adapters are typed external boundaries. They do not define world semantics, authentication, authorization, or the Mir core. See `samples_progress.md`.

## validation floor

| Evidence | Current command |
| --- | --- |
| documentation/source hierarchy | `python3 scripts/validate_docs.py` and `python3 scripts/check_source_hierarchy.py` |
| Surface static LAB anchor | `python3 scripts/surface_mir_samples.py check-all --format json` |
| OBL statement shapes | direct `lake env lean` checks under `samples/lean/lab-statements/` |
| runnable dashboards | commands recorded in `samples_progress.md` |

Run only the anchor relevant to the changed layer plus the required documentation checks; broad validation is evidence, not a phase transition.

## non-claims

No Gate/Phase exit, OBL status movement, proof discharge, conformance result, final grammar/API/ABI, real transport, distributed durable save/load, or public product claim is made by this snapshot or by runnable LAB evidence.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Owner / canon | G0-D3 | deferred and dormant until explicit owner reopen |
| Owner / canon | semantic choice, SCN expectation change, ADR/canon action, `theory/11` movement | agent prepares a decision bundle and stops at `decision-ready` |
| Owner / LAB route | OBL-001 concrete-evidence bridge | owner must explicitly defer it or authorize an artifact-free design comparison with its existing route and permitted persistence; direct theory work does not itself decide the bridge; current LAB recommendation is defer |
| Research | smallest concrete premises for OBL-020/021/001 | T-RESEARCH-001/002/003/005 complete; select a new source-grounded rule x invariant x falsifier under `plan/156` |
| Research | G2/G3 statement feasibility after the first boundary is known | select only after the `plan/156` rule is met |
| Research | G2/G3 statement boundaries | OBL-005 has only a structural-output kernel; OBL-006 does not fix a rewrite relation; THM-002 does not yet fix a complete trace formalization; THM-004 does not yet fix a mutation-origin/authorization interface |
| Later dependency | runtime, conformance, final ABI, transport, federation | do not preempt theory phase |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | repository memory and reporting discipline | cockpit and source hierarchy are in use | light | maintenance on concrete drift |
| 1 | semantic kernel and invariant boundaries | canon direction fixed; concrete proof premises under research | medium | eligible work units |
| 2 | parser-free validation substrate | existing runners are compatibility anchors | medium | reproduce only as needed |
| 3 | compile-ready actualization | Surface alpha evidence closed | heavy | maintenance only |
| 4 | sample expansion | bounded operational evidence exists | heavy | maintenance only |
| 5 | theorem / model-check bridge | T-RESEARCH-001/002/003/005 are bounded LAB evidence; no active canon package | medium | eligible work units within `plan/156` |
| 6 | distributed fabric and runtime evolution | later | heavy | later dependency |
| 7 | toolchain/backend surface | bounded LAB evidence only | heavy | later dependency |
| 8 | domain/application realization | bounded samples exist; products are later | heavy | later dependency |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node / fabric | local and bounded LAB evidence | I2/I3 and transport choice | later dependency |
| contracts / theorem / model-check boundary | statement drafts and static evidence | concrete premises, proof skeletons, `theory/11` status | research eligible |
| dynamic attach/detach / DAG-safe evolution | bounded LAB sample evidence | canon G7 / implementation | later dependency |
| `atomic_cut` / ordering | canon theory exists | G5 statements / proof research | research eligible after current priority |
| executable sample corpus | runnable bounded LAB workflows | conformance and public operational workflow | maintenance only |

## recent log

- 2026-07-17 15:15 JST: Completed the OBL-006 relation-boundary audit. A
  word-preserving finite fork lacks a join, so no canonical confluence claim
  follows without a separately specified formalization boundary.
- 2026-07-17 15:36 JST: Completed the THM-002 / OBL-007 trace-formalization
  audit. The normative monotonicity and explicit-reacquire policy remains
  fixed; trace, lineage-origin, selection, freshness, and transition bindings
  remain a coupled formalization boundary before a Lean statement.
- 2026-07-17 16:14 JST: Completed the THM-004 / OBL-015
  mutation-origin/authorization audit. Delegated grant-lineage policy and the
  owner-local alternative remain fixed; a favorable-order finite twin isolates
  the missing association boundary without defining a canonical trace.
- 2026-07-17 14:41 JST: Completed the bounded OBL-005 structural-flattening
  kernel. One reassociation preserves an experiment-local structural output;
  a hole context is meta-syntax, and reverse order / added-empty mutations
  delimit the left-to-right and source-signature boundaries without changing
  canon or proof status.
- 2026-07-17 14:20 JST: Completed the OBL-021 BND-001 postcondition audit.
  Three trusted finite falsifiers isolate projection coherence, diagnostic
  equivalence, and branch exclusion without changing canon or the Lean draft.
- 2026-07-17 14:00 JST: Oracle review narrowed PROPOSAL-003 to a symmetric
  organizational A/B/C choice. The five audit groups are now explicitly LAB
  candidate review headings, not canon vocabulary or a required Lean interface;
  no owner disposition or canon status changed.
- 2026-07-17 13:25 JST: Prepared PROPOSAL-003 for the OBL-020 formalization
  boundary. It requests an owner A/B/C choice only; no rule equation, proof
  status, ADR, Gate, Phase, or implementation change was applied.
- 2026-07-17 13:17 JST: Completed the OBL-020 source-adequacy audit: 65/65
  selected transition x WF cells lack derivation-complete canon premises; the
  result narrows five missing-premise groups without changing canon, OBL, Gate,
  Phase, or proof status.
- 2026-07-17 11:57 JST: Completed the conditional `[E-DEGRADE]/[E-REACQ]`
  lineage kernel after adversarial review: prior scratch entries remain defined
  and nondecreasing only under stated experiment-local assumptions; no canon
  lineage, OBL, Gate, Phase, or proof-status claim changed.
- 2026-07-17 10:23 JST: Independent re-review passed the corrected OBL-001
  bridge decision bundle; it remains a pending owner disposition with no new
  lane, artifact, or successor research selection.
- 2026-07-17 10:16 JST: Applied independent-review corrections to the OBL-001
  bundle: only defer and fully scoped artifact-free design comparison are
  current dispositions; a committed bridge remains a separate escalation.
- 2026-07-17 10:07 JST: Consolidated the OBL-001 concrete-evidence bridge
  decision bundle from existing evidence and independent review; no owner
  disposition, new lane, or successor research unit was inferred.
- 2026-07-17 09:51 JST: Rejected a proposed literal-RHS foreign-locus
  T-RESEARCH-004 after the source pair reproduced but lacked an existing
  authority-carrier interpretation; no new lane was created and the bridge is
  now an owner decision-ready blocker.
- 2026-07-17 09:15 JST: Reproduced the runnable LAB front doors: workspace
  tests, Surface, Full System V1, Product Alpha, installed-binary, operational,
  and current-L2 checks all accepted their bounded evidence; no canon or
  readiness status changed.
- 2026-07-17 09:04 JST: Completed a disposable `[E-OBS]` graph-kernel study:
  a weak extension admits a finite cycle, while the stated incoming-only
  extension preserves two selected graph invariants; canon state is unchanged.
- 2026-07-16 19:48 JST: Completed bounded countermodel research for the three
  statement shapes and a single OBL-020 `[E-WRITE]` store-key preservation
  clause; both are LAB evidence only and leave canon status unchanged.
- 2026-07-16 19:15 JST: Recut the human control view and LAB snapshots around canon state, runnable evidence, decision queue, and research order; recorded the owner-authorized T0-T2 research envelope without changing canon status.
- 2026-07-15 20:11 JST: ADR-0013 recorded the T0/G0 profile evidence; G0-D3 remained deferred, so no G0 exit or T1 entry was claimed.
