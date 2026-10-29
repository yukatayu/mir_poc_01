# RESUME — W6 resumed; selected cache provenance connected

Updated 2026-10-29 21:54 JST

**Canon notice:** `mirrorea_canon/` is normative. This is a LAB restart snapshot; Report2617 and exact receipts retain history.

W6「秘密を含む観測とdebug」は、2026-10-29 20:00JSTのowner指示で再開しました。研究原本と検証済みarchiveを保持し、容量を確認して必要なcacheだけを復元します。閉じたAstra研究handoffはReport2616、現在の単一実装reportは2617です。

W6は再開・未完了。Native63全1750/45独立reader/全履歴とworkspace他1190/7doctestを受理。固定275formal（41526宣言/1995反例）と別cutの寿命・型補完・再取得・費用を検証。実3world/各2cold reader、registry27組/36費用区間、30process/1116prefixを受理。初回/fresh/V1/V2画像と一般保存cache費用に続き、選択経路のcache由来を新165宣言/18反例で検証し、H≤23449953と既存90式へ接続。共通whole入力・画像全体の履歴・全caller保持/物理資源/進行、他profile、最終docs/Oracleは未完了。

## Current authority

最新owner指示はW6の継続とストレージ残量への注意です。主担当一人で可逆な有限LABを進め、区切りごとのcommit/pushを行います。研究ソース・証明・原本・ログとexact archiveを保持し、必要なcacheだけを復元します。W7/Canon採用/Plan250-I3-4/未依頼の外部write・通知・共有folderは開始しません。quotaはQUOTA-OWNER-POLICY-v6の指定reset条件と使用後35%停止条件を維持します。

Owner messages: 「引き続きW6の作業をお願いします。」「ストレージの空きには適宜注意してくださいね」。W6-OWNER-RESUME-20261029-v1で6807保護fileと保存receiptを再照合済み。研究再開時HEAD=34adc79f88f40a85dcf07a228bb9b9adef05b94d、dirtyなし。先行pause/cleanup記録は履歴として保持する。

## Exact roots and preserved evidence

R=/home/codex/dev/mir_poc_01
B=/home/codex/.local/state/mirrorea-proof-first/w6-20261019-implementation
Research baseline HEAD=42f476538fc17f11506d68ca1b8f7a875d987656, main. This saved snapshot began with 18 dirty paths; the later owner-authorized commit records them together.

Native63: NATIVE63-FULL1750-ACCEPTANCE-v1.json / native-full1750-v1/PIN.json / NATIVE63-FULL1750-READER-QUEUE-RESULT-v1.json。Native63全1750/0failed/0ignoredと45独立readerを受理。全6575source/実binary、7488raw/21plain/53stage、全履歴とreadersの新実行/ログを独立照合し、queue改変10件を拒否。旧prefix成功を流用せず、完了済み全1750を再起動しない。

Workspace: NATIVE63-WORKSPACE-PARTITION-ACCEPTANCE-v1.json。4library構成、14package/148artifact/32example/5CLI、他1190と7doctestを受理。2940unique試験=1750+1190、追加21の単独試験を重複加算しない。Native61の旧全1729/45readerも保持。

Current measurement: NATIVE-REGISTRY-LIVE63-ACCEPTANCE-v1.json。隔離した現計測構成で45allocator校正、1genuine registry control、実3private world/各2cold readerを受理。独立registry45記録、27組の6費用変数、36実測区間、54改変拒否、旧画像に対するfresh registry18拒否を照合。87原本/9画像/69query/240ledger/12charged restart/192ledger改変を確認。30processの558対/1116有効cut、16計測改変と3無効理由保持を検証した。

最大観測prefixは予約Layout263120089byte、累積要求7325990566byte。これは各processの標本であり、terminal peak/RSS/物理予約/共通public容量/進行保証ではない。先行setup/probe/loggingを含み、自身のcut後serializationと直接System/libc/stack/kernel/allocator内部は別。独立6費用変数は今回の完全入力に対する値であり、任意の初期履歴や全callerの共通上限にはしない。計測binaryはNative63全1750と別で、1790inventoryの全回帰は未実行。

Formal: COMBINED-FORMAL-ACCEPTANCE-v2.json。275module/237一般module/38独立readback、41526宣言/1995反例、fresh/replay各2309command/全4618logを受理。追加寿命/snapshot2module299宣言33偽とscoped scratch50宣言14偽は別cutで両回受理。固定275manifestへ黙って加算しない。

Current source/cost: native63-semantic-source-v2で33source/23対応/153出力を照合。owning-feature-composition-v1の90式を現9画像へ適用し、現registry6変数は上記27組へ接続済み。既存Parts/Process/Fabric/Bootstrap/Flow費用を再構築しない。


The selected initial ST Backend has N≤37932/S≤3925183, retaining both snapshots and four authorities. INITIAL-BACKEND-SHAPE-JOIN-v1 records 554 certificate declarations, 16 falsifiers, 16 native capture cases (9 shape-qualified / 7 outside), two compiled source mutations and nine original images. This does not establish arbitrary restored-cache reachability or whole-image/physical funding.

## Cache cleanup and exact restoration

ownerはtarget等の追加整理を明示し、確認への回答も「target等の追加整理を続ける」。このstorage checkpoint時点ではW6を再開していなかった。4553個のLean生成cacheと、終了済み/source-pinnedの部品test binary17本をexact archiveへ保存・全読戻し後に展開copyだけ整理。生成物の正味削減は4.10GiB、確認時のroot空きは約8.2GiB。元proof/source/raw/logとNative63全試験/計測の実行ファイルは保持。

追加整理後の復元条件（初回pauseのCargo cacheとは別）:

```bash
python3 /home/codex/.local/state/mirrorea-proof-first/w6-20261019-implementation/cleanup_additional_lean_cache_20261029_v1.py restore --confirm
python3 /home/codex/.local/state/mirrorea-proof-first/w6-20261019-implementation/cleanup_component_binaries_20261029_v1.py restore --confirm
```

Lean側は必要に応じて `--prefix formal-small-disk-runtime-20261025/` 等で登録済みsubsetを復元できる。未復元pathを読む古いverifierは失敗するので、receiptの上限・前提を変えず原byteを先に復元する。

再開後、現/旧formalのimport又はcache hash照合前にcleanup_additional_lean_cache_20261029_v1.py restore --confirm、該当17componentの実行又はbinary hash照合前にcleanup_component_binaries_20261029_v1.py restore --confirm。先行Cargo cacheのcleanup_w6_build_cache_20261029_v1.py restore --confirmも別の必要条件。全て元bytes/mode/mtimeをno-clobberで復元し、source/hash/free-spaceを確認する。

再生成可能Cargo cacheだけを5177file（計測2811/native63非実行2366）整理。全内容をhash照合済みの2archive（計817469713byte）へ保持し、証拠用実行ファイル281本は元pathに残した。Canon221/閉じた5文書/Native63全6575source/主要6receiptの計6807fileと119要件の承認rowを保持。研究ソース・証明・raw原本・ログは削除しない。

archive作成前からの正味容量改善はroot約1.50GiB/tmpfs約0.57GiB。整理後の確認時点でroot約5.3GiB/tmpfs約2.0GiB空き。並行する他作業により空きは変動する。

Cleanup evidence: pause-cache-cleanup-20261029-v1/AUTHORITY.json, RESOURCE-AUDIT-BEFORE.json, PROTECTED.json, per-root MANIFEST/VERIFIED/RETIRED.json, RETIREMENT.json and FINAL-VERIFICATION.json. Resource audit includes df/free/lsblk/findmnt/du; configured external mount was absent. No shared folder was used.

Owner resume is recorded. Restore only the required dependency subset before dependent builds or cache-path hash checks:

```bash
python3 /home/codex/.local/state/mirrorea-proof-first/w6-20261019-implementation/cleanup_w6_build_cache_20261029_v1.py restore --confirm
```

The command verifies each archive and protected file before restoring, preserves exact bytes/modes/mtime, and will not overwrite an existing different file. It requires enough free space for each selected cache plus a 512MiB margin. Never substitute a new build for archived evidence or rewrite successful receipts. The retained executables remain at their original paths; the archived .rlib/.rmeta/.d files must be restored for direct rustc components. No restoration is needed merely to read the research records.

## Remaining work after selected cache provenance join

1. Re-read current Canon/owner direction and verify preserved receipts/source plus available disk/RAM. Restore only required cache using the command above; keep the current accepted Native63 full1750/45 readers and workspace1190/7 doctests without redundant reruns.
2. Initial whole/fresh/V1/V2 geometry and saved-cache native shape/cost H are conditionally connected. Selected cache provenance is now connected; next establish whole-image reachable histories and common whole-input parameters, then allcaller lifetimes/usable physical funding. Keep independent registry custody and arbitrary-import residuals. Existing restoration shape and all90 cost formulas remain accepted conditional evidence, not a whole caller budget.
3. Connect all callers' ownership lifetimes, initial/returned/concurrent frames, non-allocating work, host/OS, physical funding and progress. Do not derive a public bound from the maxima of the measured three private inputs.
4. Reconcile other profiles, all requirements, source/proof/outward/history correspondence and appropriate measurement-configuration regression (full1790 remains unrun). Complete final integrated docs and independent Oracle review. Keep every missing premise open.

共通public初期履歴/cache、全callerの保持寿命・非割当work・host/OS・物理資源と進行、他profile、全要件/出所/証明の最終照合、計測構成1790の必要回帰、W6最終統合docs/独立Oracle reviewは未完了。

The pause full-docs/focused343 receipts cover this saved state only. They do not close W6 or replace the future final integrated review. Earlier detailed restart notes are preserved in owner-pause-20261029-preimages-v1/docs/proof-first/RESUME.md and Report2617.

## Quota, Oracle and process state

2026-10-29 21:01JSTの週次残量51%、指定10/30 Full reset使用0回。次回確認は22:01JST以降。21:07JSTのOracleはPro5of5/Usage limit reached。指定resetの期限接近/枯渇条件と使用後35%停止はQUOTA-OWNER-POLICY-v6のまま。

W6-OWNER-RESUME-20261029-v1 and QUOTA-OWNER-POLICY-v6 supersede pause scheduling. No old Native63 full regression or reader service is restarted. Reuse accepted receipts and restore only the caches required for the next bounded check.

## Repository backup scope

追加owner指示「あと、切りの良いところでcommit&pushも適宜お願いします（研究データが消えると悲しいので）」により、区切りごとのrepository変更のcommit/pushを許可。最新指示によりW6研究は再開した。Git管理外の研究ソース/証明/raw原本/大容量ログ/cache archiveはローカル保持で、今回のGitHub pushに含めない。

停止記録のfull make docsは6410入力不変・exit0で通過。以後のcommit/push許可・検証結果の文書差分はfocused343で確認し、W6最終統合full/Oracleは未完了のまま保持する。

2026-10-29 19:03 JST: ownerが依頼した蓄積済み18文書/台帳をcommit 4259bb9e038bef2744d78cc3191feb873fc8edf3としてorigin/mainへpushし、git ls-remoteで同じcommitを確認。Git管理外の研究ソース・証明・raw/log・cache archiveはローカル保持でpush対象外。この追記は確認結果だけを保存し、semantic作業は再開しない。

Additional disk analysis (2026-10-29 19:39 JST): 調査開始時、root使用約173.1GiB/一般利用者の空き約5.4GiB。Mirrorea研究作業85.3GiB（W6 63.2/W4等22.1）、Codex関連19.4GiB（session記録16.8）、swap.img16GiB。W6の圧縮済み保存物23.0GiB、JSON記録9.0GiB、保存image9.3GiB等が大きい。四世代のfull native実行記録は合計16.4GiBで前記分類の内数。通常のtargetディレクトリはroot上約0.14GiB。tmpfs約5.9GiBはRAM/swap側でroot使用量に加算しない。 swapは調査開始時約7.6GiB使用、整理中に約9GiBへ変動したが変更なし。sudoによるroot側全量auditはpasswordが必要で実行不可、読取可能範囲のduとstatvfsを区別する。rootの予約領域約8.9GiBはdfの一般利用者空きに含まれない。研究原本・旧成功/失敗・会話記録・他project・OS設定は削除しない。任意の全artifactを再生成可能と認定したものではない。

## Initial whole image evidence

2026-10-29 20:30 JST: OriginalImageの全6fieldを、初回Parts/Bootstrap/Flow/Process/Fabric/完全Backendへ同じ原本上で合成。2つのBackend snapshot、四authority、元source/owner plan、返却履歴を保持した。

新82宣言と16反例をfresh/replayで検証し、既存2915宣言を再確認。56新logと271source入力を独立照合。9実画像・24実返却を完全に読み、source12/reader32/数値6改変を拒否。root N≤1115075/S≤228393048。

各componentのsource事前条件と固定public初期materialが前提。NはJSON構文node/鍵、SはUTF-8文字列byteで、encoded長・heap/RSS・物理予約ではない。上限は後段encoder capを超え、capture成功からencode成功や失敗前費用の免除を導かない。任意import履歴/cacheはsymbolic residualを残し、到達可能性と共通public上限は未証明。Native変更・新実機実行はない。全caller寿命/work/host/物理資源/進行、他profile/最終統合reviewも未完。

Leanの予約binder名とunused simp、凍結Parts generator参照、AST inventoryの実stdout、同名version fieldの改変対象を訂正。旧draft/失敗を保持し、成功条件を弱めない。

再現: check_initial_whole_formal_v3.py（fresh/replay-v2）→ verify_initial_whole_v3.py → INITIAL-WHOLE-IMAGE-JOIN-v1.json。成功出力はno-clobber、再現時は新cut名を使う。

## Prior image geometry evidence

2026-10-29 21:06 JST: 既存V1のinterned展開とblob参照ごとの容量課金、ゼロ長root、16KiB chunkをV2の論理8MiB制限と分離。型補完と初回画像の上限へ合成し、同時保持は和、復元前後は既存recapture関係を要求した。

新104宣言/16反例をfresh/replay・78logで照合。実codec24ケース（21成功/3拒否）と9原本画像を独立Pythonの完全wire/logical hash・形状へ一致。source10/output24/数値20改変と、実compileした課金省略・payload欠落の2改変を拒否。

EntryのA/F/N≤478150656、S≤8598323200は閉じた型補完・初回source等の前提付き。V1の8MiB logical仮定は不可。保存cacheのnative形状/費用H、任意mutable履歴、全caller寿命/work/host/物理資源/進行は未受理。codec部品実行でありwhole typed restore/network/current authorityの新実行ではない。有限上限を物理予約又は使える容量へ読み替えない。

再現: check_prior_image_formal_v2.py → run_prior_image_codec_v1.py → run_prior_image_mutants_v1.py → verify_prior_image_v1.py → PRIOR-IMAGE-SHAPE-JOIN-v1.json。成功cutはno-clobber、再現時は新cut名を使う。

2026-10-29 21:13 JST: prior-image cutのfull make docsは6410入力不変・exit0、focused353は21項目成功。metadata追記はfocused354で別検証する。Oracleは21:07も利用上限で最終review未実施。

## Saved cache cost evidence

2026-10-29 21:31 JST: 保存cacheを実行用instance/独立registryと分け、全59型constructor/36named/72variantを現3module・14import/aliasへ照合。省略Noneと透明Option/Boxを含むnative形状から費用Hを導き、既存6registry項と90費用式の残項を保持して代入した。

新45宣言/16反例（100log）と90数値式（70log）をfresh/replayで照合。45実registryのnative clone測定、9保存画像、7追加DATA例、source25/reader6/数値10/合成12改変拒否を受理。27registry組・36実測区間を再利用し、新Native全体実行はない。

H=225726309597184は約205TiBの保守的な累積clone要求上限であり、使用可能な物理予約ではない。閉じた型/初回又はguarded入力条件、独立current registryを要する。任意mutable履歴の到達可能性、共通whole入力の全費用、全caller保持寿命/work/host/OS/物理資源・進行は未受理。既存cacheと同じことを一般の保存cacheへ仮定せず、spent/historyを残す。

Lean namespace不足/冗長dsimpと、readerの改変用nested型構築の構文誤りはsuccessorで訂正。90式の最初のrunnerは16MiB生成物上限で停止し、失敗を保持。事前登録済み24MiB上限のrunnerを選び直し、同一proof/inputの新cutで両回成功。source/type/import対応と意味条件は弱めない。

再現: check_saved_cache_formal_v3.py → verify_saved_cache_v2.py → compose_saved_cache_v1.py → check_saved_cache_readback_v2.py → qualify_saved_cache_v2.py → SAVED-CACHE-PUBLIC-JOIN-v2.json。成功cutはno-clobber、再現時は新cut名を使う。

Repository backup: prior-imageの13文書を458a8c86910363e9962cc0cf3cffef704dc2eb3cへcommit/pushし、ls-remote一致とcleanを確認（PRIOR-IMAGE-PUSH-v1）。研究ソース/proof/raw/log/cache archiveはGit外のローカル保持。

2026-10-29 21:38 JST: saved-cache cutのfull make docsは6410入力不変・exit0、focused355は21項目成功。検証記録追記はfocused356で別検証。W6最終統合full/Oracleは未完了。

## Selected reachable cache evidence

2026-10-29 21:54 JST: 初期の公開Source4/役割A-S-Tからのcache由来を、通常のprivate状態変更・権限更新・保存・保存値のcold復元を含む一般遷移へ接続。saved cacheと実行用instanceを別々に保持し、復元でcurrent registryに置換しない。

新165宣言/18反例をfresh/replayで受理し全66logを独立照合。現9method/既存20public構築method、admitted更新経路、9画像/45独立registryを確認。source13/DATA27/結合14改変を拒否。既存90式/30logと27registry組/36実測区間を再利用し、新数値命題として二重加算しない。

選択経路でH≤23449953byte（約22.4MiB）の累積cache clone要求上限を得た。任意import/patch/未対応callerには適用せず、一般保存cacheの約205TiB条件付き上限を保持する。Leanのsaved履歴/paidは論理的な履歴・累積会計であり、全cacheの実メモリ保持や新runtime ledgerの実装を主張しない。共通whole入力・画像全体の到達可能履歴・全caller保持寿命/work/host/OS/物理資源・進行は未完了。

Leanのrecord field改行位置とDecidable、indexed constructorの余分なcase binder、readerのdecoder import先を訂正。失敗source/cutを保持し、遷移関係・成功条件・対象profileは弱めない。

再現: prepare_reachable_cache_v1.py → check_reachable_cache_formal_v3.py → prepare_reachable_cache_source_v1.py → verify_reachable_cache_source_v2.py → qualify_reachable_cache_v1.py → REACHABLE-CACHE-JOIN-v1.json。成功cutはno-clobber、再現時は新cut名を使う。

Repository backup: saved-cacheの13文書を15eb93a2a031939a7a0f3a10404a0a1616b33d31へcommit/pushし、ls-remote一致とcleanを確認（SAVED-CACHE-PUSH-v1）。Git外研究原本/proof/raw/log/archiveはローカル保持。

2026-10-29 21:59 JST: reachable-cache cutのfull make docsは6410入力不変・exit0、focused357は21項目成功。検証記録追記はfocused358で別検証。W6最終統合full/Oracleは未完了。
