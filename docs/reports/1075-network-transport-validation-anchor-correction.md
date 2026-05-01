# Report 1075 — network transport validation anchor correction

- Date: 2026-05-01 10:56 JST
- Author / agent: Codex
- Scope: validation anchor / dashboard maintenance
- Decision levels touched: none; evidence wording and validation command mirror only

## Objective

Correct the network transport validation anchor so executable canary validation uses `check-all --format json`, while `closeout --format json` is documented as inventory-only evidence.

## Scope and assumptions

- Scope is limited to validation command wording, dashboard/progress/task mirrors, and a historical correction note in report `1066`.
- `scripts/network_transport_samples.py` behavior was not changed.
- Stop line: this package does not claim production socket, broker, crypto session protocol, durable replay, continuous shared runtime state, or final public transport ABI.

## Documents consulted

- `scripts/network_transport_samples.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/reports/1066-post-1065-full-validation-checkpoint.md`
- Reviewer findings for the network transport validation anchor mismatch

## Actions taken

- Confirmed from `scripts/network_transport_samples.py` that `check_all()` executes `NET-02..05`, while `closeout()` returns inventory metadata.
- Ran `python3 scripts/network_transport_samples.py check-all --format json`.
- Ran `python3 scripts/network_transport_samples.py closeout --format json` as inventory evidence.
- Updated `tasks.md` network transport executable-floor command and validation floor from `closeout` to `check-all`.
- Updated `progress.md` validation anchor from `closeout` to `check-all` and added a recent-log correction.
- Updated `samples_progress.md` network transport row timestamp/report reference and recent validation row.
- Added a correction note to report `1066`, preserving historical evidence while marking the network transport line as inventory-only.

## Files changed

- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1066-post-1065-full-validation-checkpoint.md`
- `docs/reports/1075-network-transport-validation-anchor-correction.md`

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git log -1 --oneline
3575ddf Compact progress recent log
```

Implementation inspection:

```text
$ sed -n '400,470p' scripts/network_transport_samples.py
def check_all() -> dict[str, Any]:
    failed: list[dict[str, str]] = []
    for row in SAMPLE_ROWS:
        try:
            run_sample(row["sample_id"])
...
def closeout() -> dict[str, Any]:
    return {
        "active_sample_root": str(ACTIVE_SAMPLE_ROOT),
...
        "validation_floor": (
            "helper-local canaries plus loopback_socket Sugoroku parity; "
            "not real socket/broker/session/replay runtime"
        ),
```

Executable canary validation:

```text
$ python3 scripts/network_transport_samples.py check-all --format json
{
  "sample_count": 4,
  "passed": [
    "NET-02",
    "NET-03",
    "NET-04",
    "NET-05"
  ],
  "failed": [],
  "transport_scope": "helper_local_process_boundary",
  "transport_seam": "loopback_socket"
}
```

Inventory evidence:

```text
$ python3 scripts/network_transport_samples.py closeout --format json
{
  "active_sample_root": "/home/yukatayu/dev/mir_poc_01/samples/clean-near-end/network-transport",
  "sample_count": 4,
  "samples": [
    "NET-02",
    "NET-03",
    "NET-04",
    "NET-05"
  ],
  "transport_scope": "helper_local_process_boundary",
  "transport_seam": "loopback_socket",
  "validation_floor": "helper-local canaries plus loopback_socket Sugoroku parity; not real socket/broker/session/replay runtime",
  "limitations": [
    "no real network socket yet",
    "no cryptographic session protocol",
    "no durable distributed commit",
    "no continuous shared runtime state across processes",
    "final public transport ABI remains deferred"
  ]
}
```

Targeted search after patch:

```text
$ rg -n 'network_transport_samples.py (closeout|check-all)|network transport|E2E-TRANSPORT|16-command full floor|inventory-only' progress.md tasks.md samples_progress.md docs/reports/1066-post-1065-full-validation-checkpoint.md scripts/README.md samples/README.md
tasks.md and progress.md now use `check-all --format json` for validation anchors.
samples_progress.md already used `check-all` for the summary and E2E rows and now has a fresh 10:56 validation row.
docs/reports/1066... keeps the historical closeout line but now includes a correction note.
```

Docs-focused validation:

```text
$ python3 scripts/check_source_hierarchy.py
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 35
  present: 35
  missing: 0
  all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1073 numbered report(s).

$ git diff --check
<no output>
```

## What changed in understanding

The previous full validation checkpoint included an inventory-only `closeout` command for network transport. The executable canary command is `check-all`, and that distinction must remain explicit to avoid overstating the network transport evidence.

## Open questions

- Real socket / broker / crypto session protocol / durable replay remain deferred.
- Actual `U1` commitment remains open and user-facing.

## Suggested next prompt

Continue autonomous maintenance: run docs-focused validation after this report, commit/push, and then reassess whether any safe maintenance work remains before stopping on `U1`.

## Plan update status

`plan/` 更新不要: validation anchor wording changed only; no roadmap or long-lived boundary memory changed.

## progress.md update status

`progress.md` 更新済み: validation anchor and recent log were updated.

## tasks.md update status

`tasks.md` 更新済み: network transport validation command was updated.

## samples_progress.md update status

`samples_progress.md` 更新済み: network transport row and recent validation row were updated.

## Skipped validations and reasons

- Full validation floor was not rerun because package `1066` recorded a fresh full validation checkpoint and this package focused on correcting/rerunning the affected network lane.
- Cargo tests and unrelated sample closeouts were not run because no code, non-network samples, runner behavior, or generated artifacts changed.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

Reviewer sub-agent `Confucius` identified the network transport validation anchor mismatch and was closed before patching.
