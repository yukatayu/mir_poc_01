# 12 — required sample matrix

This matrix defines samples needed for operational α-0.5 / α-0.8 / α-0.9. Many existing samples already provide first floors, but operational readiness requires coherent workflow and observation.

## α-0.5 samples

| Required ID | Scenario | Existing evidence | Needed next |
|---|---|---|---|
| A05-01 | accepted local dispatch | RUN-01, LR-01, Sugoroku | session-bound run |
| A05-02 | stale membership reject | RUN-02, LR-02 | session-bound negative |
| A05-03 | missing capability reject | NET / layer floors | local session negative |
| A05-04 | missing witness reject | NET / layer floors | local session negative |
| A05-05 | fallback degradation visible | LIF, VIS floors | runtime event + devtools |
| A05-06 | local save/load resume | SL-A1-01 | session command |
| A05-07 | local save/load stale membership non-resurrection | SL-A1-02 | session command |
| A05-08 | invalid distributed cut reject | CHK-CUT-01, SL-A1-03 | preflight command in workflow |
| A05-09 | event DAG export | VIS-A1-01 | session export |
| A05-10 | observer-safe view | VIS-A1-06 | session export |
| A05-11 | host input EchoText | missing | typed external direct execution |
| A05-12 | host input AddOne | missing | typed external direct execution |

## α-0.8 samples

| Required ID | Scenario | Existing evidence | Needed next |
|---|---|---|---|
| A08-01 | debug layer attach accepted | HP-A1-01, LI-01 | same-session attach |
| A08-02 | debug layer non-admin reject | LI-02 | same-session reject |
| A08-03 | auth layer contract update | HP-A1-03, LI-03 | same-session contract update |
| A08-04 | rate-limit declared failure | HP-A1-04, LI-04 | post-attach runtime effect |
| A08-05 | incompatible patch reject | HP-A1-05, LI-05 | same-session reject |
| A08-06 | stale membership attach reject | HP-A1-04B1 | session attach path |
| A08-07 | missing witness attach reject | HP-A1-04B2 | session attach path |
| A08-08 | object package attach preview | HP-A1-06 | visible session object |
| A08-09 | placeholder avatar attach | AV-A1-01 | session representation |
| A08-10 | custom Mir avatar preview | AV-A1-02 | session representation |
| A08-11 | unsupported runtime fallback | AV-A1-03 | session fallback view |
| A08-12 | deferred detach | HP-A1-07 | lifecycle event in session |
| A08-13 | hotplug lifecycle export | VIS-A1-04 | session-bound devtools |

## α-0.9 samples

| Required ID | Scenario | Existing evidence | Needed next |
|---|---|---|---|
| A09-01 | event DAG view | VIS-A1-01 | session-bound |
| A09-02 | route trace view | VIS-A1-02 | session-bound / transport-bound |
| A09-03 | membership timeline | VIS-A1-03 | session-bound |
| A09-04 | witness timeline | partial | explicit panel |
| A09-05 | hotplug lifecycle | VIS-A1-04 | session-bound |
| A09-06 | fallback degradation | VIS-A1-05 | session-bound |
| A09-07 | redacted observer view | VIS-A1-06 | session-bound |
| A09-08 | retention query | VIS-A1-07 | session-bound |
| A09-09 | admin/debug full view | missing or partial | explicit kept-later or implementation |
| A09-10 | HTML viewer openability | product preview / viewer | stable non-final viewer command |

## sample shape requirements

Each operational sample should include:

- package input or source input
- expected checker report
- runtime report or session state
- devtools export
- negative companion where applicable
- exact command sequence
- stop-line note

## anti-patterns

- `.expected.json` sidecar alone as completion
- absence of rejection as positive proof
- preview bundle as runtime execution
- report-local evidence as same-session state
- local-only save as distributed save

