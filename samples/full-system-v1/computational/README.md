# Full System V1 Computational Samples

This root carries the first source-first `P-MIR-01` textual Mir alpha grammar rows.

- `src/*.mir` is the semantic source input for the parser lane.
- `expected/*.json` stores the expected parse verdict subset consumed by `scripts/textual_mir_samples.py`.
- `package.mir.json` is not required for `P-MIR-01`; runtime/package artifact generation remains later work.

Current scope:

- positive parse acceptance for pure computation and host-boundary syntax
- negative parse rejection for:
  - unresolved import
  - malformed function signature
  - missing type annotation
  - malformed record field
  - malformed `perform ... via ...` boundary
  - malformed `transition ... at ...` entrypoint
  - malformed capability requirement
  - contract clause outside allowed position

Non-claims:

- no final public grammar
- no typed IR or runtime execution yet
- no generated package artifact yet
