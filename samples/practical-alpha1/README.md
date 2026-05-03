# samples/practical-alpha1

This root is the planned practical alpha-1 sample family.

- It is separate from `samples/alpha/`, which remains the alpha-0 evidence root.
- It is not yet the active runnable root for the whole repo.
- `P-A1-01` introduces the first narrow front-door cut here:
  directory-or-file package loading for limited `package.mir.json` inputs.
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

## Current boundary

- The current front-door reads only `package.mir.json`.
- Textual `.mir` source remains later work.
- Typed checker integration, runtime plan execution, transport, save/load,
  and devtools are later packages in the practical alpha-1 line.
