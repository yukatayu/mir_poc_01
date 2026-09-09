
# Mirrorea 今後計画・理論整合・実装自走 handoff

**想定配置先:** `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`  
**対象:** CodeX / sub-agent / future implementation lead  
**目的:** ここまでの Mir / Mirrorea 議論を踏まえ、今後の上位レイヤー計画、理論構築、検証強化、デバッグ・可視化、通信・認証・外界 adapter、projection / placement、hot-plug 仮想空間 runtime までを、一貫した計画として CodeX が自走できるようにまとめる。

---

## 0. 最重要メッセージ

このプロジェクトの主軸は次である。

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

これは単なるプログラミング言語でも、単なる OS でも、単なる VR アプリでもない。  
Mir は意味論・型・effect・witness・Place・検証境界の core。  
Mirrorea はそれを使い、runtime に後から game / object / tool / domain kernel を安全に attach し、Place 間で正しく実行・通信・観測・検証・可視化する基盤である。

短絡禁止:

```text
- 目先のサンプルだけ通して「完成」と言わない。
- 古い symbolic predicate を builtin として延命しない。
- final public API / parser grammar を根拠なく凍結しない。
- 標準 I/O を core primitive にしない。
- 認証や可視化を transport や debug hack に押し込まない。
- 型検査 / 定理証明 / モデル検査を雑な plugin として足さない。
- 上位レイヤーの計画を忘れて局所最適化しない。
```

---

## 1. 現在地の読み

### 1.1 今の current layer

これまでの作業により、Mir language alpha substrate / repo-local alpha current layer はかなり進んでいる。  
以下が active になっている想定である。

```text
- clean near-end sample suite
- finite-index first strong typing layer
- user-defined authority / label / capture / lifetime / cost
- order / handoff relation family
- model-check second line
- mutex / weak-memory examples
- Lean foundation / generated theorem stubs
- hands-on docs
- syntax highlighting prototype
- sugoroku runtime attachment vertical slice
```

ただし、これは final public product ではない。

まだ deferred:

```text
- final parser grammar
- final public parser API
- final public checker API
- final public runtime API
- final public verifier contract
- production theorem/model-check binding
- installed binary / packaging
- FFI / engine adapter / real network production stack
- exhaustive shared-space catalog
- broad application/product completion
```

### 1.2 次に必要な大きな転換

今後は Mir language alpha substrate から、Mirrorea virtual-space runtime substrate へ移る。  
次の中核を作る必要がある。

```text
- Typed external effect boundary
- TermSignature registry
- LayerSignature system
- MessageEnvelope / AuthEvidence / transport insertion seam
- VisualizationProtocol
- Projection / placement plan
- HotPlug Patch / AttachPoint system
- Logical multi-place runtime emulator
- Sugoroku / AvatarFairyFollow / future PrismCascade bridge samples
```

---

## 2. Source hierarchy

CodeX は必ず以下の情報階層を守ること。

```text
1. specs/:
   規範正本。

2. plan/:
   repository memory。現在地・roadmap・リスク・境界。

3. docs/reports/:
   task 単位の証跡。

4. progress.md / tasks.md:
   current snapshot。薄く、正確に、最新に保つ。

5. .docs/ または docs/:
   reader-facing documents / hands-on / visual guides。
   repo が `.docs/` を使っている場合は `.docs/` を優先する。
   無い場合は既存 docs layout を確認し、勝手に二重構造にしない。

6. AGENTS.md:
   agent 操作規約。現状を確認した上で、必要最小限かつ正確に更新する。

7. sub-agent-pro/*.md:
   handoff / prompt / long-form instruction。
   規範正本ではない。必要な内容は specs / plan / docs に反映する。
```

FAQ や handoff は current explanation / working directive であり、規範化するには `specs/` または `specs/examples/*` へ写す必要がある。

---

## 3. 必読ファイル

この handoff を使う task の最初に、CodeX は以下を読むこと。  
存在しないものは「missing」と報告し、推測で埋めない。

```text
sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md
sub-agent-pro/codex_clean_mir_near_end_completion_with_new_samples_2026-04-22.md
sub-agent-pro/codex_sugoroku_runtime_attachment_handoff_2026-04-23.md

AGENTS.md
README.md
Documentation.md
progress.md
tasks.md

specs/00-document-map.md
specs/01-charter-and-decision-levels.md
specs/02-system-overview.md
specs/03-layer-model.md
specs/04-mir-core.md
specs/05-mirrorea-fabric.md
specs/07-typed-effects-wiring-platform.md
specs/08-cross-system-relations.md
specs/09-invariants-and-constraints.md
specs/10-open-questions.md
specs/11-roadmap-and-workstreams.md
specs/12-decision-register.md

plan/00-index.md
plan/01-status-at-a-glance.md
plan/03-decision-strengths-and-boundaries.md
plan/04-core-semantics-current-l2.md
plan/06-surface-notation-status.md
plan/07-parser-free-poc-stack.md
plan/08-representative-programs-and-fixtures.md
plan/09-helper-stack-and-responsibility-map.md
plan/10-roadmap-overall.md
plan/11-roadmap-near-term.md
plan/12-open-problems-and-risks.md
plan/13-heavy-future-workstreams.md
plan/14-glossary-and-boundary-rules.md
plan/16-shared-space-membership-and-example-boundary.md
plan/17-research-phases-and-autonomy-gates.md
plan/90-source-traceability.md
plan/91-maintenance-rules.md
```

関連ディレクトリ:

```text
samples/
samples/clean-near-end/
samples/old/
samples/lean/
scripts/
crates/
docs/research_abstract/
.docs/             # if present
docs/reports/
```

---

## 4. 理論の主軸

### 4.1 Core judgement

現時点の主軸型判断は、概念的には次である。

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

意味:

```text
Σ = user-defined index theories / policies / finite preorders / finite lattices
Ψ = modes / stages / places / visibility / witness / durability environment
Γ = ordinary unrestricted typing context
Δ = linear / affine / capability / ownership context
e = term / expression / transition fragment
A = ordinary type
μ = mode
ε = effect row
C = compile-time finite constraints
O = residual obligations for theorem / model-check / runtime policy
```

これを忘れて局所実装してはいけない。

### 4.2 Full dependent type ではなく finite-index first layer

first public-ish implementation は full dependent core ではない。  
ただし依存型的な index typing は持つ。

扱う:

```text
- finite preorder
- finite lattice
- powerset lattice
- lifetime / region preorder
- capture set inclusion
- simple cost bound
- effect row inclusion
- authority hierarchy
- security label
- taint
```

扱わない:

```text
- arbitrary type-level computation
- unrestricted dependent pattern matching
- compiler-internal arbitrary theorem proving
- full asymptotic inference
- probabilistic IFC
- covert-channel theorem
```

### 4.3 λ◯□ / staged computation

`λ◯□` は full foundation ではなく、stage / later / stable の partial basis である。

```text
◯ A ≈ A @ later
□ A ≈ A @ stable
```

Mir ではさらに以下が必要。

```text
A @ local
A @ later
A @ stable
A @ place(room)
A @ published(room)
A @ witnessed(w)
A @ durable(scope)
```

Hot-plug patch は staged computation と相性が良い。

概念:

```text
stable package = □ Patch
activation at next cut/stage = ◯ AttachedComponent
```

ただし `Patch` や `AttachPoint` は kernel primitive にする必要はなく、multimodal dependent core 上の record / library type として扱う方が良い。

---

## 5. Place の定義

Place は単なる人間・participant ではない。

```text
Place =
  physical process
  physical machine
  OS process
  network node
  virtual thread
  authority / permission compartment
  state compartment
  observation frontier
  execution locus with local queue, state, capability, visibility
```

分ける:

```text
Principal / Participant:
  Alice, Bob, Carol, user, actor.

Place:
  ParticipantPlace[Alice]
  WorldServerPlace
  SugorokuGamePlace#1
  FairyFollowPlace
  AdapterPlace[WebSocketTransport]
```

Place は physical / virtual を一般化した実行・観測・権限の locus。  
システム全体の source を後から place-specific program に projection するための基礎でもある。

---

## 6. 標準入出力なし / typed external effect boundary

Mir core に Unix 的な標準入出力を入れない。

入れない:

```text
stdin
stdout
stderr
process-global console
untyped print/read
```

外界とは typed external effect adapter で繋ぐ。

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

Adapter example:

```mir
adapter WebSocketRoomTransport implements SendRoomMessage {
  transport websocket
  auth session_token
  route room.server_endpoint
}
```

普通の unsafe FFI ではなく、型付き effect boundary である。

---

## 7. Effect-based OS + communication system という解釈

これは局所的には正しい。

```text
Mirrorea runtime =
  typed algebraic-effect-like OS substrate
  + communication / routing system
  + adapter / hot-plug runtime
```

ただし、プロジェクトの主軸は OS ではない。

主軸:

```text
正しい理論に基づき、正しく hot-plug できる仮想空間システム。
```

Effect-based OS 的な部分は内側の runtime substrate / adapter ecosystem として扱う。

---

## 8. TermSignature registry

次に必ず作るべき中核。  
各 term / transition / effect / message / adapter に signature を持たせる。

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

これがないと、以下が全部ばらばらに解釈される。

```text
- static checker
- theorem prover
- model checker
- runtime guard
- projection / placement
- network adapter
- debug output
- visualization
- hot-plug compatibility
```

### Required package

```text
Package: TermSignature registry + debug dump
```

Deliverables:

```text
TermSignature
SignatureRegistry
SignatureDump JSON
--debug signatures
tests
docs
hands-on
```

---

## 9. LayerSignature system

認証・検証・可視化・transport・telemetry などの層も、雑な plugin ではなく typed layer として扱う。

```text
LayerSignature {
  name

  requires:
    capabilities
    input signatures
    mode assumptions
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

### 合成の不変条件

```text
No hidden authority
No hidden data downgrade
No hidden effect
Evidence preservation
Placement preservation
Visualization respects labels
Residual obligations are explicit
```

### Required package

```text
Package: LayerSignature system
```

Deliverables:

```text
LayerSignature
LayerRequires
LayerProvides
LayerTransforms
LayerChecks
LayerEmits
LayerLaws
docs
examples
```

---

## 10. 認証・通信レイヤー

認証は transport に埋め込まない。

分ける:

```text
Transport:
  bytes / messages を運ぶ。WebSocket, QUIC, local queue, in-memory emulator.

Authentication:
  誰が送ったかを証明する。session token, signature, WebAuthn, DID, OAuth-like.

Authorization:
  principal が操作してよいか。

Membership:
  world / room / game に参加しているか。

Capability:
  operation capability を持つか。

Witness / Audit:
  action の証拠を残す。
```

### MessageEnvelope

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

最初は:

```text
auth_evidence = none
transport = local_queue
```

後から:

```text
auth_evidence = SessionTokenEvidence
transport = websocket
```

さらに:

```text
auth_evidence = SignatureEvidence
transport = quic
```

### Required package

```text
Package: MessageEnvelope / Auth insertion seam
```

Deliverables:

```text
MessageEnvelope
AuthEvidence
PrincipalClaim
MembershipEpoch
CapabilityProof
TransportAdapter
tests
docs
```

---

## 11. 検証レイヤーの合成

検証層はそれぞれ入出力と保証を持つ。

### FiniteIndexChecker

```text
VerificationLayer FiniteIndexChecker {
  input:
    TermSignature
    IndexTheoryDecl
    GeneratedConstraints

  checks:
    label_flow
    authority_ge
    capture_subset
    region_outlives
    cost_leq

  output:
    StaticCheckCertificate

  residual:
    constraints_outside_decidable_fragment
}
```

### LeanTheoremLayer

```text
VerificationLayer LeanTheoremLayer {
  input:
    ProofObligation
    EvidenceRefs
    LeanContext

  checks:
    preservation_fragment
    ifc_no_illicit_flow
    no_repromotion
    rollback_cut_non_interference

  output:
    TheoremCertificate | ProofStub

  residual:
    unproven_goals
}
```

### ModelCheckSecondLine

```text
VerificationLayer ModelCheckSecondLine {
  input:
    FiniteProtocolModel
    EventRelationModel
    PropertySpec

  checks:
    no_double_owner
    stale_action_rejected
    late_join_sees_history
    mutex_mutual_exclusion
    weak_memory_counterexample

  output:
    ModelCheckResult
    CounterexampleTrace?

  residual:
    liveness_or_unbounded_property
}
```

### Required package

```text
Package: VerificationLayer composition
```

Deliverables:

```text
VerificationLayer
ProofObligation
ModelCheckCase
Certificate
ResidualObligation
tests
docs
```

---

## 12. 可視化 / デバッグ / telemetry

可視化は「便利機能」ではなく必須構成要素。  
ただし可視化も effect であり、情報漏洩しうる。

### Static visualization

```text
Place graph
Effect route graph
Type/index constraint view
TermSignature view
Proof obligation graph
Hot-plug compatibility view
Projection view
```

### Runtime visualization

```text
Event DAG
Message flow
State timeline
Membership timeline
Witness timeline
Effect latency / performance
Data flow
Failure / rejection view
Hot-plug lifecycle view
Debug overlay
```

### Visualization effect

```mir
effect ShowDebugTrace {
  input trace : DebugTrace @ DebugLabel
  output receipt : VisualReceipt

  requires authority >= DebugViewer
  requires label(trace) <= viewer_clearance
}
```

### Telemetry effect

```mir
effect RecordTelemetry {
  input event : RuntimeEvent
  input timing : TimingInfo
  output receipt : TelemetryReceipt

  requires telemetry_enabled
  label Internal
}
```

### Required package

```text
Package: VisualizationProtocol + VisualizationSecurity
```

Deliverables:

```text
StaticGraphSnapshot
RuntimeTrace
PlaceGraphSnapshot
EffectRouteSnapshot
MembershipTimeline
WitnessTimeline
ConstraintView
TelemetryTrace
DebugLabel
ViewerClearance
VisualizationEffectSig
RedactionPolicy
HTML or browser viewer prototype
docs
hands-on
```

---

## 13. Projection / placement mobility

システム全体を高位に書き、後から処理場所を動かせる性質は必須。

Source:

```mir
transition take_turn(actor) {
  draw <- roll_dice()
  publish draw
  handoff dice_owner actor -> next
}
```

Projection:

```text
server side:
  authoritative roll / publish / handoff

participant side:
  input / view / local debug

adapter:
  message transport / auth / serialization

visualizer:
  event trace / witness graph
```

### Design invariant

```text
source-level behavior should not depend on fixed server/client split
```

### Required package

```text
Package: Projection / placement plan
```

Deliverables:

```text
PlaceProjection
GeneratedServerProgram
GeneratedParticipantProgram
EffectRoutePlan
PlacementValidityReport
ProjectionValidityChecklist
docs
```

---

## 14. Hot-plug package / AttachPoint

Hot-plug patch を型付けする。

Concept:

```text
Patch Req Prov Δ
```

Meaning:

```text
Req  = requirements
Prov = provided exports/effects
Δ    = migration / compatibility transform
```

### Staged interpretation

```text
□ Patch Req Prov Δ
  stable patch artifact

attach : □ Patch Req Prov Δ -> AttachPoint Req Prov -> ◯ AttachReceipt
  activation after next cut/stage
```

### Example

```mir
slot fairy_slot : AttachPoint[
  host = AvatarHost,
  requires {
    authority >= WorldAuthority.Attach,
    imports = { HeadAnchor, VisibleTransform },
    effects <= { observe_transform, publish_visual },
    state_schema = AvatarState[v2]
  },
  provides {
    exports = { FairyObject, FollowAnchor }
  }
]

stable package FairyFollower :
  Patch[
    requires {
      authority >= WorldAuthority.Attach,
      imports = { HeadAnchor, VisibleTransform },
      effects <= { observe_transform, publish_visual },
      state_schema = AvatarState[v2]
    },
    provides {
      exports = { FairyObject, FollowAnchor }
    },
    migrate AvatarState[v2] -> AvatarState[v2]
  ]
```

### Required package

```text
Package: HotPlug Patch / AttachPoint design
```

Deliverables:

```text
Patch
AttachPoint
CompatibilityCheck
ActivationCut
MigrationContract
AttachReceipt
samples
docs
```

---

## 15. Sugoroku vertical slice

すでに計画済み。  
ただし以下の修正を必ず反映。

```text
- participants を固定リテラルではなく MembershipRegistry から読む
- world は heavy primitive ではなく host/server place sugar として扱う
- server/client DAG 全体を index theory にしない
- static roles / authority は index theory
- dynamic topology / membership / transport は runtime graph
- join process は 0 章では省略してよいが、後続で書く
```

Default:

```text
admin:
  server appoints Alice

turn order:
  Alice -> Bob -> Carol -> Alice

late join:
  history visible only; does not enter turn order automatically

leave:
  active=false, pending action invalidated, turn order skips inactive

owner leave:
  next active participant gets dice_owner, or game pauses

detach:
  TODO Mirrorea lifecycle operation
```

Required view:

```text
hands_on_sugoroku_00_overview.md
hands_on_sugoroku_01_world_bootstrap.md
...
```

---

## 16. Avatar fairy follow vertical slice

次に追加すべき object attachment sample。

要点:

```text
妖精 object が avatar に attach される。
follow target は抽象 FollowAnchor。
remote head と local head は FollowAnchor の候補。
直接 other_avatar.head -> self_avatar.head を same-lineage chain にしない。
```

Correct structure:

```text
anchor slot:
  fairy_follow_anchor

options:
  remote_head_anchor
    resolves_to other_avatar.head
    admit visible(remote_head) >= Shared
    capability observe_transform
    lease linked

  local_head_anchor
    resolves_to self_avatar.head
    admit active(self_head)
    capability observe_transform
    lease live

chain:
  fairy_follow_ref = remote_head_anchor
  fallback local_head_anchor @ lineage(remote_head_anchor -> local_head_anchor)
```

Samples:

```text
01_follow_remote_head_with_local_fallback.mir
02_remote_head_not_visible_falls_back_to_local.mir
03_remote_avatar_leaves_falls_back_to_local.mir
04_invalid_cross_anchor_chain_rejected.mir
05_follow_target_reacquired_after_return.mir
06_model_check_no_detached_anchor_observed.mir
```

This validates:

```text
- abstract role chain, not arbitrary cross-target fallback
- visibility guard
- stale anchor rejection
- fallback behavior
- object attach/follow lifecycle
```

---

## 17. Documentation / Japanese writing requirements

User requires accurate Japanese docs.

Update or create:

```text
.docs/  # if exists
docs/research_abstract/
docs/reports/
README.md
Documentation.md
progress.md
tasks.md
AGENTS.md
specs/00-document-map.md
specs/10-open-questions.md
specs/11-roadmap-and-workstreams.md
plan/01-status-at-a-glance.md
plan/12-open-problems-and-risks.md
plan/13-heavy-future-workstreams.md
plan/16-shared-space-membership-and-example-boundary.md
plan/17-research-phases-and-autonomy-gates.md
plan/90-source-traceability.md
```

Docs must distinguish:

```text
- normative spec
- repository memory
- historical report
- current sample
- historical old sample
- helper-local debug output
- final public API
- deferred mixed gate
```

Hands-on docs must include:

```text
- 目的
- 前提
- complete code
- line-by-line explanation
- built-in vs user-defined words
- command
- pretty output
- JSON output
- what is checked statically
- what remains runtime guard
- what goes to theorem/model-check
- what is deferred
```

---

## 18. AGENTS.md update requirements

AGENTS.md must be updated carefully after reading current contents. Do not overwrite blindly.

Add or refine:

```text
- Source hierarchy
- sub-agent-pro handoff reading rule
- specs/plan/report separation
- no fake validation
- no final-public completion claim without evidence
- old samples must be archived, not silently mixed
- debug/visualization output must be evidence-oriented
- update progress.md and tasks.md after each package
- create docs/reports entry for every non-trivial run
- keep Japanese docs accurate and reader-friendly
- preserve project axis:
  “正しい理論に基づき、正しく hot-plug できる仮想空間システム”
```

Also add anti-shortcut guidance:

```text
Do not:
  - reduce scope silently
  - skip validation and claim success
  - keep old active references unnoticed
  - add builtin primitives for domain predicates
  - collapse authentication into transport
  - treat visualization as untyped debug leak
  - freeze final grammar or public APIs prematurely
```

---

## 19. Reporting standard

Every package must report:

```text
- package name
- objective
- files read
- files changed
- files intentionally not changed
- implementation summary
- validation commands
- exact validation results
- docs updated
- remaining mixed gates
- remaining true user-spec gates
- next step
```

Progress should be written to:

```text
progress.md
tasks.md
docs/reports/<next>-*.md
```

If Discord or external progress reporting is available, use it.  
If unavailable, state so.

---

## 20. No shortcut / anti-goal-drift policy

CodeX must not arbitrarily redefine the goal.

Always check before claiming completion:

```text
1. Did we implement what the user actually asked?
2. Did we update specs/plan/docs/reports?
3. Did we run validation?
4. Did we keep old/new sample boundaries clean?
5. Did we preserve final-public deferred gates?
6. Did we preserve the Mirrorea axis?
7. Did we avoid hidden builtin primitives?
8. Did we report skipped checks honestly?
```

Completion words are controlled.

Allowed:

```text
repo-local alpha-ready
current layer close
helper-local prototype complete
near-end completion
```

Not allowed unless truly achieved:

```text
final language complete
final public verifier complete
production-ready
full system complete
all theory solved
```

---

## 21. Recommended next package order

### Package 1: Current-state audit

```text
Read all sources.
Check sub-agent-pro handoff files.
Find stale docs.
Find old sample references.
Produce report.
```

### Package 2: AGENTS.md and reporting discipline

```text
Update AGENTS.md.
Update progress/tasks/report standard.
```

### Package 3: TermSignature registry

```text
Add signatures to core terms/samples.
Add --debug signatures.
Update docs.
```

### Package 4: LayerSignature system

```text
Design layer composition metadata.
Add examples for auth, theorem, model-check, visualization.
```

### Package 5: MessageEnvelope / Auth seam

```text
Implement envelope carrier.
Support auth none / session-token placeholder.
Preserve typed payload/effects/capabilities.
```

### Package 6: VisualizationProtocol

```text
Add static snapshots and runtime traces.
Add JSON and pretty output.
Add data-flow / witness / membership views.
Respect labels.
```

### Package 7: Sugoroku vertical slice hardening

```text
Fix participant registry / world sugar / join/leave details.
Add signature/debug output.
```

### Package 8: Avatar fairy follow slice

```text
Implement abstract FollowAnchor.
Remote/local head fallback.
Reject invalid arbitrary cross-target fallback.
```

### Package 9: Projection / placement plan

```text
Design source-to-place projection.
Generate server/participant stubs if feasible.
```

### Package 10: Hot-plug Patch / AttachPoint

```text
Design Patch Req Prov Δ.
AttachPoint compatibility.
Activation cut.
```

### Package 11: Hands-on docs

```text
Japanese beginner-friendly docs.
No missing definitions.
Line-by-line examples.
```

### Package 12: Closeout

```text
Validate all.
Update docs/specs/plan/progress/tasks.
Report final current-layer state.
```

---

## 22. Validation expectations

Each package must run relevant validation.

Examples:

```bash
python3 scripts/validate_docs.py
python3 scripts/clean_near_end_samples.py smoke-all
python3 scripts/sugoroku_world_samples.py closeout
python3 scripts/avatar_follow_samples.py closeout
cargo test -p mir-ast
cargo test -p mir-runtime
cargo test -p mir-semantics
```

If commands differ, adapt to repo reality and report.

Add new debug commands:

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug signatures --format json
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format pretty
python3 scripts/sugoroku_world_samples.py run 07_owner_leave_reassign --debug trace --format json

python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug summary --format json
python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug trace --format pretty
```

---

## 23. Final report requirements for this roadmap task

When completing the roadmap incorporation task, CodeX must return:

```text
- handoff files read:
- repository state summary:
- AGENTS.md updated: yes/no
- .docs/docs updated:
- specs updated:
- plan updated:
- progress/tasks updated:
- reports created:
- next package queue:
- validation commands run:
- validation results:
- skipped validations and reasons:
- remaining mixed gates:
- remaining true user-spec gates:
- risks:
- recommended next CodeX prompt:
```

---

# Appendix A: Compact project axis

```text
Mir:
  semantic/type/effect core.

Mirrorea:
  hot-plug virtual-space runtime.

Typed-Effects:
  adapter/effect boundary substrate.

PrismCascade:
  separate high-performance domain kernel, bridged by effects/contracts.

Visualization/IDE:
  essential interface for structure, proof, runtime and performance.

Projection:
  system-level source to place-specific programs.

Network/Auth:
  layered typed adapter, not built into core.

Main goal:
  Correctly typed, verified, visualizable, hot-pluggable virtual space system.
```

---

# Appendix B: What not to lose

Do not lose these user requirements:

```text
- Place generalizes physical process, machine, node, virtual thread, authority box.
- Participants are not the same as Places.
- Standard I/O is not core.
- External world is typed effect adapter / FFI-like boundary.
- Auth can be added later as layer.
- Verification layers can be composed, but must preserve soundness.
- Visualization is mandatory and also typed/permissioned.
- System-wide source should allow later placement/projection changes.
- Runtime hot-plug is central.
- Sugoroku and avatar fairy follow are representative vertical slices.
- No arbitrary new builtin domain predicates.
- User-defined finite index theories define authority/label/etc.
- Final public grammar/API/verifier are deferred.
- Always update docs/plan/tasks/progress and report.
```
