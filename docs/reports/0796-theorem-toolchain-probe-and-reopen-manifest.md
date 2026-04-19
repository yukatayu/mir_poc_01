# Report 0796 — theorem toolchain probe and reopen manifest

- Date: 2026-04-19T03:37:00Z
- Author / agent: Codex
- Scope: Package 51。actual Lean execution reserve を docs-only unavailable note に留めず、toolchain probe helper と sample-aware reopen manifest として repo-local に actualize する
- Decision levels touched: L2

## 1. Objective

`specs/examples/513` と `514` を前提に、actual Lean execution が local environment で unavailable な現状を helper/CLI と focused test へ落とし、toolchain が揃った時の reopen path を machine-readable な manifest に固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/513-current-l2-theorem-actual-lean-execution-availability-probe.md`
- `specs/examples/514-current-l2-theorem-public-seam-compression-after-local-lean-unavailable-probe.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `scripts/current_l2_theorem_lean_stub_pipeline.py`
- `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py`

## 3. Actions taken

1. actual Lean execution reserve の current stop line を再確認し、toolchain unavailable という事実と sample-aware reopen path を分離した。
2. `scripts/tests/test_current_l2_theorem_toolchain_probe.py` を先に追加し、missing / ready / sample-aware reopen manifest の 3 挙動を赤で固定した。
3. `scripts/current_l2_theorem_toolchain_probe.py` を追加し、`lean` / `lake` / `elan` probe、status 判定、version capture、sample-aware reopen manifest emit を実装した。
4. local environment で CLI を実行し、current state が `unavailable` であることと、`e5-underdeclared-lineage` の pipeline plan carry-over が JSON として出ることを確認した。
5. `specs/examples/516` を追加し、current recommendation / retained alternatives / stop line を source-backed に整理した。

## 4. Files changed

- `scripts/current_l2_theorem_toolchain_probe.py`
- `scripts/tests/test_current_l2_theorem_toolchain_probe.py`
- `specs/examples/516-current-l2-theorem-actual-lean-execution-toolchain-probe-and-reopen-manifest.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts/tests/test_current_l2_theorem_toolchain_probe.py`
  - `Ran 3 tests in 0.030s`
  - `OK`
- `python3 scripts/current_l2_theorem_toolchain_probe.py`
  - `toolchain_status = "unavailable"`
  - `missing_tools = ["lean", "lake", "elan"]`
- `python3 scripts/current_l2_theorem_toolchain_probe.py e5-underdeclared-lineage`
  - `toolchain_status = "unavailable"`
  - `pipeline_plan.smoke_mode = "static"`
  - `pipeline_plan.formal_hook_command[2:4] = ["smoke-formal-hook-static", "e5-underdeclared-lineage"]`

## 6. Evidence / findings

- actual Lean execution reserve は、toolchain probe と sample-aware reopen manifest を持つ shape まで actualize できた。
- local environment の current fact は引き続き `lean` / `lake` / `elan` unavailable であり、actual Lean execution 自体は reopen されていない。
- existing Lean-stub pipeline を reopen plan に carry-over できたため、toolchain ready 後の next narrow probe は docs ではなく concrete command plan に接続できる。

## 7. Changes in understanding

- environment-conditional reserve は「今は無理」で止まるのではなく、probe helper と reopen manifest を持つ self-driven reserve として扱える。
- theorem line の remaining blocker は abstract ではなく、toolchain availability と final public theorem contract 群に narrow 化している。

## 8. Open questions

- actual Lean execution first reopen を local install で取るか、hermetic/container route で取るか。
- actual Lean execution を取った後、public theorem contract reopen と同一 package にするか。

## 9. Suggested next prompt

`specs/examples/516` を anchor に `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`specs/10`、`specs/11`、`specs/12`、`plan/90` を同期し、その後 `model-check` 側でも public seam compression を actualize してください。
