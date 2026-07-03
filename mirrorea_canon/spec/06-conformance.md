---
id: spec/06-conformance
status: L1-fixed
maturity: draft
depends_on: [spec/05-runtime-semantics, scenarios/readme]
summary: SCN-01..10 を適合性基準として束ねる。適合レベルと合否判定。
open_items: []
---

# 06 — Conformance

The frozen suite scenarios/SCN-01..10 **is** the conformance definition.
Any change to theory or spec that alters an SCN expectation requires an ADR.

Levels:

- **C-static**: parse + check + elaborate all SCN sources; every positive
  elaborates with the expected edges/obligations; every negative yields the
  expected diagnostic id at the expected span. (PHASE-I1 entry.)
- **C-runtime**: run positives under the deterministic profile; occurrence
  rows, verdicts, store states at cuts match expectations; negatives that are
  runtime-class produce the expected explicit failures. (PHASE-I1 exit.)
- **C-distributed**: same expectations with ≥2 OS processes and real
  transport for SCN-01/02/03/06 (PHASE-I3 exit).

Pass = 10/10 at the claimed level, no expectation waivers. Partial claims must
enumerate failing SCN ids. Implementations report as
`conformance: {level, scn_pass: [...], scn_fail: [...], profile_hash}`.
