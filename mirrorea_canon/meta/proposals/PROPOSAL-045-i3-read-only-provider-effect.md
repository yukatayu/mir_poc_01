---
id: meta/proposal-045
status: L1-fixed
maturity: reviewed
depends_on: [root/design-constitution, adr/ADR-0034, adr/ADR-0040, adr/ADR-0042, plan/05-i3-entry-contract, arch/07-browser-host-trust-boundaries, arch/08-browser-host-security-invariants, spec/17-i3-read-only-provider-effect]
summary: I3-3 provider failureへdistinct source effectと既存private transportを使う有限契約を選択する。実装受理ではない。
open_items: []
---

# PROPOSAL-045 — Read-only provider effect for I3-3

Direct consumer: I3-3 failure family 18 and effect-order correspondence.
Blocker reduced: current accepted owner RMW, expiry and authentication paths
cannot truthfully produce a declared external-effect result/failure.
Acceptance use: checked source -> generated remote invocation -> admitted
read-only host call -> typed outcome -> checked requester consumption.

## Authority and disposition

Integration baseline: `55f1fd7f76b86a2fc6a846c0133d55d5c8213831`, clean main
with remote parity observed `2026-09-08T12:16:45+09:00` (LAB evidence).
ADR-0034 authorizes bounded source/runtime/transport/Canon decisions for this
program. Disposition: accepted as a bounded contract under that delegated
authority after independent review and parent integration, recorded by
ADR-0042. This is not implementation acceptance or a new owner instruction.
Report 2606 and Plan 250 remain the only active
milestone record and roadmap. Complete I3-3, then honor the owner pause.

## Two candidates

**A — selected:** a distinct source-declared read-only effect at an
already remote locus, sharing the private transport/session/framing mechanism.
Generated effect request/result discriminants, checked fragments, effect-only
grants, actual invocation and typed consumption remain distinct from owner
RMW and its success receipt. A fixed T0 adapter reads a bounded nonsecret T4
fixture; no extra provider process/network leg or registry is needed.

**B — viable but deferred:** a dedicated remote-effect request/result service
and separate transport-facing family. This can preserve the same semantics,
but no current direct consumer requires a separate service, additional locus
or protocol. If shared framing cannot preserve A's exact distinctions, that
is a concrete falsifier for A, not permission to hide them in OwnerRequest.

Constitution C1/C3/C4/C9 favor A's source-first checked meaning, distinct
authority and external-effect boundary with the smallest topology. Reuse is
representational only; avoiding a necessary enum variant is not a goal.
Spec/17 supplies the finite resource, failure, retention and trust contract.

## Advisory inputs and acceptance gate

The existing Oracle consultation suggested a bounded T0/T4 read-only fixture
path. It is advisory, not authority. Its proposed requester terminal ambiguity
rule is not adopted: lost evidence retains the accepted pending/remote-unknown
state, with later genuine results subject to normal current-binding checks.
The Canon-first planner review confirms that representative success and real
provider failure must traverse selected QUIC, while checker/grant/resource
variants may have explicitly local evidence. It does not require a separate
provider OS process or permit an owner-grant/receipt shortcut.

Independent contract review found five P1 gaps in the initial draft; parent
integrated failure taxonomy, call-start ordering, provider incarnation,
enforceable resource admission and observation separation. Narrow re-review
has no remaining P0/P1/P2. Implementation still needs spec/17's positive/
falsifier matrix and separate review. No I3-3, lifecycle, proof, public or
Browser/Host product acceptance is
implied. Reopen for hidden invocation, stale grant/result reuse, provider-as-
authority, source-free edges/results, unbounded host access or false certainty.
