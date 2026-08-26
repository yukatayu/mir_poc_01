# Plan 249 — Mirrorea I2 Systems Foundation current roadmap

最終更新: 2026-08-26 23:09 JST

## 役割、authority、current control state

これは PROPOSAL-029 / ADR-0026 で owner が承認した bounded program

```text
Mirrorea I2 Systems Foundation
```

の **sole current LAB execution roadmap** である。規範正本は
`mirrorea_canon/`、公式 Gate / Phase は
`mirrorea_canon/plan/01-phases.md`、proof status は
`mirrorea_canon/theory/11-metatheory-ledger.md` だけが決める。この LAB roadmap
は実行順、依存、Goal Statement、受理証拠、停止線を保持するが、単独では
Canon、Gate、Phase、SCN、OBL、conformance、public compatibility を変更しない。

- program start revision: `49e6845ada990a1c9d9944896a5ff1754994a1b3`
- immutable M10 implementation/validation baseline:
  `23f5a8130334bf0c8516d51e9dcea38b92f50db1`
- parent goal: one ordinary `.mir` source から meaning-preserving per-locus
  executable artifacts と generated communication plan を生成し、単一 OS
  process 内の independent locus runtimes で実 dispatch し、source から runtime
  occurrence まで typed devtools で追跡できる local toy fabric を完成させる。
- completed goals: prior ADR-0015 **M0--M10** baseline, **SYS-0 baseline and
  goal alignment**, and **SYS-1 runtime kernel / internal carrier boundary**
- accepted SYS-1 source/evidence cut:
  `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`
- active goal: **SYS-2 concurrency, memory, and effect-handler refinement**
- next goal: **SYS-3 per-locus projection and executable artifact generation**
- current direct blocker: the SYS-1 kernel consumes an immutable final M9
  authority snapshot; SYS-2 must define the ST/OW happens-before,
  linearization, and visibility mapping that prevents stale use after
  revoke/publication/activation/cut edges without importing low-level memory
  order into ordinary Surface.
- official lifecycle: theory **T1**。program activation and SYS-0/SYS-1
  completion do not accept broad PHASE-I1 exit, I2 lifecycle entry, or I2 exit.

Plan 247 は closed M0--M10 execution record / R5 regression baseline のまま保持する。
Plan 249 以外の numbered plan、WRK、historical report は parallel queue ではない。

## Program parent goal acceptance

Program の user-visible completion は、既存 CLI convention と整合する最小 command
set から次を再現できることである（最終 CLI spelling は internal / provisional）。

```text
build/project ordinary.mir
run-local ordinary.mir
inspect ordinary.mir
conform-i2
```

一つの source から checked global Core、per-locus executable plans、generated
communication graph、typed internal carriers、runtime trace、minimal causal
devtools view を得る。fixture-name dispatch、expected-result lookup、source の
runtime 再解釈、直接 cross-locus store mutation は禁止する。

Parent-goal stop evidence:

- at least three loci plus the SYS-5 four-locus toy scenario;
- generated artifacts actually execute in ST and selected OW profiles;
- request/dispatch/receive/serve/failure and source/Core/artifact/occurrence
  correspondence are visible;
- owner, authority, failure, effect, lifetime, observation, fallback,
  designated-result, save, and patch invariants stay explicit;
- finite I2 conformance and independent review pass without widening a bounded
  result into a general theorem or public compatibility claim; and
- SYS-7 leaves only an inactive I3 goal/entry contract and starts no transport.

## Fixed execution order

Milestone addition or reordering requires evidence that the parent goal cannot
close without it and an explanation in this roadmap. No such addition exists
through SYS-1.

```text
SYS-0 Baseline and goal alignment
  → SYS-1 Runtime kernel / conformance separation and internal carrier boundary
  → SYS-2 Concurrency, memory, and effect-handler refinement
  → SYS-3 Per-locus projection and executable artifact generation
  → SYS-4 In-process generated dispatch runtime
  → SYS-5 Minimal typed devtools and local virtual-space vertical slice
  → SYS-6 I2 assurance, conformance, and lifecycle closeout
  → SYS-7 I3 entry contract only
```

## Common milestone close contract

Each milestone closes the applicable subset of one normative semantic rule,
one executable direct-consumer behavior, at least one positive case, one
representative falsifier, exact proof/model/runtime classification,
source/implementation correspondence, one independent review, fresh focused
validation, one milestone report, integration commit/push, and remote parity.
Not-applicable or skipped evidence is stated explicitly rather than counted as
pass.

For every subtask or research item, record:

```text
Direct consumer:
Blocker reduced:
Acceptance use:
```

Do not start it if it neither advances per-locus generation/dispatch nor closes
a required meaning/authority/failure/memory boundary, lacks a direct consumer,
or cannot close with a positive case and falsifier. A new WRK is exceptional
and also needs a reason the milestone report cannot hold the investigation,
plus an adoption/discard rule.

For a design question compare only the current/smallest design and one viable
alternative. Decide in order: meaning preservation; authority/privacy/safety;
ordinary Surface; no hidden communication/failure/effect/transaction; small
Core; determinism/inspectability; finite decidability; modular proofability;
conservative extensibility; implementation simplicity; performance.

Stop local generalization when the accepted choice preserves the Constitution,
runs the positive case, detects the primary falsifier, is usable by its direct
consumer, has a conservative extension boundary, freezes no public contract,
and independent review finds no major counterexample.

## Goal Statements

### SYS-0 — Baseline and goal alignment (completed)

**Goal ID:** SYS-0

**Goal sentence:** By the end of this milestone, the repository has one
owner-authorized execution program and one current goal/control path that can
advance the accepted M10 semantics toward per-locus generation without
changing their lifecycle acceptance.

**North Star link:** Places the next work on the direct path to correct
placement, generated communication, verification, typed observation, and
eventual checked evolution; primarily aligns all five before implementation.

**User-visible outcome:** A reader can identify the exact baseline, sole
roadmap, active goal, first blocker, acceptance evidence, lifecycle boundary,
and next executable capability without reconstructing historical reports.

**Semantic invariants:** Canon > LAB; M10 cut immutable; ordinary source is
semantic authority; no hidden communication/authority/effect/failure; official
T1 and broad I1/I2 non-acceptance preserved; public contracts remain open.

**Direct consumer:** SYS-1 runtime-kernel and internal-carrier separation.

**Non-goals:** OPEN-030 semantic resolution, Rust refactor, new runtime,
projection artifacts, I1 exit, I2 lifecycle entry, public contract freeze.

**Primary falsifier:** More than one document claims to be the current roadmap,
or program activation is reported as broad I1 exit/I2 acceptance.

**Exit evidence:** HEAD/remote/worktree baseline; focused M10 regressions;
agent/planner capability validation; PROPOSAL-029/ADR-0026/Plan 249; complete
meta-alignment matrix; current-pointer sync; prechange and close independent
reviews; Canon index/hierarchy/docs/diff validation; SYS-0 report; commit/push
and remote parity.

**Stop condition:** Close when there is one authority record, one roadmap, no
North-Star contradiction, M10 regression is preserved, lifecycle non-claims
are explicit, and SYS-1 has a direct blocker/consumer. Reopen for baseline
regression, conflicting active roadmaps, unrecorded authority, or a material
review counterexample.

### SYS-1 — Runtime kernel / conformance separation and internal carrier boundary (completed)

**Goal ID:** SYS-1

**Goal sentence:** By the end of this milestone, the ordinary `run_source` and
generic checked `OwnerEvent` production paths use a typed internal semantic
kernel independent of conformance/release orchestration, with explicit owner
request/receipt and designated remote-input request/result/consume lifecycles.

**North Star link:** Advances correct communication and observation while
preserving verification, authority, and future placement boundaries.

**User-visible outcome:** Ordinary source execution and generic checked owner
events run through a reusable kernel; focused inspection shows owner request →
serve → reply → receive/receipt and designated remote-input request →
source-owner serve → reply → receive/receipt → evaluator consume with exact
provenance. Specialized historical M10 SCN-04/09/10/route-patch runners remain
regression-only and are not this evidence.

**Semantic invariants:** Conformance depends on the kernel, never the reverse;
carrier cannot mint Core/state/authority; receipt is not authority transfer;
queue position is not identity; source/Core spans, occurrence identity,
failure/effect rows, redaction, membership/incarnation, capability/witness,
frontier, and consumption remain typed; no hidden retry/exactly-once.

**Direct consumer:** SYS-2 maps concurrency/effect ordering to this contract;
SYS-3 consumes the kernel and carrier from the projection/compiler boundary.

**Non-goals:** Public ABI/wire, transport implementation, performance rewrite,
threaded execution, per-locus artifact generation, final CLI compatibility.

**Primary falsifier:** A source-free carrier or conformance fixture can cause a
semantic mutation/grant, or the kernel imports release/profile/verifier logic.

**Exit evidence:** Source cut `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`;
crate-private typed kernel with M9→kernel→owned M8 dependency direction;
13/13 focused owner/designated-input lifecycle and falsifier tests; ordinary
source/generic owner integration; `mir-runtime` library, M10 source/CLI/
conformance, workspace, format, Clippy, diff, and secret-scan validation;
independent semantics and code-quality ACCEPT; runtime-monitored only; no
public compatibility claim.

**Stop condition:** Close once SYS-2/3 can use the internal boundary, the
representative falsifiers fail closed, and the immutable-M9-snapshot
revocation visibility plus broad-I1 OPEN-026/027/full-carrier residuals are
recorded exactly. Broad I1 exit is optional here and never obtained by
weakening criteria. Reopen on source-free mint/mutation, wrong lineage or
identity acceptance, kernel dependency on M10 orchestration, or stale use
past the visibility edge SYS-2 must define.

### SYS-2 — Concurrency, memory, and effect-handler refinement (active)

**Goal ID:** SYS-2

**Goal sentence:** By the end of this milestone, ST and one-owner-worker (OW)
backends execute the selected kernel fragment such that implementation traces
refine Mir abstract occurrence/dependency order and preserve the same allowed
semantic observations.

**North Star link:** Makes generated work execute correctly under local
concurrency while keeping communication, verification, and observation tied to
high-level semantic order.

**User-visible outcome:** Selected scenarios run in deterministic ST and OW;
the trace exposes linearization and required visibility edges, while negative
litmus variants detect a removed or reordered edge.

**Semantic invariants:** `atomic_cut` is not a fence; Surface does not acquire
`memory_order_*`; owner state is data-race-free and owner-local mutation is
serialized; publication/revocation/activation/cut/relation epochs preserve
abstract happens-before; fairness/liveness stays explicit and deferred.

**Direct consumer:** SYS-3 embeds backend requirements in locus artifacts;
SYS-4 runs the same artifacts under ST and OW.

**Non-goals:** Lock-free runtime, arbitrary memory model/scheduler theorem,
real networking, performance optimization, ordinary-Surface memory syntax.

**Primary falsifier:** Removing a required request→serve, publish→observe,
grant/revoke→use, activation→request, or cut→mutation visibility edge is not
detected, or ST/OW yield different permitted semantic results.

**Exit evidence:** ST preserved; OW execution; abstract happens-before,
linearization, reads-from/coherence mapping where needed; owner data-race
freedom evidence; required litmus/model cases including owner messaging, store
buffering, publication, witness/capability, patch, save, relation epoch, two
RMW, and presentation-gap nonmutation; exact proof/model/runtime classes;
independent review.

**Stop condition:** Close when all required edges have a finite mapping and
counterexample, selected ST/OW observations agree, and SYS-3 has a concrete
backend contract; defer arbitrary fairness/memory generalization.

### SYS-3 — Per-locus projection and executable artifact generation (next)

**Goal ID:** SYS-3

**Goal sentence:** By the end of this milestone, one checked global Core plus
declared logical topology deterministically produces executable per-locus
plans and all communication/effect/observation/persistence plans needed by the
accepted finite fragment.

**North Star link:** Directly realizes correct placement and generated visible
communication, with verification/observation provenance preserved for later
execution and evolution.

**User-visible outcome:** Building one source for at least three loci emits a
GlobalProjectionResult, LocusProgram per locus, CommunicationPlan,
EffectHandlerPlan, ObservationPlan, PersistencePlan, SourceMap, and diagnostics
without hand-written message interfaces.

**Semantic invariants:** No hidden communication/authority/failure/effect;
owner/site/source span/relation lineage/observation/cut/patch obligations
preserved; same-owner RMW remains in owner artifact; designated expression is
not re-executed at consumers; explicit receipts only for cross-owner results.

**Direct consumer:** SYS-4 starts the generated artifacts without re-parsing
source semantics; SYS-5 displays their causal correspondence.

**Non-goals:** Optimal placement, final exchange schema, public ABI/wire,
arbitrary relation-DAG theorem, transport, runtime execution itself.

**Primary falsifier:** The projector emits a communication edge not implied by
checked Core, omits a required edge/failure/obligation, moves owner mutation to
a requester/consumer, or accepts a malformed/cyclic projection.

**Exit evidence:** Deterministic projection at three or more loci; artifact
identity tied to checked source/Core; visible generated communication; no
manual interface fixture; positive/malformed cases; one three-step fallback or
shared-ancestor pressure graph via a conservative finite-DAG boundary;
project-then-evaluate coherence for accepted relation fragment; M10 behavior
reproducible; independent review.

**Stop condition:** Close when SYS-4 can execute artifacts without semantic
reconstruction and every accepted Core operation has complete visible
placement/communication or a typed projection diagnostic. Defer general DAG
theory and optimized codegen.

### SYS-4 — In-process generated dispatch runtime

**Goal ID:** SYS-4

**Goal sentence:** By the end of this milestone, independent locus runtimes in
one operating-system process execute SYS-3 artifacts and dispatch only their
generated communication through explicit endpoints under both selected ST and
OW profiles.

**North Star link:** Turns correct placement and communication into actual
execution while preserving verification, observation, and checked evolution
boundaries.

**User-visible outcome:** A local run shows a request cross a locus endpoint,
receive/serve or typed failure, owner mutation, deterministic replay, and a
consistent local multi-locus save/restore/patch state.

**Semantic invariants:** No global unpartitioned mutable store; no direct
cross-locus mutation; schedule cannot mint Core/authority/state/expected
result; transport metadata is non-authority; typed failures fail closed;
save/patch include artifact and communication state; rejected patch mutates
only lifecycle rows.

**Direct consumer:** SYS-5 builds the toy world and causal devtools from actual
generated dispatch; SYS-6 verifies the profile.

**Non-goals:** Socket/multi-process transport, WAN, durability, consensus,
exactly-once, production deployment, final public runtime API.

**Primary falsifier:** The same scenario succeeds only through fixture-name
plan selection, source re-interpretation, handwritten communication, direct
remote-store access, or schedule-created semantic facts.

**Exit evidence:** LocusRuntime/local store/queues/endpoints/views/trace;
actual endpoint crossing; ST/OW selected scenarios; request→dispatch→receive→
serve/failure trace; source/Core/artifact/occurrence correspondence; route,
membership, capability, witness, duplicate/stale receipt, target, split-frame,
revocation, and patch-frontier failures; deterministic replay; local whole-
fabric cut/save/restore and bounded patch; independent review.

**Stop condition:** Close when SYS-5 can compose a real four-locus scenario
from generated artifacts and no selected success path bypasses locus
endpoints, ownership, or typed admission.

### SYS-5 — Minimal typed devtools and local virtual-space vertical slice

**Goal ID:** SYS-5

**Goal sentence:** By the end of this milestone, one ordinary Mir source or
small module set runs a four-locus local toy virtual space through generated
artifacts/dispatch and exposes one observer-safe typed causal view from source
span to runtime behavior.

**North Star link:** Makes placement, communication, verification,
observation, and checked evolution jointly understandable in one finite
vertical slice.

**User-visible outcome:** A new user follows a short walkthrough to inspect the
source, build/project, run interactions/faults, and view owner attack RMW,
designated publication, maintained relation/fallback, auth/verification,
save/restore, and accepted/rejected patch causality.

**Semantic invariants:** World/Participant/Viewer/Bird stay sample/library
vocabulary; relation and presentation fallback stay separate; designated
result is not re-decided; raw credential/capability/witness payloads never
leave observer-safe projection; devtools rows derive from typed occurrences.

**Direct consumer:** SYS-6 finite I2 conformance/assurance profile and usability
review.

**Non-goals:** Browser renderer, final View/FFI, production UX, package
marketplace, public API/grammar freeze, real transport.

**Primary falsifier:** The demo joins expected JSON or helper files manually,
uses a thick fake wrapper instead of generated dispatch, leaks a secret, or a
Viewer sample gap changes semantic relation lineage/fallback.

**Exit evidence:** WorldAuthority/ParticipantA/ParticipantB/ViewerC; admission,
membership, capability, witness; owner-side attack; designated tick; B-owned
bird relation to A shoulder with B fallback; Viewer projection; A leave;
semantic fallback versus temporary presentation gap; fresh reacquire;
observer-safe trace; save/restore; accepted/rejected patch; policy-layer
attach/remove/revocation; optional verification residual/discharge; one joined
devtools report; concise walkthrough; positive/negative tests; usability and
semantics reviews; M10 regression.

**Stop condition:** Close when the local scenario is reproducible with a small
command set, its causal line is visible in one viewer/report, its main
falsifiers fail closed, and SYS-6 can consume its exact artifacts/evidence.

### SYS-6 — I2 assurance, conformance, and lifecycle closeout

**Goal ID:** SYS-6

**Goal sentence:** By the end of this milestone, a finite source-first I2
profile independently verifies that global Core projects to and executes as
meaning-preserving per-locus artifacts with generated communication, ST/OW
selected correspondence, and observer-safe causal traceability.

**North Star link:** Supplies explicit verification for the new placement,
communication, observation, and evolution capability and decides lifecycle
state only from actual evidence.

**User-visible outcome:** One conformance command reports the exact source,
Core, artifacts, generated edges, runtime traces, pass/fail rows, evidence
classes, non-claims, and accepted implementation cut.

**Semantic invariants:** Ordinary source remains semantic authority; no hidden
communication/remote store/authority mint; failure containment; relation
coherence and fallback separation; designated non-reexecution; no stale
save/patch mutation; observer-safe output; bounded evidence is not general
proof.

**Direct consumer:** SYS-7 uses the accepted I2 boundary to formulate, but not
activate, an I3 real-transport entry contract.

**Non-goals:** General metatheory, arbitrary scheduler/relation DAG, public
ABI/wire, real transport, production, durable distributed state, I3 work.

**Primary falsifier:** Conformance can pass while a checked Core operation is
missing from generated communication/artifacts, an artifact changes owner,
ST/OW differ semantically, or authority/state appears without source/admission.

**Exit evidence:** Finite I2 conformance rows for source authority, projection,
generated communication, actual dispatch, ST/OW correspondence, no-hidden
edges/store/authority, failure containment, relation/fallback/designated
properties, source→Core→artifact→trace correspondence, save/patch, and safe
devtools; finite evidence for projection determinism, owner preservation,
communication completeness, ST/OW correspondence, owner data-race freedom,
visibility edges, no source-free minting, relation coherence; exact evidence
classification; M10 regression; independent review; exact cut/commands/non-
claims/risks.

**Stop condition:** Close when the finite profile and review accept the parent
capability. Move broad I1/I2 lifecycle markers only if their pre-existing
actual criteria are met; otherwise record exact residuals without weakening
criteria.

### SYS-7 — I3 entry contract only

**Goal ID:** SYS-7

**Goal sentence:** By the end of this milestone, the accepted I2 boundary has
one inactive next-program goal and entry contract for preserving authority,
failure, provenance, and ordering across two or more OS processes, without
starting real transport implementation.

**North Star link:** Defines the next conservative step for real generated
communication while preserving verification and observation guarantees.

**User-visible outcome:** Current status names what must be true before a
two-process C-distributed profile starts, which decisions remain open, and
which work is explicitly inactive.

**Semantic invariants:** Transport is not authority; internal carrier and
public wire remain separate; disconnect/reconnect/duplicate/reorder and
network-order refinement are explicit; no hidden transaction/exactly-once;
source provenance and typed failure remain end-to-end.

**Direct consumer:** A future owner-authorized I3 bounded program; none is
activated by SYS-7.

**Non-goals:** Transport selection or implementation, socket code, deployment,
production security claim, public ABI/wire freeze, I3 lifecycle entry.

**Primary falsifier:** The entry contract assumes transport identity grants
authority, omits duplicate/reorder/disconnect failures, or requires selecting
a final wire/transport before a future program begins.

**Exit evidence:** One goal sentence; candidate limit of two; internal/public
carrier split; failure matrix; disconnect/reconnect and duplicate/reorder
requirements; concurrency/network-order refinement; C-distributed scenarios;
explicit non-production/non-freeze boundary; status sync and independent
review.

**Stop condition:** Close and stop the program when the entry contract is
reviewed and recorded as inactive. Do not implement I3 without new owner
direction.

## SYS-0 meta-alignment matrix

This is a decision aid for program scope, not a new semantic encyclopedia.

| Owner intent | Current Canon representation | Current implementation evidence | Current gap | This program consumer |
| --- | --- | --- | --- | --- |
| meaning-derived communication | North Star, C3, BND-001/006 | finite source-to-Core/generated-edge M10 evidence | no general per-locus executable communication path | SYS-1 carrier; SYS-3/4 projection/dispatch |
| per-locus code generation | S4 projection, `mir-project` responsibility | no accepted generator | global checked Core does not emit runnable locus artifacts | SYS-3, then SYS-4 |
| ordinary Surface | C2, M6/M7 bounded source path | accepted ordinary `.mir` finite profile | not final grammar; no I2 build/project path | SYS-3/5 |
| owner evaluation | C4/C5, theory/13, M7/M8 | owner-side RMW, FIFO runtime, and SYS-1 crate-private kernel | concurrent worker refinement absent | SYS-2/4 |
| relation-first late projection | C7, theory/14, BND-006 | finite two-anchor relation/consumer projection | artifact placement and conservative DAG extension boundary absent | SYS-3/5 |
| existence/fallback DAG | theory/06/14 monotone lineage | two-anchor/three-floor finite evidence | broader finite acyclic pressure case not projected/executed | SYS-3/5 |
| designated evaluation/timing | C6, theory/13, M7/M8 | version/frontier decision and finite consume path | generated evaluator placement/delivery across locus artifacts absent | SYS-3/4/5 |
| auth layers | C9, theory/05/18, M9 | source-bound membership/capability/witness seam integrated into SYS-1 carrier | component dispatch and revoke-after-enqueue/serve visibility absent | SYS-2/4/5 |
| optional verification | C9/C11, M9 verifier lane | residual/evidence/diagnostic finite seam | runtime/devtools residual/discharge path not joined | SYS-5/6 |
| algebraic-effect-like visible operations | effect/failure rows, provider/adaptor BNDs | bounded designated remote-input request→source-owner→result→consume contract | ST/OW ordering and later generated handler plan absent; generic registry intentionally unclaimed | SYS-2/3/4 |
| save/load and Z-cycle | C10, theory/04, M8 local cut | finite local save/restore; general Z-cycle obligations deferred | whole in-process multi-locus artifact/communication cut absent | SYS-4/5/6 |
| atomicity / memory order | ADR-0007, theory/04 high-level edges, owner seriality | deterministic ST reference | no OW happens-before/visibility/refinement mapping | SYS-2/4/6 |
| browser/headless participation | North Star browser participation horizon; host boundary | headless/helper LAB evidence only | no I2 local causal toy workflow | SYS-5 headless; browser deferred |
| View / FFI | BND-007 provider/View split | historical LAB/provider evidence | final View/renderer and public FFI not selected | deferred; SYS-5 only minimal report |
| hot-plug | C10, theory/08, finite M8/M10 patch | accepted/rejected bounded patch evidence | generated-artifact/dispatch lifecycle integration absent | SYS-4/5/6 |

The matrix supports the fixed systems path. It does not reopen arbitrary
metatheory, select a public contract, or promote historical LAB product lines.

## Dependency, ownership, and evidence map

| Milestone | Primary writers | Direct dependency | Required evidence emphasis |
| --- | --- | --- | --- |
| SYS-0 | parent + planner/config writers | owner direction, M10 baseline | authority/pointer consistency, regression, agent config, docs/review |
| SYS-1 | implementer + theory/test writers | M10 boundary, OPEN-030 | dependency inversion, carrier no-mint/source identity, effect ordering |
| SYS-2 | implementer + test/formal writers | SYS-1 kernel/carrier | ST/OW refinement, litmus/model checks, owner data-race freedom |
| SYS-3 | projection implementer + test/formal writers | SYS-1/2 contracts | deterministic projection, completeness, owner/span preservation |
| SYS-4 | runtime implementer + test writer | SYS-3 artifacts, SYS-2 backends | real endpoint dispatch, fail-closed negatives, replay/save/patch |
| SYS-5 | implementer + docs/test writers | SYS-4 runtime | four-locus vertical slice, joined typed devtools, usability/security |
| SYS-6 | conformance/formal/test writers | SYS-3--5 accepted cuts | finite I2 profile, exact evidence classes, lifecycle/non-claims |
| SYS-7 | planner/theory writer | accepted SYS-6 boundary | inactive I3 entry contract and no transport work |

The parent/orchestrator owns integration, commit, push, and remote parity.
Production source has one writer by default. The author of a change does not
serve as its independent reviewer.

## Current acceptance evidence and blockers

### SYS-0 accepted close evidence

- `HEAD == origin/main == 49e6845ada990a1c9d9944896a5ff1754994a1b3`
  at baseline; main was clean and M10 cut is an ancestor with three docs-only
  commits after it.
- focused M10 regression groups passed 67 + 2 + 4 + 3 + 5 tests with zero
  failure before SYS-0 edits.
- agent configuration validation passed; its focused unit tests passed 9/9;
  the strict-config help probe exited 0. Planner has the delegated
  planning/status write capability.
- root had about 70 GiB free and memory about 11 GiB available before work.
- Oracle was attempted once but the browser profile was logged out before
  prompt submission; it produced no advice and was not retried without new
  failure evidence.
- prechange independent planner review returned GO with no North Star
  contradiction or mandatory-stop finding.
- post-edit Canon index regeneration/check passed at 172 files; documentation
  validation passed with source hierarchy 799/799 and zero missing paths;
  `git diff --check` and the numbered-plan registry test passed. Independent
  close review then returned REJECT without P0 because authority-entry files,
  the primary HTML reader, current-state timing, and report command detail were
  incomplete. Forward correction cycles closed those findings and the final
  independent close review returned **ACCEPT — no P0/P1/P2**.
- accepted SYS-0 integration cut:
  `350e04b400ee5e50147b78af5f5313c761eeaee9`
  (`docs(i2): start systems foundation program`), pushed as
  `49e6845a..350e04b4 main -> main`.
- `git ls-remote --heads origin refs/heads/main` returned
  `350e04b400ee5e50147b78af5f5313c761eeaee9 refs/heads/main`; local
  `HEAD == origin/main` at the same cut and `git status --short --branch`
  returned clean `## main...origin/main`.
- This mechanical current-view closeout is the immediate successor to the
  accepted integration cut and cannot self-embed its own commit hash. The
  parent records its final push/parity before beginning SYS-1 source work.

### SYS-1 accepted close evidence

- Accepted source/evidence cut:
  `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`.
- `SemanticRuntimeKernel` is crate-private and is the ordinary `run_source`
  and generic checked `OwnerEvent` production path. It consumes the sealed M9
  seam, owns/extracts M8, and imports no M10 profile/verifier/release/CLI
  orchestration.
- The owner and designated remote-input lifecycles preserve checked
  source/Core provenance, origin/target lineage, typed outcome and
  effect/failure rows, occurrence identity, visibility/redaction,
  membership/incarnation, capability/witness, and applicable frontier/
  consumption state. Receipt transfers no authority; queue position is not
  request identity; pre-admission failure creates no occurrence/M8 enqueue.
- Thirteen focused tests cover valid lifecycle, FIFO identity/result alignment,
  declared failure, malformed factory diagnostics, and invalid/duplicate/
  stale/wrong target/source/origin/visibility/authority falsifiers. Formatting,
  warnings-denied changed-crate Clippy, 25/25 runtime library tests, M10 source
  2/2, CLI 4/4, conformance 67/67, and the full workspace passed.
- Independent semantics and code-quality reviews returned ACCEPT. Evidence is
  `runtime-monitored`; no Lean/model-check/general theorem was added. The
  Oracle attempt produced no advice because its private browser profile was
  logged out before prompt submission.
- OPEN-030 is resolved only for this narrow I2-internal contract.
  Architecture/04 remains L2-working; broad PHASE-I1 is unaccepted because
  OPEN-026/027 and full internal carrier freeze remain. Specialized historical
  M10 SCN-04/09/10/route-patch runners and legacy M8 receipt fixture APIs are
  not SYS-1 kernel evidence.

### SYS-2 direct blockers

1. Define abstract happens-before and operation linearization for the SYS-1
   owner and designated remote-input lifecycle in both deterministic ST and
   one-owner-worker (OW) profiles.
2. Replace the immutable-snapshot gap with an explicit mapping for
   revoke-after-enqueue/serve, publication/observation, patch activation,
   save-cut, witness creation, and relation epoch/sample visibility.
3. Demonstrate selected ST/OW semantic correspondence and owner data-race
   freedom, including counterexamples when a required edge is removed.
4. Keep the bounded remote-input effect ordering explicit without inventing a
   generic provider registry, lock-free requirement, or Surface
   `memory_order_*` vocabulary.

## Risks, assumptions, and decision checkpoints

### Accepted assumptions

- M10 finite semantics and regression evidence are inputs, not a systems
  architecture or I2 identity.
- Internal carrier/API names remain provisional and non-public.
- ST is the accepted deterministic reference; OW is the first threaded
  refinement and may use safe channels/mailboxes/mutexes.
- One finite acyclic relation pressure case is sufficient for SYS-3; arbitrary
  DAG theory remains deferred.
- SYS-5 is a local headless toy world. Browser/View product work is later.

### Main risks and mitigation

| Risk | Trigger | Mitigation / reopen rule |
| --- | --- | --- |
| lifecycle overclaim | program/SYS close called broad I1/I2 acceptance | keep lifecycle source in Canon plan/01; accept only exact criteria |
| M10 facade becomes runtime architecture | release hash/profile/verifier controls kernel semantics | dependency review; kernel has no conformance/release imports |
| internal/public collapse | carrier field names treated as compatibility promise | explicit internal versioning/non-public labels; owner stop before freeze |
| hidden authority/communication | schedule/transport/receipt creates grant/state/edge | typed provenance and no-mint/edge-completeness falsifiers |
| Surface memory leakage | backend atomics become ordinary Mir vocabulary | map every low-level ordering to high-level edge; no Surface import |
| relation overfitting | two anchors remain permanent Core restriction | finite DAG extension boundary plus one pressure case |
| fake vertical slice | wrapper sequences internal helpers or expected JSON | generated artifacts/endpoints must be actual runtime inputs |
| observation leak | devtools exports raw capability/witness/auth state | observer-safe typed projection and negative secret scan/tests |
| evidence laundering | bounded model called proof or helper called product | exact five-class evidence labels and non-claims |
| report/WRK growth | metadata work opens new record | one milestone report; direct-consumer admission rule |

### Owner-reserved stop line

Stop only for: North Star change; weaker authority/privacy/redaction/no-stale-
resurrection; domain-vocabulary Core promotion; unavoidable hidden multi-owner
transaction; irreversible public API/ABI/wire freeze; required real transport
selection/implementation; production/publication; risk to user data/secrets/
paid resources; an irreversible observable tie the Constitution cannot order;
or a reproducible parent-goal/North-Star contradiction.

Official T1, deferred general OBLs, open final grammar/public ABI, incomplete
I3+, unoptimized performance, or unread historical reports are not stop
conditions.

## Deferred scope / non-effects

- real sockets, multi-process/WAN, QUIC/WebTransport selection;
- public grammar/API/ABI/wire and compatibility freeze;
- production deployment/publication and final package marketplace;
- durable distributed save/load, consensus, exactly-once, hidden distributed
  transaction, arbitrary continuation migration;
- final browser renderer, View product, or final FFI;
- arbitrary relation-DAG, scheduler/fairness, memory-model, verifier-
  composition, or durability theorem;
- lock-free runtime and performance optimization;
- I3 implementation or lifecycle entry.

Historical Product Alpha, Full System V1, PrismCascade, Typed-Effect Wiring,
and upper applications remain separable LAB evidence/consumers. They are not
silently folded into the active Mir/Mirrorea kernel.

## Recommended next action

SYS-1 is completed and SYS-2 is active. The parent first commits and pushes
this Canon/report/status closeout successor to source cut `94e3707c...` and
verifies clean remote parity; no SYS-2 source change begins before that
repository-state check. Then pin the smallest ST and OW backend contracts to
the SYS-1 owner/designated-input lifecycle, write the required edge-removal
litmus falsifiers first, and map revocation/publication/activation/cut
visibility to high-level Mir order. Do not start SYS-3 projection semantics
until SYS-2 evidence, review, report, commit/push, and remote parity close.
