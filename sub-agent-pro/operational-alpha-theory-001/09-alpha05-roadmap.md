# 09 — α-0.5 local observable runtime roadmap

## goal

α-0.5 は local observable runtime である。

```text
package input -> check -> runtime plan -> local runtime session -> observe -> save/load
```

## operational completion condition

α-0.5 complete と呼ぶには、次を満たす。

1. practical package input を読む
2. checker report を出す
3. runtime plan を作る
4. local runtime session を起動する
5. session id / state carrier を得る
6. event DAG を観測する
7. membership / capability / witness rejection を state mutation 前に確認する
8. local save/load resume が動く
9. stale membership non-resurrection が動く
10. typed host-I/O minimal demo が動く
11. observer-safe devtools export がある
12. README / hands-on で新規開発者が再現できる

## current floor

あるもの:

- package.mir.json front-door
- first checker floor
- first local runtime floor
- first local save/load floor
- first devtools export floor
- product preview floor

不足:

- same-session runtime carrier
- host-I/O direct semantic execution lane
- unified developer workflow
- session-bound observe/save/load commands
- local missing capability / missing witness runtime negatives as part of same workflow

## recommended next implementation after theory freeze

`P-A1-19 α-0.5 session runtime carrier`

Implement or specify:

```text
PracticalAlphaSession
  session_id
  package_id
  checker_report_ref
  runtime_plan_ref
  runtime_state
  membership_frontier
  witness_store_summary
  visible_history
  event_dag
  attached_layers
  savepoints
```

Commands should eventually look like:

```bash
python3 scripts/practical_alpha05_session.py start samples/practical-alpha1/packages/run-01 --format json
python3 scripts/practical_alpha05_session.py observe <session-id> --format json
python3 scripts/practical_alpha05_session.py save <session-id> --format json
python3 scripts/practical_alpha05_session.py load <savepoint-id> --format json
```

The exact command names can be changed, but one coherent workflow is required.

## required samples

| ID | Purpose | Current evidence | Missing for operational α-0.5 |
|---|---|---|---|
| A05-01 | accepted local dispatch | RUN-01 / LR-01 | session-bound workflow |
| A05-02 | stale membership reject | RUN-02 / LR-02 | session-bound workflow |
| A05-03 | missing capability reject | transport/layer floor | local session negative |
| A05-04 | missing witness reject | transport/layer floor | local session negative |
| A05-05 | fallback degradation visible | LIF/VIS floor | runtime session event |
| A05-06 | local save/load resume | SL-A1-01 | session-bound command |
| A05-07 | stale membership non-resurrection | SL-A1-02 | session-bound command |
| A05-08 | event DAG export | VIS-A1-01 | session export flow |
| A05-09 | observer-safe view | VIS-A1-06 | session export flow |
| A05-10 | typed host-I/O EchoText/AddOne | missing | new lane |

## validation expectation

When operational α-0.5 is implemented, validation should include:

```bash
python3 scripts/practical_alpha05_session.py check-all --format json
python3 scripts/practical_alpha05_session.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha05_session
```

This does not exist yet; P-A1-18 should only document it.

## stop line

- first local runtime report alone is not α-0.5 complete
- product preview bundle alone is not α-0.5 complete
- no stdio builtin
- no distributed save/load claim

