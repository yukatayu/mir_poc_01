# Report 0916 — Sugoroku sample progress alignment

## 1. Title and identifier

- Identifier: `0916`
- Title: `Sugoroku sample progress alignment`

## 2. Objective

`samples_progress.md` の phase 4 / 7 読みを、`samples/clean-near-end/sugoroku-world/` の
actual sample、actual command、actual debug surface、actual report path と tighter に揃える。

## 3. Scope and assumptions

- 対象は active Sugoroku family の docs / dashboard / explanation であり、runtime semantics の新設計ではない。
- single OS process logical multi-place emulator という current stop line は維持する。
- real network、multi-server consensus、durable distributed commit、final public visualization protocol は固定しない。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/hands_on_sugoroku_00_overview.md`
- `docs/research_abstract/hands_on_sugoroku_detail.md`
- `docs/reports/0909-sugoroku-world-runtime-attachment-vertical-slice.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`

## 5. Actions taken

- `samples/clean-near-end/sugoroku-world/README.md` を拡張し、`SUG-00..09` の目的、phase 読み、primary command、preferred debug、expected outcome を 1 つの matrix に揃えた。
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md` を追加し、sample family の読み方、debug 出力の役割、phase 4 / 7 / 14 の切り分けを reader-facing に整理した。
- `samples_progress.md` の active sample matrix を broad PH4 / PH7 row から `SUG-00..09` row へ切り替え、focused debug evidence を紐付けた。
- `progress.md`、`tasks.md`、`README.md`、`Documentation.md`、`plan/01`、`plan/11`、`plan/16`、`plan/17` を同期し、current promoted next line が `TermSignature registry / debug output` であることを反映した。

### files changed

- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/hands_on_sugoroku_sample_matrix.md`
- `samples_progress.md`
- `progress.md`
- `tasks.md`
- `README.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`

### commands run

- `python3 scripts/sugoroku_world_samples.py check-all`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace`
- `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership`
- `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 6. Evidence / outputs / test results

- `python3 scripts/sugoroku_world_samples.py check-all`
  - 10 sample すべて pass
  - `failed: []`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - active sample root は `samples/clean-near-end/sugoroku-world`
  - debug output modes は `summary`, `turn-trace`, `membership`, `verification`, `json`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json`
  - `draw = 4`
  - witness `draw_pub#1`
  - `dice_owner` は `Bob` へ handoff
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug turn-trace`
  - `publish game_started`
  - `Alice roll draw=4`
  - `publish roll_result(Alice, 4) witness=draw_pub#1`
  - `handoff dice_owner Alice -> Bob using witness=draw_pub#1`
- `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership`
  - `epoch: 1`
  - `Alice`, `Bob`, `Carol`, `Dave` が active/pending distinction を伴って読める
- `python3 scripts/sugoroku_world_samples.py run 08_reset_interleaving_model_check --debug verification`
  - static / runtime / model-check の 3 行を human-readable に確認
- `python3 scripts/check_source_hierarchy.py`
  - required `23`, missing `0`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 913 numbered report(s).`
- `git diff --check`
  - clean

## 7. What changed in understanding

- Sugoroku family の「E2E」は 1 本の monolithic wrapper ではなく、`SUG-01` attach、`SUG-03` turn、`SUG-05..07` membership、`SUG-08` verification を束ねた family reading として示した方が正確である。
- `SUG-09` detach TODO は active matrix に入れてよいが、phase 14 completion evidence と誤読されないよう `%` を `10%` に留める必要がある。
- helper-local debug output は十分に useful だが、final public visualization protocol と混同しない明示が必要だった。

## 8. Open questions

- `--debug signatures` をどの surface で増やすか
- Sugoroku helper の debug view と今後の `VisualizationProtocol` をどこで接続するか
- detach lifecycle を `AttachPoint` package までどう stop line として保持するか

## 9. Suggested next prompt

`TermSignature registry / debug output` を進めてください。`03_roll_publish_handoff` と clean near-end order/handoff 1 本を対象に、term / transition / witness / effect route の shared signature carrier と `--debug signatures` 相当の出力を追加し、`samples_progress.md` / `progress.md` / `tasks.md` / report を同期してください。
