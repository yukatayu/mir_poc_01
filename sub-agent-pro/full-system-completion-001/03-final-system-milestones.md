# 03 — Final System Milestones

## Milestone model

Codex must replace progress/task percent thinking with milestone gates.

Each milestone must define:

- source/input
- checker
- runtime/interpreter/compiler behavior
- devtools/observability
- negative samples
- validation commands
- reports
- explicit non-goals

## FULL-V1 milestone sequence

### FS-00 Documentation Rebaseline

Goal:

- Replace `progress.md` and `tasks.md` completely.
- Create clear all-system roadmap from current state to Full System V1.
- Ensure current claims are classified precisely.

Required docs:

- `progress.md` new complete roadmap snapshot
- `tasks.md` new current task map
- `specs/33-full-system-v1-scope.md`
- `plan/58-full-system-v1-roadmap.md`
- `docs/hands_on/full_system_v1_roadmap_01.md`

### FS-01 Textual Mir Grammar MVP

Goal:

- Create alpha textual Mir source entrypoint.
- It does not need final grammar, but must parse computational and effectful baseline samples.

Must support:

- module/import
- fn
- let/mut
- int/bool/text
- record
- fixed array/vector
- if/else
- for/while minimal
- perform boundary calls
- require/ensure

Samples:

- `samples/full-system-v1/mir-source/comp/add_one.mir`
- `variables_scope.mir`
- `arrays_bounds.mir`
- `records_vec3.mir`
- `control_flow.mir`
- `imports_functions.mir`

### FS-02 Typed IR and Checker

Goal:

- Lower textual Mir to typed IR.
- Reuse finite checker line.
- Keep failure row explicit.

Must check:

- types
- variable scope
- array bounds static where possible
- effect row containment
- failure row containment
- capability requirements
- import resolution
- contract require/ensure shape

### FS-03 Mir-owned Computational Interpreter

Goal:

- Execute typed IR for C-like safe subset.
- Host read/write are boundary effects only.

Must execute:

- AddOne in Mir
- arrays positive/negative
- records/Vec3
- control-flow
- imports/functions

Must expose:

- compute trace
- observer-safe trace
- rejection cause

### FS-04 Effectful Mir Integration

Goal:

- Connect computational core to Mir effects.

Must support:

- perform read/write boundary
- publish/observe
- witness create/use
- handoff
- fallback/guarded ref limited cases
- atomic_cut local semantics

Samples:

- computational AddOne via read/write
- Sugoroku roll/publish/witness/handoff in textual Mir or typed IR
- membership chat in textual Mir or typed IR

### FS-05 PoseGraph Runtime

Goal:

- Make Transform/PoseGraph semantics runtime-visible.

Must support:

- Transform type
- PoseVersion
- Anchor
- AnchorBinding
- no-split-frame invariant
- stale-anchor rejection/reacquire
- fallback anchor
- observer-safe devtools panel

### FS-06 Projection IR

Goal:

- Define system-wide source to server/client/adapter artifacts.

Must produce:

- target manifest
- Place-to-target mapping
- packet boundary schema
- FFI boundary schema
- adapter boundary schema
- projection correctness report

No LLVM required yet.

### FS-07 Server/Client Runtime Split MVP

Goal:

- Use projection manifest to run server and headless/browser-like client artifacts in local/Docker controlled environment.

Must run:

- WorldCore server-only
- MembershipChat server/client
- Sugoroku server/client
- Portal transition
- TwoShard hard boundary

### FS-08 Engine/FFI/WASM Provider Admission MVP

Goal:

- Provide typed provider boundary without arbitrary execution.

Must support:

- provider manifest check
- effect/failure/capability/resource rows
- native execution disabled by default
- WASM provider inventory not execution unless sandbox implemented
- renderer backend as non-semantic-owner

### FS-09 Devtools Full Alpha Panels

Goal:

- Make source -> checker -> runtime -> projection -> transport -> PoseGraph -> save/load visible.

Must show:

- source/import graph
- typed IR summary
- Place graph
- projection graph
- server/client routes
- event DAG
- witness timeline
- membership/config frontier
- PoseGraph panel
- save/load panel
- hot-plug panel
- provider boundary panel

### FS-10 Native Host Bundle + Optional Backend Gate

Goal:

- Create runnable bundle for full V1 sample.

Must include:

- compiled `mirrorea-alpha`
- Mir sources
- typed IR artifacts
- projection manifest
- server/client launch scripts
- devtools bundle
- verification reports

Optional backend:

- If LLVM/WASM backend is attempted, keep it separate and require boundary preservation tests.

### FS-11 Release Check and Clean Clone Guide

Goal:

- New developer can clone and reproduce full V1 sample.

Must include:

- hands-on guide
- release-check script
- installed-binary check
- Docker check
- generated viewer
- explicit non-goals

## Completion rule

A milestone is not complete unless:

- positive sample passes
- negative sample fails for the expected reason
- devtools or report explains result
- docs are updated
- validation commands are included
- report is written
- commit and push are done
