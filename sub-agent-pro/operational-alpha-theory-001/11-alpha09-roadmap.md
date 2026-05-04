# 11 — α-0.9 session-bound devtools roadmap

## goal

α-0.9 は session-bound devtools である。

Exact reports are useful, but operational readiness requires observation derived from session state or an explicitly session-linked carrier.

## completion condition

1. event DAG export from session
2. route trace export from transport/session
3. membership timeline from session frontier
4. witness relation panel
5. hot-plug lifecycle panel
6. fallback degradation panel
7. save/load timeline panel
8. observer-safe redacted view
9. admin/debug view or explicit kept-later marker
10. on-demand trace control
11. retention query / policy evidence
12. HTML or viewer output that new developer can open

## current floor

あるもの:

- VIS-A1-01..07 devtools export first floors
- exact report-derived bundles
- non-final viewer surface

不足:

- live/session-bound export path
- witness timeline completion
- admin full view
- hotplug detach lifecycle timeline
- final viewer/telemetry API

## recommended commands later

```bash
python3 scripts/practical_alpha09_devtools_session.py export <session-id> --format json
python3 scripts/practical_alpha09_devtools_session.py render <session-id> --output /tmp/mirrorea-viewer.html
python3 scripts/practical_alpha09_devtools_session.py query-retention <session-id> <artifact-id> --format json
```

## stop line

- exact report bundle = final viewer service と書かない
- on-demand traceなしで production devtools と書かない
- observer-safe view に raw witness/auth を出さない

