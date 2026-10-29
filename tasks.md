# tasks

最終更新: 2026-10-30 08:49 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

Plan250/ADR0043のowner pauseを維持し、別途許可されたW6有限LABもSSD増設/再起動前のowner pause。I3-4/W7/Canon採用は開始しない。Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。単一report2617とW6_CHECKへ証拠を集約する。

## current promoted package

Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。

W6はSSD増設・PC再起動に備えるowner pauseで未完了。検証済み区切りはc3f286d3のOriginalImage caller paths（84宣言/22反例/48log）。Native63全1750/45reader、他1190/7doctest、固定275formalを保持。worker調査はsource解析まででLean草稿未実行。RAM上12384fileを永続archiveへ全照合済み。明示再開まで研究・quota/Oracle確認を停止する。

現全private実行/全readerまで受理済みなのはNative63。Native21/1678/21readerは旧baselineへ移した。Native59/60/62の失敗を保存。Native63はbuildと独立workspace partitionを受理。現runtime1750と45原本reader/全履歴を受理。残分類はW6_CHECK.implementation.unverified_checks。再開手順は `docs/proof-first/RESUME.md`。

## ordered self-driven packages

自走研究は停止。次に自走で進める順番は、明示再開後のstorage確認/必要subset復元、worker保持、共通入力/履歴、物理資源/進行、最終照合。直前の追加3〜7日という見積りは低確度の実作業見込みで、今回の停止・増設待ち時間を含まない。

| Package / macro position | Direct consumer / exit | Current state |
|---|---|---|
| Current build / Macro3/6後段 | ordinary library/CLI/integration/example | Native63のcfg-only3file、4library構成check通過。14packages/148targets/32examples/5CLIのビルドと独立照合通過。147他target/1190試験・21追加runtime・13doctest/7例を独立受理 |
| Current full regression / Macro3/6後段 | 全runtimeと原本・全workspace | Native63全1750と45原本reader/全履歴を受理。他1190試験と7doctestも受理。別の計測構成は1790inventoryの全回帰を未実行 |
| Resource correspondence / Macro3/6後段 | allF/J/work/setup/host/physical | owner pause。caller paths受理まで保持。workerはsource解析/未実行Lean草稿を保存。再開指示後に継続 |
| Combined source/proof/history / Macro1/5・3/5後段 | future authority/full response/current/reload | 現33source/23対応/153出力は照合済み。275module/41526宣言/1995反例の両回と全4618ログを受理。寿命/snapshot/scratch3moduleは別途両回検証。現全履歴も45原本readerで受理 |
| Stable docs / Macro3/6後段 | 再現commandと受理証拠 | c3f286d3のfull6410/focused397は受理済み。今回停止metadataはfocused398の21項目を通過。receipt metadataはfocused399で確認。最終W6受理は未完 |
| Independent final review / Macro3/6後段 | T01〜T15/全要件/反例/TCB | owner pause。08:31JST利用上限を最終観測として保持。待機browser終了、確認/prompt送信停止 |
| W7 / Macro3/6後段 | W6受理後の別owner範囲 | inactive、今回範囲外 |

## self-driven macro phase reading

Macro1/5の一般証明とMacro3/5の実Source/kernelを接続し、Macro3/6のrelease/全channel/資源/回帰に未完了項目を継続する。技術的な残課題と採用判断を分ける。初期化DATA、実Source、独立authorityを分ける。current roadmapはPlan250、W6 LABは別途owner許可範囲。

## user decision gates

容量: 2026-10-30 08:49 JST 再起動で消えるW6 tmpfs全81rootの12384 regular file、17853全entry、論理7417200732byteをw6-volatile.tar.zst（1330292039byte）へ保存。全fileのSHA256/長さ、mode/uid/gid/mtime、hardlink/symlink関係、圧縮EOFを独立読戻しで照合。原本削除なし。archive SHA256=74e89575e2a19cb90a51bb5460c1f823061ef53cd649b4b010da9be8b4d4801d。 SATA SSDの別mountへheavy artifact/cache/logを配置することは可能だが、増設後の実mount/filesystem/空き/権限を確認するまで新pathを前提にしない。NVMeのTBW増加原因は未測定。tmpfsもswapを利用し得るため、再開後に実I/Oとswap配置を点検する。今回はformat/fstab/mount/swap変更なし。

Q26の最終channel/timing/in-flight政策とQ27のdebugger範囲は未採用。可逆LAB候補と反証は継続可能。技術検証と最終採用判断を分ける。privacy弱化/L0-L1/public contract/production/Canon採用はowner判断。119要件/30Q/18PT/24SC/8alphaの出所・承認・採用区分を保持する。

## research discovery items

| Item | Effect / alternatives / current view |
|---|---|
| Historical currentness | 元publisher/current registryと独立leaseは保存DATAと別。same-owner再読込み/active再参加を検証。元custody全喪失後の一般再開は未選択 |
| Current release | 現案Aはcanonical M9/owner→observer guardをdurable intent/全DATA write中保持。最小代替Bの失効後writeは未採用。kernelにqueued済みの到着とwrite時currentは別 |
| Work and funding | 受動/能動のwire・暗号・journal入力を32query/8reloadへ合成。全M9入力Hと準備出力Jを分け、setup/driverと拒否後host-entryを別計上。実測最大値又は8MiB payloadを全service上限にする代替は不採用 |
| Temporary ownership | 呼出し元30scope/73localと計測構成38scope/99localを84宣言/22反例へ対応。新native実行なし。worker/共通入力/物理資源はOPEN |
| Graph/absence/loss | local closed/lossless/page remainder/distributed typed crossingを分ける。M8直接依存とSource順序/Flow crossingを混同しない。current全1750/全原本readerへ結合済み。最終要件別照合は残る |
| Mutation/refinement | 実7lifecycle/全6publication監査を保持。restricted-child2familiesと現147target受理結果の意味対応照合は残る。全writer/分類/historyを期待JSONだけで受理しない |
| Host and timing | honest host/key/filesystemは前提。scheduler/fsync/EINTR/同一UID隔離/一般availabilityは別の未証明境界。有限LABと本番保証を分ける |

## maintenance tasks

最新owner指示はSSD増設・PC分解/再起動前の安全な停止です。W6はowner pauseで、再開には明示指示を要します。増設候補は別ドライブのSATA SSDで、NVMe書込量への懸念も記録しました。再開後にmount/filesystem/空き容量を確認し、heavy artifact/cache/logの配置とswapを点検します。今回はdevice/partition/mount/OS設定を変更せず、cleanupもしません。指定reset使用1回とQUOTA-OWNER-POLICY-v6の35%条件を保持し、pause中はquota/Oracle確認、追加reset・購入を行いません。W7/Canon/Plan250-I3-4も開始しません。

## non-promoted references

閉じたW4/W5/Astraと未採用W6 LABを区別する。focused Native/formal supplement/旧last fullはW6全体又はCanon採用ではない。旧失敗と全cut範囲はreport2617と凍結receiptへ保持。

既存Process/Fabric/Bootstrap/Flow割当式、registry27cut/36区間とdormant37入力116区間は再利用する。再実装するqueueへ戻さない。

最終観測: quota08:11JST残量85%、Oracle08:31JST利用上限。owner pause中は確認しない。指定resetは使用1回、追加使用/購入なし。
