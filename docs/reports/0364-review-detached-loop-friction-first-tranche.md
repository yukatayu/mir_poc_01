# Report 0364 — review detached loop friction first tranche

- Date: 2026-04-09T04:07:00Z
- Author / agent: Codex
- Scope: `0363` の detached validation loop friction first tranche に対する closeout review
- Decision levels touched: L2

## 1. Objective

- detached validation loop friction first tranche の差分が current helper boundary を壊していないかを closeout 前に確認する。
- reviewer subagent がこの session では利用できないため、local diff inspection と targeted verification の fallback を明示する。

## 2. Scope and assumptions

- 対象は `scripts/current_l2_detached_loop.py`、その focused tests、`specs/examples/28`、`tasks.md`、`progress.md`、`0363` report のみとする。
- public API 化や aggregate compare policy の変更までは行わない。

## 3. Review method

- `git diff` で対象差分を再点検した。
- `scripts/current_l2_detached_loop.py` の call path を確認し、`emit-fixture` / `emit-static-gate` / `compare-fixtures` / `smoke-fixture` だけに run-label defaulting と fail-fast が入っていることを確認した。
- `scripts/tests/test_current_l2_detached_loop.py` の focused tests と shorthand smoke を evidence として用いた。

## 4. Findings

### Finding 1 — `resolve_fixture_argument` の導入は current helper scope に収まっている

- `scripts/current_l2_detached_loop.py`
- shorthand fixture stem を current fixture directory にだけ解決し、explicit path や `.json` suffix を持つ引数は direct path として扱うため、unexpected broad search にはなっていない。

### Finding 2 — run label omitted 時の defaulting は current bundle/aggregate path policy と整合している

- `scripts/current_l2_detached_loop.py`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- fixture stem を primary label にしても `bundle_artifact_path` / `aggregate_artifact_path` の current non-production discovery と矛盾しない。

### Finding 3 — next friction は aggregate-noise / reference update 側に残っているという report wording は妥当

- `progress.md`
- `tasks.md`
- `docs/reports/0363-detached-loop-friction-first-tranche.md`
- 今回の変更は path / label / typo triage の operational cost だけを下げており、aggregate compare の noisy diff や reference update flow 自体はまだ untouched である。

## 5. Result

- semantic inconsistency は見つからなかった。
- current tranche は narrow helper improvement として close してよい。

## 6. Open questions

- reviewer subagent が使える session では、次 tranche から再び external review path に戻すべきか。
- aggregate compare noise を suppress するのか、informational diff として整理するのか。
- reference bless / update helper を compare helper と同じ layer に置くのか。

## 7. Suggested next prompt

detached validation loop friction reduction の次 tranche として、aggregate compare noise、reference update flow、longer compare triage の 3 候補を narrow comparison してください。
