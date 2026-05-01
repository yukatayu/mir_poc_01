# Report 1099 — alpha0 contract variance layer compatibility review

- Date: 2026-05-02 06:20:59 JST
- Author / agent: Codex
- Scope: review of Alpha-0 contract variance / layer compatibility draft centered on `specs/14`, `plan/40`, `samples/alpha/contract-variance/`, and touched snapshot summaries
- Decision levels touched: none; review only, no normative text changed

## Objective

Review the Alpha-0 contract variance / layer compatibility draft for variance mistakes, substitutability violations, hidden shadowing risk, effect/failure row leakage, mutable covariance issues, and missing stop lines around redaction / observation labels.

## Scope and assumptions

- This package is review-only.
- No normative or snapshot document was edited.
- Findings are based on the current draft text and scaffold samples, not on an implemented checker/runtime.
- `samples/alpha/contract-variance/` is treated as planned skeleton evidence only, as stated by its own README.

## Start state / dirty state

- Worktree was already dirty at start. Modified or untracked files pre-existed in `README.md`, `Documentation.md`, `plan/00-index.md`, `plan/01-status-at-a-glance.md`, `plan/11-roadmap-near-term.md`, `plan/19-repository-map-and-taxonomy.md`, `progress.md`, `samples/README.md`, `samples/not_implemented/README.md`, `samples_progress.md`, `scripts/README.md`, `scripts/check_source_hierarchy.py`, `scripts/tests/test_validate_docs.py`, `scripts/validate_docs.py`, `specs/00-document-map.md`, `specs/11-roadmap-and-workstreams.md`, `specs/12-decision-register.md`, `tasks.md`, and the newly added `specs/13..17`, `plan/39..43`, `samples/alpha/`.
- Existing latest report `docs/reports/1098-mirrorea-alpha0-theory-freeze-and-roadmap-sync.md` was incomplete before this task and already breaks `python3 scripts/validate_docs.py`.
- Branch name and prior commit were not material to this review and were not used as correctness evidence.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `plan/00-index.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `progress.md`
- `samples_progress.md`
- `samples/alpha/README.md`
- `samples/alpha/contract-variance/README.md`
- `samples/alpha/contract-variance/*.mir`
- `samples/alpha/contract-variance/*.expected.json`

## Actions taken

- Read the required repository entry documents in the mandated order before examining the target draft.
- Reviewed the normative text in `specs/14` against mirrored repository-memory text in `plan/40` and decision register entry `D-200`.
- Reviewed the scaffold sample matrix and expected sidecars for coverage gaps and contradictions with the spec.
- Ran filesystem/docs integrity checks relevant to this package and recorded the existing unrelated docs-validation blocker.

## Files changed

- `docs/reports/1099-alpha0-contract-variance-layer-compatibility-review.md`

## Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
rg --files -g 'README.md' -g 'Documentation.md' -g 'progress.md' -g 'tasks.md' -g 'samples_progress.md' -g 'specs/*.md' -g 'plan/*.md' -g 'samples/alpha/contract-variance/**'
sed -n '1,260p' README.md
sed -n '1,320p' Documentation.md
rg -n "contract variance|layer compatibility|contract-variance|subtyping|redaction|observation|shadowing|failure row|effect row" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/alpha/README.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md plan/00-index.md plan/40-layer-compatibility-freeze-roadmap.md specs/14-contract-subtyping-layer-compatibility.md samples/alpha/contract-variance/README.md
sed -n '1,140p' specs/01-charter-and-decision-levels.md
sed -n '1,120p' specs/02-system-overview.md
sed -n '1,140p' specs/03-layer-model.md
sed -n '1,120p' specs/09-invariants-and-constraints.md
sed -n '1,220p' plan/00-index.md
rg -n "^#|^##|^###|14-contract|40-layer|39-type|41-save|42-runtime|43-alpha|alpha" specs/00-document-map.md | sed -n '1,220p'
sed -n '144,170p' specs/00-document-map.md
sed -n '480,492p' specs/00-document-map.md
sed -n '1,320p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,180p' plan/40-layer-compatibility-freeze-roadmap.md
sed -n '200,220p' specs/12-decision-register.md
sed -n '1,120p' samples/alpha/contract-variance/README.md
sed -n '1,120p' samples/alpha/README.md
sed -n '40,95p' README.md
sed -n '36,95p' Documentation.md
sed -n '60,110p' progress.md
sed -n '48,96p' samples_progress.md
rg -n "specs/14|contract-variance|layer compatibility alpha line|variance checker|contract / variance|D-200|typed layer" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md specs/00-document-map.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md plan/00-index.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/19-repository-map-and-taxonomy.md
nl -ba specs/14-contract-subtyping-layer-compatibility.md | sed -n '1,260p'
nl -ba plan/40-layer-compatibility-freeze-roadmap.md | sed -n '1,180p'
nl -ba samples/alpha/contract-variance/README.md | sed -n '1,120p'
nl -ba samples_progress.md | sed -n '52,92p'
nl -ba progress.md | sed -n '80,92p'
find samples/alpha/contract-variance -maxdepth 1 -type f | sort
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
date '+%Y-%m-%d %H:%M:%S %Z'
sed -n '1,260p' docs/reports/TEMPLATE.md
sed -n '1,260p' docs/reports/1097-post-u1-readiness-docs-validation-checkpoint.md
sed -n '1,260p' docs/reports/1098-mirrorea-alpha0-theory-freeze-and-roadmap-sync.md
```

## Evidence / outputs / test results

- `find samples/alpha/contract-variance -maxdepth 1 -type f | sort` listed the expected 31 files: package README, 15 `.mir` planned skeletons, and 15 `.expected.json` sidecars.
- `python3 scripts/check_source_hierarchy.py` passed with `required: 50`, `present: 50`, `missing: 0`.
- `python3 scripts/validate_docs.py` failed, but the failure was pre-existing and unrelated to this review target:
  - latest report `1098-mirrorea-alpha0-theory-freeze-and-roadmap-sync.md` has empty required sections
  - empty sections include `Objective`, `Scope and assumptions`, `Documents consulted`, `Evidence / outputs / test results`, `Reviewer findings and follow-up`, and others
- No checker/runtime execution exists yet for `samples/alpha/contract-variance/`; package validation remains scaffold-only by design.

## What changed in understanding

- The draft already captures the intended high-level direction from `D-200`, but its current wording does not yet cleanly separate transparent substitution from explicit contract-update paths.
- The observation/redaction boundary is treated as important policy text, but not yet as a first-class contract comparison row with a checker-visible monotonicity rule.
- The sample scaffold is broader than the spec's own "required sample references" and the roadmap's "must keep visible" subset, which weakens the future conformance floor for key cases.

## Open questions

- Should `redaction_rule` be an explicit contract row, or should it be embedded in a more structured `observation_policy` that also includes label scope and authorized observer classes?
- What is the intended substitutability law for `provided_capabilities` in transparent overlays?
- Is `VAR-01` intended to mean "logging already declared in the base effect/observation rows", or is some other special-case rule intended?

## Suggested next prompt

Patch `specs/14`, `plan/40`, and the `contract-variance` sample README to separate transparent overlays from explicit contract updates, add an explicit redaction/observation comparison row, and make the required sample set cover shadowing, cost, adapter-preservation, and output-covariance cases.

## Plan update status

`plan/` 更新不要: this task reviewed the draft but did not change repository-memory decisions.

## Documentation.md update status

`Documentation.md` 更新不要: no snapshot wording was changed in this review-only task.

## progress.md update status

`progress.md` 更新不要: this task did not change current status, roadmap, or validation reachability.

## tasks.md update status

`tasks.md` 更新不要: no task-map reordering was made in this review-only task.

## samples_progress.md update status

`samples_progress.md` 更新不要: sample dashboard status did not change.

## Reviewer findings and follow-up

- No separate reviewer sub-agent was spawned.
- Local document inspection found three substantive review findings:
  1. `specs/14` mixes transparent overlay admissibility with explicit contract-update + activation-cut flow, which are supposed to be distinct paths.
  2. `specs/14` requires redaction/observation safety, but the contract shape and checker obligations do not encode a redaction/observation comparison row strongly enough to enforce it.
  3. The required sample subset in `specs/14` and the mirrored "must keep visible" subset in `plan/40` omit several critical cases present in the scaffold, weakening future checker conformance around output covariance, cost degradation, adapter preservation, and hidden shadowing.

## Skipped validations and reasons

- No checker/runtime tests were run for `samples/alpha/contract-variance/` because the package is intentionally scaffold-only and current runners do not consume it.
- No Cargo, runtime, network, or E2E validation was run because this task was a document/sample review, not an implementation package.
- The existing `python3 scripts/validate_docs.py` failure was not repaired because it comes from the pre-existing incomplete report `1098` and was outside the requested review scope.

## Commit / push status

Not performed. This review task added a local report only.

## Sub-agent session close status

No sub-agent was spawned for this package.
