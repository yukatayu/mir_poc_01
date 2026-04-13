# 0696 — GPT-5.4-pro 質問束準備

## Objective

外部の `GPT-5.4-pro` へ独立に投げるための質問文を整理し、`sub-agent-pro/` 以下へ `question_001.md` 形式で保存する。

## Scope and assumptions

- 質問は互いに独立であることを前提にする。
- user は各質問の前に `plan/`、次に `specs/` を読ませる想定である。
- 今回は repo の規範判断を追加しない。
- `progress.md` / `tasks.md` / `plan/` は current snapshot を変えないため、更新対象にしない。
- `progress.md` 更新不要。
- `tasks.md` 更新不要。
- `plan/` 更新不要。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `faq_003.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `.agents/skills/discord-report/SKILL.md`

## Actions taken

1. repo の current status、near-term line、heavy future workstream、shared-space boundary を再読した。
2. 外部相談の価値が高い論点を、
   - 大局 roadmap
   - 型理論
   - theorem / model-check / external verifier
   - parser / public API / public CLI
   - shared-space / Mirrorea
   - host-facing I/O / FFI / visualizer / engine adapter
   - sample corpus / blind spot
   に分解した。
3. `sub-agent-pro/README.md` を新設し、利用手順と質問一覧を記載した。
4. `sub-agent-pro/question_001.md` から `question_016.md` までの 16 本を追加した。

## Evidence / outputs / test results

- resource check
  - `df -h .`
    - `/` は `99G` 中 `19G` 空き
  - `free -h`
    - `Mem 960Mi / available 243Mi`
- timestamp
  - `date '+%Y-%m-%d %H:%M:%S %Z'`
    - `2026-04-14 07:55:13 JST`

## What changed in understanding

- external 相談は 1 本の巨大質問より、理論軸ごとに独立させた方がこの repo に合う。
- current repo の相談価値が高い論点は、near-term 実装そのものよりも、重い future workstream の切り分けと boundary design に集中している。
- 特に、型理論、memory-order 再構築、theorem/model-check cut、shared-space final catalog、I/O / adapter / engine integration は、今まとめて聞いておく価値が高い。

## Open questions

- user が実際にどの順番で外部相談を回すかは未定である。
- 回答を `answer_XXX.md` へ貼った後、どの粒度で repo の `tasks.md` / `progress.md` / `plan/` に反映するかは、その回答内容次第である。

## Suggested next prompt

`sub-agent-pro/question_001.md` から順に、外部回答を `answer_001.md` 形式で追加してください。追加後は、その回答を読み込んで repo 側の計画・research line へ反映します。
