# 1026. current-l2 guided compatibility front door and historical-anchor cooling

## Objective

`scripts/current_l2_guided_samples.py` の current active compatibility front door が
`list / smoke-all / closeout` の 3 コマンドだけである事実に合わせて、
retired helper command claim と `p06` / reserve-summary の温度差を
active docs / snapshot docs 側で冷やす。

## Scope and assumptions

- docs-first maintenance package に限定する。
- final public parser / checker / runtime API、final public verifier contract、
  final public witness/provider/artifact contract、final property language、
  final public theorem contract は claim しない。
- `specs/examples/606/607/608/612` の helper command は current active front door ではなく、
  2026-04-22 clean-sample migration 前の historical helper-local memory として扱う。
- `p06` は current clean-near-end theorem/model-check live floor ではなく、
  sample-visible corrected prototype historical anchor として扱う。

## Documents consulted

- `AGENTS.md`
- `specs/12-decision-register.md`
- `specs/examples/459-current-l2-verifier-boundary-and-typed-theorem-model-check-current-first-line-integration.md`
- `specs/examples/466-current-l2-problem1-actual-adoption-package-and-theorem-first-pilot.md`
- `specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `specs/examples/606-current-l2-reserve-integration-entrypoint-summary-sync.md`
- `specs/examples/607-current-l2-parser-side-residual-lane-helper-actualization.md`
- `specs/examples/608-current-l2-true-user-spec-hold-line-freeze-sync.md`
- `specs/examples/610-current-l2-auditable-authority-witness-reserve-package-summary-index-actualization.md`
- `specs/examples/611-current-l2-delegated-rng-service-reserve-package-summary-index-actualization.md`
- `specs/examples/612-current-l2-model-check-second-line-reserve-package-summary-index-actualization.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/00-document-map.md`
- `progress.md`
- `tasks.md`
- `scripts/current_l2_guided_samples.py`

## Actions taken

1. stale current-line wording を再探索し、`D-052` / `D-058` / `D-191..193`、`specs/examples/568`、`606`、`607`、`608`、`612` を今回の close 範囲に絞った。
2. `scripts/current_l2_guided_samples.py` を再読し、current active compatibility front door が `list / smoke-all / closeout` に限定されることを source で確認した。
3. exploratory command として `lane parser-side-residual`、`hold-line`、`residuals --format json`、`reserve --format json` を実行し、すべて migration note + `supported compatibility commands: list, smoke-all, closeout` で exit 2 になることを確認した。
4. `specs/12` の `D-052` / `D-058` を、`p06` が current live floor ではなく historical anchor である reading に冷やした。
5. `specs/12` の `D-191..193` と `specs/examples/606/607/608` を、reserve / residual / hold-line helper が current active command ではなく historical helper-local memory である reading に同期した。
6. `specs/examples/612` を、historical `emit-reserve model-check-second-line` summary memory と current compatibility front door の分離へ同期した。
7. `specs/examples/568` の stale numbered queue wording を historical queue memory として冷やした。
8. `progress.md` と `tasks.md` に maintenance-lane snapshot と recent log を追記した。
9. local focused diff review を行い、reviewer sub-agent timeout 後は local evidence を fallback にした。

## Files changed

- `specs/12-decision-register.md`
- `specs/examples/459-current-l2-verifier-boundary-and-typed-theorem-model-check-current-first-line-integration.md`
- `specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `specs/examples/606-current-l2-reserve-integration-entrypoint-summary-sync.md`
- `specs/examples/607-current-l2-parser-side-residual-lane-helper-actualization.md`
- `specs/examples/608-current-l2-true-user-spec-hold-line-freeze-sync.md`
- `specs/examples/612-current-l2-model-check-second-line-reserve-package-summary-index-actualization.md`
- `progress.md`
- `tasks.md`

## Evidence / outputs / test results

### Exploratory failures that changed the reading

- `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
  - exit 2
  - stderr:
    `current_l2_guided_samples.py now forwards to the clean near-end active suite. Legacy problem1/problem2 bundle commands were retired with the 2026-04-22 clean-sample migration.`
    `supported compatibility commands: list, smoke-all, closeout`
- `python3 scripts/current_l2_guided_samples.py lane parser-side-residual`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py hold-line`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py residuals --format json`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py reserve --format json`
  - exit 2 with the same migration note

### Focused validation run

- `python3 scripts/current_l2_guided_samples.py list`
  - pass
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - pass
- `cargo test -q -p mir-runtime --test current_l2_operational_cli`
  - pass
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json`
  - pass
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass (`Found 1023 numbered report(s).` before this report)
- `git diff --check`
  - pass

### Review evidence

- docs researcher `Zeno` completed and identified the warm spots around `D-052` / `D-058` / `specs/examples/612`.
- reviewer `Russell` was waited on twice and did not return a final payload before shutdown.
- local fallback review used:
  - focused `git diff`
  - passing focused validation
  - source check against `scripts/current_l2_guided_samples.py`

## What changed in understanding

- `scripts/current_l2_guided_samples.py` is no longer a broad guided-helper surface. In current repo state it is only a compatibility shim to the clean near-end active suite and only accepts `list / smoke-all / closeout`.
- Therefore `reserve` / `residuals` / `lane ...` / `hold-line` / `emit-reserve ...` command claims in active docs are not merely warm wording; they are stale command-surface claims unless explicitly marked historical.
- `p06` can remain useful as bridge-memory evidence, but it must be named as a historical corrected prototype anchor rather than folded into the live theorem/model-check floor.

## Open questions

- `emit-theorem problem1`、`emit-scenario problem2`、`emit-reserve auditable-authority-witness`、`emit-reserve delegated-rng-service`、`matrix` / `bundle` / `reopen-map` / `split` / `quickstart` など、retired helper command claim がまだ active docs / decision register / roadmap memory に残っている。
- `specs/00-document-map.md` と `specs/11-roadmap-and-workstreams.md` の reserve/theorem/scenario helper memory は、historical command wording へ追加 cooling が必要。

## Suggested next prompt

`current_l2_guided_samples.py` の retired helper claims を次 package で続けて冷やし、`emit-theorem` / `emit-scenario` / `emit-reserve auditable-authority-witness` / `emit-reserve delegated-rng-service` と、その mirror になっている `specs/00` / `specs/11` / `specs/12` / `specs/examples/609..611` の active wording を historical memory に同期してください。

## plan/ updates

- 更新不要

## progress.md updates

- 更新した

## tasks.md updates

- 更新した

## samples_progress.md updates

- 更新不要

## Skipped validations and reasons

- full validation floor は未実行。
  - 理由: docs-only maintenance package であり、current wrapper front door / source hierarchy / docs scaffold / focused runtime sample checks で十分に今回の claim を裏づけられるため。
- `samples_progress.md` 関連の sample dashboard command は未実行。
  - 理由: runnable sample set / progress % / blocker 自体は今回変更していないため。

## Commit / push status

- report作成時点: 未commit / 未push

## Sub-agent session close status

- `Zeno` (`019dde0b-9804-7181-8080-860095bfe14e`)
  - completed を受領後に close 済み
- `Russell` (`019dde16-70f2-7701-bb3f-78b1498bbc5d`)
  - 2 回 wait したが final payload なし
  - timeout fallback を採用し、その後 close 済み
