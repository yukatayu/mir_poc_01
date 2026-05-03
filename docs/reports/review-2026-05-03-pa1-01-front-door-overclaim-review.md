# P-A1-01 Front-Door Overclaim Review

## 1. Title and identifier

- `2026-05-03-pa1-01-front-door-overclaim-review`

## 2. Objective

- Review `P-A1-01` practical alpha-1 front-door changes for semantics/type-system overclaim only.
- Confirm whether the new front-door is clearly non-final, library-first, limited to `package.mir.json`, and not overclaimed as checker/runtime/product execution.

## 3. Scope and assumptions

- Scope was limited to `README.md`, `Documentation.md`, `specs/18-practical-alpha1-scope.md`, `plan/44-practical-alpha1-roadmap.md`, and `crates/mir-ast/src/practical_alpha1.rs`.
- `plan/00-index.md` and the required baseline docs were also read to satisfy repository reading order and boundary rules.
- Review mode only; no implementation changes were made.

## 4. Start state / dirty state

- Worktree was already dirty at start.
- Pre-existing modified or untracked files included `Documentation.md`, `README.md`, `plan/44-practical-alpha1-roadmap.md`, `specs/18-practical-alpha1-scope.md`, `crates/mir-ast/src/practical_alpha1.rs`, related tests, and other unrelated files.
- This task did not alter those in-flight changes.

## 5. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`

## 6. Actions taken

- Read the required repository guidance and baseline specs in the mandated order.
- Inspected the practical alpha-1 scope, roadmap, snapshot docs, and `mir-ast` front-door implementation for overclaim language and boundary enforcement.
- Inspected the new focused tests to see what the front-door contract is currently exercising.
- Ran the practical alpha-1 front-door test target for current evidence.

## 7. Files changed

- `docs/reports/2026-05-03-pa1-01-front-door-overclaim-review.md` (new)

## 8. Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `rg --files README.md Documentation.md specs plan crates/mir-ast/src | sed -n '1,200p'`
- `git status --short`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,240p' specs/00-document-map.md`
- `sed -n '1,240p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,240p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `nl -ba plan/00-index.md | sed -n '1,220p'`
- `nl -ba specs/18-practical-alpha1-scope.md | sed -n '1,260p'`
- `nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '1,280p'`
- `nl -ba crates/mir-ast/src/practical_alpha1.rs | sed -n '1,260p'`
- `rg -n "practical alpha-1|package.mir.json|front-door|checker|runtime|product|library-first|non-final|final textual grammar|source-executed|samples/practical-alpha1|practical_alpha1" README.md Documentation.md specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md crates/mir-ast/src/practical_alpha1.rs`
- `nl -ba README.md | sed -n '1,240p'`
- `nl -ba Documentation.md | sed -n '1,220p'`
- `nl -ba crates/mir-ast/tests/practical_alpha1_front_door.rs | sed -n '1,260p'`
- `nl -ba crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs | sed -n '1,260p'`
- `rg -n "load_practical_alpha1_package_path|parse_practical_alpha1_package_text|package.mir.json|MissingPackageFile|practical_alpha1_front_door_manifest" crates/mir-ast/tests crates/mir-ast/src -g '!target'`
- `nl -ba crates/mir-ast/src/lib.rs | sed -n '1,120p'`
- `cargo test -p mir-ast --test practical_alpha1_front_door`

## 9. Evidence / outputs / test results

- `cargo test -p mir-ast --test practical_alpha1_front_door` passed: 6 passed, 0 failed.
- The targeted docs consistently state that the current cut is:
  - non-final
  - library-first
  - a limited `package.mir.json` front-door
  - not checker/runtime/product completion
- The implementation keeps explicit non-final markers through:
  - `carrier_kind: "practical_alpha1_nonfinal_package_json_front_door"`
  - retained-later refs for `typed_ir_checker_integration` and `runtime_plan_execution`
- The implementation does not enforce the claimed filename restriction:
  - directory input is rewritten to `package.mir.json`
  - direct file input is accepted if it exists, regardless of filename

## 10. What changed in understanding

- Most of the reviewed surfaces are careful about non-final and non-executing wording.
- The only concrete overclaim mismatch found is narrower: the docs describe the current front-door as limited to `package.mir.json`, but the path loader accepts any existing file path containing the same JSON shape.

## 11. Open questions

- Should `P-A1-01` treat `package.mir.json` as a literal filename contract, or only as the current schema family name?
- If the filename itself is part of the front-door boundary, a negative test for non-`package.mir.json` filenames is currently missing.

## 12. Suggested next prompt

- `Tighten P-A1-01 so load_practical_alpha1_package_path only accepts package directories or files literally named package.mir.json, add a negative test for alternate filenames, and keep the front-door non-final wording unchanged.`

## 13. `plan/` update status

- `plan/` update not performed. Review-only task; no repository-memory change proposed here.

## 14. `Documentation.md` update status

- `Documentation.md` update not performed. Existing wording was reviewed only.

## 15. `progress.md` update status

- `progress.md` update not performed. Review-only task; no status snapshot change made here.

## 16. `tasks.md` update status

- `tasks.md` update not performed. Review-only task; no task-map rewrite made here.

## 17. `samples_progress.md` update status

- `samples_progress.md` update not performed. Review-only task; no sample dashboard change made here.

## 18. reviewer findings and follow-up

- Finding:
  `crates/mir-ast/src/practical_alpha1.rs:176-193` accepts any existing file path, while the reviewed docs describe the current cut as a limited `package.mir.json` front-door (`README.md:54`, `Documentation.md:63`, `specs/18-practical-alpha1-scope.md:72-73`, `plan/44-practical-alpha1-roadmap.md:30`, `plan/44-practical-alpha1-roadmap.md:188`, `plan/44-practical-alpha1-roadmap.md:200`). This means the implementation is looser than the claimed current surface boundary. The test suite also lacks a regression that rejects alternate filenames (`crates/mir-ast/tests/practical_alpha1_front_door.rs:50-99`).
- Follow-up:
  Either enforce the literal filename in `load_practical_alpha1_package_path`, or relax the wording everywhere from “limited `package.mir.json` front-door” to “limited package-schema JSON front-door.” The former is the safer match for the stated scope.

## 19. skipped validations and reasons

- Did not run repo-wide validation (`validate_docs.py`, `cargo fmt --check`, full test matrix) because the task was a narrow review of overclaim semantics, not a package closeout or implementation change.
- Did not execute ad hoc runtime/checker flows because the review scope explicitly excluded runtime/product claims and the front-door implementation does not provide those surfaces yet.

## 20. commit / push status

- No commit created.
- No push performed.

## 21. sub-agent session close status

- No sub-agent used.
