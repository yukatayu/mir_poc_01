# 14 — validation and commands

## baseline docs/theory validation

For P-A1-18, run:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## optional current floor smoke checks

If time allows, run representative smoke commands to ensure wording matches reality:

```bash
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py closeout --format json
```

Do not claim these as fresh full validation unless you run them.

## runtime touch rule

If Rust code is not touched, do not run full cargo suite unnecessarily. Still run `cargo fmt --check`.

If Rust code is touched, run affected tests.

## report number selection

Find latest numbered report:

```bash
find docs/reports -maxdepth 1 -type f -name '[0-9][0-9][0-9][0-9]-*.md' | sort | tail -n 5
```

Use the next number.

## commit / push

```bash
git status --short
git add <changed files>
git commit --no-gpg-sign -m "mirrorea: freeze operational alpha theory boundaries"
git push
```

If push fails due to remote updates, fetch/rebase carefully and rerun affected validation before pushing.

## final response format

Return:

- package name
- added specs / plan docs
- validation passed
- report path
- commit hash
- push status
- worktree clean?
- next reopen point

