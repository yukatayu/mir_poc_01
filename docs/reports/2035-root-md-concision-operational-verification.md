# 2035 — root Markdown concision and operational workflow verification

## Objective

ルート直下の tracked Markdown が冗長な implementation / evidence inventory を重複して持っていないか確認し、現在の実装が具体的に動作することを再検証する。併せて、「もう実用できる段階か」を workflow readiness と product/public readiness に分けて明確化する。

## Scope and assumptions

- Scope is tracked root Markdown plus validation of existing operational workflow carriers.
- Runtime implementation is not widened in this task.
- `diff_investigation_01.md` and `diff_investigation_02.md` exist at repo root but are ignored / untracked local historical notes. They were inspected but not moved or deleted to avoid changing untracked user/local files.
- "実用できる" is interpreted as "an external developer can reproduce and use the named repo-local operational workflow from repository commands." It does not imply final public product readiness.

## Start state / dirty state

- start state:
  latest pushed HEAD was `bb623c4` from `P-A1-24`.
- dirty state at start:
  clean tracked worktree.
- root ignored notes:
  `diff_investigation_01.md` and `diff_investigation_02.md` were present as ignored files.
- resource check:
  not repeated; this task did not add heavy build artifacts or generated corpora.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2034-p-a1-24-workflow-readiness-metric-recut.md`
- `diff_investigation_01.md`
- `diff_investigation_02.md`

## Actions taken

1. Confirmed current location and repository hierarchy from `/home/yukatayu/dev/mir_poc_01`.
2. Audited root Markdown line counts and searched tracked root docs for stale percentage / completion wording.
3. Compressed a long `README.md` practical alpha-1 evidence inventory into concise pointers to canonical sample docs, dashboard, and repository memory.
4. Updated `AGENTS.md` so future agents use workflow readiness / evidence classification instead of progress percentage as the primary status metric.
5. Updated `Documentation.md`, `tasks.md`, `progress.md`, and `samples_progress.md` to keep snapshot and dashboard wording synchronized.
6. Re-ran operational workflow commands for α-0.5, α-0.8, α-0.9, and bounded practical α-1.
7. Re-ran focused Python and Rust behavior tests for the operational carriers.
8. Ran docs/source-hierarchy/formatting validations.

## Files changed

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2035-root-md-concision-operational-verification.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '145,230p' AGENTS.md
sed -n '1,120p' tasks.md
sed -n '1,70p' Documentation.md
git status --short
git diff -- README.md AGENTS.md tasks.md Documentation.md samples_progress.md progress.md
pwd
find . -maxdepth 2 -type d | sort | sed 's#^./##' | head -80
git status --short --untracked-files=all
git status --short --ignored -- diff_investigation_01.md diff_investigation_02.md
wc -l AGENTS.md Documentation.md README.md progress.md samples_progress.md tasks.md
rg -n 'limited|2026-05-05 時点では、|first practical avatar|VIS-A1-07' README.md Documentation.md progress.md tasks.md samples_progress.md AGENTS.md
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha08_session_hotplug.py check-all --format json
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
python3 scripts/practical_alpha1_integrated_workflow.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow scripts.tests.test_practical_alpha09_devtools scripts.tests.test_practical_alpha08_session_hotplug scripts.tests.test_practical_alpha05_session
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture
cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture
cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture
cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture
date '+%Y-%m-%d %H:%M %Z'
git diff --stat
git diff -- AGENTS.md Documentation.md README.md progress.md samples_progress.md tasks.md
```

## Evidence / outputs / test results

- `pwd` returned `/home/yukatayu/dev/mir_poc_01`.
- root tracked Markdown line count after concision:
  `AGENTS.md` 271, `Documentation.md` 126, `README.md` 293, `progress.md` 136, `samples_progress.md` 96, `tasks.md` 78.
- ignored local notes:
  `git status --short --ignored -- diff_investigation_01.md diff_investigation_02.md` returned both as `!!`.
- α-0.5 check:
  `sample_count = 7`, all `OA05-01..07` passed, `operational_alpha05_ready = true`.
- α-0.8 check:
  `sample_count = 10`, all `OA08-01..10` passed, `operational_alpha08_ready = true`.
- α-0.9 check:
  `sample_count = 9`, all `OA09-01..09` passed, `operational_alpha09_ready = true`.
- bounded practical α-1 workflow check:
  `sample_count = 8`, all `PA1W-01..08` passed, `bounded_practical_alpha1_workflow_ready = true`, `product_public_ready = false`.
- bounded practical α-1 closeout:
  repeated the same `PA1W-01..08` pass set and preserved stop lines for final public alpha-1, public viewer/telemetry ABI, distributed durable save/load, and production WAN/federation.
- focused Python unittest:
  `Ran 14 tests in 11.115s`, `OK`.
- docs unittest:
  `Ran 11 tests in 0.062s`, `OK`.
- source hierarchy:
  `required: 84`, `present: 84`, `missing: 0`.
- docs scaffold:
  `Documentation scaffold looks complete. Found 1187 numbered report(s).`
- `cargo fmt --check` passed.
- `git diff --check` passed.
- `cargo test -p mir-runtime --test practical_alpha05_session -- --nocapture` passed; `3` tests passed.
- `cargo test -p mir-runtime --test practical_alpha05_host_io -- --nocapture` passed; `2` tests passed.
- `cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture` passed; `3` tests passed.
- `cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture` passed; `3` tests passed.

## What changed in understanding

- The repo-local α-0.5 / α-0.8 / α-0.9 operational workflows are reproducible from repository commands.
- The bounded practical α-1 developer workflow is also reproducible, but its own output explicitly says `product_public_ready = false`.
- Therefore "実用できる" is correct only for bounded repo-local developer / operational workflows, not for final public product readiness.
- Root tracked docs should point to canonical detailed documents instead of repeating every evidence row.

## Open questions

- Whether the ignored root `diff_investigation_*.md` files should be moved into `docs/reports/` or archived by explicit user instruction.
- Whether the next promoted package should be `P-A1-25` product/public boundary recut or a user-facing `U1` decision checklist.

## Suggested next prompt

`P-A1-25` として、bounded repo-local operational workflow と final public/product-ready alpha-1 の差を public API / CLI / viewer / telemetry ABI / packaging / host target / support boundary の観点から recut してください。

## Plan update status

- `plan/ 更新不要`.
- Reason:
  this task did not change long-term roadmap semantics; it verified existing workflow readiness and tightened root snapshot wording.

## Documentation.md update status

- updated:
  root document map now refers to `progress.md` as workflow / evidence snapshot rather than progress snapshot.

## progress.md update status

- updated:
  appended a recent log entry for the root Markdown audit and concrete behavior validation rerun.

## tasks.md update status

- updated:
  stale percentage stop-line wording was replaced with "do not call conceptual-only rows workflow-ready."

## samples_progress.md update status

- updated:
  added a validation log entry for the α-0.5 / α-0.8 / α-0.9 / bounded practical α-1 workflow rerun and preserved `product_public_ready = false`.

## Reviewer findings and follow-up

- sub-agent reviewers were not spawned because the current user request did not explicitly ask for sub-agents and active tool policy only permits spawning when explicitly requested.
- local focused review findings:
  root tracked Markdown now avoids duplicating the full practical alpha-1 evidence inventory in `README.md`.
- local focused review findings:
  `AGENTS.md` had stale percentage-oriented rules; those are now recut to workflow readiness / evidence classification.
- follow-up:
  decide what to do with ignored root `diff_investigation_*.md` files if root cleanliness should include ignored local files.

## Skipped validations and reasons

- Full workspace `cargo test`:
  skipped to avoid broad unrelated runtime cost; focused Rust behavior tests for affected operational carriers were run and passed.
- Heavy storage audit:
  skipped because this task did not add heavy artifacts, external workdirs, or generated corpora.

## Commit / push status

- package commit:
  `7468d2c` `docs: verify operational workflow readiness`
- push:
  pushed to `origin/main` after report metadata follow-up.

## Sub-agent session close status

- no sub-agents were opened for this package.
