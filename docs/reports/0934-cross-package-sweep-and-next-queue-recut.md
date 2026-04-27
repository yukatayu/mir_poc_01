# 0934 Cross-Package Sweep And Next Queue Recut

## Objective

transport / avatar / hot-plug / dashboard / closeout landing page の current snapshot を揃え、
active evidence / planned family / mixed gate を切り分けた上で、
post-0933 の next promoted line を `FAIRY-05` residual reacquire design に recut する。

## Scope and assumptions

- current scope は queue / taxonomy / closeout reading の docs-first recut に限る。
- new runtime implementation、new helper command、new sample actualization は含めない。
- `FAIRY-05` 自体はまだ planned family に残し、`Typed external boundary / adapter executable widening` は次 reopen point に据える。
- worktree には unrelated current-L2 Rust diff が残っているため、本 package の stage / commit には含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0932-network-transport-helper-local-canaries.md`
- `docs/reports/0933-avatar-fairy-follow-residual-widening.md`

read-only sub-agent findings:

- `019dced2-31c2-7390-ab7a-4749f0c8080b` `Ramanujan`

## Actions taken

1. post-0933 queue split を明文化した。
   - active evidence:
     `NET-01..05`, `hotplug_lifecycle`, `FAIRY-01/02/03/04/06`
   - planned family:
     `FAIRY-05`, `EXT-01..05`, `samples/not_implemented/network-transport/`
   - mixed-gate later work:
     real transport / session-signature / public ABI, hot-plug final ABI / migration, projection generator / optimizer, public adapter API
2. snapshot docs と reader-facing docs の next queue を recut した。
   - current promoted line:
     `FAIRY-05` residual reacquire design
   - next reopen point:
     `Typed external boundary / adapter executable widening`
3. `samples_progress.md` と `tasks.md` の planned-family wording を揃えた。
   - avatar residual family だけでなく `samples/not_implemented/typed-external-boundary/` と `samples/not_implemented/network-transport/` も planned family として明示
4. `plan/01` の forward-looking prose を retrospective chain + next line reading に直し、already-closed packages を “今からやる queue” と誤読しにくい形へ寄せた。
5. reviewer follow-up reports `0935` / `0936` / `0937` の findings を反映し、`samples_progress.md` / `docs/hands_on/current_phase_closeout_01.md` / `tasks.md` / report taxonomy の残ズレを解消した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0934-cross-package-sweep-and-next-queue-recut.md`
- `docs/reports/0935-review-cross-package-sweep-docs-package.md`
- `docs/reports/0936-recheck-0934-docs-package-after-follow-up-fixes.md`
- `docs/reports/0937-recheck-samples-progress-current-active-packages-wording.md`

## Evidence / outputs / test results

- `python3 scripts/avatar_follow_samples.py closeout --format json`
  - pass (`FAIRY-01/02/03/04/06` active, `FAIRY-05` only planned)
- `python3 scripts/network_transport_samples.py closeout --format json`
  - pass (`NET-02..05` helper-local canary family, helper-local process-boundary scope)
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass (`hotplug_lifecycle`, `message_envelopes`, `visualization_views`, `NET-01` loopback seam, TODO detach boundary all visible)
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json`
  - pass (current clean near-end closeout still reports report-local signature/message/layer/visualization carriers)
- `python3 scripts/check_source_hierarchy.py`
  - pass (`required 23, missing 0`)
- `python3 scripts/validate_docs.py`
  - pass after final report sync (`Found 935 numbered report(s).`)
- `git diff --check`
  - pass

skipped in this package:

- `cargo test -p mir-runtime`
  - skipped; no Rust runtime code changed
- `python3 -m unittest scripts.tests.test_avatar_follow_samples`
  - skipped; queue recut package is docs-only and relies on already-fresh `0933` evidence

## What changed in understanding

- `cross-package sweep` は単なる docs cleanup ではなく、queue meaning の recut が主眼だった。特に `FAIRY-05` を narrow reopen line とし、adapter / transport / hot-plug residuals を broader mixed gate へ戻すと current next line が読みやすくなる。
- `samples_progress.md` の planned rows と `tasks.md` の planned family wording は別方向に drift しやすい。planned sample family を avatar だけに見せると `EXT-01..05` が invisible になるため、docs-first ladders も明示する必要がある。
- `plan/01` のような repository memory では、already-closed package chain を “その後の next line” と切り分けて書かないと、current promoted line を誤読しやすい。

## Open questions

- `FAIRY-05` reacquire-after-return に必要な最小 state-timeline / anchor-switch carrier は何か。
- typed external boundary の dedicated helper cut を `EXT-01..05` のどこから切るか。
- transport / adapter / projection の later mixed gate をどの順で reopen すると一番手戻りが少ないか。

## Suggested next prompt

`FAIRY-05 residual reacquire design` package を進めてください。helper-local runnable actualizationはまだ行わず、state-timeline / anchor-switch / visibility-return witness の最小 carrier、debug surface、planned sample wording、stop line を docs / plan / progress / report へ同期してください。
