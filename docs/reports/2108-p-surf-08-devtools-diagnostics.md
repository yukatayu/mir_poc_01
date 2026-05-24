# 2108 - P-SURF-08 devtools and diagnostics

P-SURF-08 devtools and diagnostics.

## Objective

Surface Mir alpha の `S { ... }` canonical syntax line に、static observer-safe devtools diagnostics evidence を追加する。対象 panel は Surface source、generated Core IR、indexed-state map、generated communication、role/admission、patch lifecycle、source spans。

## Scope and assumptions

Scope は docs-first / helper-backed static diagnostics evidence であり、final viewer / telemetry ABI、runtime devtools dispatch、production transport、final source patch ABI は主張しない。`.mir` files を semantic source authority とし、`package.mir.json` は alpha artifact のまま扱う。

## Start state / dirty state

Start state は `P-SURF-07` commit `6a292cc793f81ae61de17943b1c8b3f4405c17de` が push 済みの `main`。作業開始時点で `sub-agent-pro/surface-mir-brace-completion-001/` は untracked の handoff package として存在し、本 package commit 対象外にした。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`
- `sub-agent-pro/surface-mir-brace-completion-001/*.md`
- `sub-agent-pro/surface-mir-brace-completion-001/sample-blueprints/*.md`

## Actions taken

- Added `samples/full-system-v1-surface/devtools/` with `DEV-01` positive and `DEV-02` private-field negative rows.
- Added `devtools_bundle` runner support to `scripts/surface_mir_samples.py`.
- Made devtools output use redacted verification reports instead of raw subreports.
- Backed `indexed_state_map` panel by `surface_indexed_state_check` instead of parser-only fallback.
- Added release-check semantic gates for `DEV-01/02`, required panels, `.mir` source authority, observer-safe redaction, non-final viewer claim, and indexed-state semantic backing.
- Updated docs/status/plan/spec references for P-SURF-08 and next package P-SURF-99.

## Files changed

- `samples/full-system-v1-surface/devtools/**`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/full-system-v1-surface/README.md`
- `scripts/README.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `specs/00-document-map.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-08-final-$$`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt --check`
- `git diff --check`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-08-final-$$`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`

## Evidence / outputs / test results

- Surface tests: 44 tests OK.
- `surface_mir_samples.py check-all`: `sample_count=46`, `failed=[]`, `workflow_ready=false`.
- `surface_mir_authoring_check.py check-all`: `accepted=true`, `source_count=47`, `source_authority=".mir"`.
- `surface_mir_release_check.py check-all`: `surface_mir_release_check_ready=true`, `failed_commands=[]`, `scope=p_surf_08_devtools_diagnostics`, `result_count=15`.
- Docs unit: 18 tests OK.
- Source hierarchy: required 546, present 546, missing 0.
- Docs scaffold: complete, 1259 numbered reports before this report.
- `cargo fmt --check`: passed.
- `git diff --check`: passed.
- Product Alpha release anchor: `product_alpha1_release_candidate_ready=true`, `product_alpha1_ready=true`, `failed_commands=[]`, `result_count=29`.
- Operational product samples anchor: `status=accepted`, `failed_commands=[]`.
- Minimal alpha1 patterns anchor: `status=accepted`, `failed=[]`, `strict_family_count=4`.

## What changed in understanding

P-SURF-08 should not treat “devtools projection exists” as observer-safe by construction. The helper now separates internal verification payloads from redacted devtools-facing summaries, and release-check validates that `DEV-01/02` do not reintroduce raw patch/auth/witness payloads. The indexed-state panel is evidence-backed only when the indexed-state semantic checker ran and found indexed-state metadata.

## Open questions

- Final Surface devtools viewer / telemetry ABI remains later.
- Runtime devtools dispatch remains later.
- Full active/tombstoned indexed-key timeline panel remains later.
- Production identity provider, hardware attestation, WAN admission, and distributed durable migration remain later.

## Suggested next prompt

`P-SURF-99 final surface alpha audit`

## Plan update status

Updated `plan/00-index.md`, `plan/65-indexed-state-roadmap.md`, `plan/66-role-admission-roadmap.md`, `plan/67-source-patch-hotplug-roadmap.md`, and `plan/68-surface-full-system-v1-roadmap.md` for P-SURF-08 closure and P-SURF-99 next package. `plan/64` did not need a content change in this package.

## Documentation.md update status

Updated. It now lists P-SURF-08 static devtools diagnostics as closed, with semantic indexed-state backing and redacted patch lifecycle wording.

## progress.md update status

Updated. Current package is `P-SURF-99 final surface alpha audit`; recent log includes P-SURF-08.

## tasks.md update status

Updated. P-SURF-08 is closed and P-SURF-99 is the next autonomous package.

## samples_progress.md update status

Updated. Surface Mir row and devtools root row now include `DEV-01..02`, 46 rows, 47 `.mir` files, semantic indexed-state backing, and redacted patch summaries.

## Reviewer findings and follow-up

Sub-agent reviewer found four issues: devtools output exposed raw subreports, panel coverage was effectively constant-only, `indexed_state_map` was parser-backed rather than semantic-checker-backed, and release-check gates were too weak. Follow-up changes removed `raw_parse_report` from Surface helper output, added redacted verification reports, computed panel coverage from panel summaries, required indexed-state checker evidence for `DEV-01/02`, and widened release-check semantic gates to enforce the P-SURF-08 claim.

## Skipped validations and reasons

No required validation skipped. Product Alpha compatibility anchors were also run.

## Commit / push status

Pending at report creation. This report is intended to be included in commit `p-surf-08: add surface devtools diagnostics` and pushed to `origin/main`.

## Sub-agent session close status

Reviewer sub-agent completed and returned findings. No edit-capable sub-agent was used for this package. The reviewer session was closed after local follow-up and validation.
