# Report 1082 — network sample README anchor audit

- Date: 2026-05-01 11:53 JST
- Author / agent: Codex
- Scope: docs/sample dashboard freshness
- Decision levels touched: none; command-anchor wording only

## Objective

Audit active network transport sample docs after the corrected validation-anchor policy and make the command split explicit: `check-all --format json` executes `NET-02..05`; `closeout --format json` is inventory evidence only.

## Scope and assumptions

- Scope is docs-only: active sample README, hands-on canary guide, `plan/22`, current snapshots, sample dashboard, and this report.
- `scripts/network_transport_samples.py` behavior is unchanged.
- `NET-01` remains a reported Sugoroku loopback parity anchor, not a standalone runnable `network_transport_samples.py` sample ID.
- Stop line: this package does not claim production socket / broker / QUIC / WebRTC, cryptographic session protocol, durable replay, distributed transport runtime, or final public transport ABI.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/clean-near-end/network-transport/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `scripts/README.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/22-network-transport-roadmap.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/19-repository-map-and-taxonomy.md`
- `scripts/network_transport_samples.py`
- `scripts/tests/test_network_transport_samples.py`
- `docs/reports/1075-network-transport-validation-anchor-correction.md`
- `docs/reports/1080-validate-docs-scope-and-network-anchor-audit.md`

## Actions taken

- Spawned a docs researcher to audit active network transport docs for stale command-anchor wording.
- Spawned a code mapper to verify `network_transport_samples.py` command semantics and test lock-in.
- Confirmed `check-all` executes the `NET-02..05` loop by calling `run_sample()` for each row.
- Confirmed `closeout` returns inventory fields such as `process_boundary_canaries`, `loopback_parity_sources`, `non_collapse_lanes`, `kept_later_gates`, and `validation_floor`.
- Updated `samples/clean-near-end/network-transport/README.md` to state that `check-all` is the executable canary anchor and `closeout` is inventory evidence only.
- Updated `docs/hands_on/network_transport_canaries_01.md` with the same command split before the command block.
- Updated `plan/22-network-transport-roadmap.md` to remove the invalid `check-all --transport loopback_socket` command anchor and split reported `NET-01` parity from runnable `NET-02..05`.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` with this audit checkpoint.
- Added this report.

## Files changed

- `samples/clean-near-end/network-transport/README.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `plan/22-network-transport-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1082-network-sample-readme-anchor-audit.md`

## Commands run

```bash
git status --short
git branch --show-current
git log -1 --oneline
sed -n '1,120p' samples/clean-near-end/network-transport/README.md
rg -n "network_transport_samples\\.py (closeout|check-all)|NET-02|NET-05|transport canar|production socket|durable replay" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/clean-near-end scripts/README.md docs/hands_on docs/research_abstract plan/22-network-transport-roadmap.md plan/09-helper-stack-and-responsibility-map.md plan/19-repository-map-and-taxonomy.md --glob '!docs/research_abstract/old/**'
sed -n '1,680p' scripts/network_transport_samples.py
sed -n '1,220p' scripts/tests/test_network_transport_samples.py
sed -n '1,130p' docs/hands_on/network_transport_canaries_01.md
sed -n '1,150p' docs/research_abstract/network_transport_plan_01.md
sed -n '1,150p' plan/22-network-transport-roadmap.md
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 -m unittest scripts.tests.test_network_transport_samples
python3 scripts/network_transport_samples.py check-all -h
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
rg -n "check-all --transport|executable closeout|network_transport_samples\\.py closeout --format json|network_transport_samples\\.py check-all --format json" samples/clean-near-end/network-transport/README.md docs/hands_on/network_transport_canaries_01.md plan/22-network-transport-roadmap.md samples_progress.md progress.md tasks.md docs/reports/1082-network-sample-readme-anchor-audit.md
```

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git branch --show-current
main

$ git log -1 --oneline
9a3beba Cool guided helper plan wording
```

Sub-agent evidence:

```text
docs researcher: found ambiguous command blocks in the active sample README and hands-on guide, an invalid `check-all --transport loopback_socket` anchor in plan/22, and `executable closeout` wording that blurred the corrected split.

code mapper: confirmed `check_all()` iterates SAMPLE_ROWS and executes `NET-02..05`; `closeout()` returns inventory only. Tests lock this in through `test_check_all_covers_all_transport_canaries` and closeout inventory tests.
```

Focused validation:

```text
$ python3 scripts/network_transport_samples.py check-all --format json
sample_count: 4
passed: NET-02, NET-03, NET-04, NET-05
failed: []
transport_scope: helper_local_process_boundary
transport_seam: loopback_socket

$ python3 scripts/network_transport_samples.py closeout --format json
returned inventory fields including active_sample_root, process_boundary_canaries, loopback_parity_sources, non_collapse_lanes, kept_later_gates, validation_floor, debug_output_modes, and limitations.

$ python3 -m unittest scripts.tests.test_network_transport_samples
Ran 11 tests in 1.605s
OK

$ python3 scripts/network_transport_samples.py check-all -h
usage: network_transport_samples.py check-all [-h] [--format {pretty,json}]
```

Docs validation:

```text
$ python3 scripts/check_source_hierarchy.py
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 35
  present: 35
  missing: 0
  all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1080 numbered report(s).

$ git diff --check
<no output>
```

Post-patch search:

```text
No `check-all --transport` or `executable closeout` active command anchor remains in the audited active files.
`network_transport_samples.py closeout --format json` remains in the sample README and hands-on guide only after text that classifies it as inventory evidence.
```

## What changed in understanding

The corrected policy was already reflected in top-level dashboards and recent hands-on closeout docs, but the active sample landing page and `plan/22` still let `closeout` look like a peer execution anchor. The current split is now explicit: `check-all` executes the helper canaries; `closeout` records inventory evidence; `NET-01` is reported Sugoroku loopback parity, not a standalone network canary ID.

## Open questions

- Production socket / broker / durable replay remain deferred.
- Final public transport ABI remains unfrozen.
- Actual `U1` commitment remains open and user-facing.

## Suggested next prompt

Continue autonomous maintenance: run focused network/docs validation, review the diff, commit/push, and then reassess remaining safe docs/sample freshness work.

## Plan update status

`plan/` 更新済み: `plan/22` now separates `NET-01` loopback parity from `NET-02..05` executable canaries and removes the invalid `check-all --transport loopback_socket` anchor.

## progress.md update status

`progress.md` 更新済み: recent log records the active sample README / hands-on / `plan/22` command-anchor cleanup.

## tasks.md update status

`tasks.md` 更新済み: current task-level status records the network sample docs command split.

## samples_progress.md update status

`samples_progress.md` 更新済み: PH0 and recent validation rows record this audit and revise the older 1080 note so current closeout references are classified as inventory evidence in both the sample README and canary guide.
Reviewer follow-up also updated the `NET-02..05` row's `Last validation` to `2026-05-01 11:53 JST` and added report `1082`, matching the fresh `check-all` rerun in this package.

## Skipped validations and reasons

- Full sample/Cargo floor was skipped because this package changes only docs wording and dashboard snapshots.
- Focused network/docs validation passed after this report was added:
  `network_transport_samples.py check-all --format json`, `network_transport_samples.py closeout --format json`, `python3 -m unittest scripts.tests.test_network_transport_samples`, `check_source_hierarchy.py`, `validate_docs.py`, and `git diff --check`.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

Sub-agents spawned:

- `Leibniz` (`docs_researcher`): completed; found active sample README, hands-on canary, and `plan/22` command-anchor drift.
- `Epicurus` (`code_mapper`): completed; mapped `check-all` execution and `closeout` inventory semantics.
- `Maxwell` (`reviewer`): completed; found `samples_progress.md` network row freshness drift. Follow-up updated the `NET-02..05` row to `2026-05-01 11:53 JST` and included report `1082`.

All three sub-agent sessions were closed after review and follow-up validation.
