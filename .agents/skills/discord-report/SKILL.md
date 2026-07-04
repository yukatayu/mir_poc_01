---
name: discord-report
description: Use this skill for best-effort Discord progress and completion notifications in this repository. Trigger it before substantial work on tasks that edit files, run commands, or may take more than a few minutes. Record a silent task baseline first, skip progress on short tasks, and send one final completion notification only when ending the user request.
---

# discord-report

この skill は、Codex の作業中に Discord Webhook へ best-effort で通知を送るためのものです。

この文書の `<PYTHON>` は、その環境で使える Python 起動コマンドです。Linux では通常 `python3`、Windows では通常 `py -3`、それがなければ `python` を使います。

## 重要な制約

- この skill 自体は「いつ呼ばれるか」を強制できません。
- 呼び出しのきっかけは、ユーザープロンプト、`AGENTS.md` の指示、明示的な `$discord-report` 呼び出し、または skill description に基づく暗黙マッチです。
- 通知失敗はメイン作業の失敗にしません。Webhook 未設定や一時的な HTTP エラーがあっても、主作業は継続してください。

## 送信ルール

1. 作業を始める前に `begin` を 1 回実行し、差分基準を記録する。`begin` 自体は通知を送らない。
2. タスクが短い場合は途中経過を送らない。
3. タスクが長い場合だけ、意味のある節目で途中経過を送る。
4. 途中経過は「自然な区切り」があり、かつ前回からおおむね 1 時間以上空いたときに送る。目安は平均して約1時間に1回。ただし、ユーザーが明示的に高頻度報告を求めた場合はそちらを優先する。
5. `complete` は、次のユーザー入力が必要で、そのユーザー依頼への最終返答を返して止まる直前に 1 回だけ送る。
6. まだ作業を自走して続けられる場合は `complete` を使わない。必要なら `progress` を使うか、何も送らない。
7. 途中計画、質問待ち、追加調査、まだ作業が続く中間報告では `complete` を使わない。必要なら `progress` を使うか、何も送らない。
8. 要約文は簡潔な日本語で送る。
9. `test` は導入直後または更新直後の疎通確認専用。通常作業では使わない。

## 使用コマンド

作業開始時:

```bash
<PYTHON> .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
```

通知文に backtick、引用符、改行、長い hash 一覧など shell が解釈し得る文字を
含める場合は、inline の `--summary "..."` / `--next-step "..."` ではなく
一時テキストファイルを作り、`--summary-file` / `--next-step-file` で渡す。
これは shell command substitution による通知文欠落を避けるための運用ルール。

途中経過:

```bash
<PYTHON> .agents/skills/discord-report/scripts/discord_notify.py progress \
  --summary "<ここまでに終えたこと>" \
  --next-step "<次にやること>" \
  --cwd .
```

shell quoting を避ける途中経過:

```bash
<PYTHON> .agents/skills/discord-report/scripts/discord_notify.py progress \
  --summary-file /tmp/codex-discord-summary.txt \
  --next-step-file /tmp/codex-discord-next-step.txt \
  --cwd .
```

完了通知:

```bash
<PYTHON> .agents/skills/discord-report/scripts/discord_notify.py complete \
  --summary "<最終的に何をしたかを1〜3文で>" \
  --include-diff \
  --cwd .
```

shell quoting を避ける完了通知:

```bash
<PYTHON> .agents/skills/discord-report/scripts/discord_notify.py complete \
  --summary-file /tmp/codex-discord-summary.txt \
  --include-diff \
  --cwd .
```

導入直後または更新直後の疎通確認:

```bash
<PYTHON> .agents/skills/discord-report/scripts/discord_notify.py test \
  --installation-check \
  --summary "疎通確認" \
  --cwd .
```

## summary の書き方

- 具体的に書く。
- 1 回の通知で 1〜3 文に収める。
- 実装・調査・検証のどこまで進んだかが分かるようにする。
- 完了通知では、必要なら「テスト実施」「未解決事項」「手動確認が必要な点」を 1 文で補足する。

## diff の扱い

- `--include-diff` は、Git 管理下で `begin` が先に実行されているときは task-scoped の差分を表示します。
- `begin` が未実行でも Git 差分が取れる場合は、`変更量(参考)` として現在のワークツリー差分を表示します。
- Git 管理外などで差分が取れない場合だけ、差分欄を出しません。
- `.codex-discord/` 配下の local state や秘密設定は差分集計に含めません。
