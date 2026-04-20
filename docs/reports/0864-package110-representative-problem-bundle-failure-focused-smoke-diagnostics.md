# 0864 Package 110 representative problem bundle failure-focused smoke diagnostics

## 1. Title and identifier

- 0864 Package 110 representative problem bundle failure-focused smoke diagnostics

## 2. Objective

- `smoke-all` helper に failure-focused diagnostics を actualize し、
  aggregate summary を壊さず failing point を compact に追えるようにする。

## 3. Scope and assumptions

- current target は `smoke-all` helper の failure diagnostics hardening に限定する。
- per-problem `smoke` command は残し、aggregate 側は representative verification loop の shortest entrypoint として読む。
- exhaustive workflow automation、aggregate CI contract、final public CLI / tutorial surface、final public parser / checker / runtime API は今回の scope に入れない。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/583-current-l2-representative-problem-bundle-aggregate-smoke-summary-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` の aggregate smoke row に
   `failed_command` / `failed_return_code` / `failed_output_excerpt`
   を追加した。
2. `smoke-all` が aggregate failure を隠さず、
   failed row があるときは non-zero exit を返すようにした。
3. RED/GREEN test を追加し、failure row の compact diagnostics と
   `main(["smoke-all"])` の non-zero return を固定した。
4. `samples/problem-bundles/README.md` に
   failure diagnostics と非ゼロ終了の読みを追記した。
5. spec / snapshot / traceability を Package 110 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/README.md`
- `specs/examples/584-current-l2-representative-problem-bundle-failure-focused-smoke-diagnostics-actualization.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## 7. Commands run

```bash
python3 -m unittest scripts.tests.test_current_l2_guided_samples
python3 -m unittest scripts.tests.test_problem_sample_bundles
python3 - <<'PY'  # injected failure on `bundle problem1`
python3 scripts/current_l2_guided_samples.py smoke-all
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - aggregate row が `failed_command` を持たない点、
    `main(["smoke-all"])` が常に 0 を返していた点で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - failure row の diagnostics fields と non-zero aggregate return を含めて通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle index から `smoke-all` failure diagnostics の読みを辿れるようになった。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py smoke-all`
  - current repo state では Problem 1 / Problem 2 とも passed であり、
    compact summary を維持したまま実行できた。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `failed_command` / `failed_return_code` / `failed_output_excerpt`
    を含む machine-readable row shape を返した。
- injected failure run:
  - `run_problem_smoke_aggregate(..., runner=fake_runner)` with `bundle problem1` forced to exit `7`
  - `failed step: bundle:problem1`、`failed command: python3 scripts/current_l2_guided_samples.py bundle problem1 --format json`、
    `failed return code: 7`、`failure excerpt: stderr: ... | stdout: ...` を含む rendered summary と `exit_code=1` を確認した。

## 9. What changed in understanding

- aggregate smoke helper は passed summary だけでなく、
  failure 時の compact diagnostics まで持たせることで
  representative verification loop の入口としてかなり使いやすくなった。
- long-form log retention を持ち込まなくても、
  failed step / command / return code / excerpt があれば
  repo-local once-through closeout line の追跡には十分だった。

## 10. Open questions

- long-form failure log retention や artifact bundling をどこまで持つかは still later。
- aggregate CI / installed-binary contract、final public CLI / tutorial surface は still later。
- theorem/model-check / witness-provider public contract は mixed gate に残る。

## 11. Suggested next prompt

- `samples/problem-bundles/problem1|problem2` の mini walkthrough を harden し、
  representative sample ごとの「何を見ればよいか」を smoke / matrix / bundle / parser companion の
  4 本で短く辿れる quickstart package を actualize してください。
