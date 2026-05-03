# 08 — Practical Alpha-1 Sample Matrix

## 1. Source / parser samples

- `SRC-01`: minimal world parses
- `SRC-02`: fallback chain source parses
- `SRC-03`: layer attach source parses
- `SRC-04`: package manifest source parses
- `SRC-05`: invalid syntax rejected

## 2. Checker samples

- `CHK-LIF-01`: raw dangling reject
- `CHK-LIF-02`: fallback access path valid
- `CHK-LIF-03`: inherited chain valid
- `CHK-LIF-04`: snapshot selected distinction
- `CHK-VAR-01`: logging layer valid
- `CHK-VAR-02`: precondition strengthening reject
- `CHK-VAR-03`: mutable covariance reject
- `CHK-CUT-01`: invalid distributed cut reject
- `CHK-PKG-01`: unsigned native reject
- `CHK-PKG-02`: over-capability reject

## 3. Runtime samples

- `RUN-01`: local Sugoroku-style world
- `RUN-02`: stale membership reject
- `RUN-03`: missing capability reject
- `RUN-04`: missing witness reject
- `RUN-05`: fallback degradation trace

## 4. Hot-plug samples

- `HP-A1-01`: debug layer attach
- `HP-A1-02`: non-admin debug attach reject
- `HP-A1-03`: auth layer explicit contract-update
- `HP-A1-04`: rate-limit layer declared failure
- `HP-A1-05`: incompatible patch reject
- `HP-A1-06`: object package attach
- `HP-A1-07`: detach-minimal reject/defer/fallback

## 5. Transport samples

- `TR-A1-01`: local TCP accepted envelope
- `TR-A1-02`: Docker two-node accepted world action
- `TR-A1-03`: stale membership over TCP reject
- `TR-A1-04`: observer-safe route trace
- `TR-A1-05`: auth lane separate from transport

## 6. Save/load samples

- `SL-A1-01`: local save/load resume
- `SL-A1-02`: local load stale membership reject
- `SL-A1-03`: invalid distributed cut reject
- `SL-A1-04`: local load does not resurrect fallback position

## 7. Devtools samples

- `VIS-A1-01`: event DAG viewer export
- `VIS-A1-02`: route trace viewer export
- `VIS-A1-03`: membership timeline
- `VIS-A1-04`: hot-plug lifecycle
- `VIS-A1-05`: fallback degradation
- `VIS-A1-06`: redacted observer view
- `VIS-A1-07`: retention/on-demand trace

## 8. Package/avatar samples

- `AV-A1-01`: placeholder avatar runtime
- `AV-A1-02`: custom Mir avatar runtime
- `AV-A1-03`: unsupported runtime fallback
- `AV-A1-04`: undeclared effect package reject
- `AV-A1-05`: unsigned native reject
- `AV-A1-06`: signed over-capability reject

## 9. Practical E2E samples

- `PE2E-01`: local full toolchain run from source
- `PE2E-02`: Docker full toolchain run from source
- `PE2E-03`: hot-plug debug layer during run
- `PE2E-04`: hot-plug package with avatar fallback
- `PE2E-05`: save/load local and continue
- `PE2E-06`: invalid distributed save rejected
- `PE2E-07`: devtools export and viewer open

## 10. Completion rule

A sample counts toward practical alpha-1 only if:

- it has source or manifest input, not only sample ID
- it goes through the declared practical toolchain path
- it has expected output / diagnostics
- it has positive/negative validation
- docs and dashboard are updated
