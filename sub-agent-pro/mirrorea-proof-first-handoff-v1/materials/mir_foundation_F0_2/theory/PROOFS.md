# F0.2 — 定義と手証明

本書はF0.2の数学的な対象と、その対象についての手証明を記す。Python実装全体の形式証明、既存Mirのproof-assistant証明ではない。実行検査は証明の代用にせず、定義・実装・境界の不一致を発見するために併記する。

`I`は任意の命題ではなく、該当する箇所で指定した状態範囲に対する述語である。仮定に「操作が安全である」をそのまま置く場合は、当該操作の具体的な保存証拠の義務であることを明記し、実装済みのチェックへ対応づける。コードで未実現の契約は台帳で分ける。

## A. sourceとCFG

### A.1 構文と判定

値の型は`Int`、`Bool`とする。activity内部では`Future τ`も使うが、関数の引数・結果・owner状態へは保存しない。整数は数学的整数である。型付き値の評価は、利用可能な数学的メモリ上の関係であり、現実の有限メモリ上の無条件成功を意味しない。

式には、定数、局所変数、セル参照、整数の加減乗算、単項minus、not、同型の比較、条件式、第一階の呼出し、spawn、joinがある。副作用を持つ式の評価順序は左から右とする。

commandは局所代入、セル代入、if、while、call statement、return、skipである。owner commandはwhile/call/spawn/joinを含まず、assertを持てる。ownerのセル参照とセル代入はすべてそのownerに属する。activityのセル代入は、対象ownerで式全体を評価する一つの作用へ展開する。その右辺は、対象ownerのセルと、activityの不変な局所値だけを参照できる。

署名環境`Δ`は、すべての関数の引数型、結果型、locus、種類を先に持つ。局所型`Γ`は名前ごとに固定し、確実に初期化済みの集合を`D`で表す。

代表規則は以下である。

\[
\frac{x\in D\quad\Gamma(x)=\tau}{\Delta;\Sigma;\Gamma;D\vdash x:\tau}
\qquad
\frac{\Sigma(a).type=\tau}{\Delta;\Sigma;\Gamma;D\vdash a:\tau}
\]

\[
\frac{\Delta(f)=(\tau_1,\ldots,\tau_n)\to\tau\quad\Gamma;D\vdash e_i:\tau_i}
 {\Gamma;D\vdash f(e_1,\ldots,e_n):\tau}
\]

同locusのtask callはstackを増やし、owner callはtyped outcomeを待つ。異locusのtask callは現profileでは受理しない。importはowner operationの署名だけを受け入れる。

局所代入では右辺の型と既存/注釈型が一致し、`D`へtargetを加える。ifでは、継続する両枝の`D`の積集合を使う。片枝だけがreturnする場合には他方の`D`を使い、双方returnする場合には全体がreturnする。whileから出た後の`D`は入口と同じである。関数の全構文経路には、型が合うreturnが必要である。構文上のreturn後の文は判定・翻訳の対象にしない。

`R(s),W(s)`は実際の構文上のセル参照・書込みを収集した集合とし、インターフェースのeffect boundへ残す。synthetic assignmentでは、右辺の局所自由変数の値を要求発行時に捕捉する。セル依存はownerで読む。

### F2-01. 型検査の終了、相対健全性、相対完全性

**命題。** 有限な整ったソース、セル環境、完全な関数署名環境について、F0.2の検査は終了する。また、A.1の規則で導出が存在することと、検査が受理することは同値である。受理後の局所型、definite assignment、owner制約は導出のものと一致する。

**証明。** 署名収集は有限の宣言を一回走査する。式判定は真部分式だけを再帰判定する。関数呼出しではbodyを再帰判定せず、有限の署名を照合するため、自己再帰・相互再帰でも型検査の再帰は増えない。command判定も真部分blockを処理する。したがって有限入力で終了する。

健全性は式とblockの構造に関する相互帰納法で示す。定数はその値型、局所変数は`D`と`Γ`、セルは`Σ`の検査そのものが対応する公理を与える。算術・比較は帰納法で得た子式の型を等号検査し、指定された結果型を返す。callは引数の帰納法と`Δ`の照合から規則を適用できる。

局所代入では、既存型または注釈型との一致があるので、同型の値を置く規則が適用できる。セル代入では、型一致に加えてownerの一致を検査する。ifでは条件型がBoolであり、二つの枝の帰納法の結果から、継続経路の積集合または一方の集合を使う規則を得る。whileは入口の`D`の下で条件とbodyを検査し、外側へ新しい確実初期化を主張しない。returnの型を署名と照合する。ownerで禁止した構文には導出規則がなく、検査も拒否する。

完全性は判定導出の高さについての帰納法で示す。最後の規則は構文の種類から決まる。各前提を帰納法で検査でき、規則が要求する型・owner・署名の条件が成立するので同じ検査は失敗しない。ifのreturn区別と`D`の結合も規則どおりである。関数署名にアノテーションがあるため、再帰の型を推測する必要はない。□

これは、意味的に安全なすべてのプログラム、一般の多相型、patchのすべての安全な変更についての完全性ではない。

### F2-02. owner操作の全域評価、型安全性、frame性

**命題。** 型が合うstoreと引数で、受理されたowner操作を実行すると、有限時間の数学的評価で、型が合う返値と`W`内だけを変更したstore、または宣言されたassert失敗を得る。失敗はstoreを変更しない。

**証明。** 純粋式の構造帰納法で、各演算が宣言型上の全域関数であることを使う。条件式ではBool条件から一方を選び、両枝の型が一致する。owner commandにはloop/callがないため、実行する文数は有限構文の節点数以下である。代入では式の型安全性からtarget型の値を得る。局所storeのcopyへだけ更新し、すべてのassertとreturnを通過してから、その差分を一度に公開する。従って失敗時は元storeのまま、成功時は`W`以外が不変である。型付きreturnが全経路にあるため成功時の返値型も一致する。□

### A.2 再帰を含む対応証明の定式化

F2-03とF2-04は、callee全体の終了を相互に仮定する循環証明ではない。sourceの式継続・command継続・call stackと、CFGの対応する制御点・register・stackを同時に関係づける。構文帰納法で証明する式の場合分けでは、再帰callをcalleeの終了まで展開せず、同じ型付き引数でcalleeの入口へ移り、同じcaller継続を保存する一歩だけを示す。callから先の任意有限prefixへの適用は、この同じ対応関係のstep保存による帰納法で行う。以下で「calleeの補題を使う」は、この入口・帰還の局所対応を指す。停止する式についての値一致は、その有限prefixの系である。

上の型付け規律は、変数名ごとの型固定と、構文上の宣言制限（重複仮引数、既定引数、追加decorator、未宣言の配置を拒否）も含む。同じ名前へ別々の枝で異なる型を与えるプログラムはこの体系の外であり、checkerの完全性で受理を要求しない。

### F2-03. 式翻訳の継続対応

**命題。** 型付き式`e`、同じ初期局所値、対応する状態の下で、生成された式CFGは、ソースで定義した左から右の操作列と同じ要求を発行し、同じtyped outcomeを受け取ると、freshな結果registerへ同じ値を置く。結果待ちの状態も、同じ継続を表す。

**証明。** 式の構造帰納法。定数・局所変数は一つのconst/moveで値が一致する。単項・二項演算は、左/右部分式の帰納法を順に適用し、その結果へ同じ数学的演算を行う。途中でowner callやreadが生じれば、そこで待機した継続に帰納法を適用し、同じoutcome後に後続の評価を行う。

条件式は条件の帰納法から同じBoolを得る。生成されたbranchは同じ枝へ入り、その枝の帰納法で同じ値を得る。両枝の結果を同じfresh registerへmoveし、非選択枝をjumpで越えるので、非選択枝の効果は生じない。

cell readは、ソース意味が定めた一回のowner snapshotを、同じセルを読むgenerated operationとして発行する。操作の型・ownerは`Σ`から決まり、結果を待つregisterはfreshなので他の値を上書きしない。callの引数は帰納法で同じ順に得られ、同じ署名のcalleeへ渡る。owner callは同じ操作の継続、task callは次のstack補題を使う。spawn/joinはF2-05の型付きfuture関係を使う。□

### F2-04. command/CFGの有限prefix保存とstack安全性

**命題。** 受理されたactivity群について、ソースの小ステップ実行とCFG実行の間に、内部register操作を隠すweak simulationがある。任意の有限prefixで、操作発行・owner作用・結果消費・fork/join・返値/失敗の意味を対応づけられる。局所制御の未初期化変数読取や型の違うreturn先は生じない。

**証明。** 関係を、(i)ソースの現在block/式継続と対応するCFG区間、(ii)型付きのソース局所値と対応register、(iii)各callerの残りblockとreturn先を保存するstack frame、(iv)未完了操作に対する一意の待機register、の積とする。

skipはnopまたは零step、局所代入はF2-03とmoveへ対応する。セル代入は、右辺の局所自由値を同じ状態で捕捉し、owner側で同じASTを評価するgenerated operationへ対応する。セル値をcallerで先に読む変換ではないので、owner service時の同じstoreに対してF2-02が使える。

ifはF2-03で同じ条件を得て同じbranchへ進む。whileは条件block、body、back edgeからなり、一反復の対応後に同じ関係へ戻る。無限反復を構造帰納法で展開する必要はなく、任意の実行prefixの最後の一歩について、この関係が保たれることを示せばよい。

task callは、型付き引数を持つcallee frameと、結果型が等しいreturn先をpushする。returnは同型の値をcallerへ戻してpopする。再帰でも同じ局所規則を使い、有限prefixには有限個のframeしか現れない。型検査の`D`規則は、branch後と零反復後にも必要な変数の存在を保証する。owner結果の遅延は、source/CFGとも同じ待機状態へ対応し、輸送だけのstepはstutterする。

各caseで関係を保ち、初期環境が一致するので、prefix長の帰納法で結論を得る。これはloop/再帰の停止性を主張しない。□

### F2-05. structured futureの親子性と完了の整合性

**命題。** sourceが作ったFutureは、そのactivityがspawnした子を指す。Futureを使った循環待ちをsourceから作れない。親の成功終了時に未完了の子が残らず、子の失敗は親の成功に隠されない。

**証明。** Future型を生成するsource規則はspawnだけであり、返値・引数・セル型はscalarに限定されている。整数からFutureへ変換する規則がなく、別activityへFutureを渡す規則もない。spawnはfreshな子IDを生成し、現在の親の集合へ追加する。joinはその集合に属する子だけを受け入れる。従ってFutureによる待ちは真の子孫へ向かい、有限の生成prefixではcycleを作らない。一般のowner操作を介した待ちや、無限の子計算まで排除する命題ではない。

親の終端規則は、全子の終了を前提とし、子に失敗があるときには親も失敗側へ進む。保存されたwaiting/return継続もこの規則を迂回しない。従って成功終了時には全子が終了している。□

## B. 証拠kernelと局所理論

### F2-06. 多項式証拠kernelの健全性

**定義。** 変数評価`ν`に対し、有限多項式`p`の値を`⟦p⟧ν`と書く。仮定環境は`g_0,...,g_m≥0`と`h_0,...,h_n=0`である。`check(π,G,H)=p`は、`π`の規則を評価して得られる正規形が`p`であることを意味する。

**命題。** `ν`が全仮定を満たし、`check(π,G,H)=p`なら`⟦p⟧ν≥0`である。

**証明。** 証明項`π`の構造帰納法。

- nonnegative_constant：検査した有理数が非負なので結論はその条件である。
- premise：検査されたindexの仮定を使う。範囲外indexは証明項ではない。
- square：任意の実数/有理数/整数の二乗は非負である。
- sum：帰納法で非負な二値を加える。
- product：帰納法で非負な二値を掛ける。
- equality_multiple：`⟦h_j⟧ν=0`なので、任意の多項式`r`について積は0であり非負である。

正規化は同じ単項式の有理係数を加え、零項を除き、順序を正規化する操作である。有限和と積の分配則から、多項式の値を変えない。従って算出した正規形とgoalが一致すれば、同じ非負性を得る。任意axiomや外部成功フラグには規則がなく、導出を作れない。□

正規化されたデータ表現の妥当性は検査対象である。同じmonomialを異なる順序で二重登録し、辞書化で片方を消す表現は受理しない。実際のPython kernelの健全性は、この手証明とテストに基づくが、他の小さな証明kernelで機械検証したものではない。

### F2-07. source由来のsymbolic pathの正確性

**命題。** 型付きのloop-free owner bodyを、初期state変数`old.x`と引数`arg.y`からsymbolic実行した各成功pathは、そのpathのguard条件を満たす実行について、実際の局所変数、最終state、返値と等しい多項式を与える。全成功実行はいずれかのpathに含まれる。

**証明。** 初期symbolic環境は、それぞれの実値を評価`ν`で代入すれば実環境になる。各assignで、F2-02の純粋式の評価と、symbolic式の代入が一致することを式の構造帰納法で示す。算術は多項式の加減乗算、変数は環境の一致、単項minusは符号の反転である。

`a<b`の真枝は整数上で`b-a-1≥0`、偽枝は`a-b≥0`である。`a≤b`も同様に分けられる。従ってifは正確な二つのpath条件へ分割され、実行が進む一方の条件が成立する。assert失敗は成功pathに入らず、成功時は条件を仮定へ追加してよい。returnは実際の返値式を専用の論理変数`result`へ対応させる。

path数は有限構文の条件分岐数から有限である。文列の長さについてこの対応を帰納的に保存し、結論を得る。返値の論理変数を同名の局所変数で上書きしてはならず、これは実装の修正済み境界である。□

### F2-08. source-boundな部分契約の健全性

**命題。** `verify_export(op,I,Q,π)`が、現在の型付きsource、現在のschema、固定された`I,Q`に対して成功したなら、`I`から開始して成功したowner実行の結果は`I∧Q`を満たす。失敗したowner実行は状態を変更しない。

**証明。** 受理された証拠には、同じoperation、契約、不変条件のidentityがあり、全symbolic pathと全goalに一つずつ証拠がある。実行が成功したとする。F2-07により、その実行に一致するpathがあり、その仮定が真である。各goalの証拠へF2-06を適用すれば、最終stateの全不変条件と返値のpostが得られる。assert失敗時の非変更はF2-02による。

sourceの変更、pathの欠落、goalの欠落、異なる不変条件の証拠は、identityや数・正規形の一致を満たさず拒否される。decodeした証拠がこの照合を迂回する規則はない。□

この命題は部分正当性である。実行前提を強めて拒否を増やす変換の排除には、次の定理が必要になる。

### F2-09. TotalBodyと成功範囲を保つ交換

**命題。** `verify_total(op,I,P,Q,π)`が成功したとする。開始時に`I∧P`が成立すれば、このfinite bodyはassert失敗せず終了し、`I∧Q`を満たす。旧操作と新操作が同じインターフェース、同じ`P,Q,I`を持ち、それぞれこの証拠を持つなら、当該開始条件で旧bodyが持った成功保証を新bodyも持つ。

**証明。** F2-07のsymbolic対応を使う。ただしassertでは、その条件を仮定へ加える前に、既存のpath仮定から条件を導く証拠を要求する。開始時の仮定は`I∧P`から真である。ifで選んだguard条件は実行によって真である。各assertは、その手前までの仮定が真なのでF2-06で真と分かる。その後だけ条件を追加してよい。有限のbodyは停止し、全経路でtyped returnがあるため、必ず成功返値へ達する。最後のgoalの証拠で`I∧Q`を得る。

交換では、old interfaceで許された開始状態が同じ`I∧P`を満たすので、新証拠へ適用できる。Partial証拠へのdowngradeはこの推論を与えず、受理しない。前提を弱める一般的な交換なら`P_old⇒P_new`の別証拠が必要だが、prototypeは前提のexact一致に限定する。□

**反例。** `return abs(x)+1`と`assert x<0; return abs(x)+1`は、ともに成功時の結果が正である。しかし`x=0`での成功は保存されない。Partialだけではこの差を検出できない。

### F2-10. owner invariantの全mutator閉包

**命題。** owner Oの初期stateが`I_O`を満たし、`I_O`の自由状態変数がOに属するとする。Oの全write操作がF2-08の保存証拠を持ち、追加・交換・移行・importも同じ条件を満たすとき、任意の有限実行prefixのOの可視stateは`I_O`を満たす。

**証明。** prefix長の帰納法。初期は仮定。局所計算、輸送、権限変更、別ownerの作用はOの対象stateを変更しない。Oの操作が失敗すればF2-02で不変、成功すればF2-08で保存される。新しいmutatorは公開前に同じ保存検査を通り、proof-freeな別経路へは登録できない。状態移行とimportも検査してから有効化する。従って最後の一歩の各caseで不変条件を保つ。□

`I_O`が別ownerの可変stateに依存する場合、別ownerのstepはframeではないため、この証明は成立しない。それを型検査で拒否するか、別のrely/guaranteeまたは共同更新の定理が必要である。

### F2-11. 不変の値を介した局所理論の接続

**命題。** 提供側の成功返値`v`について`Q_1(v)`が保証され、sourceに結びついたbridgeが同じ`v`を利用側へ渡し、`Q_1(v)⇒P_2(v)`が検査されたなら、利用側の不変引数に関する前提`P_2`が満たされる。

**証明。** 提供側の証拠にF2-08またはF2-09を適用して`Q_1(v)`を得る。bridgeのdataflow検査は、再解析したclient sourceで、その返値を別値へ上書きせずcalleeへ渡すことを要求する。値はimmutableなので、間に別のstateが変わっても`Q_1(v)`は変わらない。含意の証拠へF2-06を適用して`P_2(v)`を得る。□

calleeの現在state条件、使用権限、資源枠までこの命題で解消したとは言わない。過去のstate snapshotについての命題を現在stateの前提へ変換するには、version一致または干渉安定性の別証拠が必要である。

## C. 使用権限、輸送、障害

### F2-12. 証拠とauthorityの分離、現世代の使用検査

**命題。** ownerで新しく実行された作用は、その使用線形化点で、真正な要求、現在のoperation/contract/binding epoch、現在の主体とgrantに対する許可を持つ。証明・compile・source addition・transport sessionの成立だけから作用権限は生じない。

**証明。** owner作用を含む規則は`service`の受理分岐だけである。その前提には発行済み要求との一致、checked operation identity、binding epoch、issuerが発行した現在のpermit、失効集合に含まれないことがある。証拠検査、CFG生成、message複製、結果の消費にはgrantを生成する規則がない。grant/revokeは別のauthority遷移である。古いepochは等号を満たさず、新規作用へ進めない。□

真正性・issuerの実装はideal modelの前提であり、SHA-256文字列を持つことだけがauthorityであるとはしない。Pythonオブジェクト内部を任意に改変する敵対コードは、この実行modelの境界外である。

### F2-13. durable決定とcrashを含むstate-only高々一回実行

**命題。** 要求keyを再利用せず、state deltaと決定が同じdurable recordで確定し、確認済みprefixが復旧後に保たれるとき、各keyのstate-only commitは高々一回である。cache適用後または応答消失後の再送でも二度目のstate更新は起きない。

**証明。** durable logを基準にする。新規commit規則は、keyの決定が存在せず、floorより新しいことを前提にする。同じrecordへkeyとdeltaを一度記す。復旧はそのlogを再生し、keyの決定とdeltaを同じprefixから再構築する。したがって、recordが存在するkeyは以後新規commitの前提を満たさない。

確定前のcrashではrecordが存在せず、deltaも意味上未確定である。確定後・cache/応答前のcrashではrecordが存在し、復旧によって既存決定を得る。保存の成否が不明な例外後には、その不明なwriterを使い続けず、prefixを再読する。これを省略すると、実際には書かれたrecordを見失って同じkeyを追加できる。□

物理storageが確認済みrecordを失う場合、前提が破れる。実ファイルテストはその物理前提自体の証明ではない。

### F2-14. 結果消費と継続の高々一回の再開

**命題。** 発行前にkeyとwaiting継続を保存し、成功結果の消費と継続の前進を同じ局所recordへ置き、復旧時にそのprefixを使うなら、同じ結果から二度目の意味上の継続再開は生じない。

**証明。** 結果受理には、元の真正な要求と、現在waitingであるkeyの一致を要求する。消費時にwaitingを解消して次の環境/pcを保存するため、二個目の同一結果は前提を満たさない。消費前crashはwaitingを復元し、既存結果を一度消費できる。消費後crashは前進後の状態を復元し、同一結果を無視する。古いepochや失効したauthorityによる返値を、現在の成功結果として消費する規則はない。□

返値の現在公開権限が失われた場合、ownerの過去のcommitを取り消したことにはしない。requesterが不確定なまま待つ場合を残す。

### F2-15. actor/transportの有限trace refinementと条件付き進行

**命題。** F2-04のsource/CFG対応とF2-12–14のowner・継続規則の下で、遅延・複製・破棄を持つ真正なmessage-bag実行の任意の有限prefixを、同じ意味上の作用・結果を持つsource activity実行へ写せる。

**証明。** 関係には、対応する局所stack/env、同じchecked operation、同じ確定state、keyごとの決定、waiting継続を含める。message bagのコピー数は一致条件から除く。局所stepはF2-04へ対応する。送信・複製・破棄は確定stateを変えないのでsourceの零stepでよい。初回serviceは同じbody・現在state・authorityを使う同じ原子的作用へ対応し、複製要求はF2-13で新しい作用を作らない。結果消費はF2-14で同じ継続を一回進める。crash recoveryはdurable prefixの同じ抽象stateへ戻る。各stepで関係を保存し、prefix長の帰納法を使う。□

**進行の系。** 必要なauthorityとepochが保たれ、資源が足り、必要な要求・結果が最終的に配送され、service/continuationが公平に選ばれ、owner bodyが本profileの有限bodyなら、その要求は成功または宣言された失敗を一度消費できる。証明は、配送→有限service→配送→消費の有限段階を順に用いる。永久partition、無限のactivity、繰返す失効は前提外である。

### F2-16. 外部効果の不確定性と保守的な再実行禁止

**命題。** durableなStartを記録してから外部効果を呼び、結果を記録する方式では、crash後の同じ状態から、未呼出しと呼出し済みを一般には区別できない。本profileで、曖昧なStartから再呼出ししない限り、一つのkeyによる物理呼出しは高々一回である。

**証明。** 二履歴を作る。`Start;Crash`と`Start;Call;Crash`である。結果が保存される前なら、両方のdurable stateはStartだけで一致する。一方、外界での実行数は0と1で異なる。そのdurable stateだけの決定関数は両者で同じ選択をするため、再呼出しすれば後者の重複を避けられず、しなければ前者の未実行を避けられない。

本profileの規則では、Callへ入れるのは新しく取得した一回用のStart許可だけであり、復旧後のStartにはそれを再発行しない。OutcomeRetainedがある場合は返値のみを再送する。従ってkeyごとのCallは一回以下である。これは「必ず一回」の証明ではない。□

### F2-17. 退役floorによる結果忘却後の非再実行

**命題。** `(incarnation,sequence)`のkeyについて、floorは単調でdurableであり、`sequence≤floor`の要求は決定がなくても新規実行しないなら、古い結果を削除した後も同じkeyのcommitは増えない。

**証明。** floor以下のkeyは使用規則で拒否される。floorより上で決定が残るkeyはF2-13で新規実行されない。floor更新を先にdurable化してから結果を削除するので、crashによって「結果もfloorもない」中間状態へ戻ることはない。incarnationを再利用しなければ、別の実行のsequenceと衝突しない。□

ackが何をsettledと意味するか、未発行sequenceの扱い、長期に残るincarnation一覧のGCは別の境界である。物理logの圧縮をこの定理だけで保証しない。

## D. 構成変更と分散patch

### F2-18. source-boundなadditionと契約を保つreplacement

**命題。** additionが名前・型・owner・import・effect・保護不変条件を全体検査してから公開され、replacementが実際のschemaと登録済み契約の証拠を再検査し、影響する未完了作用をsettleしてから一つの新epochとして有効化されるなら、基本型・owner局所性・登録済み不変条件・export保証を保存する。

**証明。** additionは旧storeと旧操作を変更せず、新しいセルは初期型検査を通る。新しい操作は同じsource checkerを通り、保護stateを書ける場合はF2-10の条件を満たすまで公開しない。importは実際の署名・effect/failure境界と一致する。additionだけではgrantを発行しない。

replacementは新sourceを再解析する。新操作が参照/変更するセルのownerと型を、runtimeの実schemaと照合する。従って、同じ関数署名に別のセル型を埋め込むことはできない。全証拠を検査してから、codeとbinding epochとproof登録を有効化する。途中の証拠失敗では旧登録を変更しない。旧epochで発行済みの不明な作用は先にsettleし、新しい作用は新epochを持つ。invariantはF2-10、Partial/TotalのexportはF2-08/09が与える。□

ここでのreplacement検査は保守的であり、意味上安全なすべてのコード変更を受理する完全性は主張しない。

### D.1 patch machineの定義

participantは有限集合`N={0,...,n-1}`で、n>0とする。各participantのphaseは`Old,Prepared,New,Aborted`。coordinatorのdurable decisionは`U,C,A`、知っているdecisionはparticipantごとに`U,C,A`、YESの集合は`V`である。up/downはavailabilityだけを変え、durableなphase・decisionは変えない。

遷移は次のとおり。

| 遷移 | 前提 | 更新 |
|---|---|---|
| Prepare(i) | decision U、iがOld、必要なsource/state/authority検査成功 | iをPreparedにし、shadow/fenceをdurable化 |
| Vote(i) | iがPrepared、decision U | Vへiを加える |
| DecideCommit | decision U、V=N、確定権限有効 | decisionをCへ一度変更 |
| DecideAbort | decision U | decisionをAへ一度変更 |
| Learn(i) | 一意のdurable C/Aを真正に取得 | known_iへ保存 |
| Install(i) | phase_i=Prepared、known_i=C | 新しい局所state/code/epochをdurable化しphase_i=New |
| AbortLocal(i) | known_i=A、phase_iがOldまたはPrepared | phase_i=Aborted。Preparedなら自分のfenceだけを解放 |
| Crash/Recover | 任意 | availabilityだけを変える |

参照modelは、送られないmessageを「状態が変わらない待機」、複製messageを「同じ結論の再取得」として抽象化している。独立したpacket queueを列挙したモデルではない。真正性・非equivocation・durableな単一decisionは明示的な前提である。

### F2-19. 任意の有限cohortでのpatch decision invariant

**命題。** 初期`U,Old^n,V=∅,known=U^n`から到達するすべての状態で、次が成立する。

1. decisionがUなら、phaseはOldかPreparedで、i∈Vならphase_i=Prepared。
2. decisionがCなら、V=Nで、全phaseはPreparedかNew。
3. decisionがAなら、Newは存在しない。
4. known_i≠Uならknown_i=decision。
5. phase_i=Newならknown_i=C。

**証明。** 初期では直接成立する。各遷移について示す。

PrepareはUでOld一つをPreparedへ変え、Vやdecisionを変えないので1を保つ。他は前件が偽または不変。VoteはPreparedだけをVへ加えるので1を保つ。DecideCommitの直前は1とV=Nから全員Preparedであり、Cへ変更すると2が成立する。以前のknownにC/Aはないため4も保つ。DecideAbortの直前は1からNewがなく、Aへ変更しても3が成立する。

Learnは現在の唯一のdecisionを記録するので4を保つ。decisionはUから一度しか変更されないため、将来もその値は変わらない。Installはknown_i=Cを前提とするので4からdecision=C。PreparedをNewへ変え、2と5を保つ。AbortLocalも4からdecision=Aであり、Old/PreparedをAbortedにするだけなので3を保つ。Crash/Recoverは対象のdurable成分を変更しない。

全遷移で保存されるので、履歴長の帰納法で任意nについて成立する。□

**必要性の反例。** Prepared後のtimeoutだけでOldへ戻ると、YESを返したparticipantがOldになり1を壊す。その後、coordinatorは古いYESからCを決め得る。準備またはdecisionをcrashで消す変種にも、添付のBFSで反例がある。

### F2-20. patchの原子的抽象化と、部分適用の非露出

**定義。** 抽象epochを、decision=Cなら1、その他なら0とする。upでPreparedでないparticipantは、Old/Abortedなら0、Newなら1のstateを公開できる。Prepared/downは利用不能である。

**命題。** 利用可能なparticipantの公開epochは常に抽象epochと一致する。data移行が準備時に検査され、fence下でshadowが保持されるなら、Commitを一つの原子的な抽象patchとして扱える。

**証明。** decision UではF2-19(1)により公開可能なのはOldだけなので0。decision Aでは(3)によりNewがなく、公開可能なのはOld/Abortedで0。decision Cでは(2)により公開可能なのはNewだけなので1。

dataについて、Cの前はOldまたはfence下の旧stateを抽象stateに使う。Cの後、Preparedの成分には検査済みshadowを使い、Newの成分には実stateを使う。Cへの遷移で抽象的に全成分を新stateへ変える。Installは、抽象化で既に使用しているshadowを実stateへ移すだけなのでstutterである。Preparedの影響範囲は外から変更・読取されないため、この抽象化に反する実動作はない。Newになった後は、以後の通常更新を実stateから読む。古いshadowで上書きしてはいけない。□

別々のreadがCommitの両側にある場合は、二つの合法なreadであり、同じsnapshotとは限らない。

### F2-21. 停止境界の所有と、同時patchの局所安全性

**命題。** 各停止境界が取得したpatch identityへ束縛され、他patchが取得できず、その所有者だけが解放できるなら、別patchの中止は、準備済みの停止境界を破らない。

**証明。** 境界の所有写像`Lock(r)=p`を考える。取得は未所有のrだけに定義し、解放は`Lock(r)=p`の検査を前提とする。従って別identity qの解放規則はrに適用できない。未準備のpatchは取得したrを持たず、AbortLocalはそのrを消さない。取得・解放・crash再開の各stepで一意性を保つ。□

さらに、patchの完全なread set Dとwrite set Wが相互非干渉であれば、F0.1のframe/可換性を適用できる。これは端点だけのdisjointnessではない。graph到達性、authority、schema、freshness、共有counterも必要ならD/Wへ含める。

**実反例。** 参照modelの初期`AbortLocal`は、そのpatchがPreparedでない場合にも同じdependency集合のfenceを消していた。二つのpatchを作るテストで再現し、所有identityとPrepared条件を加えた。

### F2-22. patchの条件付き進行と、blockingの位置

**命題。** coordinatorとparticipantが最終的に回復し、必要なmessageを最終的に取得し、準備に必要な影響範囲・authority・資源が安定して得られるなら、未決定のpatchをCommitまたはAbortへ決め、すべてのparticipantをNewまたはAbortedへ到達させる実行がある。decisionが既にC/Aなら、同じ判断に従って解放できる。

**証明。** 全nodeを回復した状態を考える。Uで全準備が可能なら、有限個の未準備participantを順にPrepareし、未収集のVoteを集め、Cを決める。準備が不可能ならUからAを決める。C/Aがある場合、有限個の未学習participantへその値を届ける。CではすべてPrepared/Newであり、未適用をInstallできる。AではOld/PreparedをAbortLocalできる。各処理で未処理participantの数が減るので有限である。□

この命題は無条件のfairness保証ではなく、指定された安定・回復条件の下での進行である。coordinatorが永久に失われる場合、Preparedを勝手に解放できない。複数patchが資源を待ちながら取得する方式では、別のdeadlock回避条件が必要である。

### F2-23. 検査されたsource移行とlive actor機械の接続

**命題。** 移行のsourceが実schemaと一致し、owner局所で、D/Wと不変条件保存が検査され、PrepareからInstallまでDへの書込みとWを使う作用をfenceし、旧pendingをsettleし、affected operationのbinding epochを進めるなら、F2-20の抽象化に実際のowner stateとactivity requestを対応づけられる。

**証明。** Prepareでは現version/epochを再確認した同じstateにF2-02/08を適用してshadowを計算する。fenceにより、その計算の依存stateは変化しない。保護不変条件の変数もDへ含まれる。F2-19/20によりCが決まるまでnewを公開せず、決まった後にoldを新しい作用へ使わせない。

Installでshadow deltaとepochを一つの局所記録へ置く。停止中に古いepochで発行された要求は、その後の使用検査で不一致になる。旧pendingを無断で新bodyへ渡すこともない。epochの更新前に成功した作用は履歴として残り、後の不一致は過去の更新を消さない。fence外の局所計算や非干渉なowner作用はframeとして扱える。以上により、具体なInstallやCrash/Recoverを挟んでも、対応関係を保存する。□

patch権限の有効性には、予約をissuerが維持する、またはCommitと現在権限の検査が同じ直列化境界にあるという追加条件が必要である。これは`live_patch.py`の予約profileが選んだ契約であり、無条件の即時失効を証明したものではない。

## E. 保存、関係、観測

### F2-24. quiescent fresh importの型・不変条件・権限分離

**命題。** 完了したactivity、upかつunfencedなownerを持つactor構成をcaptureし、source、body交換列、実際のstate型、全保護不変条件、登録exportを再検査し、fresh instanceへimportすると、当該actor成分の初期WFを満たす。旧grantや旧pendingだけを根拠に、新instanceで作用できない。

**証明。** source再検査で同じ型・配置のregistryを得る。交換列を再生し、保存した実stateを新registryのschemaへ照合するので、起動時の旧bodyへ戻ることはない。全不変条件を実stateで検査し、mutatorとexportの証拠を再検査してから有効化するためF2-10を再開始できる。fresh instance identityは既使用と異なる。authority/pending/patch privilegesは空であり、旧要求はそのidentityと現在grantを満たさない。従って新しいadmissionなしに旧使用を続けられない。□

このimport検査は、well-formedな保存候補を新instanceに作る検査であり、archiveが過去の特定時点から改変されていないという真正性証明ではない。元snapshotとの対応を保証する用途では、外部で保持した信頼できるmanifest/署名との照合も必要である。

完全なsystem snapshotには、登録された全componentの保存義務を含める必要がある。actorだけのcaptureから、外部effectやrelation stateまで保存済みとは結論しない。現instanceのdurable resumeとfresh importを同じ操作にしない。

### F2-25. relation DAGの局所評価とfallbackの分離

**命題。** 有限の非循環なrelation DAG、同じbinding/lineage、適合したanchor incarnationと共通の論理frontierを持つ同じsampleから、owner側の数学的評価とconsumer側のtopological評価は同じ値を返す。sample不足や不一致は、semantic bindingを変更しない。

**証明。** DAGのtopological順序に関する帰納法。anchor葉では同じsample値と一致するincarnation/frontierを使う。定数は同じ。加算と有理数倍は先行頂点の帰納法と演算の合同性から同じ値を得る。共有部分は同じ値を再利用するだけで意味を変えない。

consumerの投影関数にはbindingを変更する規則がなく、不一致時は値を返さずgap/stale/visibilityの理由を返すだけである。semantic invalidationは別の認可された遷移でcursorを前進させる。任意有限chainの単調性とfresh reacquireの分離はF0.1 T18とfresh reacquireの別lineage規則による。

labelは入力だけでなくdefinition自体もjoinするため、definitionが高labelなら低観測値として公開しない。□

sampleの物理時刻の同期や実rendererの数値誤差は、この定理の外である。

### F2-26. 逐次commandのtermination-insensitive低trace非干渉

**定義。** 観測上限o以下のセル値が一致することを`σ≈_oσ'`とする。emitは(label,value)を持ち、o以下だけを順序を保って射影する。pc labelは条件labelとjoinし、assign/emitには`pc⊔valueLabel≤targetLabel`を要求する。

**補題：high-pc confinement。** `pc≰o`で型付けされたcommandの、終了する実行は低stateを変更せず、低emitを持たない。

**証明。** assign/emitでは規則からtargetLabel≥pcなので、そのtargetがo以下ならpc≤oとなり矛盾する。seq/if/whileはpcが下がらず、終了した各部分実行へ帰納法を適用する。□

**主命題。** 同じtyped sequential commandの二実行がともに終了し、初期stateが`≈_o`なら、最終低stateと低emit列が一致する。

**証明。** command構造と、終了導出の長さについての整礎帰納法を使う。低assign/emitでは、式のlow非干渉(F0.1 T04)により同じ値である。高assign/emitでは低側を変えない。seqでは前半の帰納法で低stateとtraceをそろえ、後半へ適用し、同じ列を連結する。

ifの条件がo以下なら、式非干渉で同じ枝を選び、その枝へ帰納法を適用する。条件が高なら、選ぶ枝が違っても双方high-pcであり、confinementから低state/traceを変えない。

whileの条件が低なら、対応する反復点で低stateが一致し、同じguardを得る。guardが真ならbodyの終了導出は全体より短いので帰納法を使い、次の反復へ進む。偽なら両方終了する。guardが高ならbodyはhigh-pcであり、何回反復しても低state/traceを変えない。両実行が終了する仮定から、その後の継続へ同じ低stateを渡せる。□

片方だけの非停止、終了時刻、CPU、queue length、分散schedulerやdynamic objectの有無の漏洩は除外する。

### CE-01. 一実行のrefinementでは非干渉を保存できない

秘密h∈{0,1}に対し、抽象仕様がどちらでも低trace集合`{[0],[1]}`を許すとする。具体実装の集合をh=0で`{[0]}`、h=1で`{[1]}`とする。各hで具体集合は抽象集合の部分集合だが、具体集合はhに依存する。

従って、trace inclusionと抽象側のpossibilistic非干渉だけから具体側の非干渉は導けない。秘密を含む分散実装には、二実行のscheduler/環境選択を対応させるrelationalな条件が別に必要である。

## F. 統合と完成境界

### F2-27. 選択profileの任意有限進化に対する安全性とtrace対応

**前提。** 初期にA–Eの型、owner invariant、source/contract binding、authority、durable record、fence所有の条件が成立する。実行は本書の型付きlocal/owner/transport/patch/recovery規則だけで構成する。source identityは正確に区別でき、hostが破壊したdurable recordや偽のissuerを正しいものとして与えない。

**結論。** 任意の有限prefixについて、基本WF、登録済みowner invariant、登録済みPartial/TotalBody exportの適合、keyごとの新規state-only commitの高々一回性を保つ。CFG/輸送/journal/patch内部stepを消去して得るtraceは、指定されたsource/契約の抽象機械で実行できる。

**証明。** 単一の対応関係を、source継続とCFG frame、ownerの確定stateとdurable prefix、current authority、source/contract/epoch、patch decisionとfence/shadow、登録済み証拠、relation定義の積で定義する。

初期はsource/check/admissionと実stateの不変条件検査により成立する。最後のstepで場合分けする。

- local計算/call/return/spawn/join：F2-01–05で型と継続の対応を保存する。
- owner作用：F2-02、08–10、12–13で型、frame、不変条件、authority、決定を保存する。
- 輸送/消費：F2-14–15で意味作用を増やさず、同じ結果に対応する。
- library追加/交換：F2-18でregistryと証拠の対応を保存する。
- patch内部step：F2-19–23でshadow/fence/epochと原子的な抽象変更に対応する。
- crash/recovery：durable prefixを基準にF2-13/14を使う。fresh importは別instanceの初期状態としてF2-24を使い、旧instanceの未完了作用を偽って継続しない。
- 関係の評価：F2-25で同じ定義とsampleから同じ値を得て、意味stateを無断で変えない。

各caseに対応する抽象stepまたは零stepがあり、条件を保存するので、prefix長の帰納法で結論を得る。これは未来の全patchを列挙する証明ではなく、受理規則からの保存である。□

**範囲。** F2-26の秘密trace非干渉をこの結論へ無条件に加えない。外部効果の高々一回性はF2-16の別profileを使う。実装していないcomponentを暗黙にこの積へ入れない。全Python/Rust/QUICの機械証明、全体の非停止性、一般の任意論理の健全なimportは結論に含まれない。

### F.1 「小さい骨格」の具体的意味

この候補の小ささは、あらゆることを未解釈の`Safe`述語へ押し込めることではない。具体的に、source checker、owner評価、証拠kernel、epoch使用、journal commit、fence/decision規則を定義し、それぞれの保存を示した。

別の局所理論を導入するときは、その理論が返す保証を共通意味へ解釈し、必要な状態依存、authorityとは別の仮定、実行効果、観測を明示する。既存証拠言語へ翻訳できる場合はkernelを変更しない。翻訳できない理論は、名前だけを登録して公理にせず、別の解釈と保存証拠を要求する。

このinterface一般化の完全なmeta言語は未実装である。したがって、この条件を述べたことだけを「任意理論の導入が完成した」としない。今回の三つの局所理論の具体例と、source-boundな算術証拠の経路が、その最初の実現である。

### F.2 機械化へ持ち込むときの単位

最初の単位は、source/CFGの小ステップ関係とF2-01–05、次に証拠kernelとsource VCのF2-06–11、そしてdurable記録・patchのF2-12–24である。全部を一つの巨大な定理へしてから証明する必要はない。

各定理の前提は、別モジュールの定理、sourceの明示契約、またはhost仮定のどれなのかを保持する。実装が守ることを確認していないhost仮定を、証明済みの補題へ言い換えない。
