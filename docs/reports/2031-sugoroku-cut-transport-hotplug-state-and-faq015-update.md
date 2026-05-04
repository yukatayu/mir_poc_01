# 2031 — Sugoroku / Cut / Transport / Hot-plug State And FAQ015 Update

## Objective

ユーザからの質問
「`atomic_cut` / consistent cut を含む以前の Sugoroku sample は動くか、分割バイナリは出るか、静的検証はどこまでできるか、network / auth / hot-plug / patch rollout / vector clock / leave handling はどうなっているか」
に対して、current repo evidence を read-only inspection と focused rerun で確認し、`tmp_faq/faq_015.md` に簡潔な累積メモを追記する。

## Scope and assumptions

- scope は説明と FAQ 更新に限る。
- 新機能実装、仕様変更、public API / ABI 凍結、sample 昇格は行わない。
- 回答は `current-scope evidence closeout`、`practical alpha-1 first floor`、`public/product readiness` を混同しない。

## Start state / dirty state

- branch: `docs/layered-repro-guide-001`
- start dirty state:
  - untracked: `docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - untracked: `docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- これら既存 untracked file は今回の変更対象に含めない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `specs/18-practical-alpha1-scope.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/sugoroku_world_samples.py`
- `scripts/network_transport_samples.py`
- `scripts/practical_alpha1_check.py`
- `scripts/practical_alpha1_attach.py`
- `scripts/practical_alpha1_transport.py`
- `scripts/practical_alpha1_save_load.py`
- `scripts/projection_codegen_samples.py`

## Actions taken

- relevant specs / sample README / runner scripts を再読した。
- Sugoroku closeout、Sugoroku reset model-check、practical transport / hot-plug / save-load / checker floor を rerun した。
- helper-local process-boundary transport canary と projection/codegen closeout を確認した。
- `vector clock` 採用有無を repo-wide search で確認した。
- 得られた current boundaries を `tmp_faq/faq_015.md` に追記した。

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2031-sugoroku-cut-transport-hotplug-state-and-faq015-update.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg -n "vector clock|vector_clock|vector-clock|vectorclock|membership_epoch|incarnation|consistent cut|distributed activation ordering|atomic_cut|HotPlugVerdict|membership frontier|stale membership|owner leave|leave reassign" README.md Documentation.md progress.md tasks.md specs samples crates scripts
git status --short
ls samples/clean-near-end/sugoroku-world
sed -n '1,260p' samples/clean-near-end/sugoroku-world/README.md
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
sed -n '1,260p' specs/15-cut-save-load-checkpoint.md
sed -n '1,260p' specs/16-runtime-package-adapter-hotplug.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '360,460p' scripts/sugoroku_world_samples.py
sed -n '1000,1060p' scripts/sugoroku_world_samples.py
sed -n '1280,1325p' scripts/sugoroku_world_samples.py
sed -n '2298,2312p' scripts/sugoroku_world_samples.py
sed -n '1,260p' scripts/network_transport_samples.py
sed -n '240,290p' scripts/network_transport_samples.py
python3 scripts/practical_alpha1_check.py check-all --format json
rg -n "final emitted|emitted place|place-specific program|projection|codegen|binary|installed binary|patch distribution|distributed activation|activation cut" README.md Documentation.md specs plan samples scripts crates
sed -n '1,240p' scripts/projection_codegen_samples.py
sed -n '1,220p' samples/practical-alpha1/README.md
sed -n '1,220p' scripts/practical_alpha1_transport.py
sed -n '1,220p' scripts/practical_alpha1_attach.py
rg -n "vector clock|vector_clock|vector-clock|vectorclock" README.md Documentation.md specs plan samples crates scripts || true
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/network_transport_samples.py check-all --format json
sed -n '1,260p' tmp_faq/faq_015.md
date '+%Y-%m-%d %H:%M:%S %Z'
```

## Evidence / outputs / test results

- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - 10 samples を返した。
  - place model は `single OS process logical multi-place emulator`。
  - limitations に `no real network yet`、`no multi-server consensus`、`no durable distributed commit`、`detach is an explicit TODO stop line` が入る。
- `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification --format json`
  - `property = no_old_epoch_handoff_after_reset`
  - `model_check_result = pass`
  - broken variant は `counterexample`
- `python3 scripts/practical_alpha1_check.py check-all --format json`
  - `CHK-LIF-01..04`、`CHK-VAR-01..03`、`CHK-CUT-01`、`CHK-PKG-01/02` が pass
  - `spec18_typed_checking_complete = false`
- `python3 scripts/practical_alpha1_transport.py check-all --format json`
  - `TR-A1-01..07` pass
  - `docker_row_complete = true`
  - `wan_federation_claimed = false`
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
  - `HP-A1-01..07` pass
  - `object_attach_claimed = false`
  - `detach_minimal_contract_complete = true`
- `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - `SL-A1-01..03` pass
  - `invalid_distributed_cut_guard_present = true`
  - `distributed_save_load_claimed = false`
- `python3 scripts/network_transport_samples.py check-all --format json`
  - `NET-02..05` pass
  - `transport_scope = helper_local_process_boundary`
  - `transport_seam = loopback_socket`
- `python3 scripts/projection_codegen_samples.py closeout --format json`
  - `artifact_boundary = committed manifest bridge evidence only; not a final emitted executable program`
  - `materialization = manifest_bridge_only`
  - kept-later gate に `generated_place_program_emitter`、`placement_optimizer`、`deployment_planner`、`final_public_emitted_program_abi`
- repo-wide search
  - active line に `vector clock` / `vector_clock` / `vector-clock` / `vectorclock` の hit は無かった。

## What changed in understanding

- Sugoroku は current active sample family として runnable だが、distributed runtime completion ではないことを再確認した。
- `atomic_cut` はあくまで place-local frontier であり、consistent cut / distributed save-load と同義ではないことを再確認した。
- current freshness model は vector clock ではなく `membership_epoch` + `member_incarnation` であることを確認した。
- network / hot-plug / auth layering は first floors がかなり揃っているが、distributed activation ordering と safe network-wide patch rollout は未完のままであることを再確認した。
- projection/codegen は binary emitter ではなく manifest bridge evidence 止まりであることを再確認した。

## Open questions

- distributed activation ordering を later family のどの粒度で切り出すか。
- network-wide patch distribution / simultaneous activation を membership frontier と witness lane のどこまでで制御するか。
- current membership frontier model の後段で vector-clock-like causal summary を導入する必要が本当にあるか。
- projection/codegen を emitted place program へ進めるときの placement unit と packaging unit をどう分けるか。

## Suggested next prompt

`Layer 3/5/6/7 の境界として、Sugoroku sample の message envelope / membership frontier / auth lane / hot-plug activation cut を 1 本の流れで詳しく説明してください。`

## Plan update status

- `plan/` 更新不要。
- 今回は current evidence の説明と FAQ 更新のみで、repository memory を更新する新決定は無い。

## Documentation.md update status

- `Documentation.md` 更新不要。
- current boundary の再説明のみで、新しい actualization や stale reference 整理は発生していない。

## progress.md update status

- `progress.md` 更新不要。
- 新しい progress 変化や package close は発生していない。

## tasks.md update status

- `tasks.md` 更新不要。
- task map を変える新規 blocker discovery や package close は発生していない。

## samples_progress.md update status

- `samples_progress.md` 更新不要。
- sample status の actual change は無く、existing rows の rerun / reconfirm に留まる。

## Reviewer findings and follow-up

- reviewer / sub-agent は未使用。
- local evidence と doc inspection で閉じた。

## Skipped validations and reasons

- `python3 scripts/practical_alpha1_run_local.py check-all --format json`
  - 今回の質問の主眼が cut / transport / hot-plug / auth / patch rollout / vector clock だったため未再実行。
  - local-runtime floor 自体は過去 turn で確認済み。
- browser での viewer manual inspection
  - JSON evidence と sample closeout で十分だったため未実行。
- actual WAN / federation validation
  - current scope 外であり、repo 自身も未claim。

## Commit / push status

- pending

## Sub-agent session close status

- sub-agent session なし。
