# Mirrorea next-stage full plan handoff for CodeX

**作成日:** 2026-04-27  
**対象:** CodeX / GPT-5.4 xhigh + sub-agents  
**想定配置先:** `sub-agent-pro/mirrorea_next_stage_full_plan_handoff_2026-04-27.md`  
**目的:** 現在の repo-local alpha current layer から、Mirrorea runtime / typed external boundary / projection / visualization / network / hot-plug へ段階的に進むための全体計画・自走規律・検証方針を単一ファイルに集約する。

---

## 0. この handoff の位置づけ

このファイルは、CodeX が新規コンテキストで再開するための **full planning handoff** である。  
規範正本ではない。実際に repo に反映するときは、必ず `specs/` / `plan/` / `docs/` / `progress.md` / `tasks.md` / `samples_progress.md` / `AGENTS.md` へ、役割に応じて転記・整理する。

この handoff は、以下の会話・方針を統合している。

- Mir current layer / clean sample suite / finite-index typing / order-handoff / model-check / Lean foundation の repo-local alpha 化
- Sugoroku runtime attachment vertical slice
- Avatar fairy follow / fallback anchor vertical slice
- typed external boundary / standard I/O を持たない設計
- effect-based OS-like substrate という内側の解釈
- TermSignature / LayerSignature / MessageEnvelope / AuthEvidence
- VerificationLayer composition
- visualization / debug / telemetry
- projection / placement mobility
- hot-plug Patch / AttachPoint
- repository restructuring and samples progress dashboard
- storage / git / validation / reporting discipline
- CodeX の goal drift / shortcut 対策

---

## 1. 一番重要なプロジェクト軸

Mirrorea の主軸は次である。

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

この軸から外れる局所最適化をしてはいけない。

### 1.1 主軸に入るもの

- Place / effect / capability / authority / witness / cut / membership を持つ意味論
- finite-index typing による権限・ラベル・capture・lifetime・cost の静的検査
- theorem / model-check / runtime-policy の分担
- typed external effect boundary
- runtime attachment / hot-plug
- multi-place execution / routing / projection
- static + runtime visualization
- debug / telemetry / performance trace
- 実行可能 sample / E2E / hands-on docs

### 1.2 主軸ではないが重要な内側の解釈

Mirrorea runtime は、局所的には次のようにも読める。

```text
typed algebraic-effect-like OS substrate
+ typed communication / routing system
+ host adapter ecosystem
```

これは有用な解釈である。  
ただし、プロジェクト全体を「OS を作ること」と誤解しない。主軸はあくまで **correct hot-pluggable virtual-space system** である。

---

## 2. 現在地の読み

現状の repo は、おおむね次の段階にあると読む。

```text
Mir current substrate:
  clean sample suite, finite-index typing, order/handoff,
  model-check second line, Lean foundations, helper CLIs がかなり固まっている。

Mirrorea future-axis:
  typed external boundary, TermSignature, LayerSignature,
  MessageEnvelope/Auth seam, VisualizationProtocol,
  projection, network canary, hot-plug canary が helper-local / report-local preview として進んでいる。

Mirrorea runtime/product:
  まだ本格実装はこれから。
  mirrorea-core / mirrorea-control 等が skeleton の場合は、それを誤って完成扱いしない。
```

### 2.1 完了扱いしてよい current layer

次の範囲は、repo-local alpha current layer としてかなり完成に近い。

- `samples/clean-near-end/` active suite
- typing / order-handoff / model-check / modal / Lean hands-on
- Sugoroku runtime attach sample
- Avatar fairy follow sample
- old sample separation
- finite-index first strong typing
- high-level order relation family
- model-check second line examples
- Lean foundation / generated theorem stubs
- sample dashboards and closeout helpers

### 2.2 完了扱いしてはいけないもの

以下はまだ final public completion ではない。

- final parser grammar
- final public parser/checker/runtime API
- final public verifier contract
- production theorem-prover binding
- production model-checker binding
- real network production transport
- production auth stack
- production hot-plug package manager
- production visual debugger / IDE
- packaging / installed binary
- final application product

---

## 3. Source hierarchy

CodeX は常に次の階層を守る。

```text
specs/:
  normative source of truth

plan/:
  repository memory / roadmap / risks / sequencing

docs/reports/:
  task evidence and historical record

docs/research_abstract/ and docs/hands_on/:
  human-facing explanation

progress.md and tasks.md:
  current snapshot

samples_progress.md:
  runnable sample dashboard

sub-agent-pro/:
  handoff and task instructions; not normative by itself
```

`sub-agent-pro/` にある handoff は、読むべき作業指示である。  
ただし、規範化すべき内容は必ず `specs/` / `plan/` / docs に転記する。

---

## 4. Non-negotiable rules

### 4.1 No domain builtin shortcut

以下を、final builtin primitive にして問題を解決してはいけない。

```text
declassify_authority
observer_role
fingerprint_bound
fingerprint_visible
owner_is_valid
remote_visible
fairy_follow_ok
```

こうした domain-specific predicate は、以下に落とす。

```text
user-defined finite index theory
policy module
capability relation
membership/runtime guard
residual theorem/model-check obligation
```

### 4.2 Type/index layer is finite first

First implementation は full dependent type theory ではない。  
目標は次。

```text
finite preorder
finite lattice
finite powerset/capture set
region/lifetime preorder
simple resource/cost bound
effect row inclusion
```

### 4.3 Standard I/O is not a core primitive

Mir core は `stdin/stdout/stderr` を持たない。  
外界との接続は typed external effect adapter で行う。

### 4.4 `atomic_cut` is local

`atomic_cut` を global sync / durable commit / C++ seq_cst fence / distributed consensus にしない。

### 4.5 Layer composition must preserve evidence

検証層・認証層・可視化層を足しても、既存 witness / proof / constraint / authority / label の意味を壊さない。

### 4.6 No fake validation

実行していない validation を passed と書かない。  
コマンドが存在しない場合は、存在しないと明記する。

---

## 5. Repository structure target

今すぐ全移動はしない。  
まず layer-aware target map を repo に記録し、低リスクなところから移す。

推奨 target:

```text
repo/
  AGENTS.md
  README.md
  Documentation.md
  progress.md
  tasks.md
  samples_progress.md

  specs/
    examples/

  plan/

  docs/
    research_abstract/
    hands_on/
    reports/
    papers/
    visualizer_notes/

  sub-agent-pro/

  crates/
    mir-core/                 # future common semantic core
    mir-ast/                  # parser / AST carrier
    mir-checker/              # finite-index checker, future split
    mir-semantics/            # event DAG / semantics
    mir-runtime/              # current runner / CLI / helper runtime
    mir-verification/         # theorem/model-check carrier, future split
    mirrorea-core/            # PlaceRuntime / MessageEnvelope / Membership
    mirrorea-runtime/         # logical multi-place runtime
    mirrorea-adapters/        # external adapters
    mirrorea-visualization/   # visualization/debug/export

  samples/
    clean-near-end/
      typing/
      order-handoff/
      model-check/
      modal/
      sugoroku-world/
      avatar-follow/
    old/
    generated/
    lean/
    current-l2/
    not_implemented/

  formal/
    lean/
    rocq/
    model-check/

  scripts/
    samples/
    validation/
    docs/
    storage/
    visualization/

  tools/
    syntax-highlight/
    viewers/
    dev/
```

### 5.1 Immediate restructuring policy

Do now:

- maintain `samples_progress.md`
- active / old / generated sample separation
- document layer map
- AGENTS.md discipline update
- storage scripts / env if needed
- reports after every substantial task

Do not do yet:

- full crate renaming
- final public package structure
- final parser API
- final verifier API
- large destructive moves without validation

---

## 6. Current and next package queue

The next self-driven queue should be explicit.  
Recommended order:

```text
P0. Current-state audit and source-hierarchy validation
P1. Repository layer map and samples_progress stabilization
P2. Typed external boundary residual planned family review
P3. Projection / placement residual emitted-program gate
P4. TermSignature registry hardening
P5. LayerSignature system hardening
P6. MessageEnvelope / Auth seam hardening
P7. VisualizationProtocol + VisualizationSecurity
P8. Sugoroku runtime attach hardening
P9. Avatar fairy follow hardening
P10. mirrorea-core first real implementation tranche
P11. mirrorea-runtime logical multi-place emulator tranche
P12. external adapter / host boundary tranche
P13. network transport minimal alpha
P14. hot-plug Patch / AttachPoint package manager tranche
P15. projection/codegen first emitted server/client programs
P16. visual debugger / viewer first public prototype
P17. storage / LLVM / backend preparation
P18. public API / parser grammar gate
```

Each package must have:

```text
objective
input files
deliverables
validation commands
debug/visualization output
docs updates
report
stop line
next package
```

---

## 7. Phase / layer sample plan

Each phase/layer must have unit-like validation and E2E-like validation.  
E2E must not be a thick fake wrapper. It must naturally compose real layers.

### Phase 0: Repository memory / decision boundary

Sample/check:

```text
docs source hierarchy audit
AGENTS.md conformance
specs/plan/report separation
```

Validation:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
```

### Phase 1: Mir current L2 semantics

Samples:

```text
fallback / lease / no re-promotion
atomic_cut local behavior
guarded option chain
```

Validation:

```bash
python3 scripts/current_l2_guided_samples.py closeout --format json
```

### Phase 2: Parser-free PoC / detached validation

Samples:

```text
fixture -> run_bundle -> detached artifact -> compare
```

Validation:

```bash
python3 scripts/current_l2_source_sample_regression.py regression
```

### Phase 3: Parser boundary / checker cut

Samples:

```text
stage1 option/chain/lineage
stage2 try/atomic_cut
stage3 admit/require/ensure
```

Validation:

```bash
cargo test -p mir-ast
```

### Phase 4: Shared-space membership / practical room

Samples:

```text
membership_epoch / member_incarnation
authoritative room baseline
late join / stale reconnect
```

Validation:

```bash
python3 scripts/sugoroku_world_samples.py closeout --format json
```

### Phase 5: Small decidable core / proof boundary

Samples:

```text
typing / finite-index / theorem-first / model-check second line
```

Validation:

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
```

### Phase 6: Compile-ready minimal actualization

Samples:

```text
clean near-end full suite
Lean foundation
generated theorem stubs
```

Validation:

```bash
python3 scripts/clean_near_end_samples.py smoke-all
python3 scripts/clean_near_end_samples.py closeout
```

### Phase 7: Sugoroku runtime attach

Samples:

```text
world bootstrap
runtime attach
admin start/reset
roll publish handoff
late join
leave
owner leave
reset interleaving
detach TODO
```

Validation:

```bash
python3 scripts/sugoroku_world_samples.py closeout --format json
```

### Phase 8: Avatar fairy follow

Samples:

```text
remote head with local fallback
remote not visible -> local fallback
remote avatar leaves -> fallback
invalid cross-anchor chain rejected
no detached anchor observed
```

Validation:

```bash
python3 scripts/avatar_follow_samples.py closeout --format json
```

### Phase 9: Typed external boundary

Samples:

```text
LogText
PublishFloatingText
SendRoomMessage
ReadAvatarTransform
AttachComponent
```

Validation:

```bash
python3 scripts/typed_external_boundary_samples.py closeout --format json
```

### Phase 10: MessageEnvelope / Auth seam

Samples:

```text
local no-auth envelope
session-token auth envelope
signature auth envelope
membership epoch mismatch rejection
```

Validation:

```bash
python3 scripts/network_transport_samples.py closeout --format json
```

### Phase 11: TermSignature / LayerSignature

Samples:

```text
handoff signature dump
external effect signature dump
verification layer signature dump
visualization layer signature dump
```

Validation:

```bash
cargo test -p mir-runtime clean_near_end_closeout_records_signature_inventory
```

### Phase 12: Projection / placement

Samples:

```text
system-level Sugoroku transition -> server/participant preview
server/client projection preview
effect route plan
```

Validation:

```bash
python3 scripts/projection_placement_samples.py closeout --format json
```

If script missing, create planned task; do not fake result.

### Phase 13: Network transport

Samples:

```text
local queue transport
loopback transport
auth envelope through transport
stale/reconnect path
```

Validation:

```bash
python3 scripts/network_transport_samples.py closeout --format json
```

### Phase 14: Hot-plug Patch / AttachPoint

Samples:

```text
FairyFollower patch
SugorokuGame patch
compatibility check
activation at next cut
detach TODO
```

Validation:

```bash
python3 scripts/hotplug_attachpoint_samples.py closeout --format json
```

### Phase 15: Visualization / IDE

Samples:

```text
syntax highlighter
Place graph
Event DAG
message flow
membership timeline
witness timeline
TermSignature view
visualization redaction
```

Validation:

```bash
python3 scripts/visualization_samples.py closeout --format json
```

### Phase 16: Compiler / backend / LLVM preparation

Samples:

```text
backend env
storage policy
generated artifact placement
no root disk pressure
```

Validation:

```bash
python3 scripts/storage/detach_prepare.sh --dry-run
```

---

## 8. samples_progress.md standard

`samples_progress.md` must be maintained.  
Suggested schema:

```text
# samples_progress

Last updated: YYYY-MM-DD HH:MM JST
Current repo-local focus: ...
Current active packages: ...

## Legend

Progress:
- 0%: not scheduled
- 1%: started; sample ID and goal exist
- 10%: spec/sample skeleton exists
- 25%: parser/loader/static carrier exists
- 50%: minimal implementation passes primary positive sample
- 65%: negative/rejection samples pass
- 75%: debug/visualization output exists
- 90%: E2E/regression/closeout validation passes
- 100%: complete for current scope: implementation + positive/negative samples + debug/visualization + docs + report + tests + progress update + git commit/push

## Summary

| Layer | Overall % | Status | Current focus | Next validation |
|---|---:|---|---|---|

## Active sample matrix

| Sample ID | Layer | Path | Kind | Progress | Positive/Negative | Last validation | Docs | Notes |
|---|---|---|---|---:|---|---|---|---|

## E2E samples

| E2E ID | Scope | Path / command | Progress | What it proves | Last result |
|---|---|---|---:|---|---|

## Current blockers

| Blocker | Layer | Severity | Owner | Next action |
|---|---|---|---|---|

## Recent validation

| Date | Command | Result | Notes |
|---|---|---|---|

## Historical / archived samples

| Old path | New path / status | Reason |
|---|---|---|
```

### 8.1 Progress percent rules

- 1%: 着手直後
- 50%: 最小の実装
- 100%: current scope の完全実装

100% requires:

```text
implementation
positive sample
negative sample
debug/visualization output
docs
report
tests
progress update
git commit/push if available
```

---

## 9. TermSignature

Every meaningful term should eventually expose:

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
  "capability_requirements": ["DiceOwner(Alice)", "ActiveParticipant(Bob)"],
  "runtime_guards": ["membership_epoch is current"],
  "evidence_required": ["PublicationWitness(draw)"],
  "evidence_produced": ["DiceOwner(Bob)", "HandoffWitness(dice_owner, Alice, Bob)"]
}
```

This is required for:

- checker
- theorem
- model-check
- projection
- adapter routing
- debug visualization
- hot-plug compatibility

---

## 10. LayerSignature

Layer composition should use:

```text
LayerSignature {
  name
  requires
  provides
  transforms
  checks
  emits
  obligations
  laws
}
```

Examples of layers:

- `FiniteIndexChecker`
- `LeanTheoremLayer`
- `ModelCheckSecondLine`
- `SessionTokenAuth`
- `SignatureAuth`
- `NetworkTransport`
- `VisualizationLayer`
- `TelemetryLayer`
- `ProjectionLayer`
- `HotPlugActivationLayer`

The layer laws must preserve:

```text
no hidden authority
no hidden data downgrade
no hidden effect
evidence preservation
placement preservation
visualization respects labels
residual obligations explicit
```

---

## 11. MessageEnvelope / Auth seam

Use a typed envelope:

```text
MessageEnvelope {
  from_place       : PlaceId
  to_place         : PlaceId
  principal_claim  : PrincipalClaim
  auth_evidence    : Optional<AuthEvidence>
  membership_epoch : MembershipEpoch
  incarnation      : MemberIncarnation
  effect_request   : EffectRequest
  payload          : TypedPayload
  required_caps    : CapabilitySet
  witness_refs     : List[WitnessRef]
}
```

Do not collapse authentication into transport.

Separate:

```text
Transport
Authentication
Authorization
Membership
Capability
Witness/Audit
```

---

## 12. Typed external boundary

Mir core has no standard I/O.

All external interaction goes through typed external effects:

```mir
external effect SendRoomMessage {
  input room : RoomRef
  input sender : Principal
  input body : Text @ SecurityLabel.Public

  output receipt : MessageReceipt

  requires member(sender, room)
  requires effect_capability(sender, send_message)

  effects {
    network_send
    audit_append
  }

  failures {
    NotMember
    RoomUnavailable
    TransportFailure
  }

  evidence {
    emits MessageSentWitness
  }
}
```

Adapters implement effects:

```mir
adapter WebSocketRoomTransport implements SendRoomMessage {
  transport websocket
  auth session_token
  route room.server_endpoint
}
```

---

## 13. Visualization and debug

Visualization is not a free debug leak.  
It is an effect and must respect labels / authority / redaction.

### 13.1 Static views

- Place graph
- Effect route graph
- TermSignature view
- index constraints
- proof obligations
- projection view
- hot-plug compatibility view

### 13.2 Runtime views

- Event DAG
- message flow
- state timeline
- membership timeline
- witness timeline
- data flow
- failure / rejection view
- performance / telemetry
- hot-plug lifecycle

### 13.3 Debug output levels

Use levels:

```text
--debug summary
--debug signatures
--debug events
--debug messages
--debug membership
--debug witnesses
--debug telemetry
--debug full
```

Do not overwhelm users.  
Default debug output should be `summary`.

---

## 14. Projection / placement mobility

A core invariant:

```text
system-level source should not be tied to fixed server/client split.
```

The system should support:

```text
source program
  -> place-specific programs
  -> effect route plans
  -> adapter plans
  -> server/client generated fragments
```

Placement movement requires checking:

- type compatibility
- effect availability
- capability preservation
- witness availability
- dataflow labels
- transport route
- auth evidence
- latency/cost assumptions
- residual obligations

---

## 15. Hot-plug and patch typing

A patch should be represented as typed artifact:

```text
Patch Req Prov Δ
```

Where:

```text
Req  = required capabilities/effects/state schema
Prov = provided exports/effects/objects
Δ    = migration / compatibility transform
```

Attach point:

```text
AttachPoint Host Req Prov Δ
```

Staged reading:

```text
□ Patch Req Prov Δ       stable patch artifact
attach -> ◯ AttachedPatch next activation boundary
```

This is where λ◯□ / staged computation naturally contributes.

---

## 16. Storage and environment discipline

The environment may be a small VPS with external storage.

Never put heavy artifacts in repo root.

Use env:

```bash
export MIRROREA_WORKDIR="${MIRROREA_WORKDIR:-/mnt/mirrorea-work}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$MIRROREA_WORKDIR/cargo-target}"
export MIRROREA_GENERATED_ARTIFACT_DIR="${MIRROREA_GENERATED_ARTIFACT_DIR:-$MIRROREA_WORKDIR/generated-artifacts}"
```

Required scripts if not already present:

```text
scripts/env/mirrorea_storage_env.sh
scripts/storage/detach_prepare.sh
scripts/storage/cleanup_disposable_artifacts.sh
```

No destructive cleanup without explicit confirmation.

---

## 17. Git, reports, and Discord

### 17.1 Git

Commit after coherent milestones.  
Push where available.

Do not commit:

- build cache
- LLVM source/build
- huge generated artifacts
- secrets
- detached storage data

### 17.2 Reports

Every non-trivial run gets a report:

```text
docs/reports/<next>-<task-name>.md
```

Report must include:

- summary
- files inspected
- files changed
- validation run
- validation skipped
- risks
- next package

### 17.3 Discord

If Discord skill exists, report progress.  
Do not report 100% unless actual scope completion criteria are met.

---

## 18. Anti-shortcut policy

CodeX must not:

- silently shrink scope
- skip validation and claim success
- treat helper-local preview as final public implementation
- introduce domain builtins
- make `world` a heavy primitive without justification
- collapse auth into transport
- treat visualization as untyped debug leak
- create thick fake E2E wrappers
- leave stale references unnoticed
- claim final public completion
- make destructive storage changes

If uncertain, preserve existing working state and write a report.

---

## 19. Next major CodeX task after this handoff

Recommended immediate task:

```text
Integrate this future-plan handoff into repo docs/plan/AGENTS/progress/tasks/samples_progress,
then continue with Typed external boundary residual planned family review.
```

Recommended prompt is provided separately.

---

## 20. Expected final response format for CodeX tasks

Every major task should end with:

```text
- handoff read:
- files inspected:
- files changed:
- docs updated:
- specs updated:
- plan updated:
- samples_progress updated:
- validation run:
- validation skipped:
- report path:
- git commit:
- git push:
- remaining risks:
- next package:
```
