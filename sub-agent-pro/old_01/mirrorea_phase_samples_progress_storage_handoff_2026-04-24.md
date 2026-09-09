
# Mirrorea phase/sample/progress/storage handoff for autonomous CodeX work

**Date:** 2026-04-24  
**Intended location in repo:** `sub-agent-pro/mirrorea_phase_sample_progress_storage_handoff_2026-04-24.md`  
**Audience:** CodeX / sub-agents / repo maintainers  
**Purpose:** Define the phase-by-phase runnable sample plan, E2E validation expectations, `samples_progress.md` format, reporting discipline, git discipline, storage-management policy, and future Mirrorea implementation roadmap.

This file is a **handoff**, not the normative spec. It must be read, checked against the current repo, and then reflected into the appropriate repo files:

```text
specs/       = normative source of truth
plan/        = repository memory
docs/        = reader-facing explanation and reports
.docs/       = reader-facing docs if the repo already uses it
tasks.md     = current work queue snapshot
progress.md  = current project progress snapshot
AGENTS.md    = agent operating rules
```

The project axis must remain:

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

Do not replace that axis with “just a language,” “just an OS,” “just a plugin system,” “just a game engine,” or “just a visualization tool.”

---

## 0. What this handoff adds

This handoff adds the following missing discipline:

1. **Every phase/layer must have runnable samples.**
2. **Each phase must have unit-level checks and E2E-style checks.**
3. **E2E must arise naturally from layer composition, not from fake thick wrappers.**
4. **`samples_progress.md` must show, at a glance, what works and how far it is.**
5. **Progress is based on runnable samples, not prose only.**
6. **Git commits and pushes must be frequent and evidence-backed.**
7. **Storage must be managed explicitly because the current VPS is small.**
8. **Large build artifacts, LLVM, caches, and generated artifacts must be placed on detachable storage where appropriate.**
9. **Detachable storage must be disposable or cleanly separable from the source repo.**
10. **Debug / visualization output must become a first-class goal, not a late afterthought.**

---

## 1. Completion definitions

Use these definitions consistently.

### 1.1 Current-layer completion

A current layer is complete when:

```text
- its normative reading is reflected in specs/ or relevant specs/examples/*
- its repository memory is reflected in plan/
- it has at least one positive runnable sample
- it has at least one negative sample if the layer rejects bad patterns
- it has unit tests for its local behavior
- it has an integration or E2E-style sample
- it has debug/visualization output appropriate to that layer
- it is listed in samples_progress.md
- validation commands were actually run
- results were reported in docs/reports/
- progress.md and tasks.md were updated
```

### 1.2 Repo-local alpha completion

Repo-local alpha completion means:

```text
- clean active samples are runnable
- old samples are archived or clearly marked historical
- current layers have samples_progress.md entries
- runtime attach / membership / handoff / verification examples are runnable in logical emulator form
- static verification, runtime execution, model-check, theorem/Lean skeleton, and debug output are all represented
- docs/hands-on documents exist
- final public grammar/API/verifier/packaging remain explicitly deferred
```

Repo-local alpha completion does **not** mean:

```text
- final parser grammar
- final public parser/checker/runtime API
- final public verifier contract
- production theorem-prover binding
- production model-check binding
- installed binary / distribution packaging
- real network deployment
- full multi-server consensus
- durable distributed commit
- full VR interface
- full application ecosystem
```

### 1.3 Final Mirrorea virtual-space alpha

This is a later target. It includes:

```text
- logical multi-place runtime
- typed external effect adapters
- message envelope and auth insertion seam
- projection / placement layer
- hot-plug package and attach-point typing
- visualization/debugger
- at least one game sample and one avatar/object sample
- at least one separate-process or network-emulated deployment
```

---

## 2. Place: definition to preserve

Use this definition.

```text
Place =
  physical process / physical machine / OS process / network node
  + virtual thread
  + authority/state/visibility compartment
  + execution locus with local state, queue, capability, and observation frontier
```

Important distinctions:

```text
Principal / Participant:
  Alice, Bob, Carol, Dave. "Who."

Place:
  WorldServerPlace, ParticipantPlace[Alice], SugorokuGamePlace#1.
  "Where execution, state, queue, authority, and observation live."

Capability / Authority:
  What a principal/place may do.

Membership:
  Whether a principal participates in a world/room/game, with epoch and incarnation.

Component / Hot-plug object:
  A runtime-attached game/object/tool/domain kernel.
```

Do not collapse these concepts.

---

## 3. Standard I/O policy

Mir core should not have Unix-like standard I/O as a primitive.

Do not treat these as core primitives:

```text
stdin
stdout
stderr
print as magic
read as magic
global process console
```

External interaction should be via typed external effect boundaries:

```mir
external effect ShowDebugText {
  input message : Text @ DebugLabel
  output receipt : VisualReceipt

  requires authority >= DebugViewer
  failures DisplayUnavailable
  evidence emits DebugViewWitness
}

adapter BrowserConsoleDebug implements ShowDebugText {
  host_target browser.console
}
```

Meaning:

```text
- I/O is an effect.
- Effects have typed inputs/outputs.
- Effects require capabilities/authority.
- Effects may fail explicitly.
- Effects may emit witness/audit/debug records.
- Adapters connect effects to actual host/OS/network/VR/browser/etc.
```

This is FFI-like, but it should be called:

```text
typed external effect adapter
```

rather than unsafe FFI.

---

## 4. Layer composition model

All later layers should be described with a common shape.

```text
Layer {
  name

  requires:
    capabilities
    input signatures
    modes
    index theories
    runtime services

  provides:
    signatures
    effects
    evidence
    adapters
    views

  transforms:
    source IR / term signatures / message envelopes / runtime events

  checks:
    constraints / obligations / properties

  emits:
    witnesses / certificates / debug traces / telemetry

  laws:
    soundness / preservation / monotonicity / no hidden authority / no hidden data leak
}
```

This applies to:

```text
authentication layer
authorization layer
transport layer
encryption layer
compression layer
verification layer
model-check layer
theorem layer
visualization layer
telemetry layer
hot-plug layer
projection layer
```

### 4.1 Layer composition laws

Every composable layer must preserve these laws, or explicitly emit residual obligations.

```text
No hidden authority:
  A layer must not silently strengthen authority.

No hidden data downgrade:
  A layer must not silently lower labels, e.g. KeyMaterial -> Public.

No hidden effect:
  A layer must not make network/file/publish/debug effects look pure.

Evidence preservation:
  Witnesses/certificates/proof refs remain attached to the correct term/event.

Placement preservation:
  Moving computation between Places must preserve required effects, capabilities,
  dataflow, evidence, and runtime guards.

Visualization respects labels:
  Debug/visualization must not leak data that normal computation cannot reveal.

Residual obligations explicit:
  If a layer cannot decide/prove/check something, it must produce a residual obligation,
  not silently accept it.
```

---

## 5. TermSignature registry

Every term or core IR node that matters to execution, verification, projection, visualization, or hot-plug should have a signature.

Recommended structure:

```text
TermSignature {
  term_ref
  source_ref
  type
  mode
  effect_row
  capability_requirements
  index_constraints
  runtime_guards
  residual_obligations
  evidence_required
  evidence_produced
}
```

Example:

```json
{
  "term": "handoff dice_owner Alice -> Bob",
  "type": "HandoffReceipt",
  "mode": "witnessed(draw_pub)",
  "effect_row": ["ownership_transfer", "audit_append"],
  "capability_requirements": [
    "DiceOwner(Alice)",
    "ActiveParticipant(Bob)"
  ],
  "index_constraints": [
    "Bob is live participant"
  ],
  "runtime_guards": [
    "membership_epoch is current"
  ],
  "evidence_required": [
    "PublicationWitness(draw)"
  ],
  "evidence_produced": [
    "DiceOwner(Bob)",
    "HandoffWitness(dice_owner, Alice, Bob)"
  ]
}
```

### 5.1 Why this matters

TermSignature is the shared currency for:

```text
static checker
finite-index type layer
theorem prover
model checker
runtime guard
network projection
auth layer
adapter selection
debug output
visualizer
hot-plug compatibility
```

Without TermSignature, each layer will reinterpret the same code separately and eventually drift.

### 5.2 Required sample

Add a debug mode:

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json
```

Expected shape:

```json
{
  "sample": "03_roll_publish_handoff",
  "signatures": [
    {
      "term": "perform roll_dice",
      "type": "DiceRoll",
      "mode": "place(SugorokuGame#1)",
      "effects": ["random_draw"],
      "requires": ["DiceOwner(Alice)"],
      "produces": ["RollEvent(draw)"]
    },
    {
      "term": "publish draw",
      "type": "PublicationReceipt",
      "mode": "published(room)",
      "effects": ["publish", "audit_append"],
      "requires": ["label(draw) <= visibility(game.published_history)"],
      "produces": ["PublicationWitness(draw)"]
    },
    {
      "term": "handoff dice_owner Alice -> Bob",
      "type": "HandoffReceipt",
      "mode": "witnessed(draw_pub)",
      "effects": ["ownership_transfer", "audit_append"],
      "requires": ["PublicationWitness(draw)", "ActiveParticipant(Bob)"],
      "produces": ["DiceOwner(Bob)"]
    }
  ]
}
```

---

## 6. MessageEnvelope and authentication seam

Transport, authentication, authorization, membership, capability, and audit must be separated.

Recommended envelope:

```text
MessageEnvelope {
  from_place        : PlaceId
  to_place          : PlaceId
  principal_claim   : PrincipalClaim
  auth_evidence     : Optional<AuthEvidence>
  membership_epoch  : MembershipEpoch
  incarnation       : MemberIncarnation
  effect_request    : EffectRequest
  payload           : TypedPayload
  required_caps     : CapabilitySet
  witness_refs      : List[WitnessRef]
}
```

### 6.1 Authentication layer

Example layer:

```text
Layer SessionTokenAuth {
  requires:
    RawToken
    TokenVerifier

  provides:
    AuthEvidence[Principal]

  transforms:
    MessageEnvelope[auth = none]
      -> MessageEnvelope[auth = SessionTokenEvidence]

  effects:
    token_verify

  obligations:
    token_not_expired
    token_binds_to_principal
}
```

### 6.2 Signature auth layer

```text
Layer SignatureAuth {
  requires:
    PublicKeyRegistry
    SignedPayload

  provides:
    AuthEvidence[Principal]

  transforms:
    MessageEnvelope[auth = none]
      -> MessageEnvelope[auth = SignatureEvidence]

  effects:
    signature_verify

  obligations:
    signature_valid
    key_binds_to_principal
}
```

### 6.3 Important split

```text
Authentication:
  Are you Alice?

Authorization:
  May Alice do reset_game?

Membership:
  Is Alice currently in this world/game?

Capability:
  Does Alice hold the required capability?

Witness:
  Is there evidence that required prior event happened?
```

Do not collapse these into transport.

---

## 7. Visualization and telemetry

Visualization is not optional UI polish. It is part of making Mirrorea understandable.

### 7.1 Static views

```text
Place graph
Effect route graph
Type/index constraint view
TermSignature view
Proof obligation graph
Hot-plug compatibility view
Projection view
```

### 7.2 Runtime views

```text
Event DAG
Message flow
State timeline
Membership timeline
Witness timeline
Data flow
Failure/rejection view
Performance/telemetry view
Hot-plug lifecycle view
```

### 7.3 Visualization is an effect

Debug output may leak data, so visualize through typed effects:

```mir
effect ShowDebugTrace {
  input trace : DebugTrace @ DebugLabel
  output receipt : VisualReceipt

  requires authority >= DebugViewer
  requires label(trace) <= viewer_clearance
}
```

### 7.4 Telemetry is also information

Performance data can leak information. Treat it as typed telemetry.

```mir
effect RecordTelemetry {
  input event : RuntimeEvent
  input timing : TimingInfo
  output receipt : TelemetryReceipt

  requires telemetry_enabled
  label Internal
}
```

---

## 8. Sample progress tracking

Create and maintain:

```text
samples_progress.md
```

This document must be concise, structured, and update-in-place. Do not let it degrade into unstructured appended logs.

### 8.1 Progress percentages

Use these meanings:

```text
0%:
  not scheduled

1%:
  started; sample ID and goal exist

10%:
  spec/sample skeleton exists

25%:
  parser/loader/static carrier exists

50%:
  minimal implementation passes primary positive sample

65%:
  negative/rejection samples pass

75%:
  debug/visualization output exists

90%:
  E2E/regression/closeout validation passes

100%:
  complete for current scope:
    implementation + positive/negative samples + debug/visualization
    + docs + report + tests + progress update + git commit/push
```

The user specifically asked that:

```text
着手直後 = 1%
最小の実装 = 50%
完全な実装 = 100%
```

Do not report 100% without evidence.

### 8.2 Required file format

Use this structure.

```markdown
# samples_progress

Last updated: YYYY-MM-DD HH:MM JST
Current repo-local focus: ...
Current active packages: ...

## Summary

| Area | Current % | Positive samples | Negative samples | E2E | Visualization | Report |
|---|---:|---:|---:|---|---|---|
| Mir current substrate | 100 | ... | ... | pass | pass | ... |
| Sugoroku runtime attach | 75 | ... | ... | partial | partial | ... |
| Avatar fairy follow | 10 | ... | ... | not yet | not yet | ... |

## Active sample matrix

| ID | Layer | Sample path | Purpose | Positive/Negative | Unit | Integration | E2E | Visualization | Docs | % | Last validation | Blocker | Next step | Report |
|---|---|---|---|---|---|---|---|---|---|---:|---|---|---|---|
| SUG-00 | Mirrorea runtime | samples/... | world bootstrap | positive | pass | pass | pass | summary | yes | 100 | ... | none | ... | ... |

## Current blockers

| ID | Blocker | Affected samples | Owner | Next action |
|---|---|---|---|---|

## Recent validation

| Timestamp | Command | Result | Notes |
|---|---|---|---|

## Historical / archived samples

| Old path | New status | Replacement | Notes |
|---|---|---|---|
```

### 8.3 Rules

- Update rows in place.
- Do not append free-form progress blocks.
- Every percentage must be tied to a runnable sample or explicit blocker.
- Every 100% row needs a report reference.
- Every sample must have at least one command that a human can run.
- If a sample is conceptual only, it must not exceed 25%.

---

## 9. Phase/layer sample plan

Each phase/layer needs runnable samples.

The samples below are the current recommended plan. Add more later, but do not skip the listed categories without report justification.

### Phase 0 — Repository memory and decision boundary

Purpose:

```text
Reader and agent can recover the project state from specs/plan/reports.
```

Samples/checks:

```text
P0-01: docs source hierarchy check
P0-02: stale OPEN/FUTURE factification scan
P0-03: report/mirror update check
```

Commands:

```bash
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
```

If such scripts do not exist, create minimal checks or document missing status.

E2E:

```text
Read specs -> plan -> current task -> report generation path works.
```

Progress target:

```text
100% when docs hierarchy validation and report creation path are documented and pass.
```

---

### Phase 1 — Mir current L2 semantics

Purpose:

```text
guarded option chain, fallback, lease, try/fallback, atomic_cut, local finalization.
```

Samples:

```text
MIR-01: valid guarded fallback same-lineage
MIR-02: underdeclared lineage rejected
MIR-03: no re-promotion
MIR-04: rollback cannot cross atomic_cut
MIR-05: lease-expired option miss vs request-level Reject distinction
```

Unit tests:

```text
parser/AST shape
chain normalization
static reject
runtime reject
cut behavior
```

E2E:

```text
run MIR-01..MIR-05, emit trace, compare expected artifacts
```

Visualization:

```text
option chain graph
selected option / skipped option / fallback reason
cut frontier view
```

---

### Phase 2 — Parser-free PoC / detached validation loop

Purpose:

```text
fixture -> run bundle -> detached artifact -> compare
```

Samples:

```text
P2-01: minimal fixture success
P2-02: malformed fixture static stop
P2-03: detached artifact diff
P2-04: host-plan evidence extraction
```

E2E:

```text
input fixture -> interpreter -> artifact -> compare helper -> report
```

Visualization:

```text
artifact summary
state before/after
effect trace
```

---

### Phase 3 — Parser boundary and first checker cut

Purpose:

```text
narrow parser carriers and first local/structural checker.
```

Samples:

```text
P3-01: option declaration parse
P3-02: chain / fallback / lineage parse
P3-03: request require/ensure suite parse
P3-04: perform-head + request-clause bundle
P3-05: malformed duplicate require rejected
```

Unit:

```text
AST carrier tests
negative parser tests
first checker tests
```

E2E:

```text
source sample -> AST -> checker -> minimal runtime preview
```

Visualization:

```text
AST tree
attachment slots
request clause two-slot view
```

---

### Phase 4 — Shared-space membership / room boundary

Purpose:

```text
membership registry, participant/incarnation, authoritative room baseline.
```

Samples:

```text
P4-01: authoritative room baseline
P4-02: member join increments membership registry
P4-03: leave invalidates pending action
P4-04: stale reconnect fails then refreshes
P4-05: late join sees published history
```

E2E:

```text
room membership changes -> command attempts -> published history / rejection trace
```

Visualization:

```text
membership timeline
incarnation table
published history
```

---

### Phase 5 — Small decidable core and proof boundary

Purpose:

```text
finite-index typing, theorem-first bridge, model-check second line.
```

Samples:

```text
P5-01: authorized declassification passes
P5-02: unauthorized declassification rejected
P5-03: label flow rejected
P5-04: capture escape rejected
P5-05: cost bound rejected
P5-06: theorem preflight emits proof obligation
P5-07: model-check row-local property
```

E2E:

```text
source -> finite-index constraints -> checker result -> theorem/model-check artifacts
```

Visualization:

```text
constraint table
TermSignature view
proof obligation graph
```

---

### Phase 6 — Compile-ready minimal actualization

Purpose:

```text
mir-ast / mir-semantics / mir-runtime narrow implementation path.
```

Samples:

```text
P6-01: clean typing family
P6-02: clean order-handoff family
P6-03: clean model-check mutex family
P6-04: modal/stage minimal family
P6-05: Lean foundation sample set
```

E2E:

```text
clean sample suite smoke-all
```

Visualization:

```text
debug summary
json output
signature dump
model-check counterexample trace
```

---

### Phase 7 — Sugoroku runtime attach

Purpose:

```text
runtime attach of game into empty world/server with participants.
```

Samples:

```text
SUG-00: world bootstrap
SUG-01: runtime attach game
SUG-02: admin start/reset
SUG-03: roll -> publish -> witness -> handoff
SUG-04: non-owner roll rejected
SUG-05: late join history visible
SUG-06: leave non-owner
SUG-07: owner leave reassign
SUG-08: reset interleaving model-check
SUG-09: detach TODO
```

E2E:

```text
bootstrap -> attach -> start -> turn -> join/leave -> owner leave -> reset check -> closeout
```

Visualization:

```text
Place graph
membership timeline
turn trace
published roll history
witness/handoff arrows
```

---

### Phase 8 — Avatar fairy follow / fallback anchor

Purpose:

```text
avatar-attached object follows another avatar head with local fallback.
```

Samples:

```text
FAIRY-01: follow remote head with local fallback
FAIRY-02: remote not visible falls back to local head
FAIRY-03: remote avatar leaves falls back to local
FAIRY-04: invalid cross-anchor chain rejected
FAIRY-05: target reacquired after return
FAIRY-06: no detached anchor observed model-check
```

E2E:

```text
avatar object attached -> remote transform visible -> fallback event -> local transform used -> reacquire
```

Visualization:

```text
anchor chain graph
current anchor
fallback reason
transform source timeline
```

Important semantic note:

```text
Do not chain other_avatar.head -> self_avatar.head directly as unrelated targets.
Define an abstract FollowAnchor role and use remote/local options as candidates for that role.
```

---

### Phase 9 — Typed external boundary / adapter

Purpose:

```text
standard I/O replacement via typed external effects.
```

Samples:

```text
EXT-01: LogText adapter local console
EXT-02: ShowFloatingText world overlay
EXT-03: SendRoomMessage local queue
EXT-04: adapter failure typed result
EXT-05: debug visualization label restriction
```

E2E:

```text
effect request -> adapter route -> typed receipt/failure -> witness/debug artifact
```

Visualization:

```text
effect route graph
adapter call trace
failure route
```

---

### Phase 10 — MessageEnvelope / authentication seam

Purpose:

```text
authentication can be inserted later without changing high-level semantics.
```

Samples:

```text
AUTH-01: local unauthenticated emulator envelope
AUTH-02: session token auth layer
AUTH-03: invalid token rejected
AUTH-04: authenticated but unauthorized reset rejected
AUTH-05: signed payload reserve sample
```

E2E:

```text
message envelope -> auth layer -> authorization -> membership -> capability -> effect dispatch
```

Visualization:

```text
message flow
auth status
authorization decision
membership epoch
```

---

### Phase 11 — TermSignature / LayerSignature

Purpose:

```text
all meaningful terms and layers expose typed signatures.
```

Samples:

```text
SIG-01: handoff TermSignature dump
SIG-02: external effect signature dump
SIG-03: auth LayerSignature dump
SIG-04: visualization LayerSignature dump
SIG-05: residual obligation propagation
```

E2E:

```text
source -> signatures -> layer composition -> debug output
```

Visualization:

```text
signature table
layer input/output graph
residual obligation list
```

---

### Phase 12 — Projection / placement

Purpose:

```text
system-level source can be projected into Place-specific programs.
```

Samples:

```text
PROJ-01: Sugoroku server projection
PROJ-02: Sugoroku participant client projection
PROJ-03: move roll_dice from participant to server
PROJ-04: projection rejects missing route
PROJ-05: projection debug graph
```

E2E:

```text
system source -> projected server/client stubs -> local emulator run -> same trace
```

Visualization:

```text
source-to-place mapping
effect route plan
generated stubs
```

---

### Phase 13 — Network transport

Purpose:

```text
logical Place runtime can move to separate OS process / network transport.
```

Samples:

```text
NET-01: local queue transport
NET-02: two-process loopback
NET-03: websocket or TCP transport
NET-04: reconnect with membership epoch
NET-05: transport failure explicit reject/refresh
```

E2E:

```text
server process + participant process exchange typed envelopes and run Sugoroku turn
```

Visualization:

```text
network message timeline
latency/queue telemetry
auth evidence view
```

---

### Phase 14 — Hot-plug package / AttachPoint

Purpose:

```text
safe runtime attach of games/objects/tools.
```

Samples:

```text
HOT-01: Sugoroku package attach
HOT-02: Avatar fairy package attach
HOT-03: incompatible schema rejected
HOT-04: authority missing attach rejected
HOT-05: activation at next cut
HOT-06: detach TODO / lifecycle boundary
```

E2E:

```text
package manifest -> compatibility check -> attach -> activate -> debug lifecycle trace
```

Visualization:

```text
AttachPoint compatibility
package requirements/provides
activation cut
```

---

### Phase 15 — Visualization / IDE

Purpose:

```text
static and runtime structures become visible without leaking data.
```

Samples:

```text
VIS-01: Place graph viewer
VIS-02: Event DAG viewer
VIS-03: TermSignature viewer
VIS-04: membership timeline
VIS-05: witness timeline
VIS-06: model-check counterexample viewer
VIS-07: performance telemetry viewer
VIS-08: redaction policy demo
```

E2E:

```text
run sample -> produce trace -> open viewer -> inspect state/flow/signatures/witnesses
```

Visualization:

```text
This phase itself is visualization. It must show typed/redacted data.
```

---

### Phase 16 — Compiler/backend/LLVM preparation

Purpose:

```text
prepare for heavier compiler/backend work without breaking small VPS.
```

Samples:

```text
LLVM-01: backend environment probe
LLVM-02: generated IR stub
LLVM-03: build cache on external storage
LLVM-04: detach-safe cleanup
```

E2E:

```text
minimal compiled artifact path with caches on external storage
```

Visualization:

```text
build artifact location report
storage usage dashboard
```

---

## 10. E2E policy

Each phase must have both:

```text
Unit tests:
  local correctness of a component.

E2E-style samples:
  natural layer composition showing the phase works in context.
```

Do not create fake thick wrappers just to claim E2E.

Good E2E:

```text
Sugoroku:
  membership -> attach -> start -> roll -> publish -> handoff -> late join
```

Bad E2E:

```text
A wrapper that just calls internal functions with no realistic state, effect, or trace.
```

E2E must include:

```text
input/source
execution
debug/visualization output
expected success/failure
artifact or trace
human-readable explanation
```

---

## 11. Git discipline

Use frequent small commits.

### 11.1 Before work

```bash
git status --short
git branch --show-current
```

If dirty state exists, inspect it. Do not overwrite unknown user work.

### 11.2 Commit cadence

Commit after each completed package or coherent milestone:

```text
docs: add samples progress format
samples: add sugoroku owner leave case
runtime: add message envelope carrier
viz: add signature dump debug output
storage: add external storage environment scripts
```

### 11.3 Push cadence

Push after successful validation for each meaningful milestone.

Never push:

```text
- unvalidated breaking changes
- generated junk
- large cache/build artifacts
- secrets
- temporary local config
```

### 11.4 Commit message format

Use:

```text
<area>: <short summary>
```

Examples:

```text
docs: define samples progress matrix
runtime: add logical place message queue
typing: add finite index capture constraints
model-check: add owner leave reassignment property
viz: add term signature debug output
storage: add detachable artifact root scripts
```

### 11.5 Report before 100%

Before declaring a package 100%:

```bash
git status --short
# run validation
# update samples_progress.md
# write report
git add ...
git commit ...
git push
```

---

## 12. Storage management for ConoHa VPS

Known environment:

```text
VPS: ConoHa VPS 1GB plan
Swap/virtual memory: 20GB allocated
Additional storage: 200GB attached but not yet mounted
Concern: future LLVM/toolchain/build artifacts may exceed root disk
```

### 12.1 Storage principles

1. Keep source repo on stable main disk unless intentionally moved.
2. Put heavy disposable build artifacts on external storage.
3. Keep detachable storage cleanly separable.
4. Never put irreplaceable uncommitted source only on detachable storage.
5. Make it possible to clean/detach storage without damaging repo data.
6. Document all symlinks and environment variables.
7. Do not mount/delete/format anything without inspecting current devices.

### 12.2 First storage audit

CodeX must run:

```bash
df -h
lsblk -f
findmnt
du -sh . 2>/dev/null || true
du -sh target .git .cargo .lake 2>/dev/null || true
git status --short
```

Do not assume device names.

### 12.3 Suggested mount layout

Use an explicit mount point, for example:

```text
/mnt/mirrorea-work
```

Suggested subdirectories:

```text
/mnt/mirrorea-work/
  cargo-target/
  cargo-registry-cache/
  llvm/
    src/
    build/
    install/
  lean-cache/
  generated-artifacts/
  temp/
  logs/
```

Keep repo source and committed docs in the repo, not only on external storage.

### 12.4 Environment script

Create or update:

```text
scripts/env/mirrorea_storage_env.sh
```

Example:

```bash
#!/usr/bin/env bash
set -euo pipefail

export MIRROREA_WORKDIR="${MIRROREA_WORKDIR:-/mnt/mirrorea-work}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$MIRROREA_WORKDIR/cargo-target}"
export MIRROREA_GENERATED_ARTIFACT_DIR="${MIRROREA_GENERATED_ARTIFACT_DIR:-$MIRROREA_WORKDIR/generated-artifacts}"

mkdir -p "$CARGO_TARGET_DIR" "$MIRROREA_GENERATED_ARTIFACT_DIR"
echo "MIRROREA_WORKDIR=$MIRROREA_WORKDIR"
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "MIRROREA_GENERATED_ARTIFACT_DIR=$MIRROREA_GENERATED_ARTIFACT_DIR"
```

### 12.5 LLVM policy

LLVM must not be built under the repo root or root disk unless explicitly intended.

Recommended:

```text
LLVM source:  /mnt/mirrorea-work/llvm/src
LLVM build:   /mnt/mirrorea-work/llvm/build
LLVM install: /mnt/mirrorea-work/llvm/install
```

Do not commit LLVM sources/build outputs.

### 12.6 Detach preparation

Create script:

```text
scripts/storage/detach_prepare.sh
```

Required behavior:

```text
- print git status
- fail or warn if uncommitted generated reports/docs not committed
- summarize external storage contents
- identify disposable directories
- optionally clean cargo-target / temp / logs only after confirmation
- never delete repo source
- never delete docs/reports without explicit confirmation
```

Sample shape:

```bash
#!/usr/bin/env bash
set -euo pipefail

WORKDIR="${MIRROREA_WORKDIR:-/mnt/mirrorea-work}"

echo "[detach_prepare] git status"
git status --short

echo "[detach_prepare] disk usage"
df -h
du -sh "$WORKDIR"/* 2>/dev/null || true

echo "[detach_prepare] disposable candidates:"
echo "  $WORKDIR/cargo-target"
echo "  $WORKDIR/temp"
echo "  $WORKDIR/logs"

echo "No files deleted. Re-run cleanup script with explicit confirmation if needed."
```

### 12.7 Cleanup policy

Create a separate cleanup script. It must require explicit confirmation.

```text
scripts/storage/cleanup_disposable_artifacts.sh
```

Rules:

```text
- do not clean without explicit flag such as --confirm
- never remove source repo
- never remove committed reports/docs
- clean only known disposable directories
- log what was removed
```

### 12.8 Gitignore

Ensure `.gitignore` excludes:

```text
target/
.mirrorea-work/
generated-artifacts/
*.tmp
```

Do not ignore docs/reports or source samples.

### 12.9 samples_progress storage row

`samples_progress.md` should include storage/build environment status:

```markdown
## Build/storage environment

| Item | Status | Path | Notes |
|---|---|---|---|
| External workdir | mounted / not mounted | /mnt/mirrorea-work | ... |
| Cargo target | external / repo / default | ... | ... |
| LLVM build | not started / external / root-risk | ... | ... |
| Detach prep script | yes/no | scripts/storage/detach_prepare.sh | ... |
```

---

## 13. Reporting standard

Every non-trivial package report must include:

```text
summary
scope
files read
files changed
samples affected
samples_progress update
validation commands
validation results
debug/visualization outputs
git commit hash
push status
storage impact
remaining blockers
next step
```

Recommended report path:

```text
docs/reports/<next>-<package-name>.md
```

Do not write vague reports. Reports must be actionable.

---

## 14. Anti-shortcut rules

Coding agents often shortcut. These rules are mandatory.

Do not:

```text
- silently reduce scope
- change goal definitions without reporting
- skip negative samples
- skip validation
- fake validation success
- leave stale docs active
- move old samples without an archive note
- create thick wrappers just to claim E2E
- add domain-specific builtin primitives
- collapse authentication into transport
- treat visualization as untyped debug leak
- put source-only data on detachable storage
- declare 100% without samples_progress/report/validation/git
```

If a task is too large:

```text
- narrow it explicitly
- update tasks.md
- update samples_progress.md
- write report
- continue with next package
```

Do not silently stop.

---

## 15. What CodeX should do next

Immediate next package:

```text
Package: phase-sample-progress-storage-foundation
```

Tasks:

```text
1. Read this handoff.
2. Inspect current repo.
3. Update or create samples_progress.md with the format above.
4. Add sample plan matrix for all phases/layers.
5. Add AGENTS.md rules for sample progress, validation, commits, storage.
6. Add storage audit scripts without destructive behavior.
7. Update progress.md and tasks.md.
8. Write report.
9. Run docs validation if available.
10. Commit and push.
```

Then continue:

```text
Package 2: Sugoroku sample progress alignment
Package 3: Avatar fairy follow sample plan
Package 4: TermSignature debug output
Package 5: MessageEnvelope/auth seam
Package 6: VisualizationProtocol first implementation
```

---

## 16. Copy-paste checklist for CodeX report

At the end of each package, report:

```text
- package:
- percent reached:
- samples_progress.md updated:
- samples at 1%:
- samples at 50%:
- samples at 100%:
- validation run:
- e2e samples run:
- visualization/debug outputs:
- git commit:
- git push:
- storage status:
- report path:
- next package:
```

---

## 17. Final reminder

The key is not to rush to final public APIs. The key is to make every layer self-evident through runnable samples, validation, reports, and visualization.

This project progresses by making each layer:

```text
theoretically grounded
runnable
testable
visible
documented
incrementally composable
```

That is the correct path toward Mirrorea.
