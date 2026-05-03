# P-A1-02 Checker Surface Design Review

## 1. Title and identifier

- `review-2026-05-03-pa1-02-checker-surface-design-review`

## 2. Objective

- Review the proposed practical alpha-1 checker surface before implementation.
- Focus on theory/type-system and overclaim risk for:
  - optional alpha-local checker sections inside `package.mir.json`
  - a non-final checker report with `accepted_obligations` and `rejected_obligations`
  - initial actualized checker set `CHK-LIF-01..04`, `CHK-VAR-01..03`, `CHK-CUT-01`, `CHK-PKG-01/02`
- Decide whether the current typed package struct is acceptable as the first IR or whether a distinct lowered IR boundary is required immediately.

## 3. Scope and assumptions

- Scope followed the user-requested read order:
  - `specs/18-practical-alpha1-scope.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `sub-agent-pro/alpha-1/03-decisions.md`
  - `sub-agent-pro/alpha-1/04-stage-roadmap.md`
  - `sub-agent-pro/alpha-1/05-theory-freeze.md`
  - `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
  - `sub-agent-pro/alpha-1/08-sample-matrix.md`
  - `crates/mir-ast/src/practical_alpha1.rs`
- Additional reads were limited to existing alpha-0 helper carrier docs/scripts needed to judge overclaim boundaries.
- Review only; no implementation changes were made.

## 4. Start state / dirty state

- Worktree was clean at start.
- `HEAD` matched the user-specified commit `d942c95`.

## 5. Documents consulted

- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `sub-agent-pro/alpha-1/03-decisions.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/05-theory-freeze.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `crates/mir-ast/src/practical_alpha1.rs`
- `scripts/README.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- selected `samples/alpha/**/*.expected.json` rows for `LIF`, `VAR`, `CUT`, and package admission examples

## 6. Actions taken

- Read the requested practical alpha-1 spec/roadmap/handoff materials.
- Inspected the current `mir-ast` practical alpha-1 package loader and fixture shape.
- Inspected current alpha-0 helper-local checker, acceptance, snapshot, and runtime-mirror carriers to compare their scope boundaries against the proposed practical checker surface.
- Ran the focused `mir-ast` practical front-door test target.

## 7. Files changed

- `docs/reports/review-2026-05-03-pa1-02-checker-surface-design-review.md` (new)

## 8. Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git rev-parse --short HEAD`
- `git status --short`
- `rg --files specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md sub-agent-pro/alpha-1 crates/mir-ast/src/practical_alpha1.rs`
- `nl -ba specs/18-practical-alpha1-scope.md | sed -n '1,260p'`
- `nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '1,280p'`
- `nl -ba sub-agent-pro/alpha-1/03-decisions.md | sed -n '1,260p'`
- `nl -ba sub-agent-pro/alpha-1/04-stage-roadmap.md | sed -n '1,260p'`
- `nl -ba sub-agent-pro/alpha-1/05-theory-freeze.md | sed -n '1,280p'`
- `nl -ba sub-agent-pro/alpha-1/06-toolchain-architecture.md | sed -n '1,280p'`
- `nl -ba sub-agent-pro/alpha-1/08-sample-matrix.md | sed -n '1,300p'`
- `nl -ba crates/mir-ast/src/practical_alpha1.rs | sed -n '1,260p'`
- `rg -n "reason_codes_scope|acceptance_scope|snapshot_scope|runtime_mirror|accepted_obligations|rejected_obligations|alpha_lifetime_fallback_checker|alpha_contract_variance_checker|alpha_cut_save_load_checker|non-public checker floor" Documentation.md plan specs sub-agent-pro scripts crates -g '!target'`
- `nl -ba scripts/alpha_lifetime_fallback_checker.py | sed -n '1,260p'`
- `nl -ba scripts/alpha_contract_variance_checker.py | sed -n '1,280p'`
- `nl -ba scripts/alpha_cut_save_load_checker.py | sed -n '1,260p'`
- `nl -ba scripts/current_l2_family_checker_support.py | sed -n '1,260p'`
- `nl -ba scripts/current_l2_family_acceptance_support.py | sed -n '1,260p'`
- `nl -ba scripts/current_l2_family_snapshot_support.py | sed -n '1,240p'`
- `nl -ba scripts/README.md | sed -n '30,55p'`
- `nl -ba plan/39-type-system-freeze-roadmap.md | sed -n '20,45p'`
- `nl -ba plan/40-layer-compatibility-freeze-roadmap.md | sed -n '20,38p'`
- `nl -ba plan/41-save-load-checkpoint-roadmap.md | sed -n '20,42p'`
- `rg --files samples/alpha | rg 'lif-0[1-4]|lif-13|var-0[1-6]|cut-0[1-9]|cut-11|cut-17|package|native|capability'`
- `rg -n "checked_reason_codes|checked_acceptance_rows|checked_snapshot_rows|runtime_mirror|kind" samples/alpha/lifetime-fallback samples/alpha/contract-variance samples/alpha/cut-save-load -g '*.expected.json'`
- `nl -ba samples/alpha/lifetime-fallback/lif-01-raw_dangling_reference_rejected.expected.json | sed -n '1,120p'`
- `nl -ba samples/alpha/lifetime-fallback/lif-02-fallback_extends_access_path.expected.json | sed -n '1,120p'`
- `nl -ba samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json | sed -n '1,140p'`
- `nl -ba samples/alpha/contract-variance/var-01-logging_layer_valid.expected.json | sed -n '1,120p'`
- `nl -ba samples/alpha/contract-variance/var-02-precondition_strengthening_rejected.expected.json | sed -n '1,120p'`
- `nl -ba samples/alpha/cut-save-load/cut-01-local_try_rollback_before_cut.expected.json | sed -n '1,120p'`
- `nl -ba samples/alpha/cut-save-load/cut-05-inconsistent_distributed_snapshot_rejected.expected.json | sed -n '1,120p'`
- `nl -ba samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json | sed -n '1,140p'`
- `rg --files samples/practical-alpha1/packages`
- `nl -ba samples/practical-alpha1/packages/src-01-minimal-world/package.mir.json | sed -n '1,220p'`
- `nl -ba samples/practical-alpha1/packages/src-04-layer-manifest/package.mir.json | sed -n '1,240p'`
- `cargo test -p mir-ast --test practical_alpha1_front_door`

## 9. Evidence / outputs / test results

- `cargo test -p mir-ast --test practical_alpha1_front_door` passed: 11 passed, 0 failed.
- The requested spec/roadmap/handoff materials consistently require:
  - `parser / loader -> alpha IR -> checker` separation
  - non-final alpha-local checker surface
  - no freeze of a final public checker API
- Existing alpha-0 checker evidence is explicitly split into separate non-public carriers:
  - `reason_codes_scope = alpha-static-floor`
  - `acceptance_scope = alpha-acceptance-floor`
  - `snapshot_scope = alpha-snapshot-selected-floor`
  - `runtime_mirror.scope = alpha-runtime-mirror-floor`
- Current practical package fixtures are still narrow front-door inputs with only:
  - `format_version`
  - `package_id`
  - `package_kind`
  - `world`
  - `places`
  - `fallback_chains`
  - `layers`
  - `manifest`

## 10. What changed in understanding

- The strongest design constraint is not crate placement but carrier separation.
- Keeping the loader in `mir-ast` is compatible with the current roadmap.
- Reusing the current parsed package struct directly as the checker IR would blur the required `loader -> IR -> checker` boundary and make the checker surface look more final than the docs allow.

## 11. Open questions

- Should practical checker expectations live inside `package.mir.json` only for sample/test fixtures, or are they intended to be part of normal package authoring for alpha users?
- For `CHK-PKG-02`, is “over-capability” measured against:
  - package manifest self-declaration only, or
  - alpha-local host policy / attachpoint policy preview?
- If the latter, that policy must remain clearly alpha-local and non-runtime-final.

## 12. Suggested next prompt

- `Design P-A1-02 with a distinct PracticalAlpha1CheckerInput / CheckerReport boundary in mir-ast, namespace any package-embedded checker expectations under alpha_local_*, and define explicit report fields for non-final scope, accepted/rejected obligations, and retained-later obligations before writing code.`

## 13. `plan/` update status

- `plan/` update not performed. Review-only task.

## 14. `Documentation.md` update status

- `Documentation.md` update not performed. Review-only task.

## 15. `progress.md` update status

- `progress.md` update not performed. Review-only task.

## 16. `tasks.md` update status

- `tasks.md` update not performed. Review-only task.

## 17. `samples_progress.md` update status

- `samples_progress.md` update not performed. Review-only task.

## 18. reviewer findings and follow-up

- Finding 1:
  A distinct lowered checker IR boundary is required immediately, even if it stays in `mir-ast` for now. The requested design sources require `parser / loader -> alpha IR -> checker` rather than “parsed package struct doubles as checker IR” (`specs/18-practical-alpha1-scope.md:51-56`, `plan/44-practical-alpha1-roadmap.md:69-74`, `sub-agent-pro/alpha-1/03-decisions.md:25-28`). The current `PracticalAlpha1Package` is still a front-door carrier with raw `String`/`Vec<String>` fields for world, places, fallback chains, layers, and manifest only (`crates/mir-ast/src/practical_alpha1.rs:81-135`). It is acceptable as the parsed package carrier, but not as the checker’s first IR if `P-A1-02` wants a defensible theory boundary.
- Finding 2:
  `accepted_obligations` and `rejected_obligations` alone are too coarse to keep alpha-0 helper-local carriers from collapsing into a seemingly final checker surface. Existing alpha-0 evidence is intentionally split across separate non-public carriers and scopes (`scripts/README.md:37-46`, `plan/39-type-system-freeze-roadmap.md:25-38`, `plan/40-layer-compatibility-freeze-roadmap.md:23-34`, `scripts/current_l2_family_checker_support.py:72-105`, `scripts/current_l2_family_acceptance_support.py:121-163`, `scripts/current_l2_family_snapshot_support.py:121-168`). A practical checker report therefore needs explicit per-row or per-section boundary markers such as:
  - obligation family (`lifetime_fallback`, `contract_variance`, `cut`, `package_admission`)
  - judgment class (`static_accept`, `static_reject`, `snapshot_accept`, `package_policy_reject`)
  - scope kind (`alpha-local`, non-final)
  - retained-later / not-checked set for runtime-mirror and runtime-plan obligations
  Without that, `CHK-LIF-04` and `CHK-PKG-01/02` will read like the practical checker has already subsumed snapshot/runtime-sensitive carriers.
- Finding 3:
  If optional checker sections are embedded in `package.mir.json`, they must be namespaced as alpha-local expectation input, not as package semantics or a public checker schema. Bare names such as `checker`, `expected_static`, or `accepted_obligations` on the package input side would read too much like promoted alpha-0 sidecars or a stabilized public authoring surface. The safer boundary is:
  - package input: `alpha_local_checker_*` expectation/request sections only
  - checker output: a separate non-final report carrier
  This matches the current front-door’s explicit non-final manifest pattern (`crates/mir-ast/src/practical_alpha1.rs:12-18`, `crates/mir-ast/src/practical_alpha1.rs:34-49`) and avoids embedding verdicts back into the input surface.
- Finding 4:
  `CHK-PKG-01/02` need an explicit non-claim that they are package-admission preview checks, not hot-plug/runtime attach verdicts. `specs/18` keeps package admission in typed checking (`specs/18-practical-alpha1-scope.md:77-93`) but places runtime attach / request / verdict / activation cut in later reusable runtime and hot-plug stages (`specs/18-practical-alpha1-scope.md:95-107`, `plan/44-practical-alpha1-roadmap.md:76-88`). Existing alpha package rejection evidence also lives in runtime-private admission reports, not a public checker ABI (`samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json:15-80`). So `P-A1-02` must say plainly that `CHK-PKG-01/02` check alpha-local manifest/policy consistency only and do not imply runtime admission completion.

## 19. skipped validations and reasons

- Did not run repo-wide doc or workspace validation because this was a narrow design review with no implementation change.
- Did not run alpha runtime/hot-plug samples because the review target was the proposed checker surface boundary, not runtime behavior.

## 20. commit / push status

- No commit created.
- No push performed.

## 21. sub-agent session close status

- No sub-agent used.
