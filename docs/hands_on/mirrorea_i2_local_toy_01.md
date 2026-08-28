# Mirrorea I2 local toy walkthrough

This walkthrough follows the accepted SYS-5 local toy fabric at implementation
cut `53a21e64b5a17e24b522f720db10b6e539c058e0`. It is LAB guidance; Canon
remains under `mirrorea_canon/`.

## 1. Read the source

The source is:

```text
samples/clean-near-end/mirrorea-i2-local-toy/main.mir
```

It declares four loci:

- `WorldAuthority`
- `ParticipantA`
- `ParticipantB`
- `ViewerC`

The sample includes owner-side avatar mutation, a designated evaluator
published by `WorldAuthority`, a `bird_follow` maintained relation owned by
`ParticipantB`, and consumer-local projection at `ViewerC`.

## 2. Build/project the loci

```bash
cargo run -q -p mir-runtime --bin mir -- project-loci \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --format json
```

The command reports `source_authority = ordinary_mir_source`, four loci, 13
per-locus fragments, and 12 generated communication edges. The edge kinds are
owner request/reply, designated input request/receipt, designated result
delivery, and relation projection publication.

## 3. Run the local fabric

```bash
cargo run -q -p mir-runtime --bin mir -- run-local \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir \
  --format json
```

This executes the generated in-process dispatch workflow. The high-level
`actual_steps` include startup, participant attack, designated publish,
viewer consume, relation primary selection, save/restore, accepted and
rejected patch verdicts, `ParticipantA` leave, presentation-only gap, fresh
reacquire, capability revocation, failed consume, and verification.

The two patch paths intentionally diverge:

- `designated-plus-two.mir` is accepted because it only changes the bounded
  designated result expression.
- `owner-rmw-change.mir` is rejected with `OwnerRmwExpressionChanged`.

## 4. Inspect the causal view

```bash
cargo run -q -p mir-runtime --bin mir -- inspect \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir \
  --format json
```

`inspect` emits one observer-safe joined view. It is designed so a reader can
follow source span, Core operation, per-locus artifact, generated communication
edge, runtime occurrence, owner mutation, relation fallback, designated result
version, save/restore, patch lifecycle, and authority failure without manually
joining several files.

The observer-safe report does not expose raw credentials, capability secrets,
or witness payloads.

## Non-claims

The current CLI spelling and JSON field layout are provisional. This
walkthrough does not freeze a public API, public ABI, wire format, final
grammar, real socket transport, multi-process runtime, browser renderer,
production deployment, or official I2 lifecycle exit.
