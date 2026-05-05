# Closeout protocol

## Required report sections

Each package report must include:

- Objective
- Starting dirty state
- Decisions made
- Files changed
- Commands run
- Validation results
- Reviewer findings
- Documentation update status
- Non-claims
- Remaining blockers
- Commit/push status

## Commands

Minimum:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Behavior package:

```bash
cargo test -p <changed-crate> -- --nocapture
python3 scripts/<affected>.py check-all --format json
```

Product release package:

```bash
python3 scripts/product_alpha1_release_check.py check-all --format json
```

## Git

Use:

```bash
git status --short
git add <changed files>
git commit --no-gpg-sign -m "mirrorea: close <package>"
git push
```

If report metadata must be updated after push, commit and push a second docs-only commit.

## Progress/tasks/samples dashboard

Update:

- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `README.md`
- `Documentation.md`
- relevant `samples/*/README.md`
- relevant `scripts/README.md`
- specs/plan if normative/roadmap changed.

Do not use `100%` except product/operational workflow ready.

## Final user summary

Return:

- package;
- what became product/workflow ready;
- what remains non-goal;
- validation commands;
- report;
- commit hashes;
- clean worktree;
- next reopen point.
