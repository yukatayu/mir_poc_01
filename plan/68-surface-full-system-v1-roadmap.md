# plan/68 — Surface Full System V1 roadmap

## purpose

This document is repository memory for the Surface Mir alpha line defined by
`specs/43-surface-mir-v1-alpha-scope.md`.

It sits above the closed Full System V1 bounded release-check line and below any
final public grammar / ABI decision.

## current recognition

The repo already has:

- Product Alpha-1 bounded local/Docker alpha workflow.
- operational product suite workflow-ready in a narrow showcase.
- Full System V1 source-first parser / typed checker / runtime / operational
  suite / PoseGraph / projection / provider / release-check evidence closed
  through final audit.

The repo does not yet have:

- canonical Surface Mir brace parser.
- Surface-to-Core elaboration over `S { ... }`.
- indexed-state semantics implemented from Surface source.
- role admission / source patch hot-plug from Surface source.
- Surface Mir source operational suite.

## package order

| Order | Package | Role | Close condition |
|---:|---|---|---|
| 1 | `P-SURF-00B` | docs/spec rebaseline | docs, specs, plans, guides, report, validators |
| 2 | `P-SURF-01` | parser | `S { ... }` place blocks and `Role[instance] { ... }` role-instance blocks accepted; bare role blocks and `S[ ... ]` rejected |
| 3 | `P-SURF-02` | indexed state | owner/keyspace/access/stale semantics represented |
| 4 | `P-SURF-03` | elaboration | cross-locus reads/writes generate Core IR |
| 5 | `P-SURF-04` | auto communication | MessageEnvelope / publish / observe / failure rows visible |
| 6 | `P-SURF-05` | role admission | role claim / grant / spoof / stale rows |
| 7 | `P-SURF-06` | source patch | parse/typecheck/elaborate/admit/activation-cut pipeline |
| 8 | `P-SURF-07` | source operational suite | WorldCore / MembershipChat / Sugoroku / related roots |
| 9 | `P-SURF-08` | devtools | source/Core/generated-edge/patch/admission panels |
| 10 | `P-SURF-99` | final audit | full validation and compatibility anchors |

## planned root family

Planned, not created by P-SURF-00B:

```text
samples/full-system-v1-surface/
```

This root should remain planned until implementation packages create runnable
rows and helper validation.

## compatibility anchors

P-SURF packages must preserve:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

These are compatibility anchors, not proof that Surface Mir is already
implemented.

## stop lines

- no `S[ ... ]` syntax or sugar.
- no final public grammar / ABI / SDK claim.
- no direct LLVM/native codegen claim.
- no production WAN/federation claim.
- no distributed durable save-load R3/R4 claim.
- no arbitrary native/WASM provider execution claim.
