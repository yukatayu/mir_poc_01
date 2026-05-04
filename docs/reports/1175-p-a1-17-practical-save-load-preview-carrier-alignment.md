# Report 1175 — P-A1-17 Practical Save-Load Preview Carrier Alignment

- Date: 2026-05-04
- Author / agent: Codex
- Scope: `P-A1-17` practical save-load preview carrier alignment
- Decision levels touched: `L1`, `L2`

## Objective

`PE2E-06` invalid distributed save rejected preview の source authority を direct checker evidence から widened practical save/load lane 側へ戻し、exact `SL-A1-03` save-load preflight reject report を consume する thin product-preview bundle へ揃える。

## Scope and assumptions

- actualize 対象 row は増やさない。`PE2E-06` の source-carrier alignment のみを行う。
- `PE2E-06` は exact `SL-A1-03` save-load preflight reject report を consume する。
- `SL-A1-03` 自体の semantics は広げない。
- runtime-backed `SL-A1-01/02` saved-frontier branch と checker-backed `SL-A1-03` preflight branch は collapse しない。
- runtime distributed checkpoint execution、distributed durable save/load、same-session runtime attach/detach execution、same-session product runtime、final public CLI / viewer / save-load ABI は scope 外とする。

## Start state / dirty state

- 開始時点の worktree は clean で、`P-A1-16` closeout commit/push 済みの状態だった。
- `P-A1-17` の dirty state は、この task で追加した次の差分のみだった。
  - `scripts/practical_alpha1_product_preview.py`
  - `scripts/tests/test_practical_alpha1_product_preview.py`
  - `samples/practical-alpha1/previews/pe2e-a1-06-invalid-distributed-save-rejected/preview.json`
  - `samples/practical-alpha1/expected/pe2e-a1-06-invalid-distributed-save-rejected.expected.json`
  - snapshot docs / roadmap / dashboards / report
- unrelated user change は見当たらなかった。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/previews/pe2e-a1-06-invalid-distributed-save-rejected/preview.json`
- `samples/practical-alpha1/expected/pe2e-a1-06-invalid-distributed-save-rejected.expected.json`
- `scripts/README.md`
- `docs/reports/1174-p-a1-16-practical-invalid-distributed-cut-save-load-preflight.md`

## Actions taken

1. `scripts/tests/test_practical_alpha1_product_preview.py` を先に RED 化した。
   - `PE2E-06` が `practical-alpha1-save-load` / `SL-A1-03` を source report として要求する test へ変更
   - preflight section が `saved_local_frontier_emitted = false` を保つことを要求
2. `scripts/practical_alpha1_product_preview.py` の `PE2E-06` builder を realign した。
   - preview manifest の `required_source_reports` を `SL-A1-03` へ変更
   - direct checker report ではなく `_save_load_report("SL-A1-03")` を consume
   - `save_load_preflight_reject` section を導入
   - `source_checker_report` 由来の `orphan_receive` reject detail は relay するが、preview lane の source authority 自体は save/load report に置く
3. preview manifest と expected bundle を同期した。
   - `checker_package` -> `save_load_package`
   - `CHK-CUT-01` direct report -> `SL-A1-03` exact save-load preflight report
4. snapshot docs / roadmap / dashboard を同期した。
   - latest package を `P-A1-17` へ更新
   - `PE2E-06` が exact `SL-A1-03` evidence を consume することを明記
   - safe next package は未promoteで、broader save/load widening などの later reopen に縮退した

## Files changed

- implementation
  - `scripts/practical_alpha1_product_preview.py`
  - `scripts/tests/test_practical_alpha1_product_preview.py`
- practical preview fixtures / expected artifacts
  - `samples/practical-alpha1/previews/pe2e-a1-06-invalid-distributed-save-rejected/preview.json`
  - `samples/practical-alpha1/expected/pe2e-a1-06-invalid-distributed-save-rejected.expected.json`
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
  - `scripts/README.md`
- report
  - `docs/reports/1175-p-a1-17-practical-save-load-preview-carrier-alignment.md`

## Commands run

- RED confirmation
  - `python3 -m unittest scripts.tests.test_practical_alpha1_product_preview`
- package closeout validation floor
  - `python3 scripts/practical_alpha1_save_load.py run SL-A1-03 --format json`
  - `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - `python3 scripts/practical_alpha1_product_preview.py run PE2E-06 --format json`
  - `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
  - `python3 scripts/practical_alpha1_product_preview.py closeout --format json`
  - `python3 -m unittest scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_practical_alpha1_save_load scripts.tests.test_validate_docs`
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `cargo fmt --check`
  - `git diff --check`

## Evidence / outputs / test results

- RED confirmation
  - `python3 -m unittest scripts.tests.test_practical_alpha1_product_preview`
  - failed before implementation because `PE2E-06` still reported `practical-alpha1-checker` instead of `practical-alpha1-save-load`
- `python3 scripts/practical_alpha1_save_load.py run SL-A1-03 --format json`
  - `terminal_outcome = "rejected_invalid_distributed_cut_preflight"`
  - `checker_guard_refs = ["CHK-CUT-01"]`
  - `saved_local_frontier_emitted = false`
- `python3 scripts/practical_alpha1_product_preview.py run PE2E-06 --format json`
  - `source_reports = [{"family": "practical-alpha1-save-load", "sample_id": "SL-A1-03", ...}]`
  - `preview_sections.save_load_preflight_reject.source_rejected_kind = "orphan_receive"`
  - `saved_local_frontier_emitted = false`
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
  - all product-preview rows passed exact expected bundles
  - `PE2E-06` stayed within the non-final preview floor
- `python3 scripts/practical_alpha1_product_preview.py closeout --format json`
  - `PE2E-01..09` remain the current actualized preview rows
  - `stage_pa1_8_complete` remains false
- `python3 -m unittest scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_practical_alpha1_save_load scripts.tests.test_validate_docs`
  - product-preview / save-load / docs validator tests passed
- docs / hierarchy / formatting checks
  - source hierarchy check, docs scaffold validation, `cargo fmt --check`, and `git diff --check` passed

## What changed in understanding

- `PE2E-06` の direct checker consumption は technically possible でも、save/load lane が `SL-A1-03` まで widened した後は source authority をそちらへ戻した方が carrier graph が正直になる。
- `SL-A1-03` を preview lane が consume しても、runtime-backed `SL-A1-01/02` branch と checker-backed preflight branch の separation は保てる。
- `PE2E-06` alignment を done にしたことで、next safe package を名乗るには broader save/load semantics 側で新しい concrete row を定義する必要がある。

## Open questions

- stale witness / stale lease non-resurrection は、practical save/load lane で次に actualize できるほど narrow に定義されているか。
- broader save/load widening の前に、docs-only carrier inventory package を一度挟む方が honest か。

## Suggested next prompt

`P-A1-18` を進めるなら、stale witness / stale lease non-resurrection か別の broader save/load row を narrow に定義し、saved local frontier branch を collapse しない exact evidence package として切ってください。もしまだ row definition が不足しているなら、先に docs-only blocker inventory package を作ってください。

## Plan update status

`plan/` 更新済み:
- `plan/01-status-at-a-glance.md` に `P-A1-17` closeout と post-alignment reopen point を反映
- `plan/44-practical-alpha1-roadmap.md` に `PE2E-06` の exact `SL-A1-03` source alignment と next reopen point を反映

## Documentation.md update status

`Documentation.md` 更新済み:
- latest package を `P-A1-17` へ更新
- `PE2E-06` が exact `SL-A1-03` save-load preflight reject report を consume することを追記

## progress.md update status

`progress.md` 更新済み:
- current status / validation freshness / next autonomous package を `P-A1-17` へ同期
- `P-A1-18` 未promote を明記

## tasks.md update status

`tasks.md` 更新済み:
- `P-A1-17` closeout、package map row、ordered current work を同期

## samples_progress.md update status

`samples_progress.md` 更新済み:
- `PE2E-06` source authority が exact `SL-A1-03` へ移ったことを product-preview row と current status に反映
- validation history と `PH0` report trail を同期

## Reviewer findings and follow-up

- sub-agent review は実施しなかった。
- 現 turn の developer policy では explicit user request がない限り `spawn_agent` を使わない制約があるため、local focused review で代替した。
- local review では次を確認した。
  - `PE2E-06` が direct checker report ではなく exact `SL-A1-03` save-load preflight report を source report として持つこと
  - `PE2E-06` が `saved_local_frontier_emitted = false` を保持し、runtime-backed `SL-A1-01/02` branch と collapse していないこと
  - snapshot docs が runtime distributed checkpoint execution や broader save/load semantics を claim していないこと

## Skipped validations and reasons

- broader Cargo/Rust runtime tests は未実行とした。
- 理由:
  - この package では Rust source を変更していない
  - widening は Python preview helper / preview manifest / expected artifact / snapshot docs に限定される
  - package-specific validation floor は existing exact save-load report、product-preview runner、docs/hierarchy/fmt/diff checks で十分だった

## Commit / push status

Pending at report write.

## Sub-agent session close status

- no sub-agent session opened in this task

## 日本語要約

この package では、新しい row を増やさずに `PE2E-06` の source authority だけを整理した。invalid distributed save rejected preview はもう direct checker evidence ではなく、exact `SL-A1-03` save-load preflight reject report を consume する。これにより product-preview lane が widened save/load lane と整合し、`SL-A1-01/02` の runtime-backed saved-frontier branch と `SL-A1-03` の checker-backed preflight branch の separation も保たれた。ここから先は broader save/load semantics の concrete row が無いので、safe な `P-A1-18` はまだ未promoteである。
