# Report 2029 — typed pipeline capability check and FAQ015 update

- Date: 2026-05-04 22:25 JST
- Author / agent: Codex
- Scope: answer whether the current repo can already demonstrate a named input/output-style pipeline with viewer observation, update `tmp_faq/faq_015.md`, and commit/push the FAQ/report on the current branch
- Decision levels touched: none normative; explanation + helper memo only

## Objective

Check the current repo-local floor for a “named input/output pipeline” style scenario, distinguish active semantic execution from helper-local synthetic preview, record the result in `tmp_faq/faq_015.md`, and publish the update on the current branch.

## Scope and assumptions

- Treat the question as a current-capability check, not a feature request.
- Preserve the existing rule that `stdio` is intentionally not a Mir core primitive.
- Keep `faq_015` concise and append-oriented.
- Stay on `docs/layered-repro-guide-001`; do not merge into `main`.

## Start state / dirty state

- Current branch: `docs/layered-repro-guide-001`
- Pre-existing untracked files at task start:
  - `?? docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - `?? docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- These files were left untouched and were not staged in this task.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `tmp_faq/faq_015.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir`
- `samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir`
- `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir`
- `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
- `samples/clean-near-end/typing/05_cost_bound_rejected.mir`
- `docs/hands_on/visual_debugger_viewer_01.md`
- `scripts/typed_external_boundary_samples.py`
- `scripts/visual_debugger_viewer_samples.py`
- `scripts/sugoroku_world_samples.py`

## Actions taken

1. Re-read the top-level docs and layer docs to keep the explanation aligned with current wording.
2. Located the typed external boundary family and confirmed which rows are active, synthetic-preview only, or still planned.
3. Checked the viewer hands-on and Sugoroku runtime sample family to see whether a typed route can be observed in a viewer-like bundle.
4. Ran the typed external preview, the viewer bundle over that preview, and the Sugoroku visualization/runtime viewer bundle.
5. Compared those results against the stronger user expectation of a generic host input -> transform -> host output demo.
6. Appended the conclusion and concrete examples into `tmp_faq/faq_015.md`.
7. Added this report.

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2029-typed-pipeline-capability-check-and-faq015-update.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
rg -n "typed external|external effect|adapter|stdin|stdout|pipeline|string|int|integer|hello|viewer|devtools|observe|publish|input|output|port|socket" README.md Documentation.md specs plan samples crates scripts -g '!target'
rg --files samples scripts crates | rg 'typed|external|viewer|devtools|sugoroku|current_l2|mir_hilight|visual'
sed -n '1,220p' tmp_faq/faq_015.md
sed -n '1,260p' scripts/typed_external_boundary_samples.py
sed -n '1,260p' samples/not_implemented/typed-external-boundary/README.md
sed -n '1,260p' scripts/visual_debugger_viewer_samples.py
sed -n '1,260p' docs/hands_on/visual_debugger_viewer_01.md
sed -n '1,260p' samples/clean-near-end/sugoroku-world/README.md
sed -n '1,260p' scripts/sugoroku_world_samples.py
rg -n "String|str|text|message|room_notice|local_queue|concat|\\+ 1|plus_one|integer|Int|Nat|number|name|Taro|Hello|greet|log" samples/clean-near-end samples/current-l2 samples/practical-alpha1 crates/mir-runtime/src/clean_near_end.rs -g '!target'
sed -n '1,220p' samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir
sed -n '1,220p' samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir
sed -n '1,220p' samples/clean-near-end/typing/05_cost_bound_rejected.mir
sed -n '1,220p' samples/clean-near-end/order-handoff/05_delegated_rng_service.mir
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-05 --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
sed -n '1,220p' samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir
sed -n '1,220p' samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir
python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json
python3 - <<'PY'
from pathlib import Path
import re
nums=[]
for p in Path('docs/reports').glob('*.md'):
    m=re.match(r'(\\d+)-', p.name)
    if m:
        nums.append(int(m.group(1)))
print(max(nums) if nums else 0)
PY
date '+%Y-%m-%d %H:%M:%S %Z'
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
git add tmp_faq/faq_015.md docs/reports/2029-typed-pipeline-capability-check-and-faq015-update.md
git commit --no-gpg-sign -m "docs: update faq 015 with pipeline capability check"
git push
git add docs/reports/2029-typed-pipeline-capability-check-and-faq015-update.md
git commit --no-gpg-sign -m "docs: record faq 015 pipeline push status"
git push
```

## Evidence / outputs / test results

- `EXT-03` current floor:
  `python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json`
  returned a successful helper-local synthetic preview with:
  - adapter entry `RoomMessageAdapter#local_queue`
  - request lane `typed_effect_request`
  - receipt lane `typed_effect_receipt`
  - transport seam `local_queue`
  - source refs `room_message_request#1` and `room_message_receipt#1`
  - explicit claim that route/auth/witness lanes remain separate
- `P16-VIEW-05` current floor:
  `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-05 --format json`
  returned a viewer bundle over that typed-external preview with panel `room_message_route` and telemetry rows for both request/receipt envelopes.
- `03_roll_publish_handoff` current floor:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json`
  returned an active semantic runtime result with `draw = 4`, successful publish/handoff, message envelopes, telemetry rows, and visualization views.
- `P16-VIEW-01` current floor:
  `python3 scripts/visual_debugger_viewer_samples.py run P16-VIEW-01 --format json`
  returned typed viewer panels `turn_timeline`, `message_route`, `verification_summary`, and `projection_view` backed by the Sugoroku runtime sample.
- `samples/not_implemented/typed-external-boundary/README.md`
  explicitly states that only `EXT-03` / `EXT-04` are runnable helper-local synthetic preview rows, and that the `.mir` source stubs are not directly semantically executed by the current parser/runtime.
- `samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir`
  exists only as residual planned `EXT-01`, so a console/text demo is not active today.
- `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir`
  contains `game_epoch <- game_epoch + 1`, showing that arithmetic syntax exists in current evidence.

## What changed in understanding

- The repo already has a meaningful partial floor for a named typed route:
  adapter entry, request/receipt split, local queue seam, and viewer observation.
- That floor is not yet the same thing as a generic host input/output demo.
  The strongest “pipe-like” example in the current repo is still helper-local synthetic preview (`EXT-03`) rather than direct semantic execution of a host-boundary sample.
- Active semantic execution is stronger on the Sugoroku side:
  numeric output, publish, handoff, telemetry, and viewer observation all exist there.
  But that line does not yet expose arbitrary user-provided string/integer input on a host boundary.

## Open questions

- Whether the first honest reopen after `EXT-03` should be a direct semantic execution row for typed external boundary, or a more explicit provider-boundary runtime bridge.
- Whether the first generic host-boundary demo should be text-oriented (`EXT-01`-like) or integer-oriented, given the existing arithmetic/runtime anchors.

## Suggested next prompt

Explain what would be minimally missing to turn the current `EXT-03` / `P16-VIEW-05` floor into a true generic host input -> transform -> output demo without violating the no-privileged-stdio rule.

## Plan update status

`plan/` 更新不要:
this task only clarifies current capability and updates the helper FAQ.

## Documentation.md update status

`Documentation.md` 更新不要:
the current snapshot already describes the typed external boundary and viewer floors at the right granularity.

## progress.md update status

`progress.md` 更新不要:
no readiness or queue status changed in this explanation task.

## tasks.md update status

`tasks.md` 更新不要:
no current execution package or blocker map changed here.

## samples_progress.md update status

`samples_progress.md` 更新不要:
no sample status changed; this task only re-read and exercised existing surfaces.

## Reviewer findings and follow-up

- Local self-review only.
- No sub-agent review was opened; this is a small explanation/update task.
- Review focus:
  - separate active semantic execution from helper-local synthetic preview
  - avoid overstating EXT-03 as a real host-I/O runtime
  - append only concise, evidence-backed notes to `faq_015`

## Skipped validations and reasons

- `python3 scripts/check_source_hierarchy.py`, `python3 scripts/validate_docs.py`, and `git diff --check` were deferred until after file edits because this task first needed to confirm the capability question and then update `faq_015`.
- No broader Cargo/Rust test suite was rerun because the task changes only helper docs and a report.

## Commit / push status

- Closeout commit `40a4475` (`docs: update faq 015 with pipeline capability check`) was created with `--no-gpg-sign`.
- The current branch `docs/layered-repro-guide-001` was pushed to `origin/docs/layered-repro-guide-001`.
- This report section was refreshed after the first push so the recorded status matches the actual branch state.

## Sub-agent session close status

No sub-agent sessions were opened in this task.
