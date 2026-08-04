---
id: scenarios/SCN-12
status: L1-fixed
maturity: reviewed
depends_on: [scenarios/readme, theory/14-maintained-relation-projection]
summary: maintained bird relation の relation-first publication、C-local projection、fallback と split-frame reject を圧する M4 pressure scenario。
open_items: []
---

# SCN-12 — Bird relation projection

**Purpose**: bind M4's maintained relation to an application-shaped case
without promoting its vocabulary into Core. The following is Core-level
scenario notation, not M6 Surface grammar.

```text
loci A, B, C
B owns relation bird:
  bird follows A.shoulder + offset
  semantic fallback B.shoulder

BindingState(bird) = primary(A.shoulder), lineage λ, epoch β,
                     witness ω, activation-frontier F
publish-relation(B → C, bird, λ, β, F, derived-label)

C receives the projected relation and admitted A.shoulder/B.shoulder
presentation samples at F. Each sample is release-admitted for C at its exact
anchor/epoch/frontier/label before C evaluates bird locally at presentation-frame.
```

**Expected positive case**: B is the relation/binding owner. C is not given a
bird absolute-pose stream. With A and B samples at the same frontier F and
their expected epochs, C computes `A.shoulder + offset`; this equals the
finite owner relation evaluation, and the output label is the greatest
restriction of the bird relation and both input labels. If A's semantic anchor
becomes invalid, B records a primary → fallback degradation on λ; later reads
on λ use B.shoulder. Only an explicit fresh witness and binding epoch starts a
new lineage that can again select A.shoulder. Every B binding mutation requires
the validated owner capability and witness lineage for λ/β.

**Presentation case**: packet loss, latency, interpolation, prediction, or LOD
at C may render a local gap/fallback. It does not mutate B's `BindingState`,
advance λ, create a semantic occurrence, or make an old A sample admissible.

**Negative variants**:

- A relation dependency cycle is rejected.
- A sample with A's old epoch is rejected as stale.
- A frame combining A at F and B at F+1 is rejected as split-frame.
- C attempting to mutate B's relation/binding is rejected.
- A private anchor input requested for public release is rejected.
- An offset or derived coordinate outside the finite transform domain, including
  overflow, is rejected without fallback or wraparound.
- Replacing `publish-relation` with an absolute bird-pose value or adapter
  stream is outside this scenario's accepted M4 route.

**Refs**: ADR-0019, THM-002, OBL-035--039, theory/04, theory/06, theory/07,
theory/09, theory/14.
