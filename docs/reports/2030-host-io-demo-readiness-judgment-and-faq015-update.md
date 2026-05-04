# Report 2030 — host-I/O demo readiness judgment and FAQ015 update

- Date: 2026-05-04 22:47 JST
- Author / agent: Codex
- Scope: answer whether a generic host input/output demo is already authorable from a new Mir sample alone, update `tmp_faq/faq_015.md`, and commit/push the FAQ/report on the current branch
- Decision levels touched: none normative; explanation + helper memo only

## Objective

Make the readiness judgment explicit:
whether the current repo is already at the point where one can simply write a new Mir sample, compile/run it, and obtain a generic host input/output demo, or whether substantial implementation is still required.

## Scope and assumptions

- Treat the user question as a readiness judgment over the current repo state, not as a request to implement the missing pipeline.
- Preserve the established no-privileged-stdio boundary.
- Keep `tmp_faq/faq_015.md` concise and append-oriented.
- Commit/push only the FAQ/report updates for this turn.

## Start state / dirty state

- Current branch: `docs/layered-repro-guide-001`
- Pre-existing unrelated untracked files remained present:
  - `?? docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - `?? docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- These files were not staged in this task.

## Documents consulted

- `tmp_faq/faq_015.md`
- `README.md`
- `Documentation.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir`
- `samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir`
- `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir`
- `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir`
- `docs/reports/2029-typed-pipeline-capability-check-and-faq015-update.md`

## Actions taken

1. Re-read the previously accumulated `faq_015` notes and the immediately preceding capability-check report.
2. Reconfirmed the strongest available evidence:
   partial typed route/viewer floor exists,
   but generic host input/output semantic execution does not.
3. Wrote the readiness judgment explicitly into `tmp_faq/faq_015.md`:
   not “write one sample and go”, but “still needs implementation”.
4. Enumerated the minimum missing pieces in the FAQ:
   direct semantic execution lane, typed host payload schema, runtime bridge, exact output/viewer route, and reusable demo sample family.
5. Added this report.

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2030-host-io-demo-readiness-judgment-and-faq015-update.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
sed -n '1,260p' tmp_faq/faq_015.md
rg -n "synthetic preview|not directly semantic execution|not an active runnable sample|final public adapter API|typed external|host boundary|provider_boundary|local_queue" samples/not_implemented/typed-external-boundary/README.md scripts/typed_external_boundary_samples.py docs/hands_on/visual_debugger_viewer_01.md README.md Documentation.md
rg -n "no final public adapter API|no browser/network/VR host schema|no real transport widening|not directly semantic execution|helper-local synthetic preview|not an active runnable sample" samples/not_implemented/typed-external-boundary/README.md samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir samples/not_implemented/typed-external-boundary/03_send_room_message_local_queue.mir
rg -n "active semantic execution|terminal_outcome|draw =|output draw|game_epoch <- game_epoch \\+ 1|current parser/runtime" samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir docs/reports/2029-typed-pipeline-capability-check-and-faq015-update.md
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
git add tmp_faq/faq_015.md docs/reports/2030-host-io-demo-readiness-judgment-and-faq015-update.md
git commit --no-gpg-sign -m "docs: record host io demo readiness in faq 015"
git push
git add docs/reports/2030-host-io-demo-readiness-judgment-and-faq015-update.md
git commit --no-gpg-sign -m "docs: record faq 015 host io push status"
git push
git rev-parse HEAD
git status --short
python3 .agents/skills/discord-report/scripts/discord_notify.py complete --cwd . --result success --summary 'Recorded host-I/O demo readiness judgment in FAQ015 and pushed branch commits' --include-diff
```

## Evidence / outputs / test results

- `samples/not_implemented/typed-external-boundary/README.md`
  still says the runnable subset is only `EXT-03` / `EXT-04` via a synthetic preview helper, and that the `.mir` stubs are not directly semantically executed by the current parser/runtime.
- `samples/not_implemented/typed-external-boundary/01_log_text_local_console.mir`
  is still a residual planned stub, not an active runnable sample.
- `docs/reports/2029-typed-pipeline-capability-check-and-faq015-update.md`
  already established that the strongest current floor is:
  typed route + request/receipt + viewer observation as partial capability,
  but not generic host-I/O execution.
- `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir`
  and the previous checked runtime output still support that active semantic execution exists for numeric publish/viewer flow.
- `samples/clean-near-end/sugoroku-world/08_reset_interleaving_model_check.mir`
  still shows arithmetic syntax (`game_epoch <- game_epoch + 1`) exists.
- `python3 scripts/check_source_hierarchy.py`
  passed.
- `python3 scripts/validate_docs.py`
  passed.
- `git diff --check`
  passed.

## What changed in understanding

- The honest line is stronger than “toolchain idea exists” but weaker than “ready-to-use host demo”.
- The repo is not blocked on lack of any route/viewer concept; it is blocked on the missing direct semantic host-boundary execution lane and surrounding adapter/runtime contract.
- Therefore the correct readiness judgment is “still needs implementation”, not “already possible if you just write the sample”.

## Open questions

- Which minimal implementation path should come first if this readiness gap is ever addressed:
  provider-boundary console text,
  integer input/output adapter,
  or direct activation of the existing `EXT-03`-like room-message path?
- Whether the first reopen should land in current-L2, clean-near-end, or practical alpha-1 is still open.

## Suggested next prompt

Explain the smallest implementation package that would be required to turn the current partial typed route/viewer floor into a true generic host input/output demo, without broadening scope beyond one narrow line.

## Plan update status

`plan/` 更新不要:
this task only records an explanation judgment and FAQ entry.

## Documentation.md update status

`Documentation.md` 更新不要:
no repo snapshot drift was introduced.

## progress.md update status

`progress.md` 更新不要:
no queue or readiness state changed.

## tasks.md update status

`tasks.md` 更新不要:
no task-map change was made in this explanation task.

## samples_progress.md update status

`samples_progress.md` 更新不要:
no sample-family implementation status changed.

## Reviewer findings and follow-up

- Local self-review only.
- Review focus:
  - keep the readiness judgment explicit
  - avoid overstating “partial floor” into “sample-author-ready”
  - append concise notes into `faq_015`

## Skipped validations and reasons

- No broader runtime/Cargo test suite was rerun because this task only updates helper docs and a report.
- No new sample command execution was needed beyond the already established evidence from the immediately preceding task.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent sessions were opened in this task.
