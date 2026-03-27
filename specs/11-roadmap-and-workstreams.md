# 11 — Roadmap and Workstreams

## Principle

The project should advance by stabilizing semantics before broad implementation.

## Recommended workstreams

### Workstream A — Mir specification core
Goals:
- settle the minimum formal semantics for Mir-0,
- write precise syntax and type rules for the currently agreed primitives,
- pin down cuts, effects, contracts, failure, and monotone ownership/lifetime behavior.

### Workstream B — Mir runtime proof-of-concept
Goals:
- build a single-process interpreter,
- support event graph extraction,
- support fallback/try/cut,
- generate trace artifacts.

### Workstream C — Mirrorea minimum fabric
Goals:
- logical naming,
- route rebinding,
- overlay registration,
- downstream patch activation,
- basic audit.

### Workstream D — PrismCascade minimal kernel
Goals:
- Meta/Core/Runtime separation,
- graph normalization,
- minimal scheduler and memory plan,
- minimal trace output.

### Workstream E — Shared integration surfaces
Goals:
- shared identifiers,
- shared contract schemas where appropriate,
- linked tracing strategy,
- minimal bridge between Mir and Prism.

### Workstream F — Visualization and editor support
Goals:
- language server basics,
- graph view,
- cut/route/patch visualization,
- report-driven workflow support.

### Workstream G — Application experiments
Goals:
- at least one small synchronized shared-space example,
- one small virtual-world or collaborative editing example,
- one small route/overlay insertion example.

## Recommended phase ordering

1. Workstream A
2. Workstream B
3. Workstream C
4. Workstream D
5. Workstream E
6. Workstream F
7. Workstream G

## Provisional implementation recommendations (not architectural laws)

- Rust for core runtimes, graph processing, and tooling backends.
- Separate native crates for Mir / Mirrorea / Prism components.
- Keep engine integration behind adapters.
- Prefer explicit schemas and versioned interfaces over implicit coupling.
