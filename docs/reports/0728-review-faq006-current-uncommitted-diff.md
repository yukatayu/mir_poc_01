# Report 0728 — review faq006 current uncommitted diff

- Date: 2026-04-17T09:33:27.279781Z
- Author / agent: Codex
- Decision levels touched: なし（review only）

## 1. Objective

- current uncommitted FAQ task diff を maintainer 観点で review し、semantic correctness、current repo state との drift、`faq_006.md` の settled/current/open の切り分け、`Documentation.md` と `docs/reports/0727...` の整合を確認する。

## 2. Scope and assumptions

- review 対象は uncommitted changes のみ:
  - `faq_006.md`
  - `Documentation.md`
  - `docs/reports/0727-faq006-current-knowns-open-questions-and-tradeoffs.md`
- 判定の正本は `specs/`、current snapshot は `progress.md` / `tasks.md`、長期参照は `plan/` とする。
- この task では review report 追加以外の repo 状態は変更しない。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/examples/444-current-l2-modality-internalization-trigger-note.md`
- `faq_005.md`
- `faq_006.md`
- `docs/reports/0727-faq006-current-knowns-open-questions-and-tradeoffs.md`

## 4. Actions taken

- uncommitted diff を確認した。
- 必読順に沿って normative / snapshot / adjacent FAQ / draft report を読んだ。
- FAQ の各主張を current sample inventory、current mixed-gate boundary、syntax/modality の current wording と照合した。
- report draft を repo の reporting policy と照合した。
- files changed:
  - `docs/reports/0728-review-faq006-current-uncommitted-diff.md`
- commands run:
  - `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
    - `Task baseline recorded.`
  - `git status --short`
    - ` M Documentation.md`
    - `?? docs/reports/0727-faq006-current-knowns-open-questions-and-tradeoffs.md`
    - `?? faq_006.md`
  - `python3 scripts/new_report.py --slug review-faq006-current-uncommitted-diff`
    - `/home/yukatayu/dev/mir_poc_01/docs/reports/0728-review-faq006-current-uncommitted-diff.md`
  - `date '+%Y-%m-%d %H:%M %Z'`
    - `2026-04-17 18:33 JST`

## 5. Evidence / outputs / test results

- `Documentation.md` change は FAQ 導線追加のみで、追加先自体は current state と整合する。
- `faq_006.md` の大半は current snapshot と整合する。
  - authored sixteen / prototype octet / helper-local preview / mixed-gate framingの説明は `progress.md` / `tasks.md` / sample README 群と噛み合う。
- actionable findings:
  1. `faq_006.md:137` は fairness / replay を「current line に入れるか、mixed gate に留めるか」という現在進行の比較に見せているが、current snapshot ではその境界はすでに fixed で、remaining topic は final operational profile 側だけである。`progress.md:19-21` と `tasks.md:20-22` はどちらも `Macro 6/7 mixed-gate boundary fixed` と明記しているため、ここは「current line に入れるか」ではなく「mixed gate に留めた上で final profile をどう詰めるか」に言い換えるべきである。
  2. `docs/reports/0727-faq006-current-knowns-open-questions-and-tradeoffs.md:8-98` は repo の report template 順序に従っていない。`AGENTS.md:55-65` は report を `Objective -> Scope and assumptions -> Documents consulted -> Actions taken -> Evidence / outputs / test results -> What changed in understanding -> Open questions -> Suggested next prompt` の順で持つことを要求しているが、0727 は `Inputs consulted`、`Files changed`、`Commands run` を別 section として挿入し、必須の `Scope and assumptions` 見出しも欠いている。内容自体は有用でも、監査時に report policy 不一致として扱われる。

## 6. What changed in understanding

- FAQ 本文の主問題は broad factual drift ではなく、open/settled の粒度ずれだった。
- `Documentation.md` の FAQ 導線追加自体には問題を見つけなかった。
- 0727 report は内容の方向性ではなく、repo 固有の report 形式要件への不一致が主問題だった。

## 7. Open questions

- `faq_006.md` の syntax/modality 節で、`semantic honesty` の扱いを current comparison principle と open formalization のどちらに寄せて書くかは、`specs/10` と `specs/11` の読み分けをもう一段明示してもよい。
- この review task では `faq_006.md:311-312` を finding に上げるほどの断定 drift は確認しなかったが、次の編集で wording を少し soften すると safer である。

## 8. Suggested next prompt

- `faq_006.md` の fairness / replay の文言を current mixed-gate boundary fixed に合わせて修正し、`docs/reports/0727...` を repo の report template 順に並べ直してください。
