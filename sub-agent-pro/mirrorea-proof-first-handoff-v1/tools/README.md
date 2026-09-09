# 補助ツールの使い方

全てPython標準ライブラリのみ。使用前にコードを読む。oracle送信、Gitの変更、依存導入は自動で行わない。

## 配布とpromptの照合

リポジトリルートから:

```bash
python3 sub-agent-pro/mirrorea-proof-first-handoff-v1/tools/verify_bundle.py
```

別添promptも照合する場合は、その実在するpathを`--prompt-file`へ渡す。MANIFESTはhash照合であり、自己署名された承認や数学的証明ではない。

## 元のテストの再現

```bash
python3 sub-agent-pro/mirrorea-proof-first-handoff-v1/tools/run_baselines.py \
  --work-root /CONFIRMED_EXTERNAL_WORKDIR/mir-baseline-copy-UNIQUE \
  --evidence-root /CONFIRMED_EXTERNAL_WORKDIR/mir-baseline-evidence-UNIQUE
```

`CONFIRMED_EXTERNAL_WORKDIR`等は説明用。mount・容量と実際の新規pathを確認して置換する。既存directoryは上書きしない。これはunit suiteのみで、SMT・Lean・現repo・実networkの再検証ではない。

## oracleへ渡す凍結packet

`templates/ORACLE_QUESTION.md`を、具体的な文脈で埋めたQUESTION.mdへ保存する。そのファイルと、手で選んだrepo相対ファイルだけを`make_oracle_packet.py`へ渡す。`--repo`、`--question`、`--output`、反復可能な`--file`を使う。出力には質問、選択ファイル、hash、HEADが入る。送信は行わない。秘密に関する拒否規則は補助であり、機密がないことを保証するものではない。

## 一度の長時間commandを記録する

```text
python3 tools/run_logged.py --directory NEW_ABSOLUTE_DIR --cwd ACTUAL_REPO -- VERIFIED_COMMAND ARGUMENTS...
```

未知のoracle flagを補わない。主担当が現地manual/helpから決めたargvを、そのまま渡す。ログはファイル、表示は180秒ごとの短い状態だけ。スクリプト自体は壁時計timeoutを持たない。oracleが内部でdetachした場合、wrapperのexit 0は回答完成ではない。保存されたsessionを180秒以上の間隔で確認して回収する。

スクリプトの`wait(timeout=180)`は、その一回の状態確認を待つ間隔であり、子commandを180秒でkillする機能ではない。環境やoracle service側の制限を解除できるという保証でもない。人が明示的にCtrl-Cした場合は、そのowned processへinterruptを渡す。

## 補助ツールの自己テスト

bundleルートで`python3 validation/test_helpers.py -v`。一時的なローカルGit repoを作る試験はあるが、実プロジェクトや外部remoteを変更しない。oracle/networkへの接続は行わない。
