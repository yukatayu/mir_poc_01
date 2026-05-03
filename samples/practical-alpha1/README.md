# samples/practical-alpha1

This root is the current practical alpha-1 front-door plus first checker-floor and first local-runtime sample family.

- It is separate from `samples/alpha/`, which remains the alpha-0 evidence root.
- It is not yet the active runnable root for the whole repo.
- `P-A1-01` introduces the first narrow front-door cut here:
  directory-or-file package loading for limited `package.mir.json` inputs.
- `P-A1-02` adds the first checker-floor cut here:
  package fixtures with `alpha_local_checker_input` plus expected checker reports.
- `P-A1-03` adds the first local-runtime cut here:
  checked package fixtures with `alpha_local_runtime_input` plus expected local-runtime reports.
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

## Current boundary

- The current front-door reads only `package.mir.json`.
- Textual `.mir` source remains later work.
- The current checker floor is non-final and checker-only.
- The current local-runtime floor is also non-final and is limited to `RUN-01/02`.
- Checked packages are lowered through a distinct runtime-plan carrier before local runtime execution.
- It does not complete the full `specs/18` typed-checking list.
- It does not complete package/hot-plug practical API, Docker/local TCP transport, local save/load command, or final public runtime/devtools ABI.
