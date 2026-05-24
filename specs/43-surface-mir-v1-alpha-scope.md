# 43 — Surface Mir V1 Alpha Scope

## role

This document defines the docs-first Surface Mir V1 alpha scope that follows the
closed Full System V1 bounded release-check line.

Surface Mir makes `.mir` source files the semantic source authority and gives
users a brace-based place-scope syntax for system-wide meaning. Core Mir remains
the explicit elaboration target for checker, runtime, projection, and devtools.

## decision level

- `L1`
  - canonical Surface Mir place-scope syntax is `S { ... }`.
  - `S[ ... ]` is not adopted, including as sugar.
  - `.mir` source files are semantic source authority.
  - `package.mir.json` remains alpha compatibility / generated package
    artifact.
  - Surface Mir is user-facing; Core Mir is elaboration target.
  - communication, publish, observe, and source patch lifecycle must not be
    hidden from Core IR / devtools.
- `L2`
  - P-SURF package sequence and sample matrix.
  - planned `samples/full-system-v1-surface/` root family.
  - alpha command family and validation floor.

## current baseline

Current repo evidence before this Surface line:

- Product Alpha-1 release candidate is ready in bounded local/Docker alpha
  scope.
- Operational product suite is workflow-ready as a bounded narrow showcase.
- Full System V1 parser / typed checker / bounded effectful runtime /
  operational source suite / PoseGraph runtime / projection IR / local role
  split / provider admission / renderer pose / release-check line is closed
  through final audit.
- Full System V1 still uses Core-facing syntax and does not yet provide the
  canonical Surface Mir brace authoring layer.

## Surface Mir alpha target

The target authoring flow is:

```text
developer writes `.mir` source with `S { ... }`
  -> check-source
  -> parse-source
  -> elaborate-source to Core Mir
  -> run source-derived session
  -> patch-source for dynamic hot-plug
  -> export Core IR / communication / indexed state / patch lifecycle devtools
```

This is near product-style alpha flow, not final production.

## initial package sequence

| Package | Role | Close condition |
|---|---|---|
| `P-SURF-00B` | brace syntax / source-authority docs rebaseline | specs/plans/snapshot docs/guides/report updated; validators pass |
| `P-SURF-01` | Surface brace parser | closed: `S { ... }` place blocks and `Role[instance] { ... }` role-instance blocks parse; bare role blocks and `S[ ... ]` reject with diagnostic |
| `P-SURF-02` | indexed-state semantics | closed: `S { state player[p: Participant]: Player }` checks as S-owned map; key authority, stale key, retained-savepoint compaction, and nested-place ambient authority negatives reject |
| `P-SURF-03` | Surface-to-Core elaboration | closed: cross-locus read/write generate Core IR remote request rows, generated edges, source spans, obligations, and underdeclared failure-row rejection |
| `P-SURF-04` | auto communication | closed: MessageEnvelope / visible publish / observe / `VisibilityDenied` failure-row obligations generated and visible; private/non-visible field auto communication rejected |
| `P-SURF-05` | role admission | closed: role claim, admission request/verdict, capability grant, witness, stale rejection, hash metadata evidence |
| `P-SURF-06` | source patch hot-plug | closed: parse/typecheck/elaborate/compatibility/admission, HotPlugRequest, HotPlugVerdict, Core IR diff, activation_cut, no-direct-eval, rejection-without-mutation rows |
| `P-SURF-07` | source operational suite | closed: source-first WorldCore / MembershipChat / Sugoroku / PortalWorldlink / TwoShardHardBoundary / GradientObservation roots with positive/negative rows |
| `P-SURF-08` | devtools and diagnostics | closed: Surface source, Core IR, generated communication, semantic-checker-backed indexed-state map, role admission, redacted patch lifecycle, and source spans visible in static diagnostics |
| `P-SURF-99` | final audit | full validation and compatibility anchors rerun |

## planned sample root family

Parser evidence root:

```text
samples/full-system-v1-surface/
  syntax/
  indexed-state/
  elaboration/
  world-core/
  membership-chat/
  sugoroku-world/
  portal-worldlink/
  two-shard-hard-boundary/
  gradient-observation/
  role-admission/
  source-patch/
  devtools/
  posegraph/
  projection/
  provider/
```

`syntax/` is P-SURF-01 parser evidence only. `indexed-state/` is P-SURF-02
semantic checker evidence only. `elaboration/` is P-SURF-03/P-SURF-04
elaboration and generated communication evidence only. `role-admission/` is
P-SURF-05 report-level admission/grant evidence only, not production identity,
hardware attestation, WAN admission, or runtime membership lifecycle
completion. `source-patch/` is P-SURF-06 source patch pipeline evidence only,
not a final hot-plug ABI, distributed durable migration planner, production
patch registry, or arbitrary native/WASM execution route. The six operational
roots are P-SURF-07 source-first evidence only, not final runtime/transport or
final shared-space catalog completion. `devtools/` is P-SURF-08 static
diagnostics evidence only, not final viewer/telemetry ABI or runtime devtools
completion. Other families remain planned until
later P-SURF implementation packages actualize them, and the root family must
not be marked workflow-ready runtime evidence from parser, checker,
elaboration, generated communication, role-admission, source-patch, or
source-operational/static-devtools rows alone.

## required sample matrix

Surface syntax:

- `SURF-01`: `S { ... }` place block accepted.
- `SURF-02`: `S[ ... ]` rejected.
- `SURF-03`: record literal accepted.
- `SURF-04`: ambiguous brace construct rejected.
- `SURF-05`: role instance block accepted.
- `SURF-06`: undeclared place block head rejected.
- `SURF-07`: undeclared role-instance head rejected.
- `SURF-08`: invalid role-instance binder rejected.
- `SURF-09`: role named `S` remains a role-instance head under namespace
  resolution.

Indexed state:

- `IDX-01`: S-owned Participant-indexed state accepted.
- `IDX-02`: key write without authority rejected.
- `IDX-03`: stale key access rejected.
- `IDX-04`: compaction blocked by retained savepoint evidence; witness /
  in-flight reference blockers remain lifecycle obligations for later runtime
  carriers.
- `IDX-05`: nested place block ambient-authority bypass rejected.

Elaboration / communication:

- `ELAB-01`: cross-place read generates request / observe edge.
- `ELAB-02`: cross-place write generates request.
- `ELAB-03`: private field auto-publish blocked.
- `ELAB-04`: undeclared generated failure rejected.
- `ELAB-05`: generated Core IR has source spans.
- `ELAB-06`: unsupported statements rejected rather than silently dropped.
- `ELAB-07`: generated write request with underdeclared failure row rejected.
- `ELAB-08`: nested place read generates owner-directed request evidence.

Role admission:

- `ROLE-01`: BrowserClient join accepted through admission.
- `ROLE-02`: role claim without grant cannot write server state.
- `ROLE-03`: stale membership message rejected.
- `ROLE-04`: package/runtime hash binding optional report.

Patch hot-plug:

- `PATCH-01`: source patch adds visible state.
- `PATCH-02`: undeclared failure rejected.
- `PATCH-03`: self-grant of server authority rejected.
- `PATCH-04`: lifecycle devtools export accepted.

## validation floor

Every package in this line must run at least:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Compatibility anchors when environment permits:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

Current parser / indexed-state checker / elaboration / generated communication
Surface anchors:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

## non-claims

This document does not claim:

- final public grammar / ABI / SDK.
- Rust-level language completion.
- LLVM/native codegen.
- production WAN/federation.
- distributed durable save-load R3/R4.
- arbitrary native/WASM execution.
- Unity / Unreal / renderer semantic ownership.
- created runnable Surface Mir sample roots in this docs-only package.
