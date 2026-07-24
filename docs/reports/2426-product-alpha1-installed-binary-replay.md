# Report 2426 - Product Alpha installed-binary replay

## Title and identifier

Report 2426 - Product Alpha installed-binary replay.

## Objective

Exercise the existing installed-binary Product Alpha workflow, inspect its
actual generated evidence, and record its exact scope without promoting a
bounded alpha run into a final product or theory claim.

## Scope and assumptions

Canon remains normative. This is an operational LAB replay of existing commands
at `f90c2c29`, not an implementation or semantic change. The helper's aggregate
terminal JSON is unavailable, so only directly inspected observed reports and
separately observed commands are treated as evidence.

## Start state / dirty state

The worktree began clean at `f90c2c29`, equal to `origin/main`. Before running
the heavy workflow, the host had about 52 GiB free disk, about 7.9 GiB available
memory, and about 6.2 GiB swap free. The repository used about 5.9 GiB, of
which `target/` used about 5.9 GiB.

## Documents consulted

Canon entry points and current snapshots; `Makefile`; `scripts/README.md`;
`samples/README.md`; the installed-binary helper; Product Alpha reports; Plan
166/167; the Product Alpha/operational non-claim wording in current docs; and
the completed temporary Oracle review `product-alpha-replay-review`.

## Actions taken

1. Allowed one original installed-binary command session to proceed to a fresh
   `/tmp` output directory, including its existing Docker-enabled demo path;
   stopped and excluded two later duplicate monitoring launches.
2. Inspected observed demo, native-bundle, and Docker transport JSON reports in
   the retained output directory without individually attributing them to a
   launcher.
3. Ran `cargo check` and `cargo fmt --check` directly.
4. Recorded the evidence boundary in Plan 194 and synchronized the concise
   LAB status/task views.

## Files changed

- `plan/194-product-alpha1-installed-binary-replay-evidence.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2426-product-alpha1-installed-binary-replay.md`

No runtime, sample, Canon, schema, or generated artifact is committed.

## Commands run

`df -h .`; `free -h`; `du -sh . target .git`; `cargo check`; `cargo fmt
--check`; `python3 scripts/product_alpha1_installed_binary_check.py --format
json check-all --out /tmp/mirrorea-alpha1-installed-binary-check-20260724`; JSON
inspection with `jq`; and the final documentation/source-hierarchy/format/whitespace
checks recorded below. The temporary Oracle review used
`ask-chatgpt-pro-temp` with the Plan/report/current LAB status files attached.

## Evidence / outputs / test results

The generated `demo.json` is accepted and marks the Product Alpha demo,
same-session reopen, and attach matrix as complete within its bounded report.
The native-bundle verification report is accepted while explicitly denying
arbitrary native package execution and signature-as-safety. The Docker report
records an executed Docker Compose TCP wire roundtrip with accepted participant
and world outcomes. Direct `cargo check` and `cargo fmt --check` passed.

The outer helper console detached before its final aggregate JSON/exit status
could be captured. Therefore this report does not assert aggregate helper
acceptance, exit code zero, or completion of every `check-all` stage. One
original command session was allowed to proceed; two accidental duplicate
monitoring launches were stopped and excluded. The JSON is treated only as
output observed in the retained directory, not as individually attributed to a
launcher. No repository source was modified by any run. The generated `/tmp`
output is 123 MiB and was left intact pending explicit cleanup confirmation.

After the Oracle wording corrections, `python3 scripts/validate_docs.py`
passed (1,580 numbered reports), `python3 scripts/check_source_hierarchy.py`
passed (744/744 required paths), `cargo fmt --check`, `cargo check`, and
`git diff --check` passed. `python3 -m unittest -v
scripts.tests.test_validate_docs` also passed all 87 tests in 1151.034 seconds.

## What changed in understanding

The existing Product Alpha line has direct host evidence for the documented
native bundle and two-process Docker Compose TCP path. Its own generated
non-claims remain decisive: this is runnable bounded alpha evidence, not a
public distributed runtime, final ABI, or conformance result.

## Open questions

No new theory question was opened. A future release-specific replay must capture
the helper's aggregate result if it needs to make an aggregate release-check
claim.

## Suggested next prompt

Continue autonomous research only from a separately eligible theory or
implementation package; treat Plan 194 as a replay record, not as a new
promotion path.

## Plan update status

更新済み: Plan 194 records the exact generated evidence and non-claims.

## Documentation.md update status

更新済み: the entry guide now distinguishes the actual bounded replay from a
final product claim.

## docs/project-status.md update status

更新済み: the concise implementation/operation row records the replay boundary.

## progress.md update status

更新済み: the Product Alpha line and dated log record direct evidence without an
aggregate helper claim.

## tasks.md update status

更新済み: the current task map records the closed replay as evidence maintenance.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample, validation command, debug surface,
or sample evidence classification changed.

## Reviewer findings and follow-up

A temporary GPT-5.6 Sol Oracle review found no semantic or Canon-boundary
overclaim, but required two wording corrections: distinguish the original
evidence session from two excluded duplicate monitoring launches, and make the
missing aggregate exit/stage status explicit. Those corrections are applied.
Final local validation is recorded in the evidence section.

## Skipped validations and reasons

No Lean/model-check command was run because this package changes neither a
formal artifact nor a proof-facing statement. The installed-binary helper was
not rerun after documentation-only changes; its long original run already
produced the inspected evidence, and the post-edit checks cover the modified
documentation and Python inventories.

## Commit / push status

Pending final validation, commit with `--no-gpg-sign`, and immediate push.

## Sub-agent session close status

No independently controllable sub-agent session was exposed in this workspace.
The temporary Oracle session `product-alpha-replay-review` completed and its
findings were incorporated; its raw transcript remains external advisory
evidence, not repository state.
