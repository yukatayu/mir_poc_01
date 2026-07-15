# plan/155 - T0/G0 governance-profile adoption and evaluation

## status

LAB repository-memory and execution-routing note. `PROPOSAL-002` was adopted
through `adr/ADR-0013` in canon revision `edc2a36ce5145a897bf0e7a0e8788cf69216b2c3`.
The one-off derived result is
`plan/155-t0-g0-governance-profile-evaluation.json`. It is LAB evidence, not a
tool, SCN conformance output, proof, Gate exit record, or implementation state
change.

## owner decisions applied

- **G0-D1 accepted:** the exact five ADRs, `root/glossary` baseline, and
  `LAB:plan/153` evidence are accepted at the pinned repository evidence cut.
- **G0-D2 adopted:** `phase-governance/t0-g0` version 1 is defined by
  `plan/01-phases` and remains separate from `arch/03-toolchain`'s SCN
  `mir-conform` tool contract.
- **Production boundary:** one ephemeral evaluation produced exactly this one
  JSON artifact. No producer implementation, reusable helper, schema, CI,
  Make target, evidence lane, or report series was committed.
- **G0-D3 deferred:** G0 exit and T1 entry remain unapproved.
- **G0-D4 waived:** no separate semantic/historical LAB-demotion audit is
  required at this checkpoint; concrete future drift can reopen it.

## evaluation result

The artifact evaluates canon revision
`edc2a36ce5145a897bf0e7a0e8788cf69216b2c3`, the direct parent of the commit
that records it. It binds the profile definition, ADR-0013, the accepted
repository evidence cut, `meta/source-hierarchy`, five concrete LAB controls,
all three checks, and an RFC 8785-based self digest.

| Check | Result |
| --- | --- |
| `g0-substantive-owner-record` | pass |
| `g0-source-hierarchy-controls` | pass |
| `g0-demotion-audit-scope` | pass |
| profile result | pass |

This `pass` is necessary T0 profile evidence only. It cannot accept itself,
exit G0, enter T1, make an additional ADR effective, establish C-static /
C-runtime / C-distributed conformance, or establish a proof/runtime/product
claim.

## review and validation record

Oracle's first independent review found the original pre-commit draft too
ambiguous about the Phase JSON rule, adoption revision, JSON shape/digest, and
source-control pinning. The canon patch was narrowed accordingly. A follow-up
browser review ended before returning; the one allowed independent retry found
six additional protocol-precision defects. They were incorporated before the
adoption commit. Local validation recomputed every accepted-cut and control
hash from Git blobs, validated the artifact's shape/order/repeated evidence/
digest, rebuilt the canon index, ran source-hierarchy and documentation guards,
the focused highlighter tests, and `make check`.

## remaining stop line

G0-D3 remains the only next Gate decision: the owner must either continue to
defer G0 exit or separately accept this exact artifact digest and record the
canonical exit. Until that occurs, canon remains T0. No autonomous T1 package
is promoted by this note or by the `pass` result.

## non-claims

This package does not establish G0 exit, T1 entry, C-static/C-runtime/
C-distributed conformance, theorem/proof completion, a runnable sample, an
implementation tool, a public grammar/API, or a product/runtime state.
