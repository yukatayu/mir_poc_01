# Report 1104 — Alpha-0 Network / Docker First Cut

- Date: 2026-05-02
- Author / agent: Codex
- Scope: `P-A0-09` network / Docker E2E first cut
- Decision levels touched: `L1` / `L2` implementation and package sequencing over already-frozen `specs/17`; no new normative decision

## Objective

Close `P-A0-09` by turning `samples/alpha/network-docker/` from scaffold-only planning material into the first non-public Rust Stage-C network / Docker floor, while keeping transport/auth/membership/capability/witness separation explicit and preserving the stop line around production WAN/session/replay/final ABI claims.

## Scope and assumptions

- Scope is intentionally narrow:
  `NET-02/03/04/05/07/09` only, over a local TCP process boundary and Docker Compose bridge.
- The current cut proves:
  delivery over a real process/container boundary, explicit stale-membership rejection, explicit capability rejection, explicit witness rejection, observer-safe redacted route trace, and auth-evidence lane preservation.
- The current cut does not prove:
  `NET-01`, `NET-06`, `NET-08`, `NET-10`, route rebinding completion, partition handling, transport-medium rebinding completion, production WAN federation, durable replay, continuous shared runtime state across worlds, runtime package/avatar admission, or final public transport ABI.
- Working assumption for this package:
  keep the runtime and runner alpha-local, sample-ID keyed, and non-public so the stop line remains explicit.

## Start state / dirty state

- Branch at package start: `main...origin/main`
- Start state before `P-A0-09` was clean after `af8e963` (`mirrorea: add alpha layer insertion runtime floor`) had been pushed.
- Dirty state during this package was limited to the new Rust network runtime floor, Docker runner/compose asset, sample sidecars, synchronized docs, and this new report. No unrelated user changes were reverted.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/43-alpha-e2e-roadmap.md`
- `specs/00-overview-and-principles.md`
- `specs/01-core-calculus.md`
- `specs/02-effects-and-handlers.md`
- `specs/03-temporal-and-ownership-modes.md`
- `specs/09-language-and-toolchain-status.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- all files under `sub-agent-pro/alpha-0/`
- `docs/reports/1103-alpha0-layer-insertion-runtime-first-cut.md`

## Actions taken

1. Re-read the Alpha-0 package, repository hierarchy docs, and current Phase 5 evidence to confirm that `network-docker/` was still scaffold-only and that helper-local `scripts/network_transport_samples.py` could not honestly close the Alpha `NET-*` rows by name.
2. Ran package-specific sub-agents:
   - `Hypatia`: smallest honest Stage-C cut / file ownership / deferred rows
   - `Gibbs`: transport/auth/membership/capability/witness separation review
   - `Sartre`: Docker validation floor / helper-vs-alpha taxonomy review
   - `Helmholtz`: worker exploration for runtime/runner shape
3. Added `crates/mir-runtime/src/alpha_network_runtime.rs` as a runtime-private Stage-C transport floor over `MessageEnvelope` / `AuthEvidence` / `LogicalPlaceRuntimeShell`, with explicit admission checks:
   - principal/place binding
   - membership epoch current
   - member incarnation current
   - participant active
   - participant place current
   - capability sufficient
   - required witness present
   - auth evidence coherent
4. Added example and integration tests for the new floor, plus `Deserialize` derives on the existing `mirrorea-core` envelope/auth carriers so the process-boundary / Docker JSON path could round-trip real messages without inventing a new carrier family.
5. Added thin Docker runner `scripts/alpha_network_docker_e2e.py`, its unit tests, and `samples/alpha/network-docker/docker-compose.alpha-net.yml`.
6. Materialized the first actualized Alpha rows in `samples/alpha/network-docker/`:
   - `NET-02` accepted two-process / Docker envelope path
   - `NET-03` stale membership reject
   - `NET-04` missing capability reject
   - `NET-05` missing witness reject
   - `NET-07` observer-safe route trace
   - `NET-09` auth-evidence lane preserved
7. Resolved the main review contradiction explicitly:
   - smallest honest cut from `Hypatia` was `NET-02/03/07`
   - `Gibbs` insisted that capability/witness/auth coherence stay explicit, not inferred from delivery success
   - because those checks were bounded enough inside the same Stage-C cut, the package widened to actualize `NET-04/05/09` too
8. Synchronized roadmap/snapshot/taxonomy docs so `network-docker/` is now a non-public Rust Stage-C floor with its own validation anchors, while helper-local `network_transport_samples.py` remains a separate canary family.

## Files changed

- `crates/mirrorea-core/src/fabric.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/alpha_network_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs`
- `crates/mir-runtime/tests/alpha_network_runtime.rs`
- `scripts/alpha_network_docker_e2e.py`
- `scripts/tests/test_alpha_network_docker_e2e.py`
- `samples/alpha/network-docker/README.md`
- `samples/alpha/network-docker/docker-compose.alpha-net.yml`
- `samples/alpha/network-docker/net-02-docker_two_process_envelope.expected.json`
- `samples/alpha/network-docker/net-03-stale_membership_rejected.expected.json`
- `samples/alpha/network-docker/net-04-missing_capability_rejected.expected.json`
- `samples/alpha/network-docker/net-05-missing_witness_rejected.expected.json`
- `samples/alpha/network-docker/net-07-observer_safe_route_trace.expected.json`
- `samples/alpha/network-docker/net-09-auth_evidence_lane_preserved.expected.json`
- `samples/alpha/README.md`
- `samples/README.md`
- `scripts/README.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/43-alpha-e2e-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Commands run

```bash
git status --short --branch
df -h .
free -h
docker --version
docker compose version
sed -n '1,260p' sub-agent-pro/alpha-0/12-codex-task-packages.md
sed -n '1,220p' samples/alpha/network-docker/README.md
sed -n '1,220p' samples/alpha/README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' samples/README.md
sed -n '1,260p' scripts/README.md
sed -n '130,230p' plan/01-status-at-a-glance.md
sed -n '1,220p' plan/43-alpha-e2e-roadmap.md
sed -n '1,260p' crates/mir-runtime/src/alpha_network_runtime.rs
sed -n '1,260p' scripts/alpha_network_docker_e2e.py
sed -n '1,260p' crates/mir-runtime/tests/alpha_network_runtime.rs
sed -n '1,220p' crates/mir-runtime/examples/mirrorea_alpha_network_runtime.rs
cargo test -p mirrorea-core --test carriers
cargo test -p mir-runtime --test hotplug_runtime_skeleton
cargo test -p mir-runtime --test alpha_local_runtime
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
cargo test -p mir-runtime --test alpha_network_runtime
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku
cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- stale-membership
cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout
cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout
python3 -m unittest scripts.tests.test_alpha_network_docker_e2e
python3 -m py_compile scripts/alpha_network_docker_e2e.py
python3 scripts/alpha_network_docker_e2e.py list --format json
python3 scripts/alpha_network_docker_e2e.py run NET-02 --format json
python3 scripts/alpha_network_docker_e2e.py check-all --format json
python3 scripts/alpha_network_docker_e2e.py closeout --format json
date '+%Y-%m-%d %H:%M JST'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Resource / environment floor:
  - `df -h .` showed `/dev/vda2` with about `99G` total and `30G` available.
  - `free -h` showed `960Mi` RAM total and swap available.
  - `docker --version` and `docker compose version` both succeeded, so the Docker lane could be attempted honestly.
- `cargo test -p mirrorea-core --test carriers`
  - 12 tests passed.
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
  - 8 tests passed.
- `cargo test -p mir-runtime --test alpha_local_runtime`
  - 3 tests passed.
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime`
  - 6 tests passed.
- `cargo test -p mir-runtime --test alpha_network_runtime`
  - 7 tests passed.
  - Verified accepted/rejected Stage-C rows and kept-later refs.
- `cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout`
  - emitted JSON closeout evidence for `NET-02/03/04/05/07/09`
  - `NET-02` showed accepted two-process/TCP evidence with all admission checks passing
  - `NET-03` showed explicit `membership_freshness` rejection with epoch/incarnation drift refs
  - `NET-04` showed explicit capability insufficiency
  - `NET-05` showed explicit missing witness
  - `NET-07` showed observer-safe route trace only
  - `NET-09` showed separate auth lane with preserved bindings
- `python3 -m unittest scripts.tests.test_alpha_network_docker_e2e`
  - 7 tests passed.
- `python3 -m py_compile scripts/alpha_network_docker_e2e.py`
  - passed.
- `python3 scripts/alpha_network_docker_e2e.py list --format json`
  - returned the 6 implemented Alpha rows only.
- `python3 scripts/alpha_network_docker_e2e.py run NET-02 --format json`
  - Docker Compose run succeeded and produced both world and participant JSON outputs.
- `python3 scripts/alpha_network_docker_e2e.py check-all --format json`
  - `sample_count = 6`
  - passed all:
    `NET-02`, `NET-03`, `NET-04`, `NET-05`, `NET-07`, `NET-09`
- `python3 scripts/alpha_network_docker_e2e.py closeout --format json`
  - kept planned-only rows separate from implemented rows.
- `python3 scripts/check_source_hierarchy.py`
  - passed with required/present/missing = `60/60/0`.
- `python3 scripts/validate_docs.py`
  - passed.
- `python3 -m unittest scripts.tests.test_validate_docs`
  - 11 tests passed.
- `cargo fmt --check`
  - passed.
- `git diff --check`
  - passed.

## What changed in understanding

- The smallest honest Stage-C close is not “networking exists” in the broad sense. It is a bounded transport/admission floor where delivery success and admission success remain separate.
- Helper-local `network_transport_samples.py` and Alpha-0 `network-docker/` must stay distinct.
  The helper family remains a clean-suite canary; it cannot be silently relabeled as the Alpha Stage-C runtime floor.
- Capability, witness, and auth coherence were cheap enough to keep explicit in the first cut.
  That made `NET-04/05/09` honest to actualize now instead of deferring them.
- Docker availability itself is a package gate.
  Because the environment had working Docker/Compose, the package could close as real local-container evidence rather than docs-only deferral.

## Open questions

- `NET-06` route rebinding without shadow, `NET-08` network partition explicit failure, and `NET-10` transport-medium change remain open.
- `P-A0-10` still needs the smallest honest runtime package/avatar floor that keeps avatar/VRM/VRChat/Unity out of Mir core primitives.
- Stage-F integration still lacks package/avatar admission, save/load runtime integration, and alpha visualization/devtools surfaces.

## Suggested next prompt

Continue autonomously with `P-A0-10`: add the first runtime package / avatar skeleton without promoting avatar formats or native-package provenance to Mir core primitives or semantic safety claims.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md` now records `P-A0-09` closeout and the `P-A0-10` reopen point; `plan/43-alpha-e2e-roadmap.md` now records the first Rust Stage-C network / Docker floor and its validation anchors.

## Documentation.md update status

`Documentation.md` 更新済み:
the Alpha-0 integration summary now includes the non-public Rust network / Docker floor and its stop line.

## progress.md update status

`progress.md` 更新済み:
large stage map percentages, Stage-C status, validation freshness, and the current package reading were moved to the `P-A0-09`-closed / `P-A0-10`-next state.

## tasks.md update status

`tasks.md` 更新済み:
the current queue now marks `P-A0-09` closed, promotes `P-A0-10` to the head of the autonomous queue, and adds `P-A0-11` integrated demo closeout behind it.

## samples_progress.md update status

`samples_progress.md` 更新済み:
the Alpha network family was split from visualization / integrated E2E, `A0-NET` now records the Rust Stage-C floor, and the Docker validation row is explicit.

## Reviewer findings and follow-up

- `Hypatia`
  - recommended the smallest honest Stage-C cut as transport trait + local-subprocess/TCP bridge + Docker shape + stale-membership reject, and warned not to overclaim `NET-04/05/09` without explicit checks
  - implemented follow-up:
    kept the cut narrow and only widened after explicit capability/witness/auth checks existed in the runtime floor
- `Gibbs`
  - warned that delivery success must stay separate from admission success, and that membership freshness, capability, witness, and auth coherence must be checked explicitly
  - implemented follow-up:
    added explicit admission-check rows and rejection reason refs for those families
- `Sartre`
  - warned that helper-local `network_transport_samples.py` does not close the Alpha `NET-*` rows by name and that Docker-specific executable evidence must be separate
  - implemented follow-up:
    added `alpha_network_docker_e2e.py`, the Docker Compose asset, and distinct docs wording
- `Helmholtz`
  - explored a bounded runtime/runner split for the Docker lane
  - implemented follow-up:
    kept the runner thin, with the semantic checks in `mir-runtime` and the container orchestration in Python/Compose

## Skipped validations and reasons

- No validation for `NET-01`, `NET-06`, `NET-08`, or `NET-10` was run because those rows remain planned in this package.
- No runtime package/avatar validation was run because it belongs to `P-A0-10`.
- No integrated save/load runtime validation was run because the current save/load line is still checker-first and distributed save/load without a consistent cut remains disallowed.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Hypatia` (`019de5db-e9d7-7661-8fd5-8c4fe2b0a2db`) completed and is closed.
- `Gibbs` (`019de5db-ec0e-76e3-a47b-a92a54156493`) completed and is closed.
- `Sartre` (`019de5db-ecbb-76d3-a4b0-45ff8c7fed2a`) completed and is closed.
- `Helmholtz` (`019de5e3-38ca-7032-bfc7-3bc44061d70c`) completed and is closed.
