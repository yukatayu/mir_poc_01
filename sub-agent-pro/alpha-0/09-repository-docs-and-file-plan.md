# 09 — Repository, docs, and file plan

This file describes how Codex should integrate the package without collapsing repository hierarchy.

## 1. Existing hierarchy to preserve

- `specs/`: normative source of truth.
- `plan/`: long-lived repository memory.
- `Documentation.md`: concise current snapshot.
- `progress.md`: current progress snapshot.
- `tasks.md`: current task map and blockers.
- `samples_progress.md`: runnable sample dashboard.
- `samples/README.md`: sample taxonomy.
- `scripts/README.md`: runner/helper taxonomy.
- `docs/hands_on/`: command-oriented reader-facing guides.
- `docs/research_abstract/`: reader-facing summaries/details.
- `docs/reports/`: execution evidence and change history, not normative.
- `sub-agent-pro/`: handoff/working directives, not normative.

## 2. New specs to create

### `specs/13-type-system-lifetime-fallback.md`

Normative alpha-local spec for:

- guarded access paths
- raw ref vs guarded ref vs inherited chain vs snapshot selected
- lease / lifetime guard
- fallback canonicalization
- underdeclared fallback static error
- no re-promotion
- read/write variance
- remote observed reference model
- proof obligations
- sample references
- deferred syntax decisions

### `specs/14-contract-subtyping-layer-compatibility.md`

Normative alpha-local spec for:

- contract shape
- input contravariance / output covariance
- precondition weakening / postcondition strengthening
- effect row containment
- failure row containment
- capability monotonicity
- cost/latency contracts
- observation label / redaction / retention
- layer kinds and attach points
- auth/rate-limit caveats
- debug layer authority
- proof obligations

### `specs/15-cut-save-load-checkpoint.md`

Normative alpha-local spec for:

- `atomic_cut` place-local boundary
- event DAG / causal order
- consistent cut predicate
- save state contents
- load/rollback constraints
- in-flight messages
- Z-cycle/useless checkpoint boundary
- `durable_cut` deferred status
- external effect compensation/isolation
- proof obligations

### `specs/16-runtime-package-adapter-hotplug.md`

Normative alpha-local spec for:

- runtime package manifests
- avatar/runtime adapter boundary
- non-core treatment of VRChat/VRM/Unity/custom avatar formats
- native binary trust policy
- package hot-plug flow
- unsupported runtime fallback
- required samples
- deferred shader/asset/native details

### `specs/17-mirrorea-spaces-alpha-scope.md`

Normative alpha-local scope spec for:

- Mirrorea Spaces alpha definition
- VRChat relationship and browser-like interpretation
- completion conditions
- non-goals
- relationship to Reversed Library and PrismCascade
- stage/phase mapping
- user-decision blockers

## 3. New plan files to create

### `plan/39-type-system-freeze-roadmap.md`

Repository memory for type system freeze:

- current status
- decisions
- proof obligations
- checker roadmap
- sample matrix link
- deferred topics

### `plan/40-layer-compatibility-freeze-roadmap.md`

Repository memory for layer compatibility:

- overlay/layer rules
- variance roadmap
- layer insertion phases
- devtools/telemetry boundary

### `plan/41-save-load-checkpoint-roadmap.md`

Repository memory for save/load and checkpoint:

- atomic cut retained boundary
- local save/load first
- consistent cut checker
- Z-cycle samples
- durable cut deferred

### `plan/42-runtime-package-avatar-roadmap.md`

Repository memory for runtime package/avatar:

- avatar non-core decision
- runtime package manifest roadmap
- native trust policy
- VRM/VRChat/Unity adapter skeletons

### `plan/43-alpha-e2e-roadmap.md`

Repository memory for alpha E2E:

- Stage A..F mapping
- Docker E2E sequence
- sample validation matrix
- Mirrorea Spaces alpha completion condition

## 4. Sample directories to create

```text
samples/alpha/
  README.md
  lifetime-fallback/
  contract-variance/
  cut-save-load/
  local-runtime/
  layer-insertion/
  network-docker/
  hotplug-runtime/
  avatar-runtime/
  visualization/
  e2e/
```

Each subdir should eventually contain:

- `.mir` or source-ish sample
- `.expected.json` sidecar
- README explaining current status

Use planned markers if not runnable yet. Do not mark planned skeletons as active executable samples.

## 5. Scripts to create later

Do not create empty fake runners unless they validate real files or planned status honestly.

When ready:

```text
scripts/alpha_lifetime_fallback_samples.py
scripts/alpha_contract_variance_samples.py
scripts/alpha_cut_save_load_samples.py
scripts/alpha_hotplug_runtime_samples.py
scripts/alpha_network_docker_samples.py
scripts/alpha_avatar_runtime_samples.py
scripts/alpha_visualization_samples.py
scripts/alpha_e2e_samples.py
```

Each runner should support at least:

```bash
list --format json
check-all --format json
closeout --format json
```

If samples are skeleton-only, output must say planned/skeleton, not pass as implemented behavior.

## 6. Crate additions to consider

Add only when implementation starts, not during docs-only freeze unless necessary.

```text
crates/mir-ir
crates/mir-checker
crates/mirrorea-runtime
crates/mirrorea-transport
crates/mirrorea-devtools
crates/mirrorea-packages
crates/mirrorea-avatar
```

Potential responsibilities:

- `mir-ir`: typed core IR with regions, leases, effects, contracts.
- `mir-checker`: lifetime/fallback/variance/effect-row checker.
- `mirrorea-runtime`: Place runtime, queue, dispatch, hot-plug lifecycle.
- `mirrorea-transport`: in-process/TCP/Docker transport implementations.
- `mirrorea-devtools`: event DAG export, redaction, viewer schemas.
- `mirrorea-packages`: runtime package manifests and admission checks.
- `mirrorea-avatar`: abstract avatar roles and adapter skeletons.

## 7. Documentation updates

### `Documentation.md`

Add concise note:

- Alpha-0 theory freeze package exists.
- New specs 13..17 define alpha-local freeze boundaries.
- Mirrorea Spaces alpha is the next product target.
- Reversed Library and PrismCascade remain out of alpha implementation scope.

### `progress.md`

Must show:

- current large stage
- current concrete phase
- validation freshness
- new package status
- current blockers
- next autonomous package

### `tasks.md`

Must add:

- theory freeze docs package
- sample skeleton package
- checker skeleton package
- local runtime package
- layer insertion package
- network Docker package
- save/load package
- runtime package/avatar package

Separate user-decision blockers from research-discovery items.

### `samples_progress.md`

Add rows for alpha sample families. Do not overstate progress.

### `samples/README.md`

Add `samples/alpha/` taxonomy. Make clear active current suite remains `samples/clean-near-end/` until alpha samples become runnable.

### `scripts/README.md`

Add future runner taxonomy only when scripts exist. Do not claim commands exist before implementation.

## 8. Reports

Every non-trivial task must create a new report under `docs/reports/` using the current template. Include update statuses for:

- `plan/`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## 9. Avoid stale claims

After edits, search for and correct claims that:

- final parser grammar is fixed
- production transport exists
- production auth exists
- distributed save/load is implemented
- avatar compatibility is core
- PrismCascade is in alpha scope
- Reversed Library is implemented
- generated projection manifests are final emitted programs
- helper-local previews are final public APIs
