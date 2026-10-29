# tasks

最終更新: 2026-10-29 19:39 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

Plan250/ADR0043のowner pauseを維持し、別途許可されたW6有限LABも2026-10-29の最新owner指示で一時停止。I3-4/W7/Canon採用は開始しない。Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。単一report2617とW6_CHECKへ証拠を集約する。

## current promoted package

Sources: Canon `mirrorea_canon/adr/ADR-0043.md`、LAB `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`、`plan/proof-first-foundation-correspondence.md`。

W6はowner指示で一時停止中・未完了。Native63全1750/45独立reader/全履歴、workspace他1190/7doctestを受理。固定275formal（41526宣言/1995反例）と別cutの寿命・型補完・再取得・費用を検証。実3world/各2cold reader、registry27組/36費用区間、30process/1116prefixを受理。全90費用式へ復元形状を接続。初回Flow/slot・元source・受信DATA・Bootstrap/Parts/Process/Fabricに続き完全Backendを554certificate宣言/16反例・実16case/2コード改変/9画像で結合。Backend上限N37932/S3925183は両snapshotと四authorityを保持。完全初回画像、共通public初期履歴/cache・全caller保持/物理資源/進行、他profile、最終docs/Oracleは未完了。

現全private実行/全readerまで受理済みなのはNative63。Native21/1678/21readerは旧baselineへ移した。Native59/60/62の失敗を保存。Native63はbuildと独立workspace partitionを受理。現runtime1750と45原本reader/全履歴を受理。残分類はW6_CHECK.implementation.unverified_checks。再開手順は `docs/proof-first/RESUME.md`。

## ordered self-driven packages

現在実行中のpackageはありません。明示的な再開後に限り、下表の依存順で進めます。10/29 ownerへのrough estimateは順調なら残り2〜4日の実作業、資源前提の修正/レビュー差戻しで1週間以上の可能性。Oracle利用制限待ちとquota停止は別の不確定要因。完了日・完成率を保証しない。

| Package / macro position | Direct consumer / exit | Current state |
|---|---|---|
| Current build / Macro3/6後段 | ordinary library/CLI/integration/example | Native63のcfg-only3file、4library構成check通過。14packages/148targets/32examples/5CLIのビルドと独立照合通過。147他target/1190試験・21追加runtime・13doctest/7例を独立受理 |
| Current full regression / Macro3/6後段 | 全runtimeと原本・全workspace | Native63全1750と45原本reader/全履歴を受理。他1190試験と7doctestも受理。別の計測構成は1790inventoryの全回帰を未実行 |
| Resource correspondence / Macro3/6後段 | allF/J/work/setup/host/physical | 復元と初回Flow/slot・元source・受信DATA・Bootstrap/Parts/Process/Fabric/完全Backendまで結合。完全初回画像→public初期履歴/cache/全caller物理資源。再開後の最初の対象 |
| Combined source/proof/history / Macro1/5・3/5後段 | future authority/full response/current/reload | 現33source/23対応/153出力は照合済み。275module/41526宣言/1995反例の両回と全4618ログを受理。寿命/snapshot/scratch3moduleは別途両回検証。現全履歴も45原本readerで受理 |
| Stable docs / Macro3/6後段 | 再現commandと受理証拠 | Backend focused342受理。今回の停止記録full make docs通過（6410入力不変）。追加storage後の文書検証はADDITIONAL-STORAGE-DOCS-RECEIPT-20261029-v1へ保存。W6最終統合fullは再開後 |
| Independent final review / Macro3/6後段 | T01〜T15/全要件/反例/TCB | 10/29 18:34:16JSTもOracle Pro5of5/Usage limit reached。最終review未実施、停止中のpollingなし |
| W7 / Macro3/6後段 | W6受理後の別owner範囲 | inactive、今回範囲外 |

## self-driven macro phase reading

Macro1/5の一般証明とMacro3/5の実Source/kernelを接続し、Macro3/6のrelease/全channel/資源/回帰に未完了項目を残して停止。再開前の技術的な残課題を記録する。初期化DATA、実Source、独立authorityを分ける。current roadmapはPlan250、W6 LABは別途owner許可範囲。

## user decision gates

容量: ownerはtarget等の追加整理を明示し、確認への回答も「target等の追加整理を続ける」。W6研究は再開しない。4553個のLean生成cacheと、終了済み/source-pinnedの部品test binary17本をexact archiveへ保存・全読戻し後に展開copyだけ整理。生成物の正味削減は4.10GiB、確認時のroot空きは約8.2GiB。元proof/source/raw/logとNative63全試験/計測の実行ファイルは保持。 再開後、現/旧formalのimport又はcache hash照合前にcleanup_additional_lean_cache_20261029_v1.py restore --confirm、該当17componentの実行又はbinary hash照合前にcleanup_component_binaries_20261029_v1.py restore --confirm。先行Cargo cacheのcleanup_w6_build_cache_20261029_v1.py restore --confirmも別の必要条件。全て元bytes/mode/mtimeをno-clobberで復元し、source/hash/free-spaceを確認する。

Q26の最終channel/timing/in-flight政策とQ27のdebugger範囲は未採用。可逆LAB候補と反証も現在はowner pause。再開後の技術検証と最終採用判断を分ける。privacy弱化/L0-L1/public contract/production/Canon採用はowner判断。119要件/30Q/18PT/24SC/8alphaの出所・承認・採用区分を保持する。

## research discovery items

| Item | Effect / alternatives / current view |
|---|---|
| Historical currentness | 元publisher/current registryと独立leaseは保存DATAと別。same-owner再読込み/active再参加を検証。元custody全喪失後の一般再開は未選択 |
| Current release | 現案Aはcanonical M9/owner→observer guardをdurable intent/全DATA write中保持。最小代替Bの失効後writeは未採用。kernelにqueued済みの到着とwrite時currentは別 |
| Work and funding | 受動/能動のwire・暗号・journal入力を32query/8reloadへ合成。全M9入力Hと準備出力Jを分け、setup/driverと拒否後host-entryを別計上。実測最大値又は8MiB payloadを全service上限にする代替は不採用 |
| Temporary ownership | zero-retained scratchの逐次maxと全spent加算、initial/escape/並行ownerのframe、置換L+Cを明示。現30processの1116prefixを受理。全caller上限は未受理。phase peak/RSS/public保証を導かない |
| Graph/absence/loss | local closed/lossless/page remainder/distributed typed crossingを分ける。M8直接依存とSource順序/Flow crossingを混同しない。current全1750/全原本readerへ結合済み。最終要件別照合は残る |
| Mutation/refinement | 実7lifecycle/全6publication監査を保持。restricted-child2familiesと現147target受理結果の意味対応照合は残る。全writer/分類/historyを期待JSONだけで受理しない |
| Host and timing | honest host/key/filesystemは前提。scheduler/fsync/EINTR/同一UID隔離/一般availabilityは別の未証明境界。有限LABと本番保証を分ける |

## maintenance tasks

2026-10-29の最新owner指示は、切りのよいところで停止し、研究データを保持して再生成可能なtarget等を一時整理すること。検証済みarchiveへCargo cacheを退避し、研究ソース・証明・実行原本・ログを保持しました。指定Full resetは未使用のまま、停止中の自動使用・quota監視を停止します。以前の自律継続/reset条件はこのpause中は実行しません。W7/Canon/Plan250-I3-4/未依頼の外部write/通知/共有folderは開始しません。追加owner指示による、この停止記録のcommit/pushだけを行います。

## non-promoted references

閉じたW4/W5/Astraと未採用W6 LABを区別する。focused Native/formal supplement/旧last fullはW6全体又はCanon採用ではない。旧失敗と全cut範囲はreport2617と凍結receiptへ保持。

既存Process/Fabric/Bootstrap/Flow割当式、registry27cut/36区間とdormant37入力116区間は再利用する。再実装するqueueへ戻さない。

最終読取は2026-10-29 18:33:01JSTで週次残量60%、指定10/30 Full reset使用0回。18:34:16JSTのOracleはPro5of5/Usage limit reached。停止中は再照会・reset自動使用を行わない。
