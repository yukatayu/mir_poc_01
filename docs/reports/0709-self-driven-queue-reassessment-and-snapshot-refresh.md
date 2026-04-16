# Report 0709 — self-driven queue reassessment and snapshot refresh

- Date: 2026-04-17T07:42:58+0900
- Author / agent: Codex
- Scope: `tasks.md` / `progress.md` / `plan/11` を主対象に、current self-driven queue が過度圧縮されていないかを再判定し、必要なら adjacent `plan/17` / `plan/18` / `plan/90` の wording drift まで補正する
- Decision levels touched: L2 / L3

## 1. Objective

`tasks.md` の self-driven queue が current repo memory に対して過度に圧縮されていないかを再判定し、必要なら `progress.md` / `tasks.md` / relevant `plan/` を更新する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `faq_005.md`
- `docs/reports/0708-faq005-literature-mapping-and-theory-lab-followup.md`

前提:

- この task は snapshot / near-term order の調整であり、規範判断の新設は行わない。
- `specs/` の OPEN / comparison / later gate を fact 化しない。
- mixed gate topic でも、boundary-prep までが self-driven なら self-driven queue に戻してよい。

## 3. Actions taken

1. `tasks.md` / `progress.md` / `plan/11` の current queue が「promoted immediate package」と「boundary-prep reserve package」を混同していないかを再読した。
2. `plan/12` / `plan/17` / `plan/18` を突き合わせ、mixed gate topic のうち
   - stronger typed-surface promotion threshold framing
   - theorem discharge transport / public-contract framing
   - model-check property-language / tool-binding framing
   - shared-space fairness / replay mixed-gate boundary
   - public operational CLI installed-binary / packaging success-criteria boundary
   は、実昇格ではなく boundary-prep までなら self-driven と読めることを確認した。
3. `tasks.md` を全面更新し、self-driven queue を
   - promoted immediate 2 本
   - boundary-prep reserve 5 本
   に分けて再構成した。
4. `progress.md` を更新し、current self-driven queue 数、twin peaks の直近研究計画、自走範囲、research/user split を同じ読みへ揃えた。
5. `plan/11` を更新し、near-term order を 2 本だけの line ではなく 7 本の self-driven queue として再構成した。
6. `plan/17` と `plan/18` の wording drift を補正し、`mixed（boundary-prep までは self-driven）` と `no promoted immediate near-term package` の言い分けを明示した。
7. `plan/90` を更新し、この snapshot refresh の traceability addendum を追加した。

## 4. Files changed

- `tasks.md`
- `progress.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/reports/0709-self-driven-queue-reassessment-and-snapshot-refresh.md`

更新不要:

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `tasks.md` / `progress.md` 以外の snapshot docs

## 5. Commands run and exact outputs

- `df -h .`
  - `/dev/vda2 99G 77G 18G 82% /`
- `free -h`
  - `Mem 960Mi total, 840Mi used, 72Mi free, 119Mi available`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-17 07:41:07 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 708 numbered report(s).`
- `git diff --check`
  - 無出力
- `git status --short`
  - 更新対象 7 files と新規 report 1 file のみ

## 6. Evidence / findings

- `tasks.md` に残っていた「self-driven queue は 2 本だけ」という読みは、current repo memory より圧縮が強すぎた。
- 正確には、
  - immediate に promote されている self-driven package は 2 本
  - その後ろに boundary-prep reserve として self-driven に進めてよい package が 5 本
  ある。
- mixed gate は「その話題全体が着手不可」ではなく、「boundary-prep 以降の実昇格で user decision や stronger stop line が必要」という意味で読むのが current repo memory と整合する。
- `plan/17` と `plan/18` は wording 上だけ軽い drift があり、ここを直すことで `tasks.md` / `progress.md` / `plan/11` の読みと一本化できた。

## 7. Changes in understanding

- self-driven package の current reading は「2 本だけ」ではなく、「promoted immediate 2 本 + boundary-prep reserve 5 本」である。
- `Macro 6/7` は mixed gate 本体だが、boundary-prep note までは self-driven に進めてよい。
- typed / theorem / model-check line も、実昇格は mixed gate のまま保ちつつ、framing package は self-driven queue に戻せる。

## 8. Open questions

- modality internalization trigger のあとに、3〜5 の theory-lab framing package をどの順で切るかは、theory-lab lane の drift 次第で再調整しうる。
- reserve integration の 6〜7 は self-driven boundary-prep までであり、final operational profile や installed-binary promotion を self-driven fact と読むべきではない。

## 9. Suggested next prompt

`tasks.md` の promoted immediate 2 本または boundary-prep reserve 5 本から、まとめて進めたい package 数を指定してください。指定がなければ current order どおりに自走できます。
