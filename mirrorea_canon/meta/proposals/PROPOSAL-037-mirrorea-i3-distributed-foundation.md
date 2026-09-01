---
id: meta/proposal-037
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0033, plan/05-i3-entry-contract]
summary: project/product layer と Browser/Host 境界を整合し、accepted I2 を実 process/transport へ写す owner-directed bounded program。
open_items: [OPEN-032]
---

# PROPOSAL-037 — Mirrorea I3 Distributed Foundation bounded program

## Owner disposition

Recorded on 2026-09-01: **accepted as an owner-level direction**.

The owner authorizes one bounded successor program named **Mirrorea I3
Distributed Foundation**. Its parent capability goal is:

> Canonically separate Mir, Mirrorea, Browser/Host, Shared-Space, and upper
> application responsibilities, then meaning-preservingly map the accepted I2
> per-locus artifacts and generated communication across at least two
> operating-system processes over a selected real transport while preserving
> authority, typed failure, source/Core provenance, redaction, and Mir abstract
> ordering, and close a finite C-distributed I3 profile.

The program starts at repository revision
`ca6ffeceda6b2ed87edd2b98d6d2a6a74f61f9df`. The accepted I2 implementation
and evidence cuts recorded by ADR-0027--0032 remain immutable regression
baselines. ADR-0026 and LAB Plan 249 remain closed history and grant no
successor authority; this proposal records the new authority required by
ADR-0033 and plan/05.

## Fixed capability sequence and sole roadmap

The single current frontier advances in this order:

```text
ALIGN-0 baseline / one goal / meta-drift audit
-> ALIGN-1 project-product layer constitution
-> ALIGN-2 Browser/Host/package/View/provider boundary contracts
-> I3-0 transport candidate evidence and selection
-> I3-1 transport-neutral adapter and private wire mapping
-> I3-2 multi-process locus runtime and deployment
-> I3-3 network failure / ordering / retry / reconnect semantics
-> I3-4 C-distributed scenarios and pressure slices
-> I3-5 observer-safe network devtools and user workflow
-> I3-6 finite I3 conformance and lifecycle closeout
-> NEXT-0 inactive I4 and I5 entry contracts only
```

The sole current LAB roadmap is
`LAB:plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`. Only one
milestone is active at a time. A milestone may be added only when the parent
goal cannot close without it, and the roadmap must record the reason.

## Three-axis and upper-layer restraint

This program distinguishes three coordinate systems:

1. semantic strata `S0 Surface` through `S6 Host`;
2. project/product layers `PL-0` physical host through `PL-6` upper application;
3. lifecycle phases `T0--T2` and `I1--I6`.

It does not rename or reuse the existing semantic `S` numbers. `World`,
`Room`, `Avatar`, `Portal`, and similar domain terms remain library/application
vocabulary, never Core primitives. Shared-Space / World-Web receives only a
responsibility boundary in this program. Reversed Library remains a separate
upper application/project. PrismCascade and the Typed-Effect Wiring Platform
remain separable satellites.

ALIGN-2 may fix responsibility, required information, trust tiers, resource
controls, and prohibited authority flows without selecting a package format,
origin syntax, sandbox technology, browser UI, engine/plugin ABI, public FFI
ABI, public wire, or storage format. View/provider code may perform
presentation-local computation, but never owns authoritative domain semantics.
Untrusted packages have no direct raw-native FFI path.

## Goal-driven and evidence protocol

Each milestone records a capability Goal Statement containing its semantic,
project/product, and lifecycle coordinates; North Star link; user-visible
outcome; semantic invariants; direct consumer; non-goals; primary falsifier;
exit evidence; and finite stop condition. A goal that only says to research or
write documents is invalid.

Every new research item, carrier, lemma, report, or helper needs a direct
consumer, blocker reduced, and acceptance use. Compare at most the current
smallest design and one viable alternative. Close when the positive path runs,
the representative falsifier is detected, the direct consumer can use the
result, later conservative extension remains possible, no public compatibility
is frozen, and independent review has no P0/P1 finding.

The default is one report per milestone. Applicable milestones close a
normative rule, implementation, positive case, negative/falsifier, exact
proof/model/runtime classification, source-to-implementation correspondence,
fresh validation, independent review, commit/push, and remote parity.

## Transport comparison and evidence ordering

I3-0 retains exactly two candidates:

```text
A: TLS-over-TCP framed reliable stream
B: QUIC reliable stream
```

QUIC datagrams are excluded. Selection cannot be paper-only. Both candidates
must run an equal, private, source/Core-bound carrier canary across at least two
OS processes and expose the same authority, failure, ordering, identity, and
observer-safety checks. At minimum, wrong target/provenance/authority rejects
before mutation; partial/reset and pre-/post-admission disconnects are visible;
duplicate/reconnect preserves request and grant lineage without stale
resurrection; and Mir dependency evidence, not stream order, justifies
semantic order. If either candidate cannot satisfy the shared criteria, it
cannot be selected. If both fail, or an irreversible observable semantic tie
remains, the program stops under the owner decision rule.

I3-3 consumes every applicable failure family fixed by plan/05, not merely
disconnect/reconnect cases. I3-4 emits the minimum observer-safe trace and
diagnostic evidence required by each C-distributed gate. I3-5 may join and
present those existing facts but may not invent gate evidence.

## Delegated authority and lifecycle boundary

Within this fixed sequence, the orchestrator may autonomously update Canon
architecture, boundary contracts, ADRs, specifications, plans, and exact
evidence classification; internal Rust/Lean/model/test/CLI/devtools surfaces;
agent configuration; reports; roadmap; and derived status. It may compare and
select one retained transport, define a transport-neutral internal adapter and
private provisional framing/codec/version, implement local multi-process
execution and fault injection, and apply official I3 entry then exit only after
the actual I3-6 criteria, fresh evidence, and independent review pass.

Program activation and intermediate milestones are not official I3 entry or
exit. Runtime or conformance output cannot self-authorize a lifecycle change.
Theory remains T1 and broad PHASE-I1 remains unaccepted unless their separate
existing criteria are independently satisfied; this program does not weaken
them.

## Owner-reserved stop line

Stop and return a decision bundle only if work requires changing the North
Star; weakening authority, privacy, redaction, or no-stale-resurrection;
promoting concrete domain vocabulary into Core; introducing an unavoidable
hidden multi-owner transaction or hidden retry; irreversibly freezing a public
API/ABI/wire/package/FFI contract; selecting between two equally valid but
non-migratable observable semantics; production deployment or publication;
risk to user data, secrets, or paid resources; final Shared-Space governance,
Reversed Library product design, or Unity/Unreal plugin ABI; or accepting a
reproducible parent-goal/North-Star contradiction.

Theory T1, broad I1 residuals, deferred general obligations, provisional
grammar/public ABI, unresolved later product details, and unoptimized
performance are not stop conditions.

## Non-effects

This decision does not itself select a transport, activate I3 lifecycle,
change accepted I2 semantics, freeze any public grammar/CLI/JSON/API/ABI/wire/
package/FFI, establish production security, deploy or publish anything,
complete durable distributed persistence, choose a browser/renderer/sandbox,
define Shared-Space vocabulary/governance, start Reversed Library product
work, or prove arbitrary network, scheduler, fairness, relation-DAG,
durability, sandbox, or noninterference theorems.
