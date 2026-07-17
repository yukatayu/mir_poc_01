# Report 2255 - bounded runnable front-door audit

- Date: 2026-07-17 09:15 JST
- Author / agent: Codex
- Scope: existing runnable LAB front doors and their documented evidence scope
- Decision levels touched: none; LAB validation and snapshot maintenance only

## Objective

Reproduce the repository's existing runnable front doors after T-RESEARCH-003,
check their bounded claims against current documentation, and record a
reproducible audit without changing canon or implementation scope.

## Scope and assumptions

The audit uses only existing commands, sample roots, and temporary output
directories. A passing command is evidence that its documented bounded workflow
reproduces locally; it is not a Gate/Phase transition, proof discharge,
conformance result, final public interface, real transport, or distributed
runtime claim.

## Start state / dirty state

Started at `1456c47d` (`Record E-OBS append kernel research`) with a clean
worktree. `target/` was an existing 7.0 GiB build directory; `/mnt/mirrorea-work`
was not mounted, so no new heavy artifact directory was created. Root storage
had about 21 GiB free and memory had about 10 GiB available.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and the canon phase,
  operating-model, and metatheory-ledger entries named by them
- `AGENTS.md`, `CANON.md`, `README.md`, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`, and
  `samples_progress.md`
- `plan/147-g1-next-line-promotion-boundary-audit.md` and
  `plan/156-t0-t2-research-autonomy-envelope.md`
- `samples/README.md`, `scripts/README.md`, and `docs/reports/TEMPLATE.md`

## Actions taken

- Audited resources and retained all new release-check outputs under `/tmp`.
- Reproduced documentation, Python, workspace Cargo, Surface, Full System V1,
  Product Alpha, installed-binary, operational, and current-L2 front doors.
- Inspected each aggregate report's explicit non-claims before recording the
  result in LAB snapshots.

## Files changed

- `docs/reports/2255-bounded-runnable-front-door-audit.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

## Commands run

- `make check`
- `python3 -m unittest discover -s scripts/tests`
- `cargo test --workspace`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-audit-20260717`
- `python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release-audit-20260717`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-audit-20260717`
- `python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-audit-20260717`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
- `python3 scripts/current_l2_lean_sample_sync.py --format json`
- resource, Git-status, and focused document/dashboard inspections

## Evidence / outputs / test results

- `make check` passed documentation hierarchy/validation and Cargo check.
- The full Python script suite passed 806 tests; workspace Cargo tests completed
  with no failures.
- Surface release was ready with 53 helper rows accepted and no failed command.
- Full System V1 release accepted all 30 planned commands; its own report keeps
  final grammar/API, packet/FFI semantics, binary split, provider execution,
  WAN/federation, and distributed durable save/load out of scope.
- Product Alpha release accepted 29/29 commands; installed-binary accepted
  11/11 and identifies its delivery only as a developer-built binary plus
  generated native host-launch bundle.
- Operational `check-all` accepted with no failed command, including bounded
  local/Docker evidence and observer-safe devtools; it does not claim a final
  shared-space catalog or distributed durability.
- Current-L2 smoke/closeout reproduced 16 clean-near-end samples; Lean manifest
  sync completed and left the worktree unchanged.

## What changed in understanding

The existing runnable surfaces are internally reproducible as bounded LAB
evidence and the documentation's separation between those surfaces and canon
implementation/proof status remains accurate. The audit found no drift that
would justify promotion, a new helper lane, or a new research work unit.

## Open questions

- Which existing canon-grounded rule/clause, if any, satisfies the next
  `plan/156` research selection criterion remains unselected.
- G0-D3 remains dormant until the owner explicitly reopens it.
- Later runtime, conformance, public grammar/API, transport, persistence, and
  final catalog decisions remain outside this audit.

## Suggested next prompt

Select an existing canon-grounded T0-T2 rule/clause with an explicit source cut
and falsification criterion, or ask for a decision bundle for an owner-level
semantic or lifecycle choice.

## Plan update status

`plan/` 更新不要: the audit did not add research evidence or change the
selection/stop protocol in `plan/156`.

## Documentation.md update status

`Documentation.md` 更新不要: its concise reading order and current-position
statement remain accurate.

## docs/project-status.md update status

更新不要: canon lifecycle, research authorization, current stop line, and
owner decision queue did not change.

## progress.md update status

`progress.md` 更新済み: added a dated LAB validation log while preserving all
existing status and non-claim wording.

## tasks.md update status

`tasks.md` 更新済み: recorded that the audit closed without selecting or
promoting a successor work unit.

## samples_progress.md update status

`samples_progress.md` 更新済み: added the fresh reproducibility sweep with
commands, pass results, artifact location, and non-claims.

## Reviewer findings and follow-up

No new independent review was requested because this package makes no semantic
or lifecycle judgment. The preceding T-RESEARCH-003 Oracle review remains
closed and already requires escalation before its conditional `[E-OBS]` model
can be treated as canon semantics. Local aggregate-report and documentation
review found no claim conflict.

## Skipped validations and reasons

- No new code was added, so no new feature-specific test was applicable.
- Historical and planned-only helper families were not re-run individually:
  the active and aggregate front doors above cover the documented current
  runnable surfaces without treating planned roots as active.
- No external workdir cleanup was attempted; it was absent and no new heavy
  repository artifact was created.

## Commit / push status

Pending at report write; final documentation and focused validation will run
before committing with `--no-gpg-sign` and pushing.

## Sub-agent session close status

No sub-agent was opened for this mechanical reproducibility package. The two
Oracle consultations used for T-RESEARCH-003 are complete; no external session
is pending.
