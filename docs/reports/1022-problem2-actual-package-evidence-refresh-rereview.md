# 1022 — Problem 2 actual-package evidence refresh re-review

## Objective

Re-review `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md` after the `NET-03` wording narrowing, and determine whether any findings remain.

## Scope and assumptions

- Review scope was limited to `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`.
- This was a wording re-review, not an implementation task.
- I checked the updated wording against the current active network-transport landing page and helper source.
- `plan/` update unnecessary.

## Documents consulted

- `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
- `samples/clean-near-end/network-transport/README.md`
- `scripts/network_transport_samples.py`

## Actions taken

- Inspected the updated `NET-03` evidence row and `p08` split-reading sentence with line numbers.
- Cross-checked the wording against the active network-transport landing page and helper implementation summary for `NET-03`.

## Files changed

- Added `docs/reports/1022-problem2-actual-package-evidence-refresh-rereview.md`
- `progress.md` update unnecessary
- `tasks.md` update unnecessary
- `samples_progress.md` update unnecessary

## Evidence / outputs / test results

- In `specs/examples/467...`, line 86 now says `stale reconnect / membership-epoch guard canary`, which matches the current active transport wording.
- In `specs/examples/467...`, line 159 now says `current stale-reconnect canary (NET-03) and family check`, which preserves the intended partial/split replacement reading.
- `samples/clean-near-end/network-transport/README.md` describes `NET-03` as a `reconnect epoch guard canary`.
- `scripts/network_transport_samples.py` defines `NET-03` as rejecting reconnect when stale `membership_epoch / member_incarnation` crosses the process boundary.

## What changed in understanding

- The earlier overstatement is resolved.
- The file now distinguishes the narrow `NET-03` canary from the broader `p08` historical combined reading sufficiently for this review scope.

## Open questions

- None for this re-review scope.

## Suggested next prompt

No follow-up required for `specs/examples/467` on the reviewed `NET-03` / `p08` wording unless you want a broader drift scan across nearby Problem 2 docs.
