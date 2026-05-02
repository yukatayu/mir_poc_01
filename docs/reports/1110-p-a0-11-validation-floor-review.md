# Report 1110 — P-A0-11 validation floor review

- Date: 2026-05-02 09:51 JST
- Author / agent: Codex
- Scope: Review validation evidence expectations and reporting obligations for `P-A0-11` Mirrorea Spaces alpha demo closeout
- Decision levels touched: none; review only

## Objective

Review the existing `P-A0-11` package expectations and recommend a focused validation floor and report obligations for an integrated demo package that composes the current Stage B/C/D floors without overclaiming Stage F completion.

## Scope and assumptions

- This task reviewed the handoff/package protocol, current snapshot docs, alpha roadmap memory, and script taxonomy only.
- No normative spec text was changed.
- Recommendation assumes `P-A0-11` may compose the existing Stage B/C/D narrow cuts plus limited Stage-E evidence, rather than widening into full distributed save/load, final devtools, or public-boundary work.

## Start state / dirty state

- `git status --short` was clean at task start.

## Documents consulted

- `sub-agent-pro/alpha-0/11-validation-commit-push-protocol.md`
- `sub-agent-pro/alpha-0/12-codex-task-packages.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/e2e/README.md`
- `docs/reports/1105-alpha0-runtime-package-avatar-first-cut.md`

## Actions taken

- Read the required handoffs in the user-specified order, then the repository-mandated docs/spec sequence.
- Cross-checked `P-A0-11` package wording against the alpha scope spec and alpha E2E roadmap.
- Verified the actual active Alpha-0 runner surfaces with `--help`.
- Derived a focused validation floor for an integrated demo closeout that composes existing Stage B/C/D evidence without claiming broader completion.

## Files changed

- `docs/reports/1107-p-a0-11-validation-floor-review.md` (new report)

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `rg --files -g 'README.md' -g 'Documentation.md' -g 'progress.md' -g 'tasks.md' -g 'samples_progress.md' -g 'scripts/README.md' -g 'specs/*' -g 'sub-agent-pro/alpha-0/*.md' -g 'plan/*.md'`
- `sed -n '1,260p' ...` over the consulted docs listed above
- `rg -n "P-A0-11|integrated alpha demo|alpha demo closeout|Stage F" -S .`
- `python3 scripts/alpha_network_docker_e2e.py --help`
- `python3 scripts/alpha_avatar_runtime_samples.py --help`
- `git status --short`
- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Evidence / outputs / test results

- `python3 scripts/alpha_network_docker_e2e.py --help` confirmed the actual runner name is `alpha_network_docker_e2e.py` and that it exposes `list`, `run`, `check-all`, and `closeout`.
- `python3 scripts/alpha_avatar_runtime_samples.py --help` confirmed the avatar/package runner also exposes `list`, `run`, `check-all`, and `closeout`.
- `python3 scripts/check_source_hierarchy.py` passed with `required: 60`, `present: 60`, `missing: 0`.
- `python3 scripts/validate_docs.py` passed with `Documentation scaffold looks complete.` and `Found 1110 numbered report(s).`
- `git diff --check` returned clean.
- Review finding: `sub-agent-pro/alpha-0/11-validation-commit-push-protocol.md` still names `python3 scripts/alpha_network_docker_samples.py check-all --format json`, which does not match the current active script taxonomy.
- Review finding: current snapshot docs still mark Alpha visualization and integrated E2E as scaffold-only (`10%`), so `P-A0-11` cannot honestly be described as Stage-F complete unless runnable evidence is added or the package wording stays at focused closeout level.

## What changed in understanding

- The core package intent for `P-A0-11` is narrower than “finish Stage F.” The repo consistently frames it as an integrated demo closeout package, not final alpha completion.
- The strongest current floor is to require one integrated demo command plus the already-existing Stage B/C/D focused checks, rather than forcing a full repository-wide regression rerun as the package minimum.
- Save/load and typed devtools remain the main honesty gates. They must be satisfied either by narrow runnable evidence or by explicit checker-backed non-claims.

## Open questions

- Should `P-A0-11` satisfy the save/load condition with a local-only runnable demo, or with explicit reuse of the CUT checker plus a documented distributed-save non-claim?
- Should Stage-E evidence for `P-A0-11` be a minimal typed debug artifact emitted by the integrated demo, or a composition proof that reuses the existing viewer inventory without promoting `A0-VIS` to runnable status?
- Should the first integrated demo remain placeholder/custom-Mir avatar only, or include planned-only adapter rows as non-executable appendix evidence?

## Suggested next prompt

Implement `P-A0-11` as a focused integrated demo closeout: add one demo entrypoint that composes the current Stage B/C/D floors, emit typed debug evidence, document the save/load and public-boundary stop lines explicitly, update the validation floor docs to use the real runner names, and close with a report that records any skipped Docker or save/load validations honestly.

## Plan update status

`plan/` 更新不要: this task reviewed existing roadmap memory only.

## Documentation.md update status

`Documentation.md` 更新不要: no current snapshot wording was changed by this review-only task.

## progress.md update status

`progress.md` 更新不要: the current package, phase, and blockers were reviewed but not changed.

## tasks.md update status

`tasks.md` 更新不要: `P-A0-11` already remains the next package with the correct completion condition.

## samples_progress.md update status

`samples_progress.md` 更新不要: this task did not change runnable status or evidence levels.

## Reviewer findings and follow-up

- `sub-agent-pro/alpha-0/11-validation-commit-push-protocol.md` has stale Docker-runner naming for the Alpha-0 network floor and should be updated before using it as the `P-A0-11` validation source of truth.
- `P-A0-11` should not use docs-only completion wording. The scope spec and alpha E2E roadmap require integrated evidence for runtime, network, hot-plug, debug/devtools, avatar path, save/load-or-non-claim, and the negative test floor.
- A focused validation floor is sufficient if it includes:
  - `cargo test -p mir-runtime --test alpha_local_runtime --test alpha_layer_insertion_runtime --test alpha_network_runtime --test alpha_avatar_runtime`
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- local-sugoroku`
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- closeout`
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout`
  - `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`
  - `python3 scripts/alpha_network_docker_e2e.py check-all --format json`
  - `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
  - one integrated demo command for `P-A0-11`
  - docs/report floor: `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, `cargo fmt --check`, `git diff --check`
- If Docker is unavailable, the report must mark the Docker floor as skipped and must not claim Stage-F completion.

## Skipped validations and reasons

- No runtime, Docker, or Cargo validation was executed in this review-only task because the objective was to assess the required floor, not to implement or rerun `P-A0-11`.
- The required docs/report floor was executed after writing this report: `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, and `git diff --check` all passed.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No sub-agent session was started for this task.
