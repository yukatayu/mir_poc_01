# Plan 249 — Mirrorea I2 Systems Foundation closed execution roadmap

最終更新: 2026-08-28 20:06 JST

## 役割、authority、current control state

これは PROPOSAL-029 / ADR-0026 で owner が承認した bounded program

```text
Mirrorea I2 Systems Foundation
```

の **closed LAB execution record** である。規範正本は
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
  goal alignment**, **SYS-1 runtime kernel / internal carrier boundary**,
  **SYS-2 concurrency, memory, and effect-handler refinement**, **SYS-3
  per-locus projection and executable artifact generation**, **SYS-4
  in-process generated dispatch runtime**, **SYS-5 minimal typed devtools
  and local virtual-space vertical slice**, and **SYS-6 I2 assurance,
  conformance, and lifecycle closeout**, and **SYS-7 I3 entry contract only**
- accepted SYS-1 source/evidence cut:
  `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`
- accepted SYS-2 source/evidence cut:
  `920d3fe050b8b909253f8511d9ad897272323ced`
- accepted SYS-3 source/evidence cut:
  `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`
- accepted SYS-4 implementation/evidence cut:
  `22196f93b0112b8fd2987ec078021c8865b71651`
- accepted SYS-4 contract: PROPOSAL-033 / ADR-0030 /
  `mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`
- accepted SYS-5 implementation/evidence cut:
  `53a21e64b5a17e24b522f720db10b6e539c058e0`
- accepted SYS-5 contract: PROPOSAL-034 / ADR-0031 /
  `mirrorea_canon/spec/14-sys5-local-toy-devtools.md`
- accepted SYS-6 implementation/evidence cut:
  `5429712de89a7e41c46cfd7fb4a39c4a492864c4`
- accepted SYS-6 Canon/status integration cut:
  `bcb0f767edbb3e9e581c3b4c7f2a49e077f44067`
- accepted SYS-6 contract: PROPOSAL-035 / ADR-0032 /
  `mirrorea_canon/spec/15-sys6-i2-conformance.md`
- accepted SYS-7 contract: PROPOSAL-036 / ADR-0033 /
  `mirrorea_canon/plan/05-i3-entry-contract.md`
- superseded pre-correction SYS-3 candidate retained as partial regression evidence:
  `ded622fef91bab2cadc571ba944e5ee2c69a7b63`
- active goal: **none**
- next goal: **none**; a future I3 program is only an inactive direct-consumer
  contract and requires new owner direction plus a new current roadmap.
- current direct blocker: **none inside this closed program**. OPEN-032 remains
  unresolved for a future owner-authorized I3 transport comparison.
- official lifecycle: theory **T1**; broad PHASE-I1 remains unaccepted because
  OPEN-026/027 and full carrier freeze remain. ADR-0032 accepted official I2
  entry then I2 exit from fresh SYS-6 evidence. ADR-0033 closed SYS-7 and the
  ADR-0026 program. I3 remains inactive.

Plan 247 は closed M0--M10 execution record / R5 regression baseline のまま保持する。
Plan 249 も closed execution record であり、current queue ではない。現在は active
roadmap / goal がない。numbered plan、WRK、historical report は parallel queue ではない。

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

Milestone addition or reordering required evidence that the parent goal could
not close without it and an explanation in this roadmap. No such addition was
made through SYS-7.

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

### SYS-2 — Concurrency, memory, and effect-handler refinement (completed)

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

**Exit evidence:** Source cut `920d3fe050b8b909253f8511d9ad897272323ced`;
deterministic ST plus one coordinator/one dedicated worker-exclusive M8 OW1
for exactly one combined owner/source-owner locus; actual M8 enqueue/read/write
linearization and reads-from/coherence evidence; same-seam M9 revoke/full
retranslation/ack-before-generation-publish; source-owner-derived remote
effect result/consume; 27/27 focused tests; replayable bound-6 ten-edge model;
OBL-058 `model-checked-bounded` and OBL-059 `runtime-monitored`; preserved
SYS-1/M10/full-runtime regression; four independent review lanes ACCEPT with
no remaining P0/P1/P2.

**Stop condition:** Close when all required edges have a finite mapping and
counterexample, selected ST/OW observations agree, and SYS-3 has a concrete
backend contract; defer arbitrary fairness/memory generalization.

### SYS-3 — Per-locus projection and executable artifact generation (completed)

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
without hand-written message interfaces. The bounded clause
`designated consume E.result at C` produces a distinct consumer artifact and
visible evaluator→consumer delivery plan.

**Semantic invariants:** No hidden communication/authority/failure/effect;
owner/site/source span/relation lineage/observation/cut/patch obligations
preserved; same-owner RMW remains in owner artifact; designated expression is
not re-executed at consumers; explicit receipts only for cross-owner results;
designated consumer is source-named exactly once and never inferred from
topology/schedule/relation; same-consumer retry creates no second semantic
consume and a competing consumer conflicts typed. In SYS-3 this is a static
identity/refinement contract only, not existing M8 runtime evidence.

**Direct consumer:** SYS-4 starts the generated artifacts without re-parsing
source semantics; SYS-5 displays their causal correspondence.

**Non-goals:** Optimal placement, final/public grammar or exchange schema,
public API/ABI/wire, multi-consumer semantics, arbitrary relation-DAG theorem,
transport, runtime execution itself, legacy M8 behavior changes, or actual
idempotent-return endpoint correspondence.

**Primary falsifier:** The projector emits a communication edge not implied by
checked Core, omits a required edge/failure/obligation, moves owner mutation to
a requester/consumer, infers a designated consumer without source Core, leaks
the evaluator expression into the consumer, accepts an undeclared/competing
consumer, duplicates a same-consumer semantic consume, or accepts a malformed/
cyclic projection. It is also falsified if projection evidence reinterprets
legacy M8 `AlreadyConsumed` duplicate-delivery rejection as
`ReturnExistingNoNewConsumption` runtime evidence.

**Exit evidence:** Accepted source/evidence cut
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`, following commits `b39f3e76`,
`f37be73c`, `27e42658`, and `30be30bb`, contains the bounded Surface-v0 AST/M6/
M7 `DesignatedResultConsume` path; exactly-one source-named consumer and static
source/Core semantic-consumption identity plus
`ReturnExistingNoNewConsumption` refinement contract; consumer-only artifact;
evaluator→consumer `DesignatedResultDelivery`; joined source-map/observation/
persistence/correspondence rows; topology-non-inference and missing/extra/
moved/leaking/undeclared/competing/signature-ambiguity falsifiers; deterministic
owned owner/relation/designated-input plans; current two-anchor/test-only DAG
pressure; and final semantic/code-quality ACCEPT. AST Surface M6 9/9, M6
classification 13/13, M7 pipeline 25/25, M9 8/8, SYS-3 27/27, M8 admission
7/7, M10 source 2/2, M10 conformance 67/67, full `mir-runtime` and workspace,
format, scoped warnings-denied Clippy, and diff checks passed. OBL-060 is
`runtime-monitored` for this static finite compiler/projector evidence only.
Actual endpoint positive/retry/competing-consumer behavior remains SYS-4
evidence, not a SYS-3 exit claim.

**Stop condition:** Close when SYS-4 can execute artifacts without semantic
reconstruction and every accepted Core operation has complete visible
placement/communication or a typed projection diagnostic. Defer general DAG
theory and optimized codegen.

### SYS-4 — In-process generated dispatch runtime (completed)

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
consistent local multi-locus save/restore/patch state. For an explicit
designated consumer, the first accepted semantic consumption reaches M8 once,
and the same-consumer retry returns the retained decided result without a
second M8 consume.

**Semantic invariants:** No global unpartitioned mutable store; no direct
cross-locus mutation; schedule cannot mint Core/authority/state/expected
result; transport metadata is non-authority; typed failures fail closed;
save/patch include artifact and communication state; rejected patch mutates
only lifecycle rows. `ReturnExistingNoNewConsumption` is implemented against
the source/Core semantic-consumption identity, not raw delivery id or transport
metadata, and does not weaken the accepted M8/M10 duplicate-delivery behavior.

**Direct consumer:** SYS-5 builds the toy world and causal devtools from actual
generated dispatch; SYS-6 verifies the profile.

**Non-goals:** Socket/multi-process transport, WAN, durability, consensus,
exactly-once, production deployment, final public runtime API.

**Primary falsifier:** The same scenario succeeds only through fixture-name
plan selection, source re-interpretation, handwritten communication, direct
remote-store access, or schedule-created semantic facts; or a same-consumer
retry invokes legacy M8 again/returns `AlreadyConsumed` instead of the retained
decision, while a competing consumer is accepted.

**Exit evidence:** Accepted implementation/evidence cut
`22196f93b0112b8fd2987ec078021c8865b71651` materializes the accepted SYS-3
projection as crate-private `FabricProgram` / `LocalFabric` execution. Each
logical locus has an owned local store, generated-plan-bound endpoint and
mailbox state, projected artifact identity, and per-locus M8 observation; ST
uses one independent M8 session per locus, while eligible OW1 uses its one
worker-owned M8 runtime with typed observer-snapshot availability. The staged
path is source action → generated outbox → transport step → target inbox
→ locus dequeue/serve or typed failure; no source/AST, handwritten edge, or
remote store handle enters the runtime.

The selected finite evidence includes four-locus/two-owner ST owner isolation,
eligible ST/OW1 semantic correspondence and deterministic replay, exact
source/Core/fragment/edge/envelope/M8 occurrence lineage, source-owner-derived
designated input, evaluator publication import into the named consumer locus,
and a source/Core-bound cache retry that records exactly one accepted M8
semantic consume. Corrupt/retargeted/stale carrier material, route,
membership/capability/witness, receipt/publication identity, visibility/
redaction/policy, split-frame, revocation, and backend failures fail closed.
OW1 observer snapshot failure is typed and distinct from absence; recovery
reveals the already-committed exact state without semantic replay or payload/
authority leakage.

The ST whole-fabric cut retains locus stores, endpoint/mailbox transfer pairs,
completed receipts, designated publication/cache/consumption state, exact M8
trace/causality, counters, M9 authority floor/tombstones, and patch lifecycle.
Restore rejects missing/asymmetric endpoint history, duplicate inbox mapping,
counter or authority rollback, and forged trace/frontier state. The bounded
patch route accepts only a prechecked, projected, completely M9-admitted
candidate at an exact quiescent program/projection/authority frontier; the
accepted finite patch changes the designated expression while preserving
topology, owner RMW, relation/fallback, non-designated Core, authority, and
semantic state. Stale, nonquiescent, topology/owner-route, owner-expression,
non-designated-Core, or M9-lineage mismatch changes lifecycle rows only.

Focused validation passed SYS-4 99/99, `mir-runtime` library 179/179, and M10
source/CLI/conformance 2/4/67, plus format, `mir-runtime` all-targets
warnings-denied Clippy, and diff checks. Independent review accepted the
bounded internal cut. Evidence is `runtime-monitored`; no Lean/general theorem
changed. OW1 whole-fabric cut/restore and patch activation deliberately return
typed `BackendIneligible` in this profile and are not claimed.

**Stop condition:** Close when SYS-5 can compose a real four-locus scenario
from generated artifacts and no selected success path bypasses locus
endpoints, ownership, or typed admission.

### SYS-5 — Minimal typed devtools and local virtual-space vertical slice (completed)

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

**Stop condition:** Satisfied at implementation/evidence cut
`53a21e64b5a17e24b522f720db10b6e539c058e0`. Reopen only if fresh reproduction
finds inferred anchor placement, caller-minted lifecycle authority,
M8-before-M9 mutation, a missing exact leave→fresh join, partial live mutation
after candidate failure, invalid post-leave restore, invented/leaking causal
rows, fixture-name/expected-result semantics, endpoint bypass/direct remote
mutation, M10 regression, or an unusable SYS-6 direct-consumer boundary.

### SYS-6 — I2 assurance, conformance, and lifecycle closeout (completed)

**Goal ID:** SYS-6

**Goal sentence:** By the end of this milestone, a finite source-first I2
profile independently verifies that global Core projects to and executes as
meaning-preserving per-locus artifacts with generated communication, ST/OW
selected correspondence, and observer-safe causal traceability.

**North Star link:** Supplies explicit verification for the new placement,
communication, observation, and evolution capability and decides lifecycle
state only from actual evidence.

**User-visible outcome:** One conformance command reports the bounded source
fingerprint, safe opaque Core/artifact/generated-edge/runtime-trace references,
pass/fail rows, evidence classes, and non-claims. The JSON does not report or
authorize an accepted Git implementation cut; ADR-0032, Report 2598, and Canon
acceptance metadata pin exact cut
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`.

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

**Exit evidence:** Satisfied at accepted implementation/evidence cut
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`. A downstream-only producer/
verifier emits exactly 22 source-first rows with actual checked-program/Core/
artifact/edge/request/occurrence/model/lifecycle inventories, executed positive
and falsifier evidence, property-specific provenance, deterministic content-
bound I2 identity, typed rejection, and observer-safe serialization. The
profile covers projection determinism, owner preservation, generated-
communication completeness, actual dispatch, selected ST/one-worker OW1
correspondence, worker-exclusive owner state, required visibility evidence,
no hidden/manual communication, no direct remote store, no source-free
authority/state mint, relation/fallback/designated coherence, local cut,
checked patch, and safe devtools.

Validation passed SYS-6 25/25 + CLI 8/8, SYS-2 28/28, SYS-3 28/28, SYS-4
104/104, SYS-5 62/62, M10 conformance 67/67 + CLI 4/4, full workspace tests,
format, warnings-denied Clippy, and diff checks. Final independent assurance/
lifecycle review returned ACCEPT. OBL-063 is aggregate `runtime-monitored`;
the authority row reuses OBL-058 `model-checked-bounded` evidence without
claiming a whole-profile model proof. ADR-0032 accepted official I2 entry then
exit while retaining theory T1 and broad PHASE-I1 residuals.

**Stop condition:** Satisfied. Reopen for a passing missing/extra edge, owner
move, manual route, direct remote store, source-free mint, selected ST/OW
divergence, unbound/failed evidence, wrong diagnostic, stale cut/patch
mutation, relation/fallback/designated drift, observer leak, lower-layer
dependency on conformance, M10 regression, or a fresh counterexample to the
pre-existing I2 criteria. Broad I1 remains unaccepted; no criterion was
weakened.

### SYS-7 — I3 entry contract only (completed)

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

**Exit evidence:** PROPOSAL-036 / ADR-0033 / Canon plan/05 record one inactive
goal, exactly two unselected reliable-stream candidates, internal/public
carrier separation, the full failure matrix, disconnect/reconnect and
duplicate/reorder requirements, network-order refinement, and future
SCN-01/02/03/06 C-distributed gates. The contract makes no transport/version/
codec/wire/library/port choice and starts no I3 implementation or lifecycle.
Status is synchronized and independent pre-edit planning review returned
ACCEPT; final closeout review is recorded in Report 2599.

**Stop condition:** Satisfied. SYS-7 and the ADR-0026 program are closed. Do
not implement I3 or resolve OPEN-032 without new owner direction.

## SYS-0 meta-alignment matrix

This is the retained SYS-0 start-state decision aid for program scope, not a
current-status table or a new semantic encyclopedia. SYS-3--SYS-6 closed the
finite generated-artifact, in-process dispatch, local causal-toy, and I2
conformance gaps recorded here. SYS-7 then recorded the inactive I3 entry
contract without starting transport implementation. The program is closed.

| Owner intent | SYS-0 Canon representation | SYS-0 implementation evidence | SYS-0 gap | This program consumer |
| --- | --- | --- | --- | --- |
| meaning-derived communication | North Star, C3, BND-001/006 | finite source-to-Core/generated-edge M10 evidence | no general per-locus executable communication path | SYS-1 carrier; SYS-3/4 projection/dispatch |
| per-locus code generation | S4 projection, `mir-project` responsibility | no accepted generator | global checked Core does not emit runnable locus artifacts | SYS-3, then SYS-4 |
| ordinary Surface | C2, M6/M7 bounded source path | accepted ordinary `.mir` finite profile | not final grammar; no I2 build/project path | SYS-3/5 |
| owner evaluation | C4/C5, theory/13, M7/M8 | owner-side RMW plus SYS-2 ST/OW1 actual M8 linearization/reads-from evidence | generated artifact placement has not yet bound the backend requirement | SYS-3/4 |
| relation-first late projection | C7, theory/14, BND-006 | finite two-anchor relation/consumer projection | artifact placement and conservative DAG extension boundary absent | SYS-3/5 |
| existence/fallback DAG | theory/06/14 monotone lineage | two-anchor/three-floor finite evidence | broader finite acyclic pressure case not projected/executed | SYS-3/5 |
| designated evaluation/timing | C6, theory/13, M7/M8 | version/frontier decision and finite consume path | generated evaluator placement/delivery across locus artifacts absent | SYS-3/4/5 |
| auth layers | C9, theory/05/18, M9 | source-bound seam plus SYS-2 same-seam revoke/full retranslation/ack-before-publish | per-locus artifact authority obligations and component dispatch absent | SYS-3/4/5 |
| optional verification | C9/C11, M9 verifier lane | residual/evidence/diagnostic finite seam; SYS-5 joined one optional verification path | finite I2 profile has not yet classified the joined evidence | SYS-6 |
| algebraic-effect-like visible operations | effect/failure rows, provider/adaptor BNDs | bounded designated remote-input request→source-owner read→derived result→consume in ST/OW1 | generated per-locus EffectHandlerPlan absent; generic registry intentionally unclaimed | SYS-3/4 |
| save/load and Z-cycle | C10, theory/04, M8 local cut | finite local save/restore; general Z-cycle obligations deferred | whole in-process multi-locus artifact/communication cut absent | SYS-4/5/6 |
| atomicity / memory order | ADR-0007, theory/04 high-level edges, owner seriality | deterministic ST, OW1 worker-exclusive store, ten-edge bounded model and missing-edge counterexamples | multi-locus generated artifact/runtime mapping absent; no general memory theorem | SYS-3/4/6 |
| browser/headless participation | North Star browser participation horizon; host boundary | accepted SYS-5 headless four-locus local causal workflow | browser renderer remains absent | SYS-6 assures headless profile; browser deferred |
| View / FFI | BND-007 provider/View split | historical LAB/provider evidence plus internal SYS-5 joined report | final View/renderer and public FFI not selected | deferred; SYS-6 assures only the internal report |
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

## Accepted evidence and closed-program boundary

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

### SYS-2 accepted close evidence

- Accepted source/evidence cut:
  `920d3fe050b8b909253f8511d9ad897272323ced`.
- ST remains the deterministic reference. OW1 admits exactly one combined
  semantic owner/source-owner locus, whose dedicated worker exclusively owns
  M8 state through acknowledged zero-capacity mailbox commands; another locus
  count fails typed without state duplication.
- Successful owner RMW evidence names the actual M8 enqueue, `OwnerRead`, and
  `OwnerWrite` nodes, with the write as linearization point and per-key
  version/preceding writer as reads-from/coherence. Failed/revoked serve does
  not fabricate those rows.
- Designated remote result derives from the acknowledged source-owner read;
  a supplied mismatch fails before reply/receipt/mutation and the successful
  lifecycle reaches explicit evaluator consume.
- Same-seam M9 revoke fully retranslates the inventory, retains unrelated
  owner/designated-release lineages and monotone tombstones, waits for ST/OW1
  owner install acknowledgement, then publishes the successor. A queued stale
  use rejects with `MissingCapability` and no mutation.
- The bound-6 typed transition model covers ten required edge families and
  returns replayable missing-edge counterexamples. Full-edge ST/OW1 selected
  outcomes agree; store buffering is a separate explicit weak-memory
  calibration.
- Combined SYS-2 focused tests passed 27/27 (13 external model + 9 backend + 5
  internal model-regression). SYS-1 13/13, M10 source 2/2, CLI 4/4,
  conformance 67/67, full `mir-runtime`, format, warnings-denied Clippy, and
  diff check passed. Evidence is OBL-058 `model-checked-bounded` plus OBL-059
  `runtime-monitored`; no Lean/general theorem was added.
- Semantic/specification, concurrency/code-quality, finite-model, and
  test-contract reviewers accepted the corrected cut with no remaining
  P0/P1/P2 finding.

### SYS-3 accepted evidence and reopen history

- Former candidate source/evidence cut, retained only as partial regression:
  `ded622fef91bab2cadc571ba944e5ee2c69a7b63`, following source-test commits
  `c10a1bce`, `dae31bbe`, `db4358d1`, `cd98d81f`, and `e8c9570f`.
- The crate-private pure projector consumes only `CheckedSurfaceV0` plus an
  exact identity-bound logical-locus inventory. It deterministically owns
  placement-specific checked fragments for four loci and derives
  communication, handler, observation, persistence, relation, backend, and
  source-map plans without AST/source/runtime/conformance imports or manual
  interface fixtures.
- Same-owner RMW stays at the owner; the origin gets only its invocation.
  Relations remain owner publish plus consumer-local projection. Designated
  remote state reads stay distinct from the evaluator artifact and use
  source-owner service plus typed receipt/consume planning.
- Close review found the missing semantic edge after those checks: no ordinary
  source/AST/M6/M7 Core fact named a consumer of the evaluator result, so the
  projector could not generate E→C delivery without forbidden topology
  inference. The required correction is exactly the provisional
  `designated consume E.result at C` clause and its distinct checked Core path.
- The corrected source/evidence cut is
  `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`, following RED/repair commits
  `b39f3e76`, `f37be73c`, `27e42658`, and `30be30bb`. The review sequence found
  and fixed M6 metadata loss (P1), missing producer resolution (P2), and silent
  evaluation-signature shadowing (P1); final semantic and code-quality reviews
  returned ACCEPT.
- RED review P1 #2 separated static contract from runtime evidence. Theory/13
  requires same-consumer return without a second semantic consumption, but
  legacy M8 rejects the same delivery id as `AlreadyConsumed` and may consume a
  different id; M10 preserves the same-delivery rejection. SYS-3 records only
  the semantic identity/refinement contract. The actual carrier-side
  idempotent return/wrapper and endpoint tests are a SYS-4 obligation.
- Every edge directly names real source/target fragment refs and a checked-Core
  identity. Carrier contracts expose required lifecycle/frontier/authority
  slots but transfer or mint no authority. Observation rows are future
  occurrence requirements with reference-only redaction, and complete-row
  semantic equality makes finalization idempotent without collapsing distinct
  provenance.
- Production relation projection remains the current checked two-anchor
  primary→fallback shape. A test-only same-program typed extension pressure
  exercises a deeper/shared acyclic graph and rejects cycle/foreign identity;
  it is not production nested-relation semantics or an arbitrary-DAG theorem.
- Final focused results are AST Surface M6 9/9, M6 classification 13/13, M7
  pipeline 25/25, M9 8/8, SYS-3 27/27, M8 admission 7/7, M10 source 2/2, and
  M10 conformance 67/67. Full `mir-runtime`, full workspace, formatting,
  scoped `mir-ast`/`mir-semantics`/`mir-runtime` warnings-denied Clippy, and
  diff checks passed. Full workspace Clippy is not claimed. OBL-060 is
  `runtime-monitored` for this static finite compiler/projector evidence only;
  no Lean/general theorem changed.

### SYS-4 accepted finite boundary

1. The runtime starts checked SYS-3 artifacts without source reparse and sends
   only generated communication through explicit locus endpoints. ST owns one
   M8 session per logical locus; eligible OW1 preserves the same selected
   semantic correspondence through its bounded worker profile.
2. Actual request, transport, receive, serve/reply/failure, designated
   publication/consume, owner mutation, and M8 rows retain exact source/Core/
   fragment/edge/carrier provenance. Neither route nor observer metadata mints
   authority or state.
3. The designated-consumer wrapper binds retry to the exact source/Core
   semantic-consumption identity and sealed publication/frontier/policy data.
   The first acceptance reaches M8 once; an exact retry performs a
   non-consuming validation and returns the retained value. Legacy direct M8/
   M10 duplicate-delivery behavior remains unchanged outside this wrapper.
4. ST whole-fabric cut/restore and one bounded designated-expression patch are
   accepted. The cut is not durable or distributed. OW1 cut/patch is explicitly
   typed `BackendIneligible`; arbitrary patch shape, patch DAG, migration, and
   public compatibility are unclaimed.
5. PROPOSAL-033 / ADR-0030 / Canon spec/13 accept source/evidence cut
   `22196f93...`; SYS-4 evidence is
   `runtime-monitored` only. It establishes neither public API/ABI/wire nor a
   general dispatch, scheduler, memory, cut, patch, or noninterference theorem.

### SYS-5 accepted finite boundary

1. The ordinary source at
   `samples/clean-near-end/mirrorea-i2-local-toy/main.mir` checks and projects
   `WorldAuthority`, `ParticipantA`, `ParticipantB`, and `ViewerC`. The
   provisional internal `project-loci`, `run-local`, and `inspect` commands
   consume checked projection/runtime inputs rather than fixture-name routing,
   expected-result lookup, or handwritten communication.
2. The actual generated-endpoint workflow exercises owner-side attack RMW,
   designated publication/named consume, the B-owned bird relation with
   explicit A-primary/B-fallback anchor loci, source-derived A leave, duplicate
   leave rejection, semantic fallback, a presentation-only ViewerC sample gap,
   and fresh membership/capability/witness reacquisition without stale
   resurrection.
   Leave/fallback/fresh transitions are clone-prepared ST failure-atomic
   candidates, and the post-leave local cut preserves the exact retired lineage
   needed by fresh reacquire. This is not a hidden transaction or durable cut.
3. One deterministic observer-safe report joins source span, checked Core,
   locus fragment, generated communication edge, request/enqueue/dispatch/
   receive/serve occurrences, owner/relation/designated state, execution branch,
   save/restore cut, patch lifecycle, and typed failure. Raw credential,
   capability secret, witness payload, and private values remain redacted.
4. The finite workflow includes ST save/restore, one accepted designated patch,
   one rejected owner-RMW patch with no semantic mutation, a capability
   revocation/failure path, and an optional verification residual/discharge
   example. It does not claim OW1 whole-workflow cut/patch or general lifecycle
   commutation.
5. Accepted implementation/evidence cut is
   `53a21e64b5a17e24b522f720db10b6e539c058e0`. Focused groups passed AST 10,
   semantics 27, SYS-3 projection 28, workflow 8, relation 17, cut/patch 12,
   CLI 3, and membership leave/reacquire 4. Final `mir-runtime --all-targets`
   passed its 245 library tests and all integration targets; M10 regressions
   passed 2/4/67. Formatting, warnings-denied Clippy, diff check, and manual CLI
   redaction inspection passed. Independent M9 authority/concurrency,
   semantics, and usability/security review lanes returned ACCEPT with no
   P0/P1 finding.
6. PROPOSAL-034 / ADR-0031 / Canon spec/14 and Report 2597 record the bounded
   contract and close evidence. OBL-062 is `runtime-monitored`; no Lean,
   model-check, general theorem, public compatibility, browser/View product, or
   I2 conformance/lifecycle acceptance follows from SYS-5 alone.

Reopen SYS-5 only if fresh evidence reproduces inferred anchor placement,
caller-minted lifecycle authority, M8 mutation before M9 retirement, a missing
exact leave→fresh lineage join, partial live mutation after candidate failure,
invalid post-leave restore, manual/fixture route selection, endpoint bypass or
direct remote mutation, source/Core/artifact/occurrence mismatch, observer
leakage, presentation-gap semantic mutation, rejected-patch mutation, M10
regression, or a SYS-6 direct consumer unable to use the rows conservatively.

### SYS-6 accepted finite boundary

1. `conform-i2` executes the accepted producer path and verifies exactly 22
   rows; it neither imports the M10 facade nor controls lower runtime meaning.
2. Every accepted row joins executed positive/falsifier evidence and an actual
   property-specific provenance anchor. Missing evidence, wrong diagnostics,
   manual/extra/missing edges, owner movement, direct remote store,
   source-free mint, selected ST/OW divergence, cut corruption, and observer/
   lifecycle overclaim controls fail closed.
3. The report is deterministic, content-bound, I2-namespaced, host-path
   independent, and observer-safe. Its lifecycle fields remain non-authorizing.
4. Accepted implementation/evidence cut is
   `5429712de89a7e41c46cfd7fb4a39c4a492864c4`. PROPOSAL-035 / ADR-0032 /
   spec/15 and Report 2598 record the contract. OBL-063 is aggregate
   `runtime-monitored`; no general theorem or public compatibility follows.
5. ADR-0032 accepted official I2 entry then exit from the actual evidence.
   Theory T1 and broad PHASE-I1 remain unchanged; I3 is inactive.

### SYS-7 accepted inactive entry contract

1. The future goal maps accepted I2 artifacts and communication plans over at
   least two operating-system processes, but it is inactive.
2. Candidate A (TLS-over-TCP framed reliable-stream adapter) and Candidate B
   (QUIC reliable-stream adapter) are both **UNSELECTED**. QUIC datagrams are
   not admitted or evaluated.
3. Transport/session/certificate identity is non-authoritative. The internal
   carrier and any future public wire remain separate. The complete typed
   failure matrix and network-order refinement are prerequisites.
4. Future C-distributed gates use SCN-01/02/03/06. No production, public
   freeze, I3 lifecycle activation, or transport implementation occurred.
   OPEN-032 remains unresolved for a future owner-authorized decision.

## Risks, assumptions, and decision checkpoints

### Accepted assumptions

- M10 finite semantics and regression evidence are inputs, not a systems
  architecture or I2 identity.
- Internal carrier/API names remain provisional and non-public.
- ST is the accepted deterministic reference; OW is the first threaded
  refinement and may use safe channels/mailboxes/mutexes.
- One finite acyclic relation pressure case is sufficient for SYS-3; arbitrary
  DAG theory remains deferred.
- SYS-5 is an accepted local headless toy world and SYS-6 accepted only its
  finite assurance surface. Browser/View product work remains later; neither
  result is a public devtools contract.

### Main risks and mitigation

| Risk | Trigger | Mitigation / reopen rule |
| --- | --- | --- |
| lifecycle overclaim | accepted bounded I2 exit called broad I1, I3, or product completion | keep exact axes in Canon plan/01; theory T1 and broad I1 residual remain explicit |
| M10 facade becomes runtime architecture | release hash/profile/verifier controls kernel semantics | dependency review; kernel has no conformance/release imports |
| internal/public collapse | carrier field names treated as compatibility promise | explicit internal versioning/non-public labels; owner stop before freeze |
| hidden authority/communication | schedule/transport/receipt creates grant/state/edge | typed provenance and no-mint/edge-completeness falsifiers |
| topology invents designated consumer | evaluator result has no source/Core consume edge but projection targets a locus anyway | require explicit bounded `designated consume E.result at C`; reject missing/competing/undeclared source facts |
| retry evidence laundering | legacy M8 same-delivery `AlreadyConsumed` rejection is reported as theory/13 idempotent-return evidence | accepted SYS-4 wrapper uses the source/Core semantic-consumption identity, one actual M8 consume, and exact retry validation; keep legacy M8/M10 behavior and the wrapper scope distinct |
| Surface memory leakage | backend atomics become ordinary Mir vocabulary | map every low-level ordering to high-level edge; no Surface import |
| relation overfitting | two anchors remain permanent Core restriction | finite DAG extension boundary plus one pressure case |
| fake vertical slice | wrapper sequences internal helpers or expected JSON | generated artifacts/endpoints must be actual runtime inputs |
| observation leak | devtools exports raw capability/witness/auth state | observer-safe typed projection and negative secret scan/tests |
| conformance self-certification | profile passes by trusting expected JSON, release identity, or implementation-selected omissions | derive row inventory from accepted Core/plan operations; independent negative controls and reviewer |
| evidence laundering | bounded model called proof or helper called product | exact five-class evidence labels and non-claims |
| report/WRK growth | metadata work opens new record | one milestone report; direct-consumer admission rule |
| premature transport commitment | one untested stack, wire, codec, version, or library is treated as selected | keep both reliable-stream candidates UNSELECTED; require future owner direction and comparative C-distributed evidence |

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

- real sockets, multi-process/WAN, and selection between TLS-over-TCP framed
  reliable stream and QUIC reliable stream;
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
silently folded into the accepted Mir/Mirrorea kernel.

## Recommended next action

Stop this closed program. There is no active roadmap or goal. Preserve the
accepted I2 cuts and the inactive ADR-0033 / Canon plan/05 entry boundary.
Starting I3, selecting either transport candidate, or resolving OPEN-032
requires new owner direction, a new bounded program, and a new current
roadmap; no transport code or public wire work begins from Plan 249.
