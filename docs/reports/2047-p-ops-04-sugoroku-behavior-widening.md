# Report 2047 — P-OPS-04 Sugoroku behavior widening

- Date: 2026-05-06 22:42 JST
- Author / agent: Codex
- Scope: bounded `SugorokuWorld` runtime actualization over the operational product sample suite, helper/devtools semantic validation, snapshot/doc sync
- Decision levels touched: `L1`/`L2` wording and bounded executable-surface sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-04 Sugoroku behavior widening` by moving one bounded `SugorokuWorld` scenario from manifest-only intent to reproducible same-session runtime evidence, while preserving product alpha stop lines and keeping projection / portal / shard work out of scope.

## Scope and assumptions

- Scope includes:
  - `samples/product-alpha1/operational/sugoroku-world/`
  - `crates/mir-runtime::product_alpha1_session` bounded Sugoroku runtime evidence
  - `scripts/operational_product_samples.py` semantic checks for Sugoroku runtime/devtools evidence
  - required specs / roadmap / hands-on / dashboard / report sync
- Scope excludes:
  - final interactive game runtime
  - broader Sugoroku control surface or additional negative rows beyond the bounded carrier
  - projection packet / FFI schema formalization
  - portal / world-link runtime actualization
  - shard / federation runtime or model-check actualization
- Assumption: Docker and `docker compose` remain available for the unchanged operational transport leg in this environment.

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean after `P-OPS-03` commit / push `c038fc0`
- Existing operational suite state at start:
  - `P-OPS-01` already provided the canonical `WorldCore -> MembershipChat -> SugorokuWorld` suite
  - `P-OPS-03` already actualized the bounded `MembershipChat` `EchoText("Taro") -> "Hello, Taro!"` direct host boundary
  - `sugoroku-world` was still limited to typed `AddOne` host-I/O evidence plus manifest-level roll / publish / witness / handoff intent
- During work, helper/local runs created `.mirrorea-alpha/` session artifacts; those are cleanup targets before commit.

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/20-cut-save-load-semantics.md`
- `specs/21-auth-layer-algebra.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/reports/2045-p-ops-01-operational-product-sample-suite.md`
- `docs/reports/2046-p-ops-03-operational-chat-direct-text-host-boundary.md`
- `sub-agent-pro/operational-product-sample-001/15-next-packages.md`
- `sub-agent-pro/operational-product-sample-001/sample-blueprints/sugoroku-world-blueprint.md`

## Actions taken

- Added bounded `SugorokuWorld` runtime evidence inside `crates/mir-runtime::product_alpha1_session`:
  - `run-local` now materializes one deterministic same-session sequence for roll, publish, witness emission, handoff, and stale membership reject
  - the sequence is expressed through event DAG nodes, route graph lanes, and message failure/recovery summaries on the existing product alpha session carrier
  - the direct execution lane remains the existing typed `AddOne` host-I/O adapter; the wider gameplay evidence is layered on the same carrier without claiming final interactivity
- Added focused Rust assertions so the operational Sugoroku root now requires:
  - the bounded Sugoroku event kinds
  - the bounded Sugoroku route lanes
  - explicit `StaleMembership` rejection evidence in `message_state_lane`
- Extended `scripts/operational_product_samples.py` with Sugoroku semantic validation:
  - `run-sugoroku` now fails if the bounded runtime evidence is absent
  - `export-devtools` now fails if the exported event/route/failure panels do not preserve the same Sugoroku evidence
  - `release-check` / `check-all` now enforce both `sugoroku_runtime_ok` and `sugoroku_devtools_ok`
- Fixed one helper regression found during closeout:
  - `release_check()` initially referenced an undefined `export_result` variable after the Sugoroku semantic-check wiring
  - corrected the variable split to explicit `sugoroku_export` / `sugoroku_view` / `sugoroku_bundle`
- Updated operational suite docs and dashboards so `P-OPS-04` is reflected as the latest closeout and `P-OPS-05` as the next reopen point.
- Synced the prior `P-OPS-03` report with actual commit / push status.

## Files changed

- Runtime / tests:
  - `crates/mir-runtime/src/product_alpha1_session.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
- Operational helper / tests:
  - `scripts/operational_product_samples.py`
  - `scripts/tests/test_operational_product_samples.py`
- Specs / roadmap / docs / dashboards:
  - `specs/26-operational-product-sample-suite.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `README.md`
  - `Documentation.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `samples/product-alpha1/operational/sugoroku-world/README.md`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/operational_product_sample_01.md`
  - `scripts/README.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report sync:
  - `docs/reports/2046-p-ops-03-operational-chat-direct-text-host-boundary.md`
  - `docs/reports/2047-p-ops-04-sugoroku-behavior-widening.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
sed -n '1,260p' plan/51-operational-product-sample-roadmap.md
sed -n '1,260p' sub-agent-pro/operational-product-sample-001/15-next-packages.md
sed -n '1,260p' sub-agent-pro/operational-product-sample-001/sample-blueprints/sugoroku-world-blueprint.md
sed -n '1,260p' samples/product-alpha1/operational/sugoroku-world/package.mir.json
sed -n '1,260p' samples/product-alpha1/operational/sugoroku-world/README.md
sed -n '1,320p' crates/mir-runtime/tests/product_alpha1_session.rs
sed -n '1,360p' crates/mir-runtime/src/product_alpha1_session.rs
sed -n '1,320p' scripts/operational_product_samples.py
cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_sugoroku_root -- --nocapture
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/operational_product_samples.py run-sugoroku --format json
python3 scripts/operational_product_samples.py export-devtools --format json
python3 - <<'PY'
# inspected export-devtools panel payload shape for Sugoroku semantic checks
PY
python3 scripts/operational_product_samples.py check-all --format json
cargo fmt
python3 scripts/operational_product_samples.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
rm -rf .mirrorea-alpha
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- First targeted failures reproduced as intended before implementation:
  - `cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_sugoroku_root -- --nocapture` failed on missing `sugoroku_roll_requested`
  - `python3 -m unittest scripts.tests.test_operational_product_samples` failed on missing Sugoroku semantic helper functions
- After implementation:
  - targeted Sugoroku Rust test passed
  - `python3 -m unittest scripts.tests.test_operational_product_samples`: pass, `9` tests
  - `python3 scripts/operational_product_samples.py run-sugoroku --format json`: pass with `semantic_checks.runtime_evidence_observed = true`
  - `python3 scripts/operational_product_samples.py export-devtools --format json`: pass with `semantic_checks.runtime_evidence_observed = true`
  - direct export payload confirmed `event_dag`, `message_route_graph`, and `message_failure_recovery` panel payloads preserve the Sugoroku evidence
- First full `python3 scripts/operational_product_samples.py check-all --format json` after the runtime change:
  - failed on helper bug `NameError: name 'export_result' is not defined`
  - after fixing that, failed only on `validation:cargo-fmt`
- After `cargo fmt`, second full `python3 scripts/operational_product_samples.py check-all --format json`:
  - `status = accepted`
  - `docker_included = true`
  - `failed_commands = []`
  - `membership_chat_echo_text_ok = true`
  - `membership_chat_devtools_ok = true`
  - `sugoroku_runtime_ok = true`
  - `sugoroku_devtools_ok = true`
  - `attach_matrix_complete = true`
- After adding this report:
  - `python3 -m unittest scripts.tests.test_validate_docs`: pass, `13` tests
  - `python3 -m unittest scripts.tests.test_operational_product_samples`: pass, `9` tests
  - `python3 scripts/check_source_hierarchy.py`: pass
  - `python3 scripts/validate_docs.py`: pass
  - `cargo fmt --check`: pass
  - `git diff --check`: pass
- `.mirrorea-alpha/` helper-local session artifacts were removed before closeout.

## What changed in understanding

- The operational Sugoroku widening did not require a new command family; the existing product alpha same-session carrier was sufficient once the bounded scenario was made explicit in the session payload itself.
- The devtools export path does not mirror the full `run-local` session shape; helper semantic checks must read panel payloads as the authority for exported runtime evidence.
- Keeping the Sugoroku widening deterministic and session-bound preserves the repo's non-claims while still making save/load, transport, and viewer evidence materially stronger.

## Open questions

- Should later Sugoroku widening add broader interactive controls first, or first add more negative rows such as explicit `MissingWitness` / `NotDiceOwner` runtime evidence?
- In `P-OPS-05`, should projection packet / FFI schema remain manifest-adjacent or move to a distinct projection IR file?
- When room-chat widening is reopened, should it stay as a bounded `ChatText` lane or remain on the narrower `EchoText` host boundary until projection work lands?

## Suggested next prompt

Open `P-OPS-05 projection manifest / packet / FFI schema` and move the operational suite's projection/packet/FFI inventory from manifest-only wording to schema-backed, validation-backed inventory without claiming final server/client split or LLVM codegen.

## Plan update status

`plan/` 更新済み:

- `plan/51-operational-product-sample-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: operational suite current executable boundary now includes the bounded `SugorokuWorld` runtime scenario in addition to the bounded `MembershipChat` text lane.

## progress.md update status

`progress.md` 更新済み: latest closeout, reopen point, line snapshot, blockers, validation floor, and recent log now reflect `P-OPS-04`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-04` is marked actualized, ordered packages now start at `P-OPS-05`, and the recommendation now points to projection / packet / FFI schema formalization.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row now includes bounded Sugoroku runtime evidence, helper semantic anchors, and the new validation log row.

## Reviewer findings and follow-up

- No spawned reviewer sessions were opened in this package. Current tool policy required explicit user delegation permission before new sub-agent use, so this package used local focused review instead.
- Local review findings:
  - the helper initially assumed `export-devtools` returned the same full session shape as `run-local`; fixed by reading panel payloads for Sugoroku semantic checks
  - `release_check()` initially referenced the wrong export variable name; fixed before final closeout
  - `cargo fmt --check` initially failed after the runtime/helper edits; fixed by running `cargo fmt` and rerunning the full helper closeout
- Follow-up: if later packages widen Sugoroku controls or room-chat semantics, keep them as bounded packages rather than silently broadening the current deterministic carrier.

## Skipped validations and reasons

- No intended validation was skipped.
- No new projection / portal / shard runtime checks were added because `P-OPS-04` did not widen those planned-only boundaries.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- No new sub-agent sessions were opened in this package.
