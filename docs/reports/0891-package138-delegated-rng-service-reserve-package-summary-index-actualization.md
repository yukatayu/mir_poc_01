# 0891 — Package 138 delegated-rng-service reserve package summary index actualization

## Objective

`delegated_rng_service` reserve package を reserve list の説明だけで終わらせず、
`p09 / p07 / p08` の runnable evidence と summary index まで repo-local に materialize し、
docs / plan / tasks / progress / traceability を current reading に揃える。

## Scope And Assumptions

- 規範判断の正本は `specs/` とし、今回の変更は helper-local / non-production reserve package actualization に留める。
- `delegated_rng_service` の provider placement practical judgment 自体は既存の `specs/examples/477` と `571` を carry-over する。
- final public provider receipt schema、delegated provider attestation public contract、final public witness/provider/artifact contract、`delegated_rng_service + auditable_authority_witness` combined public contract、`distributed_randomness_provider`、control-plane separated carrier はこの task で adoption しない。

## Documents Consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md`
- `specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md`
- `specs/examples/606-current-l2-reserve-integration-entrypoint-summary-sync.md`
- `specs/examples/610-current-l2-auditable-authority-witness-reserve-package-summary-index-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## Actions Taken

1. `scripts/current_l2_guided_samples.py` に `emit-reserve delegated-rng-service` を追加し、`p09 / p07 / p08` の run-source-sample JSON と `package-summary.md` / `package-summary.json` を `target/current-l2-guided/reserve-packages/delegated-rng-service` に materialize する helper を actualize した。
2. provider placement practical line 用の summary index field として、`room_profile_provider`、`room_profile_claim`、provider boundary refs、optional attachment refs を追加した。
3. new normative anchor として `specs/examples/611-current-l2-delegated-rng-service-reserve-package-summary-index-actualization.md` を追加した。
4. `Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/11`、`plan/16`、`plan/17`、`plan/18`、`plan/90`、`specs/00`、`specs/11`、`specs/12`、`samples/problem-bundles/README.md`、`samples/problem-bundles/problem2-order-handoff-shared-space.md` を更新し、delegated-rng-service package close 後の next reopen line を model-check second-line に再同期した。

## Files Changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/611-current-l2-delegated-rng-service-reserve-package-summary-index-actualization.md`

## Commands Run

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- `python3 -m unittest scripts.tests.test_problem_sample_bundles`
- `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service`
- `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service --format json`
- `python3 scripts/current_l2_guided_samples.py reserve`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / Outputs / Test Results

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `84` tests passed
- `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - `3` tests passed
- `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service`
  - `p09` を `delegated provider placement reached`
  - `p07` を `guard-only authority-rng baseline contrast`
  - `p08` を `guard-only reconnect contrast`
    として出力した
- `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service --format json`
  - `room_profile_provider = fairness_source = delegated_rng_service`
  - `room_profile_claim = fairness_claim = opaque_authority_trust`
  - `package_summary_markdown/json` path を返した
- `python3 scripts/current_l2_guided_samples.py reserve`
  - `delegated-rng-service` entry commands の先頭が `emit-reserve delegated-rng-service` に更新されたことを確認した
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - whitespace / conflict markers なし

## What Changed In Understanding

- `delegated_rng_service` は理論判断としては既に close していたが、repo-local execution evidence としては Problem 2 全体 bundle に埋もれていた。
- `emit-scenario problem2` と `emit-reserve delegated-rng-service` を分けることで、Problem 2 全体の runnable scenario loop と provider placement package 単独の reopen entrypoint を無理なく分離できた。
- `p09` reached と `p07 / p08` contrast を単独 summary index にしたことで、authority keeps commit と optional attachment first を docs / helper / sample guide で同じ読みへ揃えやすくなった。

## Open Questions

- 次の reserve package は model-check second-line を先に単独 summary index まで actualize するか、problem-local mixed gate lane を追加で細分化するか。
- `emit-reserve` を generic summary-index family に widen するか、package ごとの narrow helper として段階的に増やすか。

## Suggested Next Prompt

`model-check second-line` reserve package についても、repo-local summary index と sample bundle 導線を actualize し、current next reopen line を later mixed gate / true user-spec hold line まで絞ってください。`
