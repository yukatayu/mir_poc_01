# Report 2198 — Product installed-binary generated path portability

- Date: 2026-07-04 15:43 JST
- Author / agent: Codex
- Scope: Product Alpha installed-binary helper generated path serialization
- Decision levels touched: none; implementation / documentation maintenance only

## Objective

Harden `scripts/product_alpha1_installed_binary_check.py` so generated output
paths in the returned helper payload are display-relative, while preserving the
actual external output paths used for command execution.

## Scope and assumptions

- Scope is limited to the installed-binary adoption probe helper and tests.
- Repo-owned binary/package inputs were already passed as repo-relative paths.
- Generated output paths under `--out` should remain absolute for actual
  subprocess execution, because they are external output locations.
- Returned JSON should display paths under `--out` relative to that output root.
- Returned JSON should display repo-owned path text relative to the checkout.
- External absolute paths outside both roots should remain unchanged.
- This is maintenance hardening only. It does not change final CLI/API/ABI,
  packaging, broader distribution, workflow status, or canon status.

## Start state / dirty state

- Start branch: `main`
- Start HEAD: `f777a05ed1a9c4896af21a530dbc4ea761b60e6a`
- Start state: clean and matched `origin/main`.

## Documents consulted

- `AGENTS.md` instructions supplied in the task context
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `docs/reports/TEMPLATE.md`
- `scripts/product_alpha1_release_check.py`
- `scripts/product_alpha1_installed_binary_check.py`
- `scripts/tests/test_product_alpha1_installed_binary_check.py`

## Actions taken

- Added installed-binary release-display helpers:
  `release_relative_path()`, `release_display_text()`, and
  `release_display_value()`.
- Applied recursive display normalization to both preflight error payloads and
  success payloads returned from `check_all()`.
- Kept actual command execution paths unchanged for generated outputs under
  `--out`.
- Added unit tests for generated path serialization, recursive display
  rewriting, external absolute path preservation, command-result argv/stderr
  display, and non-empty output-root preflight display.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.

## Files changed

- `scripts/product_alpha1_installed_binary_check.py`
- `scripts/tests/test_product_alpha1_installed_binary_check.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2198-product-installed-binary-generated-path-portability.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-path-portability
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_product_alpha1_installed_binary_check`:
  9 tests passed.
- Real installed-binary probe:
  - status `accepted`
  - passed commands `11`
  - failed commands `[]`
  - `installed_binary_candidate_ready=True`
  - `out_dir="."`
  - `session_dir="session-store"`
  - `native_bundle_dir="native-bundle"`
  - `demo_dir="demo"`
  - `/home/` matches `0`
  - `/Users/` matches `0`
  - repo absolute matches `0`

## What changed in understanding

The remaining installed-binary issue was not execution-time portability for
repo-owned inputs; that had already been fixed. The remaining issue was output
payload portability: generated `--out` paths and command-result argv/stderr
were reporting host-specific absolute paths even though the command execution
itself needs those absolute external paths.

## Open questions

- No semantic or user-choice question blocks the next maintenance package.
- Remaining broader path-portability candidates are Full System V1 nested
  source argv, alpha network Docker success/failure path serialization,
  current-L2 pipeline / detached-loop repo-owned helper argv, source-hierarchy
  status JSON, and shared practical failure-path redaction.

## Suggested next prompt

Continue the broader path-portability sweep with Full System V1 nested source
argv or alpha network Docker path serialization.

## Plan update status

`plan/` 更新不要: no long-term repository memory or normative interpretation
changed.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader snapshot is unchanged at this
granularity.

## progress.md update status

`progress.md` 更新済み: added the 2026-07-04 15:43 JST installed-binary
generated path serialization log.

## tasks.md update status

`tasks.md` 更新済み: updated the installed-binary path-portability reading and
removed installed-binary generated paths from the remaining broader candidate
list.

## samples_progress.md update status

`samples_progress.md` 更新済み: updated the installed-binary adoption-probe row
and recent validation log.

## Reviewer findings and follow-up

No separate sub-agent was used for this narrow package. The change follows the
same release-display pattern used by Product Alpha release-check and Surface
release-check.

## Skipped validations and reasons

- Product Alpha release-check was not rerun because this package only changes
  installed-binary returned-payload display; the real installed-binary probe was
  rerun.
- Broader path-portability candidates were not fixed in this package to keep
  scope narrow.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No active sub-agent remained open for this package.
