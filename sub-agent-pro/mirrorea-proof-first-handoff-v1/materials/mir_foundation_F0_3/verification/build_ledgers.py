"""Build explicit research ledgers from the supplied master requirements.
The output NEVER changes the master acceptance status or the repository ledger.
"""
from pathlib import Path
import json,re,shutil,hashlib
ROOT=Path(__file__).resolve().parents[1]
BASE=ROOT/'provenance/requirements_v1.json'
if not BASE.exists():
    src=Path('/mnt/data/input_baselines/mirrorea_system_map_v1/requirements.json')
    shutil.copy2(src,BASE)
master=json.loads(BASE.read_text())
proof=(ROOT/'theory/PROOFS.md').read_text()
sections=re.split(r'^### (F3-\d{2}) (.+)\n',proof,flags=re.M)
scopes={
1:('任意の有限な正のsupport式と有限record集合','構造帰納法、有限集合の増大','任意のsource述語や否定を含む条件の完全性ではない。'),
2:('正のsupportの帰納的な有限導出','反復段階と導出木に関する二方向の帰納法','共同循環をcoinductiveな存在根拠として許す方式は選ばない。'),
3:('独立rank/closure証拠検査','rankの強い帰納法、最小不動点包含','任意の理論pluginの証拠検査ではない。'),
4:('同じ宣言でeligible集合を減らす退役','反復包含とcursor走査','新incarnationや宣言拡張は別の遷移。'),
5:('下方閉label範囲と局所eligibility','反復ごとの制限系の一致','high制御でlowのpolicy/identityを変更する場合は対象外。'),
6:('有限claim集合、名前付きleaf、非空all/any','policy構造帰納法','暗号認証・一般の委譲・policy transformerではない。'),
7:('正確なκと現在policy/issuer/claim','構造的同一性と再検査','過去の数学的命題を失効で偽とする意味ではない。'),
8:('F0.2の多項式証拠と今回のsource adapter','条件付きの証拠再検査','F0.2 checker健全性は引継ぎ前提。任意の証明言語ではない。'),
9:('同owner、同schema、全mutatorに同じ不変条件','有限実行長の帰納法','多owner不変条件・一般的な資源移行は未証明。'),
10:('今回の宣言追加・退役・reparent・policy変更・owner遷移','各遷移caseと正規化の保存','実際の分散制御プロトコルの線形化は別。'),
11:('任意有限候補列、live access、同lineage cursor','走査の停止と退役closure','最終候補の到達性や権限が必ずあるとはしない。'),
12:('有限整数affine関係DAGと型付きsample','トポロジカル帰納法、純粋評価','sampleの物理的真実性・暗号provenance・実時計同期は前提外。'),
13:('保持された同request IDとPendingからの一回遷移','statusの単調遷移','外部効果exactly-once・global deliveryではない。'),
14:('live-result profileの現在member/module/依存/権限','consume時の再検査','snapshot値を独立契約で保持する別profileは未実装。'),
15:('停止し決定的なvalidator、完全なtracked lookup','検査実行列の帰納法','未追跡query・clock・外部I/Oを参照するvalidatorには適用しない。'),
16:('発行済みdelta封印、全read/write stamp、現在権限','再生補題と原子的照合','抽象制御の原子性を実現する分散adapterは別義務。'),
17:('同kind DAGの全枝DFS、tracked revision','新cycleは変更辺を含むことからの背理法','全cross-kindグラフをDAGにはしない。'),
18:('完全なfootprintが独立な二変更','読取り保存と可換な別key書込み','domain recordの可換性でありaudit順序の一致ではない。'),
19:('完全manifest・完全journal・独立に信頼された現head','commit prefix長の帰納法','in-memory image model。物理storage・オフラインcurrentnessは未証明。'),
20:('観測stateがdomain実行の入力にならない遷移系','有限traceの消去と挿入','物理ゼロコスト・任意scheduler・能動debugは対象外。'),
21:('固定label・同じlow制御・同じlow scheduleの二実行','low step一致、高step stutterの帰納法','任意の動的declassification、high管理、time/covert channelは未証明。'),
22:('許可filter後に有限last-kを保持','列の同一性と具体的反例','全履歴無制限保存や本番負荷の保証ではない。'),
23:('選択したF0.3各遷移の任意有限合成','各caseの保存と有限実行帰納法','全Mir機能・無限資源・一般的な進行性は出ない。'),
24:('具体と抽象のstep対応が別に成立する場合','有限traceの連結','Rust/QUICの対応前提は未解消。機密性と進行性は追加義務。')}
files={
1:['model/support.py'],2:['model/support.py'],3:['model/support.py'],4:['model/support.py','model/kernel.py'],5:['model/support.py'],
6:['model/policy.py'],7:['model/policy.py','model/kernel.py'],8:['base/certificates.py','model/kernel.py'],9:['model/kernel.py'],10:['model/kernel.py'],
11:['model/kernel.py'],12:['model/kernel.py'],13:['model/kernel.py'],14:['model/kernel.py'],15:['model/transactions.py'],16:['model/transactions.py','model/kernel.py'],
17:['model/kernel.py'],18:['model/transactions.py'],19:['model/kernel.py'],20:['model/kernel.py'],21:['model/kernel.py'],22:['model/kernel.py'],23:['model/kernel.py'],24:['WORKPLAN.md']}
rows=[]
for i in range(1,len(sections),3):
    ident,title,body=sections[i:i+3];n=int(ident[-2:]);scope,method,limit=scopes[n]
    rows.append({'id':ident,'title':title,'scope':scope,'proof_method':method,'limitations':limit,
        'proof_status':'hand-proof-not-independently-reviewed' if n!=24 else 'conditional-hand-proof-realization-premise-open',
        'machine_checked_general_theorem':False,'proof_file':'theory/PROOFS.md','related_files':files[n],
        'executable_evidence':'tests/test_foundation.py; evidence/RUN_MANIFEST.json' if n!=24 else 'none for production refinement',
        'repository_THM_OBL_discharged':False,'claim_class':'selected-profile research result'})
assert len(rows)==24
(ROOT/'theory/THEOREM_LEDGER.json').write_text(json.dumps({'version':'F0.3','theorems':rows,'scope_notice':'No row changes the Mirrorea Canon proof ledger. Tests and finite exploration do not mechanize these general hand proofs.'},ensure_ascii=False,indent=2)+'\n')
md=['# F0.3 命題・証拠台帳','', '24項目は、量化範囲と前提を持つ手証明である。機械検証・独立レビュー済みとはしない。最後の接続命題は、実装がstep対応を満たすことを未解消の前提として持つ。','', '| ID | 命題 | 対象範囲 | 残る境界 |','|---|---|---|---|']
for r in rows:md.append(f"| {r['id']} | {r['title']} | {r['scope']} | {r['limitations']} |")
md+=['','一般証明の本文は `PROOFS.md`、実行した証拠は `../VALIDATION.md` にある。F0.2から取り込んだcompilerとcertificate checkerは無変更であり、それらの証明をF0.3の新しい証明件数へ重複計上しない。']
(ROOT/'theory/THEOREM_LEDGER.md').write_text('\n'.join(md)+'\n')

# Requirement-specific evidence, not whole-goal completion.
# Class H = hand proof + implementation experiment in an exact restricted profile;
# B = inherited baseline/design connection; O = design/residual only.
G={
'AX':('B','原則を維持し、domain名を含まないsource・構成・現在性の境界を定義した。','最終利用領域の十分性・ownerによる採用は未判定。','W1 W7 W8',[]),
'SL':('B','F0.2のsource/compilerを無変更で使い、現在性と証拠のadapterへ接続した。','新しい型表現力・診断完全性・public grammarは追加していない。','W2',[]),
'ID':('H','存在/member/module/accessを同一のpositive support closureと世代へ接続した。','第一級resource handleと分散した生成・撤去の実装対応が残る。','W1 W2 W3',[1,2,3,4,10,11]),
'RL':('H','整数affine関係DAG、認可access、fallback、読取り専用のprojectionを接続した。','物理sampleの保証・designated evaluation・一般の時間/近似は未統合。','W3 W6',[11,12]),
'DS':('B','source-generated requestとowner局所意味を現在認可・復元へ接続した。','実ネットワーク・弱いメモリ・全種類の並行性と進行性は未証明。','W4',[13,14,24]),
'EF':('O','効果ful computationと外界結果を分ける実装境界を計画に置いた。','外部providerはF0.3 Kernelに未搭載。一般handler・補償は研究が残る。','W2 W4 W5',[]),
'AU':('H','ideal issued claimによる非空all/any、同一κ、失効後の使用再検査を実装した。','暗号認証・任意ポリシー変換・委譲・独立信頼domainの一般化は未完。','W1 W3 W8',[6,7]),
'TY':('B','sourceに束縛した多項式証拠、support rank証拠、label判断を別々の線として照合した。','完全な依存型・様相型・任意理論loaderは存在しない。','W1 W2',[3,7,8]),
'CT':('H','Partial/TotalBodyと全mutator不変条件を現在のコードに結びつけた。','一般の契約論理・多owner rely/guarantee・契約変更合意は未完。','W2 W3',[8,9]),
'VF':('H','24の対象限定手証明、独立rank checker、一般化と有限実行の区別を用意した。','証明支援系の機械化、別の研究者のレビュー、実Rust対応は未完。','W1 W4',[1,2,3,23,24]),
'EV':('H','動的module/source追加・退役・reparentと現在認可・pending使用を同じモデルに置いた。','実多owner activation、状態schema migration、一般patch言語は未完。','W3 W4 W5',[10,15,16,17,18,23]),
'SV':('H','全Data manifestと完全journal、信頼した現在headを使うsame-instance復元を置いた。','実storage・未知成分・外部効果状態・distributed durabilityは対象外。','W5',[19]),
'OB':('H','意味記録、観測消去、固定labelの二実行条件、filter後の保持を接続した。','任意動的IFC・物理timing・資源隔離・能動debugは未証明。','W6',[5,20,21,22]),
'HS':('O','Mir意味・View・provider・sandboxを区別する境界を計画へ具体化した。','Browser/headless参加製品、renderer、safe package実装はない。','W7',[]),
'OP':('O','資源を意味上の失敗と前提に分け、再現可能な研究実行を用意した。','本番quota・長期GC・性能基準・OS故障モデルは未実装。','W4 W5 W6 W7',[]),
'WW':('O','単一の抽象制御モデルを世界全体の中央管理要求にしないことを明記した。','独立domainの接続、リンク/公開/発見/連合は設計と実証が残る。','W8',[]),
'MG':('B','全要件と候補・証拠・残件を対応づけ、既存Canonの受理へ昇格させない。','全体alphaの達成とowner採用は未判定。','W1 W7',[])
}
X={
'SL-02':('B','F0.2のowner-local snapshotという候補を継承した。','読取り整合性を最終既定に採用する決定はしていない。','W2',[8]),
'SL-03':('H','通常sourceから生成したRMWを、現在のmember/module/policy照合後に実行する。','実際のper-locus binary/QUICへのrefinementが残る。','W4',[9,13,14]),
'SL-05':('H','要求IDや認可証拠は内部で生成する。sourceにsend/receiveを追加していない。','構成の生成・認可指定の最終source表記は未選択。','W2 W3',[6,7,13]),
'SL-06':('O','有限typed diagnosticを返すが、ソースに戻る完全な診断体系は構築していない。','拒否理由の機密性とユーザ修正可能性を含めて統合する。','W2 W6',[]),
'SL-07':('B','数学的Int/Boolのprofileを継承し、κのTrueと1を区別した。','Int64 overflow、float、文字列、比較意味は採用前に定義が必要。','W2',[7]),
'SL-08':('O','第一階compilerの一部を利用している。','closure、affine handle、module instanceを値として渡す型体系は未実装。','W2',[]),
'ID-03':('H','tracked reparent・現在認可・同kind DAG検査・退役closureを実装した。','全種類の資源撤去と分散commitは別の義務。','W3',[4,10,16,17]),
'ID-04':('H','短命anchorへのauthorized access失効とfallbackを同じclosureへ接続した。','参照型の一般的な安全性と最終候補の可用性は未保証。','W2 W3',[11,12]),
'ID-05':('H','新member/anchor/accessを作るだけではcursorを戻さず、現在認可付きreacquireで新lineageを作る。','自動再取得の宣言的source表記は未選択。','W3',[7,11]),
'ID-06':('H','過去の成功と現在のlive結果使用を区別し、退役moduleへのconsumeを拒否する。','永続引用や独立snapshot値のfirst-class contractは未実装。','W2 W8',[14]),
'ID-07':('O','F0.1の所有token例は資産として保持するが、F0.3へ統合していない。','線形資源と通常値の受渡し・型消去の保証が必要。','W2',[]),
'ID-08':('H','存在/member/module/access/関係/要求/prepared patchを同じDataに置き、相互失効を実行した。','全種類のpatch DAGと実分散activation・型handleを含む統合は残る。','W1 W3',[2,4,10,11,14,19,23]),
'RL-04':('O','designated evaluatorは既存repo/F0.1の資産で、F0.3に追加統合していない。','不変結果、frontier、consumptionを現行support/contextへ接続する。','W3 W4',[]),
'RL-05':('O','一致したpresentation contextをsample受入れの前提にした。','clock精度・deadline・lease・高頻度streamの時間モデルは未統合。','W3 W4',[]),
'RL-07':('O','整数の正確なrelation評価のみを選んだ。','誤差境界・補間・approximation contractは別の理論が必要。','W3 W7',[]),
'DS-01':('H','F0.2 compilerが作る2本のrequest/outcome edgeを実行し、手書きrouteで意味を補っていない。','locus配置の実プロセス化は既存I3系と対応させる必要がある。','W4',[13,24]),
'DS-02':('B','異なる操作順と意味記録を検査し、実stream順を意味の根拠にしていない。','一般のhappens-before graphと実memory order対応は未証明。','W4',[24]),
'DS-03':('H','Pending/Served/Rejected/Consumed/ReleaseDeniedを保持し、live消費前提を再検査する。','外部effectや任意continuationへの一般化は残る。','W4 W5',[7,13,14,19]),
'DS-04':('H','保持requestの重複serveを拒否し、復元で消費済みを戻さない。','永続的なGC後の古い要求拒否、再試行方式の選択は未完。','W5',[13,19]),
'DS-05':('B','first-order call/returnとrequest待ちを動かす。','F0.2にあるspawn/joinはF0.3 VM未対応で明示拒否。一般cancelも未統合。','W2 W4',[]),
'DS-06':('O','有限prefix安全性と、拒否・待機を区別している。','全公平実行の進行、永久故障時の可用性は証明していない。','W4',[23,24]),
'DS-07':('O','同owner不変条件のみに限定することを明示した。','複数ownerのrely/guarantee・資源移送・共同確定が必要。','W2 W4',[]),
'DS-08':('O','研究用driveの明示step budgetはある。','本番queue/CPU/通信/履歴の資源拘束を保証するものではない。','W4 W7',[]),
'AU-03':('H','member/locus/moduleの帰納的な参加根拠とrootなし循環を区別する。','各外部domainの信頼rootと発行policyを実認証へ対応させる必要がある。','W3 W8',[1,2,3,6]),
'AU-05':('H','policy successor後に古い証拠を拒否し、準備済みreparentも使用時再検査する。','一般policy transformerの透明性・互換性を自動判定する体系ではない。','W3',[6,7,16]),
'AU-06':('H','非空all/anyの有限policyについて健全性・相対完全性の手証明と実装を用意した。','任意authmoduleの形式検証、暗号の安全性、sandboxは対象外。','W1 W8',[6,7]),
'AU-07':('O','claimのscope照合はある。','再委譲・権限の減衰・委譲cycle・別domainとの合成は未モデル化。','W8',[]),
'AU-08':('B','issuer epochの退役をideal recordとして扱う。','鍵移行・root交代・cryptographic freshnessは未実装。','W8',[7]),
'TY-02':('B','局所算術証拠とsupport証拠の別checkerを共通対象へ結びつけた。','moduleごとの任意理論loaderのメタ言語は作っていない。','W2',[3,8]),
'TY-04':('H','不変値を伴う契約とsource identityを共有し、証明を権限へ変換しない。','異種logicの一般翻訳とaffine資源boundaryは未証明。','W2',[7,8,9]),
'TY-05':('B','多項式refinementの証明をF0.2から継承する。','依存型そのものや一般的な型レベル計算を導入したとはしない。','W2',[8]),
'TY-06':('B','F0.1の環境安定性の様相例を参照する。','MTTや一般の様相型checkerはF0.3未搭載。','W2',[]),
'TY-07':('H','code/contract/member/instance/request/argsとpolicy版を照合し、過去の証拠と現在使用を分けた。','任意理論版変更の証明移送は今後の義務。','W1 W2',[7,8,14]),
'TY-08':('O','checkerの責任とkernelからの分離を記述する。','CPU/memory sandbox付き第三者checker実行は未実装。','W7',[]),
'CT-05':('H','同schema/sig交換でTotalBodyをPartialへ暗黙弱化しない。','表現全体のbehavioral subtyping、外部client同意付き契約変更は未完。','W2 W3',[8,9]),
'VF-02':('H','正のsupportと有限policyに健全性・完全性、rankの独立checkerを与えた。','一般Mir表現の十分性と型付け完全性にはF0.2以上の成果を主張しない。','W1 W2',[1,2,3,6]),
'VF-05':('O','step対応で有限traceを移す条件付き命題を明示した。','Rust/QUICがその前提を満たす証明はない。','W4',[24]),
'VF-06':('H','実際の複数の研究モデル不具合を修正し、6つの意図的弱化で反例を再現した。','既存Mirコードの不具合数や第三者レビュー成果には数えない。','W1 W4',[]),
'VF-07':('O','独立レビューを受入れ条件へ置いた。','今回の手証明と実装を別の研究者がレビューした事実はない。','W1',[]),
'EV-02':('H','module-kind DAGと動的supportを参照に用いる。','package版の解決、任意module handle、分散patch依存の全lifecycleは未完。','W3',[10,17]),
'EV-03':('H','宣言とsource追加・退役・reparent・同schemaコード交換を実装した。','一般schema migrationはF0.2の別モデルから未統合。','W3 W5',[9,10,16,17]),
'EV-06':('B','同schema code交換は選択したquiescent条件で行う。reparentはtracked current-use。','分散fenceと処理中継続のgeneral migrationは未実装。','W3 W4',[]),
'EV-07':('B','F0.2の2PCモデルは別資産として保持する。','F0.3 strict-currentnessを2PC/保存へ統合した保証はない。','W4 W5',[]),
'EV-08':('B','無断のTotalBody弱化を拒否する。','正当な契約変更、旧version保持、利用者合意は新しい義務。','W3 W5',[8]),
'EV-09':('O','退役recordは保持する。','履歴/claim/tombstoneを安全に忘れる長期GCは未実装。','W5 W8',[]),
'SV-01':('B','F0.1の有限consistent-cut/Z-pathアルゴリズムを保持する。','分散全成分から入力を完全に抽出する証明は未統合。','W5',[]),
'SV-04':('H','same-instance復元を現在の独立headに束縛する。F0.2のfresh importとは区別した。','そのheadが得られないoffline復旧を同じ保証にしない。','W5',[19]),
'SV-05':('B','F0.1のcheckpoint completion/Z-pathの手証明とテストを再実行した。','F0.3から分散checkpoint抽出する一般対応は未実装。','W5',[]),
'SV-06':('O','完全image journalとrecordを全保持するモデル。','安全な圧縮・回収・stale reject継続の方式は未完成。','W5 W8',[]),
'SV-07':('H','現在headまでの認可/失効/要求/準備/関係状態を一緒に復元する。','外部identity/鍵/委譲の復元はtrusted-boundary仮定。','W5',[7,19]),
'OB-03':('H','固定label・同じlow制御/scheduleの限定profileで二実行非干渉を記述・試験した。','一般の動的IFC、time、allocation、secret-dependent policy変化は未証明。','W6',[5,21,22]),
'OB-05':('H','許可filter後に有限bufferへ保持し、high trafficによるlow行退去を防いだ。','全履歴・CPU時間を含む実資源隔離は未実装。','W6',[22]),
'OB-07':('O','active debugをobserveから分離する設計だけを示す。','breakpoint、state edit、rollbackの認可と実装は未搭載。','W6 W7',[]),
'OB-08':('H','rank witness、policy branch、source siteを説明資料として保持する。','公開するときの完全なdiagnostic prose/schemaは別仕事。','W6',[3,6,7]),
'OP-05':('H','完全offline runner、入力版、command、log、hashを成果物に含める。','実product distributionやmultiOS supportの受理ではない。','W7',[]),
'MG-02':('B','SC-24に相当する相互作用の一部を同じkernelで実行した。','実network・UI・任意local theory・全componentを含むαのSC-24は未完。','W7',[23]),
'MG-06':('H','定義・手証明・実行model・実反例・traceを作成した。','件数を完成率にしない。機械化と独立受理が必要。','W1',[23])
}
trace=[]
for req in master['requirements']:
    c,result,remaining,work,ths=X.get(req['id'],G[req['group']])
    trace.append({'id':req['id'],'title':req['title'],'origin':req['origin'],'statement':req['statement'],
      'acceptance_positive':req['acceptance_positive'],'acceptance_negative':req['acceptance_negative'],
      'master_acceptance_status':req['acceptance_status'],'requirement_completed':False,'owner_approved':False,
      'F0_3_evidence_class':{'H':'hand-proof-or-executable-selected-profile','B':'baseline-or-design-connection','O':'explicit-open-design-obligation'}[c],
      'F0_3_result':result,'remaining':remaining,'next_work_units':work.split(),'theorems':[f'F3-{n:02}' for n in ths],
      'decision_refs':req['decisions'],'master_proof_targets':req['proof_targets'],'master_scenarios':req['scenarios']})
assert len(trace)==119
payload={'version':'F0.3','input_requirements_sha256':hashlib.sha256(BASE.read_bytes()).hexdigest(),
 'notice':'Every master requirement remains unaccepted as a complete requirement. Entries record scoped contributions and unclosed obligations, not percent completion.','requirements':trace}
(ROOT/'REQUIREMENT_TRACE.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n')
md=['# 全119要件との対応','', '各行は、全要件の達成判定ではない。新しい根拠、継承した根拠、残る仕事を区別する。既存Canonと要件表の未承認状態を変更しない。','']
for r in trace:
    md += [f"## {r['id']} — {r['title']}",'',f"**元の要求（{r['origin']}）**：{r['statement']}",'',f"**今回**：{r['F0_3_result']}",'',f"**未完**：{r['remaining']}",'',f"**接続先**：{' / '.join(r['next_work_units'])}。{'、'.join(r['theorems']) if r['theorems'] else 'この要件全体を解消する新しい定理はない。'}",'',f"**正例の出口**：{r['acceptance_positive']}",f"**反例の出口**：{r['acceptance_negative']}",'']
(ROOT/'REQUIREMENT_TRACE.md').write_text('\n'.join(md)+'\n')

# Actual profile selections vs final design decisions.
D={
1:('F0.2の参照ごとのowner snapshotを継承する有限profile。','式/関係単位の整合readを表す契約を比較し、暗黙分散transactionにしない。','W2'),
2:('抽象ownerの局所原子的更新。物理的唯一サーバを要求しない。','論理ownerの実現と多owner invariantを別に定義する。','W2 W4'),
3:('第一階の一部CFGのみ。F0.3 VMはspawn/join未対応。','純粋高階値・計算型・affine handleの最小正例と型規則を作る。','W2'),
4:('数学的IntとBool。認可ctxではBool/Intを区別する。','Int64/float/textへの意味とoverflowを決める。','W2'),
5:('positive all/anyと最小不動点。same-kind DAG条件は別に維持。','ORと集団生成の使い方、局所証拠の十分性を利用例で確認する。','W1 W3'),
6:('reacquireは別認可の明示操作。新claim生成だけでは復帰しない。','宣言から再取得操作を生成するsurfaceを検討する。','W3'),
7:('instance/name/incarnation/revisionを区別。live結果のみ厳格再検査。','historical ref/snapshot/first-class handle契約を分ける。','W2 W8'),
8:('同kindと関係計算はDAG、cross-kind根拠は帰納的closure。','許された時間feedbackの意味とcache/表示境界を比較する。','W3'),
9:('presentation contextは一致する外部tagで、実clock保証なし。','logical time・deadline・clock/leaseの前提を閉じる。','W3 W4'),
10:('関係式は正確な整数演算のみ。','誤差とapproximationを権限/失敗/表示のどこで明示するか定義する。','W3 W7'),
11:('同一runtimeのrequestは重複serve拒否。結果使用は別に現在性検査。','操作別retryとtombstone回収を永続化条件に結びつける。','W4 W5'),
12:('一般的cancel/補償は未統合。','取り消す対象と既実行効果の非巻戻しを型にする。','W2 W4'),
13:('任意有限prefixの安全性のみ。','公平性、連結性、復旧条件ごとの進行性を宣言する。','W4'),
14:('sourceからのrequest/resumeを使うが、一般handler変更は未定義。','再開権の回数・所有・保存を計算型へ接続する。','W2'),
15:('外部effectはF0.3 Kernelに未搭載。','F0.2のunknown-startモデルと現在性・完全snapshotの接続を研究する。','W4 W5'),
16:('同じtyped κを全leafへ渡すnonempty all/explicit any。','任意policy transformerの順序・委譲・issuer trustを選択profileごとに証明する。','W1 W8'),
17:('ideal trusted issued records。cryptographyやByzantine hostを検証していない。','guest/host/controlplaneのTCBと信頼翻訳を決める。','W4 W7 W8'),
18:('F0.3 reparentはcommit時再検査。F0.2の準備後予約を採用しない。','strict invalidationと予約方式の違いをowner判断の対象に残す。','W3 W5'),
19:('多項式/closure/labelの既知検査器だけ。','依存型/様相の局所理論adapterとsound bridgeを機械化する。','W1 W2'),
20:('現在headに基づくsame-instance全image replay。','fresh import/過去分岐/実full resumeの意味とscopeを別に選択する。','W5'),
21:('同schema/signatureとTotalBody非弱化。','契約変更とversion coexistence、受入れ側の合意を定義する。','W3 W5'),
22:('24手証明、95tests、有限探索。Lean等の一般機械証明なし。','基礎受理は独立確認・機械化・実装対応の対象範囲で判定する。','W1 W4'),
23:('動的node/source追加・退役・reparent・同schema交換。','実αで必要な追加/撤去/状態移行をSC-24へ明示する。','W3 W7'),
24:('完全tracked reads、absence、versioned index、incarnation。','影響閉包の差分化と分散validation原子性の実現を検証する。','W3 W4'),
25:('古いrecords/journalは保持。','根拠/ログ/claim/requestの安全な忘却条件はまだ選ばない。','W5 W8'),
26:('固定labels、同low control/schedule、filter後buffer。','dynamic IFC・timing/termination/存在の脅威モデルを別に選ぶ。','W6'),
27:('受動observeのみ。active debuggerなし。','stop/edit/rollbackを別の認可された操作にする。','W6 W7'),
28:('trusted Python host、guestはtyped APIに限定。','sandbox、quota、provider isolationと実host前提を定義する。','W7'),
29:('再現用CLIと静的HTML。製品UIではない。','最小の構築/参加/診断経路とheadless APIを同じ検査に結ぶ。','W7'),
30:('性能値を目標として捏造しない。今回の時間は測定記録だけ。','対象機器とnetwork条件で検査/観測/実行の予算を決める。','W4 W6 W7')}
drows=[]
for d in master['decisions']:
    n=int(d['id'][-2:]);current,next_,w=D[n]
    drows.append({**d,'owner_approved':False,'F0_3_profile':current,'research_disposition':next_,'next_work_units':w.split(),'decision_status':'candidate-profile-only; final adoption unresolved'})
assert len(drows)==30
(ROOT/'DECISION_DISPOSITIONS.json').write_text(json.dumps({'version':'F0.3','decisions':drows},ensure_ascii=False,indent=2)+'\n')
md=['# 30の未決事項の扱い','', '今回の研究profileで試した選択を、最終設計の採用と区別する。既に説明された目的を再質問する台帳ではない。次の定義・実装の直接の前提になった判断から解く。','', '| ID | 問い | 今回の候補 | 次の検討 | 担当 |','|---|---|---|---|---|']
for d in drows:md.append(f"| {d['id']} | {d['title']} | {d['F0_3_profile']} | {d['research_disposition']} | {' / '.join(d['next_work_units'])} |")
(ROOT/'DECISION_DISPOSITIONS.md').write_text('\n'.join(md)+'\n')
print('Built:',len(rows),'theorems;',len(trace),'requirements;',len(drows),'decisions')
