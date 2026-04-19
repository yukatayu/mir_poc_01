# Report 0739 — faq 007 current status two big problems and last mile gates

- Date: 2026-04-17T16:01:10.518526Z
- Author / agent: Codex (GPT-5)
- Scope: current status synthesis を `faq_007.md` と導線 docs に反映し、二大問題・language-side implementation・last-mile gate・autonomy expectation を source-backed に整理する。
- Decision levels touched: L1/L2 summary only。新しい規範判断は追加しない。

## 1. Objective

- user の質問
  - 現状どこまで何が終わっているか
  - 二大問題を完全に解決して language-side implementation まで終わっているか
  - 全体像に対して今どこにいて、何がまだ必要か
  - 何を答えればどこまで自走できるか
  を、current repo sources に沿って `faq_007.md` に正確にまとめる。
- current explanation を doc entry point に追加し、`Documentation.md` と `specs/00-document-map.md` の導線 drift も解消する。

## 2. Inputs consulted

- Required repo entry docs
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `.docs/progress-task-axes.md`
  - `specs/00-document-map.md`
  - `specs/01-charter-and-decision-levels.md`
  - `specs/02-system-overview.md`
  - `specs/03-layer-model.md`
  - `specs/09-invariants-and-constraints.md`
- Current status / roadmap / memory
  - `tasks.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- Current explanation / integration anchors
  - `faq_006.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`
  - `specs/examples/462-current-l2-theory-line-near-end-closeout-and-mixed-gate-only-reading.md`
  - `specs/examples/463-current-l2-verifier-preview-alignment-prefloor-and-public-contract-mixed-gate-note.md`
  - `specs/examples/464-current-l2-model-check-projection-prefloor-and-property-tool-seam-mixed-gate-note.md`
  - `specs/examples/465-current-l2-theorem-discharge-prefloor-and-public-contract-mixed-gate-note.md`

## 3. Actions taken

1. Recorded a task baseline for Discord diff tracking and checked local resource headroom before substantial work.
2. Re-read required entry docs, then current snapshot / roadmap / theory-lab integration docs, and extracted the current repo reading:
   - first-line integration closed
   - narrow mixed-gate pre-floor closed
   - remaining work narrowed to mixed gate / user-spec gate
   - final parser / final public contract / concrete tool binding / final app target remain open
3. Compared `Documentation.md` against `progress.md` and found stale wording in the theory-lab lane (`early-active` versus current `near-end closeout` reading).
4. Wrote `faq_007.md` as a current explanation document that answers the user’s questions explicitly and separates:
   - already finished
   - not finished
   - mixed gate
   - true user-spec gate
   - autonomy expectation
5. Updated doc entry points so the new FAQ is discoverable from `Documentation.md` and `specs/00-document-map.md`.
6. Updated `plan/90-source-traceability.md` with an addendum for this FAQ/status-synthesis task.

## 4. Files changed

- Added:
  - `faq_007.md`
- Updated:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `plan/90-source-traceability.md`
  - `docs/reports/0739-faq-007-current-status-two-big-problems-and-last-mile-gates.md`

- `progress.md 更新不要`
  - current status / queue / gate reading already matched the latest source-backed state and did not change in this task.
- `tasks.md 更新不要`
  - current task map already matched the latest source-backed state and did not change in this task.
- `plan/` updates
  - long-lived status memory did not need semantic refresh beyond traceability; therefore `plan/90` only was updated.

## 5. Commands run and exact outputs

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   78G   17G  83% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       562Mi        70Mi       0.0Ki       325Mi       397Mi
Swap:             0B          0B          0B

$ python3 scripts/new_report.py --slug faq-007-current-status-two-big-problems-and-last-mile-gates
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0739-faq-007-current-status-two-big-problems-and-last-mile-gates.md
```

Read-only inspection commands were also used to consult the listed source files via `sed -n` and `rg`; their outputs were the current contents of those source files and are not duplicated here because the files themselves are the canonical record.

## 6. Evidence / findings

- Current repo reading is:
  - execution lane: `Macro 4 active on fixed authored/prototype floor`
  - theory-lab lane: `Macro 5 near-end closeout: no live first-line integration package`
  - reserve integration lane: `Macro 6 docs-first boundary only / Macro 7 mixed`
- The two big problems are **not fully solved** in the sense of final adoption.
  - Problem 1 has current first-line integration and three helper-local pre-floors, but not stronger typed-surface adoption, theorem public-contract adoption, settled property language adoption, or concrete tool binding.
  - Problem 2 has current first-line integration and corrected prototypes, but not final order/handoff source wording, final emitted-artifact schema, final fairness/replay operational profile, or final shared-space catalog.
- Language-side implementation is **not complete**.
  - The repo has parser-free validation substrate, fixed runnable subset, corrected runnable prototypes, CLI, and compare helpers.
  - The repo does not yet have final parser grammar, final public parser/checker/runtime API, or final theorem/model-check public contract.
- Implementation/execution comparison is already substantial.
  - `current_l2_source_sample_runner`: `22 passed`
  - `current_l2_operational_cli`: `12 passed`
  - `current_l2_verifier_preview_alignment`: `5 passed`
  - `current_l2_model_check_projection_prefloor`: `5 passed`
  - `current_l2_theorem_discharge_prefloor`: `5 passed`
- Answering true user-spec gates would let the repo self-drive much further, but would not honestly guarantee “complete no-question run to the very end,” because several remaining items are evidence-driven mixed-gate adoption judgments rather than mere missing product requirements.
- `Documentation.md` contained stale wording for the theory-lab lane and now matches `progress.md` / `tasks.md`.

## 7. Changes in understanding

- The most accurate current phrasing is not “theory solved” and not “implementation finished.”
- The most accurate current phrasing is:
  - first-line integration closed
  - narrow mixed-gate pre-floor closed
  - remaining work narrowed to mixed gate / user-spec gate
  - final public language/tool/application realization still open
- The repo is further along than a pure research skeleton, but not yet at “two big problems fully solved + language implementation done.”
- The user’s ideal “answer once and let the repo self-drive to the end” can be approached, but not fully promised yet. The blocker is no longer lack of comparison material; it is the remaining actual-adoption and target-selection gates.

## 8. Open questions

- Which first external integration target should drive the next concretization:
  theorem prover, model-check stack, or host/application?
- What backend / tooling success criteria count as “done enough”:
  repo-local CLI, installed binary, or host adapter / FFI integration?
- What first application target should define the end-to-end path?
- What scope should the shared-space final catalog and fairness/replay profile cover?
- Does the user prefer earlier source-visible stronger typed surface promotion, or a longer checker-adjacent principal phase?

## 9. Suggested next prompt

`faq_007.md` を読んだ上で、true user-spec gate と mixed-gate preference をまとめて答えてください。特に、first external integration target、backend/tooling success criteria、first application target、shared-space final catalog / fairness-replay scope、stronger typed surface を早めに source へ上げたいかどうか、を指定してもらえると、その先の actual adoption / implementation package をかなり長く self-driven に進められます。
