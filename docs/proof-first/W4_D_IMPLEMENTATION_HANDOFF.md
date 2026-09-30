# W4-D model-switch handoff (LAB)

This is a bounded implementation handoff within the existing W4 request, not a
new roadmap or Canon decision. The owner requested a pause when switching from
GPT-6-Astra xhigh to GPT-6.1-sol xhigh becomes useful. Resume the same task after
the owner switches and asks to continue. Sole main; no subagents or automatic
model changes. After D, stop again before E. E/W5+/Plan250-I3-4 remain inactive.

## Exact starting point

- Prior committed checkpoint: `13e7b893`, normal push and remote parity verified.
  The successor commit containing this handoff is the next starting cut; use
  `git rev-parse HEAD` and inspect dirty state instead of assuming a short hash.
- Production Rust remains the adopted baseline. C's fifteen-file reference is
  external, unadopted and contains test-only source execution. Do not copy all
  fifteen files into a normal build or claim that C ran across processes.
- Persistent evidence root:
  `/home/codex/.local/state/mirrorea-proof-first/w4-20260926-integration`.
  D evidence is its `d-source-process/` directory. Frozen directories and logs
  must not be overwritten; create a successor for each changed source/run.
- C reference: `c-source-effect/statement-current-development/foreign-fault-retention-green-v1/`.
  Its `PATHS.json` names fifteen flat files. C's default806/feature818 tests and
  190-module/19502-owned/156-false audit retain their original scope.
- D M9 component: `d-source-process/m9-live-frame-controls-v1/`, two flat files
  with `PATHS.json` and `SOURCE_PINS.json`. This is the new component to build on;
  it is not an installed process path. Full/default and normal-build outcomes
  are recorded in W4_CHECK and Report2614, including failures.
- Proof sources and exact pins: `W4_D_PREREQUISITE_SOURCE_MANIFEST.json` and
  `samples/lean/foundations/MirroreaProofFirstDPrerequisites.md`.
  The earlier fresh197/20278/168/366 run and subsequent changed-source audits
  are different receipts. Do not upgrade the old run's claims by wording.

## Selected boundary contract

One trusted supervisor owns the actual M9 successor publisher for a fixed finite
cohort. It allows one scoped local semantic action at a time. A grant identifies
run, endpoint instance, action kind, fresh serial, protocol revision and exact
current authority binding. Protocol revision, M9 generation, source activation,
statement ordinal, semantic request ID and TLS peer identity are different fields.

The supervisor records the active grant BEFORE sending it on the endpoint's
registered control stream. Delivery loss, timeout, disconnect, panic or process
exit never clears it or refunds escaped identity/resource reservations. Only the
exact endpoint/kind/serial completion on its registered stream can finish it.
Grant serial and new counters use checked arithmetic before anything escapes.
The run may become unavailable; this is not process-death recovery.

A local interval includes semantic mutation and coherent retention of its actual
outcome/report/reply association. It cannot end at backend return or merely when
a local M9 floor guard drops. All fallible post-body bookkeeping must have a
retained real lower outcome and preflighted capacity, and cannot rerun the body.
Prepare the real outgoing request/reply locally before ending its grant; actual
QUIC send/wait/read does not hold that interval. A held network request alone
does not prevent a genuine authority update.

Authority update order is fixed:

1. Close issuance of new grants; drain the exact active local action.
2. Use the actual parent publisher to prestage an exact M9 owner-capability
   successor. The parent performs no owner/consumer/release validations.
3. Derive each endpoint's old/new restricted authority from the full genuine
   delta and the same checked source/Core-derived restriction. Never send full
   global authority merely to make a child's equality test succeed.
4. Freeze every endpoint and prepare it in place while all semantic entry paths
   are disabled. Compare actual live authority and actual canonical floor with
   the strong M9 authority-facts relation. Preserve all three M9 observation
   maps from that actual child. Refresh the real backend and adopt local M9/floor
   before producing a prepared ACK. Preserve the existing runtime object,
   pending/history/store/cache/causality/queues/result/source state.
5. Require exact prepared ACKs from every fixed endpoint, including a third
   endpoint unaffected by the selected revocation. Commit the actual parent
   staged publisher and bind the protocol revision to that actual publication.
6. Activate the exact retained preparation at every endpoint; reopen grants only
   after all activations. A prepared backend is never effectively usable before
   publication. Partial failure stays frozen/unavailable; no silent rollback.

The coordinator models derive current held values and the prepared-record
invariant. AuthorityLocalFrame derives exact child observation/runtime framing
under explicit authentic-stage/projection premises. These do not authenticate an
FD, execute the M9 publisher, prove Rust/OS correctness or install an M8 backend.
In particular, the model's updated floor represents completed preparation;
the pure Rust M9 candidate constructor has not yet performed that update.

## First Sol implementation package

This contract is ready for a bounded implementation pass. It does not assert that
all D source/process design or implementation is complete.

1. Reproduce and repair the normal private-QUIC build mismatch below in a new
   reference cut; preserve the existing fault-injection test behavior. Check the
   actual caller feature guards before choosing the smallest correction.
2. Extend the genuine M9 restricted-delta component with a strict bounded DTO
   containing authority facts only. Decode it as untrusted data. Do not weaken
   the old prelaunch exact successor API or reuse `matches_for_restore` for the
   strong authority-prior/floor check. Nonincident wire records should not carry
   unnecessary foreign operation metadata or authority rows.
3. Implement the supervisor/endpoint control state machine over actual owned,
   registered Unix streams. Session construction owns each stream exactly once.
   Only its read method may mint an opaque, noncloneable control token. There
   must be no generic `Read`, byte-buffer, JSON/Deserialize or public raw-stream
   factory that converts attacker-supplied values into that token. A fixed
   inherited-FD bootstrap must be taken once and bound to the compiled role/run.
4. Bind run/cohort, endpoint instance and role, source purpose, checked program,
   exact restriction, old/new authority, stage, revision and codec version. Track
   replay in the session state; Rust move semantics alone do not reject duplicate
   bytes. Use independent parent-known authority bindings and child prepared-state
   bindings; later valid observations may change the latter without changing
   authority, so it cannot become an immutable grant prerequisite.
5. Connect the new preparation seam to actual SYS4 live M9/floor/backend while
   disabled. A wrapper may not expose a mutable Runtime/LocalFabric, copied
   executable seed or publisher that bypasses the gate. Generic start/emitter,
   typed/decoded/feature ingress, lifecycle, cut/restore, direct admission and
   report paths must either be protected by the same owned mode or refuse in it.
6. Establish actual producer/alias/resource bounds before body entry. This
   includes new control/transport IDs, observation counts, pending/tombstone
   limits, endpoint trace/aggregate indices and whole-activation result slots.
   Distinguish numeric capacity from actual allocation; no ID rollback after
   escape, no fabricated successful receipt on allocation/report failure.

Expected result: normal-build source for the new component/control path, exact
state/field/caller correspondence, reproducible positive/negative tests, explicit
remaining consumers and a focused diff. Remain in an external unadopted reference
until the same task's integration gate is met. Keep the existing source tree
restored between overlay runs. Do not claim a process E2E from sequential calls
to private helpers or treat a matching TLS peer as source custody/M9 authority.

## Concrete known build failure for the first package

With the D two-file overlay, default `mir-runtime --lib` passes all451 tests.
The non-test command below fails, and the identical failure is reproduced on
clean production Rust at `13e7b893`:

```sh
cargo check --locked --offline -j1 -p mir-runtime --lib --features i3-private-quic
```

`sys5_i3_private_quic.rs:2245` calls `write_blob_length_prefix_only`, whose
implementation at2082 is gated by `i3-process-test-seams`; its caller
`send_original_owner_request_attempt_length_prefix_then_hold` is not. This is an
existing normal-feature closure failure, not a caused-by-M9 regression. The
receipts are `m9-live-frame-full-v1/` and `normal-quic-baseline-check-v1/`.
No correction has been made in this handoff. Ordinary source/M9 runtime tests
passing under `cfg(test)` do not establish a successful normal QUIC build.

## Remaining D design and integration gates

These remain visible, with no used premise deferred to E:

- **Unique source progress:** one normal-build requester owns the original full
  checked handler, arguments, activation/global ordinal and exact pending/result
  cursor. Remote owners hold inert descriptors. Generic cloned/decoded images,
  same-peer sessions or a second startup cannot create another executable cursor.
  Preserve the actual M7 metadata rather than externally looping operation IDs.
- **Combined admission:** retain the existing I3 admission-budget/clock/one-use
  handoff. Add source-origin/cursor checks at its actual owned lower entry; do not
  replace it with C's local `enqueue_source_reference` branch, which bypasses
  that separate I3 permit. Before using a combined handoff, write the exact
  source/I3 consumption, typed failure and resource correspondence.
- **Real retained result:** follow the actual SYS5 finalizer after SYS4/M8; body
  commit, reply extraction, occurrence projection and source acceptance differ.
  Preserve an actual lower result before a later report can fail. A historical
  result does not grant a current-generation source acknowledgment.
- **Distributed causality:** source issue → real private QUIC request/receive →
  actual admission/body/history → retained report/reply → real source receive/
  current acknowledgment. C's same-fabric causal map is not automatically a
  distributed occurrence graph. Use the same actual events in redacted output.
- **Scope:** no new grammar/public API/wire, global audit-map merge, blanket ban
  on all held requests, general recovery, timing/resource noninterference or
  whole-language/alpha completion. Broad W6 and W5 guarantees stay separate;
  if a D consumer needs one of their premises, discharge it before that use.

Return to Astra xhigh for a material change to these contracts, a counterexample
that changes authority/custody/resource semantics, and D's integrated acceptance.
This is a review checkpoint, not a requirement to switch models for every bug.
After explicit E resume, Sol xhigh can implement/run its campaign; Astra xhigh
should reconcile the union of A–E obligations and the119-row disposition before
W4 completion. Model choice does not replace actual validation or review evidence.

## Required acceptance and refusal cases

Use funded/authentic positive companions for each negative. At minimum retain:

- Original S→T→S source and another activation in the same world, complete
  arguments/Core/global ordinal, and the real same-request result origin.
- Matching peer without source custody, generic protected start, owner-as-cursor,
  wrong descriptor/program/arguments/ordinal/activation and duplicate bootstrap.
- Actual child validation then unrelated/own revocation, old ordinary carrier
  versus already queued admitted use, an unaffected third endpoint, exact local
  observations/history retained, old acknowledgment rejected where required.
- Wrong or old predecessor/floor, foreign/extra authority, wrong/missing/replayed
  prepare ACK, activation before actual parent publication, incomplete activation,
  wrong endpoint/kind/serial finish and finish before coherent result retention.
- Missing resource capacity before body with funded success, actual result/report
  loss after body with no repeated body, duplicates/delayed receive and truthful
  unresolved outcome. No imagined OOM/process-death recovery claim.
- Normal default/private-QUIC builds, focused controls, required existing I3
  regressions, ownership/false-control proof audit and docs validation at the
  exact final source cut. E's full fresh campaign remains a separate next unit.

## Operational constraints

Keep heavy runs serial: Rust8GiB address-space/-j1/test-threads1; Lean6GiB,
`--trust=0 -j1`. Check disk/RAM before growth. External mount `/mnt/mirrorea-work`
is absent; do not use the host shared folder. Preserve all source/proof/evidence
and browser state; only already-authorized known disposable build artifacts may
be cleaned. No external notifications/publication. Commit with `--no-gpg-sign`
and preserve the previously authorized normal-push/parity workflow.

Weekly remaining was82% at2026-09-30 06:48:14 UTC. Check no earlier than07:48:14
UTC; stop near50% at a sensible checkpoint. Only the owner resets the account.
The immediate pause in this handoff is for the requested model switch, not quota
exhaustion, D completion, or a change to Plan250's independent owner pause.
