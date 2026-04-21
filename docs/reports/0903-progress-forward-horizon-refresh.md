# Report 0903 — `progress.md` の forward-looking horizon 再構成

- Date: 2026-04-21
- Author / agent: Codex
- Scope: `progress.md` を、repo ルールを保ったまま、より先を見据えた gate / band / owner 中心の snapshot へ再構成し、必要最小限の `tasks.md` 整合も取る
- Decision levels touched: none; snapshot / roadmap reading の整理のみ、`specs/` 規範変更なし

## 1. Objective

`progress.md` を、現在地だけでなく「次にどの gate / band が見えているか」が読み取りやすい文書へ更新する。

特に次を満たすことを目的とした。

- `progress.md` を thin snapshot のまま保つ
- `tasks.md` の current task map を侵食しない
- `plan/` にある長い reopen chain を `progress.md` に複写しない
- それでも reader が immediate / mid / reserve / user-spec の帯域を見失わないようにする

## 2. Scope and assumptions

- `progress.md` には package inventory を移植せず、topic family + macro + gate + owner の粒度に留める。
- immediate な実行単位、entry command、rough estimate は `tasks.md` を正本とする。
- `plan/10` / `plan/11` / `plan/12` / `plan/13` / `plan/18` の owner split はそのまま維持する。
- `tasks.md` は current task map 自体を全面再編しないが、`progress.md` と矛盾する箇所があれば最小限同期する。

## 3. Documents consulted

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
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- explorer sub-agent result (`Newton`)

## 4. Actions taken

1. AGENTS 指定順に基礎文書と snapshot / plan / task-map の役割分担を再確認した。
2. `.docs/progress-task-axes.md` を読み、`progress.md` に必要な
   `current capability snapshot` / `macro phase map` / `feature maturity matrix` /
   `layer / subsystem status` / `current self-driven line` / `research vs user decision`
   を再確認した。
3. `tasks.md`、`plan/10`、`plan/11`、`plan/12`、`plan/13`、`plan/18` を突き合わせ、
   future-looking な見通しを
   `immediate / mid mixed gate / reserve / heavy future / user-spec hold line`
   の band と owner に再圧縮した。
4. `progress.md` を全面的に再構成し、次の section を追加または再編した。
   - `reproducibility anchors`
   - `current self-driven horizon`
   - `future gate map`
   - `macro phase map` の `next band` / `先読み`
   - `feature family snapshot` の `next band`
   - `layer / track progress` の `next milestone`
   - `owner map for deeper detail`
   - `research を通して見つけること`
   - `user が決める必要があること`
5. reviewer 指摘を受け、`tasks.md` の `rough next order` に `reserve package hardening` を戻し、`progress.md` と矛盾しない最小同期を行った。

## 5. Evidence / outputs / test results

### 5.1 Files changed

- 更新:
  - `progress.md`
  - `tasks.md`
- 追加:
  - `docs/reports/0903-progress-forward-horizon-refresh.md`

補足:

- `plan/` 更新不要
- `specs/` 更新不要

### 5.2 Commands run

主要コマンドのみ記す。

```bash
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ sed -n '1,260p' progress.md
$ sed -n '1,260p' tasks.md
$ sed -n '1,260p' .docs/progress-task-axes.md
$ sed -n '1,260p' plan/10-roadmap-overall.md
$ sed -n '1,260p' plan/11-roadmap-near-term.md
$ sed -n '1,220p' plan/12-open-problems-and-risks.md
$ sed -n '1,220p' plan/13-heavy-future-workstreams.md
$ sed -n '1,320p' plan/18-type-proof-modelcheck-and-ordering-research-program.md
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.

$ git diff --check
(no output)

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-21 13:46 JST
```

### 5.3 What changed in `progress.md`

- `2026-04-21 に再確認した evidence` の長い列挙を、snapshot 向けに `reproducibility anchors` へ圧縮した。
- immediate な実行ラインを、`tasks.md` と同型になりすぎないよう cluster / band 単位へ再整理した。
- 将来見通しを `future gate map` として
  `immediate / mid mixed gate / reserve / heavy future / user-spec hold line`
  の 5 band で整理した。
- `macro phase map` に `next band` と `先読み` を追加し、`Macro 0..8` それぞれの次の読み筋を明示した。
- `feature family snapshot` と `layer / track progress` を、`multi-node / fabric`、`dynamic attach / detach / DAG-safe evolution`、`atomic_cut` と higher-level ordering / `memory_order` family、`Mirrorea / Typed-Effect / Prism / 上位アプリ` が読める粒度へ再編した。
- `owner map for deeper detail` を追加し、future-looking な topic がどの文書に続くかを示した。
- `tasks.md` は全面再編せず、`rough next order` に `reserve package hardening` を戻す最小同期だけを行った。
- reviewer の narrow re-review は `no findings` だった。

## 6. What changed in understanding

- `progress.md` を future-looking にすることは、
  「先の package を全部並べること」ではなく、
  「次にどの band / gate が見えているかを mirror すること」だと明確になった。
- この repo では、forward-looking な情報の上限は
  `topic family + macro + gate + owner`
  までであり、それ以上の package chain は `tasks.md` と `plan/` が持つ方が整合的だと確認できた。

## 7. Open questions

- `tasks.md` にも `band:` 行を薄く足して往復性を上げるかどうかは未決である。
- `progress.md` の forward-looking band を今後さらに増やす必要があるかは、
  current-L2 mainline の closeout 速度よりも、mixed-gate / hold-line の再開速度に依存する。

## 8. Suggested next prompt

`tasks.md` も `progress.md` と往復しやすいように、各 task package に `band` と `stage` を薄く足して整理してください。
