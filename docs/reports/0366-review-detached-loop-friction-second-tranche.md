# Report 0366 — review detached loop friction second tranche

- Date: 2026-04-09T04:22:00Z
- Author / agent: Codex
- Scope: `0365` の detached validation loop friction second tranche に対する closeout review
- Decision levels touched: L2

## 1. Objective

- detached validation loop friction second tranche の差分が current non-production helper boundary を壊していないかを closeout 前に確認する。
- `compare-fixture-aggregates` の追加が aggregate compare API の premature publicization になっていないかを見る。

## 2. Scope and assumptions

- 対象は `scripts/current_l2_detached_loop.py`、`scripts/tests/test_current_l2_detached_loop.py`、`specs/examples/26`、`specs/examples/28`、`plan/07`、`plan/09`、`plan/90`、`plan/91`、`AGENTS.md`、`tasks.md`、`progress.md`、`0365` report に限る。
- final exporter API、final path policy、reference update flow の導入までは今回扱わない。

## 3. Review method

- reviewer subagent に、working tree の未commit差分を対象として
  - helper boundary 逸脱
  - focused tests の十分性
  - docs / plan / task / progress mirror drift
  を確認してもらった。
- あわせて local verification evidence を再確認した。

## 4. Findings

### Finding 1 — `compare-fixture-aggregates` は current helper boundary に収まっている

- `scripts/current_l2_detached_loop.py`
- current helper は temporary single-fixture directory を内部で作り、existing aggregate emitter / diff helper を再利用しているだけであり、`lib.rs` / `harness.rs` の public aggregate API を増やしていない。

### Finding 2 — focused tests は second tranche の意図を十分に固定している

- `scripts/tests/test_current_l2_detached_loop.py`
- fixture stem shorthand、`<stem>-single` default label、`.host-plan.json` sidecar copy、compare path derivationを 1 本の test で押さえており、薄い helper convenience としては十分である。

### Finding 3 — mirror / provenance / maintenance rule の drift は見当たらない

- `specs/examples/26-current-l2-detached-aggregate-compare-helper.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `AGENTS.md`
- `tasks.md`
- `progress.md`
- second tranche の helper cut、`tasks.md` の phase column rule、progress log は互いに矛盾していない。

## 5. Result

- substantive finding は無かった。
- current tranche は narrow helper improvement として close してよい。

## 6. Open questions

- next friction を `reference update / bless` と `longer compare triage` のどちらから取るか。
- `compare-fixture-aggregates` の run label policy を今後 generalize する必要があるか。

## 7. Suggested next prompt

detached validation loop friction reduction の次 tranche として、reference update flow と longer compare triage のどちらを先に薄く改善するかを narrow comparison してください。
