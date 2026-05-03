# 09 — Validation Plan

## 1. Mandatory validation for every package

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Also run relevant focused tests.

## 2. Practical alpha-1 validation floor

A full practical alpha-1 closeout should eventually run:

```bash
# source / parser / checker
python3 scripts/practical_alpha1_check.py samples/practical-alpha1/packages/sugoroku-space --format json

# local runtime
python3 scripts/practical_alpha1_run_local.py samples/practical-alpha1/packages/sugoroku-space --format json

# Docker/runtime transport
python3 scripts/practical_alpha1_run_docker.py samples/practical-alpha1/packages/sugoroku-space --format json

# devtools
python3 scripts/practical_alpha1_export_devtools.py <run-id> --format json

# existing regression floors
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/current_l2_lean_sample_sync.py
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json

# alpha evidence floors
python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json
python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json
python3 scripts/alpha_hotplug_lifecycle_samples.py stage-d-closeout --format json
python3 scripts/alpha_visualization_samples.py stage-e-closeout --format json
python3 scripts/alpha_e2e_samples.py stage-f-closeout --format json
```

The practical alpha-1 scripts may not exist at P-A1-00. Add them as stages progress.

## 3. Docker validation

If Docker is unavailable, do not claim Docker E2E success. Record skipped reason.

## 4. Storage validation

For heavy builds:

```bash
df -h .
free -h
bash scripts/env/mirrorea_storage_env.sh --ensure-dirs
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
```

Do not create heavy artifacts in repo root.

## 5. Report requirements

Every non-trivial task requires a report under `docs/reports/`. Include:

- objective
- scope / assumptions
- dirty state
- docs consulted
- actions
- files changed
- commands run
- evidence
- changed understanding
- open questions
- next prompt
- plan / Documentation / progress / tasks / samples_progress update status
- reviewer findings
- skipped validations
- commit / push status
- sub-agent close status
