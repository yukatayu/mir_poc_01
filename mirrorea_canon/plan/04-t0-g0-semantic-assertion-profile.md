---
id: plan/04-t0-g0-semantic-assertion-profile
status: L1-fixed
maturity: draft
depends_on: [root/design-constitution, adr/ADR-0013, adr/ADR-0015]
summary: T0/G0 semantic-assertion profile v3。revision-bound な意味 assertion と deterministic artifact の contract を定め、mutable reader-facing whole-file hash を control にしない。
open_items: []
---

# 04 — T0/G0 semantic-assertion profile v3

## Purpose and status

`phase-governance/t0-g0` version `3`, kind
`semantic-assertion-evaluation`, is the current T0 profile. It is a bounded
governance evaluation, not SCN conformance, a proof, a runtime test, an I1
authorization, or a public contract.

The profile evaluates one committed `source_revision`. Its result is `pass` or
`fail`; malformed input, unknown revision, an invalid artifact shape, or a
wrong digest is consumer rejection, never a valid `fail`. Version-1 and
version-2 artifacts remain byte-preserved historical evidence under ADR-0013;
they cannot be regenerated, re-pinned, renamed, or treated as v3 evidence.

## Input and assertion contract

The deterministic producer is
`scripts/evaluate_t0_semantic_assertions.py --revision <40-hex-commit>`. It
reads only Git blobs at that revision. The revision is the reproducible evidence
cut; no working-tree value participates.

The following ordered assertions are the complete v3 semantic contract. A
selector sees only its listed semantic witnesses. Whole-file SHA-256 values of
mutable reader-facing documents are neither a pass predicate nor a control pin.

| ID | Selector | Semantic witnesses |
| --- | --- | --- |
| SA-01 | canonical-source-and-lab-evidence-notices | `CANON.md`, root `README.md`, source hierarchy |
| SA-02 | meaning-before-communication-interface | North Star, Constitution C3 |
| SA-03 | world-room-avatar-game-are-not-core-primitives | ADR-0001, Constitution C2 |
| SA-04 | canon-is-normative-lab-is-evidence | ADR-0012, source hierarchy |
| SA-05 | clean-suite-explicitly-lab-evidence | `samples/clean-near-end/README.md` |
| SA-06 | agents-canon-first-read-order | root `AGENTS.md` |

The producer emits an ordered row with `id`, `subject_refs`, `selector`,
`result`, and `normalized_finding` for every assertion. `result` is `pass` iff
every row passes; otherwise it is `fail`. Therefore an unrelated prose change
that leaves each selected predicate true does not itself become semantic drift.

## Artifact and validation contract

The fresh artifact is LAB evidence at
`plan/248-t0-g0-semantic-assertion-v3-evaluation.json`. It contains profile
identity/version/path/source revision/source hash, producer path/hash, the six
ordered rows, derived root result, non-claims, and `artifact_digest`.

For digesting, omit `artifact_digest`, serialize the parsed artifact as UTF-8
compact JSON with lexicographically sorted object keys, then SHA-256 the bytes.
This narrow `sorted-keys-utf8-compact-json-v1` rule is profile-local and does
not select a general public serialization format. Re-running the producer on
the same source revision must reproduce the artifact bytes. A consumer verifies
the source revision, ordered row IDs, derived root result, and digest before it
can rely on an artifact.

## Acceptance boundary

Profile adoption alone changes no lifecycle state. A `pass` artifact is only
evidence until the canonical record below names its exact digest, source
revision, and producer re-evaluation. Under ADR-0015 and the owner-approved M2
scope, that same record may then accept G0-D3, G0 exit, and T1 entry in this
order. The acceptance must state that SCN/C-static/C-runtime/C-distributed,
proof/OBL, runtime implementation, I1 authorization, public API/ABI/wire, and
deployment remain non-claims.

## M2 acceptance record

The fresh v3 `pass` artifact is
`LAB:plan/248-t0-g0-semantic-assertion-v3-evaluation.json`, generated from
source revision `644ec1cdfa7d69600af3463ab60a6b7d745913c8`. Its profile source
SHA-256 is `45cf5da9c7be03de89a47f74d37d3454c7c9029e0ac59bb7538f7c11b5974f37`,
its declared producer SHA-256 is
`1234566706bcfda51279bb6d0a88176be85c84f812e7f368171bc0ae6070a59f`, and its
canonical artifact digest is
`b32bd2c87e1dc77ca2a4f7a7426cda0bff8bcbf80155d19addd7db3a8288aa23`.

The artifact validator checked its digest, declared source binding, and exact
canonical-byte reproduction from that revision. Focused negative controls
reject clean-suite/agent-hierarchy selector drift and digest tampering. An
independent M2 review found no unresolved P0/P1 counterevidence.

Under the owner-approved M2 scope in ADR-0015 and ADR-0017, the ordered
acceptance is now applied exactly once:

```text
v3 pass digest accepted
→ G0-D3 accepted
→ G0 exit accepted
→ T1 entry accepted
```

This moves the official lifecycle to `T1` only. It does not claim
SCN/C-static/C-runtime/C-distributed conformance, proof/OBL completion, runtime
implementation, I1 authorization, public API/ABI/wire, or deployment.
