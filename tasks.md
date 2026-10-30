# tasks

最終更新: 2026-10-30 15:30 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

Plan250/ADR0043のowner pauseを維持し、別途許可されたW6有限LABはowner再開指示により継続中。I3-4/W7/Canon採用は開始しない。Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。単一report2617とW6_CHECKへ証拠を集約する。

## current promoted package

Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。

W6継続・未完了。workerの7source/69関数/806localを両構成で照合し、新Lean262宣言/22反例・48log、source14/reader5/結合18改変拒否とstd部品8例×2回を受理。join後の返却、TLS退避、detach中の保持を区別。poll回数の共通上限は未成立。Native63全1750/45reader・他1190/7doctest・固定275formalを保持。共通入力/履歴・全caller費用・物理資源/進行・最終照合は未完。

現全private実行/全readerまで受理済みなのはNative63。Native21/1678/21readerは旧baselineへ移した。Native59/60/62の失敗を保存。Native63はbuildと独立workspace partitionを受理。現runtime1750と45原本reader/全履歴を受理。残分類はW6_CHECK.implementation.unverified_checks。再開手順は `docs/proof-first/RESUME.md`。

## ordered self-driven packages

workerの所有移管/並行保持は条件付きに接続済み。次はpolling driverの共通work条件と最小代替の検証、各caller/共通public入力・履歴の費用、物理資源/進行、最終照合。既存3〜7日の見積りは低確度で、待機/停止時間を含まない。

| Package / macro position | Direct consumer / exit | Current state |
|---|---|---|
| Current build / Macro3/6後段 | ordinary library/CLI/integration/example | Native63のcfg-only3file、4library構成check通過。14packages/148targets/32examples/5CLIのビルドと独立照合通過。147他target/1190試験・21追加runtime・13doctest/7例を独立受理 |
| Current full regression / Macro3/6後段 | 全runtimeと原本・全workspace | Native63全1750と45原本reader/全履歴を受理。他1190試験と7doctestも受理。別の計測構成は1790inventoryの全回帰を未実行 |
| Resource correspondence / Macro3/6後段 | allF/J/work/setup/host/physical | worker69関数/806localと262宣言/22反例を条件付き対応。poll共通上限と各caller/入力/履歴の数値費用、物理資源・進行はOPEN |
| Combined source/proof/history / Macro1/5・3/5後段 | future authority/full response/current/reload | 現33source/23対応/153出力は照合済み。275module/41526宣言/1995反例の両回と全4618ログを受理。寿命/snapshot/scratch3moduleは別途両回検証。現全履歴も45原本readerで受理 |
| Stable docs / Macro3/6後段 | 再現commandと受理証拠 | worker所有関係対応のfull6410入力不変・exit0。focused402通過、metadataはfocused403で別検証しcommit/pushへ。W6最終統合full/Oracleは残る |
| Independent final review / Macro3/6後段 | T01〜T15/全要件/反例/TCB | 10/30 15:27JSTもGPT-6 Pro利用上限。prompt未送信。次回16:27JST以降 |
| W7 / Macro3/6後段 | W6受理後の別owner範囲 | inactive、今回範囲外 |

## self-driven macro phase reading

Macro1/5の一般証明とMacro3/5の実Source/kernelを接続し、Macro3/6のrelease/全channel/資源/回帰に未完了項目を継続する。技術的な残課題と採用判断を分ける。初期化DATA、実Source、独立authorityを分ける。current roadmapはPlan250、W6 LABは別途owner許可範囲。

## user decision gates

容量: 2026-10-30 15:02 JST 移行開始時からroot空きは56794910720byte増え、60476727296byte（約60.5GB）。SSD空き203691089920byte（約203.7GB）。並行する他作業で空きは変動する。 58個/19385573byteのimport cacheをRAMへhash一致で配置。process cgroup MemoryMax=6GiB/MemorySwapMax=0の実propertyを確認。RAM hot領域2GiB上限と合わせ約8GiBの保守的運用枠とし、必要分だけ使う。RAMは唯一の証拠保存先にしない。全OSのswap設定は変更しない。

Q26の最終channel/timing/in-flight政策とQ27のdebugger範囲は未採用。可逆LAB候補と反証は継続可能。技術検証と最終採用判断を分ける。privacy弱化/L0-L1/public contract/production/Canon採用はowner判断。119要件/30Q/18PT/24SC/8alphaの出所・承認・採用区分を保持する。

## research discovery items

| Item | Effect / alternatives / current view |
|---|---|
| Historical currentness | 元publisher/current registryと独立leaseは保存DATAと別。same-owner再読込み/active再参加を検証。元custody全喪失後の一般再開は未選択 |
| Current release | 現案Aはcanonical M9/owner→observer guardをdurable intent/全DATA write中保持。最小代替Bの失効後writeは未採用。kernelにqueued済みの到着とwrite時currentは別 |
| Work and funding | 受動/能動のwire・暗号・journal入力を32query/8reloadへ合成。全M9入力Hと準備出力Jを分け、setup/driverと拒否後host-entryを別計上。実測最大値又は8MiB payloadを全service上限にする代替は不採用 |
| Temporary ownership | worker7source/69関数を条件付き対応。返却/TLS/detach/共有backingを保持。全caller数値費用と物理資源はOPEN |
| Graph/absence/loss | local closed/lossless/page remainder/distributed typed crossingを分ける。M8直接依存とSource順序/Flow crossingを混同しない。current全1750/全原本readerへ結合済み。最終要件別照合は残る |
| Mutation/refinement | 実7lifecycle/全6publication監査を保持。restricted-child2familiesと現147target受理結果の意味対応照合は残る。全writer/分類/historyを期待JSONだけで受理しない |
| Host and timing | honest host/key/filesystemは前提。scheduler/fsync/EINTR/同一UID隔離/一般availabilityは別の未証明境界。有限LABと本番保証を分ける |

## maintenance tasks

最新owner指示はroot空きを増やし、頻用データを合計約8GB以内のRAM運用へ置き、その後W6を継続することです。SSDへ全照合済みの旧重複copyだけを整理しました。研究原本/成功失敗履歴を保持し、主担当一人で可逆LABを進めます。永続成果はSSD、頻用cache/小規模一時生成はRAM、検証process上限6GiBとRAM領域2GiBを併用します。指定resetは使用済み1回、QUOTA-OWNER-POLICY-v6の35%停止を維持し、追加reset/購入は行いません。W7/Canon/Plan250-I3-4/未依頼の通知・共有folder配置は開始しません。

## non-promoted references

閉じたW4/W5/Astraと未採用W6 LABを区別する。focused Native/formal supplement/旧last fullはW6全体又はCanon採用ではない。旧失敗と全cut範囲はreport2617と凍結receiptへ保持。

既存Process/Fabric/Bootstrap/Flow割当式、registry27cut/36区間とdormant37入力116区間は再利用する。再実装するqueueへ戻さない。

最終観測: quota14:21JST残量80%、次回15:21JST以降。Oracle14:27JST利用上限、次回15:27JST以降。指定reset使用1回、35%停止、追加使用/購入なし。
