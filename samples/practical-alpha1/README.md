# samples/practical-alpha1

This root is the current practical alpha-1 front-door plus first checker-floor, first local-runtime, and non-final practical hot-plug sample family.

- It is separate from `samples/alpha/`, which remains the alpha-0 evidence root.
- It is not yet the active runnable root for the whole repo.
- `P-A1-01` introduces the first narrow front-door cut here:
  directory-or-file package loading for limited `package.mir.json` inputs.
- `P-A1-02` adds the first checker-floor cut here:
  package fixtures with `alpha_local_checker_input` plus expected checker reports.
- `P-A1-03` adds the first local-runtime cut here:
  checked package fixtures with `alpha_local_runtime_input` plus expected local-runtime reports.
- `P-A1-04a` adds the first practical hot-plug cut here:
  layer package fixtures with `alpha_local_hotplug_input` plus exact expected hot-plug reports.
- `P-A1-04b` widens the same practical hot-plug cut here:
  attach-time stale-membership reject, attach-time missing-witness reject, and a narrow object package attach preview seam.
- `P-A1-04c` widens the same practical hot-plug cut here:
  explicit deferred detach minimal contract with `operation_kind = detach` and `detach_boundary_ref`.
- This front-door is non-final and does not freeze the final public grammar.

## Current package map

- `packages/src-01-minimal-world/`
  - `SRC-01`: minimal world parses
- `packages/src-02-fallback-chain/`
  - `SRC-02`: fallback chain source parses
- `packages/src-03-layer-debug/`
  - `SRC-03`: layer attach source parses
- `packages/src-04-layer-manifest/`
  - `SRC-04`: package manifest source parses
- `packages/src-05-invalid-syntax/`
  - `SRC-05`: invalid syntax rejected
- `packages/chk-*/`
  - `CHK-LIF-01..04`: first lifetime/fallback checker floor
  - `CHK-VAR-01..03`: first contract-variance checker floor
  - `CHK-CUT-01`: first cut-predicate checker floor
  - `CHK-PKG-01/02`: checker-only package-admission preview floor
- `packages/run-*/`
  - `RUN-01`: checked practical Sugoroku-style local dispatch with event DAG export
  - `RUN-02`: stale-membership rejection before local state mutation
- `packages/hp-a1-*/`
  - `HP-A1-01`: debug layer attach accepted through manifest-driven hot-plug plan
  - `HP-A1-02`: non-admin debug attach rejected before activation cut
  - `HP-A1-03`: auth layer accepted only through explicit contract-update path
  - `HP-A1-04`: rate-limit layer accepted with explicit preview rejection evidence
  - `HP-A1-05`: incompatible patch rejected before activation cut
  - `HP-A1-04B1`: stale-membership attach rejected before activation cut
  - `HP-A1-04B2`: missing-witness attach rejected before activation cut
  - `HP-A1-06`: object package attach admitted only as a narrow preview seam
  - `HP-A1-07`: detach admitted only as an explicit deferred minimal contract boundary

## Current boundary

- The current front-door reads only `package.mir.json`.
- Textual `.mir` source remains later work.
- The current checker floor is non-final and checker-only.
- The current local-runtime floor is also non-final and is limited to `RUN-01/02`.
- The current hot-plug floor is also non-final and is currently limited to `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, and `HP-A1-07`.
- Checked packages are lowered through a distinct runtime-plan carrier before local runtime execution.
- Checked layer packages are lowered through a distinct hotplug-plan carrier before hot-plug report assembly.
- Object package preview still goes through the distinct hotplug-plan carrier and keeps `object_attach_claimed = false`.
- It does not complete the full `specs/18` typed-checking list.
- It does not complete final object package attach, detach runtime lifecycle, Docker/local TCP transport, local save/load command, or final public runtime/devtools ABI.
