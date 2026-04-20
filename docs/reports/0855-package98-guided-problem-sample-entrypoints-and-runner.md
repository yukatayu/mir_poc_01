# 0855 — Package 98 guided problem sample entrypoints and runner

## 目的

Package 98 として、二大問題それぞれの representative sample を `samples/` 配下の日本語 guide と repo-local helper runner から辿れるようにし、docs / plan / progress / tasks / traceability の closeout を次の residual package へ繋ぐ。

## Scope and assumptions

- corrected prototype 自体は既存 sample を再利用し、duplicate sample file は増やさない。
- helper runner は repo-local / non-production cut に留める。
- final public CLI や final public tutorial surface はこの package で採らない。

## Documents consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md`
- `specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md`
- `samples/prototype/README.md`
- `samples/current-l2/README.md`
- `samples/prototype/current-l2-typed-proof-model-check/`
- `samples/prototype/current-l2-order-handoff/`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `samples/prototype/current-l2-typed-proof-model-check/README.md` を追加し、Problem 1 の representative / supporting sample を日本語で案内した。
2. `samples/prototype/current-l2-order-handoff/README.md` を追加し、Problem 2 の representative pair / reserve / negative sample を日本語で案内した。
3. `scripts/current_l2_guided_samples.py` を追加し、`list / show / run` helper を実装した。
4. guided runner の Problem 1 supporting sample に `p12-typed-classified-fingerprint-publication-block` を追加し、`run` 実行時の見出しが CLI 出力より先に見えるよう `flush=True` を入れた。
5. `scripts/tests/test_current_l2_guided_samples.py` を追加し、problem spec / guide text / primary command / `--all` command expansion を固定した。
6. `samples/prototype/README.md` と `samples/current-l2/README.md` の sample-facing wording を日本語寄りに整えた。
7. `specs/examples/572`、`Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/11`、`plan/16`、`plan/17`、`plan/18`、`plan/90` を guided sample entrypoint closeout と next residual package reading に同期した。

## Files changed

- `samples/prototype/README.md`
- `samples/current-l2/README.md`
- `samples/prototype/current-l2-typed-proof-model-check/README.md`
- `samples/prototype/current-l2-order-handoff/README.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

## Commands run

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
- `python3 scripts/current_l2_guided_samples.py list`
- `python3 scripts/current_l2_guided_samples.py show problem1`
- `python3 scripts/current_l2_guided_samples.py show problem2`
- `python3 scripts/current_l2_guided_samples.py run problem1 --format pretty`
- `python3 scripts/validate_docs.py`

## Evidence / outputs / test results

- `test_current_l2_guided_samples` は 4 tests 全て green。
- `list` は `problem1` / `problem2` と要約を表示した。
- `show problem1` は `p06` primary と `p10 / p11 / p12 / p15 / p16` supporting samples、`verification_preview`、`artifact_preview` を日本語で表示した。
- `show problem2` は `p07 / p08` representative pair、`p09` reserve、`p13 / p14` negative pair を日本語で表示した。
- `run problem1 --format pretty` は `p06-typed-proof-owner-handoff` を実際に流し、helper runner から current operational CLI summary へ到達することを確認した。
- `validate_docs.py` は `Documentation scaffold looks complete.` を返し、Package 98 closeout 後の snapshot / traceability drift がないことを確認した。

## What changed in understanding

- 二大問題の sample は新規 source file を増やさなくても、existing corrected prototype と Japanese guide / helper runner の組み合わせで十分に入口化できる。
- representative pair / reserve / negative pair の読み分けを `samples/` README に置くと、docs 側の current first line と operational evidence の距離がかなり縮まる。
- queue drift を戻さずに次 package へ進むには、docs-only closeout でも next residual package を明示しておく方が自然である。

## Open questions

- theorem / model-check public-seam residual bundle はまだ self-driven package として残る。
- witness/provider/public-shape residual bundle もまだ self-driven package として残る。

## Suggested next prompt

theorem / model-check public-seam residual bundle、または witness/provider/public-shape residual bundleのどちらかを次 package として進めてください。
