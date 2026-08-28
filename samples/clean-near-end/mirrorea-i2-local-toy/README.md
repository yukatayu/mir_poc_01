# Mirrorea I2 local toy

This is LAB evidence for SYS-5. The normative source remains
`mirrorea_canon/`; this directory is a small ordinary Mir source and two source
patches used to exercise the accepted local toy fabric cut
`53a21e64b5a17e24b522f720db10b6e539c058e0`.

## What it shows

- `main.mir` declares four loci: `WorldAuthority`, `ParticipantA`,
  `ParticipantB`, and `ViewerC`.
- `project-loci` derives per-locus executable plans and generated
  communication from the checked source.
- `run-local` executes the generated in-process dispatch workflow under the
  bounded SYS-5 ST profile.
- `inspect` emits one joined, observer-safe source -> Core -> artifact ->
  communication -> runtime view.
- `designated-plus-two.mir` is accepted as a designated-result-only patch.
- `owner-rmw-change.mir` is rejected with `OwnerRmwExpressionChanged`.

`WorldAuthority`, `ParticipantA`, `ParticipantB`, `ViewerC`, `Player`, and
`Bird` are sample/library vocabulary. They are not Mir Core primitives.

## Minimal workflow

```bash
cargo run -q -p mir-runtime --bin mir -- project-loci \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --format json

cargo run -q -p mir-runtime --bin mir -- run-local \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir \
  --format json

cargo run -q -p mir-runtime --bin mir -- inspect \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir \
  --format json
```

Expected high-level result:

- `project-loci` reports four loci, 13 locus fragments, and 12 generated
  communication edges.
- `run-local` reports actual steps including `attack`, `designated_publish`,
  `relation_primary`, `save`, `restore`, `patch_accepted`, `patch_rejected`,
  `participant_a_leave`, `presentation_gap`, `fresh_reacquire`, and
  `failed_consume`.
- `inspect` reports joined row kinds including `source_span`, `core_operation`,
  `per_locus_artifact`, `generated_communication_edge`, `runtime_occurrence`,
  `typed_causal_segment`, `relation_selected_fallback`, `save_cut`,
  `restore_cut`, `patch_lifecycle`, and `authority_failure`.

## Non-claims

This is not a public API, public wire format, final grammar, real transport,
multi-process runtime, durable distributed save/load, production View/browser
renderer, arbitrary relation-DAG theorem, or official I2 lifecycle acceptance.
The CLI and JSON fields are provisional internal SYS-5 surfaces.
