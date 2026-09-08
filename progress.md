# progress

最終更新: 2026-09-08 19:08 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins. This concise LAB snapshot
creates no Canon, Gate, Phase, proof, lifecycle, or compatibility decision.

## document role

Plan 247 and Plan 249 are closed execution records. PROPOSAL-037 / ADR-0034
authorize the active bounded Mirrorea I3 Distributed Foundation program;
Plan 250 is the sole current roadmap. ALIGN-0, ALIGN-1, ALIGN-2, I3-0 and I3-1
are completed; I3-2 is accepted and the owner has resumed execution at I3-3. I3-4, I3-5,
I3-6, and NEXT-0 remain inactive and dependency-gated.

Latest owner control: finish I3-3 through required validation, independent
review, commit/push and parity, then pause. I3-4 requires explicit owner resume.
Until I3-3 acceptance it remains the sole active milestone; afterward Plan 250
is retained with no active semantic milestone, not blocked or program-closed.

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

The accepted boundary reaches in-process generated dispatch and finite typed
assurance. I3-1 adds bounded private QUIC adapter/encoding evidence, and I3-2
adds accepted finite local two-process QUIC evidence. I3-3 full failure/order /
C-distributed evidence, WAN, durability, browser and public/production layers
remain later.

## current milestone position

Provider static handoff `3862b168` is the last pushed cut (clean parity
2026-09-08 15:41 JST). Its static coverage/composite verification/
generated projection passes fresh semantics 6/17/8, privacy 1, static runtime
11. Current Stage2b runtime is 362/362, including provider filtering 26/26
(not additive); process is 63/63, M10 is 67/67, I2 local/CLI is 5/5 and 8/8,
and public guards are 5/5. Three-crate all-target warnings-denied Clippy,
workspace format, and workspace all-target check pass; workspace tests and
CLI/probe Clippy are not claimed. Independent spec/quality review is P0/P1/P2
zero after three direct tests and generic M8 typed refusal. The source is
frozen; Stage2b is inactive/green for parent integration and awaits a new Git
cut. Actual binding, M9 authorization and intact composite verification feed
separate policy/scoped M8, then the existing SYS4/SYS5 mixed private-image/
admission variant; actual bounded host/QUIC runtime remains later. Six of eleven
milestones are accepted; I3-3 remains unaccepted.

| Axis | Current status | Startability |
|---|---|---|
| Logical specification | finite source -> Core -> artifact -> communication -> in-process trace/conformance accepted; Theory T1 and broad PHASE-I1 unaccepted | maintenance **着手可能**; general widening **後段依存** |
| User-facing specification | provisional project/run/inspect/conform workflow exists; public grammar/CLI/JSON/API/ABI/wire/devtools unfrozen | regression **着手可能**; public contract **要仕様確認** |
| Implementation / operation | I2 exit preserved; I3-1/I3-2 accepted; provider source and static projection/verification tested; effect grant/invocation and remaining membership/redaction/cut/full-matrix gates remain | I3-3 **着手可能**; I3-4+ **後段依存** |

```text
Theory: T1
Broad PHASE-I1: unaccepted (OPEN-026/027 + full carrier freeze)
Official I2: entry accepted -> exit accepted (ADR-0032)
ADR-0026 program: SYS-0--SYS-7 closed (ADR-0033)
Active roadmap / goal: Plan 250 / I3-3 active after owner resume at accepted I3-2 cut
Sequence: ALIGN-0 completed → ALIGN-1 completed → ALIGN-2 completed → I3-0 completed → I3-1 completed → I3-2 completed/accepted → I3-3 active → I3-4..6 → NEXT-0
I3 bounded program current; I3-3 is the only active semantic milestone, later milestones are dependency-gated inactive; lifecycle entry not official; OPEN-032 resolved only for this program
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
checked carrier mapping, and I3-3 now refines failure/ordering.

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

ALIGN-0 changed Canon/docs/status only and is closed. ALIGN-1 changed Canon/docs/status only and is closed. ALIGN-2 closed the Browser/Host/package/View/provider responsibility boundary: trust tier T0–T4 (Theory T0–T2 とは別), package admission and semantic grant remain separate, T1 has no raw FFI or direct store, and View permits presentation-local computation without authoritative domain semantics. I3-0 closed the private transport choice after equal canaries; I3-1 and I3-2 closed the private adapter/encoding and two-process runtime boundaries. Owner resume activates I3-3 only; later milestones remain dependency-gated.

## non-claims

No broad PHASE-I1 exit, Theory T2, I3-3 full failure/order or WAN runtime, public
transport/platform selection, official I3 lifecycle entry, public grammar/CLI/API/ABI/wire/JSON/devtools schema,
durable distributed persistence, production/publication, browser/View product,
whole-toy OW1, arbitrary relation DAG/scheduler/fairness/memory/data-race
theorem, exactly-once, lock-free runtime, or public completion is claimed.

## user decision items vs research-discovery items

| Class | Item | Current state |
|---|---|---|
| Maintenance | accepted M10/I2 regressions and docs consistency | **着手可能** |
| Current package | I3-3 network failure/order refinement | active after explicit owner resume; consumes accepted I3-2 runtime |
| Research discovery | private carrier mapping, network failures/order, C-distributed gates | fixed I3-1/I3-3/I3-4 consumers |
| Delegated decision | OPEN-032 transport choice | resolved for this program by ADR-0037 |
| Owner decision | public freeze or production | reserved |
| Later dependency | broad I1 carrier freeze/general theory | separate residual; no weaker criteria |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| 0 | governance/repository memory | ALIGN-0--2 and I3-0/I3-2 completed; I3-3 resumed | medium | status/decision maintenance |
| 1 | semantics/shared model | finite semantics through I2 | heavy | ADR-0014 research only |
| 2 | parser-free evidence | historical | medium | maintenance |
| 3 | source/checker/runtime | I2 accepted; I3-3 provider source/guard component verified | heavy | I3-3 direct consumers only |
| 4 | executable samples | toy + conform reproducible | medium | regression |
| 5 | theorem/model bridge | OBL-058 bounded; 059--063 runtime | heavy | class maintenance |
| 6 | generated/distributed fabric | I3-2 actual owner runtime accepted; I3-3 failure/order active | heavy | I3-3 self-drive; later gates dependency-gated |
| 7 | toolchain/backend | provisional commands | heavy | no freeze |
| 8 | applications | toy is library/sample | heavy | no Core promotion |

## feature maturity rows

| Feature/subsystem | Evidence status | Remaining gate | Startability |
|---|---|---|---|
| Mir core/runtime | finite source/check/project/dispatch assured | general/public widening | maintenance |
| Mirrorea fabric | generated two-process owner runtime plus accepted private adapter; I3-3 active | I3-3 full failure/order matrix | **着手可能** |
| contracts/model | typed falsifiers + bounded/runtime classes | network/general proof | **後段依存** |
| attach/detach/DAG | leave/fresh, local cut, bounded patch | durable/general evolution | **後段依存** |
| `atomic_cut` / ordering | high-level edges, ST/OW1, bounded model | network/general memory | **後段依存** |
| samples | four-locus toy + conform | public workflow | regression possible |
| Typed-Effect | typed request/result + no-mint | broader network/providers | **後段依存** |
| PrismCascade | separate performance kernel | no I2 integration | deferred |
| View/browser | ALIGN-2 Canon responsibility/trust boundary fixed | safe package runtime and product/API program | **後段依存** |
| upper applications | toy + historical consumers | no domain Core promotion | product-specific |

## current validation checkpoint

I3-3 is active and unaccepted. Time/reply checkpoint 55f1fd7f is pushed;
its full probe 41/41 and actual replay 3/3 are retained evidence, not reruns
of the provider component. Genuine G1-expiry→G2 rejection remains local
binder evidence with inspected QUIC correspondence. Earlier detailed cuts,
counts and timing are retained in Report 2606, not a parallel current queue.

Retained Stage 1 evidence at `9e8d674a`, implementing the source/checking and
unsupported-legacy-handoff portion of ADR-0042/spec/17 (not current Stage2b):

- Provider source suite 17/17; external constructor privacy doctest 1/1.
- Private runtime guards 2/2; public M8/M9/SYS5/M10 guards 5/5.
- Stage 1 feature-union runtime library 334/334; process runtime 63/63;
  M8 admission 7/7 and M9 external-boundary target 1/1.
- I2 local 5/5, I2 CLI 8/8 and M10 conformance 67/67.
- Three-crate all-target Clippy with private-QUIC/process-seam features and
  warnings denied, plus workspace format check, pass.
- Independent source/spec and runtime review: no remaining P0/P1/P2.
  Invalid snapshot identifiers, public pre-M7 Core construction and unsupported
  provider-tagged M8 restore were reproduced and repaired. Filtered suites
  overlap broader suites and are not additive totals.

Logs, earlier AST/M6/M7/time regressions and final docs/Git state are in Report
2606. The new sample is an incomplete local effect-authorization prototype with
partial local authorization, not an M5/Lean clean-runner registration or
executable provider workflow. No accepted host invocation, supervisor
availability, row-18 acceptance, new Lean/general proof or I3 lifecycle is
claimed. Remaining rows 13/15/19/20, ordering, and whole-I3-3 validation remain
required. Latest measured free disk is 30,906,707,968 bytes (~28.78 GiB); the
10 GiB guard has no current capacity hold. Older lower-disk readings remain
historical only.

Owner clarification remains ordinary meaning -> generated distribution ->
continually checked composition. Domain words remain library/sample vocabulary;
finite ledger/cohort bounds are not general Mir requirements. Complete I3-3
then stop with Plan 250 retained and I3-4 inactive.

## recent log

- 2026-09-08 19:08 JST: final Stage2b quality/regression packet passes runtime
  362/362, provider filter 26/26 within that total, process 63/63, M10 67/67,
  I2 local/CLI 5/5 and 8/8, and public guards 5/5; reviews are P0/P1/P2 zero.
  Component is inactive/green for parent integration; no provider activation or
  I3-3 acceptance is claimed.
- 2026-09-08 16:48 JST: Stage2b module behavior recorded as 7 passed, 3 failed,
  and 345 filtered; two policy regressions pass and three repair blocks remain.

- 2026-09-08 12:07 JST: actual successful/expiry reply replay and wrong-session
  prewrite refusal pass 3/3, full probe 41/41 and final Clippy/format pass;
  independent time/reply review P0/P1/P2 zero. Provider contract is next after
  integration commit/parity; I3-3 remains active, with I3-4 inactive.

- 2026-09-08 10:53 JST: retained predecessor delivered expiry/loss/reconnect and
  local-wait evidence (six named tests), full probe 38/38, runtime library
  332/332 and integration 63/63; narrow review, Clippy/format and I2/M10 floor
  are green. This is historical evidence, not the current Stage2b rerun.
  Actual reply replay and the remaining I3-3 matrix are next; no milestone acceptance.

- 2026-09-08 09:42 JST: predecessor owner-budget run and full probe integration
  pass with four exact delivery joins; host-driver 5/5 and broad local I3 56/56
  were green. Superseded by the current 61/61 feature-union rerun.

- 2026-09-07 22:39 JST: I3-3 owner clock/gate/one-use handoff slice passes
  14+1+1+1 focused tests after four review repairs and exact G2 handoff repair;
  independent source review has no P0/P1. Dirty delta over `050f5067`, not a
  milestone close. Next: typed expiry reply/requester terminal retention,
  then actual QUIC evidence; provider/full matrix remain open.

- 2026-09-07 21:19 JST: default-guard checkpoint `050f5067` is pushed with
  remote parity verified at 21:09. Source-based staging falsifier now reaches
  the expected behavioral RED (0/1); serialized owner gate implementation
  begins within I3-3. No milestone closes or later milestone activates.

- 2026-09-07 18:13 JST: owner schedules pause after complete I3-3 acceptance,
  validation/review and push/parity; I3-4 will require explicit resume. Current
  I3-3 work and all failure/order requirements remain unchanged.

- 2026-09-02 02:52 JST: I3-0 equal TLS/TCP and QUIC actual-process canaries
  passed the bounded nine-case floor; PROPOSAL-040 / ADR-0037 selected private
  QUIC at criterion 10 after criteria 8/9 tied, retained TLS/TCP as replacement evidence, kept official
  I3 unentered, and activated I3-1.

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
- 2026-09-02 09:39 JST: I3-1 private adapter/encoding cut `d75fa2e7...` and
  ADR-0038 were recorded; focused evidence/reviews passed and I3-2 became the
  sole active goal. Workspace validation remains explicitly NOT PASS after
  disk-pressure exit 130.
- 2026-09-02 20:00 JST: I3-2 source/evidence cut `19c5b386...` accepted after
  repeated 12/12 localnet, full probe 62/62, exact four-record lineage and
  independent P0=0/P1=0 review. Plan 250 remains authorized/current but execution
  resumed at I3-3; later I3 milestones remain dependency-gated inactive, the
  bounded program is not closed, and official I3 lifecycle remains unentered.

- 2026-09-07 11:38 JST: owner resume accepted at the I3-2 cut; status mirrors
  now designate only I3-3 as active and retain I3-4/I3-5/I3-6/NEXT-0 as inactive.
- 2026-09-07 17:52 JST: genuine revocation/retained-ingress slice passes runtime
  61/61, feature library 289/289, probe 29/29, QUIC unit 2/2, Clippy/format/diff,
  I2 5/5 + 8/8 and M10 67/67; independent repair review clears P0/P1. Earlier
  model evidence is retained, not rerun; I3-3 full-matrix acceptance remains open.
- 2026-09-07 18:52 JST: endpoint-close/two-write/truncation and validated-only
  observer evidence pass probe 32/32, full probe library 3/3, runtime 61/61,
  QUIC 2/2, I2 5/5 + 8/8 and focused Clippy; independent P0/P1 clear.
  Feature-library/M10/model remain prior; time/provider and full I3-3 stay open.
- 2026-09-07 19:31 JST: ADR-0041/spec/16 time contract selected in the current
  integration; independent normative P0/P1 resolved. Static source test confirms
  behavioral parser RED before implementation; runtime/network evidence remains
  pending, provider OPEN, I3-3 active until complete acceptance then owner pause.
- 2026-09-07 19:56 JST: static budget/identity/snapshot tests 14/14, old
  AST/M6/M7 10/13/27, compile-fail 1/1 and focused Clippy pass after reviewed
  cardinality/span/raw-Core repairs. Generated contracts and runtime gating
  remain pending; no milestone or annotated network workflow accepted.
- 2026-09-07 20:34 JST: generated contracts 8/8, snapshots 6/6, default M8
  guard 2/2, M8 regressions 33/33 and current I2/I3/M10 regressions pass;
  independent projection/M8 review clear. Alternative-entry tests, precise
  kernel rejection and runtime Clippy repairs precede the actual clock gate.
- 2026-09-07 21:00 JST: default budget guard integration is validated after
  repairing a contextual-trace panic and false route diagnosis. Budget library
  11/11, M10 source 3/3, runtime 63/63, library 298/298, M8/I2/M10/probe/QUIC
  regressions and Clippy/format pass; independent review clear. Actual clock,
  permit and expiry are next; I3-3 remains active, not accepted.
- 2026-09-08 12:43 JST: pinned pushed time/reply checkpoint `55f1fd7f` with
  clean remote parity; ADR-0042/spec/17 provider contract selected after
  five P1 repairs and independent P0/P1/P2-zero re-review. Source/test work
  follows docs validation; no provider execution or I3-3 acceptance claimed.
- 2026-09-08 13:42 JST: provider source/snapshot and unsupported legacy handoff pass
  17 source, 1 privacy, 2 private and 5 public guard tests; runtime 334,
  process 63, I2/M10 and focused quality gates pass after three reproduced
  review repairs. No provider invocation or I3-3 acceptance; exact Git state
  remains in Report 2606.
- 2026-09-08 14:05 JST: source/guard component `9e8d674a` pushed with clean
  HEAD/origin/main/live parity after code, review and docs gates. Static provider
  coverage/projection is next; no host execution, I3-3 close or I3-4 activation.
- 2026-09-08 15:23 JST: static provider coverage/composite verification/projection
  final test packet passes 6/17/8 semantics, 1 privacy, 11 static and 345 full
  runtime tests after four review repairs. Final integration gates remain;
  no provider authority/invocation or I3-3 acceptance follows.
- 2026-09-08 15:35 JST: static handoff independent spec/quality reviews have
  no P0/P1/P2; process 63, I2/M10 5/8/67, public guards 5 and final lint/format/
  workspace compilation pass. Post-lint static 11 passes; actual binding and
  M9 policy/scoped-M8 follow, not provider invocation or milestone acceptance.
- 2026-09-08 15:44 JST: static handoff `3862b168` committed/pushed with clean
  HEAD/origin/main/live parity at 15:41:54 JST. Stage 2b actual resource binding,
  M9 composite authorization and inactive scoped M8 are current; no invocation,
  I3-3 acceptance or I3-4 activation.
