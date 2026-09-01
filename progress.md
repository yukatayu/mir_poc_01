# progress

最終更新: 2026-09-01 23:03 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins. This concise LAB snapshot
creates no Canon, Gate, Phase, proof, lifecycle, or compatibility decision.

## document role

Plan 247 and Plan 249 are closed execution records. PROPOSAL-037 / ADR-0034
authorize the active bounded Mirrorea I3 Distributed Foundation program;
Plan 250 is the sole current roadmap. ALIGN-0, ALIGN-1, and ALIGN-2 completed;
I3-0 sole active goal. Reports retain detailed evidence.

## project axis

```text
正しい理論に基づき、正しく hot-plug でき、Place をまたいで
実行・通信・検証・可視化できる仮想空間システム
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform remain
separable. World, Avatar, Bird, and Viewer remain sample/library vocabulary.

## three independent axes

The current Canon map [`architecture/06-project-product-layers.md`](mirrorea_canon/architecture/06-project-product-layers.md)
keeps semantic strata (`S0`--`S6`), project/product responsibility layers
(`PL-0`--`PL-6`), and lifecycle phases (`T0`--`T2` / `I1`--`I6`) independent
and many-to-many. `S6 Host` is not `PL-0`, and lifecycle maturity is not a
product-layer acceptance claim. PL-4 is responsibility-only; PL-6 remains a
separate inactive application project, and PrismCascade/Typed-Effect remain
satellites.

## final ideal

```text
ordinary source -> checked Core -> ownership/effect/failure/lifetime
-> per-locus artifacts + generated communication
-> process/network execution -> typed devtools -> save/patch/hot-plug
-> View/browser/renderer -> persistent virtual-space system
```

The accepted boundary reaches in-process generated dispatch and finite typed
assurance. Real multi-process transport and product layers remain later.

## current milestone position

| Axis | Current status | Startability |
|---|---|---|
| Logical specification | finite source -> Core -> artifact -> communication -> in-process trace/conformance accepted; Theory T1 and broad PHASE-I1 unaccepted | maintenance **着手可能**; general widening **後段依存** |
| User-facing specification | provisional project/run/inspect/conform workflow exists; public grammar/CLI/JSON/API/ABI/wire/devtools unfrozen | regression **着手可能**; public contract **要仕様確認** |
| Implementation / operation | I2 exit preserved; ALIGN-1/2 constitution and boundary contracts closed; I3-0 active | I3-0 **着手可能**; lifecycle entry not official |

```text
Theory: T1
Broad PHASE-I1: unaccepted (OPEN-026/027 + full carrier freeze)
Official I2: entry accepted -> exit accepted (ADR-0032)
ADR-0026 program: SYS-0--SYS-7 closed (ADR-0033)
Active roadmap / goal: Plan 250 / I3-0 sole active goal
Sequence: ALIGN-0 completed → ALIGN-1 completed → ALIGN-2 completed → I3-0 active → I3-1..6 → NEXT-0
I3 bounded program active; lifecycle entry not official; OPEN-032 unresolved
```

The plan/05 boundary retains Candidate A TLS-over-TCP framed reliable
stream and Candidate B QUIC reliable stream, both **UNSELECTED**. QUIC
datagrams are excluded. No version, codec, wire, library,
certificate representation, port, retry, topology, implementation, or public
compatibility decision exists. Transport/session/certificate/route identity
is not authority; internal carrier and public wire remain separate. Future
SCN-01/02/03/06 C-distributed evidence must cover the full typed network
failure/order matrix without hidden retry or exactly-once.

Sources: `mirrorea_canon/adr/ADR-0034.md`, `mirrorea_canon/plan/05-i3-entry-contract.md`,
`mirrorea_canon/plan/01-phases.md`, and
`plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`.

## milestone map

| Milestone | Capability | Position / evidence |
|---|---|---|
| M0--M10 | finite Mir Theory v0 + deterministic I1+ | closed `23f5a813...`; ADR-0025 |
| SYS-0 | authority and one goal/control path | closed; Report 2592 |
| SYS-1 | kernel/conformance separation + carrier | closed `94e3707c...` |
| SYS-2 | ST/OW1 + bounded ordering/model | closed `920d3fe...`; OBL-058/059 |
| SYS-3 | per-locus artifacts + generated plans | closed `3013e7fe...`; OBL-060 |
| SYS-4 | in-process generated dispatch | closed `22196f93...`; OBL-061 |
| SYS-5 | four-locus toy + joined devtools | closed `53a21e64...`; OBL-062 |
| SYS-6 | finite I2 conformance/lifecycle | closed `5429712d...`; OBL-063 / ADR-0032 |
| ALIGN-0 | status/roadmap activation alignment | completed; Plan 250 |
| ALIGN-1 | project/product layer constitution | closed; three-axis map accepted |
| ALIGN-2 | Browser/Host/package/View/provider boundary contracts | completed; BND-007, BND-010..BND-016 and trust boundaries accepted |
| I3-0 | transport candidate evidence and selection | sole active goal; both candidates UNSELECTED, OPEN-032 unresolved |
| SYS-7 | inactive I3 entry contract only | closed; ADR-0033 / Canon plan/05 |

## line snapshots

### Product Alpha line

Historical Product Alpha and Full System V1 remain LAB consumers, not the
semantic queue or public-product completion evidence.

### Operational Suite line

The four-locus toy plus provisional `project-loci`, `run-local`, `inspect`, and
`conform-i2` form a bounded reproducible I2 workflow. They are not stable public
interfaces and no network sample was added by SYS-7.

### Mir Language line

Ordinary `.mir` source and checked Core remain semantic authority. Bounded
designated-consumer and relation-anchor clauses do not freeze final grammar.
Carriers, reports, workers, transports, and schedules cannot mint Core,
authority, state, or expected results.

### PoseGraph line

The accepted relation fragment preserves explicit A-primary/B-fallback
lineage, consumer-local late projection, presentation-gap nonmutation, and
leave/fresh incarnation. Arbitrary DAG theory remains deferred.

### Projection/Backend line

Checked Core creates owned locus artifacts and generated plans; SYS-4 executes
them across explicit endpoints. ST is the reference and selected OW1 is a
separate exactly-one-worker source. Network refinement is authorized for the
fixed I3 milestones but is not yet implemented at current I3-0.

### Engine/Provider line

Observer-safe internal outputs are evidence surfaces, not public devtools.
Transport authentication and providers remain non-authority. PrismCascade,
browser/View/renderer, and upper applications remain separable.

## validation floor

| Changed layer | Required evidence family |
|---|---|
| Canon/docs | regenerated INDEX, hierarchy/docs/HTML tests, `make docs`, diff check |
| accepted SYS-6 | library 25 + CLI 8 |
| preserved systems | SYS-2 28, SYS-3 28, SYS-4 104, SYS-5 62 |
| M10 baseline | conformance 67 + CLI 4 |
| implementation close | workspace, format, warnings-denied Clippy |
| lifecycle close | independent I2/broad-I1/I3/no-roadmap review |

ALIGN-0 changed Canon/docs/status only and is closed. ALIGN-1 changed Canon/docs/status only and is closed. ALIGN-2 closed the Browser/Host/package/View/provider responsibility boundary: trust tier T0–T4 (Theory T0–T2 とは別), package admission and semantic grant remain separate, T1 has no raw FFI or direct store, and View permits presentation-local computation without authoritative domain semantics. I3-0 is now active with no transport selected.

## non-claims

No broad PHASE-I1 exit, Theory T2, socket/multi-process/WAN runtime, transport
selection, official I3 lifecycle entry, public grammar/CLI/API/ABI/wire/JSON/devtools schema,
durable distributed persistence, production/publication, browser/View product,
whole-toy OW1, arbitrary relation DAG/scheduler/fairness/memory/data-race
theorem, exactly-once, lock-free runtime, or public completion is claimed.

## user decision items vs research-discovery items

| Class | Item | Current state |
|---|---|---|
| Maintenance | accepted M10/I2 regressions and docs consistency | **着手可能** |
| Active package | I3-0 transport comparison/selection | **着手中**; ALIGN-2 contracts closed |
| Research discovery | candidate comparison, network failures/order, C-distributed gates | fixed I3-0/I3-3/I3-4 consumers |
| Delegated decision | OPEN-032 transport choice | I3-0 equal canaries + ADR required |
| Owner decision | public freeze or production | reserved |
| Later dependency | broad I1 carrier freeze/general theory | separate residual; no weaker criteria |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| 0 | governance/repository memory | ALIGN-0--2 completed; I3-0 active | medium | transport selection evidence |
| 1 | semantics/shared model | finite semantics through I2 | heavy | ADR-0014 research only |
| 2 | parser-free evidence | historical | medium | maintenance |
| 3 | source/checker/runtime | in-process I2 accepted | heavy | after ALIGN-1/2, I3-1/2 |
| 4 | executable samples | toy + conform reproducible | medium | regression |
| 5 | theorem/model bridge | OBL-058 bounded; 059--063 runtime | heavy | class maintenance |
| 6 | generated/distributed fabric | in-process accepted; network absent | heavy | fixed I3-0..6 sequence |
| 7 | toolchain/backend | provisional commands | heavy | no freeze |
| 8 | applications | toy is library/sample | heavy | no Core promotion |

## feature maturity rows

| Feature/subsystem | Evidence status | Remaining gate | Startability |
|---|---|---|---|
| Mir core/runtime | finite source/check/project/dispatch assured | general/public widening | maintenance |
| Mirrorea fabric | generated in-process endpoints; official I2 exit | multi-process transport | **後段依存** on ALIGN-2 and I3-0 |
| contracts/model | typed falsifiers + bounded/runtime classes | network/general proof | **後段依存** |
| attach/detach/DAG | leave/fresh, local cut, bounded patch | durable/general evolution | **後段依存** |
| `atomic_cut` / ordering | high-level edges, ST/OW1, bounded model | network/general memory | **後段依存** |
| samples | four-locus toy + conform | public workflow | regression possible |
| Typed-Effect | typed request/result + no-mint | broader network/providers | **後段依存** |
| PrismCascade | separate performance kernel | no I2 integration | deferred |
| View/browser | historical boundary | product/API program | deferred |
| upper applications | toy + historical consumers | no domain Core promotion | product-specific |

## recent log

- 2026-09-01 23:03 JST: ALIGN-2 Browser/Host/package/View/provider boundaries
  were accepted; BND-007 and BND-010..BND-016, trust tiers, typed reverse paths,
  raw FFI separation, redaction, and resource termination responsibilities were
  synchronized. I3-0 became the sole active goal; both transports remain
  UNSELECTED and OPEN-032 unresolved.
- 2026-09-01 22:22 JST: ALIGN-1 project/product three-axis map was accepted;
  PL-4 remained responsibility-only, PL-6 stayed separate, and ALIGN-2 became
  the sole active goal.
- 2026-09-01 22:05 JST: ALIGN-0 activation cut `2f198105...` passed focused
  I2/M10 and docs/config validation, independent review, push/parity; ALIGN-0
  completed and ALIGN-1 became the sole active activation-only goal.
- 2026-09-01 21:06 JST: PROPOSAL-037 / ADR-0034 and Plan 250 activation state
  synchronized into LAB snapshots with ALIGN-0 active/closing; transport
  candidates remained UNSELECTED and OPEN-032 unresolved.
- 2026-08-28 18:58 JST: SYS-6 cut `5429712d...` closed the exact 22-row I2
  profile and ADR-0032 accepted official I2 entry then exit.
- 2026-08-28 14:09 JST: SYS-5 cut `53a21e64...` closed toy/devtools.
- 2026-08-27 21:06 JST: SYS-4 cut `22196f93...` closed generated dispatch.
- 2026-08-27 07:07 JST: corrected SYS-3 cut `3013e7fe...` closed projection.
- 2026-08-27 01:09 JST: SYS-2 cut `920d3fe...` closed ST/OW1 evidence.
- 2026-08-26 23:09 JST: SYS-1 cut `94e3707c...` closed kernel/carrier.
- 2026-08-05 15:53 JST: M10 cut `23f5a813...` accepted finite I1+ baseline.
