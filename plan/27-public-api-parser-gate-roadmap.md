# plan/27 — public API / parser grammar gate roadmap

## 目的

`P18` public API / parser grammar gate の
**repo-side first-cut closeout memory** を置く。

- ここで固定するのは final public contract そのものではない
- ここで固定するのは、repo が先に揃えてよい
  freeze checklist / public-boundary inventory / mixed-gate と true user-spec hold line の分離である
- prior helper-local preview / report-local inventory / current crate-local carrier を
  final public ABI や final parser grammar と誤読させないための stop line を
  current repo memory に残す

## current role of `P18`

`P18` は **last mixed gate** である。

- repo-side で先に閉じるもの:
  - public-facing seam の inventory
  - preview / prototype / committed generated bridge evidence の qualifier
  - final freeze 前提条件の checklist
  - post-`P18` true user-spec hold line の切り分け
- post-`P18` に残すもの:
  - actual final parser grammar
  - actual final public parser / checker / runtime / verifier API
  - actual final public adapter / viewer / hot-plug / transport / projection ABI
  - packaging / installed binary / FFI / engine adapter / final shared-space operational catalog

## already decided before final freeze

### syntax / parser boundary

- current `.mir` surface は companion notation であり、final parser grammar ではない
- parser-free PoC stack と current crate-local parser/checker cut は non-production であり、
  final public parser API ではない
- `mir_hilight.html` は readable viewer であり、final parser / checker / verifier ではない
- public freeze は parser だけでなく、checker / runtime / verifier / viewer / adapter /
  projection / hot-plug / transport public surface を含む mixed gate として読む

### verification boundary

- current `VerificationLayer` reading は typed explanation / evidence carrier である
- helper `verification_handoff_witness` と runtime `verification_model_check` は
  current emitted floor だが、final public verifier contract ではない
- current repo は emitted row / evidence carrier / downstream consumer /
  emitted verifier handoff artifact の split inventory を先に整理してよいが、
  それ自体を final public verifier contract と読まない
- `R1` current line では theorem bridge / runtime policy / visualization-telemetry を
  active emitted row に上げず、widening threshold inventory として別管理する
- hidden verifier builtin を既成事実化しない

## repo-side freeze candidates

### 1. auth / message seam inventory

current repo は次を boundary wording として先に固定してよい。

- `message_envelope_scope`
- `transport_medium` と `transport_seam` の split
- `emitter_principal`
- `freshness_checks`
- shared `AuthEvidence` lane inventory
- legacy `transport` は compatibility-only alias であり、
  current invariant は `transport == transport_seam`

これは final public auth schema や transport ABI ではない。

### 2. visualization / viewer / telemetry inventory

current repo は次を boundary wording として先に固定してよい。

- `label`
- `authority`
- `redaction`
- `retention_scope`
- `source_refs`
- fail-closed observer route trace
- `typed public prototype inventory over helper/runtime surfaces; not a final public viewer API`

これは final public viewer API、final public visualization schema、
final telemetry backend を意味しない。

### 3. typed external / host-boundary inventory

current repo は次を boundary wording として先に固定してよい。

- `host_boundary_scope`
- `host_boundary_lanes = request / receipt / failure / visualization`
- `non_collapse_lanes = transport / auth / membership / capability / witness / visualization`
- `host_family_gates`
- residual reopen matrix for `EXT-01` / `EXT-02` / `EXT-05`

これは final public adapter API、exact host schema、
browser / network / VR family final split を意味しない。

### 4. projection / emitted-program inventory

current repo は次を boundary wording として先に固定してよい。

- system-wide source と place-specific program の distinction
- authority / participant / adapter / visualizer split
- validity checklist
- validity-report minimum
- committed generated bridge evidence only
- generated reserve policy

これは final projection IR、optimizer、equivalence checker、
actual emitted executable family を意味しない。

### 5. hot-plug inventory

current repo は次を boundary wording として先に固定してよい。

- compatibility checklist
- activation cut
- explicit detach boundary / migration stop line
- helper-local package-manager inventory
- runtime-private `HotPlugRequest` / `HotPlugVerdict` carrier anchor
- runtime-private `HotPlugRuntimeEngineState` / `HotPlugRuntimeEngineReport` engine-state anchor
- public request/response/event naming と `AttachPoint` / `Patch` package catalog naming の candidate inventory

ここで fixed するのは
`freeze prerequisite fixed; public ABI still unfrozen`
という bridge line までである。

これは final hot-plug ABI、rollback protocol、
durable migration engine を意味しない。

### 6. network transport inventory

current repo は次を boundary wording として先に固定してよい。

- widening invariant
- `NET-01..05` ladder
- process-boundary closeout inventory
- transport does not mint auth / membership / capability / witness
- canary evidence is evidence only であり public transport ABI ではない

これは final socket / broker choice、session / signature protocol、
federation / consensus contract を意味しない。

### 7. shared-space/runtime subset inventory

current repo は次を boundary wording として先に固定してよい。

- `Place != participant`
- `world` は host-side sugar
- `MembershipRegistry` is source of truth
- `PlaceCatalog` と thin runtime shell が current substrate
- exhaustive shared-space final catalog は kept-later

これは final shared-space operational catalog を意味しない。

### 8. toolchain adjacency inventory

current repo は次を public-freeze adjacency として先に固定してよい。

- external workdir
- `CARGO_TARGET_DIR`
- `CARGO_HOME`
- non-destructive cleanup guard
- `llvm` owner / writable visibility

これは packaging semantics や actual backend freeze を意味しない。

## kept-later gates inside the mixed gate

- final parser grammar
- final public parser / checker / runtime / verifier API
- final public `AuthEvidence` shape
- final public `witness_refs` role taxonomy
- final public visualization / telemetry schema
- final public viewer API
- final public adapter API / exact host schema
- final public transport ABI
- final projection IR / emitted executable family / deployment planner
- final hot-plug ABI / rollback / durable migration engine
- real network / durable replay / consensus
- actual LLVM build / backend choice / production packaging policy

## post-`P18` true user-spec hold line

repo-side framing の後でも、次は user choice が要る。

- packaging / installed binary
- FFI / engine adapter / host integration target
- broader application target
- exhaustive shared-space operational catalog

repository-memory recommendation は、
repo-local helper / runtime / report-local evidence を維持したまま
この hold line を explicit option inventory として扱うことである。

## validation floor

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
bash scripts/env/mirrorea_storage_env.sh
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
CARGO_HOME=/mnt/mirrorea-work/cargo-registry-cache cargo test -p mir-ast --no-run
git diff --check
```

`network_transport_samples.py check-all` is the executable canary anchor for
`NET-02..05`; `closeout` is inventory evidence. `projection_codegen_samples.py
check-all` is live anchor / manifest alignment validation; `closeout` is
committed manifest inventory evidence. Neither command freezes a public
transport ABI or emitted-program ABI.

The storage commands are guardrail evidence for external workdir routing,
non-destructive detach audit, explicit-confirmation cleanup policy, and
external cargo cache usability. They are not actual LLVM build, backend choice,
or packaging adoption evidence.

## stop line

- helper-local preview を final public implementation と呼ばない
- report-local inventory を final public contract と呼ばない
- current crate-local carrier を final public ABI と呼ばない
- `P18` closeout を final parser grammar freeze と呼ばない
- `P18` closeout を final public completion と呼ばない

## related memory

- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `plan/26-visual-debugger-viewer-roadmap.md`
- `plan/38-post-p21-final-public-hotplug-abi-family.md`
