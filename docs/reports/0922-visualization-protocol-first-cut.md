# 0922 VisualizationProtocol First Cut

## Objective

`VisualizationProtocol` の helper-local / report-local first cut を追加し、
visualization / telemetry を untyped debug leak ではなく
label / authority / redaction を持つ evidence carrier として current repo に反映する。

## Scope and assumptions

- current scope は repo-local alpha current layer の helper-local / report-local inventory である。
- final public visualization protocol、retention policy、multi-tenant telemetry service は固定しない。
- 既存の `MessageEnvelope / Auth seam` first cut を前提に、visualization が auth / transport / witness を潰さない reading を保つ。
- worktree には unrelated current-L2 CLI formatting diff も存在したが、この package では stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`

## Actions taken

1. Sugoroku helper に `TelemetryRow` / `VisualizationView` を追加し、`run_sample()` payload へ `telemetry_rows` / `visualization_views` を含めた。
2. Sugoroku helper に `--debug visualization` を追加し、pretty formatter で label / authority / redaction と summary / fields を読めるようにした。
3. Sugoroku closeout に `visualization_views`、`visualization_view_kinds`、`telemetry_rows`、`telemetry_row_kinds`、`redaction_policy_names` を追加した。
4. clean near-end runtime report / closeout に report-local `VisualizationView` / `TelemetryRow` inventory を追加した。
5. Python unittest と Rust test を追加し、helper/runtime の最小 inventory surface を検証した。
6. README / Documentation / progress / tasks / samples_progress / plan / specs / hands-on docs を同期した。
7. unrelated current-L2 CLI formatting diff は読み取りのみとし、この package には含めないと明示した。

## Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `crates/mir-runtime/src/clean_near_end.rs`
- `crates/mir-runtime/tests/clean_near_end_samples.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/hands_on_sugoroku_08_visualization_protocol.md`
- `samples/clean-near-end/sugoroku-world/README.md`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
- `cargo test -p mir-runtime --test clean_near_end_samples`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- `VisualizationProtocol` は final public viewer API を先に決めなくても、
  helper-local / report-local evidence carrier として段階 actualize できる。
- visualization / telemetry は auth / transport / witness の上に重なる別 lane として保持した方が、
  今後の projection / adapter / hot-plug package とも整合しやすい。
- current Sugoroku sample では `turn_timeline`、`message_route`、`membership_timeline`、
  `verification_summary` 程度の narrow view kind でも十分に package close evidence になる。

## Open questions

- final public `VisualizationProtocol` schema をどこまで固定するか。
- retention / export / multi-tenant telemetry policy をどの layer に置くか。
- cross-place projection viewer と visualization authority model をどこで合流させるか。

## Suggested next prompt

`Typed external boundary / adapter` package を進めてください。`samples_progress.md` に `EXT-01..05` の sample ladder を追加し、`stdio` を Mir core primitive にしない current line、provider boundary / adapter failure / debug label restriction を docs-first に同期し、`progress.md` / `tasks.md` / report まで同じ task で閉じてください。
