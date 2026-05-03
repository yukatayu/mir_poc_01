# Report 1138 — P-A1-02 sample/validation design review

- Date: 2026-05-03 16:13:36 JST
- Author / agent: Codex
- Scope: Review `P-A1-02` practical checker sample/validation design against commit `d942c95`, focused on `samples/practical-alpha1/`, `crates/mir-ast/tests/practical_alpha1_front_door.rs`, `crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs`, `specs/18` typed-checking floor, and `sub-agent-pro/alpha-1/08` + `09`
- Decision levels touched: None

## Objective

Assess whether the proposed `P-A1-02` direction has a sufficient minimum sample set, an honest expected-artifact shape, and explicit regression anchors so a practical checker does more than "not reject".

## Scope and assumptions

- Review-only task. No production code or spec edits were requested.
- Analysis is anchored to commit `d942c953698eb1bc75b051bc99418cef2f287aca` (`d942c95`), which is the current `HEAD` on `main`.
- The proposed direction under review is: practical checker sample packages plus expected checker artifacts for `CHK-LIF-01..04`, `CHK-VAR-01..03`, `CHK-CUT-01`, `CHK-PKG-01/02`, with `scripts/practical_alpha1_check.py` as a front-door over a Rust checker implementation.

## Start state / dirty state

- Worktree was dirty at start due to a pre-existing untracked file: `docs/reports/review-1139-p-a1-02-boundary.md`.
- I did not modify or remove that pre-existing file.

## Documents consulted

- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `crates/mir-ast/tests/practical_alpha1_front_door.rs`
- `crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs`
- `scripts/README.md`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_family_acceptance_support.py`
- representative alpha expected sidecars for comparison:
  - `samples/alpha/lifetime-fallback/lif-01-raw_dangling_reference_rejected.expected.json`
  - `samples/alpha/lifetime-fallback/lif-02-fallback_extends_access_path.expected.json`

## Actions taken

- Read the named alpha-1 handoffs first, then the repository baseline docs in AGENTS.md order.
- Read the current practical alpha-1 front-door sample root and focused Rust tests.
- Compared the proposed checker sample set with the normative typed-checking bullet list in `specs/18`.
- Compared current practical front-door expected artifacts with existing alpha checker/acceptance artifact shapes.
- Ran the current practical front-door test target as a baseline.

## Files changed

- `docs/reports/1138-p-a1-02-sample-validation-design-review.md` (new report)

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git rev-parse --verify d942c95^{commit}
git branch --contains d942c95
sed -n '1,260p' sub-agent-pro/alpha-1/08-sample-matrix.md
sed -n '1,260p' sub-agent-pro/alpha-1/09-validation-plan.md
git rev-parse HEAD
git show d942c95:README.md | sed -n '1,220p'
git show d942c95:Documentation.md | sed -n '1,260p'
git show d942c95:specs/00-document-map.md | sed -n '1,240p'
git show d942c95:specs/01-charter-and-decision-levels.md | sed -n '1,240p'
git show d942c95:specs/02-system-overview.md | sed -n '1,240p'
git show d942c95:specs/03-layer-model.md | sed -n '1,240p'
git show d942c95:specs/09-invariants-and-constraints.md | sed -n '1,240p'
git show d942c95:specs/18-practical-alpha1-scope.md | sed -n '60,120p'
git ls-tree -r --name-only d942c95 -- samples/practical-alpha1 crates/mir-ast/tests/practical_alpha1_front_door.rs crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs | sort
git show d942c95:crates/mir-ast/tests/practical_alpha1_front_door.rs | sed -n '1,220p'
git show d942c95:crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs | sed -n '1,260p'
git show d942c95:scripts/README.md | rg -n "practical alpha-1|practical_alpha1_check|checker"
git show d942c95:plan/44-practical-alpha1-roadmap.md | rg -n "P-A1-02|typed IR|checker|practical_alpha1_check|CHK-"
rg -n "checked_reason_codes|checked_acceptance_rows|expected_static|expected_acceptance|reason_codes_scope|acceptance_scope" scripts samples/alpha -g '!target'
sed -n '1,260p' scripts/current_l2_family_checker_support.py
sed -n '1,220p' scripts/current_l2_family_acceptance_support.py
nl -ba specs/18-practical-alpha1-scope.md | sed -n '68,95p'
nl -ba sub-agent-pro/alpha-1/08-sample-matrix.md | sed -n '1,110p'
nl -ba sub-agent-pro/alpha-1/09-validation-plan.md | sed -n '1,80p'
nl -ba crates/mir-ast/tests/practical_alpha1_front_door.rs | sed -n '1,180p'
nl -ba crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs | sed -n '1,220p'
nl -ba samples/practical-alpha1/expected/README.md | sed -n '1,80p'
nl -ba samples/alpha/lifetime-fallback/lif-01-raw_dangling_reference_rejected.expected.json | sed -n '1,50p'
nl -ba samples/alpha/lifetime-fallback/lif-02-fallback_extends_access_path.expected.json | sed -n '1,50p'
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `git rev-parse HEAD` returned `d942c953698eb1bc75b051bc99418cef2f287aca`, matching the requested scope commit.
- `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture`
  - Passed: 11/11 tests in `tests/practical_alpha1_front_door.rs`
- Current practical front-door uses per-fixture source/manifest packages plus per-fixture expected sidecars.
- Current alpha checker floor separates negative typed rows from positive acceptance rows; positive evidence is not modeled as empty diagnostics.

## What changed in understanding

- The strongest design risk for `P-A1-02` is not the script wrapper itself, but under-specified checker evidence.
- The practical line already has a good front-door pattern: real package inputs plus expected sidecars. The next checker layer should preserve that, but it should not copy the alpha evidence scaffold wholesale.
- The proposed `CHK-*` set is close to the matrix, but still undershoots the normative `specs/18` typed-checking floor unless additional rows are added.

## Open questions

- Should practical checker expected artifacts live as separate files under `samples/practical-alpha1/expected/`, or as checker sections inside per-package expected files?
- Is `CHK-PKG-02` intended to stand in for capability monotonicity, or should capability monotonicity be split into its own named checker row?
- Should `effect row containment`, `failure row containment`, and `observation/redaction/retention constraints` enter `P-A1-02` immediately, or be promoted as explicit residual rows with an `UNRESOLVED` note elsewhere? Current `specs/18` wording suggests they belong in the floor, not later.

## Suggested next prompt

Design the `P-A1-02` checker sample set and expected artifact schema so it covers every `specs/18` typed-checking bullet with explicit positive acceptance rows and explicit negative diagnostics, then add focused tests for `scripts/practical_alpha1_check.py` that exercise one package fixture per `CHK-*` row in addition to the aggregate command.

## Plan update status

`plan/` 更新不要 / review-only task; no roadmap-memory edit applied.

## Documentation.md update status

`Documentation.md` 更新不要 / review-only task; no snapshot wording edit applied.

## progress.md update status

`progress.md` 更新不要 / review-only task; no status snapshot edit applied.

## tasks.md update status

`tasks.md` 更新不要 / review-only task; no task-map edit applied.

## samples_progress.md update status

`samples_progress.md` 更新不要 / review-only task; no dashboard edit applied.

## Reviewer findings and follow-up

- Finding 1: the proposed `CHK-LIF-01..04`, `CHK-VAR-01..03`, `CHK-CUT-01`, `CHK-PKG-01/02` set is not yet a spec-complete minimum. `specs/18` requires effect-row containment, failure-row containment, capability monotonicity, observation/redaction/retention constraints, invalid distributed cut rejection, and package manifest admission as part of the typed-checking floor. The current matrix/proposal covers raw dangling, fallback, inherited/snapshot distinction, one positive logging case, two negative variance cases, one cut reject, and two package rejects, but it does not name effect/failure containment or observation/redaction/retention at all, and it has no positive package-manifest admission row.
- Finding 2: a single `scripts/practical_alpha1_check.py samples/practical-alpha1/packages/sugoroku-space --format json` command is not a sufficient regression anchor by itself. The sample matrix completion rule requires expected output/diagnostics and positive/negative validation per sample, and the current practical front-door already uses one package fixture plus one expected sidecar per row. `P-A1-02` should keep row-keyed fixtures and focused tests on top of the aggregate command; otherwise a product-like smoke check will only prove “some package checked” and will not localize which checker obligation regressed.
- Finding 3: expected checker artifacts must carry explicit positive evidence, not just `accepted: true` or “no diagnostics.” `specs/18` explicitly forbids treating positive proof as absence of negative reason codes. The current practical front-door negative shape (`accepted` + `error_kind`) is fine for parser errors, but it is too weak for typed checking. The existing alpha checker floor shows the needed split: explicit negative rows (`expected_static.checked_reason_codes`) and explicit positive acceptance rows (`expected_acceptance.checked_acceptance_rows`). A practical checker artifact should preserve that separation, even if the exact field names change.
- Finding 4: do not reuse the alpha evidence sidecar schema wholesale for practical checker artifacts. The alpha files include `status`, `phase`, `stage`, `claims`, `current_validation`, and `deferred` fields tied to a synthetic, non-public checker floor. That would be dishonest once `scripts/practical_alpha1_check.py` fronts a real Rust checker over practical package inputs. Practical checker expectations should stay machine-check-oriented: verdict, structured diagnostics, explicit acceptance rows/witnesses, and any scope/version tag needed to detect cross-floor mismatches.

## Skipped validations and reasons

- Did not run `python3 scripts/validate_docs.py`, `cargo fmt --check`, or `git diff --check`. This task was a design review with a report-only file addition, and current repository state includes pre-existing report/worktree dirt that would confound repo-wide validation unrelated to the requested checker-sample review.
- Did not run any `scripts/practical_alpha1_check.py` command because that script does not exist yet at `d942c95`; the task was to review the design before implementation.

## Commit / push status

Not committed. Not pushed.

## Sub-agent session close status

No sub-agent sessions were opened.
