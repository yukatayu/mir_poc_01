# Mirrorea I2 finite conformance input

This LAB sample supplies the separate ordinary `.mir` source selected for the
bounded OW1 correspondence row in SYS-6. The complete four-locus local toy is
an ST workflow; this source has exactly one combined semantic owner/source-
owner locus so ST and OW1 can execute the same accepted fragment.

Run the accepted finite profile:

```bash
cargo run -q -p mir-runtime --bin mir -- conform-i2 \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --selected-ow1-source samples/clean-near-end/mirrorea-i2-conformance/ow1-selected-owner-designated.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir \
  --format json
```

For the accepted cut `5429712de89a7e41c46cfd7fb4a39c4a492864c4`, the canonical
inputs produce an observer-safe report with exactly 22 passing rows. The
selected OW1 row compares typed result, state, frontier, and trace while
retaining the primary toy's typed whole-workflow OW1 ineligibility.

This directory is source input, not generated output. The command, JSON,
identity encoding, and row names are internal/provisional. The runtime report
cannot authorize lifecycle transitions; ADR-0032 separately records official
I2 entry then exit. This sample does not select transport, activate I3, freeze
a public contract, or claim production/general proof.
