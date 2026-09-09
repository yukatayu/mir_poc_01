# Mirrorea：証明先行・単独エージェントによる引継ぎ

このフォルダは `sub-agent-pro/mirrorea-proof-first-handoff-v1/` に配置する。フォルダ名に sub-agent を含むが、sub-agentの起動を要求するものではない。**実行担当は主エージェント一人だけ**とし、外部の単発助言は主担当自身がoracleへ依頼する。

## 最初に区別すること

- 目的：普通の意味あるコードから配置・通信を導き、検証・観測でき、稼働後も構成を追加・変更・撤去できる基盤。
- 直近の仕事：全体要件を保持しながら、次に依存する基礎理論を機械検証し、その契約に従う実装へ進むこと。
- 入力の地位：F0.1/F0.2/F0.3は研究候補。要件表v1の詳細受入れ条件とD要件は提案。どれも受理済みCanon、機械検証済みの一般理論、製品ではない。
- 到達先：選択したα profileについて、理論・実装・使える能力を一続きにした**検証済みα候補**。任意の未来理論、公開WAN、World-Web全体の完成をこの実行の無条件な終了条件にはしない。
- 終了しない理由：読了、計画、手証明、有限テスト、oracleの賛成だけで完了扱いしない。
- 終了できる条件：実行プロンプトの完了条件、依頼者の停止、または実行権限・安全性・不可逆な意味選択に関する具体的な停止条件。未達は未達と明記する。

## 読み順

1. 実行用の `RUN_CODEX_MIRROREA.md`（このbundleにも同一コピーがある）。
2. `context/OWNER_INTENT.md`、`context/STATUS_AND_AUTHORITY.md`、`context/RESEARCH_LIMITS.md`。
3. `protocols/SINGLE_AGENT.md`、`protocols/ORACLE.md`、`protocols/PROOF_FIRST.md`、`protocols/READING.md`。
4. `materials/mirrorea_system_map_v1/MASTER.md` と `requirements.json`。119要件、30判断、18保証対象、24シナリオ、8つのα条件を確認する。
5. `materials/mir_foundation_F0_3/FOUNDATION.md`、`theory/PROOFS.md`、`theory/THEOREM_LEDGER.md`、`WORKPLAN.md`、`REQUIREMENT_TRACE.md`、`DECISION_DISPOSITIONS.md`、`AUDIT.md`、`VALIDATION.md`。
6. F0.2・F0.1の本文・証明・監査と、各系統の実装・試験を読む。F0.3だけで前版の機能が全て統合されていると推定しない。
7. 実リポジトリの現行AGENTS/Canon/plan/spec/samples等を `protocols/READING.md` に従って読む。添付の基準SHAへ強制的に戻さない。

## 正確な収録

`archives/` に提供された4 ZIPをそのまま保存し、`materials/` に各ZIPの全ファイルを展開した。以前のcoverage auditの直接添付コード、補助画像も収録した。単独添付の24文書等は全て元ZIP内ファイルとbyte一致した。原本、引用元の研究成果、コード、テスト、反例、検証ログ、JSON台帳、HTML、元の出典情報を削って要約だけにしていない。

`provenance/` に元入力のSHA-256、照合結果、基準リポジトリ情報がある。`MANIFEST.json` は配布bundleの全ファイルを検査する。hash一致は改変検出であり、著者認証・理論の健全性・実装の正しさの証明ではない。

`materials/` と `archives/` は読取り用とする。runnerがログを上書きするので、その場で再実行しない。`tools/run_baselines.py` は作業コピーで試験し、原本を保持する。新しい成果は実リポジトリの認められた研究laneに置き、原本を書き換えて過去の成果を正しかったことにしない。

## 補助ツール

- `python3 tools/verify_bundle.py`：bundleのhash、入力原本、JSONの対応、同梱promptの一致を検査する。
- `python3 tools/run_baselines.py --work-root ABS --evidence-root ABS`：新規の外部workdirにコピーし、F0.1→F0.2→F0.3のunit suiteを逐次実行する。実リポジトリの試験やLean証明ではない。
- `python3 tools/run_logged.py --directory ABS --cwd REPO -- EXACT_COMMAND ...`：任意の検査済みcommandを一度実行し、全出力をファイルに残す。既定180秒間隔、commandの壁時計timeoutなし。oracle自体がdetachした場合、exit 0は回答完了ではない。

補助ツールは使用前にソースを読む。自動設定変更、依存導入、git操作、外部への送信は行わない。
