# Mirroreaの構成・証拠・現在性を結ぶ基礎理論

## 0. 対象と証拠の位置

F0.3は、全体要件表の119要件をすべて充足した宣言ではない。存在、参加、モジュール、関係、局所証拠、認可、要求、更新準備、観測、復元を、共通の抽象状態の遷移として接続する研究候補である。以下の証明は人手記述であり、Lean/Rocq/Isabelleによる検査を経ていない。有限探索と実行試験は別の証拠として示す。

数学的な量化の対象は、各時点で有限な記録集合、任意の有限長の式・policy・実行prefixである。参照実装のFormula入力には深さ128以下の明示budgetがあり、実装checkerとの完全性対応はこの資源profile内に限定する。budget外の入力はFormulaBudgetOrShapeとして拒否する。数学上の導出規則や最小不動点の定理自体には128という定数を入れない。将来の全主体や全moduleを事前列挙する必要はない。長期の資源回収、物理的な無停止性、任意の悪意あるhostは保証しない。

参照実装は `model/support.py`、`model/policy.py`、`model/transactions.py`、`model/kernel.py` にある。`base/language.py` と `base/certificates.py` はF0.2から無変更で取り込んだ。引き継いだコンパイラ・多項式checkerの健全性は、F0.2の手証明と同じ信頼範囲に留まる。

## 1. 基本定義

### 1.1 同一性と型付き記録

記録の値の一致は型を保つ構造等値であり、BoolのTrueとIntの1を同一視しない。この規則は認可κだけでなく、prepared deltaとcheckpointの照合にも適用する。記録の識別は `k=(namespace,name,incarnation)` とする。namespaceは参照モデルでは整数labelを含む。別incarnationは別対象である。構造改変のrevisionと状態値のversionはincarnationから独立する。数学上は非負整数で、wraparoundは認めない。

記録kindはlocus、member、module、state、relation、anchor、access、theoryの区別を持つ。これらは業務のWorld/Avatar等ではない。完全な最終Core語彙の選択でもない。

状態Sは少なくとも、宣言N、活性条件E、support式Φ、導出されたlive集合L、現在のmember対応、値store、checked sourceとoperation、契約証拠、policy/claims/失効、関係/binding、要求/結果/継続、更新準備、観測可能な意味記録を持つ。固定されたData manifestは、これらの全fieldを列挙する。観測者のローカルbufferはSの外へ分ける。

### 1.2 Support式

```
φ ::= ⊤ | ⊥ | ref(k) | all(φ1,...,φm) | any(φ1,...,φm)
```

意味は通常の正の論理式で、`all()`は真、`any()`は偽である。これはsupport式の規則であり、認可policyでは空の枝や無条件Topを受理しない。`deps(φ)`は全branchに現れる参照の集合である。式に否定や arbitrary source predicateを入れていない。失効はeligible集合を変更する入力として扱う。

`E(S)`はenabledな記録から、現在のpolicy照合に失敗したaccess記録を除いた集合である。accessの照合は署名を推論で作るのではなく、既に発行されたclaimと現在のissuer epoch・policy版・要求文脈を検査する。member自身のlive条件はsupport式に残すので、権限からmember、memberから同じ権限を相互に正当化することはない。

```
L0 = ∅
L(i+1) = Li ∪ { n∈E | ⟦Φ(n)⟧Li = true }
Live(S) = 最初の不動点
```

liveでないことは、コードや記録の論理的矛盾とは限らない。未充足の参加条件、失効、循環だけで根拠を作ろうとする構成も含む。履歴としての定義や証明は保持できる。

### 1.3 グラフの区別

同kindの構造依存辺は、前提から従属先へ向ける。それぞれの宣言された存在・member・module等のグラフではDAG条件を検査する。一方、異kindをまたぐsupportは最小不動点の帰納的根拠で扱う。ORで外部根拠を得る場合まで、一律の全体グラフ非循環性で禁止しない。

関係式の計算依存DAGは別に検査する。関数再帰、ループ、物理的なネットワーク、相互の引用をすべてDAGにする条件ではない。

### 1.4 情報label

参照モデルは有限入力中に現れる非負整数を全順序labelとして使う。証明の正のsupportに関する部分は、任意の順序の下方閉集合へ一般化できる。値の機密性と構造の存在を一つのlabelへまとめた保守的profileである。分離する拡張は別の義務である。

support辺、関係依存辺、全fallback候補について、前提label ≤ 従属結果labelを必要とする。選択済みbranchだけではなく、選択を変え得る候補も含む。owner作用のlabelは読み値、書込み対象の存在、module/locus、policyをすべて支配する。書込み先がそれより低い場合は拒否する。

### 1.5 認可文脈

```
κ = (principal, member-incarnation, action, target, instance,
     request, arguments, code, contract)
```

policyは名前付き必要条件の非空all/明示anyである。各leafはissuer、predicate、現在のepoch、同じprincipal/member、action/targetのscope、情報labelを照合する。証拠にはκ、policy版、使ったclaim、選んだbranchを残す。

claimは信頼境界が発行した理想的な記録である。暗号認証・鍵管理の正しさを証明したのではない。policy checkerは発行済み集合との一致を確認する。証明checker、source、transportはclaimを発行できない。

### 1.6 局所的な契約

F0.2の多項式証拠について、実sourceから再生成した検証条件とevidenceを照合する。`Partial`は成功した場合の保存、`TotalBody`は選択した有限owner bodyが既定前提から失敗せず返ることを表す。後者はnetwork、認可、資源、環境の可用性を無条件に保証しない。

今回のadapterはTotalBodyの外部前提を空に固定する。証拠の中にある任意の前提を勝手に採用しない。追加前提を導入する将来の境界では、提出側と独立に、source契約から期待前提を生成し、実行点で満たす義務がある。

同じownerで保護した不変条件を、全mutatorと交換後のmutatorが保存しなければならない。別ownerの可変状態をその不変条件へ直接混ぜることは今回のprofile外である。

### 1.7 観測と状態遷移

authoritativeな一歩は、(a)現在の前提を検査し、(b)必要な有限計算と変更を行い、(c)liveとfallbackを正規化し、(d)実際の変化から意味記録を生成する。拒否時には候補状態を公開しない。

`_wf`は防御的な実行検査でもあるが、以下の手証明で「WFを事後に検査したから正しい」だけを根拠にはしない。各caseが何を読み、どの前提により保存するかを列挙する。例外時に全拒否できることと、有用な正例を受理できることも区別する。

受動観測のpullは、現在のobserver権限で許された意味記録を先に射影し、その後にobserver自身の有限bufferへ格納する。Sや次のdomain stepの選択は書き換えない。物理CPU時間がゼロという主張ではない。

## 2. Supportと現在性

### F3-01 正の単調性と有限停止

**命題。** Φが上記の正の式でNが有限なら、X⊆Yに対して ⟦φ⟧X⇒⟦φ⟧Y。deriveは高々|N|回の真の集合増大の後に停止する。

**証明。** 式の構造に関する帰納法。refの場合は集合包含、allは各子の帰納法、anyは成立した一枝の帰納法、定数は直ちに従う。反復は前の集合をunionするので減らない。停止しない一回は新しい元を最低一つ追加する。追加済み元は再追加しないので、真の増大は|N|回を超えない。∎

### F3-02 帰納的根拠に関する健全性と完全性

**命題。** live(n)を「n∈E、かつΦ(n)のrefを先行する有限導出で満たす」という帰納規則で定義すると、deriveが返す集合は、その有限導出を持つ元とちょうど一致する。

**証明。** 反復iで新たに入った元は、反復i-1までの元のみから式を満たす。iに関する帰納法で有限導出を構成する。逆に導出の木の高さについて帰納法を使う。有限個の子の導出がある段階までに得られれば、親の式はその段階で真になり、次の反復で親が入る。従って全有限導出が最終集合へ含まれる。∎

これは任意のMirプログラムの完全検証ではない。選択した有限support言語の完全性である。

### F3-03 独立certificate checkerの健全性・完全性

**検査規則。** 提出物は集合Cと、Cだけをdomainに持つ自然数rank ρである。(1)C⊆E、(2)各n∈CについてΦ(n)がrankの小さい参照だけで成立、(3)各n∈E\CについてΦ(n)がCで偽、を検査する。加えて、Topのheight=-1、Bottom=∞、refのheight=rank、all=max、any=minとしてρ(n)=1+height(Φ(n))を検査する。

**命題。** 合格するCは最小不動点であり、rankは同期反復で初めて追加される段階に一致する。deriveの出力は必ず合格する。

**証明。** rankに関する強い帰納法で、(2)からCの全元に有限導出がある。F3-02よりC⊆L。条件(3)はCがeligibleな含意に閉じていることを表す。L0⊆C、Li⊆Cなら、次に追加される元も(3)によりC内なのでL⊆C。集合が一致する。height方程式は、allでは全前提がそろう最も遅い段階、anyでは一枝がそろう最も早い段階を与える。根を0としてrankの帰納法から最初の追加段階と一致する。deriveはその段階を記録しているので全条件を満たす。∎

`check_closure`はderiveを呼ばない。循環した自己証拠と、全部拒否して導出可能な根を落とす証拠の双方を拒否する。

### F3-04 退役の単調性と正規化の冪等性

**命題。** 宣言とsupportが同じでE'⊆EならL(E')⊆L(E)。同一構成を二度正規化してもlive、rank、fallback cursorは変わらない。

**証明。** 各反復の集合包含をiに関する帰納法で示す。E'に許された追加はEでも許される。正規化は固定したE,Φから同じ最小不動点と最小rankを得る。cursorは現在位置以後の最初のlive候補か末尾へ移り、二回目にはその位置の条件が変わらない。∎

新しいincarnationや明示的なreparent等でEやΦが増える場合は、単調退役の前提の外である。それでも既存bindingのcursorは勝手に後退しない。

### F3-05 低いsupportの独立性

**命題。** Bがlabelの下方閉集合で、B内の記録のsupport参照はすべてB内、かつB内のeligibilityはB内の制御事実だけで決まるとする。このとき `Live(S)∩B = Live(S|B)`。

**証明。** 各反復iについてLi∩Bと制限系のLiが一致することを示す。i=0は空。B内の式はB外を参照しないので、前反復の一致から同じ真偽を得る。eligibilityも一致しているため、次の追加集合が一致する。有限停止後の等式を得る。∎

このため、秘密の親の存在を公開childの条件に使ったり、秘密claimの有無で公開policyを成立させたりすることを、型/label境界で禁止する。値だけの情報流検査では足りない。

## 3. 認可と局所証拠

### F3-06 同一文脈のpolicy合成

**命題。** `evaluate(policy,κ,claims)`が返す証拠は、同一κについて、そのpolicyの全all枝または明示したany枝を、真正な現在claimで満たす。有限な提出claim集合に関して、満たす証拠が存在するならevaluateは一つ返す。

**証明。** leafでは、発行済み表との全record一致、非失効、issuer epoch、principal/member、action/target、labelを明示的に確認する。allの場合は同じκを全再帰呼出しへ渡すので、異なる主体・要求を混ぜない。anyは順に各枝を検査して最初の成功を返す。どの枝にも成功がなければ導出はない。有限な式と候補集合なので停止し、構造帰納法で健全性と完全性を得る。∎

これは認可の論理的判定であり、署名方式やissuerを信頼すべきかの証明ではない。任意のpolicy transformer、委譲論理、第三者pluginは対象外である。

### F3-07 過去の証拠と現在の使用

**命題。** 証拠に記録したκのどれか、policyの版、使ったclaimの有効性が変われば、旧証拠は無条件には使用できない。正確な再検査が成功した場合だけ、同じ使用条件が現在も成立する。

**証明。** κとpolicy版は全項目の等値で比較する。その際、比較先のcode/対象revision等は現在の状態から構成する。証拠に格納した過去のκをそのまま比較先にも渡すことでは、現在性の検査にならない。anchor accessは現在の構造revisionを再構成し、古いrevisionで得たaccessを同じ対象へ無断で使用しない。使用claimを現在の表から取り直し、F3-06を再実行する。全結果とbranch証拠を比較するので、古い根拠を別の根拠へ無断で置き換えない。memberの現在対応とlive条件は使用境界でも確認する。検証器はこの処理でclaim表を書かないため、数学的な証拠を権限へ変換しない。∎

別の新claimで同じoperationを行うことは、新しい要求として可能である。これは過去の要求やaccessを自動で再有効化する規則ではない。

### F3-08 ソース束縛の契約受理

**命題。** F0.2の選択した証拠checkerの健全性を前提とする。sourceから作ったOperation、その不変条件I、所定の強さPartial/TotalBodyについて証拠を照合したなら、異なるコード、勝手に加えた前提、弱い保証の証拠では、その契約を受理しない。

**証明。** 検証条件を実Operationから再生成し、Operation identity、invariant hash、契約種別、期待前提を比較してから証拠木を検査する。TotalBodyの場合、このprofileの期待前提は空である。証拠に書かれた前提を期待側へコピーしない。交換時はTotalBodyからPartialへ落とさない。従って証拠の対象と受入れ対象は一致する。checkerの手証明から、その一致する対象の部分/全body正当性を得る。∎

有限hashを数学的な同一性と無条件に同一視しない。仕様上は完全な構造一致を要求し、実装のhashはcollisionがないことと完全record確認の境界を持つ。

### F3-09 全mutatorによる不変条件保存

**命題。** 初期storeがIを満たし、Iで保護したowner状態を書く全operationがF3-08でI保存を確認され、書込みが正確なowner transitionを通るとする。型付き成功・失敗・再送・認可拒否・正当な同schema交換を任意の有限回行ってもIは保存される。

**証明。** 実行長の帰納法。成功するowner bodyでは、事前のIと契約証明から返されたstoreのIを得る。評価が失敗した場合はlocal copyを公開せず、元のstoreを保つ。拒否・再送・policy変更は保護storeを書かない。コード交換ではstoreを保ち、新writerの証拠を先に検査する。元のIが次の成功の前提になる。未検証writerを追加する経路はadmissionで拒否する。∎

異なるownerの同時不変条件はこの命題からは出ない。rely/guarantee、資源移送、必要な共同確定について別の検証を要する。

## 4. 参照・要求・構成

### F3-10 正規化された多グラフの保存

**命題。** 宣言参照の閉包、同kindのDAG、関係DAG、labelの向き、値の型、invariant、正確なmember対応をWFとする。宣言追加、退役、検査済みreparent、policy/claim更新、sourceに基づくowner実行の各受理遷移はWFを保つ。

**証明。** 新宣言では、未知参照を拒否し、同kindのDAGをKahn法で検査する。終了時に全頂点を除去できたこととcycle不在の同値は、入次数0頂点が存在する有限DAGの帰納法による。関係計算の辺も別に同様に検査する。label条件と値型を検査する。reparentでは変更後の辺に同じ検査を適用する。退役/policy失効は辺を新設せず、F3-04の再導出で利用可能性を下げる。owner遷移は構造を変えず、F3-09で型とIを保つ。各場合にnormalizeがF3-02/03の同じliveを生成し、次のF3-11でbindingを正規化する。現在memberを交換する場合は旧incarnationを退役させ、fresh keyだけを対応表に置く。∎

循環検査の成功を、影響範囲の小さい分散実装が既に存在する証明とはしない。現在は抽象的な制御遷移である。

### F3-11 認可失効を含むfallbackとfresh reacquire

**命題。** 候補列c0..cm-1とcursor pについて、normalize後にはp≤mであり、p<mならcp∈Liveである。同一lineageでpは減らない。accessのpolicy/claimが失効したなら、anchorの物理的存在を保ったまま、そのaccessを使わない候補へ進む。

**証明。** normalizeは現在pから非live候補を飛ばすwhileだけを使う。高々m-p回で停止する。失効accessはeligibleから除かれ、そのaccessを要求する導出はF3-02により消える。従ってselected accessが失効すればwhileが進む。sampleの有無はEやnormalizeの入力に含まれないので欠測では変化しない。reacquireだけが新lineageを作ってp=0から選び直す。その遷移には独立した現在認可と候補型・label検査が必要である。新しいclaimや同名anchorの生成だけでは、旧candidate keyもcursorも変えない。∎

### F3-12 認可付き関係DAGの純粋射影

**命題。** 検査済みの有限関係DAG、現在liveなbinding、全依存sampleについて一致したpresentation contextとlineageがあるなら、memo化した評価と、同じDAGを展開した再帰評価は一致する。評価はSを変更しない。必要sampleが欠ければPresentationGapであり、semantic fallbackは起こらない。

**証明。** トポロジカル順序に関する帰納法。bindingのleafでは現在のselected anchor/accessとsample headerが一致して同じ整数を得る。const、加算、整数倍は同じ子の値から同じ結果を得る。共有子をmemo化しても決定的な値は同じである。unknown/stale sampleは値として採用されない。コードはstore/Bindingを変更せず、memoは関数ローカルに限る。F3-05と全候補のlabel上界により、許可された投影の可用性が隠れた高い候補に依存しないprofileを構成できる。∎

presentation contextは物理的な世界共通frame番号ではない。この命題は実時計の同期や近似誤差を証明していない。

### F3-13 一つの要求の一回の状態確定

**命題。** request IDとκが保持され、Pendingからだけserveできる遷移では、同じ要求によるowner storeの変更は高々一回である。

**証明。** 最初の成功でstoreとServed決定を同じ抽象遷移で保持する。以後statusはPendingではなく、重複要求はowner評価前に拒否する。失敗はRejectedを保持する。消費/ReleaseDeniedへ進んでもPendingへ戻す遷移はない。journal復元についてはF3-19がこの決定を保つ。∎

これは配送保証ではない。送られたが失われた要求の完了や、実外部効果のexactly-onceを主張しない。

### F3-14 live結果の使用は、過去の成功とは別に検査する

**命題。** service後にmember、使用依存module、policy等が失効した場合、過去のstate changeを消さず、liveな継続への結果の消費を拒否できる。

**証明。** serveは現在性と権限を先に検査し、成功したら結果と意味記録を残す。consumeでは改めてmemberの現在対応、要求に束縛した全live依存、現在code/contract、policy証拠を検査する。失敗するとReleaseDeniedへ進み、store/Served履歴を戻さない。正常消費はPending continuationを一度だけ外すので二度消費しない。∎

これはlive-resultという保守的な結果profileである。失効後も独立したsnapshot値を読むprofileには、別の公開契約が必要である。静的な数学的命題まで失効で偽になるとはしていない。

## 5. 追跡された更新検査

### F3-15 検査read traceの再生補題

**前提。** 検査Vは決定的・停止し、状態を直接参照せず、キー取得APIだけを通る。APIは成功したlookupだけでなく不在lookupも記録する。同一keyの反復読取りが異なるstampを観測した場合は、その検査を中断する。最後のstampで過去の読取りを上書きしてはいけない。各(value, stamp)取得は同じrecordの原子的な読取りである。集合queryはversion付きindex recordの読取りとして実装し、更新側はそのindexを維持する。

**命題。** Vがdelta Δの検査中に読んだ各keyの完全な値とstampが別状態でも同じで、blind write先のstampも同じなら、Vは同じ読取り列と判定を返す。

**証明。** Vの実行ステップを帰納する。純粋な計算は同じ入力から同じ結果を返す。次のlookup keyは一致する過去の計算から選ばれ、記録されたstamp/値が一致しているので同じ結果になる。不在も一つの値であり同様。従って途中で新しいbranchやkeyへ逸れず、同じ終端へ到達する。∎

stampから値の一致を導くには、すべての変更がstampを増やし、削除してもその世代を忘れないことが必要である。queryをguestが独自に未記録scanする場合、この補題は適用できない。

### F3-16 現在認可付きOCCの保存

**命題。** prepareの結果を、変更内容・read stamp・提出主体へ封印する。commitがその封印、全read/blind-write stamp、現在の使用権限を同じ制御遷移で確認してから、まさに検査したdeltaを適用するなら、prepareで検証した性質をcommit時にも利用できる。

**証明。** 変更内容の差し替えは封印の不一致になる。状態の差はF3-15の条件で排除されるので、現在状態でもVの判定は同じ。認可はVの過去の判定から推論せず、commit時の別の前提として成立する。その後に別操作が割り込まないため、まさに検査した状態遷移を適用できる。∎

これはグローバルに全実行を止める設計ではない。抽象的な一歩に必要なread/write範囲だけを、実装で原子的に照合する義務を指定する。実装がその義務を満たす証明は別である。

### F3-17 reparentの局所検査

**命題。** 対象nの新しい親から、同kindのsupportを逆向きにたどり、nへ到達しないことを確認する。読んだ全記録のrevisionとn自身をcommitで照合すれば、同kindのDAG性を壊さないreparentを適用できる。

**証明。** 変更される辺は新親→nだけである。新cycleがあれば、必ずその辺の一つを含み、新親からsupportの逆向き探索でnへ到達する。全枝を探索するため、現在ORで使われていない辺も見落とさない。探索した記録のrevisionが一致すれば経路は変わらない。未探索の新記録だけで新しい到達経路を作るには、探索済み記録の出辺を書き換える必要があり、そのrevision照合で拒否される。認可はF3-16の使用時条件で別に確認する。∎

異kind循環のrootなし構成はF3-02で利用不能になる。reparentのためにすべてのグラフを同じDAGへ潰してはいない。

### F3-18 独立変更の可換性と適用範囲

**命題。** 変更A,Bの全read集合D、write集合Wについて、WA∩(DB∪WB)=∅、WB∩(DA∪WA)=∅であり、権限と検査に影響するrecordもその集合に含まれるとする。両変更は互いの準備をstaleにせず、domain recordの最終結果は可換である。

**証明。** 各変更は他方が読む値・stampを変えないのでF3-15が適用できる。write集合は交わらず、各writeの計算入力も変わらないから最終record mapは一致する。観測上の順序やaudit列の順番が同じとはしていない。因果的に独立な変更として、domain内容を比較する命題である。∎

変更辺の端点が異なるだけでは前提を満たさない。b→c,d→aがあるときa→bとc→dを別々に検査すると、合成後だけcycleになる。この反例は実行試験に含む。

## 6. 復元と観測

### F3-19 現在の確定prefixに基づく全成分復元

**前提。** manifestがこのprofileの全authoritative成分を列挙し、各commitは同じ順序の完全recordをdurableに保持する。現在の確定headはsnapshot提出者から独立に信頼される。hashを使う実現ではchain integrityの暗号前提を明示する。

**命題。** 正当なcheckpointと、その後の現在headまで欠けないrecord列を検査して復元すると、現在のauthoritative状態と同じ状態を得る。過去のsnapshotだけで失効、消費済み要求、更新準備、関係lineageを巻き戻さない。

**証明。** checkpointは発行時の完全状態と一致する。各suffix recordは直前digestに結びつき、その位置の完全commit状態を持つので、prefix長に関する帰納法でそのcommitと一致する。末尾が独立に保持されたheadに一致しなければ拒否する。復元はコードや外部操作を再実行せず、記録された状態を再構成する。manifestの欠落や未知成分は受理しない。∎

このモデルは各commitの完全なin-memory imageを持つ。実ファイルのincremental log、power failure、replica合意、back-up管理、外部device状態まで検証済みではない。現在のheadをtrustedに得られないオフラインloadは、この命題の適用外であり、fresh import等の別意味が必要である。

### F3-20 受動観測の消去同値

**命題。** 観測付き状態(S,O)において、domain stepはSだけを読み(S',O)へ進み、observeはSを変えず(S,O')へ進むとする。任意の有限実行からobserve stepを消すと、元と同じSの列とdomain outcomeを持つ実行が得られる。逆に任意のdomain実行へ有限個のobserveを挿入できる。

**証明。** 実行長の帰納法。domain stepはO非依存なので消去後にも同じstepが可能。observe stepではSが変わらず、その一歩を省ける。逆方向も同じ非依存性により挿入可能。失効したobserverの拒否もSを書かない。∎

これは物理時間ゼロでも、observerを無限に優先するschedulerの下での進行保証でもない。無限実行については、domainの公平性をobserverの有無から独立に保つ必要がある。breakpoint/rollbackはこのobserve遷移に入れない。

### F3-21 固定label境界での二実行非干渉

**対象profile。** 公開catalogue、低いmodule/source、policyと低い入力が同じ二状態を比較する。高い値は異なってよい。低い状態へ影響するcontrol操作は同じものを対応させる。高い操作は、低いcatalogue・issuer epoch・principal slotを変更せず、label検査を満たすものだけである。per-label counterとobserver bufferを使い、physical timing、allocation、global log位置、secret-dependent terminationは観測しない。

**命題。** 同じ低い操作scheduleと低い入力に対して、任意の有限prefixで、低いrecordと低い意味記録列が一致する。高い内部stepは低い射影でstutterする。

**証明。** F3-05により低いlive導出が一致する。低いpolicyは高いclaimを使えず、同じ低いκの結果が一致する。低いowner操作の潜在的な読み値、guard、書込み先の存在はすべて低く、同じchecked sourceから同じ結果/失敗を得る。高い操作はwrite-downを禁止され、低いrecordを変えない。関係の全候補と式依存もlabelで閉じている。高いeventsはfilterで消え、per-label IDは低いIDをずらさない。低い値を返す高いcontinuationのtaintを落とさない。各caseの局所保存から、対応した有限scheduleの帰納法で結論を得る。∎

任意の高い管理操作、labelの変更、動的declassification、観測policyの秘密依存変更、scheduler covert channelについての一般定理ではない。この制限は明示的な未解決事項である。permissionとprohibitionを含む一般的な動的情報流は、既存研究でも複数の意味を区別している。[S4]

### F3-22 許可射影と有限保持の順序

**命題。** `safe=filter_allowed(H)`を先に計算し、その後に`last_k(safe)`を取るなら、高いイベントの挿入だけで保持される低い最後k件は変わらない。逆の順序では一般に成立しない。

**証明。** 高い挿入はfilter後の列を変えないので、その同じ列にlast_kを適用した結果も同じ。反例としてH=[low]、k=1へhighを末尾挿入すると、先にlast_kを取る実装は[high]から空列を返し、元の[low]と異なる。∎

完全履歴を無制限に保持すべきという要件ではない。productionでは、許可された単位の独立queue/retentionと課金を実現する必要がある。

## 7. 全体と拡張

### F3-23 任意の有限回の合成保存

**命題。** 本章の受理規則を通るsource追加、owner実行、構造変更、認可変更、fallback、正当なcode交換、checkpoint復元の有限列は、初期WFからWFと、各使用点の現在性を保存する。拒否された候補は、新しい権限・値・利用可能性を部分的に残さない。

**証明。** 基底はbootstrapの明示検査。帰納stepはF3-06～19の各該当caseとF3-10の再正規化を用いる。権限変化で過去の数学的証明を偽にする必要はなく、使用時にF3-07/14がその適用可能性を改めて判断する。新たなsourceやnodeも同じ規則へ入り、次のstepの前提を満たす。個々の事後WF検査だけでなく、値変更はF3-09、graph変更は具体的なcycle/label検査、認可は現在のclaim照合、復元は正当なprefixを根拠とする。∎

この有限prefix安全性は、有用な正例をすべて受理すること、無限拡張の物理資源、永久故障での進行を保証しない。正例・拒否例を含む統合実行を別の受入れ条件にする。

### F3-24 実現への条件付き接続義務

**条件付き命題。** 具体実行Cと本抽象状態Sの対応Rについて、各具体stepが、同じ公開効果を持つ有限個の抽象stepまたはsilent stepへ対応し、対応Rを保つとする。初期対応があれば、任意の有限具体traceの公開効果は抽象traceに含まれる。

**証明。** 具体step数の帰納法で、各stepの対応する抽象列を連結する。初期対応と各step後のRが次の接続を保証する。∎

これは必要な接続条件であり、Rust/QUICに対する前提をこの成果でdischargeしてはいない。また、この一実行refinementだけからF3-21を移してはならない。機密性には二状態の低同値を保つstep対応が、進行性には無限silent stutterを排除する条件が別に必要である。並行システムのsecurity-preserving refinementについても、この追加の問題は一次研究で扱われている。[S5]

## 8. 条件付きの理論拡張契約

共通の意味モデルBに対する契約Cを公開する局所理論Tは、(a)使用する仮定、(b)ソース/契約/理論版への結びつけ、(c)Bへ解釈したCの健全性、(d)値/資源の境界変換、(e)残余義務、を提供する。任意の理論Tがこの条件を自動で満たすという定理ではない。

F0.3は、多項式証拠、現在supportのrank証拠、label境界の三種類について、異なる判定を同じκと対象へ結びつけている。算術の真理、liveの現在性、許可という三つを一つの命題に混ぜない。完全な依存型、様相型、任意handler、所有resourceを消去する変換の一般的な証明は残る。

既存の理論adapterを異種の反応的言語へ接続するPiCoreの方針は、この境界を具体化する参考になる。ただし、同研究の成果をF0.3の未証明部分として流用しない。[S3]

## 9. 証明から導く実装上の責任

最小不動点を毎回世界全体で同期計算する必要がある、とは定理から導かれない。参照モデルは一つの意味的な比較器である。productionでは、影響閉包の差分更新、現在のclaim/policyのview、版付きの索引、受動観測の独立buffer等で実現し、それが同じ意味を保つことを示す。

同様に、原子的なreparent検査をPythonのglobal lockで行うことと、全インターネットに唯一の権限者を置くことは別である。独立instance同士の接続は、公開した契約の対応と、必要な相互認証・根拠の伝播を持つ新しいboundaryになる。未知の体系を既定で信用したり、世界全体の単一コンパイルを必須にしたりしない。

この研究候補は、19a6decfbea0815b7b4265e8fadfb399270aaed2のCanonを変更しない。I3-3の有限受理とpauseをそのまま保持し、既存の定理台帳を自動的に解消済みにしない。[S1,S2]

出典の詳細は `SOURCES.md` を参照する。
