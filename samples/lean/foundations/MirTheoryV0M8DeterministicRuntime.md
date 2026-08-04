# MirTheoryV0M8DeterministicRuntime.lean

## Summary

This is the fresh, self-contained finite M8 checked-artifact/runtime handoff
model.  It models an immutable checked artifact, a typed and source-bound
runtime-admission input, deterministic lowering, and one finite save-relevant
runtime configuration.  It does not import or identify Rust, M5, or M7
carrier types.

## Exact finite coverage

- `CheckedProgramIdentity` contains the retained program identity, static
  environment, checked evaluation/Core shape, effect/obligation shape, stable
  source-to-Core map, and residual-source-reference rows.  The theorem named
  `checked_program_identity_covers_static_environment_evaluation_effect_obligation_and_source_map`
  checks those fields against the concrete owner, relation, and designated
  artifacts.
- The finite admission family accepts the residual-free owner profile, the
  exact relation payload `⟨SourceRef, label/redaction, declared lease
  reference, binding frontier, primary/fallback epoch⟩`, and the exact
  designated `⟨SourceRef, label/redaction⟩` payload. Admission checks that
  declared payload only: wrong source, label, lease reference, frontier, or
  epoch, duplicate/conflicting residual evidence, wrong redaction, or
  mismatched identity rejects before installation. It does not consult live
  lease inventory. `AuthDeferred` and `VerifyDeferred` yield `DeferredToM9`
  with no semantic mutation.
- Lowering preserves the finite source-map order: owner request/local-read/
  write; owner relation publication/consumer-local projection; or designated
  request/receipt-use/value publication.  It does not parse source, classify
  source, or reconstruct a map by name.
- The one finite `K8` configuration covers admitted identity/evidence and the
  active retained checked-plan set, owner FIFO/store,
  membership/capability/witness/lease context, relation/designated stores,
  local cut, patch lifecycle, and one unified occurrence trace `H`. Its fixed
  owner schedule reaches `100 → 90 → 80`; `H` retains the authority/witness/
  write facts and the stale-witness rejection/failure facts.
- After relation installation, the operation-time gate rechecks the exact live
  inventory tuple `⟨relation, owner, declared lease reference, live,
  binding frontier, lease epoch⟩` for consumer projection, relation transition,
  and re-acquire. Missing, expired, or mismatched inventory rejects that
  operation without changing its prior admission result. The selected
  re-acquire rejects a reused witness and installs only the declared distinct
  fresh witness with its exact fresh inventory owner/frontier/epoch tuple and
  new binding/epoch/lineage. A private local fallback retains the admitted
  private label and redaction. The selected accepted patch atomically installs
  its checked artifact, identity, admission, and lowered plans at the local
  cut; rejected/deferred patches retain the semantic snapshot. `H` carries
  internal raw authority/witness/capability/failure payloads, while observer
  projection exports only typed structural rows with explicit label/redaction.

## Evidence boundary

The file provides the exact finite Lean evidence recorded as OBL-050--056 in
`mirrorea_canon/theory/11-metatheory-ledger.md`. OBL-057 is now
`runtime-monitored`: the current bounded typed/source-bound Rust M8 route and
fixture matrix passed 53 focused M8 tests, full runtime/semantics all-target
checks, format/clippy, raw-public-API absence and observer label/redaction
scans, Canon/hierarchy/docs checks, and the trusted 28-theorem axiom-free Lean
check. This is bounded validation correspondence, not a Rust theorem or a
claim of general implementation equivalence. The file does not prove a general
admission/scheduling/DAG/cut/patch theorem, M9 authorization or verification
semantics, M10 or SCN conformance, transport or receipt delivery, or a public
API/ABI/wire contract.

## Compile

```bash
lean --trust=0 samples/lean/foundations/MirTheoryV0M8DeterministicRuntime.lean
```

The file prints dependency checks for its selected theorems.  It declares no
user `axiom` and uses no `sorry` or `admit`.
