# progress

最終更新: 2026-09-09 22:02 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins. This concise LAB snapshot
creates no Canon, Gate, Phase, proof, lifecycle, or compatibility decision.

## document role

Plan 247 and Plan 249 are closed execution records. PROPOSAL-037 / ADR-0034
authorize the active bounded Mirrorea I3 Distributed Foundation program;
Plan 250 is the sole current roadmap. ALIGN-0, ALIGN-1, ALIGN-2, I3-0 and I3-1
are completed; I3-2 and I3-3 are accepted, and execution is owner-paused with no
active semantic milestone. I3-4, I3-5, I3-6, and NEXT-0 remain inactive and dependency-gated.

Latest Plan250 owner control: I3-3 is accepted and owner-paused. Plan 250 remains the
sole retained current roadmap; I3-4 requires explicit owner resume. This is not
blocked, stale or program-closed.

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
-> process/network execution (I3-2 bounded evidence accepted) -> typed devtools -> save/patch/hot-plug
-> View/browser/renderer -> persistent virtual-space system
```

The accepted boundary reaches the finite I3-3 source/evidence profile. I3-4 owns
C-distributed evidence after explicit resume; WAN, durability, browser and
public/production layers remain later.

## current milestone position

Separate current task: user-authorized proof-first LAB research, active single goal
W1-passive-erasure (PL1 S1/S3, theory/proof). This is not a Plan250 milestone or
I3-4 resume. Scoped support/current-use/tracked-query/graph Lean proofs and reviews
exist; passive exact-retention extension review completed. Source/type/dynamic
composition and actual implementation/alpha gates remain open. Required corpus
reading is incomplete. Memory: `plan/proof-first-foundation-correspondence.md`;
evidence: `docs/reports/2611-mirrorea-proof-first-w1-foundations.md`.

| Task axis | Current status | Startability |
|---|---|---|
| 論理仕様 | general conditional Lean proofs; source IFC/current-head/all-mutator/physical bridge open | reversible research **着手可能** |
| ユーザ向け仕様 |119requirement inputs retained; ordinary-source alpha profile not adopted | necessary source/type investigation **着手可能**; irreversible policy **要仕様確認** |
| 実装 / 運用 | existing M8 observer4tests pass; trusted-setup effective-label mismatch reproduced; no new network/durable alpha evidence | relevant proof gates **後段依存** |


I3-3 is accepted at source/evidence cut `fe5dd972e2ddb3a513c785458a07702e4d4d99fa`.
The finite profile covers all twenty failure/order families; retained assurance
is workspace 1573, runtime doctests 4, format, Clippy and final review P0/P1/P2=0.
I3-3 accepted; owner pause leaves no active semantic milestone. Plan 250 remains
the sole retained current roadmap. I3-4/I3-5/I3-6/NEXT-0 remain dependency-gated
inactive; I3-4 requires explicit owner resume. Official I3 lifecycle entry remains
unaccepted. Browser, world semantics, public/production and save/patch/global-cut
claims remain outside the cut. Seven of eleven milestones are accepted; this is
not a workload percentage.

| Axis | Current status | Startability |
|---|---|---|
| Logical specification | finite source -> Core -> artifact -> communication -> in-process trace/conformance accepted; Theory T1 current; broad PHASE-I1 exit unaccepted | maintenance **着手可能**; general widening **後段依存** |
| User-facing specification | provisional project/run/inspect/conform workflow exists; public grammar/CLI/JSON/API/ABI/wire/devtools unfrozen | regression **着手可能**; public contract **要仕様確認** |
| Implementation / operation | I2 exit preserved; finite I3-3 source/process/QUIC/host/custody and ordering profile accepted; later distributed/browser/public layers remain deferred | I3-4+ **後段依存** |

```text
Theory: T1
Broad PHASE-I1: unaccepted (OPEN-026/027 + full carrier freeze)
Official I2: entry accepted -> exit accepted (ADR-0032)
ADR-0026 program: SYS-0--SYS-7 closed (ADR-0033)
Active roadmap / goal: Plan 250 retained / I3-3 accepted and owner-paused
Sequence: ALIGN-0 completed → ALIGN-1 completed → ALIGN-2 completed → I3-0 completed → I3-1 completed → I3-2 completed/accepted → I3-3 accepted/paused → I3-4..6 → NEXT-0
I3 bounded program is owner-paused with no active semantic milestone; later milestones are dependency-gated inactive; lifecycle entry remains unaccepted; OPEN-032 resolved only for this program
```

PROPOSAL-040 / ADR-0037 select Candidate B QUIC reliable stream
as the private bounded-program adapter after Candidate A TLS/TCP and Candidate B QUIC reliable
stream passed equal source/Core-bound actual-process canaries and tied on
criteria 1--7. Criteria 8 and 9 had no auditable winner; criterion 10 future
browser relevance was the first material difference. A TLS/TCP remains a deferred replacement baseline and QUIC datagrams are
excluded. No public version, codec, wire, certificate representation, API/ABI,
port, topology, platform or production compatibility decision exists.
Transport/session/certificate/route identity is not authority; internal carrier
and public wire remain separate. Future
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
| I3-0 | transport candidate evidence and selection | completed; private QUIC selected, TLS/TCP deferred baseline |
| I3-1 | checked private adapter/encoding/admission | completed by ADR-0038; bounded evidence, not official lifecycle |
| I3-2 | generated-artifact multi-process runtime | completed/accepted bounded evidence; FM-5, not public workflow |
| I3-3 | finite failure/ordering refinement | accepted; ADR-0043, source fe5dd972 / integration aafde922 |
| I3-4 / I3-5 / I3-6 / NEXT-0 | scenarios / devtools / conformance / entry contracts | inactive; explicit resume then dependency-gated |
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
fixed I3 milestones; I3-0 supplied the selection canary, I3-1 accepted the
checked carrier mapping, and I3-3 accepted finite failure/ordering refinement.

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

ALIGN-0 changed Canon/docs/status only and is closed. ALIGN-1 changed Canon/docs/status only and is closed. ALIGN-2 closed the Browser/Host/package/View/provider responsibility boundary: trust tier T0–T4 (Theory T0–T2 とは別), package admission and semantic grant remain separate, T1 has no raw FFI or direct store, and View permits presentation-local computation without authoritative domain semantics. I3-0 closed the private transport choice after equal canaries; I3-1 and I3-2 closed the private adapter/encoding and two-process runtime boundaries. ADR-0043 accepted finite I3-3; execution is owner-paused and I3-4 requires explicit resume.

## non-claims

No broad PHASE-I1 exit, Theory T2, arbitrary-network failure/order or WAN runtime, public
transport/platform selection, official I3 lifecycle entry, public grammar/CLI/API/ABI/wire/JSON/devtools schema,
durable distributed persistence, production/publication, browser/View product,
whole-toy OW1, arbitrary relation DAG/scheduler/fairness/memory/data-race
theorem, exactly-once, lock-free runtime, or public completion is claimed.

## user decision items vs research-discovery items

| Class | Item | Current state |
|---|---|---|
| Maintenance | accepted M10/I2 regressions and docs consistency | **着手可能** |
| Current package | I3-3 network failure/order refinement | accepted finite cut; owner-paused after consuming accepted I3-2 runtime |
| Research discovery | private carrier mapping, network failures/order, C-distributed gates | fixed I3-1/I3-3/I3-4 consumers |
| Delegated decision | OPEN-032 transport choice | resolved for this program by ADR-0037 |
| Owner decision | public freeze or production | reserved |
| Later dependency | broad I1 carrier freeze/general theory | separate residual; no weaker criteria |

## macro phase map

This map records the retained Plan250 program and its milestone gates. Separately
authorized task-local LAB startability is described above; Plan250 resume
requirements do not govern that independent research.

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| 0 | governance/repository memory | ALIGN-0--2 and I3-0--3 completed; owner pause | medium | status maintenance |
| 1 | semantics/shared model | finite semantics through I2 | heavy | ADR-0014 research only |
| 2 | parser-free evidence | historical | medium | maintenance |
| 3 | source/checker/runtime | I2 and finite I3-3 source/provider/custody accepted | heavy | maintenance; later work requires resume |
| 4 | executable samples | toy + conform reproducible | medium | regression |
| 5 | theorem/model bridge | OBL-058 bounded; 059--063 runtime | heavy | class maintenance |
| 6 | generated/distributed fabric | I3-2 actual owner runtime and I3-3 failure/order accepted | heavy | owner pause; I3-4 requires explicit resume |
| 7 | toolchain/backend | provisional commands | heavy | no freeze |
| 8 | applications | toy is library/sample | heavy | no Core promotion |

## feature maturity rows

| Feature/subsystem | Evidence status | Remaining gate | Startability |
|---|---|---|---|
| Mir core/runtime | finite source/check/project/dispatch assured | general/public widening | maintenance |
| Mirrorea fabric | generated two-process owner runtime plus accepted private adapter and I3-3 finite cut | I3-4 direct consumer after explicit resume | **後段依存** |
| contracts/model | typed falsifiers + bounded/runtime classes | network/general proof | **後段依存** |
| attach/detach/DAG | leave/fresh, local cut, bounded patch | durable/general evolution | **後段依存** |
| `atomic_cut` / ordering | high-level edges, ST/OW1, bounded model | network/general memory | **後段依存** |
| samples | four-locus toy + conform | public workflow | regression possible |
| Typed-Effect | typed request/result + no-mint | broader network/providers | **後段依存** |
| PrismCascade | separate performance kernel | no I2 integration | deferred |
| View/browser | ALIGN-2 Canon responsibility/trust boundary fixed | safe package runtime and product/API program | **後段依存** |
| upper applications | toy + historical consumers | no domain Core promotion | product-specific |

## current validation checkpoint

Source/evidence cut `fe5dd972e2ddb3a513c785458a07702e4d4d99fa` is accepted by
ADR-0043; Canon/status integration is `aafde92229bb0ff18116f38d4750a0a8f61cb069`.
Retained source validation: workspace1573, runtime doctests4, format/Clippy and
independent source review P0/P1/P2=0. These I3-3 results remain source-cut evidence, not reruns for this
synchronization. Separate LAB Lean and targeted M8 executions are recorded in
`docs/proof-first/FOUNDATION_CHECK.json` and `docs/proof-first/OBSERVER_BASELINE_CHECK.json`.

Actual process/network, LOCAL authority/codec/custody/graph and bounded-model
evidence stay distinct. The lifecycle model explores432states/2136transitions;
it is not a general theorem. Private provider-fault assertions are not exported
observer traces. The A-local cut emits no saved image and does not implement
save/restore, live patch, global cut or durability.

Detailed historical counts and commands remain in immutable Report2606.
Earlier Plan250 maintenance recorded63GiB free and target2GiB. The current task
M8 baseline grew target to4.4GiB; current preflight has about60GiB free and12GiB
available RAM. Additional large variant builds await a measured storage plan;
bounded proof/rustc work continues. Source/.git/reports/logs are preserved.
Only ignored Python/pytest caches were cleaned under the owner's instruction.
The authorized roadmap is paused, not blocked, stale or closed.

## recent log

- 2026-09-09 14:08 JST: I3-3 final source cut `fe5dd972` passes workspace 1573,
  runtime doctests 4, format/Clippy and independent source review P0/P1/P2=0.
  PROPOSAL-046 / ADR-0043 record finite acceptance and owner pause; Plan 250 is
  retained with no active semantic milestone and I3-4 requires explicit resume.
- 2026-09-09 15:41 JST: Consolidated LAB snapshots to the accepted/owner-paused
  frontier; detailed component history remains in Report 2606. Reader validation
  was run for the overview after the status synchronization.
- 2026-09-09 15:52 JST: Snapshot maintenance passes overview13, full docs
  (218 Canon / 800 hierarchy / 1760 reports), diff checks and independent planner
  review. About3MiB Python/pytest cache cleaned; source, current2GiB build cache
  and evidence retained. No Rust/Lean rerun or I3-4 activation; details in Plan250.

- 2026-09-09 22:02 JST: Proof-first LAB: general support/current-use/tracked/graph and passive Lean
  checks passed at recorded cuts; Oracle reviews and source reading ongoing. M8
  observer4baseline tests passed; effective-label trusted-setup countermodel
  reproduced. No implementation refinement/alpha acceptance or I3-4 resume.
