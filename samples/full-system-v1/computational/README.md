# Full System V1 Computational Samples

This root carries the first source-first Full System V1 computational rows.

- `src/*.mir` is the semantic source input.
- `expected/parse.json` stores the parser-lane subset consumed by `scripts/textual_mir_samples.py`.
- `expected/check.json` stores the typed-checker subset consumed by `scripts/full_system_v1_samples.py`.
- `expected/run.json` stores the source-derived interpreter/runtime subset consumed by `scripts/full_system_v1_samples.py`.
- `package.mir.json` is not required in this root; runtime/package artifact generation remains later work.

Current scope:

- `P-MIR-01`
  - positive parse acceptance for pure computation and host-boundary syntax
  - negative parse rejection for malformed grammar rows
- `P-MIR-02`
  - positive typed-check acceptance for pure computation, record rows, and host-boundary evidence
  - negative typed-check rejection for:
    - unresolved import
    - ambiguous import resolution
    - semantically broken imported module closure
    - return type mismatch
    - unbound variable
    - statically provable array bounds failure
    - undeclared effect
    - missing effect failure row
    - undeclared capability requirement
- `P-MIR-03`
  - source-derived runtime acceptance for add-one, lexical scope, arrays, records, control-flow, and imports
  - explicit split between static rejection and runtime rejection
  - observer-safe compute traces from the pure interpreter lane
- `P-MIR-04`
  - transition/effect runtime acceptance for host read/write and one bounded Sugoroku-like publish/observe/witness/handoff/cut chain
  - runtime rejection for missing publication, missing live witness, violated local `R2` cut preconditions, rollback-across-cut, and stale-state non-resurrection
  - observer-safe effect-session summaries in `expected/run.json`

Non-claims:

- no final public grammar
- no final typed IR or public runtime API
- no final effect grammar or public effect ABI
- no distributed or durable cut/save execution beyond bounded local rejection evidence
- no generated package artifact yet
