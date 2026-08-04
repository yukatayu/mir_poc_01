---
id: scenarios/SCN-11
status: L1-fixed
maturity: reviewed
depends_on: [theory/13-evaluation-materialization, scenarios/SCN-02]
summary: designated evaluator が frontier ごとに versioned value を決定し、consumer は再評価しない M3 pressure scenario。
open_items: []
---

# SCN-11 — Designated evaluator / versioned materialization

**Purpose**: make an authoritative semantic decision separate from presentation
and from an owner mutation.

```text
at logical frontier F, designated evaluator E decides r under policy p
and publishes value v as version n; consumer C consumes (E, r, F, n).
```

**Expected (C-static)**: elaboration emits an `eval` Core item in its designated
form with
`computation / designated(E) / logical-tick-or-frontier-advance /
admitted-evaluator(E) / publish-value`, explicit policy and input frontier. A
missing frontier is `E-EVAL-FRONTIER`; a consumer attempt to re-evaluate `r`
is rejected rather than silently choosing a new decision site.

**Expected (C-runtime)**: at fixed `(E, r, F)`, service decides and publishes
one versioned value, but does not consume it. The explicit `C` consumption of
that decided version is a subsequent occurrence with C's identity; duplicate
publication or duplicate consumption does not create a second semantic
consumption. Failure appends a typed failure occurrence and publishes no
success value. Presentation interpolation at C may use the decided value but
changes neither the semantic value nor its version.

**Negative variants**: (a) omit F; (b) have C semantically recompute r;
(c) publish success after E fails; (d) produce two values/versions for the
same E/r/F; (e) let provider or consumer mutation stand in for E's decision.
**Refs**: ADR-0018, theory/13, theory/05, theory/07. Save/load and stale
consumption behavior are SCN-10/M8 work, not claimed by this scenario.
