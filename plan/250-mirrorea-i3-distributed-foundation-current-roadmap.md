# Plan 250 — Mirrorea I3 Distributed Foundation current execution roadmap

最終更新: 2026-09-01 20:53 JST

## 役割、authority、current control state

これは owner-authenticated direction により開始された bounded program

```text
Mirrorea I3 Distributed Foundation
```

の **唯一の current LAB execution roadmap** である。規範正本は
`mirrorea_canon/`、公式 Gate / Phase は
`mirrorea_canon/plan/01-phases.md`、proof status は
`mirrorea_canon/theory/11-metatheory-ledger.md` だけが決める。この roadmap は
実行順、依存、Goal Statement、受理証拠、停止線を保持するが、単独では Canon、
Gate、Phase、SCN、OBL、proof status、public compatibility を変更しない。

この program の owner direction は、ADR-0033 が要求した fresh owner direction であり、
従来の「active program なし」という状態を **この固定 scope に限って** supersede
する。durable な Canon mirror / activation authority は次で記録する。

- `mirrorea_canon/meta/proposals/PROPOSAL-037-mirrorea-i3-distributed-foundation.md`
- `mirrorea_canon/adr/ADR-0034.md`

Plan 250 はその authority の代用品ではない。ALIGN-0 close では上記 Canon record と
roadmap/status pointer の一致を検証する。

- dispatch baseline revision:
  `ca6ffeceda6b2ed87edd2b98d6d2a6a74f61f9df`
- dispatch baseline state: `HEAD == origin/main`, clean `main`
- immutable accepted M10 baseline:
  `23f5a8130334bf0c8516d51e9dcea38b92f50db1`
- accepted I2 implementation/evidence cut:
  `5429712de89a7e41c46cfd7fb4a39c4a492864c4`
- accepted I2 Canon/status integration cut:
  `bcb0f767edbb3e9e581c3b4c7f2a49e077f44067`
- accepted ALIGN-0 integration cut:
  `2f19810500b07d4b924b8201545dc2dc397c5f54`
- sole current roadmap: **Plan 250**
- completed goal: **ALIGN-0**
- sole active goal: **ALIGN-1**
- next goal after accepted ALIGN-1 close: **ALIGN-2**
- all other milestones: **ordered, inactive, and dependency-gated**
- Plan 247: closed M0--M10 execution record / regression baseline
- Plan 249: closed SYS-0--SYS-7 execution record / accepted I2 baseline
- official lifecycle at program start: theory **T1**; broad PHASE-I1
  **unaccepted**; official I2 entry and exit **accepted**; official I3
  **inactive**
- OPEN-032: **UNRESOLVED until I3-0 comparative evidence and an authorized
  selection ADR**

Program authorization and official lifecycle acceptance are distinct. Work in
the fixed program may proceed after ALIGN-0 authority/alignment close, but
official I3 entry is not claimed by program activation. I3-6 alone may propose
the ordered official transition `I3 entry accepted -> I3 exit accepted`, and
only after all preceding finite evidence passes independent review and an
authorized Canon acceptance record applies it.

Numbered plans, historical reports, Product Alpha, Full System V1, existing
WRKs, PrismCascade, Typed-Effect Wiring, and upper applications are not
parallel current queues. ADR-0014 remains the route outside this exact program.

## Source hierarchy and accepted inputs

Normative direction for this roadmap is read in the following order:

1. `mirrorea_canon/README.md`, `MAP.md`, `NORTH-STAR.md`, and
   `DESIGN-CONSTITUTION.md`;
2. `mirrorea_canon/architecture/01-strata.md` through
   `architecture/05-satellites.md`;
3. `mirrorea_canon/plan/00-gates.md`, `plan/01-phases.md`, and
   `plan/05-i3-entry-contract.md`;
4. ADR-0026, ADR-0028 through ADR-0033, and
   `mirrorea_canon/theory/11-metatheory-ledger.md`; and
5. the owner-authorized PROPOSAL-037 / ADR-0034 activation mirror when
   accepted.

LAB evidence is `progress.md`, `tasks.md`,
`.docs/progress-task-axes.md`, closed Plan 249, and milestone Reports
2598--2599. If LAB wording conflicts with Canon, Canon wins. The owner direction
supplies new program authority but does not silently rewrite an existing Canon
semantic invariant.

The accepted input pipeline is unchanged:

```text
ordinary checked source
  -> checked global Core
  -> owned per-locus executable artifacts
  -> generated CommunicationPlan / typed internal carrier
  -> transport-neutral adapter boundary
  -> remote locus runtime
```

Deployment may map an already checked logical locus to a process endpoint. It
may not invent an edge, operation, owner, authority, capability, witness,
failure, effect, state, occurrence, observation permission, or expected result.

## Program parent Goal Statement

**Goal ID:** MIRROREA-I3-DISTRIBUTED-FOUNDATION

**Goal sentence:** By the end of this program, the repository canonically
separates semantic strata, project/product responsibilities, and lifecycle
phases and fixes non-freezing trust boundaries, then executes accepted I2
per-locus artifacts and generated communication across at least two
operating-system processes through one selected reliable-stream adapter, with
typed network failure, source/Core/network/runtime correspondence,
observer-safe diagnostics, and finite C-distributed conformance, while public
wire and upper product interfaces remain unfrozen.

**Layer advanced:** Semantic strata: primarily S4 Projection/fabric execution,
while preserving S0 Surface, S1 Core, S2 Trace, S3 Verify, bounded S5 Domain
samples, and replaceable S6 Host boundaries. Project/product layer: advances
Mirrorea fabric toward the separately mapped Browser/Host and shared-space
responsibility boundaries without implementing those upper layers. Lifecycle:
owner-authorized bounded program over accepted I2; official I3 remains inactive
until the I3-6 acceptance decision and theory remains T1.

**North Star link:** Turns communication derived from one ordinary source into
actual inter-process communication while preserving correct placement,
verification, typed/redacted observation, and the future checked-evolution
boundary. Communication remains a projection of checked meaning rather than the
design origin.

**User-visible outcome:** From a fresh checkout, a bounded few-command workflow
can build/project ordinary source, start at least two local OS processes, run
remote owner operations and declared relation/designated-result paths, inject
representative network faults, inspect one observer-safe causal view, and run a
finite C-distributed conformance profile. Command names and encodings remain
internal/provisional unless separately authorized.

**Semantic invariants:** Owner mutation stays owner-local; transport, process,
address, session, stream, certificate, queue position, and retry token are not
authority; privacy/redaction is monotone; failures are typed and fail closed;
membership/capability/witness/fallback/consumption lineage never resurrects
through reconnect; retry and ambiguous delivery are explicit and never imply
exactly-once; Mir abstract order is justified by dependency/frontier/provenance,
not stream order; ordinary source and checked Core remain provenance authority;
internal carrier and future public wire remain separate.

**Direct consumer:** NEXT-0 records inactive I4 durability/live-patch and I5
Browser/Host/View entry contracts from an accepted finite I3 boundary. Those
future programs are not activated by this goal.

**Non-goals:** North Star change; broad PHASE-I1 acceptance; theory T2; final
Surface grammar, CLI, API, ABI, artifact, JSON, codec, or public wire freeze;
QUIC datagrams; hidden retry/exactly-once/distributed transaction; production or
WAN deployment; general network-order/fairness/liveness/security theorem;
durable distributed persistence; live distributed patch; browser product;
renderer; Domain Kits; Reversed Library implementation; PrismCascade or
Typed-Effect runtime collapse.

**Primary falsifier:** A distributed success depends on a handwritten or
source-free route, transport identity grants authority, a missing/revoked
lineage mutates owner state, a disconnect is reported as success or blindly
retried, stream/session order substitutes for Mir order, observer output leaks
protected material, or conformance passes without actual two-process generated
dispatch and bound positive/falsifier evidence.

**Exit evidence:** Accepted ALIGN-0--ALIGN-2 governance/trust contracts; an
authorized I3-0 transport-selection ADR based on comparative evidence; private
transport-neutral adapter and provisional encoding tests; actual two-or-more
process execution; the complete required failure/order matrix; SCN-01/02/03/06
C-distributed positive and falsifier executions; relation and designated-result
cross-process pressure; observer-safe joined trace; fresh-checkout workflow;
an exact finite downstream conformance row inventory; preserved I2/M10
regressions; exact `lean-proved` / `lean-stated` /
`model-checked-bounded` / `runtime-monitored` /
`intentionally-deferred` labels; independent semantic, security, concurrency,
and usability review; reproducible commands; and an authorized lifecycle
acceptance record.

**Stop condition:** Close only after I3-6 applies an evidence-backed official
I3 entry then exit and NEXT-0 records inactive I4/I5 entry contracts, closes
this program, and leaves no active roadmap or goal. Stop earlier for any
mandatory owner-reserved condition below. Reopen a closed milestone only for
its named falsifier, regression, or a direct-consumer counterexample.

## Fixed execution order and control transitions

A milestone may be added autonomously only when the parent goal cannot close
without it; Plan 250 must record the indispensable dependency, direct consumer,
and reason. Deletion, split, merge, or reordering requires owner direction.
Difficulty or an attractive adjacent feature is not authority.

```text
ALIGN-0 Baseline, parent goal, authority, and regression-floor alignment
  -> ALIGN-1 Three-axis semantic/product/lifecycle map
  -> ALIGN-2 Non-freezing trust and host-boundary contracts
  -> I3-0 Reliable-stream candidate comparison and selection
  -> I3-1 Transport-neutral adapter and private provisional encoding
  -> I3-2 Two-or-more-process generated-artifact execution
  -> I3-3 Network failure, retry, and ordering refinement
  -> I3-4 C-distributed scenarios and cross-process semantic pressure
  -> I3-5 Joined observer-safe devtools and fresh-checkout workflow
  -> I3-6 Finite I3 conformance and lifecycle closeout
  -> NEXT-0 Inactive I4/I5 entry contracts and program close
```

The control transition is fail-closed:

```text
active milestone accepted
  -> its one report is complete
  -> required validation and independent review pass
  -> accepted evidence/integration commit is pushed and remote parity checked
  -> roadmap/status pointers advance to exactly one next active milestone
```

No next milestone implementation begins before this transition. Planning,
tests, formal obligations, and source work may run concurrently only inside the
same active semantic candidate and with non-overlapping write ownership.

## Common milestone acceptance, report, and commit contract

Each milestone closes one bounded integration unit. The applicable subset must
include one accepted contract or semantic rule, one direct-consumer behavior,
one positive case, one representative falsifier, exact evidence classification,
source/implementation correspondence, focused validation, independent review,
and one milestone report. Inapplicable evidence and skipped commands are named
with reasons; they are never counted as pass.

Every subtask or research item records:

```text
Direct consumer:
Blocker reduced:
Acceptance use:
Alternative and falsifier:
Adoption/discard rule:
```

A WRK may be opened only when ADR-0014 and the current-milestone direct-consumer
rule both permit it. Historical WRKs do not become a program queue.

Reports use `docs/reports/TEMPLATE.md`, retain all 22 required report sections,
and are exactly one per milestone:

| Milestone | Report path | Status at program start |
| --- | --- | --- |
| ALIGN-0 | `docs/reports/2600-mirrorea-i3-distributed-foundation-align0-baseline-goal-alignment.md` | completed |
| ALIGN-1 | `docs/reports/2601-mirrorea-i3-distributed-foundation-align1-layer-map.md` | active report |
| ALIGN-2 | `docs/reports/2602-mirrorea-i3-distributed-foundation-align2-trust-boundaries.md` | inactive |
| I3-0 | `docs/reports/2603-mirrorea-i3-distributed-foundation-i3-0-transport-selection.md` | inactive |
| I3-1 | `docs/reports/2604-mirrorea-i3-distributed-foundation-i3-1-adapter-encoding.md` | inactive |
| I3-2 | `docs/reports/2605-mirrorea-i3-distributed-foundation-i3-2-two-process-runtime.md` | inactive |
| I3-3 | `docs/reports/2606-mirrorea-i3-distributed-foundation-i3-3-failure-ordering.md` | inactive |
| I3-4 | `docs/reports/2607-mirrorea-i3-distributed-foundation-i3-4-c-distributed-scenarios.md` | inactive |
| I3-5 | `docs/reports/2608-mirrorea-i3-distributed-foundation-i3-5-devtools-workflow.md` | inactive |
| I3-6 | `docs/reports/2609-mirrorea-i3-distributed-foundation-i3-6-conformance-lifecycle.md` | inactive |
| NEXT-0 | `docs/reports/2610-mirrorea-i3-distributed-foundation-next0-i4-i5-entry-contracts.md` | inactive |

Do not create separate reports for registration, evidence attachment, metadata,
pointer synchronization, agent configuration, or commit/push. Material
counterevidence after close is recorded forward; a closed report is not
overwritten.

Commit boundaries are milestone boundaries:

1. Source/test/formal work may use reviewable commits inside the active
   milestone, but may not mix a later milestone.
2. The accepted implementation/evidence cut, when applicable, is pinned before
   lifecycle or status acceptance.
3. One milestone integration commit contains the report, roadmap pointer,
   required Canon/status/docs synchronization, and acceptance metadata.
4. Each commit is pushed non-interactively; remote branch parity is checked
   before the next milestone becomes active.
5. If a mechanical pointer-only successor commit is required because a commit
   cannot contain its own hash, it remains part of the same milestone and does
   not get another report.

The parent/orchestrator owns integration, acceptance, commit, push, and remote
parity. Production source has one writer by default. Tests/formal evidence have
separate bounded owners where practical. A change author does not act as the
sole independent semantic/correctness reviewer.

Before heavy builds or generated artifacts, run the repository resource audit
required by AGENTS.md and use the configured external workdir when applicable.
No cleanup deletes repository source or an unconfirmed directory.

## ALIGN-0 detailed Goal Statement — completed

**Goal ID:** ALIGN-0

**Goal sentence:** By the end of this milestone, the accepted I2 baseline is
preserved and the repository has one owner-authorized I3 Distributed Foundation
parent goal, one current roadmap, one active goal, one regression floor, and one
meta-drift control matrix that can advance safely to ALIGN-1.

**Layer advanced:** Semantic strata: no semantic layer behavior changes;
alignment preserves S0--S6 boundaries. Project/product layer: no product layer
is implemented or collapsed; ALIGN-1 receives the mapping question. Lifecycle:
activates only the bounded program and ALIGN-0 execution control; theory T1,
broad PHASE-I1 unaccepted, official I2 exit accepted, and official I3 inactive
remain distinct.

**North Star link:** Establishes the sole control path for advancing accepted
meaning-derived in-process communication toward verified, observable
inter-process communication without changing the five project verbs or making
transport the source of semantics.

**User-visible outcome:** A new reader can locate the owner authority, accepted
I2/M10 cuts, sole roadmap, active goal, fixed milestone sequence, current
blocker, regression commands, evidence classes, mandatory stops, and the next
ALIGN-1 consumer without reconstructing historical reports.

**Semantic invariants:** Canon > LAB; M10 and I2 accepted cuts remain immutable
inputs; ordinary source and checked Core remain semantic/provenance authority;
owner/authority/privacy/failure/lifetime/ordering/redaction guarantees are not
weakened; transport/session/certificate stays non-authoritative; no hidden
retry/transaction/exactly-once; no stale resurrection; public contracts remain
unfrozen; Mir, Mirrorea, PrismCascade, Typed-Effect Wiring, Browser/Host,
shared-space, Domain Kits/apps, and Reversed Library remain separable.

**Direct consumer:** ALIGN-1 uses the aligned authority, baseline, and drift
matrix to record the canonical three-axis semantic-strata, project/product-layer,
and lifecycle-phase map without changing existing S0--S6 meanings.

**Non-goals:** Source, test, Lean, model, sample, runtime, transport, CLI, API,
ABI, wire, deployment, product, or lifecycle acceptance changes; OPEN-032
selection; public freeze; upper-layer API invention; North Star revision.

**Primary falsifier:** More than one roadmap or active goal remains current; a
status file calls Plan 249 current; program activation is reported as official
I3 entry/exit or broad I1 acceptance; the accepted I2 regression floor fails;
or the drift matrix omits a source hierarchy, layer, lifecycle, evidence,
authority, failure/order, public-freeze, or subsystem-separation axis.

**Exit evidence:** PROPOSAL-037 and ADR-0034 accepted through the Canon process;
Plan 250 is the unique current LAB roadmap; Plan 249 is named only as a closed
baseline; current goal pointers say exactly ALIGN-0 before close and advance
exactly to ALIGN-1 in the close integration; the meta-drift matrix below is
complete; fresh I2/M10 regression and docs/hierarchy validation pass; no
production behavior changes; an independent pre-edit Canon-first planning
review and final close review find no unresolved P0/P1; any P2 has an explicit
disposition and is resolved when it materially affects acceptance. Report 2600 records
commands, results, skipped checks, dirty state, commit/push, and sub-agent close
state.

The ALIGN-0 command floor is:

```bash
df -h .
free -h
git rev-parse HEAD
git rev-parse origin/main
git status --short --branch
git merge-base --is-ancestor 23f5a8130334bf0c8516d51e9dcea38b92f50db1 HEAD
git merge-base --is-ancestor 5429712de89a7e41c46cfd7fb4a39c4a492864c4 HEAD
cargo test -p mir-runtime --lib sys6_i2_conformance_tests
cargo test -p mir-runtime --test sys6_i2_cli
cargo test -p mir-runtime --test m10_conformance
cargo test -p mir-runtime --test m10_cli
(cd mirrorea_canon && python3 meta/build-index.py)
(cd mirrorea_canon && python3 meta/build-index.py --check)
python3 -m unittest scripts.tests.test_build_index -v
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html -v
python3 scripts/validate_agent_configs.py
python3 -m unittest scripts.tests.test_validate_agent_configs -v
codex --strict-config -C . --help
make docs
git diff --check
```

ALIGN-0 changes no Rust, Lean, model, sample, or runtime contract. Workspace,
SYS-2--5 exhaustive suites, format, Clippy, Lean, and bounded-model expansion
are therefore recorded as not run rather than represented as ALIGN-0 passes.
A current equivalent may replace a command only when the report proves that it
covers the same accepted regression family.

**Stop condition:** Close when authority is durably mirrored, the source and
status hierarchy has exactly one roadmap/goal, I2/M10 regressions and document
validation pass, the drift matrix has no unexplained conflict, and ALIGN-1 has
an exact direct-consumer contract. Reopen for pointer conflict, missing owner
authority, regression, lifecycle overclaim, hierarchy drift, or an independent
review counterexample.

### ALIGN-0 owner-intent meta-drift matrix

| Topic | Owner intent | Current Canon | Current implementation | Risk / gap | This program consumer |
| --- | --- | --- | --- | --- | --- |
| Meaning-derived communication | communication follows checked owner/dependency/effect/failure meaning, never a handwritten interface | architecture/03--04 and ADR-0029/0030 require projection-derived `CommunicationPlan` and carrier | ordinary source projects to 12 generated edges; SYS-4 dispatches only generated routes in-process | a socket facade or conformance fixture could invent routes/meaning | I3-1 checked mapping; I3-2 generated-plan-only deployment; I3-4 gates |
| Per-locus code | each process runs only artifacts assigned to its logical loci | architecture/03--04 fixes checked-Core projection and artifact identity | SYS-3 emits four owned locus programs from the canonical I2 source | process bootstrap could reparse source, select fixtures, or run the global program | I3-2 deployment/runtime; I3-6 correspondence rows |
| Owner evaluation | the semantic owner performs authoritative read-modify-write and designated work | theory/13, ADR-0028--0031, and plan/05 preserve owner/authority separation | SYS-4/5 perform owner-side mutation and selected designated evaluation in-process | requester precomputation or direct remote store could move authority | I3-2 remote serve; I3-4 SCN-01/02 and designated slice |
| Maintained relation | owner-published relation lineage crosses loci; consumer performs only local projection | theory/14 and accepted two-anchor profile preserve primary/fallback lineage and consumer-local projection | SYS-5 publishes `bird_follow` relation evidence to ViewerC without an absolute-pose stream | sample vocabulary could harden into general relation semantics or provider could become owner | I3-4 relation pressure slice; I3-5 joined view |
| Authentication / verification | membership, capability, witness, and verdict authorize; transport identity does not | theory/05, M9 contracts, BND-004, and plan/05 require revalidation and no stale resurrection | sealed admission and runtime revalidation exist for the local I2 carrier | certificate/session/reconnect/package identity could mint or resurrect authority | ALIGN-2 trust gates; I3-1 admission; I3-3 stale/revoked faults; SCN-03 |
| Memory / ordering | Mir dependency and linearization edges refine concrete execution; byte/stream order is insufficient | theory/04, ADR-0028, and plan/05 fix request/serve/result/receipt and grant/revoke/use edges | ST and selected OW1 plus bounded ordering evidence are accepted in-process | reconnect/cross-stream order, retry, ambiguity, cut traffic, and clocks are not implemented | I3-1 carrier fields; I3-3 full ordering/fault refinement; I3-6 classification |
| Real transport | choose one reliable-stream adapter after equal executable evidence; keep it replaceable/non-authoritative | ADR-0033/0034 and plan/05 retain TLS-over-TCP and QUIC reliable streams only | accepted path has only SYS-4 in-process outbox/inbox; unrelated alpha TCP is LAB-only | no selected library, framing, actual process boundary, or common canary yet | I3-0 selection; I3-1 adapter; I3-2 process runtime |
| Browser/Host package admission | checked third-party packages are admitted separately from grants and resource policy | current Canon forbids upper-layer collapse; detailed responsibility contract is intentionally absent until ALIGN-2 | no accepted package-admission/browser runtime exists; current CLI accepts local source as a development workflow | package authenticity could be mistaken for mutation authority; package/API format could freeze early | ALIGN-1 placement; ALIGN-2 admission/resource contracts; future inactive I5 |
| Typed FFI / provider | untrusted package -> typed effect -> policy/capability/resource check -> trusted adapter/provider; raw FFI is privileged | BND-007/008 and Design Constitution keep host effects typed and providers non-authoritative; detailed tiers remain a gap | historical provider/engine paths are LAB and outside accepted I2; no normal-package raw FFI path is accepted | provider/native pointer could bypass authority, crash isolation, redaction, or revocation | ALIGN-2 provider/native-tier contract; I3 constraint; future inactive I5 |
| View input | View may compute presentation locally but returns typed commands/effect requests; no direct semantic store | architecture/02 keeps View/provider from semantic ownership; reverse input contract is not yet explicit | SYS-5 observer-safe joined view is evidence-only; no accepted device/input bridge exists | renderer could mutate state or acquire authority from host/session identity | ALIGN-2 input boundary; I3 non-interference constraint; future inactive I5 |
| Shared-Space / World-Web horizon | retain a separate persistent participatory platform layer without fixing World/URL/Portal vocabulary | architecture/01/05 excludes domain vocabulary from Core; the precise PL position is not yet Canon | only bounded sample/library vocabulary and historical upper consumers exist | upper-product addressing/discovery/governance could leak into Core or I3 wire | ALIGN-1 responsibility-only PL-4; NEXT-0 I5 horizon only |
| Reversed Library separation | treat it as a distinct upper application/project, never a Mirrorea completion condition | architecture/05 keeps upper applications and satellites separable | no accepted I2 or planned I3 source depends on a Reversed Library product | project completion or domain vocabulary could be pulled into the fabric gate | ALIGN-1 PL-6 separation; NEXT-0 records no activation |
| Three-axis numbering | preserve semantic strata separately from PL-0--6 and T/I lifecycle; do not silently rename S numbers | architecture/01 currently says primary S0--S5 plus parenthesized `(S6 Host)` and also records a legacy realized S0--S7 path; MAP abbreviates architecture as S0--S7 | accepted I2 code is organized by SYS milestones/modules, not a product-layer numbering API | calling S0--S6 already uniform would hide a real Canon reader drift and could make Host maturity imply semantic maturity | ALIGN-1 must reconcile and state the canonical three-axis reading without changing settled semantics by accident |
| BND-007 View wording | View/provider has no authoritative domain semantics but may perform presentation-local computation | architecture/02 currently abbreviates BND-007 as “View has no logic,” while Design Constitution keeps semantic ownership below the provider boundary | SYS-5 exposes observer-safe evidence only; historical renderer/provider code is outside accepted I2 | literal no-logic wording forbids valid animation/interpolation/IK/local relation evaluation or encourages undocumented bypasses | ALIGN-2 explicitly clarifies BND-007 and the typed input reverse path |
| Conformance/runtime direction | runtime capability produces evidence; finite conformance never controls semantics or lifecycle | architecture/03 and ADR-0032 require a downstream-only verifier | SYS-6 consumes SYS-2--5 evidence and passes 22/22; lower layers do not import SYS-6 | release/hash/report machinery could dominate runtime design or self-authorize I3 | every implementation milestone; fixed downstream I3-6 verifier |
| Current authority / lifecycle | Canon is normative; Plan 250 is the sole LAB roadmap; only one goal is active | PROPOSAL-037/ADR-0034 activate the program but not official I3 lifecycle | no runtime change at ALIGN-0; accepted I2 cut remains the executable floor | stale no-program text, extra queue, or milestone progress could be called lifecycle/proof | ALIGN-1 entry and every close transition |

### ALIGN-0 implementation concentration and risk inventory

| Accepted implementation surface | Size at start | Current responsibility | Main risk / preserved seam | Direct consumer |
| --- | ---: | --- | --- | --- |
| `sys4_dispatch.rs` | 13,061 LOC | generated-route materialization, mailboxes, M9 revalidation, state, occurrence causality, cut/patch | highest blast radius; I3 must wrap/refine the transport seam rather than let network or SYS-6 control this kernel | I3-1/I3-2 |
| `sys5_local_slice.rs` | 6,867 LOC | build/project facade, source-derived admission, four-locus workflow, joined evidence | keep canonical I2 local regression intact; do not turn sample vocabulary or CLI shape into public semantics | I3-2/I3-4/I3-5 |
| `sys6_i2_conformance.rs` | 5,350 LOC | fixed 22-row downstream evidence producer/verifier | must stay downstream-only and remain an I2 regression, not become I3 runtime architecture | I3-6 design and regression floor |
| `semantic_runtime_kernel.rs` | 3,763 LOC | sealed semantic carrier and M8 execution seam | preserve source/Core provenance, receipt non-authority, and M9-before-M8 ordering | I3-1 carrier admission |
| `sys3_projection/model.rs` | 3,653 LOC | artifact, communication, effect, observation, persistence, and source-map contracts | extend only from checked identities; no handwritten process route or public-wire freeze | I3-1/I3-2 |

Existing `alpha_network_runtime.rs` and renderer/provider modules are LAB-only
paths outside the accepted SYS-1--SYS-6 direction. They may supply bounded
counterexamples or implementation techniques but cannot be promoted as I3
semantics, authority, or evidence without the current milestone gates.

### ALIGN-0 Host/View responsibility-gap inventory

| Gap at accepted I2 | Preserved boundary now | First direct consumer |
| --- | --- | --- |
| no package admission or package-instance lifecycle | package identity/provenance, checking, admission verdict, authority grant, allocation, update/revocation remain separate | ALIGN-2 |
| no accepted Browser/Host resource sandbox | CPU/time, memory, storage namespace/quota, effect/network rate, device access, abuse termination stay responsibility requirements, not chosen technology | ALIGN-2; inactive I5 |
| no accepted typed input reverse path | observer-safe projection cannot authorize direct store mutation; input must return as a typed command with principal/capability | ALIGN-2; inactive I5 |
| no accepted provider/FFI trust tiers | ordinary checked packages have no raw-native path; trusted adapter and privileged native provider are separate claims | ALIGN-2; inactive I5 |
| no real transport/process deployment | logical locus mapping may choose endpoint placement but never communication edges, Core, authority, failure, or expected result | I3-0--I3-2 |
| no public View/devtools contract | SYS-5 JSON is provisional observer-safe evidence; presentation-local computation is allowed but authoritative domain semantics remains in Mir | ALIGN-2/I3-5; inactive I5 |
| no Shared-Space/product platform semantics | record PL position, lower requirements, upper promises, non-primitives, and deferred questions only | ALIGN-1; inactive future owner program |

## ALIGN-1 milestone contract — active

**Goal:** Record one canonical three-axis map that keeps semantic strata,
project/product responsibility layers, and lifecycle phases distinct.

**Entry:** ALIGN-0 accepted, Report 2600 complete, integration pushed, remote
parity confirmed, and current goal advanced to ALIGN-1.

**Required content:**

- add a separate Canon architecture document for project/product layers; do
  not rename or reuse `architecture/01` semantic-strata numbers;
- semantic strata remain exactly S0 Surface, S1 Core, S2 Trace, S3 Verify,
  S4 Projection, S5 Domain, and S6 Host;
- project/product responsibility layers are recorded separately as PL-0
  physical host, PL-1 Mir language/kernel, PL-2 Mirrorea fabric, PL-3 safe
  Browser/Host participant, PL-4 Shared-Space/World-Web responsibility-only,
  PL-5 Domain Kits/applications, and PL-6 Reversed Library as a separate
  application; and
- T0--T2 and I1--I6 remain lifecycle phases, not architecture or product
  layers.

For every PL-0--PL-6 row, record responsibility, admitted input, produced
output, prohibited ownership/dependency flow, and current maturity. The PL-4
Shared-Space/World-Web boundary records only position in the stack, lower-layer
requirements, upper-layer promises, non-primitives, deferred questions, and
owner-clarification points for a future program. It must not choose addressing,
linking, discovery, publication, federation, or governance.

The PL map is responsibility-only. It freezes no package, protocol, API, ABI,
wire, deployment, browser engine, domain kit, or application design.

**Direct consumer:** ALIGN-2 binds trust and host boundaries to the accepted
three-axis map; I3-0 later uses it to keep transport in PL-2/S4 realization
rather than treating it as source semantics or an upper product API.

**Primary falsifier:** A map renumbers or changes S0--S6 semantics, treats a
phase as a layer, makes PL-4 a final shared-space API/catalog, folds
Browser/Host/provider/domain/application semantics into Mir Core, or makes
Reversed Library the architecture of lower layers.

**Exit and validation:** Canon proposal/decision/index/changelog updates needed
for a canonical map; exact cross-reference table; contradiction scan against
architecture/01 and plan/01; docs hierarchy/index/HTML validation; no source or
runtime behavior delta; independent architecture/semantic review; Report 2601;
one ALIGN-1 integration commit/push/parity; roadmap/status advance exactly to
ALIGN-2.

If the owner-fixed Canon cut already contains the complete three-axis map, a
literal coverage/contradiction audit with no production or normative source
delta is a valid close path. The report, independent ACCEPT, validation, and
roadmap/status integration remain required; do not manufacture a source edit.

**Stop/reopen:** Stop for a North Star change, semantic-strata change outside
the owner direction, domain/provider/app collapse, or any irreversible public
surface. Reopen for a cross-axis ambiguity that permits two different owners or
lifecycle readings for the same responsibility.

## ALIGN-2 milestone contract — inactive until ALIGN-1 closes

**Goal:** Define non-freezing trust contracts and resource/sandbox boundaries
for packages, Browser/Host participation, View/renderer separation, typed
input/effects, providers, and privileged native integration.

**Entry:** Accepted ALIGN-1 canonical three-axis map, Report 2601, pushed
integration, remote parity, and current goal ALIGN-2.

**Required trust tiers:** These are **trust tiers**, not Theory T0--T2 phases.

| Trust tier | Responsibility | Non-authority / non-freeze boundary |
| --- | --- | --- |
| T0 | trusted Mir kernel and checker/runtime base | smallest trusted computing base; does not make host identity semantic authority |
| T1 | checked untrusted Mir package | enters only through package-to-admission checks; package identity alone grants nothing |
| T2 | sandboxed external provider process | typed effect boundary, declared resource/failure policy, no ambient Mir authority |
| T3 | privileged native plugin | separate explicit high-risk tier with least privilege, review, and revocation; never the default provider path |
| T4 | host browser, engine, or operating system | supplies host facilities under adapters; host/process/session identity is not Mir authority |

**Required non-freezing contracts:** package -> admission; browser runtime ->
Mirrorea fabric; runtime -> observer-safe View; View -> renderer; typed input ->
Mir command; typed effect -> provider; privileged raw FFI as a separate tier;
and explicit resource/sandbox limits and typed failures. BND-007 is clarified
without weakening it: View owns no authoritative domain semantics or mutation,
but may perform presentation-local computation such as admitted coordinate
conversion, interpolation, or rendering preparation that cannot change
semantic owner state, authority, lineage, fallback, or history.

Each edge records its required input, output/verdict, validation owner,
authority/non-authority consequence, typed failure, revocation/termination
path, observer/redaction consequence, and explicit non-freeze. Package
admission includes identity/provenance, source/content identity,
parse/check/elaborate, verification/residual, requested
capabilities/effects/resources, and verdict. Browser-to-fabric keeps package
instance, locus allocation, grant, storage namespace, termination/update, and
revocation distinct. View projection carries observer principal, safe
state/relation, visibility/redaction, presentation frontier, semantic version,
and source/reason refs. Input carries device event, typed command, principal,
capability, handler, and semantic transition/failure. Provider requests carry
typed effect, policy/capability/resource admission, typed result/failure, and
provenance. Privileged native plugins retain process/crash isolation,
revocation, publisher/provenance, and resource/data-access responsibility.

The resource/sandbox checklist explicitly covers CPU/time budget, memory
budget, storage namespace/quota, effect/network rate, device access, infinite
loop/allocation abuse, observation abuse, and termination. ALIGN-2 chooses no
sandbox technology, UI, package format, or final plugin/FFI ABI.

**Direct consumer:** I3-0 uses the trust/host constraints when comparing
libraries and adapters; I3-1 uses them for private encoding, resource limits,
logs, and process boundaries; I3-5 uses the View/renderer boundary for safe
joined output.

**Primary falsifier:** Package signature, browser origin, process, provider,
plugin, renderer, certificate, or transport session becomes a Mir grant; View
re-decides authoritative domain meaning; raw FFI is silently available to T1
or T2; a resource limit fails open; or the contract freezes a public API/ABI.

**Exit and validation:** Accepted normative contract through the Canon process;
positive and denial-flow responsibility matrix for every edge/tier; least-
privilege, revocation, redaction, resource-exhaustion, and provider-failure
falsifiers; architecture/02 BND consistency review; docs/index/hierarchy/HTML
validation; independent security/semantics review; Report 2602; one ALIGN-2
integration commit/push/parity; roadmap/status advance exactly to I3-0.

If the owner-fixed Canon cut already contains every required tier, contract,
non-authority rule, and BND-007 clarification, a literal coverage/falsifier
audit with no production or normative source delta is a valid close path. The
report, independent ACCEPT, validation, and roadmap/status integration remain
required; do not manufacture a source edit.

**Stop/reopen:** Stop for a guarantee weakening, ambiguous privileged boundary,
unavoidable raw-secret exposure, or required public freeze. Reopen if an I3-0
candidate cannot satisfy the tier/contract model without transport-as-authority.

## I3-0 milestone contract — inactive until ALIGN-2 closes

**Goal:** Compare exactly the two retained reliable-stream candidates under one
common semantic/failure/order test harness and select at most one through an
authorized ADR without freezing a public wire.

**Entry:** ALIGN-2 accepted, Report 2602, pushed parity, current goal I3-0;
Candidate A TLS-over-TCP framed reliable stream and Candidate B QUIC reliable
stream remain UNSELECTED; QUIC datagrams remain excluded.

**Comparison rule:** Apply the Design Constitution lexicographically: meaning
preservation; authority/privacy/safety; ordinary Surface unchanged; explicit
communication/failure/effect; small orthogonal Core; determinism and
inspectability; finite decidability; modular proof/model/runtime evidence;
conservative extensibility; implementation simplicity; and performance. Under
those criteria, compare deterministic CI behavior, fail-closed framing/stream
fault behavior, library maintenance/security fit, supported-platform evidence,
future Browser/Host feasibility, resource use, and performance. A lower-ranked
advantage cannot compensate for a higher-ranked semantic or safety failure.

The exact supported OS/CI and browser-feasibility evidence matrix is decided
and recorded in I3-0 before selection. Missing coverage is an explicit residual,
not an assumed pass.

**Direct consumer:** I3-1 implements a transport-neutral adapter and private
provisional encoding against the selected candidate while retaining a
conservative replacement seam.

**Primary falsifier:** Candidate-specific metadata changes semantic identity or
authority; the common fault/order harness is not behaviorally comparable; a
candidate cannot fail closed deterministically; a selection relies only on
performance/convenience; or both candidates fail a mandatory plan/05 gate.

**Exit and validation:** Both A and B run an equal private source/Core-bound
carrier canary across at least two actual OS processes under reproducible
same-gate positive/falsifier experiments; dependency/security/license and
supported-platform evidence; deterministic CI and resource/performance
measurements classified below semantic safety; explicit alternative rejection
rationale and replacement boundary; independent semantic/security/portability
review; authorized transport-selection ADR resolving OPEN-032 only for this
bounded program; Report 2603; one I3-0 integration commit/push/parity; advance
exactly to I3-1. No candidate is selected by this roadmap text.

**Stop/reopen:** Mandatory stop if both candidates fail the contract or if they
are tied on an irreversible, externally observable, non-migratable semantic
choice that the Constitution cannot order. Reopen for a selected-candidate
security/semantic counterexample or loss of the required supported-platform
floor before I3-1 acceptance.

## I3-1 milestone contract — inactive until I3-0 closes

**Goal:** Implement a private, provisional, transport-neutral adapter/encoding
boundary that round-trips the accepted internal semantic carrier and fails
closed on version, size, framing, provenance, and decoding faults.

**Entry:** I3-0 selection ADR accepted, Report 2603 complete, pushed parity,
current goal I3-1, and the unselected alternative retained as comparison and
replacement evidence rather than a second implementation queue.

**Required boundary:** Internal carrier -> private versioned encoding ->
adapter bytes/reliable stream -> checked decoding/admission -> internal carrier.
Round-trip evidence covers the applicable source/Core/program/artifact/edge,
request and occurrence identities, owner/origin/target, membership epoch and
incarnation, capability/witness references, effect/failure rows, visibility and
redaction, frontier/version/publication/consumption lineage, and declared
limits. Logs expose observer-safe references and typed reasons, never raw
credentials, capability/witness material, private payload/state, or host paths.
Request, serve, result, and receipt remain distinct semantic messages/states;
encoding may not collapse them. The private version policy states separately
how unknown versions and unknown fields reject, ignore, or preserve data; no
policy may default required semantic meaning.

**Direct consumer:** I3-2 uses only this adapter boundary to place accepted
generated endpoints in distinct OS processes.

**Primary falsifier:** Encode/decode adds, drops, aliases, or defaults semantic
meaning; unknown version, malformed/truncated/oversized frame, duplicate field,
or limit violation reaches admission; logs leak protected values; or a private
field/layout becomes a public compatibility promise.

**Exit and validation:** Exact semantic-field round-trip and mutation corpus;
canonical/deterministic private encoding evidence where required for identity;
partial read/write, malformed, truncated, oversized, unknown-version,
resource-limit, provenance, redaction, and secret-scan negatives; selected-
candidate adapter tests; property/fuzz decode tests over round-trip,
truncation, malformed length, oversize, and unknown fields/versions; preserved
I2/M10 regression; independent codec/
security/semantics review; Report 2604; accepted source/evidence and integration
commits pushed with parity; advance exactly to I3-2.

**Stop/reopen:** Stop if lossless mapping requires a public freeze or carrier
meaning change outside the program. Reopen for ambiguous decoding, allocation
before limit checks, semantic defaulting, secret leakage, or replacement-seam
failure.

## I3-2 milestone contract — inactive until I3-1 closes

**Goal:** Execute accepted generated artifacts and communication across at
least two actual OS processes, with deployment limited to logical-locus-to-
endpoint mapping and with remote owner service/result behavior observable.

**Entry:** I3-1 adapter/encoding accepted, Report 2604, pushed parity, current
goal I3-2, and no unresolved I3-1 semantic/security finding.

**Required execution:** The launcher starts clean independent processes from
checked per-locus artifacts, maps each declared logical locus to an endpoint,
connects only generated edges, performs remote request/admission/owner serve/
typed result or receipt, and terminates/cleans up deterministically. Neither
launcher nor deployment metadata supplies Core, routes absent from the plan,
authority, state, arguments, expected results, or semantic occurrence IDs.

**Direct consumer:** I3-3 injects the complete network failure and ordering
matrix into this actual multi-process route.

**Primary falsifier:** A success path reparses source at runtime, chooses a
fixture/expected result, hand-writes an edge, uses a shared cross-process store,
mutates a non-owner, mints authority from endpoint/session identity, executes in
one process while claiming distributed evidence, or leaves nondeterministic
orphan processes/resources.

**Exit and validation:** Actual two-or-more-process positive execution; process
identity and address non-authority negatives; missing/extra/retargeted edge and
direct-remote-store falsifiers; remote owner mutation and typed result lineage;
deterministic replay at the declared bounded scheduler profile; clean startup,
shutdown, port/resource cleanup, and fresh-run isolation; source/Core/artifact/
edge/carrier/network/runtime correlation; preserved I2/M10 regression;
independent runtime/concurrency/security review; Report 2605; accepted
source/evidence and integration commits pushed with parity; advance exactly to
I3-3.

**Stop/reopen:** Stop for unavoidable manual communication, authority collapse,
hidden shared state, unsafe cleanup, or production/deployment requirement.
Reopen for a reproducible route/owner/provenance mismatch or process boundary
that the I3-3 fault harness cannot control deterministically.

## I3-3 milestone contract — inactive until I3-2 closes

**Goal:** Close the required network failure, explicit retry/ambiguity, and Mir
ordering-refinement boundary on the actual multi-process runtime without stale
resurrection or exactly-once overclaim.

**Entry:** I3-2 multi-process route accepted, Report 2605, pushed parity,
current goal I3-3, and fault injection controls cannot supply semantic facts.

**Required failure matrix:** Every row needs a positive/control path, a typed
negative outcome, no forbidden semantic mutation, exact request/provenance
binding, and bounded termination.

| Condition family | Required result / invariant |
| --- | --- |
| undeclared route, partition, absent/refused endpoint | explicit unavailable/route failure; no hang, drop, or route invention |
| handshake or peer admission failure | typed pre-semantic rejection; peer/certificate is not authority |
| wrong target locus/owner/artifact/operation | reject before owner mutation |
| source/Core/artifact/carrier mismatch | fail closed; no source-free operation/state |
| partial read/write or split frame | buffer one checked frame or reject incomplete input; no partial request |
| malformed, truncated, oversized, or wrong-version frame | typed rejection before carrier admission and bounded allocation |
| disconnect before remote admission | unavailable/cancelled with no remote mutation |
| disconnect after admission before result/receipt | explicit ambiguous-delivery state bound to the original request |
| reconnect, new connection, or migration | new non-authoritative session; old request and grant lineage revalidated |
| duplicate request | operation-specific stored-result/no-new-consume or typed duplicate rejection; no second mutation |
| duplicate/stale result or receipt | typed duplicate/stale rejection or exact already-decided observation |
| reorder across streams/connections/control-data paths | dependency buffering or typed stale/order rejection; stream order is insufficient |
| stale membership/epoch/incarnation | reject before mutation; retirement remains monotone |
| missing/revoked capability or witness | declared authority failure; reconnect does not renew authority |
| unavailable/rejected/revoked auth/policy layer | typed failure before activation/use |
| backpressure/queue capacity | typed bounded outcome; no silent loss or hidden unbounded buffer |
| timeout/lease/clock advance | explicit external-time/lease outcome; schedule creates no facts |
| external effect/provider failure | declared effect failure; provider remains non-authoritative |
| visibility/redaction mismatch | deny or redact without credential/private-state leak |
| patch activation or cut/save with in-flight traffic | explicit quiescence/admission rule or typed rejection; no stale post-boundary traffic |

Retry records initiator, reason, authorization, original request identity, and
whether the operation returns a stored decision or rejects duplication.
Ambiguous delivery is not silently retried or reported as success.

**Required ordering refinement:** Concrete occurrences must justify local
eligibility -> request -> send admission -> transmission -> complete receive ->
carrier admission and authority revalidation -> owner/effect linearization ->
typed result/failure send -> result receive -> receipt/consume. Preserve owner
coherence, request->serve, send->receive, serve->result, result->receipt/consume,
publish->observe, witness-create->use, grant/revoke->use, membership-update->
dispatch, verdict->activation, fallback->later-access, and cut/quiescence->later
transition. Late old-session traffic is rejected or placed by an explicit
current dependency.

**Direct consumer:** I3-4 runs frozen C-distributed scenarios and added
cross-process semantic pressure only after this failure/order contract is
executable.

**Primary falsifier:** Blind retry, false success, second mutation, unbounded
hang/buffer, stale lineage resurrection, stream-order-as-Mir-order, missing
linearization/provenance, or a typed network failure that mutates semantic state.

**Exit and validation:** Executed matrix with deterministic fault controls;
bounded scheduler/order model where used and exactly classified; retry/
ambiguous-delivery/duplicate identity tests; stale membership/capability/
witness and late-session negatives; trace/order correspondence; preserved
I2/M10 regression; independent concurrency, distributed-systems, semantic, and
security review; Report 2606; accepted source/evidence and integration commits
pushed with parity; advance exactly to I3-4.

**Stop/reopen:** Stop if the selected transport cannot expose a required
failure/order distinction without changing Mir semantics, or if safe progress
requires hidden retry/exactly-once. Reopen for any failure-atomicity,
linearization, stale-resurrection, or bounded-termination counterexample.

## I3-4 milestone contract — inactive until I3-3 closes

**Goal:** Produce actual C-distributed positive and representative falsifier
evidence for frozen SCN-01, SCN-02, SCN-03, and SCN-06, plus bounded maintained-
relation and designated-result cross-process pressure, without theorem or
product overclaim.

**Entry:** I3-3 failure/order boundary accepted, Report 2606, pushed parity,
current goal I3-4, and all scenario execution uses generated artifacts and the
actual multi-process adapter.

**Required scenario evidence:**

- SCN-01: requester and owner in distinct processes; source lineage reaches
  request/serve/write/publish/observe; route/visibility faults do not hang,
  mutate, or leak.
- SCN-02: owner performs same-owner RMW remotely; two accepted requests
  serialize `100 -> 90 -> 80`; requester-side precomputation, revoked/missing
  capability mutation, and ambiguous-delivery double application fail.
- SCN-03: remote admission verdict plus epoch/incarnation/grant dominates use;
  pre-verdict write, replayed capability, reconnect, and certificate-as-
  authority fail.
- SCN-06: missing/partitioned route returns explicit `RouteUnavailable`-family
  failure within the finite turn budget, and a later admitted route can serve
  the same ordinary source without transport-created semantics.
- maintained-relation pressure: owner publication, primary/fallback lineage,
  consumer-local late projection, presentation-gap nonmutation, and redaction
  survive the process boundary; no bird/object absolute-pose stream is
  generated as a substitute for the relation.
- designated-result pressure: source-owner input, designated evaluation,
  named consumer delivery, version/frontier/policy, exact retry/no-new-consume,
  and competing-consumer rejection survive the process boundary.

The last two are bounded pressure cases, not changes to frozen SCN expectations
and not new Core/domain primitives.

Before I3-5 starts, I3-4 must already emit the minimum observer-safe trace and
typed diagnostic evidence required by every accepted C-distributed gate and
pressure case. I3-5 may join and present only those existing facts; it cannot
invent missing gate evidence, occurrences, diagnostics, or provenance.

**Direct consumer:** I3-5 joins the accepted source/Core/artifact/network/
runtime scenario evidence into an observer-safe workflow.

**Primary falsifier:** A scenario uses a handwritten interface, helper-only or
single-process substitute, expected-result lookup, direct remote store, moved
owner, unbound negative case, secret-bearing trace, or bounded evidence labeled
as a general theorem.

**Exit and validation:** Actual two-process positive and fault executions for
all four SCNs and both pressure cases; source/Core/artifact/carrier/network/
runtime correspondence; exact evidence classes; frozen expectation comparison;
deterministic rerun; preserved I2/M10 regression; independent semantic,
authority/security, concurrency, and conformance review; Report 2607; accepted
source/evidence and integration commits pushed with parity; advance exactly to
I3-5.

**Stop/reopen:** Stop if satisfying a frozen scenario requires weakening it or
adding a domain primitive/public interface. Reopen for an unexercised required
negative, missing provenance join, owner movement, duplicate mutation, or
evidence-class overclaim.

## I3-5 milestone contract — inactive until I3-4 closes

**Goal:** Expose one observer-safe source-to-network causal view and one
few-command fresh-checkout workflow that reproduces the accepted distributed
capability without turning helper output into a public interface.

**Entry:** I3-4 scenario evidence accepted, Report 2607, pushed parity, current
goal I3-5, and observer inputs are existing typed facts rather than raw runtime
memory or reconstructed expected results.

**Required joined view:** Correlate source span -> checked Core -> locus
artifact -> generated edge -> semantic request identity -> process/locus
deployment mapping -> internal carrier -> private encoding/frame -> adapter
send occurrence -> distinct network frame/stream occurrence -> complete
receive/decode -> admission/authority revalidation -> owner/effect serve
linearization -> typed result/failure -> receipt/designated consume -> observer
projection. Preserve distinct occurrence IDs, causal edges,
linearization/frontier/version, branch/fault identity, label/redaction,
reason/evidence refs, explicit retry and ambiguous-delivery state,
relation/fallback lineage, and patch/save interaction. Exclude raw source text
where policy forbids it, host paths, credentials, capability/witness material,
private state/payload, and raw M8/M9 identity.

The documented workflow starts from a fresh checkout, uses a small provisional
command set to build/project, launch, exercise positives/faults, inspect, and
conform, and performs deterministic cleanup. It must compose real compiler,
projection, adapter, process, runtime, and observer layers; a thick helper that
calls internal functions in sequence is not acceptance evidence.

**Direct consumer:** I3-6 consumes the actual producer inventories and joined
observer evidence for downstream-only finite conformance.

**Primary falsifier:** The view invents a join/occurrence, hides a required
network failure, leaks protected data, relies on filename/fixture/expected JSON,
or the walkthrough cannot reproduce from a clean checkout without undocumented
manual state.

**Exit and validation:** Fresh-checkout reproduction at least twice; documented
provisional commands and cleanup; deterministic observer output modulo declared
non-semantic environment fields; negative secret/host-path/redaction scans;
corrupt/missing join rejection; usability and security review; preserved
scenario/I2/M10 regression; Report 2608; accepted source/evidence and
integration commits pushed with parity; advance exactly to I3-6.

**Stop/reopen:** Stop for required secret exposure, public compatibility freeze,
production credentials/resources, or a workflow that cannot compose the real
layers. Reopen for nondeterministic identity, invented causality, stale branch
visibility, or clean-checkout reproduction failure.

## I3-6 milestone contract — inactive until I3-5 closes

**Goal:** Close an exact finite source-first I3 conformance profile and apply
official I3 entry followed by I3 exit only if every required row, negative,
regression, review, and lifecycle non-claim passes.

**Entry:** I3-5 workflow accepted, Report 2608, pushed parity, current goal
I3-6, and a pre-implementation conformance contract freezes the exact finite row
inventory and predicates for this milestone without freezing a public schema.

The row count is intentionally not guessed in ALIGN-0. At I3-6 entry the
accepted contract enumerates every row exactly, with no implementation-selected
omission. At minimum its coverage partitions include source/checked identity;
artifact/edge completeness; selected transport/adapter and private encoding;
actual multi-process execution; transport non-authority; owner preservation;
no direct remote store; no source-free state/authority mint;
membership/capability/witness revalidation; full failure/retry/ambiguity/order
boundary; SCN-01/02/03/06; maintained relation; designated delivery without
consumer semantic re-execution; observer safety; source-to-network-to-runtime
correspondence; deterministic workflow; I2/M10 regression; evidence
classification; lifecycle non-self-activation; and provisional/public
non-claims.

The producer executes accepted lower layers and records typed inventories. The
verifier reads those inventories only. It cannot parse, project, admit,
schedule, dispatch, mutate, add evidence, select a transport, or authorize a
phase. Missing/failed evidence, wrong diagnostic, extra/missing row, absent
provenance, or unexecuted falsifier rejects.

**Direct consumer:** NEXT-0 may use only an accepted I3 boundary to write
inactive I4 and I5 entry contracts.

**Primary falsifier:** Conformance passes with a missing/extra edge or row,
single-process/helper substitute, source-free authority/state, unexecuted
negative, wrong evidence class, observer leak, I2/M10 regression, or producer/
verifier self-authorized lifecycle success.

**Exit and validation:** Exact row specification and fixed verifier; executed
positive/falsifier/provenance binding for every row; deterministic content-bound
identity and observer-safe report; full I3 focused suite and fresh-checkout
workflow; preserved I2/M10/workspace/format/warnings-denied checks; exact proof/
model/runtime/deferred classes; final independent assurance, semantic,
security, concurrency, and lifecycle ACCEPT with no unresolved P0/P1; any P2
has an explicit disposition and is resolved when it materially affects
acceptance; an
authorized Canon acceptance record applying, in order,
`official I3 entry accepted -> official I3 exit accepted`; Report 2609;
accepted evidence and lifecycle integration commits pushed with parity; advance
exactly to NEXT-0.

**Stop/reopen:** Do not apply either lifecycle transition if any row, command,
review, regression, or authority record is missing. Reopen for a passing
omission, fabricated/unbound evidence, wrong diagnostic/classification,
lower-layer dependency on the conformance aggregator, lifecycle self-activation,
or a counterexample to the pre-existing I3 criteria.

## NEXT-0 milestone contract — inactive until I3-6 closes

**Goal:** Record inactive entry contracts for I4 durability/live patch and I5
safe Browser/Host/View participation, close this bounded program, and leave no
active roadmap or goal.

**Entry:** Official I3 entry then exit accepted by I3-6, Report 2609 complete,
all commits pushed with parity, and current goal NEXT-0.

**Required inactive contracts:**

- I4 contract: local durable save/load and live patch over the accepted distributed
  boundary, with durable-format non-freeze, restart/crash, cut/in-flight/
  quiescence, patch/save ordering, provenance, authority lineage,
  rollback/no-stale-resurrection, failure recovery, migration, and an exact
  failure matrix explicit. It does not implement durability or patching and
  makes no distributed-durability claim.
- I5 contract: safe Browser/Host participant and observer-safe View boundary using the
  ALIGN-1/2 product/trust maps, preserving typed input -> Mir command, runtime
  -> View, View -> renderer presentation-local compute, typed effect ->
  provider, sandbox/resource, privacy/redaction, and non-authority rules. It
  keeps browser/Unity/Unreal renderer candidates unselected, freezes no public
  package/FFI/provider ABI, and defers upper Shared-Space concrete semantics.
  It does not implement a browser, renderer, Domain Kit, Shared-Space API, or
  Reversed Library.

These are two separately recorded, separately reviewable, and independently
activatable inactive contracts. They may share accepted I3 evidence but remain
separable future programs. They do not establish an execution order between I4
and I5 without new owner direction.

**Direct consumer:** Future owner-authorized I4 and/or I5 bounded programs;
none is activated by NEXT-0.

**Primary falsifier:** NEXT-0 implements future work, selects a public
API/ABI/wire, treats Browser/Host/View as semantic authority, weakens
save/patch lineage or privacy, creates a combined I4/I5 product monolith, or
leaves Plan 250/current status pointing to an active successor.

**Exit and validation:** Canon-process inactive entry contracts; explicit
entry/stop/reopen criteria and non-effects for both future areas; three-axis and
trust-contract consistency review; docs/index/hierarchy/HTML validation;
independent roadmap/semantic/security review; Report 2610; final integration
commit/push/remote parity; Plan 250 marked closed; Plan 249 retained as closed
I2 baseline; progress/tasks/status state no active bounded program, roadmap,
semantic milestone, or goal.

**Stop/reopen:** Close and stop after the inactive contracts and no-roadmap
state are accepted. Reopen only for a counterexample to an entry boundary or an
owner-authenticated successor direction; missing future product features are
not a reopen reason.

## Dependency and ownership map

| Milestone | Direct dependency | Primary ownership | Required independent review |
| --- | --- | --- | --- |
| ALIGN-0 | owner direction, accepted M10/I2, Canon plan/05 | parent integration; Canon writer; planner/status/config writers; eval owner | Canon-first planner plus lifecycle/semantic reviewer |
| ALIGN-1 | accepted ALIGN-0 | architecture/Canon writer; planner/status writer | architecture and semantic-boundary reviewer |
| ALIGN-2 | accepted ALIGN-1 | architecture/security/Canon writer; test/formal support | authority/privacy/sandbox and semantics reviewers |
| I3-0 | accepted ALIGN-2 | bounded candidate implementer/eval/test owners; ADR writer | security, portability, distributed-systems, semantic reviewers |
| I3-1 | selected I3-0 candidate | one adapter/encoding implementation writer; separate test owner | codec/security/semantic reviewer |
| I3-2 | accepted I3-1 | runtime implementer; process/integration test owner | concurrency/runtime/security reviewer |
| I3-3 | accepted I3-2 | runtime/fault implementer; test/model owners | distributed-systems/concurrency/security/semantics reviewers |
| I3-4 | accepted I3-3 | scenario/conformance test owners; bounded source support | semantic/authority/concurrency/conformance reviewers |
| I3-5 | accepted I3-4 | devtools/workflow implementer; docs/test owners | usability/security/causality reviewer |
| I3-6 | accepted I3-5 | conformance producer/verifier writers; formal/test/status owners | independent assurance/lifecycle panel |
| NEXT-0 | accepted official I3 exit | planner/Canon/status writers | roadmap/semantic/security reviewer |

The same writer may not own overlapping alternative implementations in I3-0
when that would contaminate comparison evidence. The parent retains whole-
program integration, Canon alignment, cross-layer trade-offs, milestone
acceptance, and stop decisions.

## Decision checkpoints and unresolved boundaries

| Checkpoint | Decision authority | Earliest trigger | Current disposition |
| --- | --- | --- | --- |
| Three-axis map acceptance | Canon process under owner direction | ALIGN-1 evidence | owner-fixed target; not an API/product freeze |
| Trust tiers/contracts | Canon process under owner direction | ALIGN-2 evidence | owner-fixed target; concrete APIs/layouts intentionally deferred |
| OPEN-032 transport selection | I3-0 authorized ADR | same-gate A/B comparative evidence | both UNSELECTED; datagrams excluded |
| Supported OS/CI/browser-feasibility matrix | I3-0 selection record | reproducible candidate probes | exact tested matrix recorded before selection; unsupported claims remain explicit |
| Private encoding/version/limits | I3-1 internal contract | carrier round-trip/fault evidence | internal and provisional; public wire remains separate |
| Retry/ambiguous delivery policy | operation-specific I3-3 contract | actual failure injection | explicit only; no global exactly-once |
| Exact I3 conformance row count | I3-6 pre-implementation contract | accepted I3-5 producer inventory | enumerated before verifier evidence; no implementation-selected omission |
| Official I3 lifecycle | authorized I3-6 acceptance record | all rows/reviews/regressions pass | inactive until then; entry and exit applied only in order |
| I4/I5 activation/order | future owner direction | after NEXT-0 inactive contracts | neither activated or ordered by this program |
| Public API/ABI/wire or production | owner-reserved separate decision | evidence plus explicit request | outside this program |

## Risks, assumptions, and mitigation

### Accepted assumptions

- Accepted I2 artifacts/carriers and SYS-2--SYS-6 evidence are regression inputs,
  not a public wire or network architecture.
- At least one retained reliable-stream candidate may satisfy plan/05, but this
  is a hypothesis tested by I3-0, not a selection.
- ST remains the deterministic semantic reference; network/process execution
  refines it only for the accepted finite profile.
- Local multi-process execution is sufficient for finite C-distributed evidence;
  production WAN/security strength remains outside scope.
- Browser/Host, View, provider, shared-space, Domain Kits, and applications need
  responsibility/trust contracts before implementation and remain separable.

### Main risks

| Risk | Trigger | Mitigation / reopen rule |
| --- | --- | --- |
| lifecycle overclaim | bounded program or test output called official I3/product completion | only I3-6 authorized record changes official lifecycle; preserve theory T1/broad-I1 axes |
| transport-as-authority | certificate/session/endpoint identity grants mutation | ALIGN-2 contracts plus I3-1/2 admission negatives |
| public/internal collapse | private carrier/encoding becomes compatibility promise | explicit provisional/private labels; owner stop before irreversible freeze |
| hidden retry/exactly-once | disconnect/ambiguity automatically resends or reports success | I3-3 explicit request-bound retry and ambiguity states |
| stream-order collapse | within-stream order justifies cross-stream/reconnect use | explicit dependency/frontier/provenance refinement and reorder falsifiers |
| stale resurrection | reconnect accepts retired membership/capability/witness/result | revalidation and late-old-session negatives before mutation |
| fake distribution | one-process helper or handwritten edge passes | OS-process evidence, generated-plan-only route, process cleanup and provenance checks |
| encoding confusion/resource abuse | malformed/version/size input allocates or admits | limits before allocation/admission, fixed negative corpus, fail closed |
| observer leak | logs/view export secrets, private state, or host paths | reference-only typed projection, secret scans, independent security review |
| evidence laundering | runtime/model result called proof or helper called product | ledger-only proof status and exact five-class labels |
| layer collapse | product/provider/browser/domain vocabulary enters Core | ALIGN-1/2 maps, ownership table, mandatory stop |
| both transports fail | A and B cannot meet the same mandatory gate | stop for owner direction; do not add a third candidate autonomously |
| non-migratable tie | A/B require different irreversible visible semantics | stop for owner decision; do not choose by convenience/performance |
| resource/external risk | paid service, production endpoint, secret, user data, or unsafe cleanup required | stop; keep local/synthetic evidence and follow repository resource policy |

## Mandatory owner-reserved stop conditions

Stop the active milestone, preserve evidence, and request owner direction if
any of the following is required or reproduced:

1. change the North Star or its five verbs;
2. weaken authority, privacy, redaction, failure explicitness, lifetime
   monotonicity, no-stale-resurrection, or meaning-derived communication;
3. promote domain/provider/browser/application vocabulary into Mir Core;
4. introduce a hidden multi-owner transaction, hidden retry, or exactly-once
   claim;
5. irreversibly freeze a public grammar, CLI, API, ABI, artifact, JSON, codec,
   or wire contract;
6. both retained transport candidates fail the same mandatory plan/05 criteria,
   or continuation would require selecting a third candidate autonomously;
7. choose between tied non-migratable externally observable semantics that the
   Constitution cannot order;
8. deploy or publish to production, access user data/secrets, consume paid
   resources, or create material external state;
9. accept an unsafe privileged raw-FFI/sandbox/resource boundary;
10. weaken frozen SCN expectations or evidence classes to obtain a pass; or
11. reproduce a parent-goal/North-Star counterexample that the fixed sequence
    cannot conservatively resolve.

Difficulty, slower performance, an unfrozen public contract, deferred general
proofs, theory T1, broad PHASE-I1 residuals, or incomplete I4+ are not by
themselves stop conditions.

## Program-wide non-effects

This roadmap does not itself change Canon semantics, proof/OBL status,
production source, tests, samples, lifecycle, public compatibility, transport
selection, or external state. The bounded program does not authorize QUIC
datagrams, WAN/production deployment, public release, durable distributed
save/load, live distributed patch, consensus, hidden distributed transaction,
global exactly-once, general scheduler/fairness/security/noninterference proof,
browser/renderer product, Domain Kit, Reversed Library, PrismCascade
integration, or Typed-Effect platform collapse.

## Recommended next action

Execute ALIGN-1 only: add the separate Canon project/product-layer document,
reconcile the existing semantic-strata numbering without changing settled
semantics, record responsibility/input/output/prohibition/maturity for PL-0--6,
keep PL-4 responsibility-only and PL-6 separate, validate no production/runtime
behavior delta, obtain independent architecture review, write Report 2601,
commit/push with parity, then advance exactly to ALIGN-2. Do not begin ALIGN-2
or transport comparison before ALIGN-1 closes.
