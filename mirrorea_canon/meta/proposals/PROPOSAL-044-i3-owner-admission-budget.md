---
id: meta/proposal-044
status: L1-fixed
maturity: reviewed
depends_on: [root/design-constitution, adr/ADR-0034, adr/ADR-0040, plan/05-i3-entry-contract, theory/02-types-effects-failures, spec/16-i3-owner-admission-budget, adr/ADR-0041]
summary: I3-3の時間failure consumerへ、明示source opt-inのowner admission budgetと型付きterminal failureを選択する。
open_items: []
---

# PROPOSAL-044 — Source-declared owner admission budget

Direct consumer: I3-3 row 17, owner-local expiry/serve ordering and the actual
two-process generated-request failure path. The later provider-failure row
may reuse the terminal-outcome separation, but is not selected here.

Blocker reduced: the accepted owner reply is success-only; an external wait
timeout neither establishes remote nonexecution nor supplies a declared Core
failure. The finite source path currently has no deadline condition.

Acceptance use: for an opted-in request reaching the owner gate, serialized
resolution reserves admission while `resolve_tick < d`, or commits the exact
declared `DeadlineExpired` at or after `d` without owner mutation. Reservation
does not guarantee actual serve or successful execution. The requester learns
the failure only from a validated delivered outcome; lost evidence remains
uncertainty. This proposal does not accept I3-3 or any whole failure family.

## Authority and disposition

Integration baseline: `31be54b9ec83b3a75a56f6d71a5523674237b035`.
ADR-0034 supplies owner-delegated authority for the bounded failure/ordering,
source/runtime and Canon changes. The latest owner instruction stops execution
after complete I3-3 acceptance, validation, review, commit/push and parity;
I3-4 requires explicit resume.

Disposition: **accepted as a bounded contract decision under ADR-0034's
owner-delegated authority**, after independent exact-diff review and parent
integration. ADR-0041 records that decision. It accepts no implementation or
milestone and invents no additional owner instruction.

## Two candidates and selection

**A: separately admitted outer lifecycle expiry contract.** This can preserve
an ordinary request's old failure row only with its own explicit authorized
ContractUpdate and a serialized owner decision. The existing M9 update
implements layer Attach/Remove, not request expiry, and restricted SYS5 has
no reachable such update path. An opaque bridge or origin reference cannot
supply the missing authorization rule. Retain A as deferred, not an active
implementation or a transparent overlay.

**B: new source opt-in and initial checked contract — selected.**
The source declares a bounded owner-admission time condition and its generated
`DeadlineExpired` failure. M6/M7 retain it as typed admission metadata alongside
the unchanged owner-RMW computation, with exact identity and provenance.
Generated request/reply contracts carry that condition. Existing sources and
their failure rows remain unchanged. This is a new checked initial contract,
not a runtime ContractUpdate substitute.

B follows Constitution C1--C3 and C9: the new failure is visible to checking
before placement/execution, and no unsupported authorization transition is
invented. The new feature remains subject to initial M9 admission and ordinary
use-time authority checks. Bare M5 owner-RMW evidence does not prove this new
admission/outcome layer.

## Required evidence and stop

Use spec/16's finite positive/falsifier matrix, including old-source regression,
deadline boundary arithmetic, authority/tick/resolve ordering, failure transport,
retained replay decisions and lower-layer bypass rejection. The implementation
must use real generated artifacts and selected QUIC across actual processes;
no source-selected raw clock, hidden retry, success-only receipt shortcut or
process reaper counts as deadline semantics.

Stop this design investigation when those contracts are implementable and
independent review has no P0/P1. Stop implementation expansion when the specified
positive/falsifier and direct-consumer evidence passes. General clocks, leases,
WAN deadlines, durability, provider syntax, public compatibility and I3-4 are
not consumers of this decision. Existing owner-reserved stops remain unchanged.
