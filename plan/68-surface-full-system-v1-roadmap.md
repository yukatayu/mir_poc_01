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

- runtime MessageEnvelope dispatch or local queue delivery for generated
  communication rows.
- indexed-state runtime carrier.
- role admission / source patch hot-plug from Surface source.
- Surface Mir source operational suite.

The repo now has:

- canonical Surface Mir brace parser floor in `crates/mir-ast::surface_alpha`.
- `samples/full-system-v1-surface/syntax/` with `SURF-01..09` parser evidence.
- indexed-state semantic checker floor in `crates/mir-semantics::surface_indexed_state`.
- `samples/full-system-v1-surface/indexed-state/` with `IDX-01..05` checker
  evidence.
- Surface-to-Core elaboration floor in `crates/mir-semantics::surface_to_core_elaboration`.
- `samples/full-system-v1-surface/elaboration/` with `ELAB-01/02/04/05/06/07/08`
  elaboration evidence for cross-locus read/write requests, source spans,
  read/write generated failure-row rejection, unsupported-statement rejection,
  and nested-place read placement.
- the same `elaboration/` root with `ELAB-03/09/10` generated communication
  evidence for private/non-visible field rejection, visible field
  MessageEnvelope / publish / observe rows, and `VisibilityDenied` failure-row
  containment.
- `scripts/surface_mir_samples.py`, `scripts/surface_mir_authoring_check.py`,
  and `scripts/surface_mir_release_check.py` plan/check surfaces.

## package order

| Order | Package | Role | Close condition |
|---:|---|---|---|
| 1 | `P-SURF-00B` | docs/spec rebaseline | docs, specs, plans, guides, report, validators |
| 2 | `P-SURF-01` | parser | closed: `S { ... }` place blocks and `Role[instance] { ... }` role-instance blocks accepted; bare role blocks and `S[ ... ]` rejected |
| 3 | `P-SURF-02` | indexed state | closed: owner/keyspace/access/stale/compaction/nested-place guard semantics represented |
| 4 | `P-SURF-03` | elaboration | closed: cross-locus reads/writes generate Core IR remote request rows, generated edges, source spans, obligations, and underdeclared failure-row rejection |
| 5 | `P-SURF-04` | auto communication | closed: MessageEnvelope / publish / observe / failure rows visible |
| 6 | `P-SURF-05` | role admission | next: role claim / grant / spoof / stale rows |
| 7 | `P-SURF-06` | source patch | parse/typecheck/elaborate/admit/activation-cut pipeline |
| 8 | `P-SURF-07` | source operational suite | WorldCore / MembershipChat / Sugoroku / related roots |
| 9 | `P-SURF-08` | devtools | source/Core/generated-edge/patch/admission panels |
| 10 | `P-SURF-99` | final audit | full validation and compatibility anchors |

## planned root family

Created by P-SURF-01 as parser evidence:

```text
samples/full-system-v1-surface/
  syntax/
```

Created by P-SURF-02 as semantic checker evidence:

```text
samples/full-system-v1-surface/
  indexed-state/
```

Created by P-SURF-03/P-SURF-04 as elaboration and generated communication evidence:

```text
samples/full-system-v1-surface/
  elaboration/
```

These roots are not workflow-ready runtime evidence. Future implementation
packages should add sibling roots/rows only after role admission, runtime
dispatch, source patch, and operational surfaces exist.

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
