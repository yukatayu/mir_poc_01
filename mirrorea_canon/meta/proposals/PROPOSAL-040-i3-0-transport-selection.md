---
id: meta/proposal-040
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-037, adr/ADR-0034, adr/ADR-0036, plan/05-i3-entry-contract, arch/04-runtime-carriers]
summary: I3-0の同条件実process canaryと固定criteriaに基づき、private selected adapterをQUIC reliable streamとする提案。
open_items: []
---

# PROPOSAL-040 — I3-0 transport selection

## Owner disposition and direct capability

Under PROPOSAL-037 / ADR-0034, accept Candidate B, QUIC reliable stream, as the
private selected adapter for the bounded Mirrorea I3 Distributed
Foundation program. This concretizes the owner's delegated selection authority
without selecting a public wire or applying official I3 lifecycle entry.

```text
Direct consumer: I3-1 private transport-neutral encode/decode/admission boundary
Blocker reduced: OPEN-032 lacked equal executable evidence and one bounded choice
Acceptance use: let I3-1 target one real reliable stream while retaining a
  conservative transport replacement seam and every semantic admission rule
```

## Equal executable evidence

Both Candidate A and Candidate B, QUIC reliable stream, ran the same private,
source/Core-bound nine-case receiver-child canary over actual distinct local
operating-system processes:

1. connect without semantic admission;
2. deterministic fragmented round trip;
3. truncated frame;
4. oversized frame;
5. disconnect before admission;
6. disconnect after admission before result;
7. duplicate across reconnect;
8. tampered retained-contract fingerprint; and
9. observer-safe evidence.

The receiver child, rather than a coordinator expectation table, performs full
decode, exact retained-contract revalidation, bounded request-cache lookup and
handler linearization. The duplicate case records two receives and two
revalidations but one handler linearization and a stored decision. Both
candidates produced equal normalized semantic rows. This is bounded I3-0 canary
evidence, not an actual owner runtime, full plan/05 failure-matrix completion,
durability, exactly-once or a general network theorem.

## Lexicographic comparison and selection

Apply the I3-0 finite criteria derived from the Design Constitution in the
owner-fixed order. Criteria 1--7 tie within the nine-case canary: lossless
carrier transport, visible typed failure, deterministic local fault injection,
fail-closed partial/truncated/oversized input, explicit reconnect/duplicate/
reorder boundary, no hidden retry or exactly-once, and transport
non-authority. Neither candidate therefore wins on semantics or safety.

Criterion 8, implementation/library maturity, has no auditable winner in the
bounded evidence: both use the same Rustls trust stack and current maintained
async libraries, and candidate size is implementation simplicity rather than
maturity. Criterion 9 likewise has no tested winner beyond Linux x86_64
localhost. Criterion 10, future browser relevance, is therefore the first
material difference and favors QUIC reliable streams. Candidate B is selected.

Candidate A's bounded implementation is smaller (584 lines versus 732), and
warm focused measurements were 0.22 seconds and 44,052 KiB maximum resident set
size for A, versus 1.08 seconds and 43,944 KiB for B. Those facts are
lower-ranked performance/Design-Constitution C12 simplicity evidence. They do
not retroactively become maturity evidence and cannot override criterion 10.
Candidate A remains a rejected/deferred comparison and replacement baseline;
it is not a parallel active implementation queue. QUIC datagrams remain
excluded.

## Smallest alternative and rejection

The only viable alternative is to select Candidate A now. It preserves the
same bounded semantic rows and is smaller in this finite implementation, but
that is lower-ranked simplicity/performance evidence rather than a criterion-8
maturity win. Selecting it would skip the first actual fixed-order difference
at criterion 10. Retaining it as a replacement baseline preserves conservative
migration if QUIC later fails an I3-1 semantic or security gate.

## Falsifier and acceptance

The primary falsifier is that either candidate-derived metadata changes
semantic identity/authority, the receiver-child rows cease to be equal, a
partial or tampered frame is semantically admitted, duplicate/reconnect causes a
second handler execution, protected data appears in observer output, or the
selection depends on a lower-ranked benefit despite a higher-ranked failure.

Accept only with equal positive/falsifier evidence, an exact tested-platform
non-claim, independent semantic/security/distributed-systems review with no
P0/P1, and an I3-1 residual list for remaining bounded hardening.

## Non-effects

This proposal resolves OPEN-032 only for the active bounded program. It does not
freeze a public wire, codec, version, certificate representation, API, ABI,
deployment topology, platform support or production security claim. It does not
claim mutual TLS, client authentication, live membership/capability/witness
admission, actual owner-runtime execution, persistence, exactly-once, complete
ambiguous-delivery semantics, I3-1 completion or official I3 entry/exit. Theory
remains T1 and broad PHASE-I1 remains unaccepted.
