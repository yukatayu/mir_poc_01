# progress

最終更新: 2026-07-16 19:48 JST

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

`T-RESEARCH-001` and its narrow `T-RESEARCH-002` continuation are
`research-complete`: finite countermodels established the boundary of the
OBL-001/020/021 statement shapes, then one `[E-WRITE]` store-key clause was
checked under the value-only/frame reading of the canon update notation. No
canon theorem, status, or lifecycle changed. The next concrete rule/clause is
selected only by `plan/156-t0-t2-research-autonomy-envelope.md`; the canonical
lifecycle remains `mirrorea_canon/plan/01-phases.md`.

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
| Research | smallest concrete premises for OBL-020/021/001 | T-RESEARCH-001/002 complete; select the next explicit rule/clause only with a falsification criterion |
| Research | G2/G3 statement feasibility after the first boundary is known | select only after the `plan/156` rule is met |
| Later dependency | runtime, conformance, final ABI, transport, federation | do not preempt theory phase |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | repository memory and reporting discipline | cockpit and source hierarchy are in use | light | maintenance on concrete drift |
| 1 | semantic kernel and invariant boundaries | canon direction fixed; concrete proof premises under research | medium | eligible work units |
| 2 | parser-free validation substrate | existing runners are compatibility anchors | medium | reproduce only as needed |
| 3 | compile-ready actualization | Surface alpha evidence closed | heavy | maintenance only |
| 4 | sample expansion | bounded operational evidence exists | heavy | maintenance only |
| 5 | theorem / model-check bridge | T-RESEARCH-001/002 are bounded LAB evidence; no active canon package | medium | eligible work units within `plan/156` |
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

- 2026-07-16 19:48 JST: Completed bounded countermodel research for the three
  statement shapes and a single OBL-020 `[E-WRITE]` store-key preservation
  clause; both are LAB evidence only and leave canon status unchanged.
- 2026-07-16 19:15 JST: Recut the human control view and LAB snapshots around canon state, runnable evidence, decision queue, and research order; recorded the owner-authorized T0-T2 research envelope without changing canon status.
- 2026-07-15 20:11 JST: ADR-0013 recorded the T0/G0 profile evidence; G0-D3 remained deferred, so no G0 exit or T1 entry was claimed.
