# 1029. current-l2 guided bundle cluster and check-source-sample cooling

## Objective

`scripts/current_l2_guided_samples.py` の retired helper / bundle / matrix / quickstart surfaces を current docs から外し、surviving current surface を `list / smoke-all / closeout`、archived parser-companion compare-floor、clean-near-end `check-source-sample` に限定して mirror を揃える。

## Scope and assumptions

- docs-first maintenance package として扱い、final public parser grammar、final public parser / checker / runtime API、final public verifier contract、final typed calculus は claim しない。
- active current-L2 sample root は `samples/clean-near-end/`、archived pre-clean-near-end roots は `samples/old/2026-04-22-pre-clean-near-end/` に留める。
- archived parser-companion inspection は repo-local compare floor として残してよいが、active sample root や current guided helper front door と混ぜない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md`
- `specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
- `specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md`
- `specs/examples/578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md`
- `specs/examples/579-current-l2-parser-side-request-head-clause-bundle-inspector-actualization.md`
- `specs/examples/580-current-l2-parser-side-representative-mapping-matrix-actualization.md`
- `specs/examples/581-current-l2-explained-representative-problem-sample-bundles.md`
- `specs/examples/582-current-l2-representative-problem-bundle-smoke-runner-actualization.md`
- `specs/examples/583-current-l2-representative-problem-bundle-aggregate-smoke-summary-actualization.md`
- `specs/examples/584-current-l2-representative-problem-bundle-failure-focused-smoke-diagnostics-actualization.md`
- `specs/examples/585-current-l2-representative-problem-bundle-quickstart-walkthrough-hardening.md`
- `specs/examples/586-current-l2-representative-problem-quickstart-cli-mirror-actualization.md`
- `specs/examples/587-current-l2-representative-problem-quickstart-parity-checks-actualization.md`
- `specs/examples/600-current-l2-typed-checker-first-executable-slice-actualization.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`

## Actions taken

1. `specs/examples/572..578` と `580..582`、`585..587` を current helper / bundle / matrix / quickstart surface から historical helper memory / archived bundle memory へ冷やした。
2. `specs/examples/583/584` を surviving `smoke-all` compatibility wrapper として書き直し、retired per-problem smoke bundle と混同しない wording へ寄せた。
3. `specs/examples/579` を archived parser-companion `p06 / p07 / p08` compare-floor inspection command として current repo-local command cut に整理した。
4. `specs/examples/600` を archived Problem 1 quickstart から切り離し、active clean-near-end `.mir` family に対する `check-source-sample` current surface として整理した。
5. `specs/12` の `D-157..172` と `D-185`、`specs/11`、`specs/00` を同じ historical/current split に mirror した。
6. archived `samples/old/.../problem-bundles/` docs を archive framing へ書き換え、Problem 1 doc の sample-side current anchor を clean-near-end `typing/01_authorized_declassification.mir` に差し替えた。
7. `progress.md` と `tasks.md` を current stale-docs queue に合わせて更新し、next package を earlier example cluster の pre-clean-near-end path memory cooling へ切り替えた。
8. validation 中に `specs/examples/579` の evidence 欄が実在しない `mir-ast` test target 名を指している drift を発見し、actual target 名 (`current_l2_request_head_clause_bundle_manifest` / `current_l2_stage3_request_head_clause_bundle_spike`) に補正した。
9. reviewer 指摘を受けて `specs/examples/577` の test target drift、archived Problem 1 doc の stale live Lean path、`samples/problem-bundles/` pseudo-path を real archived root へ揃える mirror drift を同 package 内で補修した。

## Files changed

- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md`
- `specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
- `specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md`
- `specs/examples/578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md`
- `specs/examples/579-current-l2-parser-side-request-head-clause-bundle-inspector-actualization.md`
- `specs/examples/580-current-l2-parser-side-representative-mapping-matrix-actualization.md`
- `specs/examples/581-current-l2-explained-representative-problem-sample-bundles.md`
- `specs/examples/582-current-l2-representative-problem-bundle-smoke-runner-actualization.md`
- `specs/examples/583-current-l2-representative-problem-bundle-aggregate-smoke-summary-actualization.md`
- `specs/examples/584-current-l2-representative-problem-bundle-failure-focused-smoke-diagnostics-actualization.md`
- `specs/examples/585-current-l2-representative-problem-bundle-quickstart-walkthrough-hardening.md`
- `specs/examples/586-current-l2-representative-problem-quickstart-cli-mirror-actualization.md`
- `specs/examples/587-current-l2-representative-problem-quickstart-parity-checks-actualization.md`
- `specs/examples/600-current-l2-typed-checker-first-executable-slice-actualization.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`

## Evidence / outputs / test results

- retired helper negative evidence:
  - `python3 scripts/current_l2_guided_samples.py show problem1`
  - `python3 scripts/current_l2_guided_samples.py matrix problem1`
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py smoke problem1`
  - `python3 scripts/current_l2_guided_samples.py quickstart problem1`
  - `python3 scripts/current_l2_guided_samples.py quickstart-parity`
  - all exited `2` with migration message that only `list / smoke-all / closeout` remain supported compatibility commands.
- surviving helper current surface:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`
- archived parser-companion compare floor:
  - `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt --format json`
  - `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt --format pretty`
- parser-side tests:
  - `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_manifest`
  - `cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_spike`
- clean-near-end checker-shaped current surface:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/typing/01_authorized_declassification.mir --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json`
- runtime CLI test:
  - `cargo test -p mir-runtime --test current_l2_operational_cli`
- docs floor:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
- post-review drift recheck:
  - `rg -n "samples/problem-bundles|current_l2_stage3_request_head_clause_bundle_sample_bundle|samples/lean/current-l2/p06-typed-proof-owner-handoff" ...` returned no matches in the touched mirror set.
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- guided helper stale-current drift は `588..612` cluster だけでは閉じず、older `572..587` と `600`、さらに archived problem-bundle docs まで同じ helper-memory line として連動していた。
- archived parser-companion compare-floor inspection (`579`) と clean-near-end `check-source-sample` current surface (`600`) は current repo-local command として残せるが、どちらも final public parser / checker / verifier surface の claim にはならない。
- parser-side evidence rows は wording だけでなく actual test target 名も drift しうるため、docs-only package でも fresh command rerun が必要である。

## Open questions

- earlier example cluster に残る pre-clean-near-end prototype path と `samples/lean/current-l2` path の stale current wordingを、どの package 切りで冷やすか。
- active example docs に残る archived path mention のうち、current compare-floor として残すものと完全 historical memory に落とすものの境界をどこまで一括で整理するか。

## Suggested next prompt

`specs/examples/451..567` 周辺の earlier example cluster を対象に、pre-clean-near-end prototype / old lean path の stale current wording を historical memory / archived compare-floor / active clean-near-end current surface に切り分け、mirror docs と progress/tasks を同期してほしい。

## plan/ update status

- `plan/ 更新不要`

## progress.md update status

- 更新した。

## tasks.md update status

- 更新した。

## samples_progress.md update status

- 更新不要。runnable active sample root / validation command / dashboard status はこの package では変えていない。

## Skipped validations and reasons

- full validation floor は未実行。今回の変更は docs-first maintenance と current command evidence refresh に限定し、sample taxonomy や Rust implementation 自体は変更していないため、focused validation に留めた。

## Commit / push status

- report authoring 時点では pending。
- package close commit / push はこの report 生成後に実施する。

## Sub-agent session close status

- reviewer session (`019dde5f-fc36-7432-906c-ee7aff7c50f7`): completed, findings 3 件を回収したうえで close 済み。
- inherited `Lovelace` session (`019dde40-4989-7692-a419-0ffffbae9a89`): close 済み。
