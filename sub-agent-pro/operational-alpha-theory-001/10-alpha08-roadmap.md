# 10 — α-0.8 same-session hot-plug runtime roadmap

## goal

α-0.8 は same-session hot-plug runtime である。

```text
α-0.5 session + attach package/layer/object -> verdict -> activation/defer/reject -> observable state change
```

## operational completion condition

α-0.8 complete と呼ぶには、次を満たす。

1. α-0.5 session に対して attach command/API がある
2. debug layer attach accepted
3. non-admin debug attach rejected before state mutation
4. auth layer accepted only through explicit contract update
5. rate-limit layer accepted with declared failure and observable runtime rejection
6. incompatible patch rejected before activation
7. stale membership attach rejected
8. missing witness attach rejected
9. object or avatar placeholder package attach visible in session
10. unsupported runtime visible fallback
11. detach request gives explicit accepted/rejected/deferred boundary
12. hot-plug lifecycle appears in session-bound devtools
13. attach changes later observation / behavior in same session

## current floor

あるもの:

- practical hotplug plan/report
- HP-A1 rows
- LI rows
- AV-A1 avatar preview companion floor
- VIS-A1 hot-plug lifecycle export
- product preview companion bundles

不足:

- same-session attach execution
- detach runtime lifecycle
- object attach affecting runtime state
- debug attach causing subsequent session export change
- rate-limit affecting subsequent session action in same session
- final package/avatar ABI

## recommended next implementation after α-0.5

`P-A1-21 α-0.8 same-session hot-plug runtime`

Commands should eventually look like:

```bash
python3 scripts/practical_alpha08_hotplug_session.py start samples/practical-alpha1/packages/run-01 --format json
python3 scripts/practical_alpha08_hotplug_session.py attach <session-id> samples/practical-alpha1/packages/hp-a1-01-debug-layer --format json
python3 scripts/practical_alpha08_hotplug_session.py observe <session-id> --panel hotplug --format json
python3 scripts/practical_alpha08_hotplug_session.py run-action <session-id> roll --format json
python3 scripts/practical_alpha08_hotplug_session.py detach <session-id> debug-layer --format json
```

## required samples

| ID | Purpose | Current evidence | Missing for operational α-0.8 |
|---|---|---|---|
| A08-01 | debug attach accepted | HP-A1-01 / LI-01 | same-session mutation |
| A08-02 | non-admin attach rejected | HP/LI rows | same-session rejection |
| A08-03 | auth contract update | HP-A1-03 / LI-03 | same-session semantics |
| A08-04 | rate-limit declared failure | HP-A1-04 / LI-04 | same-session behavior change |
| A08-05 | incompatible patch reject | HP-A1-05 / LI-05 | same-session rejection |
| A08-06 | stale membership attach reject | HP-A1-04B1 | session attach path |
| A08-07 | missing witness attach reject | HP-A1-04B2 | session attach path |
| A08-08 | object attach preview | HP-A1-06 | actual visible session representation |
| A08-09 | unsupported runtime fallback | AV-A1-03 | same-session fallback view |
| A08-10 | detach minimal contract | HP-A1-07 | session lifecycle view |
| A08-11 | hotplug lifecycle export | VIS-A1-04 | session-bound devtools |

## theoretical requirements

- attach accepted implies activation event
- rejected attach does not mutate session state
- deferred detach is not accepted detach execution
- auth/rate-limit contract update is explicit
- debug layer has authority/redaction/retention
- layer behavior change is observable after activation

## stop line

- hot-plug report alone is not same-session hot-plug
- product preview bundle is not runtime mutation
- deferred detach is not detach lifecycle completion
- native execution remains later

