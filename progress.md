# progress

最終更新: 2026-07-28 00:41 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

`docs/project-status.md` is the human control view, `tasks.md` is the current
work map, `plan/` is detailed repository memory, and `docs/reports/` is
immutable task evidence. This file creates no normative decision.

## project axis

```text
正しい理論に基づき、正しく hot-plug でき、Place をまたいで
実行・通信・検証・可視化できる仮想空間システム
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform remain
separable. `World` and `Game` are user-defined concepts, not Mir primitives.

## final ideal

```text
.mir -> Surface parse/elaboration -> Core typed IR and obligations
     -> checker/runtime -> projection/deployment -> provider/View evidence
```

Communication, authority, failure, observation, and evolution stay explicit
at typed boundaries. Authentication is not transport identity; visualization
is not an untyped debug leak.

## current milestone position

| Axis | Status | Next boundary | Startability |
| --- | --- | --- | --- |
| Logical specification | official `T0`; all OBL rows `open`; core direction fixed, proof-facing interfaces incomplete | T0 profile repair, semantic dispositions, T1/T2 profiles, shared formal model | conditional |
| User-facing specification | bounded notation/scenario/sample evidence; final Surface closure unresolved | PROPOSAL-004, OPEN-005, `return`, SCN-08 reconciliation | owner specification required |
| Implementation / operation | bounded Surface/current-L2/Product Alpha/Full System/operational/Lean evidence is runnable | owner-selected I1 authorization route, exact target fragment, and C-static timing | later dependency |

Current exact blockers:

1. T0 profile uses both `pass` and `derived-pass`.
2. The retained artifact self-binds the contradictory v1 source and cannot be
   made conforming by a later text-only correction.
3. G0-D3 remains deferred; no G0 exit / T1 entry record exists.
4. T1/T2 lack canonical phase profiles.
5. Outcome totality, value/receipt/service/admission identity, request
   validation context, and Surface/SCN closure remain owner-reserved choices.
6. No accepted shared Core/Config/Step/WellFormed/elaboration/history model
   exists for T1/T2 proof-facing packages.
7. `spec/06` calls C-static 10/10 I1 entry while the phase table places it in
   I1 exit; Canon has no distinct bootstrap authorization.

The last source-cut screen selected no new WRK from its reviewed delta. That
is a LAB priority result, not a permanent restriction on ADR-0014. A fresh
literal-transcription or conditional-lemma candidate may proceed only after
its own standing-eligibility preflight.

Sources: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`,
`plan/196-t0-t2-implementation-entry-roadmap.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`.

## milestone map

| Phase | Primary aim | Current position | Self-drive |
| --- | --- | --- | --- |
| T0 | vocabulary, decisions, G0 | official current; exact profile conflict | research/preparation yes; exit no |
| T1 | calculus, G1-G3 statements, final SCN expectations | no official entry | ADR-0014-eligible packages after owner dispositions |
| T2 | OBL-020/021/002 skeletons and G5 statements | later | skeleton research after accepted model; exit no |
| I1 | single-process reference implementation | later | blocked on owner-selected authorization route and C-static timing |
| I2 | in-process multi-locus | later | blocked on I1 |
| I3 | real transport | later; first real LAN phase | blocked on I2 and transport ADR |
| I4-I6 | persistence/patch, View, distributed persistence/federation | later | sequential dependency |

Current Canon T2 is narrower than proven I1 readiness. The current LAB
recommendation, if the owner selects narrow T2, is a separately accepted
I1-readiness / bootstrap record. It would bind all-SCN / G0-G7 statement-level
criteria, OBL-003/027 classification, C-static timing, and scoped production
authorization; integrated and phase-contract alternatives remain open. It does
not imply final carrier freeze or C-runtime conformance.

## line snapshots

### Product Alpha line

`samples/product-alpha1/` has bounded package check/run, attach, save/load,
native bundle, and two-process Docker TCP evidence. `.mir` is not its direct
execution input. It is not final ABI, WAN federation, distributed durability,
or official implementation status.

### Operational Suite line

The WorldCore through TwoShard/Gradient suite exercises documented bounded
contracts and evidence. It is not final shared-space catalog or distributed
durable execution.

### Mir Language line

Surface parser, indexed-state checks, elaboration, admission, source patch,
static devtools, and a bounded computational fragment are runnable LAB
evidence. Final grammar, public error contract, and Canon-aligned common proof
model remain open.

### PoseGraph line

PoseGraph has bounded sample evidence. Its performance-sensitive kernel remains
separate from Mir runtime semantics.

### Projection/Backend line

Projection, deployment, and backend/provider artifacts are bounded LAB
evidence or planned boundaries. BND-006 and later implementation phases govern
their normative realization.

### Engine/Provider line

Provider and engine adapters are typed external boundaries. They do not define
world semantics, authentication, authorization, or Mir core.

## validation floor

| Evidence | Current command |
| --- | --- |
| docs and source hierarchy | `python3 scripts/validate_docs.py`; `python3 scripts/check_source_hierarchy.py` |
| Canon metadata | `python3 meta/build-index.py --check` from `mirrorea_canon/` |
| Surface LAB | `python3 scripts/surface_mir_samples.py check-all --format json` |
| Lean statement shapes | direct `lake env lean` under `samples/lean/lab-statements/` |
| runnable dashboards | commands in `samples_progress.md` |

Run the changed layer's focused validation plus docs/index/hierarchy checks.
Passing LAB evidence does not move a Gate, Phase, or OBL.

## non-claims

No Gate/Phase exit, OBL discharge, final proof, conformance result, final
grammar/API/ABI, production implementation, real federation, distributed
durable save/load, or public product completion is claimed.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Owner / Canon | T0 profile and artifact route | recommend v2 `pass`, retain v1 as nonconforming history, authorize one fresh v2 artifact |
| Owner / Canon | G0-D3 | decide exact fresh digest separately from profile repair |
| Owner / Canon | T1/T2/I1 lifecycle contract | define profiles, Gate-to-ledger mapping, proof-skeleton evidence class, narrow T2 vs I1 readiness, bootstrap/C-static relation |
| Owner / Canon | PROPOSAL-008/012/013 | decide totality, V/R/S/A, and request-validation context |
| Owner / Canon | Surface/SCN closure | decide PROPOSAL-004, OPEN-005, `return`, and SCN-08 coherently |
| Research | conservative statement preflight | test ADR-0014 eligibility; open L3 only for non-duplicate existing-lane literal/conditional evidence |
| Research after decisions | shared model and Gate packages | compare, formalize, falsify, validate, review, and prepare acceptance packets |
| Later dependency | runtime, conformance, transport, federation | do not preempt the theory/lifecycle contract |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | repository memory and governance | current snapshots and Plan 196 synchronized | light | yes |
| 1 | semantic kernel | direction fixed; proof-facing relations incomplete | heavy | owner choices unlock only ADR-0014-eligible existing-lane packages |
| 2 | parser-free validation | compatibility anchors runnable | medium | maintenance/reproduction |
| 3 | compile-ready actualization | bounded Surface/Full System evidence exists | heavy | production widening deferred |
| 4 | sample expansion | active roots runnable | heavy | maintenance before I1 |
| 5 | theorem/model-check bridge | drafts, countermodels, frozen/not-promoted WRKs exist | heavy | conservative L3 preflight now; main line after shared model |
| 6 | distributed fabric | later | heavy | after I1/I2 |
| 7 | toolchain/backend | bounded LAB only | heavy | later |
| 8 | applications | bounded samples only | heavy | outside T0-T2 critical path |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- | --- |
| multi-node / fabric | bounded local/two-process LAB | I2/I3, transport and distributed contracts | later dependency |
| contracts / theorem / model-check | statement drafts, countermodels, L3/frozen evidence, dependency audits | accepted shared model, exact statements/skeletons, ledger status | conservative preflight only |
| dynamic attach/detach / DAG-safe evolution | bounded LAB sample evidence | G7 and implementation contract | later dependency |
| `atomic_cut` / ordering | Canon theory plus bounded hook/cut evidence | noncircular G5 model and OBL-009..014 statements | after model choice |
| executable sample corpus | runnable active roots | conformance and public workflow | maintenance |
| Mir language foundation | bounded parse/check/elaboration/compute evidence | exact Surface/Core fragment, owner-selected I1 authorization route, C-static timing | owner specification required |
| Mirrorea fabric | bounded alpha/runtime evidence | I2/I3 semantics and transport ADR | later dependency |
| Typed-Effect Wiring | typed adapter evidence | public contract and authority integration | later dependency |
| PrismCascade | separable bounded sample/kernel evidence | dedicated semantics/performance contract | later dependency |
| upper applications | user-defined sample worlds | stable lower-layer workflow | later dependency |

## recent log

- 2026-07-28 00:41 JST: completed an I1 bootstrap/readiness audit with a
  planner, an independent reviewer, and GPT-5.6 Sol Pro Oracle. It found no
  current official start path and isolated the C-static entry/exit tension.
  Plan 197 recommends a separate I1-readiness record only if the owner selects
  narrow T2; integrated and phase-contract alternatives remain open. The first
  owner checkpoint remains the T0 profile/artifact route. No Canon, Gate/Phase,
  OBL, sample, implementation, or public status moved.
- 2026-07-27 19:38 JST: audited the path from official T0 through T2 and the
  I1 implementation entrance with independent sub-agent and GPT-5.6 Sol Pro
  review. Plan 196 now separates official owner-reserved transitions from
  autonomous ADR-0014 research, defines the dependency DAG and package gates,
  and records that current Canon T2 is narrower than all-SCN I1 readiness. No
  Canon, Gate/Phase, OBL, sample, implementation, or public status moved.
- 2026-07-26 23:54 JST: added the Japanese HTML orientation map and Mermaid
  source; browser, print, Mermaid, Product Alpha, and operational validations
  passed. This was a reader-facing LAB map only.
- 2026-07-25 01:50 JST: completed the whole-theory source audit; identified
  the T0 `pass` / `derived-pass` conflict and retained all semantic/formal
  boundaries as decision requests or reserved interfaces.
