# samples/practical-alpha1

This root is the current practical alpha-1 front-door plus first checker-floor sample family.

- It is separate from `samples/alpha/`, which remains the alpha-0 evidence root.
- It is not yet the active runnable root for the whole repo.
- `P-A1-01` introduces the first narrow front-door cut here:
  directory-or-file package loading for limited `package.mir.json` inputs.
- `P-A1-02` adds the first checker-floor cut here:
  package fixtures with `alpha_local_checker_input` plus expected checker reports.
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

## Current boundary

- The current front-door reads only `package.mir.json`.
- Textual `.mir` source remains later work.
- The current checker floor is non-final and checker-only.
- It does not emit runtime plans or run local/Docker worlds.
- It does not complete the full `specs/18` typed-checking list.
- Runtime plan execution, transport, save/load, and devtools remain later packages.
