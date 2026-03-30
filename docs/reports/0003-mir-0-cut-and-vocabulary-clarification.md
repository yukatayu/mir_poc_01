# 報告 0003 — Mir-0 の cut / 語彙境界の明確化

- 日付: 2026-03-27T04:33:07.992517Z
- 担当 / agent: Codex
- 対象範囲: Mir-0 の境界を広げずに、`atomic_cut`、`perform`、`barrier`、`durable_cut` の位置づけを狭く固定する。
- 触れた decision level: L1 の境界明確化。L0 の因果・failure・ownership invariants 自体は変更しない。

## 1. 目的

Mir-0 の最小意味論核のうち、cut と effect request operation の扱いに残っていた曖昧さを減らし、Mir-0 に入るものと Mir-1 以降へ送るものをより明確にする。

## 2. スコープと前提

- 今回の対象は仕様整流のみであり、実装・コード生成・新規サブシステム設計は行わない。
- 会話の過去文脈は正本とせず、repo 内文書のみを根拠にした。
- 既存仕様を広げないことを優先し、未確定事項は未決定として残した。
- PrismCascade や Mirrorea の詳細設計には踏み込まず、Mir-0 境界確認に必要な範囲だけ参照した。
- `Documentation.md` は今回の判断を反映しなくても全体要約として破綻しなかったため、更新しなかった。

## 3. 参照した文書

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0001-mir-0-semantic-core-alignment.md`
- `docs/reports/0002-mir-0-review-findings.md`
- 境界確認のみ:
  - `specs/05-mirrorea-fabric.md`
  - `specs/07-typed-effects-wiring-platform.md`
  - `specs/08-cross-system-relations.md`

## 4. 実施内容

1. 指定順で文書を読み直し、Mir-0 の現状定義と直近レビュー結果を再確認した。
2. `code_mapper` を使い、今回触るべきファイルが `specs/04`, `specs/09`, `specs/10`, `specs/12` にほぼ限定されることを確認した。
3. `specs/04-mir-core.md` を修正し、Mir-0 における `atomic_cut` の意味を次のように固定した。
   - `atomic_cut` は current `place` の rollback frontier だけを確定する。
   - 単一プロセス全体の同期点でも、複数 `place` をまたぐ合意点でも、永続化完了点でもない。
   - `try` の rollback は current `place` の中で閉じ、途中の `atomic_cut` を越えない。
   - cut 後に failure が出ても pre-cut 部分は rollback されず、compensation / fallback / 明示的 failure handling に移る。
4. `specs/04-mir-core.md` で `perform` を Mir-0 の正式語彙としては採用せず、最小 effect request operation を説明するための便宜的記法に下げた。
5. `specs/04-mir-core.md` と `specs/10-open-questions.md` で、`barrier` と `durable_cut` を Mir-0 から外し、Mir-1 以降の cut vocabulary 側へ送る整理に揃えた。
6. `specs/09-invariants-and-constraints.md` を更新し、`atomic_cut` の scope と rollback の停止条件が Mir-0 と矛盾しないようにした。
7. `specs/12-decision-register.md` に D-012 を追加し、今回の cut / 語彙境界の判断を記録した。
8. 日本語の新規レポートとして本ファイルを追加した。

## 5. 変更したファイル

- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md`

## 6. 実行したコマンドと出力

- `python3 scripts/new_report.py --slug mir-0-cut-and-vocabulary-clarification`
  - Output: `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - Output: `/home/yukatayu/dev/mir_poc_01/docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md`
- `git status --short`
  - Output at start of this task:
    - `M specs/04-mir-core.md`
    - `M specs/09-invariants-and-constraints.md`
    - `M specs/10-open-questions.md`
    - `M specs/12-decision-register.md`
    - `?? docs/reports/0001-mir-0-semantic-core-alignment.md`
    - `?? docs/reports/0002-mir-0-review-findings.md`

## 7. 根拠・所見・テスト結果

- `atomic_cut` を Mir-0 に残すなら、place-local な rollback frontier の確定に限定するのが最も狭い。
- `durable_cut` を Mir-0 に残すと persistence / distributed finalization を持ち込むため、Mir-1 以降へ送る方が境界は明確になる。
- `barrier` は Mir-0 の意味論核に必須ではないため、Mir-0 語彙から外す方が境界は狭く保てる。
- `perform` は意味論説明には便利だが、repo 内文書だけでは最終的な表面構文として確定する根拠がない。

規範変更メモ:

- 今回は `atomic_cut` を current `place` の rollback frontier を確定する最小 cut として固定した。
- `durable_cut` は Mir-0 に含めず、Mir-1 以降の persistence / distributed finalization 側へ送った。
- `barrier` は Mir-0 の cut vocabulary から外した。
- `perform` は規範的表面構文ではなく、説明用記法として整理した。

## 8. 理解の更新

- `try` / rollback / failure handling の関係は、`atomic_cut` を place-local cut と固定すると最も素直に整合する。
- Mir-0 を曖昧にする主因は、cut の scope と語彙の地位を同時に未決のまま抱えていたことだった。
- `durable_cut` と `barrier` は、Mir-0 の内側で迷わせるより、Mir-1 以降の cut vocabulary 候補として外に出した方がよい。
- `perform` は意味論上の最小操作の呼び名としては使えるが、構文仕様まで確定する段階にはまだない。

## 9. 未解決事項

- `barrier` が Mir-1 以降で独立した cut vocabulary として本当に必要かどうか。
- `durable_cut` が要求する persistence / distributed finalization guarantees の正確な定義。
- `perform` を最終的な表面構文として採用するか、それとも別の表記に置き換えるか。
- fallback 正規化の最終形。
- `emit` と effect handler / routing の関係。
- `place` 導入規則と cross-place transfer の静的規則。

## 10. 次にやるべき作業

`specs/04, specs/09, specs/10, specs/12 を前提に、Mir-1 側の cut vocabulary 候補として durable_cut と barrier の関係だけを整理し、Mir-0 を広げずに「後段へ送った概念の責務分担」を日本語で明文化してください。`
