# Report 0938 — FAIRY-05 residual reacquire wording risk review

- Date: 2026-04-27 21:25:25 JST
- Author / agent: Codex
- Scope: review-only for `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`, `docs/research_abstract/avatar_fairy_follow_plan_01.md`, `plan/24-avatar-follow-representative-slice-roadmap.md`, `specs/10-open-questions.md`
- Decision levels touched: none; wording-risk review only

## 1. Objective

FAIRY-05 residual reacquire design の docs-first package 前に、current snapshot wording が
active / planned / mixed gate を混同していないか、FAIRY-05 を過剰昇格していないか、
future public API を誤読させないか、state-timeline / anchor-switch / visibility-return witness の
carrier 未定義を隠していないか、macro phase label drift を起こしていないかを点検する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md`
- `plan/24-avatar-follow-representative-slice-roadmap.md`

## 3. Actions taken

1. repo 指示どおりの順序で基礎文書と基礎 specs を確認した。
2. FAIRY-05, active, planned, mixed gate, public API, carrier, macro phase 周辺語を対象 docs 群で横断検索した。
3. snapshot docs と `specs/10-open-questions.md` の gate / unresolved wording を比較し、矛盾と未定義面を洗った。

## 4. Files changed

- `docs/reports/0938-review-fairy05-residual-reacquire-wording-risk.md` を新規追加

target docs 本体は未編集。

- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `rg --files specs | sort`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,260p' progress.md`
- `sed -n '1,240p' specs/00-document-map.md`
- `sed -n '1,240p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' tasks.md`
- `sed -n '1,280p' samples_progress.md`
- `sed -n '1,260p' plan/00-index.md`
- `sed -n '1,260p' .docs/progress-task-axes.md`
- `rg -n "FAIRY-05|active|planned|mixed gate|public API|public contract|final public|anchor-switch|visibility-return|state timeline|state-timeline|witness|macro phase|Macro [0-9]|phase 8|phase 9|phase 13|reacquire|reopen|carrier" ...`
- `nl -ba ...` on target docs for line-anchored review
- validation / tests は未実行
  - review-only task のため

## 6. Evidence / findings

### Finding 1 — queue / gate status of `FAIRY-05` is not stated with one consistent level

High severity.

- `samples_progress.md:5` は `Current active packages` に `promoted FAIRY-05 residual reacquire design` を置いている。
- 同じ `samples_progress.md:79-80` は `FAIRY-05` を `residual planned skeleton` / `10` と書く。
- 同じ `samples_progress.md:135` は blocker severity を `low` とし、`decide later` と書く。
- `tasks.md:141-156` は `FAIRY-05` を current self-driven `Package 1` として書く。
- `progress.md:99-100` は同件を `後段依存` と書く。
- 一方で `docs/hands_on/avatar_fairy_follow_representative_slice_01.md:41-48` と `plan/24-avatar-follow-representative-slice-roadmap.md:42-49` は `FAIRY-05` を residual planned family に留める。

このため reader は、`FAIRY-05` を
active 実装 package、planned sample skeleton、optional later blocker、後段依存 package
のどれとして読むべきかを一意に判断しにくい。

修正提案:

- `FAIRY-05` の状態語彙を 3 つに固定する。
  - active evidence: `FAIRY-01/02/03/04/06`
  - next docs-first package: `FAIRY-05 residual reacquire design review`
  - planned sample family: `samples/not_implemented/.../FAIRY-05`
- `Current active packages` のような見出しでは `FAIRY-05` を active 側に置かない。
- `後段依存` と `self-driven next package` を両立させたいなら、
  `sample remains planned, but docs-first design review is the next self-driven package`
  のように 2 層を 1 文で分離する。

### Finding 2 — `FAIRY-05` の prerequisite carrier が未定義なまま、必要概念だけが先走っている

High severity.

- `tasks.md:148-150` は `state-timeline / anchor-switch debug carrier` と `state timeline view / anchor switch evidence` を deliverable に置く。
- `progress.md:100` も同じ carrier requirement を示す。
- `docs/research_abstract/avatar_fairy_follow_plan_01.md:38` は `FAIRY-05` の debug focus を `state timeline / anchor switch` とする。
- 同文書 `:61-63` は `membership / anchor / witness / visualization` をどこまで同じ carrier に乗せるか未決だと書く。
- `specs/10-open-questions.md:180-182` は `FAIRY-05` widening、`state-timeline / anchor-switch carrier`、future lane connection を未決とする。

しかし snapshot docs 群には、user が指定した `visibility-return witness` を
どの carrier で表すか、そもそも
`state timeline` / `anchor switch` / `visibility-return witness`
を 1 本に束ねるのか 3 本に分けるのかが明示されていない。

この欠落のまま `reacquire-after-return` を進めると、
visibility return を mere membership refresh へ潰す、
anchor switch を hidden repair と誤読する、
witness を既存 `witness_refs` / visualization lane に暗黙吸収する
という silent requirement invention が起きやすい。

修正提案:

- FAIRY-05 へ触れる snapshot 文には次の種別を明記する。
  - `UNRESOLVED: reacquire-after-return needs an explicit decision on whether visibility return is carried as timeline event, anchor-switch event, witness event, or a typed bundle of these.`
- 既存 helper carrier で十分だと言っていないことを明文化する。
  - `current anchors / membership / verification / visualization_views are not yet the settled FAIRY-05 carrier`
- `state-timeline` や `anchor-switch` を provisional name として扱うなら、その旨も書く。

### Finding 3 — `phase 8` と `Macro 6` / `Macro 8` の書き分けが弱く、phase label drift を起こしやすい

Medium severity.

- `docs/hands_on/avatar_fairy_follow_representative_slice_01.md:3`
- `docs/research_abstract/avatar_fairy_follow_plan_01.md:3`
- `plan/24-avatar-follow-representative-slice-roadmap.md:5`
  はいずれも `phase 8 avatar fairy follow` と書く。
- 一方で `tasks.md:143-145` は `Macro 6, reserve` と分類する。
- `progress.md:67-77` では `Macro 8` が application realization 専用として別定義されている。
- `.docs/progress-task-axes.md:10-13` と `:91-99` は、historical phase label と macro phase を明示的に書き分けるよう求める。

いまの wording だと、`phase 8` が
legacy sample-family label なのか、
current repo-wide macro phase なのか、
`Macro 8` と関係があるのかが読み手に依存する。

修正提案:

- FAIRY docs では `phase 8` を必ず `legacy phase 8 sample-family label` と呼ぶ。
- 同じ段落か表で `current macro-phase reading: Macro 6 (reserve)` を併記する。
- `phase 8` 単独表現は snapshot docs から避ける。

### Finding 4 — root snapshot docs の FAIRY-05 queue wording is more assertive than the residual stop-line wording

Medium severity.

- `README.md:64` と `Documentation.md:66` は、
  `cross-package sweep` 後の `current next queue` / `次の package` を
  `FAIRY-05 residual reacquire design` と強く前景化する。
- しかし同じ snapshot 群の FAIRY 専用 docs では、
  `docs/hands_on/...:31-37,39-48` と `plan/24...:40-49,60-70` が
  repo-local representative slice only / residual planned family / no final public API をより慎重に書く。

root snapshot docs だけを skim すると、
`FAIRY-05` が active widening に入った、
あるいは avatar/public visualization surface を次に ratchet する package だと誤読しやすい。

修正提案:

- root snapshot の FAIRY-05 queue 文に 1 句だけ足す。
  - 例: `next queue is FAIRY-05 residual reacquire design review, while the sample itself remains planned and no public avatar/visualization API is implied.`
- README / Documentation 側では `design review`、`planned sample remains under samples/not_implemented` の 2 点を同じ文に入れる。

## 7. Changes in understanding

- current snapshot は FAIRY-05 を「未実装 sample」と「次に扱う docs-first design package」の二重身分で運用している。
- 問題はその二重身分自体より、どの文書でも同じ 2 層の分け方で書けていない点にある。
- future public API misread については、FAIRY 専用 hands-on / plan 文書の stop line 自体は比較的明示的で、より大きい risk は root snapshot と dashboard 側の queue wording にある。

## 8. Open questions

- `visibility-return witness` を `FAIRY-05` の必須概念として独立させるか。
- `state-timeline` / `anchor-switch` を carrier 名候補として扱うのか、単なる説明語として扱うのか。
- `FAIRY-05` docs-first package の優先順位を、本当に `Typed external boundary / adapter executable widening` より前に置くのか。

## 9. Suggested next prompt

`0938` report の findings に沿って、対象 docs の wording だけを修正してください。特に `FAIRY-05` の status を `active evidence / next docs-first package / planned sample family` の 3 層に固定し、`legacy phase 8` と `Macro 6 reserve` を併記し、carrier 未定義を `UNRESOLVED` として明文化してください。
