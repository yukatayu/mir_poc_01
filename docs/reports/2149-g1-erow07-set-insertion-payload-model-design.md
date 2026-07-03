# Report 2149 - G1 ELAB-07 set-insertion payload-model design

- Date: 2026-07-04 03:56 JST
- Author / agent: Codex
- Scope: LAB `ELAB-07` set-insertion payload-model design
- Decision levels touched: L3 LAB repository memory only

## Objective

Record a docs-only LAB payload-model design for a possible future `ELAB-07`
set-insertion repair item.

Close target for this package:

- keep current executable output unchanged;
- keep `ELAB-07` and `ELAB-04` no-repair;
- preserve singleton repair evidence for `ELAB-10` and `ELAB-13..16`;
- design a separate future set payload shape instead of widening the current
  singleton `missing_failure` payload;
- add future positive / negative test expectations before implementation;
- avoid any claim of executable set-insertion support, bundle semantics,
  OBL-025 completion, conformance, final ABI, canon edit, or G1 exit.

## Scope and assumptions

Scope included:

- current `ELAB-07` fixture facts and expected no-repair output;
- `ELAB-04` as the nearest mixed visibility / base-failure exclusion fence;
- singleton repair output for `ELAB-10` and `ELAB-13..16`;
- LAB set-insertion vocabulary from `plan/96`;
- gate / preflight / assumption memory from `plan/97`, `plan/99`, and
  `plan/100`;
- OBL-025 LAB statement vocabulary as an abstract boundary only;
- snapshot docs that mention G1 E-ROW repair status.

Assumptions:

- `mirrorea_canon/` remains normative.
- The current package is docs / repository-memory only.
- Candidate payload roles are not final Rust field names, JSON keys, or public
  ABI.
- A later executable package must add a separate set payload model and tests;
  it must not loosen the singleton path by accepting
  `missing_failures.len() > 1`.
- The design applies only to the current base-only `ELAB-07` fact pattern.

## Start state / dirty state

Start state for this package:

- Branch: `main`
- Upstream: `origin/main`
- Starting HEAD: `2d92978a7ecd462ab1c9eba51e019b73317f3529`
- Start dirty state: clean at package start.
- Discord baseline: `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`

Current draft dirty state:

- Docs and plan files modified.
- New plan file added:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md`.
- This report is the new task report.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/oracle-chatgpt-pro-operations.md`
- `/home/codex/.codex/docs/oracle-chatgpt-pro.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/03-elaboration.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `mirrorea_canon/theory/10-diagnostics.md`
- `mirrorea_canon/spec/07-diagnostics-format.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `plan/00-index.md`
- `plan/83-g1-erow-repair-payload-inventory.md`
- `plan/87-g1-obl025-lean-statement-draft.md`
- `plan/88-g1-erow-repair-shape-inventory.md`
- `plan/90-source-traceability.md`
- `plan/93-g1-erow001-singleton-repair-assumption.md`
- `plan/94-g1-erow001-singleton-repair-prototype.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/97-g1-erow07-set-insertion-gate-review.md`
- `plan/98-g1-erow04-mixed-visibility-branch-inventory.md`
- `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/surface_mir_samples.py`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/README.md`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/main/src/write-failure-row-negative.mir`
- `samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-10-visibility-denied-failure-row-negative/expected/elaboration.json`
- `samples/full-system-v1-surface/elaboration/elab-13-erow001-missing-witness-singleton-negative/expected/elaboration.json`
- `docs/research_abstract/surface_mir_alpha_01.md`

External / delegated review:

- Sub-agent `019f2949-bc0a-7e41-b8bb-27a7e92ba2a8` completed a read-only
  design inventory and was closed.
- Oracle consult `we-are-in-a-specificat` completed and advised treating the
  package as docs-only, with candidate roles rather than final ABI names, one
  top-level set item, no child repairs, no bundle semantics, no partial
  guidance, and no executable output widening.

## Actions taken

- Added `plan/101-g1-erow07-set-insertion-payload-model-design.md`.
- Updated `plan/00-index.md` and `plan/90-source-traceability.md`.
- Updated `plan/96`, `plan/97`, `plan/99`, and `plan/100` to reference the new
  payload-model design without rewriting earlier gate history.
- Updated `README.md`, `Documentation.md`, and
  `docs/research_abstract/surface_mir_alpha_01.md`.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md`.
- Left Rust, expected JSON, sample matrices, Lean files, and canon files
  unchanged.

## Files changed

- `README.md`
- `Documentation.md`
- `docs/reports/2149-g1-erow07-set-insertion-payload-model-design.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/97-g1-erow07-set-insertion-gate-review.md`
- `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`
- `progress.md`
- `samples_progress.md`
- `tasks.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M %Z'
git status --short --branch
git rev-parse HEAD
git rev-parse --abbrev-ref HEAD
sed -n '1,240p' .agents/skills/discord-report/SKILL.md
sed -n '1,240p' /home/codex/.codex/superpowers/skills/subagent-driven-development/SKILL.md
sed -n '1,240p' /home/codex/.codex/superpowers/skills/requesting-code-review/SKILL.md
sed -n '1,240p' /home/codex/.codex/superpowers/skills/verification-before-completion/SKILL.md
sed -n '1,240p' samples_progress.md
sed -n '1,280p' plan/101-g1-erow07-set-insertion-payload-model-design.md
sed -n '280,520p' plan/101-g1-erow07-set-insertion-payload-model-design.md
tail -n 80 samples_progress.md
git diff -- samples_progress.md plan/99-g1-erow07-set-insertion-executable-preflight.md plan/100-g1-erow07-set-insertion-assumption-acceptance.md
sed -n '1,180p' README.md
sed -n '1,200p' Documentation.md
sed -n '1,220p' progress.md
sed -n '1,220p' tasks.md
ls docs/reports | tail -n 12
sed -n '1,260p' docs/reports/2148-g1-erow07-set-insertion-assumption-acceptance.md
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/validate_docs.py
python3 scripts/check_source_hierarchy.py
git diff --check
cargo fmt --check
python3 scripts/surface_mir_samples.py --format json run ELAB-07 > /tmp/elab07-2149.json
python3 scripts/surface_mir_samples.py --format json run ELAB-04 > /tmp/elab04-2149.json
python3 scripts/surface_mir_samples.py --format json run ELAB-10 > /tmp/elab10-2149.json
python3 scripts/surface_mir_samples.py --format json run ELAB-13 > /tmp/elab13-2149.json
python3 scripts/surface_mir_samples.py --format json check-all > /tmp/mirrorea-surface-check-all-2149.json
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples
jq 'keys' /tmp/mirrorea-surface-check-all-2149.json
jq 'keys' /tmp/elab07-2149.json
jq 'to_entries | map({key, type:(.value|type)})' /tmp/mirrorea-surface-check-all-2149.json
jq 'paths(objects | select(has("suggested_repair")))' /tmp/elab10-2149.json
jq -n --slurpfile elab07 /tmp/elab07-2149.json --slurpfile elab04 /tmp/elab04-2149.json --slurpfile elab10 /tmp/elab10-2149.json --slurpfile elab13 /tmp/elab13-2149.json --slurpfile all /tmp/mirrorea-surface-check-all-2149.json '{elab07:{accepted:$elab07[0].accepted,mismatches:$elab07[0].mismatches,has_suggested_repair:($elab07[0].actual.lab_diagnostic_details[0]|has("suggested_repair"))},elab04:{accepted:$elab04[0].accepted,mismatches:$elab04[0].mismatches,has_suggested_repair:($elab04[0].actual.lab_diagnostic_details[0]|has("suggested_repair"))},elab10:{accepted:$elab10[0].accepted,mismatches:$elab10[0].mismatches,repair_count:(($elab10[0].actual.lab_diagnostic_details[0].suggested_repair // [])|length)},elab13:{accepted:$elab13[0].accepted,mismatches:$elab13[0].mismatches,repair_count:(($elab13[0].actual.lab_diagnostic_details[0].suggested_repair // [])|length)},check_all:{sample_count:$all[0].sample_count,failed_count:($all[0].failed|length),workflow_ready:$all[0].workflow_ready,elab07_repair_count:(($all[0].results[] | select(.sample_id=="ELAB-07") | .actual.lab_diagnostic_details[0].suggested_repair // [])|length),elab04_repair_count:(($all[0].results[] | select(.sample_id=="ELAB-04") | .actual.lab_diagnostic_details[0].suggested_repair // [])|length),elab10_repair_count:(($all[0].results[] | select(.sample_id=="ELAB-10") | .actual.lab_diagnostic_details[0].suggested_repair // [])|length),elab13_repair_count:(($all[0].results[] | select(.sample_id=="ELAB-13") | .actual.lab_diagnostic_details[0].suggested_repair // [])|length)}}'
rg -n '^## ' docs/reports/2149-g1-erow07-set-insertion-payload-model-design.md
rg -n -f /tmp/mirrorea-stale-status-patterns-2149 docs/reports/2149-g1-erow07-set-insertion-payload-model-design.md README.md Documentation.md progress.md tasks.md samples_progress.md plan/101-g1-erow07-set-insertion-payload-model-design.md plan/96-g1-erow-set-insertion-bundle-payload-inventory.md plan/97-g1-erow07-set-insertion-gate-review.md plan/99-g1-erow07-set-insertion-executable-preflight.md plan/100-g1-erow07-set-insertion-assumption-acceptance.md
git add README.md Documentation.md docs/research_abstract/surface_mir_alpha_01.md progress.md tasks.md samples_progress.md plan/00-index.md plan/90-source-traceability.md plan/96-g1-erow-set-insertion-bundle-payload-inventory.md plan/97-g1-erow07-set-insertion-gate-review.md plan/99-g1-erow07-set-insertion-executable-preflight.md plan/100-g1-erow07-set-insertion-assumption-acceptance.md plan/101-g1-erow07-set-insertion-payload-model-design.md docs/reports/2149-g1-erow07-set-insertion-payload-model-design.md
git status --short --branch
git diff --cached --stat
git diff --cached --check
git commit --no-gpg-sign -m "Record ELAB-07 set payload model design"
git push
git status --short --branch
git rev-parse HEAD origin/main
git log -1 --oneline
```

Tool / external actions:

- Oracle consult executed through `ask-chatgpt-pro`; completed session:
  `we-are-in-a-specificat`.
- Sub-agent design inventory:
  `019f2949-bc0a-7e41-b8bb-27a7e92ba2a8`.
- Sub-agent final reviewer:
  `019f295a-a287-7642-9ca7-431c792404fc`.
- Repo-local secret-pattern scans were run. The exact sensitive expression is
  intentionally not mirrored into the report.

One `jq` summary attempt failed with `Cannot iterate over null` because it
assumed a `.samples[]` field. A second inspection showed `check-all` stores
rows under `.results[]`, and repair payloads are under
`actual.lab_diagnostic_details[0].suggested_repair`; the corrected summary is
the one recorded below.

## Evidence / outputs / test results

Design evidence recorded in `plan/101`:

- current repair-bearing rows remain singleton-only:
  `ELAB-10` and `ELAB-13..16`;
- current no-repair fences remain:
  `ELAB-07` and `ELAB-04`;
- future `ELAB-07` set item is one top-level candidate `set_insertion` item,
  not three serialized singleton child repairs;
- future set item avoids singular `missing_failure` field reuse for
  multi-failure coverage;
- designed set arithmetic is exact:
  `insert_failures = required - declared_before`, and
  `declared_failures_after == required_failures`;
- future tests must include whole-gap positive checks, proper-subset negative
  checks, duplicate / padded insertion negatives, `VisibilityDenied` exclusion,
  `ELAB-04` no-repair preservation, singleton repair regression checks, and
  no empty-list output standardization.

Validation results:

- Documentation validator unit tests: 20 tests passed.
- Documentation scaffold validator: complete, 1301 numbered reports found.
- Source hierarchy check: 602 required paths, 602 present, 0 missing.
- `git diff --check`: clean.
- `cargo fmt --check`: clean.
- Stale lifecycle wording scan: no matches after report refresh.
- Changed-file repo-local secret-pattern scan: no matches.
- Changed-file notification-target-word scan: no matches.
- Staged diff check: clean.
- Staged repo-local secret-pattern scan: no matches.
- Staged notification-target-word scan: no matches.
- `ELAB-07` individual run: `accepted = true`, `mismatches = []`,
  `has_suggested_repair = false`.
- `ELAB-04` individual run: `accepted = true`, `mismatches = []`,
  `has_suggested_repair = false`.
- `ELAB-10` individual run: `accepted = true`, `mismatches = []`,
  `repair_count = 1`.
- `ELAB-13` individual run: `accepted = true`, `mismatches = []`,
  `repair_count = 1`.
- Surface helper `check-all`: `sample_count = 52`, `failed_count = 0`,
  `workflow_ready = false`, `ELAB-07 repair_count = 0`,
  `ELAB-04 repair_count = 0`, `ELAB-10 repair_count = 1`,
  `ELAB-13 repair_count = 1`.
- Rust elaboration tests: 20 passed, 0 failed. The visible panic line is the
  expected `should_panic` placeholder-repair detector test.
- Python Surface helper tests: 45 tests passed.

## What changed in understanding

`plan/100` resolved the narrow edit-cardinality assumption for `ELAB-07`.
This package identifies the next missing boundary: payload shape. The safe next
step is not to relax the singleton repair emitter, but to introduce a separate
set payload with explicit whole-gap coverage and exclusion gates.

The smallest defensible future executable item is one local set insertion into
one existing `when_fails_row`. Bundle, partial guidance, visibility split,
branch ordering, and ranking remain separate later problems.

## Open questions

- What concrete Rust/JSON field names should represent the candidate roles if
  the executable prototype is promoted?
- Should a later implementation expose both `declared_failures_before` and
  existing `declared_failures`, or collapse those names internally while keeping
  report semantics readable?
- Should future negative fixtures be separate samples or Rust/Python unit-only
  guard cases before sample promotion?
- How should OBL-025 vocabulary be refined after executable set payload output
  exists?

## Suggested next prompt

Promote the exact `ELAB-07` set-insertion executable payload prototype using
`plan/101` as the design gate. Keep `ELAB-04` no-repair, preserve singleton
repair outputs, and do not claim final ABI, OBL-025 completion, conformance, or
G1 exit.

## Plan update status

Updated:

- `plan/00-index.md`
- `plan/90-source-traceability.md`
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md`
- `plan/97-g1-erow07-set-insertion-gate-review.md`
- `plan/99-g1-erow07-set-insertion-executable-preflight.md`
- `plan/100-g1-erow07-set-insertion-assumption-acceptance.md`
- `plan/101-g1-erow07-set-insertion-payload-model-design.md`

## Documentation.md update status

Updated. It now records `plan/101` as docs-only payload-model design and keeps
`ELAB-07` executable output no-repair.

## progress.md update status

Updated. It now records the current payload-model design package, without
promoting executable set-insertion support or G1 exit.

## tasks.md update status

Updated. It now promotes the next candidate package to exact `ELAB-07`
set-insertion executable payload prototype, with explicit constraints.

## samples_progress.md update status

Updated. It records `plan/101` as docs-only design evidence, not workflow-ready
sample completion or executable support.

## Reviewer findings and follow-up

Sub-agent reviewer `019f295a-a287-7642-9ca7-431c792404fc` found no semantic
overclaim in `plan/101`, `progress.md`, `tasks.md`, or
`samples_progress.md`. The reviewer found report-closeout issues only:

- stale report lifecycle text for validation / reviewer / sub-agent status;
- non-command placeholders in the command log.

Follow-up:

- refreshed this report section and sub-agent close status;
- changed the command log to separate exact shell commands from tool / external
  actions;
- preserved the docs-only no-support / no-proof / no-conformance wording.

## Skipped validations and reasons

No validation was intentionally skipped for this docs-only package.

## Commit / push status

Content commit:
`473cebf6d079c14203e67d4f865d1ea42b866a08`
(`Record ELAB-07 set payload model design`).

Push status:

- `git push`: succeeded.
- `HEAD`: `473cebf6d079c14203e67d4f865d1ea42b866a08`
- `origin/main`: `473cebf6d079c14203e67d4f865d1ea42b866a08`
- Clean/equal check after push: `git status --short --branch` reported
  `## main...origin/main`.

This report update is intended to be committed and pushed as a follow-up
bookkeeping commit.

## Sub-agent session close status

- Read-only design inventory sub-agent
  `019f2949-bc0a-7e41-b8bb-27a7e92ba2a8`: completed and closed.
- Package-local final reviewer
  `019f295a-a287-7642-9ca7-431c792404fc`: completed and closed.
