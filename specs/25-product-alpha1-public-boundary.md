# 25 — Product Alpha-1 Public Boundary

## role

この文書は、Mirrorea Spaces の
**product/public-ready alpha-1 boundary** を規範として置く。

- ここで定義するのは final public product ではない
- ここで定義するのは bounded practical workflow でも first-floor evidence でもない
- ここで定義するのは、外部開発者が repo を clone し、documented alpha command に従って小さな shared virtual-space product prototype を再現できる product alpha line である

`specs/18` は practical alpha-1 first-floor / bounded workflow の正本であり続ける。
`specs/19..24` は bounded operational alpha readiness の正本であり続ける。
この文書は、それらを product-facing alpha surface へ束ねる追加境界である。

## decision level

- `L1`
  - product alpha-1 readiness は outside developer reproducibility で判断する
  - canonical alpha entrypoint は Python helper ではなく Rust CLI / binary crate へ寄せる
  - `package.mir.json` は versioned alpha package format として扱い、final textual grammar とは分ける
  - product demo は exact-report bundle の合成だけではなく same-session state transition を持つ
  - local save/load、ordinary consistent cut、quiescent save、distributed durable save/load を分ける
  - auth / membership / capability / witness / transport lane を collapse しない
  - native output は native launch bundle であり、direct Mir-to-machine-code codegen ではない
- `L2`
  - alpha CLI command family
  - alpha package schema and compatibility policy
  - product demo sample matrix
  - non-final viewer/devtools bundle shape
  - native launch bundle manifest shape

## one-sentence definition

Mirrorea Spaces product alpha-1 とは、外部開発者が alpha-stable entrypoint から
small shared virtual-space product prototype を check / run / hot-plug / transport /
observe / save / quiescent-save / bundle できる、非 final だが public-ish な alpha toolchain である。

## relationship to existing categories

| Category | Current reading | Product alpha-1 relationship |
|---|---|---|
| current-L2 active floor | runnable evidence | foundation, not product line |
| alpha-0 evidence | current-scope evidence | historical/evidence input only |
| practical alpha-1 first floors | first-floor evidence | source evidence and fixtures, not product-ready |
| bounded operational alpha-0.5 / 0.8 / 0.9 | workflow-ready operational layers | required lower layers |
| bounded practical alpha-1 integrated workflow | reproducible bounded workflow | useful carrier, still not product-ready |
| product alpha-1 | not workflow-ready at this document's creation | target of `P-A1-26..31` |
| final public product | later | non-goal |

## alpha defaults for `U1`

`P-A1-25` closes the previous `U1` ambiguity only for alpha-1 work.
The defaults are:

- shipped surface:
  Rust CLI / binary named `mirrorea-alpha` or implemented by an equivalent crate/package command
- host target:
  native process in controlled local/Docker environment
- package source:
  versioned `package.mir.json` alpha format
- viewer:
  non-final static HTML or local viewer over exported JSON bundles
- native output:
  native host launch bundle containing compiled Rust runtime/CLI, package files, viewer assets, reports, manifest, and run script
- support boundary:
  alpha-stable enough for documented demos, explicitly not final ABI

These defaults do not decide final public grammar, final public ABI, hosted service scope, marketplace, production WAN, or final engine adapter target.

## required product toolchain

The product alpha-1 toolchain path is:

```text
package/source front-door
  -> checker
  -> runtime plan
  -> same-session runtime
  -> typed host-I/O
  -> hot-plug layer/object/avatar-preview package
  -> local/Docker transport
  -> devtools/viewer
  -> local save/load + bounded quiescent-save
  -> native launch bundle
  -> documented reproducible product demo
```

Each step must either be implemented and validated, or explicitly listed as a product alpha-1 limitation.

## canonical alpha entrypoint

Product alpha-1 requires a single developer-facing entrypoint.
The preferred command family is:

```text
mirrorea-alpha check
mirrorea-alpha run-local
mirrorea-alpha session
mirrorea-alpha attach
mirrorea-alpha transport
mirrorea-alpha save
mirrorea-alpha load
mirrorea-alpha quiescent-save
mirrorea-alpha export-devtools
mirrorea-alpha view
mirrorea-alpha build-native-bundle
mirrorea-alpha demo
```

Python scripts may remain orchestration helpers and regression anchors.
They are not the canonical product alpha-1 entrypoint.

The CLI surface is alpha-stable only.
It must not be documented as final public API.

Minimum alpha command semantics:

- `check <package>` validates package schema and finite checker obligations
- `run-local <package>` creates or updates a documented run/session directory for local execution
- `session <package-or-session>` starts, resumes, or inspects the same-session carrier used by product commands
- `attach <session> <package>` applies accepted/rejected/deferred hot-plug behavior against the session carrier
- `transport <package-or-session> --mode local|docker` exercises local or Docker Compose TCP transport with separated auth/membership/capability/witness lanes
- `save <session>` emits an R0 local savepoint
- `load <savepoint>` resumes from an admissible local savepoint
- `quiescent-save <session>` emits or rejects bounded R2 savepoint with quiescence evidence
- `export-devtools <session> --out <dir>` writes product devtools JSON / viewer assets
- `view <devtools-dir>` opens or validates the non-final viewer for exported assets
- `build-native-bundle <package> --out <dir>` emits a native host launch bundle
- `demo --out <dir>` runs the documented product walkthrough and records command outputs

Before a command is implemented, the alpha CLI must return an explicit unsupported / not-yet-implemented diagnostic rather than silently succeeding.
`P-A1-31` release validation must exercise the full implemented command family.

## package / source format

The initial product alpha-1 package format is versioned `package.mir.json`.

Minimum product alpha fields:

```text
schema_version
package_id
package_version
package_kind
dependencies
effects
failures
capabilities
witness_requirements
membership_requirements
auth_policy
auth_stack
contracts
observation_policy
redaction_policy
retention_policy
message_recovery_policy
savepoint_policy
native_policy
compatibility
```

Textual `.mir` final grammar is a non-goal for product alpha-1.
If a textual `.mir` input is provided directly before that surface exists,
the product CLI must return an explicit unsupported / non-goal diagnostic.

## checker requirements

The checker must produce explicit accepted evidence, not only absence of negative diagnostics.

For the product demo finite fragment Line 1 must check or explicitly reject declared finite package facts:

- lifetime / fallback selected cases
- contract variance for layers
- effect row containment
- failure row containment
- capability / witness / membership requirements
- auth policy / auth stack declarations
- package admission
- message recovery policy schema and coverage declarations
- savepoint policy declarations
- observability redaction / retention policy
- native policy admission

Runtime/preflight evidence, model-check evidence, or proof-side residual obligations remain required for stateful recovery and quiescent-save claims:

- message state transitions and recovery outcomes
- `NoInFlight`
- `AllPlacesSealed`
- `NoPostCutSend`
- stale membership / witness / lease non-resurrection
- provenance connectivity
- package-version compatibility
- external-effect compensation or isolation

Residual obligations remain explicit and must identify their intended line:
static checker, model-check second line, proof side line, or kept-later.

## same-session runtime requirements

The product demo must carry state through a single session carrier or documented run directory.

Minimum session state:

- session id
- package identity and version
- checker report ref
- runtime plan ref
- event DAG
- membership frontier
- witness relation summary
- capability decision summary
- auth decision summary
- message/envelope route state
- hot-plug lifecycle state
- local savepoints
- quiescent-save protocol state
- devtools/export refs

Exact-report composition may support reports and viewer fixtures,
but it must not replace same-session runtime transition evidence.

## transport boundary

Product alpha-1 requires:

- local transport demo
- Docker Compose TCP demo
- observer-safe route trace
- separate transport / auth / membership / capability / witness lanes

Production WAN / federation is a non-goal.
Docker/local TCP success must not be described as WAN readiness.

## message failure and savepoint boundary

Product alpha-1 distinguishes:

```text
R0 LocalSavePoint
R1 ConsistentSavePoint
R2 QuiescentSavePoint
R3 DurableQuiescentSavePoint
R4 DistributedDurableReplaySavePoint
```

`R1 ConsistentSavePoint` is a savepoint whose cut is prefix-closed under the
`Consistent(cut)` semantics in `specs/20`.
`R2 QuiescentSavePoint` is `R1` plus quiescence evidence.

Product alpha-1 must implement R0 and bounded R2 for controlled local/Docker scope.
R3 / R4 are non-goals unless a later spec and implementation actually close them.

R2 quiescent save requires runtime/preflight evidence for the R1/load-admissibility subset implemented by the carrier, plus:

- `NoInFlight`
- `AllPlacesSealed`
- `NoPostCutSend`

Message failure / recovery must expose:

- `MessageState`
- `TransportContract`
- `RecoveryPolicy`
- failure class handled by the product demo
- retry / reject / fallback / timeout behavior when implemented

Message and recovery transitions must be represented in the event DAG or in save-object/channel carriers linked from that DAG.
They must not live only in an untyped side table.

Modal markers such as `○` and `□` may be used as operational obligation indices.
They are not an integrated proof language.

## hot-plug and layer stack

The product demo must include visible attach behavior for:

- debug layer
- auth layer
- rate-limit layer
- object package
- avatar-preview or placeholder package

Auth and rate-limit are not presumed transparent overlays.
They must use explicit contract update or a base contract that already declares the added requirements/failure rows.

Accepted detach execution is not required for product alpha-1 if not implemented,
but accepted / rejected / deferred detach semantics must be explicit and observable.

Unsupported runtime fallback must be visible monotone fallback.
It must not be described as native execution success.

## devtools / viewer requirements

Product alpha-1 requires a non-final viewer or static HTML surface that helps an outside developer understand the demo.

Every viewer surface must show:

- active view role, at minimum `observer_safe` or `admin_debug`
- redaction level currently applied
- retention scope currently applied
- admin/debug view status, including explicit `kept_later` when full admin view is not implemented
- source session/export records that back the panel

Minimum panels / sections:

- product overview
  shows package id/version, session id, places, participants, active layers, latest savepoint, and non-claims
- place graph
  shows Place / participant / object / adapter nodes and host/observe/send/attach edges
- event DAG
  shows transition, message, witness, fallback, attach, save/load, and quiescent-save events
- message/envelope route graph
  shows message state, transport contract, auth lane, membership epoch/incarnation, capability requirement, witness refs, and dispatch outcome
- membership/config frontier timeline
  shows membership/config epoch changes and stale reject points
- witness relation timeline
  shows witness creation/use/missing-witness rejection without raw witness payloads
- hot-plug lifecycle
  shows request, compatibility/auth/capability/witness checks, verdict, activation cut, mutation flag, and deferred detach boundary
- save/load and quiescent-save timeline
  shows savepoint class, seal/drain checks, `NoInFlight`, `AllPlacesSealed`, `NoPostCutSend`, and restore result
- message failure/recovery panel
  shows failure kind, message state progression, recovery policy, retry/fallback/reject outcome
- fallback degradation panel
  shows degraded from/to target, reason, and whether fallback was visible
- auth/capability decision panel
  shows auth stack, contract update status, auth decision, required/granted capability summary, capability decision, and failure row as separate lanes
- observer-safe vs admin/debug view status
  shows active role, redaction level, and explicit admin/debug kept-later marker if applicable
- retention/on-demand trace panel
  shows retention scope, retained artifact ids, query selector, and hit/miss result

Observer-safe output must not expose raw witness payloads, raw auth evidence, private capability grants, or secrets.
Derived route / membership / recovery metadata must also respect the active redaction policy and avoid reconstructing hidden secrets from ids or timelines.
Admin/debug full view may remain explicitly kept-later.

## native launch bundle

Product alpha-1 native output means a host launch bundle:

```text
bin/mirrorea-alpha
packages/
devtools/
reports/
manifest.json
run.sh
README.md
```

This is not direct Mir-to-machine-code codegen.
It is not arbitrary native package execution.
Signature/provenance is not semantic safety.

Default `NativeExecutionPolicy` for alpha-1 is `Disabled`.
Any broader native execution mode requires explicit sandbox, effect/failure containment, resource limits, timeout, audit, and revocation story.

## documentation and reproducibility

Product alpha-1 closeout requires:

- clean clone developer guide
- product demo command path
- small package variant guidance
- viewer inspection guidance
- native launch bundle guidance
- research abstract / public alpha explanation
- synchronized `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`
- report with validation evidence

Required docs are expected to include:

- `docs/hands_on/product_alpha1_01.md`
- `docs/research_abstract/product_alpha1_01.md`

These docs are release-candidate closeout requirements, not `P-A1-25` implementation claims.

## validation floor

Every product alpha-1 package must run at least:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Behavior packages must add focused tests for touched crates/scripts.
Release-candidate closeout must run product CLI, viewer, native bundle, and release-check commands documented in `plan/50`.

## package sequence

The promoted product alpha-1 package line is:

1. `P-A1-25` product/public alpha-1 boundary recut
2. `P-A1-26` alpha CLI / package schema stabilization
3. `P-A1-27` product demo same-session runtime
4. `P-A1-28` message failure/recovery + quiescent-save first cut
5. `P-A1-29` product local/Docker transport plus devtools viewer UX
6. `P-A1-30` native launch bundle
7. `P-A1-31` clean-clone product alpha-1 validation / release candidate closeout

## non-goals

- final public parser grammar
- final public ABI
- final public viewer / telemetry service
- production WAN / federation
- distributed durable save/load / R3 / R4
- direct Mir-to-machine-code compiler
- arbitrary native package execution
- marketplace / registry
- full VRM / VRChat / Unity compatibility
- PrismCascade integration
- Reversed Library implementation

## stop line

- Do not call bounded practical workflow product alpha-1.
- Do not call exact report bundles same-session runtime.
- Do not call local save/load distributed durable save/load.
- Do not call R2 quiescent save successful without `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`.
- Do not collapse auth / membership / capability / witness into transport.
- Do not leak raw witness/auth/capability secrets in observer-safe output.
- Do not treat provenance signature as semantic safety.
- Do not freeze final grammar or final ABI from alpha package format.

## success statement

Only after `P-A1-31` validates the full product line may the repo say:

> Mirrorea Spaces product alpha-1 is an alpha, non-final, externally reproducible toolchain for a small shared virtual-space product prototype.

Until then, the correct reading is:

> Product alpha-1 boundary is defined; implementation and release validation remain open.
