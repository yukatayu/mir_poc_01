# Report 1081 — guided helper retirement audit

- Date: 2026-05-01 11:42 JST
- Author / agent: Codex
- Scope: docs freshness / guided helper retirement audit
- Decision levels touched: none; repository-memory wording only

## Objective

Recheck `scripts/current_l2_guided_samples.py` active command surface and cool remaining repository-memory wording that could read pre-clean-near-end prototypes, representative bundles, or reserve summaries as the current active runner floor.

## Scope and assumptions

- Scope is docs-only: `plan/00`, `plan/10`, current snapshots, sample dashboard, and this report.
- The live compatibility front door remains `list / smoke-all / closeout`.
- Retired `bundle`, `matrix`, `emit-*`, `reserve`, `lane`, `reopen-map`, `residuals`, and `split` guided-helper memories remain historical / archived / repository-memory references unless a current runner explicitly supports them.
- Stop line: this package does not change sample semantics, promote legacy helper commands, freeze a final parser/API, or claim final public current-L2 completion.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/19-repository-map-and-taxonomy.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/clean_near_end_samples.py`
- `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
- `docs/reports/1080-validate-docs-scope-and-network-anchor-audit.md`

## Actions taken

- Spawned a docs researcher to audit active reader-facing and repository-memory docs for stale `current_l2_guided_samples.py` wording.
- Spawned a code mapper to verify the wrapper command surface and validation paths.
- Confirmed active reader-facing docs already point to `smoke-all` / `closeout` and do not expose retired guided-helper commands as active commands.
- Confirmed `plan/00` and `plan/10` still had stale current-active wording around pre-clean-near-end prototype floors, representative bundles, and reserve summaries.
- Updated `plan/00` to separate `samples/current-l2/` as the active base corpus and `samples/clean-near-end/` as the active canonical executable suite, while demoting `p01...p16` to historical comparison memory.
- Updated `plan/10` Macro 4 wording from fixed authored/prototype floor to current-L2 base corpus and clean near-end executable floor.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` with this audit checkpoint.
- Added this report.

## Files changed

- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1081-guided-helper-retirement-audit.md`

## Commands run

```bash
git status --short
git branch --show-current
git log -1 --oneline
sed -n '1,220p' tasks.md
sed -n '1,190p' progress.md
sed -n '1,220p' plan/00-index.md
rg -n "current_l2_guided_samples\\.py|guided samples|guided sample|smoke-all|closeout --format json|bundle|lane|reserve|hold-line|emit-|reopen-map|residuals" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md scripts/README.md docs/hands_on docs/research_abstract plan/00-index.md plan/07-parser-free-poc-stack.md plan/08-representative-programs-and-fixtures.md plan/09-helper-stack-and-responsibility-map.md plan/10-roadmap-overall.md plan/11-roadmap-near-term.md plan/18-type-proof-modelcheck-and-ordering-research-program.md plan/19-repository-map-and-taxonomy.md
sed -n '1,260p' scripts/current_l2_guided_samples.py
rg -n "current_l2_guided_samples\\.py|smoke-all|closeout" scripts crates tests samples docs/reports/TEMPLATE.md Makefile .github 2>/dev/null
rg -n "current_l2_guided_samples\\.py (bundle|matrix|smoke |emit-|emit|reserve|lane|reopen-map|residuals|split|mapping)" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/current-l2 samples/clean-near-end scripts/README.md docs/hands_on docs/research_abstract plan/00-index.md plan/07-parser-free-poc-stack.md plan/08-representative-programs-and-fixtures.md plan/09-helper-stack-and-responsibility-map.md plan/10-roadmap-overall.md plan/11-roadmap-near-term.md plan/18-type-proof-modelcheck-and-ordering-research-program.md plan/19-repository-map-and-taxonomy.md --glob '!docs/research_abstract/old/**'
rg -n "current_l2_guided_samples\\.py" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/current-l2 samples/clean-near-end scripts/README.md docs/hands_on docs/research_abstract plan/00-index.md plan/07-parser-free-poc-stack.md plan/08-representative-programs-and-fixtures.md plan/09-helper-stack-and-responsibility-map.md plan/10-roadmap-overall.md plan/11-roadmap-near-term.md plan/18-type-proof-modelcheck-and-ordering-research-program.md plan/19-repository-map-and-taxonomy.md --glob '!docs/research_abstract/old/**'
python3 scripts/current_l2_guided_samples.py list
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/current_l2_guided_samples.py bundle problem1
date '+%Y-%m-%d %H:%M %Z'
sed -n '18,52p' plan/00-index.md
sed -n '1,110p' plan/10-roadmap-overall.md
sed -n '1,220p' docs/reports/TEMPLATE.md
python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_clean_near_end_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
rg -n "authored sixteen|corrected prototype set|representative bundle|reserve summary index|active on fixed authored/prototype floor|fixed authored/prototype floor" plan/00-index.md plan/10-roadmap-overall.md README.md Documentation.md progress.md tasks.md samples_progress.md docs/hands_on docs/research_abstract scripts/README.md samples/README.md --glob '!docs/research_abstract/old/**'
```

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git branch --show-current
main

$ git log -1 --oneline
c93108b Align network validation docs anchors
```

Command-surface evidence:

```text
$ python3 scripts/current_l2_guided_samples.py list
exit 0; listed 16 clean-near-end samples across typing, order-handoff, model-check, and modal families.

$ python3 scripts/current_l2_guided_samples.py smoke-all --format json
exit 0; JSON included list, typing, order_handoff, model_check, modal, matrix, and closeout payloads.

$ python3 scripts/current_l2_guided_samples.py closeout --format json
exit 0; closeout payload reported active_sample_root = samples/clean-near-end and archive_sample_root = samples/old/2026-04-22-pre-clean-near-end.

$ python3 scripts/current_l2_guided_samples.py bundle problem1
exit 2; printed the clean-near-end migration note and supported compatibility commands: list, smoke-all, closeout.
```

Search evidence:

```text
$ rg -n "current_l2_guided_samples\\.py (bundle|matrix|smoke |emit-|emit|reserve|lane|reopen-map|residuals|split|mapping)" ... --glob '!docs/research_abstract/old/**'
<no matches>
```

Sub-agent evidence:

```text
docs researcher: active stale wording was limited to plan/00 and plan/10. README, Documentation, progress, tasks, samples_progress, samples/README, scripts/README, current hands-on, and current research abstract were aligned.

code mapper: wrapper hard-gates supported first arguments to list / smoke-all / closeout. Underlying clean_near_end_samples.py and mir-clean-near-end still have their own broader active surfaces, but those are not exposed through current_l2_guided_samples.py.
```

Post-report validation:

```text
$ python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_clean_near_end_samples
Ran 4 tests in 0.001s
OK

$ python3 scripts/check_source_hierarchy.py
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 35
  present: 35
  missing: 0
  all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1079 numbered report(s).

$ git diff --check
<no output>
```

Post-patch stale-wording search:

```text
No remaining active-current claim says fixed authored/prototype floor.
Remaining hits in plan/00, plan/10, and progress.md explicitly classify pre-clean-near-end prototypes / representative bundle / reserve summary as historical comparison memory.
```

## What changed in understanding

The active reader-facing docs were already aligned with the retired guided-helper command surface, but two long-lived plan entries still used old prototype/bundle wording with current-active temperature. The correct current reading is: `samples/current-l2/` is the active base corpus; active canonical execution goes through `samples/clean-near-end/` and the guided compatibility wrapper's `list / smoke-all / closeout`; pre-clean-near-end prototypes and bundle/reserve memories remain historical comparison material.

## Open questions

- Actual `U1` commitment remains open and user-facing.
- Final public parser/API remains unfrozen.
- Historical prototype material can remain useful for comparison, but it must not be re-promoted as current active runner evidence without implementation support.

## Suggested next prompt

Continue autonomous maintenance: run focused docs validation, review the diff, commit/push, and then choose the next safe validation or docs freshness package.

## Plan update status

`plan/` 更新済み: `plan/00` and `plan/10` were cooled from pre-clean-near-end active wording to current-L2 base corpus / clean near-end executable floor wording.

## progress.md update status

`progress.md` 更新済み: recent log records the guided-helper retirement audit and plan wording cooldown.

## tasks.md update status

`tasks.md` 更新済み: current task-level status records the rechecked front door and plan cooldown.

## samples_progress.md update status

`samples_progress.md` 更新済み: PH0 and recent validation rows record this audit and its focused validation results.

## Skipped validations and reasons

- Full sample/Cargo floor was skipped because this package changed only docs and repository-memory wording.
- Focused docs validation passed after this report was added.
- Wrapper-focused validation passed: `list`, `smoke-all`, and `closeout` exited 0; retired `bundle problem1` returned exit 2 as expected; wrapper/unit helper tests passed 4/4.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

Sub-agents spawned:

- `Huygens` (`docs_researcher`): completed; identified narrow stale plan wording and acceptable historical references.
- `Nash` (`code_mapper`): completed; mapped wrapper command surface and validation commands.
- `Tesla` (`reviewer`): completed; found a taxonomy overclaim that called `samples/current-l2/` part of the active runnable floor. Follow-up patch split `samples/current-l2/` as base corpus from `samples/clean-near-end/` as active canonical executable suite.

All three sub-agent sessions were closed after review and follow-up validation.
