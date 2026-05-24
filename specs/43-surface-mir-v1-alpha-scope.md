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
| `P-SURF-02` | indexed-state semantics | `S { state player[p: Participant]: Player }` lowers/checks as S-owned map |
| `P-SURF-03` | Surface-to-Core elaboration | cross-locus read/write generate Core IR edges |
| `P-SURF-04` | auto communication | MessageEnvelope / publish / observe / failure-row obligations generated and visible |
| `P-SURF-05` | role admission | role claim, admission request, membership/capability grant, stale rejection |
| `P-SURF-06` | source patch hot-plug | parse/typecheck/elaborate/admit/activation-cut pipeline |
| `P-SURF-07` | source operational suite | source-first WorldCore / MembershipChat / Sugoroku / related roots |
| `P-SURF-08` | devtools and diagnostics | Surface source, Core IR, generated communication, indexed state, role admission, patch lifecycle visible |
| `P-SURF-99` | final audit | full validation and compatibility anchors rerun |

## planned sample root family

Parser evidence root:

```text
samples/full-system-v1-surface/
  syntax/
  world-core/
  membership-chat/
  sugoroku-world/
  role-admission/
  patch-hotplug/
  posegraph/
  projection/
  provider/
```

`syntax/` is P-SURF-01 parser evidence only. Other families remain planned until
later P-SURF implementation packages actualize them, and the root family must
not be marked workflow-ready runtime evidence from parser rows alone.

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
- `IDX-04`: compaction blocked by savepoint / witness / in-flight reference.

Elaboration / communication:

- `ELAB-01`: cross-place read generates request / observe edge.
- `ELAB-02`: cross-place write generates request.
- `ELAB-03`: private field auto-publish blocked.
- `ELAB-04`: undeclared generated failure rejected.
- `ELAB-05`: generated Core IR has source spans.

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

Current parser-floor Surface anchors:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
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
