# 07 — Repository Structure Plan

## 1. Existing structure to preserve

- `specs/`: normative source
- `plan/`: repository memory
- `docs/reports/`: task evidence
- `progress.md`, `tasks.md`, `samples_progress.md`: snapshots / dashboards
- `samples/clean-near-end/`: active runnable floor
- `samples/current-l2/`: base corpus
- `samples/lean/`: mechanization evidence
- `samples/alpha/`: mixed alpha-local scaffold / non-public floors
- `samples/not_implemented/`: planned residual family
- `samples/generated/`: generated bridge evidence

## 2. Add practical alpha-1 docs

Recommended:

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/hands_on/practical_alpha1_01.md`
- `docs/research_abstract/practical_alpha1_01.md`

## 3. Add practical sample root

Do not repurpose `samples/alpha/` as active root immediately.

Recommended new root:

```text
samples/practical-alpha1/
  README.md
  packages/
    sugoroku-space/
    layer-debug/
    avatar-placeholder/
  source/
  expected/
  docker/
```

This root can become the active practical alpha sample root only after parser/checker/runtime front-door exists.

## 4. Keep `samples/alpha/` as evidence root

`samples/alpha/` remains current-scope evidence root. It should not be silently promoted.

## 5. Add scripts carefully

Potential scripts:

- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_run_local.py`
- `scripts/practical_alpha1_run_docker.py`
- `scripts/practical_alpha1_closeout.py`

If implementing in Rust CLI later, scripts may wrap cargo examples temporarily.

## 6. Reports

Every package must create a new numbered report. Do not overwrite reports.

## 7. Source hierarchy validation

If new directories are added, update:

- `scripts/check_source_hierarchy.py`
- tests if any
- `samples/README.md`
- `scripts/README.md`
- `Documentation.md`
- `samples_progress.md`
