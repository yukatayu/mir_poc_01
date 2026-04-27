# 0931 HotPlug AttachPoint Executable Widening

## Objective

phase 14 `HotPlug Patch / AttachPoint` の docs-first plan を、helper-local executable canary へ narrow に widen する。
具体的には `SUG-01` / `SUG-09` で `hotplug_lifecycle`、`--debug hotplug`、attach/detach telemetry-view、
`detach_request#1` auth-none envelope canary を current line に actualize する。

## Scope and assumptions

- current scope は Python helper `scripts/sugoroku_world_samples.py` とその test、関連 docs / plan / snapshot sync に限定する。
- `hotplug_lifecycle` は helper-local derived summary であり、authoritative seam は `MessageEnvelope` に残す。
- final hot-plug ABI、rollback protocol、durable migration engine、distributed activation ordering は固定しない。
- runtime hot-plug lifecycle と storage detach / cleanup script は別 concern として保つ。
- worktree には unrelated Rust-side current-L2 diff が残っているため、この package では stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`

## Actions taken

1. `scripts/tests/test_sugoroku_world_samples.py` に hot-plug lifecycle summary、detach boundary、`detach_request#1` seam grounding、closeout inventory を検査する test を追加した。
2. test red を確認した。`09_detach_todo` に `detach_request#1` envelope が存在せず、`hotplug_lifecycle.activation_cut.request_envelope` が inventory に接続していなかった。
3. `scripts/sugoroku_world_samples.py` に helper-local `hotplug_lifecycle` carrier、`hot-plug` layer signature、attach/detach telemetry rows、attach/detach visualization views、`--debug hotplug` formatter を追加・整理した。
4. `09_detach_todo` に `detach_request#1` accepted envelope を追加し、`detached_roll_request#1` rejected envelope と分けた。
5. detach 側 telemetry / visualization の source refs と summary を実在 envelope ID に揃え、derived summary が authoritative seam を曖昧化しないようにした。
6. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`samples_progress.md`、`plan/21-hotplug-attachpoint-roadmap.md`、`plan/01-status-at-a-glance.md`、`plan/11-roadmap-near-term.md`、`specs/10-open-questions.md`、`specs/11-roadmap-and-workstreams.md`、hands-on / research docs を current snapshot に同期した。
7. current promoted next package を `Network transport` `NET-02..05` に進め、hot-plug row は helper-local lifecycle canary close として読み替えた。
8. reviewer sub-agent を 2 回試したが completion が返らなかったため、local diff inspection と validation evidence を closeout 根拠に採用した。

## Files changed

- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
- `samples/clean-near-end/sugoroku-world/README.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/research_abstract/hotplug_attachpoint_plan_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0931-hotplug-attachpoint-executable-widening.md`

## Evidence / outputs / test results

- red phase:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples.SugorokuWorldSamplesTests.test_detach_todo_keeps_hotplug_summary_grounded_in_message_envelopes`
    - fail
    - `detach_request#1` missing from `09_detach_todo` envelope inventory
- green / closeout phase:
  - `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
    - pass
  - `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
    - pass
    - `AttachPoint[SugorokuGame#1]`, compatibility `compatible`, activation cut `attach_request#1`
  - `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
    - pass
    - `detach_request#1` accepted, `detached_roll_request#1` rejected, migration `deferred`
  - `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug visualization --format json`
    - pass
    - detach lifecycle view references `detach_request#1` and `detached_roll_request#1`
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
    - pass
    - `hotplug_lifecycle_states`, `hotplug_activation`, `hotplug_detach`, `--debug hotplug`
  - `python3 scripts/sugoroku_world_samples.py check-all --format json`
    - pass
  - `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json`
    - pass
  - `python3 scripts/check_source_hierarchy.py`
    - pass
  - `python3 scripts/validate_docs.py`
    - pass before report add: `Found 928 numbered report(s).`
  - `git diff --check`
    - pass
  - final docs-only rerun after report add:
    - `python3 scripts/check_source_hierarchy.py`
      - pass
    - `python3 scripts/validate_docs.py`
      - pass: `Found 929 numbered report(s).`
    - `git diff --check`
      - pass
- skipped validation:
  - `cargo test -p mir-runtime`
    - skipped
    - this package did not change Rust runtime code; helper-local hot-plug carrier only
  - reviewer sub-agent response
    - 2 回待機したが返答なし
    - retry 1 回の後に shutdown し、local diff inspection に切り替えた

## What changed in understanding

- hot-plug helper summary は、それ単体で authoritative seam になってはいけない。`MessageEnvelope` 側に attach / detach request と post-detach rejection が visible であることが必要だった。
- `detach_request#1` を accepted envelope として分けることで、compatibility / activation cut と post-detach rejection を同一イベントに潰さずに読めるようになった。
- `SUG-09` は依然として TODO boundary だが、TODO のままでも attachpoint compatibility / activation / rejection evidence を narrower canary として十分 actualize できる。

## Open questions

- final public `AttachPoint` ABI を object / schema / proof obligation のどこで切るか。
- rollback protocol と durable migration engine をどの package で current line に上げるか。
- hot-plug lifecycle と `NET-02..05` two-process widening をどの程度 machine-check で共有するか。
- avatar residual widening (`FAIRY-02`, `FAIRY-05`) を hot-plug / projection lane とどこで接続するか。

## Suggested next prompt

`Network transport` `NET-02..05` を進めてください。`NET-01` helper-local loopback preview と今回の hot-plug lifecycle carrier を前提に、two-process / reconnect / typed failure / redacted route trace のうち low-risk に actualize できる最小 canary を helper-local に切り出し、docs / plan / progress / tasks / samples_progress / report を同じ task で同期してください。
