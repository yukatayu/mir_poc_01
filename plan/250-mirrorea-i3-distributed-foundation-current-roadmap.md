# Plan 250 — Mirrorea I3 Distributed Foundation current execution roadmap

最終更新: 2026-09-09 05:16 JST

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
- accepted ALIGN-1 architecture decision: PROPOSAL-038 / ADR-0035
  (the milestone integration commit cannot embed its own future hash)
- accepted ALIGN-2 trust-boundary decision: PROPOSAL-039 / ADR-0036
- accepted I3-0 private transport selection: PROPOSAL-040 / ADR-0037
- accepted I3-1 private adapter/encoding: PROPOSAL-041 / ADR-0038, cut `d75fa2e7...`
  (the milestone integration commit cannot embed its own future hash)
- accepted I3-2 two-process source/evidence cut: `19c5b386613d6adb1f0b934e6ced81acb327d245`
- accepted I3-2 Canon/status integration cut: `ab038f277d3cfd5ae1db2ad3e1fbc0147dfe0180`
- owner-resume baseline cut: `648425f6bd4304d003d36bc04d346ddf0e78c058`
  (`HEAD == origin/main`, clean before this task; 2026-09-07)
- resume authority: explicit owner direction, mirrored by PROPOSAL-043 /
  ADR-0040; ADR-0039 remains the immutable I3-2 acceptance/pause record
- sole current roadmap: **Plan 250**
- completed goals: **ALIGN-0, ALIGN-1, ALIGN-2, I3-0, I3-1, I3-2**
- execution state: **execute through I3-3 acceptance, then owner-requested pause**
- sole active semantic milestone: **I3-3**
- retained runtime checkpoint: **row 11 duplicate/stale replies plus
  row 17 / spec/16 bounded time-and-reply
  evidence integrated at `55f1fd7f`, pushed with clean remote parity**.
  ADR-0042/spec/17 selects the provider contract; its source/checker and
  fail-closed legacy guards are integrated at `9e8d674a`. Exact static coverage,
  composite verification and projection are integrated at `3862b168`, with
  clean pushed parity. Genuine resource binding -> actual M9
  authentication/verification and separate policy -> inactive scoped M8
  component (Stage 2b) is integrated at `7142b205`, with clean HEAD/origin/main/
  live parity observed `2026-09-08T19:22:07+09:00`. This is not a new milestone
  or provider runtime activation.
  The Stage 2b gates above remain its accepted input evidence. Current
  Stage 2c carries exact scoped M8/M9/static data through existing SYS4/SYS5
  inactive images, private codec and separately held expected validation.
  Final gates pass focused13 (included in runtime375), process63, M10/I2
  67/5/8, public guards5, actual ordinary localnet41, scoped Clippy, workspace
  format and all-target check. Independent spec/quality reviews are P0/P1/P2
  zero after reproduced expected-commitment/retirement repairs; the planner
  gives conditional GO for inactive-only integration. Those gates are met:
  Stage 2c is integrated at `a027d61b8030a903f892de2f7483ec8b64627963`,
  with clean HEAD/origin/main/live parity observed
  `2026-09-08T21:24:14+09:00`. The current direct consumer is actual Stage 3
  provider runtime/host/QUIC within I3-3, not another inactive-only handoff.
- next goal: **I3-4 (requires I3-3 acceptance and explicit owner resume)**
- all other milestones: **ordered, inactive, and dependency-gated**
- latest owner control (2026-09-07 18:13 JST observation): complete I3-3 with
  validation, independent review, commit/push and remote parity, then stop.
  The earlier through-NEXT-0 execution instruction is superseded only at this
  pause boundary; original program scope and fixed remaining order are unchanged.
  I3-3 is active until accepted; afterward Plan 250 remains the authorized
  paused roadmap with no active semantic milestone, neither blocked nor closed.
- Plan 247: closed M0--M10 execution record / regression baseline
- Plan 249: closed SYS-0--SYS-7 execution record / accepted I2 baseline
- official lifecycle at program start: theory **T1**; broad PHASE-I1
  **unaccepted**; official I2 entry and exit **accepted**; official I3
  **inactive**
- OPEN-032: **RESOLVED for this bounded program only by PROPOSAL-040 /
  ADR-0037; no public/production/platform selection follows**

LAB execution estimate, `2026-09-07T13:57:42+09:00`: the parent estimates
26–52 hours of remaining continuous execution across I3-3 through NEXT-0;
the dated per-milestone ranges are in `tasks.md`. This is a rough planning
estimate, not measured remaining work or an acceptance deadline. The earlier
I3-2 gate-to-gate Git timestamps span approximately ten hours, not separately
tracked active effort. Provider/time contracts, actual-session falsifiers and
review corrections dominate uncertainty. Six of eleven milestones are
accepted; that count is not a workload-weighted completion metric. No scope,
gate, evidence requirement or fixed ordering changes to meet an estimate.

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
   `architecture/10-i3-multi-process-runtime.md`;
3. `mirrorea_canon/plan/00-gates.md`, `plan/01-phases.md`, and
   `plan/05-i3-entry-contract.md`;
4. ADR-0026, ADR-0028 through ADR-0041, PROPOSAL-037 through PROPOSAL-044,
   `mirrorea_canon/spec/16-i3-owner-admission-budget.md`, and
   `mirrorea_canon/theory/11-metatheory-ledger.md`; and
5. the owner-authorized PROPOSAL-037 / ADR-0034 activation mirror and the
   milestone acceptance chain through PROPOSAL-042 / ADR-0039, followed by
   the PROPOSAL-043 / ADR-0040 execution-resume record.

LAB evidence is `progress.md`, `tasks.md`,
`.docs/progress-task-axes.md`, closed Plan 249, and milestone Reports
2598--2605; Report 2606 is the ongoing I3-3 record, not acceptance evidence.
If LAB wording conflicts with Canon, Canon wins. The owner direction
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

The control transition is fail-closed, with an explicit owner-pause exception:

```text
active milestone accepted
  -> its one report is complete
  -> required validation and independent review pass
  -> accepted evidence/integration commit is pushed and remote parity checked
  -> absent an explicit owner pause, advance to exactly one next active milestone

After an accepted milestone, an explicit owner pause retains Plan 250 as the
current roadmap but leaves no active semantic milestone; the next milestone
remains inactive until an explicit resume and a fresh cut/parity recheck.
```

The current owner instruction schedules this exception immediately after
I3-3 acceptance. I3-4 must not activate automatically.

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

| Milestone | Report path | Current status |
| --- | --- | --- |
| ALIGN-0 | `docs/reports/2600-mirrorea-i3-distributed-foundation-align0-baseline-goal-alignment.md` | completed |
| ALIGN-1 | `docs/reports/2601-mirrorea-i3-distributed-foundation-align1-layer-map.md` | completed |
| ALIGN-2 | `docs/reports/2602-mirrorea-i3-distributed-foundation-align2-trust-boundaries.md` | completed |
| I3-0 | `docs/reports/2603-mirrorea-i3-distributed-foundation-i3-0-transport-selection.md` | completed |
| I3-1 | `docs/reports/2604-mirrorea-i3-distributed-foundation-i3-1-adapter-encoding.md` | completed; ADR-0038 |
| I3-2 | `docs/reports/2605-mirrorea-i3-distributed-foundation-i3-2-two-process-runtime.md` | completed/accepted; historical pause superseded by ADR-0040 |
| I3-3 | `docs/reports/2606-mirrorea-i3-distributed-foundation-i3-3-failure-ordering.md` | active; ongoing, not accepted |
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

## ALIGN-1 milestone contract — completed

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

## ALIGN-2 milestone contract — completed

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

The shared security record also separates semantic decision, policy,
validation, enforcement, evidence and consumer roles; binds content/request,
instance/incarnation, target, policy/grant epoch, authority lineage, frontier,
freshness and scope; and requires use-time revalidation, queued/in-flight revoke,
typed ambiguous effects, metadata redaction and pre-limit resource accounting.

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

## I3-0 milestone contract — completed

**Goal:** Compare exactly the two retained reliable-stream candidates under one
common semantic/failure/order test harness and select at most one through an
authorized ADR without freezing a public wire.

**Entry:** ALIGN-2 accepted, Report 2602, pushed parity, current goal I3-0;
Candidate A TLS-over-TCP framed reliable stream and Candidate B QUIC reliable
stream were UNSELECTED; QUIC datagrams were excluded.

**Comparison rule and accepted result:** Both candidates ran the same private,
source/Core-bound nine-case receiver-child canary in actual distinct local OS
processes. The receiver child performed decode, exact retained-contract
revalidation, bounded cache lookup and handler linearization; the coordinator
did not fabricate those outcomes. The fixed rows were connect without semantic
admission, fragmented round trip, truncated frame, oversized frame, disconnect
before admission, disconnect after admission before result, duplicate across
reconnect, tampered retained-contract fingerprint and observer-safe evidence.

The I3-0 finite criteria, derived from the Design Constitution and applied in
the owner-fixed order, closed as follows:

| Order | Criterion | Equal evidence / material difference | Disposition |
| ---: | --- | --- | --- |
| 1 | internal semantic carrier transported losslessly | same retained owner-request facts and equal normalized rows | tie in bounded canary |
| 2 | failure matrix remains explicit | same typed common rows; no fabricated semantic result | tie; full plan/05 matrix remains I3-3 |
| 3 | deterministic local CI/fault injection | same fixed nine cases and child-event validation | tie |
| 4 | partial/truncated/oversized fail closed | same no-admission falsifier floor | tie |
| 5 | reconnect/duplicate/reorder stays explicit | duplicate has two receives/revalidations, one handler and stored decision; ambiguity remains typed/bounded | tie |
| 6 | no hidden retry / no exactly-once | no automatic resend or global delivery claim | tie |
| 7 | transport is not authority | retained contract, not session/certificate/stream, controls admission | tie |
| 8 | implementation/library maturity | both use the same Rustls trust stack and current maintained async libraries; LOC/configuration surface is simplicity, not maturity | no auditable winner; tie |
| 9 | cross-platform maintainability | only Linux x86_64 localhost tested | no tested winner; tie |
| 10 | future browser relevance | QUIC has the stronger future path | **first material difference; Candidate B wins** |
| 11 | performance | warm TLS 0.22 s / 44,052 KiB max RSS; QUIC 1.08 s / 43,944 KiB; TLS is 584 LOC versus QUIC 732 | lower-ranked performance/C12 simplicity evidence; cannot override criterion 10 |

PROPOSAL-040 / ADR-0037 therefore select Candidate B, QUIC reliable stream, for
the private bounded I3 adapter. Candidate A remains a
rejected/deferred comparison and replacement baseline, not a second active
implementation queue. QUIC datagrams remain excluded. No public wire, codec,
version, certificate representation, API/ABI, deployment, production security
or supported-platform set is selected.

**Direct consumer:** I3-1 implements a transport-neutral adapter and private
provisional encoding against Candidate B while retaining a conservative
replacement seam.

**Primary falsifier:** Candidate-specific metadata changes semantic identity or
authority; the common fault/order harness is not behaviorally comparable; a
candidate cannot fail closed deterministically; selection relies only on
performance/convenience; or both candidates fail a mandatory plan/05 gate.

**Exit and validation:** Both candidates ran equal private source/Core-bound
canaries across actual child processes, with common positive/falsifier rows,
resource measurements below semantic safety, explicit alternative disposition,
independent final ACCEPT with P0/P1 zero, PROPOSAL-040 / ADR-0037 resolving
OPEN-032 only for this bounded program, and Report 2603. The parent owns the
I3-0 integration commit/push/parity transition to I3-1.

**Stop/reopen:** Reopen for a selected-candidate security/semantic
counterexample, a loss of the equal canary floor, or evidence that Candidate B
cannot conservatively satisfy I3-1. An irreversible public/non-migratable tie or
both candidates failing would trigger the owner stop rule; neither occurred.

## I3-1 milestone contract — completed

**Goal ID:** I3-1

**Goal sentence:** By the end of this milestone, the accepted internal semantic
carrier passes through one checked transport-neutral, private provisional
encode/decode/admission boundary over the selected QUIC reliable stream without
losing meaning or admitting incomplete/untrusted bytes.

**Layer advanced:** Semantic stratum S4 Projection; project/product layer PL-2
Mirrorea distributed fabric; lifecycle implementation phase I3. S0/S1 meaning,
S2 trace, S3 verification and S5/S6 consumer boundaries remain preserved.

**North Star link:** Advances correct communication, verification and
observation by making the meaning-to-bytes-to-meaning boundary explicit,
checked, replaceable and fail-closed. Transport remains an implementation,
never the design source or authority.

**User-visible outcome:** A retained carrier can be losslessly encoded, sent
through the selected adapter boundary, decoded and admitted, while malformed,
incomplete, oversized, unknown-version or tampered input produces a typed
failure and no partial semantic request.

**Semantic invariants:** Source/Core/artifact provenance survives; effect,
failure, visibility and redaction remain explicit; request, serve, result and
receipt do not collapse; transport/process/session/certificate is not
authority; admission constructs no partial semantic request; logs expose only
observer-safe references and typed reasons.

**Direct consumer:** I3-2 starts actual locus processes and sends only generated
communication through this accepted adapter boundary.

**Non-goals:** Public wire/version/codec/API/ABI freeze; actual process/locus
deployment or owner runtime; retry, reconnect or order semantics beyond carrying
their required fields; complete I3-3 fault behavior; production security,
deployment or supported-platform claim.

**Primary falsifier:** Tampered, malformed, partial, oversized or unsupported-
version bytes become an admitted semantic carrier, or any round trip loses,
aliases, invents or defaults a required semantic field.

**Exit evidence:** Exact field-level round-trip/mutation tests, selected-adapter
tests, malformed/truncated/oversized/version/provenance/redaction negative tests,
deterministic property/mutation evidence for private decode (not coverage-guided
fuzz), preserved I2/M10 regressions and an
independent codec/security/semantic review with P0/P1 zero.

**Stop condition:** Stop I3-1 when I3-2 can consume the accepted adapter without
reconstructing or inventing semantic fields and independent review has no P0/P1.
Stop earlier only if lossless mapping requires a public freeze, semantic change
or guarantee weakening; reopen for the named falsifier or selected-adapter
regression.

**Entry:** I3-0 selection ADR accepted, Report 2603 complete, current goal I3-1,
and Candidate A retained as comparison/replacement evidence rather than a
second implementation queue. The parent completes the transition commit/push/
parity before starting I3-1 source work.

**Required boundary:** Internal carrier -> private versioned encoding ->
adapter bytes/reliable stream -> checked decoding/admission -> internal carrier.
Round-trip evidence covers applicable source/Core/program/artifact/edge,
request and occurrence identities, owner/origin/target, membership epoch and
incarnation, capability/witness references, effect/failure rows, visibility and
redaction, frontier/version/publication/consumption lineage, and declared
limits. Logs expose observer-safe references and typed reasons, never raw
credentials, capability/witness material, private payload/state, or host paths.
Request, serve, result, and receipt remain distinct. The private version policy
states separately how unknown versions and unknown fields reject, ignore, or
preserve data; no policy may default required semantic meaning.

**Detailed validation:** Exact semantic-field round-trip and mutation corpus;
canonical/deterministic private encoding where required for identity; partial
read/write, malformed, truncated, oversized, unknown-version, resource-limit,
provenance, redaction and secret-scan negatives; selected-candidate adapter
tests; property/fuzz decode tests over round-trip, truncation, malformed length,
oversize and unknown fields/versions; preserved I2/M10 regression; independent
codec/security/semantics review; Report 2604; accepted source/evidence and
integration commits pushed with parity; advance exactly to I3-2.

**Close evidence / disposition:** Cut `d75fa2e7...` passed focused source suites
60/60 and accepted floors 104/104. Final source-implementation reviews found
P0/P1/P2 = 0 and quality 0/0/0; the aggregate close review separately retains
the accepted/deferred architecture/04 size-budget P2. The workspace-wide run
exited 130 before tests under disk pressure and
is NOT PASS. Deterministic property/mutation tests passed; no coverage-guided
fuzz or general proof is claimed. P2-1 is closed for probe-owned key buffers
(library-copy scope remains a non-claim); P2-2 is closed by bounded reaping;
P2-3 remains Linux x86_64 localhost only; P2-4 has static no-mint/revalidation
evidence while live admission remains I3-3; P2-5 is deferred to I3-3; P2-6
remains canary-only and I3-2 replaces it; P2-7 strict negatives are closed;
P2-8 v2 identity and six-family coverage are closed.

The former P2 table is retained below as entry-history only; none is an open
I3-1 blocker.

| ID | Residual / exact non-claim | I3-1 acceptance use |
| --- | --- | --- |
| P2-1 | generated credential/private-key `Vec<u8>` is not zeroized | zeroize or bound/document lifetime before stronger secret-handling claim |
| P2-2 | cleanup has no second bounded deadline after kill plus wait/join; cleanup wording is canary-local | add bounded reaper behavior and keep claims scoped |
| P2-3 | macOS, Windows, browser and production are untested; evidence is Linux x86_64 localhost only | retain platform non-claim; test only a named direct-consumer matrix |
| P2-4 | no mutual TLS/client auth or live membership/capability/witness admission; cache reuse lacks a separately named no-mint assertion | keep transport authentication non-authoritative and add explicit no-mint/revalidation checks |
| P2-5 | TLS disconnect-after-admission ambiguity evidence has limited ordering strength | retain typed bounded ambiguity; leave complete semantics to I3-3 |
| P2-6 | request cache is fixed-capacity 8, in-memory, no-eviction and not actual owner runtime/durability/exactly-once | preserve canary-only claim and replace through the I3-2 owner-runtime seam |
| P2-7 | private decoder still needs duplicate-JSON-key rejection and wrong-marker classification separate from unknown version; clean `finish_event()` terminality is already covered by I3-0 regression | add the remaining exact fail-closed decoder negatives before acceptance and preserve the terminality regression |
| P2-8 | request-hash domain `v2` and textual `v1` label are misaligned; facade is owner-request-only and relation/designated fields are not yet mirrored | align identity label and document/extend exact carrier coverage before reuse |

**Stop/reopen detail:** Stop if lossless mapping requires a public freeze or
carrier meaning change outside the program. Reopen for ambiguous decoding,
allocation before limit checks, semantic defaulting, secret leakage or
replacement-seam failure.

## I3-2 milestone contract — completed/accepted; historical pause after close

**Goal ID:** I3-2

**Goal sentence:** By the end of this milestone, accepted generated artifacts
and communication execute across at least two actual OS processes, with
deployment limited to logical-locus-to-endpoint mapping and with remote owner
service/result behavior observable.

**Layer advanced:** Semantic S4/PL-2 Mirrorea fabric, lifecycle I3; Core meaning,
Browser/Host boundaries and official lifecycle remain unchanged.

**North Star link:** Correctly communicate checked meaning across real process
boundaries while preserving placement, authority, verification and observation.

**User-visible outcome:** A bounded fresh-checkout workflow starts two processes
from checked per-locus artifacts and performs a remote owner request/result with
source/Core/artifact/network/runtime correspondence.

**Semantic invariants:** Deployment maps only logical loci to endpoints; edges,
owner, authority, capability, witness, state, failure and occurrence identity
come from checked artifacts. No cross-process direct store; transport/session/
address is non-authoritative; provenance, redaction and typed failure survive.

**Direct consumer:** I3-3 injects its complete network failure, retry, reconnect
and ordering matrix into this actual route.

**Non-goals:** Complete fault matrix, retry/reconnect policy, durability, live
patch, browser/provider integration, public wire/API/ABI, production/WAN,
general proof, and official I3 entry/exit.

**Primary falsifier:** Runtime reparses source, hand-writes a route, uses a
fixture/expected result, shares a cross-process store, mutates a non-owner,
mints authority from endpoint/session identity, or leaves orphan processes.

**Exit evidence:** Actual two-or-more-process generated dispatch over the
accepted adapter; remote owner serve/result/receipt; correspondence and clean
shutdown; representative falsifiers; typed fault evidence; independent review;
Report 2605 and remote parity.

**Close result:** Positive/falsifier evidence passed with no P0/P1 at source/evidence cut
`19c5b386613d6adb1f0b934e6ced81acb327d245`; Report 2605 records the exact bounded
scope and non-claims. ADR-0039 recorded an owner pause at close; ADR-0040
subsequently resumes execution at I3-3 without reopening this accepted cut.

**Stop condition:** Close when positive/falsifier evidence passes with no P0/P1
and I3-3 can consume the runtime seam. Reopen for source-free routing,
authority violation, stale resurrection, hidden retry, direct store,
provenance/redaction loss or unbounded orphan resources.

**Entry:** I3-1 adapter/encoding accepted, Report 2604, pushed parity, current
goal I3-2, and no unresolved I3-1 semantic/security finding. Satisfied at close.

**Required execution:** The launcher starts clean independent processes from
checked per-locus artifacts, maps each declared logical locus to an endpoint,
connects only generated edges, performs remote request/admission/owner serve/
typed result or receipt, and terminates/cleans up deterministically. Neither
launcher nor deployment metadata supplies Core, routes absent from the plan,
authority, state, arguments, expected results, or semantic occurrence IDs.

**Detailed validation:** Actual two-or-more-process positive execution; process
identity and address non-authority negatives; missing/extra/retargeted edge and
direct-remote-store falsifiers; remote owner mutation and typed result lineage;
deterministic replay at the declared bounded scheduler profile; clean startup,
shutdown, port/resource cleanup, and fresh-run isolation; source/Core/artifact/
edge/carrier/network/runtime correlation; preserved I2/M10 regression;
independent runtime/concurrency/security review; Report 2605; accepted
source/evidence and integration commits pushed with parity; advance exactly to
I3-3.

**Stop/reopen detail:** Stop for unavoidable manual communication, authority collapse,
hidden shared state, unsafe cleanup, or production/deployment requirement.
Reopen for a reproducible route/owner/provenance mismatch or process boundary
that the I3-3 fault harness cannot control deterministically.

## I3-3 detailed Goal Statement — active

**Goal ID:** I3-3

**Goal sentence:** By the end of this milestone, the accepted source-derived
multi-process runtime executes all 20 required network failure families and
their controls with typed, request-bound retry/ambiguity and explicit Mir
ordering refinement, preserving owner-local mutation, authority, provenance,
redaction and bounded termination so I3-4 can run its C-distributed scenarios.

**Layer advanced:** Semantic S4 Projection/fabric over replaceable S6 Host,
preserving S0 Surface/S1 Core authority and S2 Trace/S3 Verify evidence; bounded
S5 sample vocabulary stays outside Core. Project/product PL-2 Mirrorea fabric
consumes PL-1 checked semantics and PL-0 process/network delivery; PL-3
Browser/Host, PL-4 Shared-Space, PL-5 kits/apps and PL-6 Reversed Library are
not implemented. Lifecycle remains the resumed ADR-0034 bounded program,
theory T1, broad PHASE-I1 unaccepted and official I2 exit accepted. Official I3
entry/exit remains an I3-6 acceptance action, not a consequence of this resume.

**North Star link:** Make communication derived from ordinary checked source
correct under actual process/network faults, with explicit verification and
typed/redacted observation, while preserving the checked evolution boundary.
Network schedules and handwritten interfaces do not become semantic authority.

**User-visible outcome:** A maintainer can rerun finite positive/control and
fault cases on the accepted local two-process QUIC route and inspect typed
outcomes and observer-safe source/Core/artifact/request/network/runtime joins.
The evidence distinguishes pre-admission rejection, possible post-admission
execution, operation-specific duplicate handling and rejected late traffic.
The joined end-user workflow remains I3-5's direct task after I3-4.

**Semantic invariants:** Ordinary source, checked Core and generated plans own
semantic edges, operations, authority and provenance. Mutation remains
owner-local; stores remain process-local. Certificate, connection, session,
stream, process, route, retry token and timing cannot grant authority. Retired
membership/epoch/incarnation/capability/witness and fallback/consume lineage
cannot resurrect. Retry is explicit and operation-specific: stored-result /
no-new-consume where the accepted operation permits it, or typed duplicate
rejection; there is no universal replay policy or global exactly-once claim.
Post-admission ambiguity stays bound to the original request and never proves
nonmutation, clean success or permission for blind retry. Mir dependency,
frontier and provenance justify semantic order; stream order cannot substitute.
Failure atomicity applies at the declared semantic boundary, and observer
output preserves label/authority/redaction without exposing private material.

**Direct consumer:** I3-4 consumes this executable failure/order boundary for
SCN-01/02/03/06 C-distributed cases and bounded relation/designated-result
cross-process pressure; it remains inactive until I3-3 is accepted.

**Non-goals:** Changing the parent goal or fixed remaining milestone sequence;
accepting I3-4 scenarios or I3-5 workflow early; official I3 entry/exit; broad
theory or ledger promotion; universal retry, hidden transaction or exactly-once;
public grammar/CLI/API/ABI/codec/wire freeze; QUIC datagrams; WAN/production
support; general scheduling/fairness/security/durability proof; durable
save/load or live distributed patch implementation; Browser/Host/provider
product integration; upper-domain/Core promotion or satellite collapse.

**Entry and resume cut:** I3-2 is accepted at source/evidence cut
`19c5b386613d6adb1f0b934e6ced81acb327d245`, with ADR-0039, architecture/10 and
Report 2605 as inputs. Historical owner direction resumed the original program
through NEXT-0 under PROPOSAL-043 / ADR-0040; the latest instruction schedules
a pause after I3-3 acceptance without changing that program's scope. At the resume baseline
`648425f6bd4304d003d36bc04d346ddf0e78c058`, `HEAD == origin/main` and the
worktree was clean. Parent-reported fresh baseline evidence passed localnet
12/12 and probe package 62/62, with no orphan children. This is retained I3-2
regression evidence, not new I3-3 acceptance. Only I3-3 becomes active; fault
controls must not supply semantic facts.

**Required failure matrix:** Every row needs a positive/control path, a typed
negative outcome, no forbidden semantic mutation, exact request/provenance
binding, and bounded termination.

All 20 families below must be executed at meaningful runtime/admission/adapter
boundaries. A harness-selected label or a report row without a bound execution
is not coverage. Each family records the reached boundary, triggering input,
actual typed result, mutation/consume evidence and request/provenance join;
post-admission cases report observed mutation or bounded uncertainty explicitly.

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
Ambiguous delivery is not silently retried or reported as success. The final
patch/cut/save family may use a positive quiescent admission boundary and an
in-flight typed rejection/late-traffic falsifier. It must exercise the actual
boundary, but does not require durability or live distributed patch machinery.

**Required ordering refinement:** Concrete occurrences must justify local
eligibility -> request -> send admission -> transmission -> complete receive ->
carrier admission and authority revalidation -> owner/effect linearization ->
typed result/failure send -> result receive -> receipt/consume. Preserve owner
coherence, request->serve, send->receive, serve->result, result->receipt/consume,
publish->observe, witness-create->use, grant/revoke->use, membership-update->
dispatch, verdict->activation, fallback->later-access, and cut/quiescence->later
transition. Late old-session traffic is rejected or placed by an explicit
current dependency.

**Primary falsifier:** Blind retry, false success, second mutation, unbounded
hang/buffer, stale lineage resurrection, stream-order-as-Mir-order, missing
linearization/provenance, harness-label-only coverage, or a pre-admission typed
rejection that nevertheless mutates owner state. Post-admission ambiguity
cannot be relabeled as proof of nonmutation.

**Exit evidence and validation:** All 20 families have executed positive/control
and typed negative cases at meaningful boundaries with deterministic fault controls;
bounded scheduler/order model where used and exactly classified; retry/
ambiguous-delivery/duplicate identity tests; stale membership/capability/
witness and late-session negatives; trace/order correspondence; preserved
I2/M10 regression; independent concurrency, distributed-systems, semantic, and
security review; Report 2606; accepted source/evidence and integration commits
pushed with parity; apply the owner-requested pause with I3-4 inactive.

**Stop condition / reopen:** Close only when the complete failure/order
inventory, its positive/falsifier evidence, fresh applicable regressions,
independent review and accepted source/integration cut satisfy the common
milestone contract; then stop and report the accepted boundary, leaving I3-4
inactive until explicit owner resume. Stop earlier if the selected transport cannot expose a required
failure/order distinction without changing Mir semantics, or if safe progress
requires hidden retry/exactly-once. Reopen for any failure-atomicity,
linearization, stale-resurrection, or bounded-termination counterexample.
The program-wide owner-reserved stops still apply; ordinary implementation
uncertainty or a missing general proof does not itself require owner input.

### I3-3 ordered work phases, ownership and decision checkpoints

These are parts of I3-3, not new milestones or independent active queues.

| Phase | Dependency and bounded work | Ownership | Evidence / checkpoint |
| --- | --- | --- | --- |
| 1. Contract and seam inventory | accepted architecture/09--10, ADR-0039, plan/05 and Report 2605; map every failure family and ordering edge to actual admission/serve/result/receipt boundaries | parent integrates Canon; mapper/planner advisory | exact 20-family inventory; operation-specific duplicate policy; request-bound ambiguity states; controls cannot mint semantic inputs |
| 2. Runtime fault and order refinement | phase 1 mapping; explicit before/after-admission fault controls and dependency checks on accepted generated dispatch | one production implementation owner; separate test owner | positive/control and falsifier executions; exact joins, mutation/consume counts or bounded uncertainty; reject blind retry, stale lineage and fake coverage |
| 3. Full matrix and finite ordering evidence | phase 2 behavior; complete all 20 families and the preserved ordering inventory | test/evaluation owners; bounded model owner only where needed | all families bound to meaningful execution; finite scheduler assumptions and runtime/model/deferred classifications; quiescent positive plus in-flight negative without durability implementation |
| 4. Acceptance and owner pause | phases 1--3, fresh applicable I2/M10 and I3 regression | parent acceptance; independent semantics/security/concurrency reviewer; status/report owners | one Report 2606, no unresolved P0/P1; disposition material P2; Canon acceptance and required validation; accepted commits/push/parity, then no active milestone; I3-4 requires explicit resume |

**UNRESOLVED implementation questions:** Which accepted operation returns its
stored decision and which rejects a duplicate; how each exact request retains
an observer-safe ambiguity witness across a new session; which explicit
dependency/frontier admits or rejects reordered traffic; and the smallest
finite boundary controls needed to exercise authority/provider/time/cut rows.
Resolve these against accepted operation contracts within I3-3, comparing the
current candidate with at most one smallest viable alternative. Do not choose
a universal retry policy, infer nonmutation after admission, omit a difficult
family, or activate durability/product work to simplify evidence.

**Adoption/discard rule:** Retain a refinement only when its real execution and
falsifier preserve the source-bound operation contract and all required edges.
Discard label-only evidence or a candidate that creates authority, hidden retry,
second consume or false success; reopen I3-2 only for its named reproducible
runtime-seam counterexample. Record implementation questions and evidence in
Report 2606 rather than creating a resume report or adjacent roadmap.

**Owner-intent alignment checkpoint:** I3-3 refines checked meaning under
network faults. Its finite owner-ledger limit, retained coordinator, closed
cohort and provisional operation-specific retry behavior are bounded
realization assumptions, not general Mir composition or continued-operation
semantics. All 20 failure families remain required. Each row must identify its
accepted semantic or explicitly admitted external boundary, actual execution,
positive control, falsifier and evidence class. Missing source/effect coverage
remains **OPEN**; matrix completion alone does not justify new Surface/Core
meaning. Checked evolution constrains this work now, while later implementation
acceptance remains dependency-gated. Broader compositional research is not a
new active queue or an added I3-3 gate.

**Actual-reconnect evidence checkpoint (`2026-09-07T15:00:30+09:00`):** The
same two live process runtimes now execute pre-write reconnect with one checked
receipt, and post-admission reconnect with exact-original duplicate rejection,
no second mutation and retained requester uncertainty. Sender/receiver
commitment and attempt-binding falsifiers preserve observer-safe distinctions;
verified local session misuse no longer blames the peer. That committed
checkpoint passed probe 22/22 and lifecycle unit 2/2.

**Successor / retained-ingress checkpoint (2026-09-07):** B installs a genuine
M9 capability-revocation successor before reconnect, then rejects the original
session-one frame with zero owner admission/serve/write; A remains pending.
G1 control admits once and consumes one checked receipt. Publication requires
an install-gated, exactly bound registered-B ACK; actual A-stdout tainted input,
duplicate completion, repeated ingress acquisition and contradictory terminal
observations are rejected at their respective boundaries. Fresh runtime
integration 61/61, feature library 289/289, private QUIC unit 2/2 and probe
29/29 pass, with focused deny-warnings Clippy and no remaining P0/P1 in the
independently reviewed repair. I2 SYS-5 5/5, SYS-6 CLI 8/8 and M10 conformance
67/67 regressions pass. No new model or Lean run is claimed.
Report 2606 retains exact commands and scope. This is not I3-3 acceptance.
**Selected-adapter delivery checkpoint (2026-09-07):** An actually closed
endpoint produces bounded unavailability with a retained zero-admission owner
terminal. A complete generated frame split across two application writes
round-trips once; strict body truncation followed by FIN rejects at the owner
before decode/admission. These are not packet-fragmentation or full-row claims.
The observer join now projects only validated owner records, rejecting malformed
provenance without erasing genuine admitted evidence. Fresh probe integration
32/32, full probe library 3/3, private QUIC 2/2, runtime integration 61/61,
I2 5/5 + 8/8, focused Clippy and format/diff pass; independent review has no
remaining P0/P1. The earlier feature library 289/289, M10 67/67 and model 3/3
are prior evidence, not rerun here. All 20 families and remaining ordering
obligations remain required. That cut leaves time/provider contracts open;
the following integration selects the time contract without accepting its
implementation. No later milestone or general harness framework is activated.

**Owner-admission time contract checkpoint (2026-09-07, committed integration
`050f5067`):** PROPOSAL-044 / ADR-0041 select spec/16's explicit source
opt-in budget and declared `DeadlineExpired`. Candidate A, a separately
authorized outer lifecycle contract, is deferred: the existing M9 update does
not supply that request-expiry authorization. Provider remains **OPEN**.
Independent normative review has no remaining P0/P1. Contract selection is
not implementation, runtime evidence, or acceptance of row 17 or I3-3.

The concrete dependent implementation steps within this same milestone are:

1. Static preservation: `mir-ast/src/surface_v0.rs`, M6 classification, M7
   checked pipeline and its private snapshot retain the clause, source span,
   owner-clock condition and conditional failure row. A separate test owner
   owns `mir-semantics/tests/i3_owner_admission_budget.rs`; the sole evaluator
   captured behavioral `UnexpectedSyntax` RED before source implementation.
   Test old-source identity/behavior, budget distinctions, failure padding,
   underdeclaration, unsupported shapes and non-erasing M5 export. Direct
   consumer: generated carrier contracts and guarded execution.
2. Projection and enforcement: retain the condition in both generated
   request/reply contracts and private execution translations. Every actual
   M8/SYS4/SYS5/M10 entry enforces the condition or rejects before mutation;
   initial artifact admission cannot replace the one-use per-request permit.
   Then implement serialized owner clock/stage/resolve and retained requester
   terminal-failure state. Direct consumer: genuine typed failure transport.
3. Selected-QUIC execution: timely reservation, delivered expiry with zero
   owner mutation, lost outcomes with requester uncertainty, replay and
   authority/capacity falsifiers. Observe reservation, actual serve,
   failure send/receive/consume and success separately. Run applicable
   regressions and independent review before checkpoint acceptance.

Reopen for erased/aliased metadata, a serve bypass or reused permit,
requester-controlled time, false nonexecution, reservation counted as actual
serve, budget restart after loss, or failure counted as success. No generic
clock/lease theorem, provider implementation or new roadmap is opened.

Static-slice checkpoint (`2026-09-07T19:56+09:00`): step 1 now passes
14 new static/snapshot tests, old AST/M6/M7 10/13/27, a compile-fail raw-Core
export falsifier and focused Clippy, with no remaining independently reviewed
P0/P1. Step 2 is the direct consumer; source acceptance is not committed while
annotated execution lacks its required lower guard. This is not I3-3 acceptance.

Default-enforcement checkpoint (`2026-09-07T20:34+09:00`): generated contracts
8/8, private projection snapshots 6/6, ordinary M8 rejection 2/2 and M8
regressions 33/33 pass; independent projection/M8 reviews have no P0/P1.
Current-delta I2 5/5 + 8/8, I3 runtime 61/61 and M10 67/67 pass. Three
runtime Clippy style lints are under repair. Before committing this safe
checkpoint, verify actual alternative entry/private image negatives and
prevent the kernel from relabelling unsupported admission as `RouteUnavailable`.
The next direct consumer is the existing owner ledger extended with serialized
clock/stage/resolve and retained decisions, then a non-Clone one-use permit
through SYS4/M8, genuine typed failure consumption, and selected QUIC evidence.
Do not introduce a second scheduler/identity ledger. Started process runtimes
have no live cut/export API; pre-start images do not preserve live requests.
Genuine authority installation must retain awaiting decisions and allow current
authority revalidation to reject them, not silently restart or revive them.

Final safe-checkpoint validation (`2026-09-07T21:00+09:00`): an actual SYS5
negative exposed the old contextual executor's assumption that every rejection
has an M8 trace. The repaired ST/OW1 path carries a typed unobserved rejection;
M10's false route diagnosis is separately repaired for this exact new guard.
Budget library 11/11 (including real worker rejection/snapshot/shutdown), M10
source 3/3, direct guard 2/2 and full SYS5 runtime 63/63 pass. Broader runtime
library with test seams 298/298, M8 33/33, M10 conformance 67/67, I2 5/5+8/8,
probe 32/32, private QUIC 2/2 and focused Clippy/format pass. Independent
integration review has no P0/P1. These are finite default-rejection and
preservation results, not clock/permit/expiry execution or row-17 acceptance.
The Stage C consumer remains as above; provider and remaining I3-3 matrix/order
remain required. Only full I3-3 acceptance triggers the owner-requested pause.

The safe checkpoint was pushed as
`050f5067c5f63384abd7b8a6389158ddc76187da`; fresh remote parity and a clean
worktree were verified at `2026-09-07T21:09:01+09:00`. Stage C now advances
the existing ledger through owner-local staging, serialized clock resolution
and one-use admission, without a second scheduler or identity version.
Its initial source-based behavioral falsifier fails 0/1: the current SYS5
entry returns `CarrierAdmissionRejected` instead of staging with no owner
effects. The preceding opaque-Option assertion compile failure is not this
behavioral evidence. Typed failure transport/requester retention is the next
dependent consumer, not implemented by staging alone.

Stage C1/C2 internal checkpoint (`2026-09-07T22:39:42+09:00`): the frozen
dirty runtime/test delta over `050f5067` passes the complete owner-admission
module 14/14, absent-target 1/1, queued-owner 1/1 and source staging 1/1.
Independent narrow review has no remaining P0/P1 after correcting binding
nonmutation, immutable resolution provenance, trace-less typed rejection,
record-derived finalization and exact current-generation handoff. Ordinary
historical-carrier rejection remains strict. These are local finite results,
not typed expiry transport or full fault-row acceptance; integration still
emits 16 unused-path warnings until its direct consumer is connected.
The immediate consumer is C3: gate-produced declared expiry in the generated
reply, bounded requester terminal retention before pending deletion, then
actual selected-QUIC time/loss/replay evidence. Provider and all remaining
I3-3 failure/order obligations remain required. No additional milestone,
general generation migration, scheduler or public compatibility is introduced.

Resumed C3 checkpoint (`2026-09-08T08:36:36+09:00`): the owner again explicitly
requests execution through I3-3 completion, followed by the same pause. HEAD
and freshly queried remote main remain `050f5067`; C1/C2 and C3 changes are
preserved uncommitted. Fresh local tests compile and run 16 passed / 3 failed:
independently modified decision provenance and an expiry substituted into an
unbudgeted request are incorrectly accepted; the capacity test stops in its
fixture because repeated tick 1 correctly becomes a no-op. The separate
genuine G1-pending/G2-successor test also reproduces incorrect acceptance.
The private QUIC classifier test has compile errors (missing intended API and
test-only private-field access), not network evidence. Repair those test setup
issues, capture capacity rejection before dequeue, and complete the narrow
receiver repairs before connecting the actual selected-transport consumer.
Independent read-only planning review retains all 20 families and ordering
requirements; it establishes no mandatory owner escalation. Provider remains
the distinct unselected contract, not a relabelled RMW or expiry failure.

The next time consumer uses the existing two-process probe, not a second
transport fixture. Under spec/16's already selected T0 host-clock boundary,
the parent selects one opaque runtime-bound host driver over the larger
alternative of changing serialized startup controls to carry sealed clock
stimuli. Its bounded operation advances one checked owner clock and resolves
the deterministic next Awaiting request through the existing gate/handoff;
it cannot accept a request identity, permit, carrier or expected outcome.
Keep ClockHandle/Awaiting/Reserved implementation types private. The default
immediate owner scheduling resolves at the current tick; explicit monotonic
host tick input tests expiry only after genuine complete-frame admission.
Neither source requests nor deployment mappings provide that clock input.

The ordinary source consumer is the separate budget-1 source at
`samples/clean-near-end/mirrorea-i3-owner-admission/main.mir`, retaining the
existing four-locus shape and adding the condition only to `init_avatar_hp`.
Its initial existing-API process test must expose the missing host driver
before implementation. Later delivered-expiry, served/lost-reply uncertainty
and retained-expiry replay use the same launcher and selected QUIC adapter.
The probe must distinguish successful receipt from actual terminal failure;
it may not relax its normal `serve/write/receipt == 1` validation to invent
expiry evidence. Add only actual producer-derived expiry decision references,
requester terminal references and corresponding delivery records. Missing
actor observations remain unknown rather than zero. This is I3-3 row-17
evidence, not I3-4 scenario acceptance, a public control API or a new clock
framework.

The actual served/lost-reply test initially observes peer close, not a
requester wait expiry. The bounded completion step is an explicit T0 local
monotonic wait after that observed loss, over the same retained pending
request. The existing causal path is owner serve/write -> withheld reply and
physical close -> requester loss observation -> local wait -> elapsed
observation. Validate actual pending, receipt and terminal-consumption state
before/after; elapsed time creates no remote fact, cancellation, retry right
or semantic `DeadlineExpired`. This is not an adapter timeout or the global
child/reaper deadline. Independent review prefers this smallest design over
keeping the session open with a new serve-to-wait causal barrier: the latter
adds machinery without a stronger required semantic fact. Direct consumer:
spec/16's served/lost-reply/requester-wait row. Falsifier: local wait clears
pending, reports unserved/expiry, consumes a receipt, or silently resends. The
current peer-close-only test is predecessor evidence, not this row's closure.

Row 11's remaining time consumer is exact generated-reply replay through a
verified successor QUIC session, separately for successful and declared-expiry
replies. The current API consumes a non-Clone reply on send and cannot perform
that fault injection. Select one feature-gated adapter-only, non-Clone opaque
replay candidate over exposing raw encoding/caller bytes: issue it only after
the original generated reply was actually sent; bind its exact body, original
peer/cohort and permitted successor session; consume it for one explicit replay
attempt with a new network occurrence. The ordinary checked receiver, not the
fault control, determines rejection and preserves the first receipt/terminal
decision. No new semantic retry policy, source action, authority or public wire
is introduced. Adapter tests/refinement and probe-local wait are independent
implementation parts of the same time/reply boundary, with disjoint writers.

Independent Canon-first planning review confirms the evidence split: actual
network duplicates for both reply variants, plus the existing genuinely
G1-produced expiry bytes rejected against a genuinely updated G2 requester at
the production binder, explicitly classified **local stale-generation binder
evidence**. Verify that actual QUIC consumption uses that same checked binder.
Do not add B-G2 while A stays G1 and call it requester-staleness evidence; that
does not test the relevant condition. This plan does not waive any family or
accept unexecuted evidence. Direct consumer: row 11/spec/16 and the current
time checkpoint; primary falsifiers are second receipt/terminal consumption,
replaced decision, retargeted/reissued replay, current-generation bypass or a
network occurrence mislabeled as successful delivery/admission.

Time-runtime checkpoint (`2026-09-08T10:55:34+09:00`): delivered expiry,
served/lost-reply uncertainty, retained-expiry reconnect and the explicit local
wait now pass six named tests (seven cases), with full probe integration 38/38.
Feature-union runtime library 332/332 and process integration 63/63, I2 5/5
plus 8/8, M10 67/67, two-crate deny-warnings Clippy, workspace format and docs
validation pass. Narrow independent time/local-wait review has no P0/P1.
The optional HTML-reader embedded catalog has a documented preexisting
failure, outside this runtime evidence; it is not recorded as passing.
The parent integrates this bounded checkpoint before the row-11 actual reply
replay consumer. Neither this checkpoint nor the passed local stale-generation
test accepts row 17, all of spec/16, I3-3 or official I3 lifecycle. Provider and
the remaining complete matrix/order still follow; I3-4 remains inactive.

This checkpoint is committed and pushed as
`30429d5d0521d4ad03fd0500ad49092c89042caf`. Fresh remote lookup, HEAD and
origin/main match with a clean worktree at `2026-09-08T10:58:12+09:00`.
It is the pinned input to the row-11 reply-replay work, not I3-3 acceptance.

Current time/reply disposition (`2026-09-08T12:04:34+09:00`): actual successful
and genuine expiry replies are consumed once, then their exact retained bytes
are replayed through verified successor QUIC sessions and rejected by the
ordinary receiver without changing the first requester decision or owner
state. The initial-session falsifier rejects before replay occurrence/write.
Full probe integration is 41/41; after observer-layout/format-only cleanup,
focused replay is 3/3, two-crate all-target deny-warnings Clippy and workspace
format pass. Final independent review has P0/P1/P2 zero. Canon-first planner
review confirms that these results close the bounded row-11/spec/16 consumer
after documentation integration and commit/push/parity, not I3-3 itself.
The genuine G1-expiry/G2-requester check remains local production-binder
evidence; no requester-generation network update is claimed. The 30429d5
runtime 332/process 63/I2/M10 5/8/67 floor is retained baseline evidence, not
rerun evidence for this adapter/probe delta. Report 2606 holds the exact logs.

Next direct consumer: select the smallest source-declared typed external
invocation contract for provider failure, then implement its checked grant,
projection and actual provider attempt/result path. No existing owner RMW,
expiry producer, auth-discharge rejection or transport error substitutes for
that effect. The existing Oracle advice is advisory and unadopted. Provider
contract/code, remaining membership/redaction/cut families, full ordering,
milestone regressions and acceptance review remain required. I3-4 is inactive.

The reviewed time/reply integration is now committed and pushed as
`55f1fd7f76b86a2fc6a846c0133d55d5c8213831`; HEAD, origin/main and live remote
matched with a clean worktree at `2026-09-08T12:16:45+09:00`. This completes
that bounded component, not I3-3. Its frozen evidence and non-claims above
remain unchanged.

### Provider component contract checkpoint — within I3-3

Direct consumer: failure family 18 and effect-order correspondence.
Blocker reduced: no accepted source-derived external invocation currently
exists; owner RMW, authentication rejection and expiry cannot stand in for it.
Acceptance use: ordinary checked source produces a remote effect request,
actual bounded provider call and validated typed result/failure consumption.

Read-only Canon-first planner review recommends distinct source effect
execution at an already remote locus, with generated request/result branches
sharing the existing private QUIC framing. A separate provider process/network
leg is unnecessary. Separate Core/effect authorization/result semantics remain
mandatory. The one alternative is a dedicated effect transport-facing service;
do not collapse this comparison into a ban on necessary enum variants.

Parent-written PROPOSAL-045/spec/17 were reviewed and **adopted by ADR-0042**
under ADR-0034's delegated authority. They constrain a T0 adapter to a dedicated
nonsecret T4 regular-file fixture,
with exact source resource/allowance, independent effect grant, actual read,
typed missing-resource failure, bounded retention and no owner write. Oracle
advice is advisory; its terminal-ambiguity suggestion is rejected in favor of
the accepted pending/remote-unknown rule. The initial five P1 findings on exact
failure classes, call-start ordering, provider incarnation, resource admission
and observation were integrated; narrow independent re-review has P0/P1/P2
zero. Production still requires the test-first gate. No I5, public API,
arbitrary file access, new roadmap or milestone is opened.

#### Provider implementation sequence (activation follows contract adoption)

Contract integration is committed/pushed as
`985ee179b76d8e1a9e72571ef5a275e190ae4cf9`, with clean HEAD/origin/main/live
remote parity observed `2026-09-08T12:50:23+09:00`. Stage 1 starts from that
exact cut. The test author owns only the new source fixture and source test;
the production writer waits for the evaluator's actual behavioral RED.

**Goal:** The existing checked-source pipeline retains a distinct authorized
external invocation and executes its actual typed provider result/failure
through the selected two-process transport, without owner writes or hidden
retry. Parent owns cross-layer acceptance; specialist writers own disjoint
source/test files. These stages are components of I3-3, not new goals.

1. **Source/checking and fail-closed handoff.** Production owner:
   `time_host_driver`; test owner: `c3_test_repair`. Add
   `crates/mir-ast/src/surface_v0/read_only_provider_effect.rs` and wire the
   declaration into `surface_v0.rs`. Add
   `crates/mir-semantics/src/surface_v0_provider_effect.rs`; wire its typed M6
   template through `surface_v0_classification.rs`, distinct M7 checked core,
   effect/authority obligation through `surface_v0_pipeline.rs`, exact
   `surface_v0_pipeline/private_snapshot.rs` preservation and a non-authorizing
   source contract in `m9_finite_refinement.rs`. The existing axes are
   Computation/Locus/OnRequest/Caller/PublishValue; provider permission remains
   a separate use-time grant. Do not overload an owner signature or theorem.
   Unsupported lower execution/export paths must reject, not omit the effect.
   Any necessary ownership expansion for exhaustive guards is coordinated
   before editing; production and test writers never overlap.
2. **Projection and authority.** After stage 1 API freezes, one production owner
   adds distinct SYS3 requester/service/consumer fragments, generated edge
   contracts and private projection snapshots, and the effect-specific M9
   policy/opaque grant. Tests derive all coordinates from the ordinary source
   and prove wrong/missing/revoked grant and tampered snapshot cannot invoke.
   Existing owner grants and valid transport are negative controls, not inputs
   to a permissive adapter. Exact symbols/file ownership are assigned at this
   gate, rather than guessed before source types exist.
3. **Actual runtime/transport/provider.** Add the monotone source-bound effect
   ledger and host adapter, then distinct private carrier branches through
   existing SYS4/SYS5/QUIC. Preserve the existing process launcher, not a second
   Quinn harness. The private supervisor envelope, resource binding and
   provider incarnation must be available before activation. Execute success,
   actual missing resource, invalid data and rejection/failure lifecycle tests.
4. **Network evidence and component integration.** Test author exercises two
   fresh values, actual typed provider failure, missing grant, duplicate/loss/
   revocation and observer-safe joins. Run applicable source/runtime/probe and
   I2/M10 regressions, independent spec and quality review, then commit/push
   and parity. Only afterward proceed to remaining I3-3 membership/redaction/
   cut/order consumers. No component green run is whole-I3-3 acceptance.

Stage 1 RED uses existing APIs, not a missing-symbol compile error:

```rust
#[test]
fn ordinary_provider_source_is_checked() {
    let source = include_str!(
        "../../../samples/clean-near-end/mirrorea-i3-provider-effect/main.mir"
    );
    let result = check_and_elaborate_surface_v0(FixtureSource::new(
        "samples/clean-near-end/mirrorea-i3-provider-effect/main.mir", source,
    ));
    assert!(result.is_ok(), "declared provider effect must check: {result:?}");
}
```

- [x] Contract review/adoption and docs validation (Canon index 216,
  hierarchy 800/800, scaffold 1760; diff and bounded 17-file secret scan clean).
- [x] Test-only source fixture and `crates/mir-semantics/tests/i3_provider_effect.rs`;
  execute `cargo test --locked -p mir-semantics --test i3_provider_effect
  ordinary_provider_source_is_checked -- --test-threads=1` and capture the
  actual unsupported-source assertion failure before production changes.
  Actual RED at `985ee179`: one test body fails `UnexpectedSyntax` at source
  line 57 (`effect`), not a missing API; log
  `/tmp/c3-provider-effect-initial-red-20260908.BgbxVG/01-provider-effect.log`.
  Parent inspected the log and authorized the bounded AST/semantics writer;
  separate typed source tests are being added. No provider runtime is enabled.
- [x] Implement AST/M6/M7 preservation; add focused typed tests for exact
  coordinates, ten generated failures, one-declaration limit, unsupported
  profile, underdeclared failure, identity change and non-erasing snapshots.
- [x] Run the whole new source target plus existing M6/M7/budget targets;
  map and test every unsupported execution/export guard before handoff.
- [x] Stage 1 independent review and precise downstream interface packet;
  source support alone is not admitted provider execution.

Stage 1 final evidence: source 17/17, constructor-privacy doctest 1/1,
private runtime guards 2/2, public runtime guards 5/5; existing AST/M6/M7/time
targets pass. Feature-union runtime library 334/334 and process integration
63/63 pass, as do M8 7/7, M9 external boundary 1/1, SYS5 5/5, I2 CLI 8/8
and M10 67/67. Three-crate all-target Clippy, workspace all-target compilation
and format pass. Workspace compilation is not workspace test execution;
overlapping suites are not added together. Source/spec and runtime quality
reviews report no remaining P0/P1/P2 after three construction/restore repairs.
Exact commands, logs, skipped final gates and component Git status are in
Report 2606. The new sample is source-checking evidence only.

Stage 2 must preserve default legacy rejection while introducing explicit,
source-bound mixed-program coverage and separate provider permission. It must
not silently erase provider Core to materialize M8 or treat an owner grant as
effect authorization. A bounded advisory Oracle comparison checks that handoff;
no second transport/runtime or I3-4 implementation is opened.

Stage 1 is committed/pushed as
`9e8d674a29b724d3947c8fccf1cf78e62a491206`; HEAD, origin/main and live remote
match with clean worktree at `2026-09-08T14:05:06+09:00`. It closes only the
source/guard component. The paused product goal and stop-after-I3-3 control
are unchanged.

Stage 2 implementation choice (LAB, within ADR-0042/spec/17): choose the
smallest explicit, activation-pending M9 composite over a separate provider
issuer/runtime. One Oracle consultation and independent read-only planner
confirm the actual M8-before-M9 dependency; parent verifies against the code.
The sequence is static coverage/projection, then actual M9 authority plus
mixed finite verification/separate effect policy, then exact SYS4/SYS5 closure.
These are ordered components of I3-3, not parallel semantic frontiers.

The current static component derives total ordered executable source-map
coverage, four provider lowering associations, three distinct roles and two
generated edges from the full checked source. Pure projection must not depend
on a finite admission verdict: the shared coverage producer is static and
non-authorizing; a separate composite verifier preserves all existing finite
checks and the exact provider profile. It must retain all four provider effects
and the source-label row `provider-effect:{operation}` exactly. That row alone
does not prove observer authority, redaction/rate enforcement or permission to
disclose a result. No generic legacy refinement rule or redaction lattice changes.

Source-only independent review found two composite interface gaps; both were
reproduced before repair on 2026-09-08. A source-A candidate was accepted for
source B when only its resource changed, and the composite exposed a cloneable
ordinary discharge without its pending coverage. The bounded repair uses a
new opaque exact-source-bound composite candidate and removes raw inner
discharge access. Existing generic candidates remain unchanged. Current
runtime guards still prevent invocation; these findings concern the next
admission consumer, not evidence of an executed unauthorized provider call.

Further static review distinguishes typed-marker concealment from complete
local omission. Ordinary snapshot/SYS4 guards share a provider-only typed
marker scan; source identity must also preserve the full-program requirement
when a locus restriction contains no provider-local rows. The smallest
selected repair direction queries the existing canonical identity record in
the source producer (fixed record positions, not arbitrary text scanning),
rather than introducing duplicated profile state. Changing that record changes
the exact identity; expected-source/image binding remains the authority
boundary. This query does not authenticate a freely supplied snapshot.

The final frozen Stage 2a test packet passes semantics 6/17/8, composite
privacy doctest 1, static runtime 11 and the full feature-union runtime library
345. Independent spec review closes all four interface/integrity findings with
no P0/P1/P2. Separate quality review also reports P0/P1/P2 zero; process 63,
I2/M10 5/8/67, public guards 5, three-crate Clippy, workspace format and
all-target compilation pass. The post-lint static 11 rerun passes; annotation
and lifetime changes do not alter behavior. Workspace compilation is not tests.
The test-only snapshot module was moved outside the production projection
directory to preserve its dependency scanner; no architecture guard was waived.
Report 2606 records each actual RED and final GREEN separately.

Stage 2a is committed/pushed as
`3862b1688821c8fb243f8f148f54cd313ad3b163`; HEAD, origin/main and live remote
match with clean worktree at `2026-09-08T15:41:54+09:00`. The static handoff
is closed; sole-active I3-3 and all remaining gates are unchanged.

Stage 2b now consumes that cut. A genuine setup-owned resource incarnation
precedes the dedicated M9 composite entry. Reuse actual finite-local bootstrap
validation, authenticate membership, retain the full opaque finite composite
verdict, and require separate fixed effect policy before issuing any effect
capability. Correct source, provider identity, an owner grant or generic
capability-scope input cannot supply that approval. Preserve every exact
spec/17 coordinate, actual lifetime retirement and observer-safe diagnostics.
Only that seal permits an inactive legacy M8 component; its internal scope
must survive cloning/restriction and reject ordinary export/restore without
composite context. No filtered checked source or raw inner discharge is allowed.
Missing fixture target is a valid setup binding; request lookup waits for the
later CallStarted runtime. No invocation, supervisor proof, network activation
or row-18 acceptance is manufactured by this component.

Two direct-consumer details are fixed within that implementation choice.
First, supplied composite evidence must equal the exact supported canonical
MembershipAuth strengthening, not merely share source identity and coverage:
a valid static refinement may add requirements that this finite runtime does
not implement. Compare the intact discharge; do not expose its legacy inner
proof or replace a missing input. Second, actual setup issues a fresh opaque
32-byte OS-random resource/runtime incarnation independently of temp path,
PID, session and counters. Reuse already locked `getrandom` 0.4.3 as a direct
runtime dependency; entropy failure is typed setup rejection with no fallback.
This is finite implementation freshness, not a uniqueness theorem, authority
credential or a new public identity scheme. M9 checks remain necessary.

Single coupled production ownership moves to `c3_runtime_fix` for M9/M8 and
trusted binding, with narrowly necessary ordinary snapshot callers only;
`time_network_tests` owns a new separate private test module. Parent retains
Canon/status/Git and evidence gates. Exact failing behavior precedes completion,
with one evaluator and the same resource floor. Read-only planner's reviewed
Stage 2b entry contract supplies this sequence; no second roadmap is opened.

Stage 2b final evidence observed `2026-09-08T19:07:58+09:00` supersedes
intermediate results retained in Report 2606: runtime362/362 (including the
provider filter26), process63/63, M10/I2 67/5/8 and public guards5/5 pass.
Clippy covers mir-ast/mir-semantics/mir-runtime with warning denial; workspace
format and all-target check pass. The workspace check is not test execution.
Generic M8 constructor refusal and three direct review negatives are closed;
independent spec/quality reviewers have no remaining P0/P1/P2 and the planner
approves inactive-only component integration. Parent's LAB/Git gates remain.
Actual M9 effect authorization and scoped inactive M8 are produced; provider
invocation/networking/runtime/supervisor and row18 remain absent. Final free
space is 30,906,707,968 bytes after the approved five-file deletion and owner
housekeeping; no additional agent cleanup occurs. No gate is waived.

Stage 2b closes at `7142b205b4e2805d50ce78156de99e6feb1db37a`; commit/push and
clean live parity are observed at `2026-09-08T19:22:07+09:00`. Stage 2c is the
private checked mixed-image variant through
the EXISTING SYS4/SYS5 admission/initial-state/restriction/snapshot/trusted-start
seams. It must retain the actual consumed composite evidence and needed M9
facts at sealing, full identity, exact four-row scope, independent effect
authority and current resource incarnation; it must not manufacture a
filtered source, bare ordinary admission or second runtime/launcher/transport.
The current inactive component is not already a complete proof-retaining
serialized image. Decisive negatives are evidence splicing, stale/foreign
context, scope widening/stripping, ordinary-path laundering and hidden
activation. Runtime/supervisor/actual host and QUIC evidence follow this join.

Stage 2c implementation uses three narrow shared seams: the existing finite-local
M9 source-derived capability/witness issuer and active-snapshot inventory
translation; SYS4's checked per-locus restriction traversal; SYS5's existing
deployment/image/control assembly. The ordinary path retains ordinary final
verification and its successor publisher. The composite path retains the
actual consumed composite discharge, ContractUpdate facts and scoped M8,
without synthesizing `M9FinalAdmissionEvidence` or a provider-free verdict.
Composite successor/patch routes remain unavailable in this component.
Private image seeds distinguish ordinary generation from inactive composite
association. All source loci remain deployed; provider roles/edges are exact
inactive descriptors, not an excuse to widen child legacy execution scope.
Decoded images remain candidates until separately trusted expected-start
validation; that validation still cannot start a provider/LocalFabric/endpoint.

- [x] Separate test author and fail-closed production scaffold; record an actual
  positive assertion RED independently of any missing-API compiler error.
- [x] Genuine M9 legacy issuer/inventory reuse and retained composite evidence.
- [x] Exact A/B restriction, private codec round-trip and inactive expected-start
  validation; ordinary escape, splicing, altered coverage/scope and actual
  retired-context falsifiers reject without activation.
- [x] Focused quality/regression, independent review, LAB synchronization and
  observed commit/push/parity. Then continue actual provider runtime in I3-3.

Initial Stage 2c positive executes and fails at the genuine sealed composite's
typed `InactiveProcessHandoffPending` boundary: 0 passed, 1 failed, 363
filtered, exit 101, inspected `2026-09-08T19:32:46+09:00`. The uninhabited
scaffold emitted nine warnings, not final warning-clean evidence. The subsequent
genuine handoff exposes missing retained relation-reacquire facts; preserving
those already validated facts yields six passing image/codec/guard tests and
one reached foreign-checked-identity query failure. That query and the reached
ordinary-restore/single-slot-deployment falsifiers are repaired: nine focused
checks pass. Required private M8 scope is preserved in version 2; old or
unknown scope cannot default to ordinary. The genuine retained-start Debug
privacy falsifier then fails and is repaired with a reference-only formatter:
ten focused tests now pass (362 filtered, 7.57 seconds). Removing four redundant
lint expectations then passes scoped warnings-denied Clippy and workspace
format. Runtime372/372 and process63/63 pass before independent review finds
two acceptance blockers: missing equality with the separately held exact image
commitment, and inconsistent receipt facts on actual mid-validation setup
retirement. Both are reproduced and repaired; ordinary decoded-start and
actual excluded-provider-row negative coverage are also tightened. The final
frozen packet passes thirteen focused tests (included in runtime375/375),
process63/63, M10 67/67, I2 local/CLI 5/5 and 8/8, provider guards5/5,
ordinary localnet41/41, scoped warnings-denied Clippy, workspace format and
workspace all-target check. Both independent reviews have zero remaining
P0/P1/P2 findings. The planning review's evidence conditions are met except
final LAB/docs/Git integration. This is local inactive-image evidence, not
provider activation or whole-I3-3 acceptance. Actual Stage3 host/QUIC execution
is the next direct consumer, not another inactive-only handoff.

Stage 2c closes at `a027d61b8030a903f892de2f7483ec8b64627963`; clean
HEAD/origin/main/live parity is observed `2026-09-08T21:24:14+09:00`.

### Stage 3 — actual provider execution through the existing process fabric

Direct consumer: I3-3 row 18 and its request/call/result/order/cut obligations.
Blocker reduced: the admitted source effect has no actual host/QUIC execution.
Acceptance use: real A request -> B bounded file read -> retained typed outcome
-> A checked consumption, with source/Core/artifact/edge/occurrence provenance.
This is one executable implementation component within I3-3, not a new
milestone, roadmap or inactive-only acceptance shell.

The read-only planner requires explicit custody at activation. Parent-held
Stage 2c setup/seal currentness is historical and local to the parent; its
`Arc` is not shared across `exec`. The genuine admission must be consumed
into one-shot, role-restricted A/B installation records through the existing
trusted child-control path. B owns serialized current M9 checks, reservation,
`CallStarted`, retirement and release; A owns its current-grant check before
consumption. Already-issued facts are installed, not re-issued from an image,
transport identity or serialized `verified/current` flag. Pre-transfer parent
retirement denies activation. Post-transfer parent state cannot be advertised
as child revocation without actual causal delivery/installation evidence.
Arbitrary live rebinding and instantaneous global revocation are not supplied.

The actual supervisor must own the trusted fixture lifetime through termination
and reaping, and supply B-only resource/deadline enforcement through existing
FD3 control. Parent selects the narrow privileged T0 writer and runtime-owned
inherited-control reader boundary after implementer/Oracle advice. The private
probe whole-run owner performs the write only after child registration under
the armed deadline/reaper. A public/doc-hidden writer is not access control:
T0 can capture bytes or fabricate a pipe, outside this finite trust profile.
Actual supported control flow and tests establish supervision; ordinary image
or generic decoded control cannot construct the opaque inherited install.
Moving broad Child/event/reaper ownership across crates is deferred as
unnecessary under the same T0 assumption. Reopen for an untrusted activation
escape or a supported run that omits enforcement. An availability Boolean or
hash-only proof is not a substitute for operational custody.
Ordinary M8/M9/SYS4/SYS5 rejectors remain unchanged in meaning. Reuse the
existing process runtime, private carrier codec, QUIC session and launcher;
distinct effect variants must not become owner RMW/request/receipt aliases.

The first acceptance path is real source-first execution with two fresh
admissions and two different nonsecret fixture values, plus actual missing
resource failure with zero owner writes. Assert the designated consumer's
actual typed result separately from value-free observer output; neither a
test expectation nor a fault tag may supply the semantic result. Then cover
strict bounded parsing, resource/type/policy refusal, pre/post-call retirement,
capacity/spent allowance, duplicate request/result, reconnect, lost reply and
requester retention. Exact local tests remain labelled local. Pending work
rejects unsupported cut/export; the later row-20 consumer integrates the full
process-local cut obligations before I3-3 closes.

Initial Stage 3 source-real positive executes RED, inspected
`2026-09-08T21:42:20+09:00`: 0 passed/1 failed/375 filtered, exit101,
typed `ProviderRuntimeActivationPending` after genuine composite/cohort
preparation (not a compiler error or actual-child run). Runtime and disjoint
probe behavior implementation now proceeds with separate tests and one Cargo
evaluator. The real fixture setup guard must be transferred and retained
through reaping; context cloning alone cannot survive its retiring Drop.
The full-fact child M9 custody/interface compile checkpoint subsequently
passes the narrow runtime library check after an exact setup-context type
repair; four unfinished-interface warning groups remain. This is compilation
only, not provider installation/host/QUIC success. The runtime and probe
writers continue the actual one-frame inherited-control and runtime path.
After the FD3/install source is implemented, the local source-real module
passes4/4 and the probe library check passes. These cover genuine preparation
and local role-restricted M9 installation/tamper/retirement, not an actual
FD3 child run, host call or QUIC result. Runtime/probe writers continue the
actual LocalFabric/carrier/host/consume path without an intermediate close.
The local fallback-order test then passes1/1 after its assertion is corrected
to retain the legitimate prior ViewerC M8 observation as well as the new
target-dequeue predecessor. The actual-launch integration positive executes
RED even after a factory-only physical slot-name correction: the observed
generic `ProviderLaunchRejected` does not establish child/FD3/host/QUIC success.
This supplies the direct process consumer while runtime/probe writers complete
the scoped common LocalFabric initializer and actual effect lifecycle. No
further inactive-only component acceptance is introduced.
The scoped shared-initializer runtime library check passes at the 23:05
checkpoint (9.87 seconds, two unfinished dead-code warning groups). This is
structural compilation only; the process positive above remains the latest
actual-run result. The selected ordinary source profile has an explicitly
empty `Sys4InitialStateSeed`; installed provider children preserve that state
and the checked schemas/legacy authority, rather than inventing initial
values from handler literals. Actual carrier/host/consume work continues.
Capacity scope follows spec/17: the sole executor enforces at most64
invocation reservations per admitted operation, and each installed runtime
has one at-most64 decision budget shared across its pending/reserved/rejected/
retained/consumed states. A/B records for the same semantic request do not
create a second invocation allowance. This does not require a distributed
mutable counter, a new permission round trip or a shared physical ledger;
reconnect still cannot replenish either runtime's retained budget.

The distinct provider request/result and existing QUIC session profile pass
the narrow runtime library check after a misplaced helper-impl/borrow repair
(9.97 seconds, five staged dead-code warning groups). This is not an actual
process success. Terminal audit correctly remains fail-closed: current role
data retains no independent observer capability/policy, and ordinary M9
Observation is specifically ContractUpdate-history scoped. The mapper and
independent planner confirm that spec/17 already requires this missing direct
consumer. The selected private implementation adds a separate trusted-policy
Observation capability/witness with a bound provider observer-use record,
exact authenticated membership/source/operation/incarnation/role, fixed
reference-only fields and a single bounded terminal export per child. Generic
Observation meaning and ordinary M8 projection remain unchanged. Actual
current-use and export capacity checks precede projection/release; observer
denial does not change effect facts or enable fallback diagnostic leakage.
Missing/revoked observer-only authority, effect-cap substitution, wrong
operation/incarnation/role, repeat export/reconnect reset and raw-data output
are decisive falsifiers. This is not a new grammar, telemetry framework,
Canon guarantee or semantic milestone. `provider_observer_authority` now owns
only `m9_auth_verification.rs`, explicitly released by `c3_runtime_fix`, who
retains the other production runtime files; probe and tests retain their
separate ownership. All Stage3 work remains one actual-execution component.
Fresh compilation including the new observer producer/consumer stops on two
M9 snapshot-versus-auth membership type mismatches; no tests run in that
packet. The M9 writer fixes only the pre-install structural matcher, retaining
actual reconstructed-authority current-use checks, and withdraws an accidental
claim based on the older pre-observer green checkpoint. The fresh corrected
runtime check passes in10.15s, and the local provider module passes6/6;
neither establishes host/QUIC execution. SYS5/probe now implement a strict bounded,
versioned, non-authorizing observer-view candidate for child stdout; decoding
it cannot manufacture an issued M9 export or permission. The runtime keeps
the real permit/projection producer, and the probe checks owned-run/child and
source/request correspondence before using received evidence. This boundary
is connected in source, but the fresh actual provider test remains0/1 RED
(`ProviderLaunchRejected`). Fresh fixed-stage diagnosis narrows it to
`ExecutorBootstrap -> ControlHandoff -> ProviderControlWriteRejected`
(0.54s body,4.97s compile); a freshly rebuilt child excludes a stale binary.
That runtime kind still includes construction failures and is not proof of
an actual `WouldBlock`. Source inspection separately identifies a defect:
the nonblocking FD3 stream requires an entire control chunk in one `write`.
The selected bounded repair keeps frame construction and byte access inside
the runtime, uses offset/poll progress under the supervisor's unchanged
absolute deadline, closes on success/failure, and never reissues a control
or semantic request. The probe passes only its existing deadline, not a
generic writer that could inspect control bytes. Safe pre-write/size/physical
failure categories and actual rerun must distinguish the reached boundary.
The writer's three physical progress/deadline tests pass, but the actual
rerun now identifies `ProviderControlSnapshotOversized`; this clarifies the
old conflated error rather than proving a prior physical write failure.
The selected next repair replaces only the full peer start-binding copy with
the exact peer slot/program/image/component references used by its direct
transport consumer. Those references still come from the actual parent-held
binding. Full local image correlation and actual role M9 authority remain,
and each peer validates its own full image. The512-KiB bound stays unchanged;
raising it without a measured sufficient bound is the rejected alternative.
This is lossless for the peer identity consumer, not a general full-snapshot
round-trip claim. If still oversized, inspect component sizes before any
further choice. No normal observer output includes private control bytes.
The compact peer tests pass2/2 but the actual control remains oversized.
Count-only measurements through the real consuming snapshot producer give
requester/executor empty controls3,917,843/3,803,597 bytes and worst admitted
transport upper bounds4,179,987/4,065,741 bytes. Source-local static snapshots
account for about3MiB each. Independent Canon-first planner review and parent
inspection distinguish the new provider-only limit from accepted ordinary
control512KiB, semantic messages64KiB, images8MiB and existing event/queue
limits. Those accepted limits remain unchanged. The selected provider-only
cap is now5MiB (5,242,880 bytes), leaving1,062,893 bytes above the measured
maximum transport bound. Preserve full local image/M9 checks and enforce the
cap during encoding, not only after allocation, plus bounded strict reading.
The alternative compact-local/hash reconstruction would introduce a new
validation argument and is not selected. Tests must cover both roles and all
three current fixture profiles, cap+1/truncation/length/duplicate rejection,
unchanged ordinary limits, actual bootstrap and its memory/deadline behavior.
Measurement is not a general heap bound or observer-policy expansion; full
frame/JSON/DTO/retained-image copy accounting remains an explicit gate.
The provider-only encoder now reserves bounded capacity before serialization;
the strict reader rejects an oversized prefix before body allocation. These
changes remain unaccepted pending the frozen packet. The first packet stops
at a test-helper compile error (`expect_err` requires Debug on the private
authority-bearing success DTO); fix the test with a non-printing match, not
by exposing the DTO through Debug. No test body ran in that packet.
The59-KiB peer program field is a source-derived structural stable key, not
an opaque reference. Preserve that full key for local image/M9 validation,
but use the existing domain-separated checked-program SHA reference for
provider peer prefaces and carrier binding only. Move the unchanged helper
to a private shared leaf so M9 does not depend on the SYS5 facade. Exact
expected-reference comparison and current authority checks remain required;
a compact reference cannot issue or restore authority. This does not widen
the64-KiB semantic frame or observer output policy.
The repaired frozen packet passes control9/9, local execution8/8,
compact-peer2/2 (included in control9), and the first actual source-real
provider A/B supervised QUIC probe1/1. Parent reads its log at
`2026-09-09T01:28:52+09:00`; the15.59s test body includes fixture preparation
and is not the measured child-only lifecycle duration. No orphan probe remains.
This proves the normal path, not exact41/-7 value correspondence or the full
provider fault profile. Next integrate the private A post-consume fixture
assertion, typed M9 revalidation and bounded read accounting; then the
remaining spec/17 falsifiers. Four runtime dead-code warning groups, one
probe unused-mut warning and final regression/review remain open at that cut.
The next packet passes parser/read5, execution9 and control9. After correcting
an inverted probe predecessor predicate, actual normal/41/−7/absent tests
pass4/4, including private A retained-consume comparison and genuine M8
owner-occurrence-zero checks. Parent then finds the provider success gate
accepted cleanup-only reaping without requiring zero child exits. These green
probes therefore do not establish clean lifecycle completion. The strict
existing zero-exit/no-kill/deadline observation is now required and the normal
probe is0/1 RED at `NaturalReap`; diagnosing and repairing this shutdown path
was the immediate blocker. Exit-only tracing of unchanged old binaries then
confirms one child exited1 and the other0 (role mapping deliberately absent).
Waiting for A's QUIC endpoint drain after close, inside the unchanged timeout,
fixes the reached defect: strict normal1/1 passes in1.10s and the full four
profiles pass4/4 in4.19s. Parent reads both logs at
`2026-09-09T01:56:49+09:00`; normal is duplicated across the two commands.
This is the first strict normal-exit evidence. Next add actual post-terminal
nonzero rejection and local genuine-component retirement/capacity/duplicate
tests. The local fixture may construct actual checked/M9/M8 execution pieces
while borrowing the trusted setup guard; it is not an inherited-FD3 installer
or end-to-end evidence and cannot promote a decoded control into authority.
Keep one-export budget reservation before projection allocation and currentness
at commit; post-work denial does not refund the allocation allowance. Actual
network loss/reconnect, observer faults, remaining rows and review still follow.
The following frozen packet passes execution15/15 (including six local
retirement/capacity/duplicate cases), parser/read5, control9, and full actual
probe5/5. The fifth probe is a real A post-terminal exit9 which the strict
gate rejects; its focused run is duplicated, not a sixth unique case.
Local64/65 and duplicate results are not reconnect/network evidence. Current
work has passed bounded observer encoding before allocation: parser/read/writer7,
execution15 and strict actual probe5 are green in the subsequent frozen packet.
Actual installed observer-currentness/budget falsifiers are the direct consumer.
Independent planner and parent select a sealed, fixed, nonsecret conformance
profile in the source-real launch/private runtime control, consumed only after
normal FD3/image installation. Four cases exercise real installed M9 retirement
before preflight, retirement after projection before commit, second export,
and effect retirement with independently current observation permission.
Exact denial/no-export/budget assertions remain private to the child. One
generic opaque experiment-completion result, identical across cases, is finite
owner-requested test instrumentation, not an observation, provider outcome or
proof of nonexecution. Ordinary denied observations cannot use this route.
There are no caller-defined callbacks, semantic physical-request flags or
denial-reason/private-reference exports. T0 identity is not authority. Reuse
the existing real QUIC supervisor and strict natural-zero gate; no second
launcher or information-flow theorem is introduced. Stop this route if it
requires unauthorized metadata release. The provider component and I3-3
remain unaccepted.

The installed-observer packet now passes actual probe11/11 (five prior
normal/physical cases, four observer cases and two prelaunch mode-confusion
rejections), local execution15, parser/read/writer7 and control9. Parent reads
the logs at `2026-09-09T02:44:18+09:00`. The generic conformance event contains
only its fixed marker and physical slot; extra image/locus/transport flags were
removed before evaluation. Next execute three fixed source-real resource
profiles: noncanonical integer,33-byte overflow and directory-kind rejection,
plus the missing local witness-before-call zero-host falsifier.
Independent planner and parent omit the optional actual EMFILE/RLIMIT experiment:
spec/17 requires representative actual I/O, not every OS error atom. Preserve
`AdapterUnavailable` implementation, production-read-loop LOCAL injected-I/O
classification and typed carrier/consumer preservation; do not claim actual
OS operational-error evidence. B64 plus A's checked65th rejection establishes
the finite system bound, not independently exercised B65. No new requester
source or authority path is introduced solely to force that unreachable case.
The resource packet now passes actual probe14, execution16, parser/read/writer7
and control9; parent reads all logs at `2026-09-09T02:58:09+09:00`.
The three new actual profiles reach their expected typed outcomes. The local
before-call table covers capability and witness retirement; the added local
codec/admission test mutates a genuine NotFound result into an explicitly
untrusted AdapterUnavailable candidate and proves atom preservation/one consume,
not an actual host operational failure. Private FD3 control is nowv6 for the
expanded trusted assertion enum. Provider network producer retention/join and
payload-hash removal are the current implementation slice; reconnect/fault
execution follows it and is not yet claimed.

The next provider-network slice has independent planner conditional GO for
reuse of the existing bounded two-session control transfer. Carry the same
installed runtime, M9 lineage, occurrence counter and ingress obligations;
reverify peer/preface without reinstall, authority issuance or a third session.
An original genuine result held unsent may receive its first send on session2
after current B release revalidation and current A consumption checks. This
is withheld-result/post-call-disconnect evidence, not already-transmitted loss
or stored-ledger result reconstruction. Separately exercise actual transmission
with lost consumption and bounded genuine request/result replay through normal
receivers. Preserve pending remote-unknown state and typed duplicate rejection,
with no second invocation, consume, refund or hidden retry.
Actual adapter attempt references use run/session/direction/ordinal, never
raw result/frame hashes; join admitted semantic identity afterward. Reserve,
completed send, complete receive, host/outcome/release and consume producers
remain distinct. Safe metadata still requires the separate observer gate,
including rejected attempts; public error accessors/Debug are not an exemption.
The alternative reconnect refusal plus only local late-result evidence cannot
support the required actual-network claim. This is selected bounded work,
not executed evidence; stop if it needs authority renewal, stored-result
reconstruction, unbounded replay or unauthorized fault diagnostics.

Pre-edit M9 mapping finds v1 does not authorize new network metadata. Parent
and independent planner authorize a separate private fixed terminal profilev2
under existing ADR-0042/spec/17, not a silent expansion of v1 or a new owner
decision. Existing separate T0 observation-policy proof and current M9 bound-use
must pin v2's allowlist and limits: A request-send reservation/completion and
complete result receive; B complete request receive, host-start, retained outcome,
release and result-send reservation/completion; existing source/Core/artifact/
edge/request/adapter/consume refs and permitted retained-state counts. Transport
kinds have two fixed slots per semantic entry, preserving the first occurrence;
this is schema capacity, not retry permission or executed second-session evidence.
Retain64 semantic rows AND65,532 body bytes plus4-byte prefix, with one export;
reject the complete projection if either bound fails, without truncation, refund
or a claim that all64-row shapes fit. Generic Observation remains unchanged.
The first network slice fills real first-session producer records and joins them;
reconnect/loss/replay behavior remains the next slice. References are allocated
at actual transitions, and complete receive precedes semantic admission/host
execution even though accepted semantic binding is attached afterward.

At `2026-09-09T03:31:30+09:00`, slice1 remains unaccepted. Initial v2
compilation exposed six runtime integration errors; bounded fixes preserve
the stored send-reservation reference and its completion check. The rerun
reached only the two expected old probe send-method arity errors. After their
mechanical migration, the real value41 two-process test executes and fails
at `TerminalObservationCorrelation` (0/1,13 filtered). This is the behavioral
RED for migrating the old host join to the v2 producer records, not passing
network evidence. Runtime/M9/tests stay frozen during the probe-only join
update. Existing local execution/M9 checks and malformed-version behavior
remain explicit regression checks; reconnect/loss/replay has not started.

Independent bounded planner review and the parent select the smaller fault
evidence path: keep the strict normal v2 exported join, and verify each actual
QUIC fault with assertions inside the genuine installed children, exposing
only a generic conformance completion. This is actual network execution with
child-private assertions, not an exported provider-fault trace, and ordinary
owner traces cannot substitute for these provider experiments. Assertions must
use actual pending/ledger/allowance/currentness and producer checkpoints;
held-unsent and transmitted-but-unconsumed outcomes stay distinct. A later
replay branch must not be flattened into predecessors of the original host
call or consume. No new audit constructor, general trace schema, permission
profile or semantic retry is needed. If the assertions cannot establish the
required facts without manufacturing or exposing denied observations, reopen
only the alternative narrowly enumerated v2 state/branch projection.

Slice1's bounded gate passes, parent read `2026-09-09T03:41:42+09:00`:
actual v2 probe14/14; genuine local execution/M9 tests17/17; parser/read/writer
7/7; control9/9. The synthetic untrusted decoder baseline first exposed a
test-only enum spelling error; after correcting it to the existing private
Serde spelling, isolated profile/schema/missing-slot/predecessor negatives
pass. Those negatives establish structural validation, not authenticity or
cryptographic attestation. The actual producer path supplies the positive
joined provenance. Release the bounded two-session fault slice to separate
runtime, host and test writers; no provider/component or I3-3 acceptance is
implied at that checkpoint.

Slice2 subsequently executes19/20 actual probe cases (parent log read
`2026-09-09T04:34:37+09:00`): duplicate delivery, sent-result discard before
semantic admission, and post-call retirement pass, as do the prior16 cases.
Only the held result's first send/consume on session2 fails; its requester
incorrectly enters a generation1-only receive guard. A private shared receive
body repair preserves the normal guard and existing currentness checks.
The sent/discarded case is not wire packet loss. These fault results are
child-private assertions with generic completion, not exported fault traces.
An independently authored synthetic second-slot terminal candidate also
reproduces an intended decoder RED; normal v2 producer/decoder must reject
that unsupported shape without reducing private two-slot retention capacity.
Both bounded repairs precede the fresh full-slice gate and independent review;
no provider component or milestone acceptance is claimed.

After those repairs, the exact second-slot guard and held-session2 positives
both pass1/1; the complete actual provider probe passes20/20 (17.90s bodies,
parent read `2026-09-09T04:40:28+09:00`). This supersedes the19/20 behavioral
checkpoint, not the still-pending broad regression, lint and independent
acceptance gates. No additional fault-profile expansion is required by the
independent finite acceptance inventory: existing unsupported export/restore
guards must rerun; actual process-local cut admission remains row20.

Provider Stage3 final runtime gate is now met. Independent review's one P2
mode/budget incompatibility is reproduced by a genuine held-session2 test and
repaired with early network-profile audit denial plus an unspent-budget
completion guard. Its observed RED is generic `RequesterTerminal`, not an
externally demonstrated unauthorized audit. Fresh post-repair tests pass
runtime library410, all probe targets (ordinary41/provider20), process64,
M1067, I2 local5/CLI8 and provider public guards5; scoped Clippy and format
pass. Independent review has no remaining P0/P1/P2. Report2606 pins the exact
commands and distinguishes the earlier full runtime all-target packet from
these fresh affected-target reruns. Parent prepares finite component
integration, not another milestone or whole-I3-3 acceptance. No additional
EMFILE/B65/general fault-trace work is admitted by this completed consumer.

Next row13 selects the smaller checked `init_avatar_hp` A-to-B path and a
genuine parent-produced retirement of its source-declared WorldAuthority
membership, replacing the earlier `init_focus`/reversed-role sketch below.
This requires its own exact membership-successor validation and qualified
install/ACK/publication within the existing chain, not reuse of an
owner-capability-revocation predicate or a child issuer. Generic M9/SYS4
construction/install/publication need no use of the retired ContractUpdate
or observer lineage; those retirements must remain effective. Preserve the
normal old-carrier `CarrierAdmissionRejected` precedence and pair the actual
network case with LOCAL current-generation `StaleMembership` classification.
Implementation stays gated on provider commit/push/parity; no source grammar,
arbitrary principal selector or new authority policy is authorized by this
LAB consumer substitution. Rows15/19 reuse their exact evidence before row20
cut/ordering and whole-I3-3 acceptance.

The old fixture
default remains0; the feature-gated source-real factory now explicitly selects
a fixed real41 fixture, with fresh -7 and absent-target variants, never caller
bytes, native paths or an injected result.

Stop expansion when these direct runtime consumers and falsifiers, required
regressions and independent review pass. Rows 13/15/19/20, remaining ordering
and whole-I3-3 validation/acceptance still follow; only that complete milestone
and observed push/parity trigger the owner's stop. I3-4 remains inactive.

Default M8/M9/SYS5/kernel execution remains rejecting, and new provider
fragments must explicitly fail the SYS4 completeness boundary until a genuine
composite admission exists. Static snapshots cannot become ordinary executable
images. `RuntimeUnsupported` is retained as an exact activation-pending
requirement, not waived or declared discharged. Actual binding/permit/runtime/
supervisor/call/cut evidence remains required in the dependent components.

Historical Stage 2a ownership (superseded for current Stage 2c):
`time_host_driver` owned the semantics composite checker and SYS3
static production files; `c3_test_repair` owned the separate semantics test
target, `time_network_tests` the separate private runtime tests. Only necessary
downstream explicit refusal/compile guards were delegated to `c3_runtime_fix`;
dynamic composite work waited for the static interface freeze. One evaluator
ran commands, parent owned integration, and independent reviewers remained
separate from authors. A necessarily new API's compiler gap is recorded as
such; an assertion RED on a fail-closed interface precedes behavior completion.

The sole evaluator serializes Cargo with `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=2`, `--locked` and serial tests. Before heavy commands inspect
disk/memory; below 10 GiB free, do not start another heavy command or silently
delete artifacts. Final I3-3 gates include runtime feature-union library,
process integration, `i3_request_lifecycle_model`, I2/M10 targets, all probe
tests, workspace all-target tests/Clippy, format, docs, diff and secret scan.
Overlapping suites have separate result classes, never additive counts.

Independent remaining-row sequencing review keeps the following direct
consumers after the time path. Membership uses the actual M9 retirement of
ParticipantA and the already checked `init_focus` operation, not capability
revocation renamed as membership. Auth/policy can exercise real pre-activation
auth-discharge rejection; do not claim dynamic policy revocation from it.
Visibility/redaction can reject a changed existing redaction contract through
the actual tainted process-image admission boundary, paired with local
visibility/consume negatives; no invented owner-message visibility field or
designated network crossing is required. Process cut admission must check
actual retained adapter ingress and unresolved SYS5 requests in addition to
local fabric queues, and execute a quiescent positive plus in-flight denial.
None of these sketches is executed evidence or permission to omit a row.
Relation/designated ordering uses executed local producer positives and
falsifiers in I3-3, accurately labelled local; their actual cross-process
pressure remains I3-4. Unsupported-carrier rejection alone cannot replace a
positive ordering producer. The time consumer is now integrated at `55f1fd7f`
and ADR-0042/spec/17 selects the distinct provider contract; its implementation
and all remaining evidence are still required.

Parallel test-only work within this same I3-3 ordering obligation is admitted:
`c3_test_repair` owns `sys5_relation_dispatch_tests.rs` to make the real
fallback publication's causal path through ViewerC's later access explicit.
Direct consumer: the I3-3 `fallback -> later access` ordering row. Blocker
reduced: existing initial-publication causality and fallback-shadow tests
cover the facts separately. Acceptance use: one focused actual LocalFabric
causal assertion/falsifier, reusing existing APIs without production or
cross-process changes. The mapper's existing relation/designated and M10
runtime-backed tests remain reusable local evidence, not new network claims.
After that local test passes, the same test writer separately owns one
test-only addition in `tests/sys5_i3_process_runtime.rs`: flip the existing
image carrier's `reference_only_redaction` field and require the actual
decoder to reject it before releasing an executable candidate, paired with
the unchanged source-derived image's validated start. Direct consumer: row19.
Blocker reduced: the production fail-closed guard has no exact image-level
falsifier. Acceptance use: focused decoder test now1/1 GREEN after correcting
the test's JSON pointer to top-level `projection`, without production changes;
both original-image start and changed-redaction `Malformed` execute. No new
wire field, dynamic policy or network visibility claim.

The read-only post-provider planning review confirms row20 is process-local
cut admission, not simultaneous distributed quiescence or persistence.
Runtime queues, unresolved requests/remote-unknown states, one-use admission
reservations, staged lifecycle obligations and actual adapter acquisition /
complete ingress must be checked and committed under the same serialized
ownership boundary. Completed decisions and spent allowances remain retained
history, never reset by a cut/reconnect. A positive actual ordinary round
trip must reach this boundary and then a later checked transition; pending,
retained-ingress, reservation and late-old-traffic cases are falsifiers.
These are implementation/evidence obligations, not executed results or a
new milestone. Runtime/probe production work remains provider-first.
The smallest later transition is a genuine existing checked owner dispatch /
serve, causally joined to an identity-bound committed cut-admission occurrence.
Independent Canon-first review confirms plan/05 requires that actual retained
runtime/adapter boundary, not new checked-patch installation. Reject the larger
patch-installation alternative for this finite row: it adds a consumer not
required by the quiescence rule. The admission occurrence is evidence, never
authority, a saved image or restore support. This selection does not waive any
pending/ingress/ambiguity/lifecycle/late-traffic falsifier or activate row20
implementation before the provider component closes.

## I3-4 milestone contract — inactive until I3-3 closes and owner resumes

**Goal:** Produce actual C-distributed positive and representative falsifier
evidence for frozen SCN-01, SCN-02, SCN-03, and SCN-06, plus bounded maintained-
relation and designated-result cross-process pressure, without theorem or
product overclaim.

**Entry:** I3-3 failure/order boundary accepted, Report 2606, pushed parity,
explicit owner resume after the scheduled I3-3 pause,
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
| OPEN-032 transport selection | I3-0 authorized ADR | same-gate A/B comparative evidence | resolved for this bounded program by PROPOSAL-040 / ADR-0037: private QUIC reliable stream selected; TLS/TCP deferred baseline; datagrams excluded |
| Supported OS/CI/browser-feasibility matrix | I3-0 selection record | reproducible candidate probes | Linux x86_64 localhost only; macOS/Windows/browser/production explicitly untested |
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
- Both retained reliable-stream candidates satisfied the equal bounded I3-0
  canary; ADR-0037 selected QUIC at criterion 10, the first material difference
  after criteria 8--9 also tied, while
  full plan/05 failure/runtime criteria remain later milestones.
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
| I3-0 canary overclaim | fixed cache/owner-request probe is called actual owner runtime, durable or exactly-once | preserve P2 residual table; I3-1/2 replace only through direct-consumer evidence |
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
production source, tests, samples, lifecycle, public compatibility, or
external state. Transport selection authority is PROPOSAL-040 / ADR-0037, not
this LAB file. The bounded program does not authorize QUIC
datagrams, WAN/production deployment, public release, durable distributed
save/load, live distributed patch, consensus, hidden distributed transaction,
global exactly-once, general scheduler/fairness/security/noninterference proof,
browser/renderer product, Domain Kit, Reversed Library, PrismCascade
integration, or Typed-Effect platform collapse.

## Recommended next action

From the integrated `55f1fd7f` time-and-reply checkpoint, implement the
ADR-0042/spec/17 provider contract and complete the remaining I3-3
failure/order inventory. Preserve all 20 families, applicable regressions and
independent acceptance review. After I3-3 commit/push and remote parity, stop
and report the accepted boundary. I3-4 remains inactive until explicit owner
resume; the original fixed program through NEXT-0 is neither removed nor closed.
