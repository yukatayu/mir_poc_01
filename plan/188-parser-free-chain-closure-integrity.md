# Plan 188 - Parser-free chain closure integrity correction

## Role and authority

This is LAB implementation-integrity memory. `mirrorea_canon/` remains the
normative source. It records a fail-closed correction in the existing
parser-free current-L2 lane; it does not amend theory, a Core form, an OBL,
Gate, Phase, scenario, runtime contract, or public interface.

## Trigger

The post-PROPOSAL-012 theory frontier screen deliberately excluded the
owner-reserved value-flow, receipt, occurrence-identity, totality, and
proof-interface choices. An independent code review then found a separate,
already-settled representation invariant:

```text
head = primary
edges = [mirror -> archive]
```

where all three declared options have matching target and capability. Before
this correction, the parser-free static gate accepted that input. The evaluator
constructed `[primary, archive]` by collecting the head and every edge
successor, silently ignoring `mirror` as a predecessor.

## Canon reading

`theory/06-existence-fallback.md` fixes a guarded reference as a finite,
left-to-right chain over one logical access path and writes it as
`o1 > o2 > ... > on`. Its canonical normalization preserves order and requires
each later stage to be a monotone degradation of its predecessor. The Surface
`ChainDecl` grammar in `spec/02-surface-grammar.md` has the same sequential
shape: a head followed by `>` option references.

This establishes ordered predecessor-to-successor continuity for the existing
parser-free representation. It does not decide whether repeated option names,
strict versus non-strict degradation, cycles, a new chain grammar, or a new
diagnostic ABI are admissible. Those questions are outside this correction.

## Root cause and bounded correction

`static_gate_program_detailed` already checked each edge's endpoints,
edge-local lineage assertion, declared access target, and capability direction.
It did not compare an edge predecessor with the head or the preceding edge's
successor. `resolve_chain_order` then used the declared head and only edge
successors.

The correction keeps the existing ordered `Vec<ChainEdge>` representation and
adds one static condition after both edge endpoints resolve: an edge predecessor
must equal the expected predecessor, initially the chain head and thereafter
the preceding successor. Missing-head and missing-endpoint paths retain their
pre-existing diagnostics without additional continuity findings. The new
finding stays unclassified by the existing stable reason-code carrier.

## Evidence and validation

Three synthetic regressions cover the base, recurrence, and diagnostic
preservation conditions: the disconnected one-edge input above; a two-edge
input whose second predecessor does not equal the first successor; and a
syntactically connected sequence that crosses an undeclared option. The first
two failed before the correction because the static verdict was `Valid`; they
now require `Malformed` with the literal continuity reason. The third showed
that an early missing endpoint could leave the expected predecessor stale and
add a later continuity finding. The gate now advances the expected predecessor
from each raw successor while only checking continuity after both endpoints
resolve, so that the third case retains only its pre-existing missing-endpoint
diagnostics. The detached static-gate artifacts for the continuity cases keep
`detached_noncore` absent, so the new wording does not extend the existing
stable reason-code carrier. The existing parser-free semantics suite, including
source-shaped sequential chains and missing-head/endpoint diagnostics, remains
green.

This is a concrete existing-lane implementation consumer: both the fixture
bundle harness and current-L2 source-runtime skeleton run the static gate before
evaluation. No WRK is created because the package repairs a direct LAB
representation/integrity defect rather than registering a reversible L3 theory
proposition.

## Non-claims and reopen boundary

The correction does not claim THM-002, OBL-005/006/007/008, final chain syntax,
general parser correctness, evaluator correctness, runtime conformance, or
public readiness. A future question about any additional graph or option
admissibility property must be screened independently; this record must not be
used to infer a broader chain-algebra decision.
