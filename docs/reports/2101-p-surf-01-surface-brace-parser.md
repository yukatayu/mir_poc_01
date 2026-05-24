# Report 2101 — P-SURF-01 Surface Brace Parser

- Date: 2026-05-24 15:38 JST
- Author / agent: Codex
- Scope: `P-SURF-01 surface brace parser`
- Decision levels touched: L1/L2 parser-floor evidence for `S { ... }` Surface Mir syntax; no final public grammar claim

## Objective

Implement the narrow Surface Mir parser floor for canonical `S { ... }`
place-scope syntax, reject `S[ ... ]`, parse the P-SURF-01 declaration/block
surface, add executable parser samples and helper scripts, synchronize docs, run
validation, and prepare the repository for `P-SURF-02 indexed-state semantics`.

## Scope and assumptions

This package is parser evidence only. It accepts `.mir` source as semantic
authority and keeps `package.mir.json` as an alpha artifact. It does not claim
Surface-to-Core elaboration, indexed-state authority semantics, runtime
execution, generated communication, role-admission authority, source-patch
activation, final grammar, ABI, or SDK completion.

## Start state / dirty state

The branch started from pushed `P-SURF-00B` commit
`32707c78898d1aecd9a70d3aef1b23707ec92757`. The handoff directory
`sub-agent-pro/surface-mir-brace-completion-001/` was present as untracked local
directive material and was intentionally not staged as normative source.

## Documents consulted

Consulted the required repo entry points and status documents: `README.md`,
`Documentation.md`, `AGENTS.md`, `progress.md`, `tasks.md`,
`samples_progress.md`, the ordered base specs, Surface specs `39..43`, Surface
plans `64..68`, and the handoff package under
`sub-agent-pro/surface-mir-brace-completion-001/` including sample blueprints.

## Actions taken

- Added `crates/mir-ast::surface_alpha` and exported it from `mir-ast`.
- Added `surface_mir_alpha_parse` example for JSON parse reports.
- Implemented canonical `S { ... }` place blocks and rejected `S[ ... ]` with
  `bracket_place_scope_not_supported`.
- Parsed `role`, `principal`, `place`, `record`, `capability`, `state`,
  `visible`, `when`, `fails`, `join`, assignments, nested place blocks, and
  role-instance blocks.
- Resolved brace heads by declared namespace for place blocks and
  role-instance blocks.
- Limited `Role[instance]` binders to non-empty principal/path syntax.
- Added `SURF-01..09` executable parser sample rows under
  `samples/full-system-v1-surface/syntax/`.
- Added `surface_mir_samples.py`, `surface_mir_authoring_check.py`, and
  `surface_mir_release_check.py`.
- Updated docs/status/plan files to mark `P-SURF-01` as parser evidence and
  `P-SURF-02` as the current promoted package.

## Files changed

Primary implementation:

- `crates/mir-ast/src/surface_alpha.rs`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/examples/surface_mir_alpha_parse.rs`
- `crates/mir-ast/tests/surface_mir_parser.rs`

Sample/helper/test surface:

- `samples/full-system-v1-surface/**`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_authoring_check.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

Documentation/status:

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `specs/00-document-map.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

Core validation and focused tests:

```bash
cargo fmt
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
```

Compatibility anchors:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-01
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

Report-close validation reran the standard docs/fmt/diff checks after this
report was written.

## Evidence / outputs / test results

- `cargo test -p mir-ast --test surface_mir_parser -- --nocapture`: 13 tests
  passed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`: 11 tests passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: accepted,
  `sample_count: 9`, passed `SURF-01..09`, failed `[]`,
  `workflow_ready: false`.
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`:
  accepted, `source_count: 9`, diagnostics `[]`.
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release`:
  `surface_mir_release_check_ready: true`, failed commands `[]`.
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-01-final`:
  `surface_mir_release_check_ready: true`, failed commands `[]`, after this
  report existed.
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-01`:
  accepted, product alpha release candidate ready, failed commands `[]`.
- `python3 scripts/operational_product_samples.py check-all --format json`:
  accepted, failed commands `[]`.
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`:
  accepted, failed `[]`.

The first Product Alpha-1 release-check attempt against
`/tmp/mirrorea-alpha1-release` stopped at the existing-output preflight guard;
it was rerun with `/tmp/mirrorea-alpha1-release-p-surf-01` and accepted.

## What changed in understanding

The parser floor must perform namespace-based brace-head disambiguation even
before semantic elaboration. A spelling-only check incorrectly treats
undeclared names as place/role blocks and incorrectly rejects `S[self]` when
`S` is declared as a role. The sample matrix was widened so the helper lane now
guards those namespace and binder cases.

## Open questions

- Full role/principal binding and authority semantics remain for `P-SURF-05`.
- Indexed-state owner/keyspace/access/stale semantics remain for `P-SURF-02`.
- Surface-to-Core generated Core IR, obligations, and source spans remain for
  `P-SURF-03`.
- Runtime source patch activation and durable compatibility checks remain
  later work.

## Suggested next prompt

Continue autonomously with `P-SURF-02 indexed-state semantics`.

## Plan update status

`plan/00-index.md`, `plan/64-surface-mir-placement-roadmap.md`, and
`plan/68-surface-full-system-v1-roadmap.md` were updated. `plan/64` now keeps
`ELAB-01..05` as future elaboration rows instead of parser evidence.

## Documentation.md update status

`Documentation.md` was updated to include the P-SURF-01 parser floor, current
non-claims, and the P-SURF-02 next package.

## progress.md update status

`progress.md` was updated with the P-SURF-01 parser evidence closeout, runnable
commands, non-claims, timestamp, and current package `P-SURF-02`.

## tasks.md update status

`tasks.md` was rewritten as the current task map with P-SURF-01 closed and
P-SURF-02 first in the self-driven queue.

## samples_progress.md update status

`samples_progress.md` was updated in-place to track
`samples/full-system-v1-surface/syntax/` as `SURF-01..09` parser evidence only,
not runtime workflow readiness.

## Reviewer findings and follow-up

Sub-agent parser review found blocking issues in namespace resolution, role
binder validation, sample coverage, `plan/64` overclaiming, and release-check
self-test coverage. Follow-up: added failing tests for undeclared heads,
invalid/empty binders, declared-place bracket heads, and role named `S`;
implemented namespace resolution and binder checks; widened samples to
`SURF-01..09`; moved `ELAB-01..05` back to future rows; added
`scripts.tests.test_surface_mir_release_check` to the release-check plan.

Sub-agent docs/status review found the same `plan/64` overclaim and stale
future/current wording for Surface helpers. Follow-up: updated `specs/43`,
`progress.md`, hands-on docs, sample docs, and roadmap wording so parser-floor
helpers are current anchors while semantics/runtime remain future packages.

## Skipped validations and reasons

No requested validation was skipped for this package. The default Product
Alpha-1 output path was non-empty on first attempt, so that command was rerun
with a fresh output directory.

## Commit / push status

Pending at report write. The package commit will use
`git commit --no-gpg-sign -m "p-surf-01: add Surface Mir brace parser"` and will
be pushed after final staged validation.

## Sub-agent session close status

Parser/docs reviewer agents completed, findings were addressed, and both
sessions were closed.
