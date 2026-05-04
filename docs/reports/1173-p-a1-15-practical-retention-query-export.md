# Report 1173 — P-A1-15 Practical Retention-Query Export Widening

- Date: 2026-05-04
- Author / agent: Codex
- Scope: `P-A1-15` practical retention-query export widening
- Decision levels touched: `L1`, `L2`

## Objective

`VIS-A1-07` を practical alpha-1 devtools lane の narrow row として actualize する。条件は、new runtime semantics や durable retention service を作らず、exact `SL-A1-02` save-load report を source にした report-local retained-artifact catalog と hit/miss query trace だけを distinct devtools bundle へ下ろすこと。

## Scope and assumptions

- `P-A1-14` の blocker 判定は維持する。`retention_scope` label と `retained_later_refs` inventory だけでは widen しない。
- widen 元は exact `SL-A1-02` save-load report に限定する。
- `VIS-A1-07` は report-local retained-artifact catalog と hit/miss query trace のみを扱う。
- durable retained-artifact catalog service、cross-session / remote retrieval API、retention expiry / lease lifecycle、distributed durable save/load は scope 外とする。
- product-preview lane、same-session runtime lane、public ABI lane はこの package で広げない。

## Start state / dirty state

- 開始時点の worktree は clean ではなく、`P-A1-15` 実装途中の dirty state だった。
- 既に code/expected sidecar 側には次の未commit差分があった。
  - `crates/mir-runtime/src/practical_alpha1_save_load.rs`
  - `crates/mir-runtime/tests/practical_alpha1_save_load.rs`
  - `scripts/practical_alpha1_export_devtools.py`
  - `scripts/tests/test_practical_alpha1_export_devtools.py`
  - `samples/practical-alpha1/expected/sl-a1-01-local-save-load-resume.expected.json`
  - `samples/practical-alpha1/expected/sl-a1-02-local-load-stale-membership-rejected.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-07-retention-query.expected.json`
- unrelated user changeや別 package の dirty state は無かった。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1172-p-a1-14-practical-retention-trace-blocker.md`

## Actions taken

1. `PracticalAlpha1SaveLoadReport` に report-local retention carrier を追加した。
   - `retention_scope = report_local_inventory`
   - `retained_artifacts`
   - saved membership frontier / restored membership frontier / resumed dispatch verdict / resumed event DAG の exact retained-artifact rows
2. `SL-A1-01/02` expected reports を widened し、save/load exact report が retained-artifact catalog を carry するよう同期した。
3. `scripts/practical_alpha1_export_devtools.py` に `VIS-A1-07` retention-query export を追加した。
   - source は exact `SL-A1-02` のみ
   - query requests / query results / retained artifact catalog / retained_later_refs を distinct bundle section として出力
   - outcomes は `hit` / `miss` のみ
4. `scripts/tests/test_practical_alpha1_export_devtools.py` に `VIS-A1-07` の exact bundle test を追加した。
5. Rust save/load test を widened し、`retention_scope` と retained artifact ids を exact expected と照合した。
6. practical alpha-1 snapshot docs / roadmap memory / sample taxonomy / script taxonomy を `P-A1-15` の narrow claim へ同期した。

## Files changed

- implementation
  - `crates/mir-runtime/src/practical_alpha1_save_load.rs`
  - `crates/mir-runtime/tests/practical_alpha1_save_load.rs`
  - `scripts/practical_alpha1_export_devtools.py`
  - `scripts/tests/test_practical_alpha1_export_devtools.py`
- expected artifacts
  - `samples/practical-alpha1/expected/sl-a1-01-local-save-load-resume.expected.json`
  - `samples/practical-alpha1/expected/sl-a1-02-local-load-stale-membership-rejected.expected.json`
  - `samples/practical-alpha1/expected/vis-a1-07-retention-query.expected.json`
- snapshot / roadmap / taxonomy docs
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `specs/18-practical-alpha1-scope.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `samples/practical-alpha1/expected/README.md`
  - `scripts/README.md`

## Commands run

- red-phase checks run before the implementation was completed
  - `python3 -m unittest scripts.tests.test_practical_alpha1_export_devtools`
  - `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture`
- green-phase checks after implementation sync
  - `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture`
  - `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py closeout --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-07 --format json`
  - `python3 scripts/practical_alpha1_product_preview.py run PE2E-07 --format json`
  - `python3 -m unittest scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_practical_alpha1_save_load scripts.tests.test_validate_docs`
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `cargo fmt --check`
  - `git diff --check`

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test practical_alpha1_save_load -- --nocapture`
  - 4 tests passed
- `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - `sample_count = 2`
  - `passed = ["SL-A1-01", "SL-A1-02"]`
  - `local_save_load_first_floor_complete = true`
  - `invalid_distributed_cut_guard_present = true`
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
  - `sample_count = 7`
  - `passed = ["VIS-A1-01", "VIS-A1-02", "VIS-A1-03", "VIS-A1-04", "VIS-A1-05", "VIS-A1-06", "VIS-A1-07"]`
  - `deferred_observables = []`
  - `stage_pa1_6_complete = true`
- `python3 scripts/practical_alpha1_export_devtools.py closeout --format json`
  - `implemented_observables = ["VIS-A1-01", "VIS-A1-02", "VIS-A1-03", "VIS-A1-04", "VIS-A1-05", "VIS-A1-06", "VIS-A1-07"]`
  - `deferred_observables = []`
  - `stage_pa1_6_complete = true`
- `python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-07 --format json`
  - static HTML bundle emitted successfully
  - `bundle_kind = retention_query_export`
- `python3 scripts/practical_alpha1_product_preview.py run PE2E-07 --format json`
  - product-preview lane remained unchanged
  - `what_it_does_not_prove` still includes `retention/on-demand trace completion`

## What changed in understanding

- `P-A1-14` の blocker は、「retention query は impossible」という意味ではなく、「exact source carrier が無い状態では widen しない」という意味だった。
- save/load exact report に report-local retained-artifact catalog を narrow 追加すれば、runtime semantics を増やさず `VIS-A1-07` を actualize できる。
- `VIS-A1-07` の honest reading は devtools-side observability widening であって、save/load stage completion や retained-artifact service completion ではない。

## Open questions

- `PA1-7` を次に widen するなら、invalid distributed-cut reject を practical save/load row としてどの carrier で表すか。
- stale witness / stale lease non-resurrection を `PA1-7` current lane に入れるか、別 lane に分けるか。
- `PE2E` lane に same-session runtime semantics を足す前に、preview と runtime execution boundary をどこまで固定するか。

## Suggested next prompt

`P-A1-16` として、saved-local-frontier boundary を崩さない explicit invalid distributed-cut reject row を practical save/load lane に追加し、`PA1-7` を narrow に widen してください。durable distributed save/load や stale witness / stale lease completion はまだ claim しないでください。

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` を `P-A1-15` / `VIS-A1-07` / `PA1-6 100%` の読みへ同期した。

## Documentation.md update status

`Documentation.md` 更新済み:
practical alpha-1 line の current actualization と non-claim を `P-A1-15` に同期した。

## progress.md update status

`progress.md` 更新済み:
`PA1-6` を 100% に更新し、`P-A1-15` を latest package として記録し、recent log を追記した。

## tasks.md update status

`tasks.md` 更新済み:
`P-A1-15` を closeout 済み package に追加し、next autonomous package を save/load widening 候補へ更新した。

## samples_progress.md update status

`samples_progress.md` 更新済み:
`VIS-A1-07` を actualized row として反映し、`PA1-6` を 100% に更新し、save/load row の source-carrier note も同期した。

## Reviewer findings and follow-up

- explicit sub-agent review は実施していない。
- 理由:
  - 現 turn では user から delegation / sub-agent 使用の明示指示が無く、現在の tool policy では `spawn_agent` を使わない。
- local focused review では次を確認した。
  - `VIS-A1-07` の source が exact `SL-A1-02` report に限定されていること
  - `retention_scope` / `retained_artifacts` が report-local carrier として独立していること
  - durable / remote / expiry semantics を docs 上で claim していないこと
  - `PE2E-07` preview が retention completion を claim していないこと

## Skipped validations and reasons

- `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture`
  - skipped
  - front-door parser/loader codeは未変更で、package scope は save-load report と devtools bundle widening に限定したため
- `cargo test -p mir-ast practical_alpha1_checker -- --nocapture`
  - skipped
  - checker semanticsは未変更で、`CHK-CUT-01` reuse wordingのみ維持したため
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
  - skipped
  - hot-plug runtime codeは未変更で、`VIS-A1-07` source は save-load exact reports のみであるため
- `python3 scripts/practical_alpha1_transport.py check-all --format json`
  - skipped
  - transport laneは source carrier として再利用したが、この package で transport implementation / expected artifact は変更していないため

## Commit / push status

- Primary package commit: `5e2ce33` (`mirrorea: close p-a1-15 practical retention query export`)
- Push status: pushed to `origin/main`
- Report status finalized in a docs-only follow-up commit after the primary package push

## Sub-agent session close status

- no sub-agent sessions were opened in this package

## 日本語要約

`P-A1-15` では、`P-A1-14` で止めた `VIS-A1-07` を、save/load exact report に report-local retained-artifact catalog を足すことで narrow に actualize した。意味するのは「exact report を viewer/export 側で retained artifact catalog + hit/miss query trace として読める」ということだけであり、durable retained-artifact service、remote retrieval、retention expiry lifecycle、distributed durable save/load はまだ完成扱いにしていない。
