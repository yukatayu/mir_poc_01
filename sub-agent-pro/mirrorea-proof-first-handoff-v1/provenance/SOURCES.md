# 引継ぎの出所

## 提供された資料

4つの原ZIPと全展開ファイルを保全した。SHA-256とfile数はINPUT_ARCHIVES.json、単独添付との照合はLOOSE_INPUT_COMPARISON.jsonにある。元の研究の出典は各materials内のprovenance/SOURCES等にそのまま残る。

出典一覧があることを、その外部論文を今回全文再読した証拠として扱わない。将来の命題採用に使う場合、versionと定理の前提を再確認する。

## 現在の運用を確認した一次資料

- リポジトリmain metadata： https://api.github.com/repos/yukatayu/mir_poc_01/branches/main
- AGENTS（基準版）： https://github.com/yukatayu/mir_poc_01/blob/19a6decfbea0815b7b4265e8fadfb399270aaed2/AGENTS.md
- oracleの現地運用： https://github.com/yukatayu/mir_poc_01/blob/19a6decfbea0815b7b4265e8fadfb399270aaed2/.docs/oracle-chatgpt-pro-operations.md
- 研究委任の既存境界： https://github.com/yukatayu/mir_poc_01/blob/19a6decfbea0815b7b4265e8fadfb399270aaed2/mirrorea_canon/adr/ADR-0014.md
- I3-3受理とpause： https://github.com/yukatayu/mir_poc_01/blob/19a6decfbea0815b7b4265e8fadfb399270aaed2/mirrorea_canon/adr/ADR-0043.md

これらは基準時の情報であり、remoteや作業treeの将来の状態を固定しない。本bundleはリポジトリ全体の複製ではない。

## Codexとoracleの使い方の参考

- OpenAI, Custom instructions with AGENTS.md： https://developers.openai.com/codex/guides/agents-md （照合時は公式learn.chatgpt.comへredirect）
- OpenAI, Best practices： https://developers.openai.com/codex/learn/best-practices
- oracle upstream README： https://github.com/steipete/oracle

これらから任意のflagや機能を使用可能と推測しない。実際に導入されたlocal manual/helpを優先する。最新userのsub-agent禁止、単発cold-context、180秒間隔の待機は独立の運用指示である。待機経路に有料APIへの自動fallbackを追加しない。
