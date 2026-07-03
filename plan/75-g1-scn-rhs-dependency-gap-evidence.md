# plan/75 - G1 SCN RHS dependency-gap LAB evidence

## Purpose

This file records the LAB evidence package that closes the main G1 static
dependency-evidence gap identified in `plan/72`: SCN-01 same-field RHS reads
and SCN-02 target/self RHS reads are now represented as explicit dependency
rows in Surface-to-Core elaboration output.

This is LAB repository memory. It does not edit canon, does not claim C-static
conformance, does not discharge THM-001 / OBL-001, does not prove G1 exit, and
does not define final read materialization or runtime dispatch.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB gap inventory: `plan/72-g1-scn01-scn02-static-consequence-drilldown.md`
- LAB statement draft: `plan/74-g1-obl001-lean-statement-draft.md`
- LAB implementation evidence:
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- LAB fixture evidence:
  `samples/full-system-v1-surface/elaboration/elab-11-scn01-rhs-dependency-positive/`
  and
  `samples/full-system-v1-surface/elaboration/elab-12-scn02-two-read-dependency-positive/`

If this LAB evidence conflicts with canon, canon wins.

## What changed

`SurfaceCoreIr` now has `dependencies`, a LAB carrier for static dependency
evidence. The new row kind is:

```text
rhs_indexed_read
```

The row records:

- `write_request_id`, linking the dependency to the generated owner-directed
  write request;
- requester and owner loci;
- state name, key expression, and field name;
- source access text;
- generated-from reason;
- source span.

The dependency row is intentionally not a remote read request, message
envelope, observe row, publication row, runtime occurrence, cache policy, reply
policy, freshness policy, or final Core JSON/API contract.

## New evidence rows

| Row | Purpose | Evidence |
|---|---|---|
| `ELAB-11` | SCN-01-shaped `player[self].position = player[self].position + draw` records one same-field RHS dependency while visible write publish/observe remains explicit. | one write request, one `rhs_indexed_read` dependency, publish, observe |
| `ELAB-12` | SCN-02-shaped `player[target].hp = player[target].hp - player[self].atk` records both RHS dependencies without forcing observer-safe read materialization. | one write request, two `rhs_indexed_read` dependencies, no publish/observe |

Existing `ELAB-02` and `ELAB-09` also now expose dependency summaries for their
RHS indexed reads.

## Why dependency rows, not remote reads

Remote read lowering currently carries observer-safe communication behavior and
visibility checks. SCN-02 has no visible declaration. Treating all RHS
dependencies as remote read/observe requests would conflate dependency tracking
with observer-safe publication and would prematurely freeze OPEN-014.

The safe G1 evidence is therefore:

```text
remote write request + explicit RHS dependency rows
```

not:

```text
remote write request + forced runtime/observe read materialization
```

## Status

- `scripts/surface_mir_samples.py check-all --format json` validates 48 Surface
  rows, including `ELAB-11` and `ELAB-12`.
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
  validates 15 Rust tests, including direct dependency assertions.
- Surface `.mir` source file count under `samples/full-system-v1-surface/` is
  now 49.

## Remaining boundaries

- No C-static conformance pass is claimed.
- No C-runtime request serving, store mutation, or occurrence order is claimed.
- No C-distributed transport behavior is claimed.
- No OBL-001 or OBL-002 proof status moved.
- No OBL-020 / OBL-021 status moved.
- No OPEN-014 materialization policy is frozen.
- No final Core IR JSON/API is frozen.

## Next safe packages

The first item below was actualized in
`plan/76-g1-obl020-021-dependency-inventory.md`.

1. OBL-020/021 dependency inventory, kept separate from OBL-001 proof work.
2. OBL-001 statement refinement only if `THM001StatementDraft.lean` should
   mention the concrete LAB `rhs_indexed_read` carrier as evidence, without
   importing it into canon.
3. Separate OBL-020 / OBL-021 LAB Lean statement-shape drafts, only if useful.
4. Negative diagnostic alignment package for canon E-ROW-001 / E-ROW-002 versus
   LAB `generated_failure_not_declared`, still without diagnostic ABI freeze.

## Non-claims

- No G0 exit.
- No G1 exit.
- No T1 transition.
- No OBL status movement in canon.
- No Lean proof completion.
- No theorem discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No runtime MessageEnvelope dispatch.
- No final grammar, final Core IR JSON, public API, runtime, transport,
  projection, devtools, telemetry, provider, or product completion.
