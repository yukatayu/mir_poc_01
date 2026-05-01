# 11 — Validation, report, commit, and push protocol

This project treats documentation and validation as part of correctness. Every non-trivial package must close with evidence.

## 1. Baseline validation for docs-only theory freeze

Run at minimum:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

If docs mention sample paths or scripts, check that the paths/scripts exist or are explicitly marked planned.

## 2. Baseline validation when samples are added

If sample skeletons are added:

```bash
find samples/alpha -maxdepth 3 -type f | sort
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

If sample runners are added, also run their `list`, `check-all`, and `closeout` commands.

## 3. Baseline validation when Rust crates are added or changed

Run focused tests first:

```bash
cargo fmt --check
cargo test -p <changed-crate>
git diff --check
```

If workspace-level changes are significant, also run:

```bash
cargo test -p mir-ast
cargo test -p mir-semantics
cargo test -p mir-runtime
cargo test -p mirrorea-core
```

Use repository storage policy. Do not create heavy build artifacts in root if external workdir policy applies.

## 4. Baseline validation when Docker E2E is added

Run:

```bash
docker compose -f <compose-file> config
python3 scripts/alpha_network_docker_samples.py check-all --format json
```

If Docker is unavailable, report it explicitly as skipped validation with reason. Do not claim pass.

## 5. Reports

Every non-trivial package must create a new numbered report under `docs/reports/`. Use the current repository report template. Include every required section in order.

The report must include:

- objective
- scope and assumptions
- start dirty state
- documents consulted
- actions taken
- files changed
- commands run
- evidence / outputs / test results
- what changed in understanding
- open questions
- suggested next prompt
- `plan/` update status
- `Documentation.md` update status
- `progress.md` update status
- `tasks.md` update status
- `samples_progress.md` update status
- reviewer findings and follow-up
- skipped validations and reasons
- commit / push status
- sub-agent session close status

## 6. Progress and task updates

After every package:

- `progress.md` must show current stage and phase.
- `tasks.md` must show next autonomous package and user-decision blockers.
- `samples_progress.md` must update rows whose sample/runnable status changed.

Do not use `progress.md` as an exhaustive append-only ledger. Keep it a current snapshot with a short recent log.

## 7. Git protocol

Before commit:

```bash
git status --short
git diff --check
git diff --stat
```

Commit:

```bash
git add <files>
git commit --no-gpg-sign -m "<concise message>"
git push
```

If push fails, report exact reason and leave commit status explicit.

## 8. What not to claim

Never claim:

- validation passed if skipped
- final parser grammar if only companion notation exists
- production auth/transport/viewer/verifier if helper-local only
- distributed save/load if only local save or checker exists
- runtime package safety from signature alone
- avatar compatibility beyond skeletons
- Reversed Library implementation during Mirrorea alpha
- PrismCascade integration unless actually implemented

## 9. Storage / build safety

For long-running or build-heavy tasks, follow repository storage policy:

- check disk/memory if heavy work
- use external workdir if configured
- do not create large artifacts under repo root
- cleanup only explicit disposable paths
- never delete source or reports without explicit reason

## 10. Validation freshness language

Use concrete timestamps when updating snapshot docs. Do not infer timestamps manually; use system date command in the actual repo environment.
