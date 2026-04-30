# 1001 full validation rerun and front-door parity

## Objective

Rerun the repo-local full validation floor after the recent maintenance packages, then fix the remaining front-door documentation drift in `samples/README.md`, `scripts/README.md`, and `samples_progress.md` without reopening any implementation line.

This package is maintenance only. It does not change semantics, does not promote a new post-`P21` implementation package, and does not freeze any public API / ABI.

## Scope and assumptions

- `progress.md` / `tasks.md` remain queue authority; `plan/` remains repository memory.
- The current promoted line remains maintenance until `U1` actual commitment.
- Helper-local preview, generated bridge evidence, and runtime-private hot-plug names remain non-public.
- Docs parity changes are limited to sample/script taxonomy, validation-floor wording, and fresh validation timestamps.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `samples/README.md`
- `scripts/README.md`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Reran the full repo-local validation floor: docs checks, current-L2 / clean near-end / representative helper closeouts, storage guardrail, Rust crates, formatting, and diff checks.
- Ran `python3 scripts/current_l2_lean_sample_sync.py` to refresh Lean-side generated manifest parity and confirmed it left the working tree clean.
- Audited front-door sample docs against the actual helper scripts and validation surface.
- Updated `samples/README.md` to:
  - record that `current_l2_guided_samples.py` is a compatibility wrapper over the clean near-end suite,
  - add the viewer helper to the front-door commands,
  - clarify that `EXT-03` / `EXT-04` are helper-local synthetic preview anchors read from a planned-path root without promoting that root to active semantic sample status,
  - clarify that runnable network canary IDs are `NET-02..05`, while `NET-01` remains a reported Sugoroku loopback parity anchor.
- Updated `scripts/README.md` so the front-door runner taxonomy now mirrors the current wrapper/canary split.
- Updated `samples_progress.md` with fresh validation timestamps, the `NET-02..05` wording, the current report reference for dashboard maintenance, and a new recent-validation snapshot.
- Updated `progress.md` and `tasks.md` to record this package close and to keep the validation-floor wording aligned with the commands actually rerun, including Lean sync.

## Files changed

- `samples/README.md`
- `scripts/README.md`
- `samples_progress.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1001-full-validation-rerun-and-front-door-parity.md`

## Evidence / outputs / test results

Commands run:

| Command | Result | Output summary |
|---|---|---|
| `git status --short` | pass | clean working tree before edits |
| `git branch --show-current` | pass | `main` |
| `git log -1 --oneline` | pass | `ce91dc4 Cool plan temperature wording` |
| `df -h .` | pass | `/dev/vda2` 99G total, 32G available |
| `free -h` | pass | `960Mi` RAM total, `444Mi` available at audit time |
| `lsblk -f` | pass | `/dev/vdb1` mounted at `/mnt/mirrorea-work` |
| `findmnt` | pass | `/mnt/mirrorea-work` remained mounted on `/dev/vdb1` |
| `du -sh .` | pass | repo size `109M` |
| `du -sh target .git .cargo .lake 2>/dev/null || true` | pass | `target` symlink shows `0`, `.git` `87M` |
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 998 numbered report(s).` before this report; rerun after edits recorded in snapshot docs |
| `python3 scripts/current_l2_guided_samples.py closeout --format json` | pass | clean near-end compatibility wrapper returned canonical current-L2 closeout JSON |
| `python3 scripts/clean_near_end_samples.py closeout` | pass | canonical clean near-end closeout JSON returned |
| `python3 scripts/sugoroku_world_samples.py closeout --format json` | pass | representative slice / hot-plug stop-line inventory returned |
| `python3 scripts/avatar_follow_samples.py closeout --format json` | pass | active `FAIRY-01/02/03/04/06` slice passed; `FAIRY-05` remained planned |
| `python3 scripts/typed_external_boundary_samples.py closeout --format json` | pass | helper synthetic preview subset `EXT-03/04` and residual matrix returned |
| `python3 scripts/network_transport_samples.py closeout --format json` | pass | runnable helper-local canaries were `NET-02..05`; closeout only reported Sugoroku parity anchors and did not execute them directly |
| `python3 scripts/projection_codegen_samples.py closeout --format json` | pass | committed generated manifest bridge evidence remained aligned |
| `python3 scripts/visual_debugger_viewer_samples.py closeout --format json` | pass | typed viewer prototype inventory remained aligned |
| `python3 scripts/current_l2_lean_sample_sync.py` | pass | refreshed `samples/lean/manifest.json`; working tree stayed clean |
| `cargo test -p mir-ast` | pass with warnings | existing dead-code warnings only |
| `cargo test -p mirrorea-core` | pass | carriers and runtime substrate tests passed |
| `cargo test -p mir-runtime` | pass | clean near-end, source-corpus, operational CLI, and hot-plug runtime tests passed |
| `cargo test -p mir-semantics` | pass with warnings | Lean/tooling support and minimal interpreter tests passed; existing dead-code warnings only |
| `cargo fmt --check` | pass | no output |
| `bash scripts/storage/detach_prepare.sh` | pass with warning | `/mnt/mirrorea-work` mounted; LLVM parent remained `root:root` / non-writable; no files deleted |
| `git diff --check` | pass | no output before edits; rerun after edits also passed |

Fresh docs-floor rerun after edits:

| Command | Result | Output summary |
|---|---|---|
| `python3 scripts/check_source_hierarchy.py` | pass | `required: 23`, `present: 23`, `missing: 0` |
| `python3 scripts/validate_docs.py` | pass | `Documentation scaffold looks complete. Found 999 numbered report(s).` |
| `git diff --check` | pass | no output |

Drift found and corrected:

- `samples/README.md` did not mention the viewer helper in its front-door command block.
- `samples/README.md` did not explain that `current_l2_guided_samples.py` is now a compatibility wrapper over the clean near-end suite.
- `samples/README.md`, `scripts/README.md`, and `samples_progress.md` had `NET-01..05`-style ambiguity even though the runnable helper-local network script exposes `NET-02..05`, with `NET-01` serving only as a reported Sugoroku parity anchor rather than a standalone sample ID.
- `samples/README.md` needed an explicit note that typed-external preview reads planned-path source files without promoting that root to active semantic sample status.
- Storage guardrail remains healthy, but the LLVM staging parent directory is still root-owned and non-writable; cleanup remains guarded until explicit ownership repair.

## What changed in understanding

- The repo-local full floor remains green after the recent maintenance packages; no implementation regression surfaced.
- The remaining drift was not in semantics or runtime behavior but in front-door wording and dashboard freshness.
- `samples/README.md` needed tighter wording to avoid two common misreadings:
  - treating compatibility wrappers as independent current-L2 runners,
  - treating planned-path preview anchors as silently promoted active samples.
- The network transport helper line is better described as runnable `NET-02..05` canaries plus a separate reported `NET-01` parity anchor.

## Open questions

- Should `README.md` / `Documentation.md` also gain an explicit one-line note that `current_l2_guided_samples.py` is a compatibility wrapper, or is keeping that detail in `samples/README.md` sufficient?
- Should `README.md` / `Documentation.md` also surface `current_l2_lean_sample_sync.py` now that the snapshot anchors explicitly list it, or is keeping it in `progress.md` / `tasks.md` / `samples_progress.md` sufficient?
- Does `plan/01-status-at-a-glance.md` still need the stronger point-in-time disclaimer previously identified as a possible next maintenance package?

## Suggested next prompt

Continue with the next safe maintenance package: audit `plan/01-status-at-a-glance.md` for point-in-time wording that could still be misread as current queue authority, then rerun the docs-focused floor and close it with a new report.

## plan/progress/tasks/samples updates

- `plan/`: 更新不要
- `progress.md`: updated
- `tasks.md`: updated
- `samples_progress.md`: updated

## Skipped validations and reasons

- No second full sample/Cargo rerun is planned after the docs-only edits because this package changes only snapshot/front-door documentation. Instead, the docs-focused floor will be rerun after edits.
- No cleanup command was executed because storage work in this package was audit-only and cleanup requires explicit confirmation by policy.

## Commit / push status

- Pending at report authoring time. This package will be committed and pushed immediately after the post-edit docs-floor rerun and diff review.

## Sub-agent session close status

- `docs_researcher` `019ddcf4-fd64-7363-826e-eb7abebc8964` returned a repository-memory summary and will be closed after package close.
- `docs_researcher` `019ddcf5-0157-7a52-9ba9-f45eac71c0af` returned a long-horizon roadmap / gate summary and will be closed after package close.
- `code_mapper` `019ddcf5-006b-71d1-8d0d-be313cb3996b` returned validation-surface and taxonomy drift notes and will be closed after package close.
