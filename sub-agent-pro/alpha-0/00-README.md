# Alpha-0 package README

This directory is a handoff package for the next Codex work tranche. It consolidates the user discussion and the assistant's decisions into a form that can be integrated into the repository.

The package is intentionally long. `prompt.md` at repository root is the short operational prompt. The files here are the detailed source of truth for the Alpha-0 planning tranche.

## Package purpose

The goal is to prepare Mir / Mirrorea for a real Mirrorea Spaces alpha. The immediate work is not a final public product. It is a theory-freeze and roadmap synchronization package covering:

1. Lifetime / fallback / guarded reference typing.
2. Contract subtyping, layer insertion, and variance.
3. `atomic_cut`, save/load, consistent cuts, rollback, and Z-cycle boundaries.
4. Runtime package / avatar adapter / trusted native boundary.
5. Mirrorea Spaces alpha scope and staging.
6. Sample coverage and validation strategy.
7. Repository structure and documentation synchronization.
8. Sub-agent review workflow.

## Required reading order inside this package

1. `01-canonical-decisions.md`
2. `02-layer-and-product-boundaries.md`
3. `03-stage-roadmap-and-phase-plan.md`
4. `04-theory-lifetime-fallback.md`
5. `05-theory-contract-layer-variance.md`
6. `06-theory-cut-save-load-zcycle.md`
7. `07-runtime-package-avatar-adapter.md`
8. `08-sample-matrix.md`
9. `09-repository-docs-and-file-plan.md`
10. `10-sub-agent-operating-model.md`
11. `11-validation-commit-push-protocol.md`
12. `12-codex-task-packages.md`
13. `13-progress-dashboard-model.md`
14. `14-first-spec-drafts-outline.md`
15. `15-glossary-and-stop-lines.md`

## Core interpretation

Mirrorea Spaces alpha is not a VRChat clone. VRChat-class functionality is a long-term functional lower bound, but the platform is intended to be a browser-like virtual world substrate: worlds, runtime packages, avatar adapters, effects, permissions, debug layers, and hot-plug modules should be typed, inspectable, contract-aware, and safely composable.

Mirrorea itself is not the Reversed Library. Mirrorea is the fabric/runtime. Reversed Library / 裏返した図書館 is a flagship upper-layer knowledge-space application built on top of Mirrorea Spaces / Mirrorea Atlas.

PrismCascade is a separate media-processing kernel candidate. It may later connect through typed external effects or adapters, but it is not in Mirrorea alpha implementation scope.

## Immediate repository deliverables

The first Codex package should add or update:

- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/...` sample skeletons and expected verdict sidecars
- synchronized `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, and `scripts/README.md`
- one new numbered report under `docs/reports/`

## Validation floor for the first package

At minimum:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

If sample runners or checker skeletons are added, add and run the relevant tests. Do not claim unrun validation as passing.
