# P-OPS-01 Operational Runtime / Toolchain Review

## 1. Title and identifier

- `review-2026-05-06-p-ops-01-operational-runtime-toolchain-review`

## 2. Objective

- Review the operational sample runtime/toolchain/devtools changes around `P-OPS-01`.
- Focus on `crates/mir-runtime/src/product_alpha1_devtools.rs`, `crates/mirrorea-cli/src/main.rs`, `scripts/operational_product_samples.py`, `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`, `crates/mirrorea-cli/tests/alpha_cli.rs`, and `samples/product-alpha1/operational/**`.
- Check command ergonomics, bundle layout correctness, dependency copying, observer-safe export, panel completeness, Docker/local transport treatment, and whether the helper script or tests overclaim runnable status.

## 3. Scope and assumptions

- Scope was limited to repository guidance, the baseline specs required by `AGENTS.md`, the product/operational specs relevant to this review, and the named implementation/test/sample files.
- Review mode only. No implementation edits were made.
- This report is the only repository change in this task.

## 4. Start state / dirty state

- `git status --short` showed a dirty worktree at review start, including the target files and several docs/spec/sample additions already in progress.
- Existing user/worktree changes were left untouched.
- This task adds only this new report file.

## 5. Documents consulted

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `docs/hands_on/operational_product_sample_01.md`
- `samples/product-alpha1/operational/README.md`

## 6. Actions taken

- Read the required repository and spec documents in the mandated order.
- Inspected the target runtime, CLI, helper script, tests, and operational sample files.
- Cross-checked implementation behavior against the product alpha-1 and operational sample suite completion conditions.
- Isolated issues in Docker transport wiring, native bundle content, devtools panel completeness, and release-check/test coverage.

## 7. Files changed

- `docs/reports/review-2026-05-06-p-ops-01-operational-runtime-toolchain-review.md` (new)

## 8. Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md`
- `sed -n '1,260p' .agents/skills/discord-report/SKILL.md`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `rg --files specs | sort`
- `sed -n '1,220p' plan/00-index.md`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,220p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,240p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' specs/22-observability-devtools-semantics.md`
- `sed -n '1,260p' specs/23-typed-external-host-boundary.md`
- `sed -n '1,280p' specs/24-operational-alpha05-alpha08-readiness.md`
- `sed -n '1,520p' specs/25-product-alpha1-public-boundary.md`
- `sed -n '1,320p' specs/26-operational-product-sample-suite.md`
- `sed -n '1,260p' plan/50-product-alpha1-public-boundary-roadmap.md`
- `sed -n '1,260p' plan/51-operational-product-sample-roadmap.md`
- `sed -n '1,260p' docs/hands_on/operational_product_sample_01.md`
- `git status --short`
- `git diff --stat -- crates/mir-runtime/src/product_alpha1_devtools.rs crates/mirrorea-cli/src/main.rs scripts/operational_product_samples.py crates/mir-runtime/tests/product_alpha1_transport_devtools.rs crates/mirrorea-cli/tests/alpha_cli.rs samples/product-alpha1/operational`
- `find samples/product-alpha1/operational -maxdepth 3 -type f | sort`
- `sed -n '1,520p' crates/mir-runtime/src/product_alpha1_devtools.rs`
- `sed -n '1,1520p' crates/mirrorea-cli/src/main.rs`
- `sed -n '1,520p' scripts/operational_product_samples.py`
- `sed -n '1,260p' crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
- `sed -n '1,640p' crates/mirrorea-cli/tests/alpha_cli.rs`
- `sed -n '1,260p' samples/product-alpha1/operational/README.md`
- `sed -n '1,260p' samples/product-alpha1/operational/world-core/package.mir.json`
- `sed -n '1,260p' samples/product-alpha1/operational/membership-chat/package.mir.json`
- `sed -n '1,320p' samples/product-alpha1/operational/sugoroku-world/package.mir.json`
- `sed -n '1,220p' samples/product-alpha1/operational/packages/*/package.mir.json`
- `sed -n '1,260p' scripts/tests/test_operational_product_samples.py`
- `sed -n '1,260p' samples/product-alpha1/operational/deployments/docker/docker-compose.operational.yml`
- `sed -n '1,260p' samples/product-alpha1/operational/deployments/local/deployment.profile.json`
- `sed -n '1,260p' samples/product-alpha1/operational/deployments/projection/projection.profile.json`
- `rg -n "transport-fixture|__product-transport-world-server|__product-transport-participant-client|docker-compose\\.operational|docker-compose\\.product-alpha1" -S crates/mirrorea-cli/src/main.rs samples/product-alpha1 -g '!target'`
- `nl -ba ...` focused reads for the cited lines in `crates/mirrorea-cli/src/main.rs`, `crates/mir-runtime/src/product_alpha1_devtools.rs`, `crates/mir-runtime/src/product_alpha1_session.rs`, `scripts/operational_product_samples.py`, `crates/mirrorea-cli/tests/alpha_cli.rs`, `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`, `specs/25-product-alpha1-public-boundary.md`, `specs/26-operational-product-sample-suite.md`, `docs/hands_on/operational_product_sample_01.md`, and `samples/product-alpha1/operational/README.md`
- `date '+%Y-%m-%d %H:%M:%S %z'`

## 9. Evidence / outputs / test results

- Review was static. No cargo test, helper workflow, Docker command, or sample runtime command was executed in this task.
- `git status --short` confirmed the worktree was already dirty before this report was added.
- The strongest implementation evidence came from code-path inspection of the CLI/runtime helper wiring and the current tests.

## 10. What changed in understanding

- The operational suite reuses much of the product alpha-1 carrier, but the operational Docker and bundle paths are not yet wired as a distinct operational workflow.
- The devtools surface currently satisfies panel presence/inventory more than semantic completeness for some required panels.
- The helper/test surface is currently stronger as a documentation/index smoke check than as a real operational closeout guard.

## 11. Open questions

- Should `mirrorea-alpha transport --mode docker` choose the compose file from the package root / deployment profile instead of one hard-coded demo compose path?
- Should operational native bundles include shared attach packages and deployment inventory so the bundle can replay the same attach flow documented for `P-OPS-01`?
- Should incomplete devtools panels carry explicit `kept_later` metadata instead of appearing complete with placeholder values?

## 12. Suggested next prompt

- `Fix the P-OPS-01 operational Docker/bundle wiring so transport uses the operational deployment compose file, build-native-bundle includes the shared attach packages needed by the operational workflow, and release-check/tests validate the deferred object/avatar boundaries and real panel payloads.`

## 13. `plan/` update status

- `plan/` update not performed. Review-only task; repository-memory edits were out of scope.

## 14. Documentation.md update status

- `Documentation.md` update not performed. Reviewed only.

## 15. progress.md update status

- `progress.md` update not performed. Reviewed only.

## 16. tasks.md update status

- `tasks.md` update not performed. Reviewed only.

## 17. samples_progress.md update status

- `samples_progress.md` update not performed. Reviewed only.

## 18. reviewer findings and follow-up

- Finding 1:
  `mirrorea-alpha transport --mode docker` is not operational-suite-specific. The CLI always resolves Docker transport through `samples/product-alpha1/docker/docker-compose.product-alpha1.yml`, not `samples/product-alpha1/operational/deployments/docker/docker-compose.operational.yml`, so the operational Sugoroku workflow never uses its own deployment artifact. The operational compose file is also not executable as written because it calls a nonexistent `transport-fixture` subcommand instead of the internal transport helper commands.
- Follow-up 1:
  Route Docker transport through the operational deployment file for operational roots and make the compose entrypoint match actual callable CLI commands.

- Finding 2:
  `build-native-bundle` does not preserve the operational attach workflow. It copies only the declared dependency tree and looks for `packages/debug-layer` underneath the root package directory, which works for `samples/product-alpha1/demo/` but not for `samples/product-alpha1/operational/sugoroku-world/`, where the shared attach packages live as siblings under `operational/packages/`. The resulting operational bundle can therefore omit attach packages and attach evidence even though the operational docs and completion condition treat attach behavior as part of the runnable workflow.
- Follow-up 2:
  Teach native bundle assembly to include the operational shared attach package set or a package-root-declared attach inventory, and extend tests beyond dependency-file presence.

- Finding 3:
  The devtools export overclaims panel completeness for route/config timelines. `message_route_graph` is emitted directly from `ProductAlpha1RouteGraph`, whose entries only contain envelope endpoints, lane flags, and transport lane; it does not carry the message-state / transport-contract / membership-incarnation / capability-requirement / dispatch-outcome fields required by `specs/25`. `membership_frontier_timeline` also hardcodes `config_epoch: 0`, and the current test locks that placeholder in rather than checking real config-frontier evidence or explicit `kept_later` wording.
- Follow-up 3:
  Either enrich these panels with the required fields or mark them explicitly as partial/kept-later instead of shipping them as if complete.

- Finding 4:
  The operational release helper and tests do not cover the full runnable claim they are used to advertise. `scripts/operational_product_samples.py release_check` marks the suite `accepted` after checking only debug/auth/rate-limit attach plus save/transport/export/bundle; it never exercises the deferred placeholder-object or custom-avatar-preview paths that `specs/26` requires as visible deferred boundaries. The dedicated Python tests only cover `list`/format behavior, and the Rust operational bundle test checks dependency files only.
- Follow-up 4:
  Make `release-check` verify the deferred object/avatar rows and add executable coverage for the operational helper, Docker selection, and bundled attach assets.

## 19. skipped validations and reasons

- Did not run the operational helper, Rust tests, or Docker commands because the task was a code review and the user requested findings only.
- Did not run end-to-end validation against Docker because the main issues were already visible in static command/file wiring.

## 20. commit / push status

- No commit created.
- No push performed.

## 21. sub-agent session close status

- No sub-agent used.
