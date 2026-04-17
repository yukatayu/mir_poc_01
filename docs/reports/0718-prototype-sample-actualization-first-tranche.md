# Report 0718 — prototype sample actualization first tranche

- Date: 2026-04-17T01:37:00.082752Z
- Author / agent: Codex
- Scope: first runnable prototype sample tranche plus exact rough-stimulus preservation bucket for current L2 sample work
- Decision levels touched: L2 sample-bucket policy, helper-local operational support, docs mirror refresh

## 1. Objective

Bring a first set of previously discussed motivating examples into repo-managed sample form under the new bucket policy:

- `samples/current-l2/` for the admitted authored current subset
- `samples/prototype/` for corrected runnable prototypes
- `samples/not_implemented/` for exact rough stimuli that current parser / runner cannot support

The task also needed to keep the sample flow debuggable by allowing the corrected prototypes to run through the existing current L2 source-sample pipeline with readable CLI output.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `faq_005.md`
- `docs/reports/0716-sample-stimuli-status-map.md`
- `docs/reports/0717-inspect-current-l2-source-sample-pipeline.md`

## 3. Actions taken

1. Read the current sample-policy / roadmap documents and the two scouting reports to distinguish admitted current-L2 sample surface from prototype-only and not-implemented material.
2. Added helper-local runtime support so `run_current_l2_source_sample` can accept direct paths under `samples/prototype/**` without silently promoting them into the authored current-L2 inventory.
3. Relaxed the fixed-source line collector only for the initial contiguous `#` comment block. This allows every sample file to start with a short Japanese intent comment without widening the current L2 surface more broadly.
4. Updated the operational CLI so `mir-current-l2 run-source-sample <sample-or-path> [--host-plan <path>] --format pretty|json` can auto-discover an adjacent `.host-plan.json` sidecar for explicit sample paths that provide one.
5. Added the first runnable prototype tranche:
   - `p01-dice-publication-handoff`
   - `p02-dice-publication-fallback`
   - `p03-avatar-controller-attach-detach`
6. Added exact preservation files for rough stimuli A-D under `samples/not_implemented/order-handoff/`.
7. Added the normative helper note `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`.
8. Synced repo memory and status snapshots so readers can find the new buckets without mixing them into the authored current-L2 inventory.

## 4. Files changed

- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `samples/prototype/README.md`
- `samples/prototype/current-l2-order-handoff/p01-dice-publication-handoff.txt`
- `samples/prototype/current-l2-order-handoff/p01-dice-publication-handoff.host-plan.json`
- `samples/prototype/current-l2-order-handoff/p02-dice-publication-fallback.txt`
- `samples/prototype/current-l2-order-handoff/p02-dice-publication-fallback.host-plan.json`
- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.txt`
- `samples/prototype/current-l2-dynamic-attach-detach/p03-avatar-controller-attach-detach.host-plan.json`
- `samples/not_implemented/README.md`
- `samples/not_implemented/order-handoff/rough-stimulus-a-release-context.txt`
- `samples/not_implemented/order-handoff/rough-stimulus-b-atomic-owner-block.txt`
- `samples/not_implemented/order-handoff/rough-stimulus-c-fence-seq-cst.txt`
- `samples/not_implemented/order-handoff/rough-stimulus-d-update-global-state.txt`
- `specs/examples/451-current-l2-runnable-prototype-and-not-implemented-sample-buckets.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `faq_005.md`

## 5. Commands run and exact outputs

- `df -h .`
  - `overlay 99G 77G 18G 82% /`
- `free -h`
  - `Mem: 960Mi total / 210Mi available`
- `cargo test -p mir-runtime --test current_l2_source_lowering current_l2_source_lowering_ignores_leading_hash_comment_lines -- --exact`
  - initial state before implementation: failed because leading `#` comment lines were treated as unsupported rows
  - final state: `test current_l2_source_lowering_ignores_leading_hash_comment_lines ... ok`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_explicit_prototype_path -- --exact`
  - initial state before implementation: failed because explicit prototype paths were outside the accepted sample set
  - final state: `test current_l2_source_sample_runner_accepts_explicit_prototype_path ... ok`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_uses_adjacent_host_plan_for_prototype_sample_when_omitted -- --exact`
  - initial state before implementation: failed because `--host-plan` was still mandatory
  - final state: `test operational_cli_uses_adjacent_host_plan_for_prototype_sample_when_omitted ... ok`

The full post-change verification evidence is recorded after the final reruns listed below.

## 6. Evidence / findings

- The current L2 runner can keep its admitted authored inventory (`samples/current-l2/`) while also accepting explicit prototype paths under `samples/prototype/` as a helper-local route.
- An initial contiguous `#` comment block is enough to satisfy the “sample intent in Japanese at top of file” requirement without introducing a broader current-L2 comment syntax production.
- The first corrected runnable prototypes now cover three previously user-visible scenario families:
  - dice roll -> publication -> handoff
  - publication failure -> fallback to last confirmed snapshot
  - attach / detach lifecycle
- Exact rough syntax stimuli remain preserved, but are kept clearly outside the runnable parser / runner path.
- The bucket split now reads as:
  - `samples/current-l2/` = source-backed authored current subset
  - `samples/prototype/` = corrected runnable prototype bucket
  - `samples/not_implemented/` = exact rough stimulus preservation bucket

## 7. Changes in understanding

- The repo is now ready to compare “corrected prototype UX” against admitted current-L2 UX without pretending that prototype files are already promoted into the authored current subset.
- The operational CLI can serve as a readable debug surface for future sample-driven theory work without waiting for installed-binary packaging decisions.
- The user’s old examples do not need to be discarded when unsupported; they can be kept verbatim under `samples/not_implemented/` while corrected runnable analogues live beside them in `samples/prototype/`.

## 8. Open questions

- Whether `samples/prototype/` should eventually gain its own inventory / regression helper rather than relying on explicit-path execution.
- Whether corrected prototype promotion criteria should be documented per feature family, not only as a general bucket rule.
- Which prototype family should be widened next: authority-handoff / room-profile samples, typed-surface samples, or theorem/model-check visible samples.

## 9. Suggested next prompt

Take the next corrected sample family that corresponds to a user-proposed feature cluster, put the runnable version under `samples/prototype/`, keep any unsupported exact syntax under `samples/not_implemented/`, and extend the current-L2 debug path only if the change stays helper-local.
