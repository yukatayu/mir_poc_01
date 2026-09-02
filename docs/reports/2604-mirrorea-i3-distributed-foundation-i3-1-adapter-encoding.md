# Mirrorea I3-1 private adapter/encoding closeout — Report 2604

Identifier: Report 2604 / I3-1.

## Objective

Accept the bounded source/Core-derived private adapter boundary and advance the
sole active frontier to I3-2 without entering official I3 lifecycle.

## Scope and assumptions

Start was I3-0 cut `de231cd12fd0a655f58f146244de5ce34c10d6f0`; accepted source
cut is `d75fa2e716802f75f1973b63e7db82ef2e87c673`. Scope is Linux x86_64
localhost QUIC reliable bidi stream, six closed carrier families/twelve edges,
strict private provisional codec and fail-closed admission. TLS/TCP remains a
replacement baseline; public compatibility is not claimed.

Direct consumer: I3-2 actual two-or-more-process generated-artifact runtime.

Blocker reduced: the accepted I2 internal carrier lacked a checked private
bytes/stream encode, decode and admission mapping over the selected QUIC seam.

Acceptance use: I3-2 may transport generated edges without reconstructing or
inventing owner, authority, state, route, occurrence or other semantics.

## Start state / dirty state

At close validation HEAD and `origin/main` were the clean source cut above.
Earlier package-scoped cleanup removed 2.8 GiB; final workspace-pressure
cleanup removed 45.5 GiB. Final free space was approximately 44 GiB.

## Documents consulted

Canon README/MAP, Constitution, runtime-carrier and I3 entry contracts,
ADR-0034/0037, PROPOSAL-037/040, Plan 250, and relevant LAB status/evidence.
The browser-backed Oracle advisory was recorded at
`/tmp/i3-1-carrier-boundary-oracle.md`, SHA-256
`8f7e6255e6feeae4e71db1985ef6078c6f628464ee90d2f743474cc9a89fd756`;
its metadata had `verified=false`, so it is advisory rather than normative or
model-verified evidence.

## Actions taken

Implemented and reviewed `surface_v0_pipeline`, `sys3_projection`, `sys5_local_slice`,
`static_adapter_framing` and `quic_static_adapter`: exhaustive static carrier mapping, strict marker/version
and bounded u32 framing, complete-frame-only JSON admission, QUIC seam evidence,
observer-safe references, zeroizing key handling, bounded reaping, and request
identity label v2. Added PROPOSAL-041 and ADR-0038 and synchronized status.

## Files changed

Source cut `d75fa2e7` contains changes in `mir-semantics`, `mir-runtime`, and
`mirrorea-i3-probe`, including `static_adapter_framing.rs`,
`quic_static_adapter.rs`, source/facade mappings, supervisor hardening, and
their tests. This closeout adds PROPOSAL-041, ADR-0038, the runtime-carrier
mapping, Canon lifecycle/current pointers, Plan 250/status mirrors, the reader
view and its regression test, `Documentation.md`, `docs/project-status.md`,
`progress.md`, `tasks.md`, `samples_progress.md`, and this single report.

- `mirrorea_canon/meta/proposals/PROPOSAL-041-mirrorea-i3-1-private-adapter-encoding.md`
- `mirrorea_canon/adr/ADR-0038.md`
- `mirrorea_canon/architecture/09-i3-private-adapter.md`
- `docs/project-status.md`
- `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Commands run

Heavy commands used `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2` and a 10 GiB
pre-command guard. Principal commands were:

```text
cargo test --locked -p mirrorea-i3-probe --test quic_static_adapter -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test supervisor_falsifiers -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test frame_contract -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test source_binding -- --test-threads=1
cargo test --locked -p mir-runtime --test sys5_i3_probe_facade -- --test-threads=1
cargo test --locked -p mir-runtime --lib sys6_i2_conformance_tests -- --test-threads=1
cargo test --locked -p mir-runtime --test sys6_i2_cli -- --test-threads=1
cargo test --locked -p mir-runtime --test m10_conformance -- --test-threads=1
cargo test --locked -p mir-runtime --test m10_cli -- --test-threads=1
cargo clippy --locked -p mir-semantics --all-targets -- -D warnings
cargo clippy --locked -p mir-runtime --all-targets -- -D warnings
cargo clippy --locked -p mirrorea-i3-probe --all-targets -- -D warnings
cargo fmt --all -- --check
cargo test --locked --workspace -- --test-threads=1
cargo clean -p mir-runtime -p mir-semantics -p mirrorea-i3-probe
(cd mirrorea_canon && python3 meta/build-index.py --check)
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_agent_configs.py
python3 -m unittest scripts.tests.test_validate_agent_configs scripts.tests.test_mirrorea_project_overview_html -v
python3 scripts/validate_docs.py
make docs
git diff --check
```

## Evidence / outputs / test results

Final source-implementation reviews reported P0/P1/P2 = 0 and quality findings
0/0/0. The separate final closeout reviews returned reviewer ACCEPT and planner
GO with P0=0, P1=0 and one accepted/deferred documentation-size P2.
Deterministic property/mutation evidence passed; this is not coverage-guided
fuzzing. A workspace-wide run exited 130 before tests when free space fell from
11.55 GiB to 1.2 GiB; it is NOT PASS and is not represented as a pass.
Focused source evidence passed 60/60 and the accepted SYS-6/M10 floors passed
104/104. Canon index check covered 205 files, source hierarchy 800/800, and
agent-config plus overview regressions 21/21. After correcting report timestamp,
heading, declaration and changed-path conformance, `validate_docs.py` and
`make docs` both passed; each reported a complete scaffold and 1758 numbered
reports.

## What changed in understanding

The selected reliable-stream seam can carry exact receiver-retained snapshots
without making transport identity authoritative. I3-2 can now consume a
checked private boundary; actual multi-process owner runtime is still absent.

## Open questions

I3-2 must provide actual generated-artifact process deployment. I3-3 must add
the complete network failure/order/retry/reconnect semantics. Public wire,
platform support, browser/provider integration, durability and official I3
lifecycle remain open.

## Suggested next prompt

Continue with I3-2 only: launch two independent OS processes from checked
per-locus artifacts through the accepted QUIC adapter.

## Plan update status

`plan/` 更新済み: Plan 250 closed I3-1 and activated I3-2; fixed order and
non-claims retained. Plan index mirrors the accepted I3-1 boundary.

## Documentation.md update status

`Documentation.md` 更新済み: current status, decisions, risks and verification
state now distinguish I3-1 close from I3-2 runtime work.

## docs/project-status.md update status

更新済み: milestone checklist and current active goal now record I3-1 closed and
I3-2 sole active without official lifecycle entry.

## progress.md update status

`progress.md` 更新済み: current frontier, evidence classification and a
command-derived close log are synchronized.

## tasks.md update status

`tasks.md` 更新済み: current promoted package and ordered task map now describe
the actual I3-2 multi-process runtime consumer.

## samples_progress.md update status

`samples_progress.md` 更新済み: I3-0 and I3-1 remain separate bounded runnable
evidence rows, not product/workflow completion.

## Reviewer findings and follow-up

Initial reviews rejected out-of-band wrong-hint receiver behavior, child-only
cleanup completion, overstated zeroization, incomplete observer consistency,
non-profile sender cardinalities, and same-family A/B source substitution.
Test-first corrections made receiver selection byte-derived, strong cleanup a
child-reap/I/O-completion/no-residual conjunction, library-owned key copies an
explicit non-claim, observer validation exact, finite census fail-closed, and
admission exact across sender/selected edge, frame lineage, and snapshot.
Initial close review REJECTED with P1=7/P2=3. Corrections covered receiver
selection, cleanup, zeroization, observer consistency, cardinality, source
substitution, stale current pointers and documentation. A later closeout
re-review REJECTED with P1=3/P2=4 for missing ADR-0038 navigation/dependencies,
premature final-review wording, stale Plan metadata, duplicate assurance text,
and the pre-existing architecture/04 size residual. Those closeout findings
were corrected except that architecture/04 remains above the 15 KB style guide
budget; its I3-1 material is already only a concise pointer to architecture/09,
so deeper broad-carrier decomposition is deferred rather than mixed into this
milestone. The final reviewer returned ACCEPT and the final planner returned GO,
both with P0=0, P1=0 and P2=1 for that explicitly dispositioned size residual.

## Skipped validations and reasons

Workspace test was interrupted by disk pressure before tests (exit 130), so it
is NOT PASS. No Lean/model-check run and no coverage-guided fuzz run were
performed; no general proof or fuzz claim is made. Final index generation/check,
`git diff --check`, source hierarchy, agent configuration, focused overview
tests, `validate_docs.py`, and `make docs` passed. The workspace-wide test
remains the explicit operational validation residual and is not evidence for
the semantic boundary.

## Commit / push status

Source implementation cut `d75fa2e7` was committed and pushed with remote
parity. Closeout documentation is currently uncommitted and unpushed; the
parent integrator owns its final commit/push.

## Sub-agent session close status

Implementation, test, closeout-writer, planner and reviewer sub-agent sessions
are complete; no I3-1 session remains active or retained.

Exact non-claims: no public wire/API/ABI/package/FFI freeze; no production,
WAN, browser, renderer or supported-platform claim; no mutual TLS/live
membership admission; no retry/reconnect/exactly-once or durable cache; no
complete I3-3 matrix; no general proof, fuzz, lifecycle entry or exit.
