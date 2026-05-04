# 2033 — Priority / Devtools / Formal Boundary Review And FAQ015 Update

## Objective

ユーザからの
「ここまで挙げた基礎機能は高優先度か、debug/visualization は必須か、形式検証の理論は十分固まっているか、docs は実装方針まで整理済みかを、全体的に文書を見返したうえで今の立ち位置と今後の方針としてまとめてほしい」
という依頼に対し、relevant docs を再読し、current position / safe next policy を整理し、`tmp_faq/faq_015.md` に累積メモを追記する。

## Scope and assumptions

- scope は explanation / prioritization / doc review / FAQ 更新に限る。
- 新規実装、仕様変更、package promotion、public API / ABI 凍結は行わない。
- current-scope evidence closeout と practical alpha-1 readiness を混同しない。

## Start state / dirty state

- branch: `docs/layered-repro-guide-001`
- start dirty state:
  - untracked: `docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - untracked: `docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- これらは今回の変更対象に含めない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/03-layer-model.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples_progress.md`
- `samples/practical-alpha1/README.md`
- `scripts/practical_alpha1_export_devtools.py`
- `tmp_faq/faq_015.md`

## Actions taken

- top-level snapshot docs と practical roadmap docs を再読した。
- typed external boundary、lifetime/fallback、layer compatibility、cut/save-load、hot-plug、practical alpha-1 roadmap の current non-claim を見直した。
- devtools/export の位置づけが optional か essential かを docs wording から再確認した。
- priority judgment / formal-boundary judgment / implementation policy judgment を FAQ 向けに圧縮した。

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2033-priority-devtools-formal-boundary-review-and-faq015-update.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,240p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,260p' specs/07-typed-effects-wiring-platform.md
sed -n '1,260p' specs/13-type-system-lifetime-fallback.md
sed -n '1,260p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,260p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' samples_progress.md
sed -n '1,260p' samples/practical-alpha1/README.md
sed -n '1,260p' scripts/practical_alpha1_export_devtools.py
sed -n '1,340p' tmp_faq/faq_015.md
date '+%Y-%m-%d %H:%M:%S %Z'
git status --short
```

## Evidence / outputs / test results

- `README.md` / `Documentation.md`
  - project axis は「正しい理論」「正しく hot-plug」「Place をまたいで実行・通信・検証・可視化」。
  - final public parser/runtime/verifier/API/ABI、installed binary、backend、packaging は未完と明示。
- `specs/07-typed-effects-wiring-platform.md`
  - external boundary は typed / inspectable / rewritable であるべきとされ、std I/O core primitive 化を避ける current direction が明示。
- `specs/13` / `specs/14` / `specs/15` / `specs/16`
  - fallback / variance / layer compatibility / `atomic_cut` / consistent-cut / hot-plug admission の L1 invariant がかなり具体化されている。
  - 一方で full dependent theory、final public ABI、distributed durable semantics は later に残る。
- `plan/44-practical-alpha1-roadmap.md`
  - practical alpha-1 は front-door -> checker -> runtime -> hot-plug -> transport -> devtools -> save/load -> product preview の bounded package sequence。
  - same-session runtime、distributed durable save/load、final public surface は still later。
- `samples/practical-alpha1/README.md`
  - `RUN-*`, `HP-A1-*`, `TR-A1-*`, `VIS-A1-*`, `SL-A1-*`, `AV-A1-*`, `PE2E-*` の first floors と exact non-claim が整理されている。
- `scripts/practical_alpha1_export_devtools.py`
  - devtools は distinct export bundle / non-final viewer / typed panels & telemetry lanes / stop lines を持つ。
  - typed observability が current line の主張であり、thin debug dump ではない。

## What changed in understanding

- user がここまで求めた基礎機能は、repo の project axis と practical alpha-1 roadmap の両方から見て high priority と言ってよい。
- ただし high priority の中でも first practical substrate と later distributed completion は分けるべきである。
- devtools / visualization は optional polish ではなく essential capability として docs 上も整理済みである。
- 形式検証は full completion ではないが、bounded first-floor 実装を進めるには十分な L1/L2 境界が整っている。
- docs はかなり整理済みだが、safe next package ordering と typed external direct semantic execution lane はまだ implementation policy として詰める余地がある。

## Open questions

- broader save/load widening と same-session runtime のどちらを next promoted package にするか。
- typed external boundary を practical alpha-1 mainline にいつ接続するか。
- devtools を same-session runtime 側へ widen する前に retention / witness / lease のどこまで exact report lane に足すか。

## Suggested next prompt

`ここまでの整理を前提に、first practical substrate と later distributed completion を分けた実装優先順位表を作ってください。`

## Plan update status

- `plan/` 更新不要。
- 今回は roadmap 読み直しと explanation 整理のみであり、新しい repository-memory judgment は追加していない。

## Documentation.md update status

- `Documentation.md` 更新不要。
- current snapshot wording の再確認のみで、新 actualization や stale reference 修正はない。

## progress.md update status

- `progress.md` 更新不要。
- 新しい package close や progress change はない。

## tasks.md update status

- `tasks.md` 更新不要。
- task map を変更する新 blocker 決定はない。

## samples_progress.md update status

- `samples_progress.md` 更新不要。
- sample progress の actual change はない。

## Reviewer findings and follow-up

- reviewer / sub-agent は未使用。
- docs review と local reasoning で閉じた。

## Skipped validations and reasons

- runtime / checker / transport rerun
  - 今回は current docs と existing evidence の読み直しが主目的であり、新しい成功 claim を追加しないため未再実行。
- `python3 scripts/check_source_hierarchy.py` / `python3 scripts/validate_docs.py`
  - report 作成後に実行予定。

## Commit / push status

- committed and pushed:
  - `77c9ed6ab3b0b136e5fec0f732b20bbf5d89e750`
  - message: `docs: review priority and formal boundary`
- current file may receive a small follow-up status-only commit if later push metadata needs explicit mirroring.

## Sub-agent session close status

- sub-agent session なし。
