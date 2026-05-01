# 17 — Risk register

This file lists high-risk mistakes Codex should actively avoid.

## 1. Lifetime/fallback risks

### R-001 — Treating fallback as target lifetime extension

Wrong:

```text
c fallback a makes c live as long as a
```

Correct:

```text
c fallback a creates a guarded access path that degrades from c to a.
```

### R-002 — Implicit fallback-chain propagation

Wrong:

```text
c = d fallback a automatically inherits d's internal chain.
```

Correct:

```text
inherit/splice requires explicit evidence.
```

### R-003 — Capability promotion via fallback

Reject any fallback that moves to a stronger capability as ordinary degradation.

### R-004 — Mutable covariance

Reject Java-like mutable variance problems.

## 2. Layer/contract risks

### R-101 — Auth/rate-limit as transparent overlay

Auth/rate-limit often add failure outcomes or strengthen preconditions. They are only transparent if the original contract already permits them.

### R-102 — Debug leaks

Debug layers can leak more than auth layers. Require labels, authority, redaction, retention.

### R-103 — Hidden shadowing

Overlay must not hide an interface by returning untyped no-service or silently dropping requests.

## 3. Cut/save/load risks

### R-201 — Making atomic_cut global

Never expand `atomic_cut` to distributed/durable/global commit.

### R-202 — Save/load without consistent cut

Reject distributed save/load without prefix-closed cut predicate.

### R-203 — Ignoring in-flight messages

Distributed save must account for channel state/in-flight messages.

### R-204 — Ignoring Z-cycles

A checkpoint dependency cycle can make checkpoints unusable. Add negative samples.

### R-205 — Resurrecting stale facts

Load must not resurrect expired lease, stale membership, or stale witness.

## 4. Runtime package/avatar risks

### R-301 — Avatar format as core primitive

Do not make VRM/VRChat/Unity/core avatar format part of Mir core.

### R-302 — Signature equals safety

Signature is provenance only. Need sandbox/effect/capability/resource policy.

### R-303 — Opaque adapter overclaim

If adapter behavior is opaque, say which guarantees remain outside Mir semantics.

### R-304 — Shader/native complexity too early

Keep shader/native execution deferred or skeleton-only during alpha planning.

## 5. Product scope risks

### R-401 — Mirrorea = Reversed Library

Wrong. Reversed Library is an upper application.

### R-402 — Mirrorea alpha = full VRChat

Wrong. Alpha is substrate + minimal demo.

### R-403 — PrismCascade creep

Keep PrismCascade separate.

### R-404 — Final parser too early

Do not freeze syntax before typed IR/checker/cut/layer theory.

## 6. Repository risks

### R-501 — Normative claims in handoff only

If a decision is normative, put it in `specs/`.

### R-502 — Planned sample marked active

Do not mark skeletons active/runnable until they run.

### R-503 — Validation overclaim

Do not claim skipped tests pass.

### R-504 — No report/commit/push

Every non-trivial package needs report, validation, commit, and push.
