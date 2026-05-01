# 12 — Codex autonomous task packages

This file decomposes the roadmap into autonomous packages. Codex should close each package with docs sync, validation, report, commit, and push, then continue to the next package unless a true blocker appears.

## Package P-A0-00 — Read and install Alpha-0 package

Objective:

- Read this package.
- Confirm repository hierarchy.
- Add package files to repo if not already present.

Actions:

- Read root docs and package files.
- Do not make normative changes yet except copying handoff if needed.
- Create report only if repository state changes.

Completion:

- Understanding recorded.
- Next package ready.

## Package P-A0-01 — Theory freeze specs

Objective:

- Add normative alpha-local specs `13..17`.

Files:

- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`

Use package files 04..07 and 02..03.

Sub-agents:

- type-system reviewer
- contract-variance reviewer
- cut/checkpoint reviewer
- runtime-package/avatar reviewer
- product-scope guardian

Validation:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Completion:

- specs created and validated
- report created
- commit/push

## Package P-A0-02 — Plan roadmap memory

Objective:

- Add repository memory plan files `39..43`.

Files:

- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- update `plan/00-index.md`

Validation:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Completion:

- plan index updated
- report / commit / push

## Package P-A0-03 — Alpha sample skeleton matrix

Objective:

- Add `samples/alpha/` root and sample skeletons/expected sidecars.

Files:

- `samples/alpha/README.md`
- subdirectories from `09-repository-docs-and-file-plan.md`
- initial `.mir` skeletons and `.expected.json` sidecars for LIF/VAR/CUT/HP/NET/AV/VIS/E2E rows

Do not pretend skeletons are runnable.

Validation:

```bash
find samples/alpha -maxdepth 3 -type f | sort
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Completion:

- sample matrix represented in files
- samples_progress updated honestly
- report / commit / push

## Package P-A0-04 — Snapshot docs sync

Objective:

- Synchronize reader-facing and snapshot docs.

Files:

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- optional `docs/hands_on/alpha_*.md`
- optional `docs/research_abstract/alpha_*.md`

Validation:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Completion:

- progress shows Stage/Phase current status
- tasks shows next autonomous package
- report / commit / push

## Package P-A0-05 — Checker skeleton first cut

Objective:

- Add typed IR/checker skeleton for the first decidable subset.

Possible files:

- `crates/mir-ir/`
- `crates/mir-checker/`
- tests for LIF and VAR samples

Initial implemented checks:

- raw dangling reference reject
- fallback evidence required
- capability promotion reject
- mutable invariance
- read-only covariance valid
- precondition strengthening reject
- postcondition weakening reject
- effect/failure row widening reject

Validation:

```bash
cargo fmt --check
cargo test -p mir-checker
python3 scripts/alpha_lifetime_fallback_samples.py check-all --format json  # if runner exists
python3 scripts/alpha_contract_variance_samples.py check-all --format json  # if runner exists
git diff --check
```

Completion:

- checker emits diagnostics for key samples
- report / commit / push

## Package P-A0-06 — Cut/save/load checker skeleton

Objective:

- Add consistent cut predicate and local save/load skeleton.

Initial implemented checks:

- rollback across `atomic_cut` rejected
- orphan receive invalid
- orphan observe invalid
- orphan witness use invalid
- hot-plug activation closure
- expired lease not resurrected
- Z-cycle sample marked invalid or planned if algorithm not implemented

Completion:

- CUT samples have checker output or explicit planned status
- report / commit / push

## Package P-A0-07 — Local Mirrorea runtime integration

Objective:

- Move minimal local runtime behavior into Rust runtime rather than Python-only helper.

Deliverables:

- Place runtime
- queues
- MessageEnvelope dispatch
- membership freshness
- local Sugoroku sample
- event DAG export

Completion:

- local integrated Sugoroku E2E passes
- report / commit / push

## Package P-A0-08 — Layer insertion runtime

Objective:

- Implement initial debug layer and contract-checked layer insertion skeleton.

Deliverables:

- LayerKind
- AttachPoint
- LayerContract
- debug layer attach
- auth/rate-limit planned/limited samples
- redaction sample

Completion:

- debug layer attaches only with authority
- trace appears only after attach
- report / commit / push

## Package P-A0-09 — Docker network E2E

Objective:

- Run minimal world/participant over Docker network.

Deliverables:

- transport trait
- TCP/subprocess JSON implementation
- Docker Compose file
- E2E runner
- stale membership negative test

Completion:

- Docker E2E passes or Docker unavailability reported honestly
- report / commit / push

## Package P-A0-10 — Runtime package/avatar skeleton

Objective:

- Add runtime package manifest and placeholder/custom avatar runtime skeleton.

Deliverables:

- manifest schema
- package admission checker
- placeholder avatar
- custom Mir avatar skeleton
- VRM/VRChat/Unity adapter skeletons as planned/limited
- native package reject samples

Completion:

- core has no avatar-format primitive
- runtime package samples validate
- report / commit / push

## Package P-A0-11 — Mirrorea Spaces alpha demo closeout

Objective:

- Integrate minimal local/network world, hot-plug, debug, avatar placeholder, and save/load status into one alpha demo.

Completion:

- demo command exists
- docs explain stop lines
- validation floor updated
- report / commit / push
