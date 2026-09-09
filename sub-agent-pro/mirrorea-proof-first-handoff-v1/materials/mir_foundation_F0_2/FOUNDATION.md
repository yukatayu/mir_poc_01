# Mir基礎理論 F0.2 — 合成、継続実行、検査可能な進化

## 成果の位置

F0.2は、F0.1の個別の保存則を、通常のソース、局所理論の証拠、非同期実行、障害後の再開、実行中の構成変更へ接続する研究候補である。**選択した実行・検証プロファイルには具体的な定義、手証明、参照実装、反例がある。既存Mir実装全体の検証済み仕様、最終的な型体系、Canonの採用記録ではない。**

F0.1の本文と台帳は`theory/F01_BASELINE.md`、`theory/F01_LEDGER.md`へ変更せず保持した。証明の詳細は`theory/PROOFS.md`、成立範囲は`theory/THEOREM_LEDGER.md`、実装への接続条件は`IMPLEMENTATION_PATH.md`、実行記録は`VALIDATION.md`にある。

照合基準はリポジトリ`yukatayu/mir_poc_01`の`f6aae7ca277690ac558a08f124e414ae2d2d35ad`である。リポジトリは変更していない。既存THM/OBLの状態も変更しない。F0.1との照合差は`AUDIT.md`へ記した。[R1–R3]

### F0.1から実際に進めたこと

| 領域 | F0.2の具体的な増分 |
|---|---|
| 通常のコード | 条件分岐、while、第一階の再帰・相互再帰、関数呼出し、同一locusのstructured spawn/join、通常の代入と状態参照を検査し、CFGとowner別の操作を生成する。 |
| 局所理論 | 検証器の成功フラグではなく、正確な多項式証明項を小さな検査器で再検査する。sourceからVCを再生成し、証拠の仮定・結論を外から注入させない。 |
| 合成 | 独立に検査した操作の型・効果境界と、値のrefinementの受け渡しを検査する。保護する状態について、すべてのmutatorを同じ不変条件へ従わせる。 |
| 進化 | 型が同じだけの交換を超え、登録された保証と成功条件を保つ交換を扱う。複数ownerの明示的なpatchについて、準備・確定・適用・障害・再開を具体化する。 |
| 保存と障害 | 状態変更と要求決定を同一のdurable recordに置くモデル、実ファイル上のframed journal、結果を忘れても再実行させないfloor、fresh instanceへの検査付きimportを作る。 |
| 観測と関係 | 固定schemaの逐次commandについて低観測traceの非干渉を証明する。別の実行モデルで、任意の有限な関係DAGを保持して利用側で評価する。 |

`.mirx`は研究用の仮記法であり、現行Mirのgrammarではない。Pythonのparserで構文木を得るが、入力ソースをPythonとして`eval`／`exec`しない。文字列としてのsourceを正とし、受け入れる際に再検査する。

## 1. 変えない目的

目的は、通信APIや低水準のイベントを先に組み立てることではない。計算、状態、関係、作用を記述し、それを実現する配置別のコードと連携を導くことである。World、Room、Avatar、SNS、3次元座標は基礎のprimitiveではない。プログラムの追加・交換・撤去は、最初の起動後にも起き得る。[R1]

「やりたいことを書けば動く」は、未指定の意味をコンパイラが勝手に選ぶことを意味しない。共有状態のどの版を使うか、成功と失敗をどう区別するか、誰が作用を許すかは意味の一部である。実現機構は生成し、必要な意味の選択は宣言・契約・最小のアノテーションへ残す。

F0.2の中心は、次の四者を同一視しないことである。

1. 実行意味：何を計算し、何を変更したか。
2. 証拠：どの前提から、何が示されたか。
3. 権限：現在の主体に、どの作用が許されるか。
4. 実現：どのプロセス・通信・記憶装置が、その意味を実現するか。

ある数が正であることを証明しても、メモリ確保や外部I/Oを許すgrantは得られない。逆に、操作権限があることは、操作がアプリケーション不変条件を守る証明ではない。

## 2. 一つの共通意味と、複数の保証範囲

### 2.1 実行構成

抽象構成を次で表す。

\[
S=\langle\Sigma,\sigma,F,A,G,L,H,B,J,D\rangle.
\]

- `Σ`：状態の型・owner・世代、および操作インターフェース。
- `σ`：owner別の状態。
- `F`：activityの局所環境、継続、呼出しstack、子activity、未完了要求。
- `A`：現在の主体・grant・失効・権限世代。
- `G`：存在、モジュール、admissionなど、意味を区別した依存関係。
- `L`：使用時に検査するhandle、fallbackのlineageとcursor。
- `H`：意味上の出来事、依存、実際の結果と不確定状態。
- `B`：パッチの候補、依存stamp、停止境界の所有、確定判断。
- `J`：状態・決定・構成のjournalと、そのdurableなprefix。
- `D`：ownerが保持する関係定義と、許可された提示入力。

この積を一つの巨大な中央データベースとして実装せよという意味ではない。証明で同じ対象を指すための数学的な構成である。`Engine.issued`等の大域写像も、モデル内の完全性・由来検査の表現であり、実装で全世界の一台へ集める要求ではない。

各時点の構成は有限とするが、識別子の母集合、activityの反復回数、将来のモジュール数には、事前に固定した上限を置かない。現実の資源不足は別の契約であり、実行driverの試験用上限と意味上の失敗を混同しない。

### 2.2 この版で選択したプロファイル

| 名前 | 選択した意味 | 選択しなかった意味 |
|---|---|---|
| 計算 | 数学的整数、Bool、第一階の関数、非停止も許すactivity | Int64/Floatの全挙動、高階closure、一般の依存型ソース |
| owner操作 | 一つのownerの有限・loop-freeな状態遷移 | 一般の複数owner transaction、owner内での任意の外部I/O |
| 通常の遠隔read | 一回の参照ごとにownerで得る不変snapshot | 暗黙の分散snapshot、常に最新・無遅延な共有メモリ |
| message | 遅延・複製・破棄を許す、由来と内容が検査されたmessage | 任意の敵対hostが正しく動く保証、暗号方式の証明 |
| patch | 明示的な有限cohort、停止境界、durableな一意の確定判断 | partition中も必ず進む非停止更新、全世界の一律同期 |
| 復旧 | 現instanceのdurable prefix再生、またはfresh instanceへのimport | 古いsaveを現世界の現在と偽って上書きすること |
| 公開 | actorの`.mirx`実装はpublic scalar専用 | actor全体の秘密情報・タイミングの非干渉 |

read方式、patchの可用性、patch権限の予約は**研究上の候補選択**である。現行Canonへの無断採用ではない。とりわけ、宣言的な関係を毎フレームsnapshot RPCへ変換する必要はない。第11節の関係経路は定義を保持し、別のread/presentation契約を実現する。

### 2.3 不変条件を追加するときの原則

基本WFは、型整合性、owner局所性、現在世代での使用、要求決定の重複排除、整合した継続、対象とコードの対応、停止境界の所有等の積とする。アプリケーション不変条件は追加の述語`I`として扱う。

`I`を登録した後は、同じ状態を書ける**全操作、全移行、全import**が`I`を保存しなければならない。最初に検証したモジュールだけを拘束し、後から追加する普通のコードが自由に同じ状態を書ける構造は認めない。これが局所理論を外側のコードへ接続する最小の統制である。

## 3. 通常の記述と意味

### 3.1 一つの例

```text
stock: Cell["B", int] = 10

@owner("B")
def take(amount: int) -> int:
    assert 0 <= amount
    assert amount <= stock
    stock = stock - amount
    return stock

@task("A")
def session(n: int) -> int:
    total: int = 0
    i: int = 0
    while i < n:
        value: int = take(1)
        total = total + value
        i = i + 1
    return total
```

`take`はBの現在状態で実行する有限操作である。`session`はAのactivityであり、呼出しを出した後、その結果を待つ継続を持つ。要求・結果の辺、要求key、結果を代入して再開する位置は、コードから導く。

`stock = stock + 2`をactivityに直接書いた場合も、右辺をAで古い値として計算してblind writeするのではない。対象ownerのBで、現在の`stock`を読む一つの操作へ展開する。activityの不変な局所変数だけを引数として渡す。

別ownerの可変状態が同じRMW式に現れた場合は、このプロファイルでは拒否する。一度局所変数へsnapshotを受け取った後で使用する記述なら受理できる。この拒否は「通信を手書きせよ」という要求ではなく、同時性・鮮度が未指定な計算を勝手に原子化しないためのものである。

### 3.2 readの透過性の限界

`v = stock`は、表層では通常の代入だが、意味上はBのある線形化点で得た不変の整数をAへ渡す。以後の`v-v`は0である。一方、このreadプロファイルで`stock-stock`をactivityへ書くと二回のreadになるため、間に更新があれば0とは限らない。

これは最終Mirの既定方式を選んだという意味ではない。将来、式単位のcoherent readやmaintained relationを既定とするなら、そのread handlerとelaboration規則を別に定義する必要がある。今回のcompiler保存定理は、この点を「遠隔readは純粋」と偽っていない。

### 3.3 再帰と並行構成

関数の引数と結果の型を宣言し、呼出し先の署名を先に登録する。再帰・相互再帰の実行を型検査時に展開しない。activityは無限に反復してよい。型検査の終了と、そのactivityの停止は別である。

`spawn`は同じlocusの子activityを生成し、`join`はその結果を待つ。Futureは現在の実装ではactivity内部に限り、通常の整数として偽造できず、外部へ返す型でもない。親は子が残ったまま成功終了せず、未joinの子の失敗も無視しない。遠隔activityの自由な移動・spawnは、参加・資源・権限の別契約を必要とし、この版では未実装である。

### 3.4 型規律と完全性の意味

関数環境`Δ`、セル環境`Σ`、局所変数の固定型`Γ`、確実に初期化済みの集合`D`について、

\[
\Delta;\Sigma;\Gamma;D\vdash e:\tau,
\qquad
\Delta;\Sigma;\Gamma;D\vdash s\Rightarrow(D',r)
\]

を定義する。`r`は当該経路がreturnしたかを表す。

条件分岐の後で使用可能なのは、継続する両枝で初期化済みの変数である。片枝がreturnするなら、継続する枝の集合を使う。while本体だけで初期化された変数は、零回反復の可能性があるのでループの外では確実に初期化済みとしない。

型・definite assignment・配置条件の検査は、有限のソースと署名環境について終了する。宣言的規則を第A節で固定し、それに対する相対的な健全性・完全性を示す。意味上安全なあらゆるプログラムを受理する完全性ではない。また、多項式証明の**探索器**が完全という意味でもない。

## 4. ソースから配置別実行への翻訳

### 4.1 意味の中間形

activityは、概念上、返値、内部計算、型付きの操作要求と継続からなる。再帰や非停止を許すため、この構造は有限な木に限定しない。

\[
T\,A = \operatorname{Ret}(A)
\mid \operatorname{Tau}(T\,A)
\mid \operatorname{Vis}(o,a,\operatorname{Result}(o)\to T\,A).
\]

失敗は`Result`の明示的な分岐であり、未着の結果は「失敗値」ではなく継続が待機した状態である。この整理はITreesの効果と継続の考え方を参照する。ただし本成果はITreeのCoqライブラリを組み込んで証明したものではない。[S1]

### 4.2 CFG生成

実行機は、局所register、program counter、呼出しstackを持つ。各ソース式をfreshな一時registerへ計算し、条件はbranch、反復はback edge、関数呼出しはreturn先を持つframeへ変換する。

owner操作には、引数registerとソース位置を持つ`request`命令を生成する。生成先はchecked signatureのownerから決まり、deploymentはそのlocusを物理endpointへ対応させるだけである。

普通のセル代入では、その関数とソース位置から一意の操作を生成する。一行に二つの文を置いた場合も、関数・行・列の組を区別する。初期モデルでは行番号だけを使ったため、同じ行の二代入が同じ操作へ潰れる反例が生じた。修正と検査記録を残した。

### 4.3 compilerの保存対象

ソースの制御状態と、CFGのprogram counter、stack、局所環境を対応づける。式の途中で必要なsnapshotや呼出しが生じた場合、対応関係は未完了の継続も含む。

意味を持たないregister移送やjumpは内部stepへ対応し、操作発行、owner実行、結果消費、fork/joinは同じ意味上の出来事へ対応する。再帰については、実行を無限展開せず、署名とstack invariantを用いた有限prefixの帰納法で示す。

この版のcompiler定理は、定義した抽象source/CFGに関する手証明である。Pythonのparser・データ型・実装バグがないことを、proof assistantで証明したものではない。独立のAST制御interpreterとの比較を実施したが、owner操作の純粋な評価関数は共有しており、完全に独立な二実装とは呼ばない。

## 5. 局所理論のlibrary化と、証拠の検査

### 5.1 安全な拡張の最初の形

この版で具体化したのは、追加した理論がkernelの新しい公理になる方式ではない。

**未信頼の生成器が、既存の小さな証明規則へ展開した証明項を出す。受け入れる側は、その項とソースへの対応を再検査する。**

整数の区間規律、線形不等式、二乗非負の議論を、同じ証拠言語へ下ろせる。任意の新論理をこの言語へ表現できるとは主張しない。表現できない場合は、意味を変えずに`Unknown/Unsupported`へ戻すか、別の検証済み解釈を追加する設計が必要である。

### 5.2 多項式証拠kernel

有理係数の多項式を、整列した単項式と正確な係数の有限写像として表す。仮定は`g_i≥0`と`h_j=0`である。証明項の規則は次のみである。

\[
\frac{c\ge0}{c\ge0},\quad g_i\ge0,\quad q^2\ge0,
\quad\frac{p\ge0\;q\ge0}{p+q\ge0},\quad
\frac{p\ge0\;q\ge0}{pq\ge0},\quad h_jr\ge0.
\]

最後の規則は`h_jr=0`による。kernelは規則から得る多項式を計算し、必要な結論の正規形と一致するか調べる。任意の`axiom`、外部solverの成功フラグ、実行callbackは受理しない。

証拠生成の探索は未信頼で、不完全でよい。探索の失敗を証明の失敗・命題の偽と同一視しない。逆に、返した証拠は、生成器を再実行せずデータとして検査できる。デコードだけでは受理せず、sourceから検証条件を再生成する。

### 5.3 ソースとの束縛

証拠には、操作本体のidentity、契約のidentity、不変条件のidentity、全経路・全結論の証明を束縛する。symbolic executionは、old stateと引数に別の名前を与え、各代入で式を代入し、ifの枝条件とassert成功条件を追加する。

証拠の提出側が「この前提も成り立つ」と自由に前提を追加することはできない。VCはsourceと受け入れ側が指定した契約から再生成する。

実際の反例として、局所変数`result`が論理上の戻り値`result`を上書きし、`return -1`なのに正の戻り値の証拠が通る初期実装を再現した。論理の束縛を局所変数と区別して修正した。これも、証明とソースの対応が独立に必要であることを示す。

### 5.4 部分正当性と、成功範囲の保存

`Partial(I,Q)`は、`I`から実行して成功したなら`I∧Q`を得ることを示す。assertによる失敗は状態を変えない。これは、途中のassertを経路仮定に入れて証明できる。

一方、`TotalBody(I,P,Q)`は、owner操作の開始時に`I∧P`が成立すれば、その有限bodyがassertで失敗せず、結果を返して`I∧Q`を満たすことを示す。この場合、各assertを仮定する**前に**、現在の仮定からassertを示すVCが必要になる。

例えば`abs(x)+1`を、`assert x<0; abs(x)+1`へ交換しても、「成功すれば正」という部分契約は維持される。しかし全整数入力で成功するという契約は壊れる。型と成功時postだけの交換では、これを見逃す。

F0.2では、登録されたtotal契約をpartial証拠へ落とす交換、同じ署名のまま成功前提を強める交換を拒否する。実装上は同じ前提集合の維持を要求する保守的な方式である。一般の含意を用いた弱化・等価化は、追加の証拠で拡張できるが未実装である。

TotalBodyは、ネットワークが必ず到達することや権限が将来も有効であることを意味しない。計算の成功条件と分散系の可用性を分離する。

### 5.5 全mutatorの閉包

`I_O`はowner Oの状態だけを参照する。登録時に現在状態が`I_O`を満たすことを検査し、Oの状態を書けるすべての操作へ、`I_O`の保存証拠を要求する。新しいmutator、body交換、状態移行、snapshot importも同じ境界を通る。

Oだけが証明した不変条件が、別ownerの可変状態を仮定していたら、この閉包は成立しない。実際にAの不変条件がBのセルを含む候補を作り、拒否条件を追加した。複数ownerの共同不変条件を持つには、別の干渉規律か、明示された合意・資源移送の意味が必要である。

不変条件を後から変更する場合、既に公開した証拠の前提も変わる。その変更を通常のポリシー追加と見なして無断で適用しない。現実装は、そのような変更を明示的な再検証なしでは拒否する。

### 5.6 二つのlibraryの接続

`Positive.positive`は、結果`r`に対して`r≥1`を証明する。`Pool.allocate_size`は、正の長さとowner側の残量条件を用いて、残量の非負を保存する。

```text
from Positive import positive
from Pool import allocate_size

@task("Client")
def allocate_positive(x: int) -> int:
    n: int = positive(x)
    return allocate_size(n)
```

このclientは、提供側の内部sourceを使わず、署名から個別にコンパイルできる。論理的なbridgeは、受け取った同じ値が`allocate_size`へ渡され、`r≥1`から`n≥1`が得られることをsourceに結びつける。`n=-1`を挟む変更や、元sourceと異なるASTの差込みは拒否する。

ただし、正の長さの証明から、**現在の残量が十分であることまで得られるわけではない**。残量条件はownerで検査する。このbridgeは、calleeの前提の一部を正しく解消する具体例であり、全前提・全理論を自動接続するloaderではない。

F0.1の領域所有と環境安定性の様相は、別の局所理論として保持する。MTTは、より豊かなmode theoryの研究候補であり、今回は採用済みとはしない。[S4]

## 6. 非同期実行、権限、継続

### 6.1 要求の同一性

要求は、instance incarnation、activity identity、局所sequence、operation、body identity、contract identity、binding epoch、不変引数、主体、grant参照へ結びつける。接続番号やpacket順を、意味上の要求identityにしない。

送信前に、要求keyと結果を待つ継続を保存する。ownerは、現在の操作・schema・epoch・権限を検査し、guardとbodyを現在の状態で評価する。返された結果は、元の要求と実際の決定へ結びつき、同じ継続に高々一度消費される。

コードを受け入れることと、操作を使うgrantを発行することは、別の遷移である。子activityは親の主体で動いても、新しいgrantを生成しない。`grant_all`等は試験の明示的なtrusted setupであり、Mir sourceに公開されていない。

### 6.2 結果の不明性

ownerが実行した後、結果だけが失われる場合がある。要求元の未完了状態は、それを未実行へ書き換えない。

今回のstate-onlyプロファイルでは、明示的な再送に対して、保持した決定を再送できる。再評価はしない。権限・epochが失効した場合は、新しい使用を拒否する。ただし、既に起きた更新の事実を消さない。

これは「要求は必ず一度実行される」という保証ではない。要求が届かなければ零回であり、応答が失われれば要求元は知らない。旧結果の保持・再送と、新しい物理効果の再実行は別である。

### 6.3 継続の構造

呼出しstackは、型付き引数、返値の型、return registerを保存する。各requestは待機先registerを保存し、結果の型と由来を確かめて再開する。非停止activityは基盤の論理矛盾ではないが、親のstructured completionを妨げる場合がある。

親が成功する条件には、子activityの終了と失敗の伝播を含める。初期モデルでは未joinの子の失敗を無視して親が成功するケースがあり、修正した。取消しの一般プロトコルは未追加なので、未確定の子を勝手に「実行されなかった」と扱って親を完了させない。

## 7. durableな状態操作と外部効果

### 7.1 state-onlyの線形化点

一つのowner操作の書込みdelta、要求key、実際の結果を、同じdurable recordへ置く。recordの確定を線形化点とし、cacheの適用と応答送信はその後に行う。

record確定前のcrashは、操作の更新を残さない。確定後・応答前のcrashでは、再開時に同じdeltaと決定を復元する。同じkeyを再実行せず、既存結果を返すか、現在の使用条件に応じて拒否する。

`journal.py`は、長さ・payload・checksumを持つrecordを実ファイルへ書き、flush/fsyncし、途中のtailを切り捨てて再開する参照実装である。ただし、ハードウェア・filesystemのdurabilityそのものを証明していない。single-writer、trusted local storage、成功したfsyncが約束するprefix保存というhost契約が必要である。

### 7.2 忘却後の再実行を防ぐ

結果を永久に保持しない場合、incarnationごとに`floor`を保持し、それ以下のsequenceは、結果がなくても新規実行しない。floorをdurableにしてから古い決定をメモリから捨てる。

floorを進めるには、要求元がそのprefixをsettledとして保存したという、現在の契約に適合するacknowledgementが必要である。prototypeはそのackをtrusted入力として受け取る。ackの生成・真正性・失効は未実装である。

これはメモリ中の結果保持を減らす方法であり、物理journalファイルのcompactionを実装したという意味ではない。結果の永久再送を約束する契約と両立するわけでもない。`ForgottenButFenced`をどう扱うかは、保持契約の一部である。

### 7.3 外部効果では同じ方法だけでは足りない

外部I/Oの開始を記録しても、「記録後・物理呼出し前」と「物理呼出し後・結果記録前」でcrashすると、同じdurableな`Started`だけが残り得る。実行回数は前者で0、後者で1だが、再開側から見える状態は等しい。

この二つを区別できないまま再実行すれば、後者で二重実行になる。再実行しなければ、前者では未実行のままである。

F0.2の保守的な外部効果モデルは、`Started`以降の不明な要求を自動再実行しない。保持した結果だけは再送できる。追加の相手側idempotencyや確認・取消しプロトコルがなければ、この不確定性は消えない。

この点は、基盤の設計で隠すべき困難ではなく、型付き結果と観測で保持すべき事実である。外部providerを実装するたびに、新しい「exactly once」という標語で省略しない。

## 8. 動的な構成の合成

### 8.1 additionとreplacement

additionは新しい宣言と操作を全体検査してから公開する。既存の意味を書き換えず、既存の名前・instanceと衝突せず、依存・owner・型・輸入契約が適合していることを必要とする。新しいauthorityは別に与える。

replacementでは、基本署名の一致だけでは足りない。現在のセルschema、効果範囲、公開したpostcondition、成功範囲、不変条件を検査する。新しいsourceでセルをBoolとして再宣言し、旧runtimeのIntセルへ書く候補は、関数署名が同じでも拒否する。

ここでの宣言instanceは大域的に一意な名前を使う簡易profileである。独立libraryを任意回instance化する一般的な名前束縛・handle型は未実装であり、最終languageの表現力の確認項目に残す。

### 8.2 影響範囲

変更集合`W`だけでなく、型検査、不変条件、移行、権限、graph到達性の証拠が依存する集合`D`を持つ。対象とコードのincarnation、値やschemaのversionも含める。

F0.1の反例のとおり、変更する辺の端点が異なるだけでは、二つのDAG変更の独立性は得られない。否定的な到達性の根拠を追跡するか、共通のrank証拠等を使う必要がある。

影響範囲を過少申告する自由はない。導入する局所理論が独自の状態依存を持つなら、それも義務として境界へ公開する。

## 9. 明示的な分散patchのプロトコル

### 9.1 2PCを使う範囲

F0.2は、**patch activationの制御面に限り**、明示的なtwo-phase commit型のプロトコルを一つの実現候補として選ぶ。通常の代入を隠れた複数owner transactionに変えるものではない。coordinatorは当該patchのcohort内の役割であり、唯一のWorld rootや全世界の中央管理者ではない。

この方式はcoordinator障害時に停止し得る。その性質は古典的2PCでも知られている。[S2] 非停止性を得たと偽るために、タイムアウトを合意済みAbortへ変換しない。

### 9.2 手順

1. candidateのsource、移行、不変条件、完全な依存stampを検査する。
2. 各participantは影響範囲の未完了操作をsettleし、依存stampを再確認する。
3. その範囲を、このpatchが所有する停止境界として予約し、旧状態とshadow/new image、patch identityをdurableに保存する。
4. durableな準備の後にだけYESを返す。
5. coordinatorは全員の一致するYESと必要なpatch権限を根拠に、一意のCommit/Abortをdurableに決める。
6. participantはその判断の証拠を学習して適用する。Commitを知らない準備済みparticipantは、旧状態を通常処理へ公開せず待つ。
7. 各適用は状態・binding epoch・構成記録を同じ局所journalへ置く。その後に自分が所有する停止境界を解放する。

Crash後も準備状態と確定判断を保持することが安全性の前提である。半端な状態を「たぶん古い方が正しい」と推測しない。

### 9.3 抽象的な原子性

durable Commitの前は、抽象状態は旧状態である。Commit後は、まだ適用していないparticipantについてshadowを用いた新状態とする。

Commit後に実状態が旧のparticipantは、Preparedで停止しているので、旧状態を新たな操作に使わせない。適用済みのparticipantは新状態を使える。従って、利用可能な操作は、同じ抽象epochへ対応する。

ただし、別々の通常readがCommitの前後にまたがれば、その二readの値は旧・新を混在し得る。それは非原子的な二つのreadの合法な意味であり、coherentな複数owner snapshotを取得したことにはならない。

### 9.4 実行系への接続

`live_patch.py`は、sourceで書いた有限のowner移行を検査し、`engine.py`の実際の受付fence、状態、journal、binding epochへ接続する。

停止中に発行された旧epoch要求が、適用後に届いても、新世代として実行しない。影響範囲外の純粋activityは継続できる。失敗・中止で解放できるのは、そのpatch自身が取得したfenceだけである。

初期の接続実装では、準備していないpatchのAbortが、他のpatchのfenceを外してしまった。別の反例では、完了済みpatchの古いshadowが、その後の正しいowner更新を抽象観測から消していた。fenceの所有権と、shadowを使用するPrepared期間を限定して修正した。

### 9.5 権限と停止性について、選択を隠さない

この参照profileでは、準備済みpatchの権限予約が、当該patchの確定まで有効なものとして扱われる。準備後に要求された権限失効は、その予約の解決後に有効化する。これは**一般的な即時失効の意味と同じではない**。

実装候補は、issuer側でもこの予約を一意に保持するか、Commit判断と現在権限の検査を同じ信頼・直列化境界に置かなければならない。単なる事前のYESから現在権限が続いていると推測してはいけない。

この権限reservationを許さないポリシーでは、本profileをそのまま使えない。全participantがCommitを学ぶ前の即時取消しなどを要求するなら、別のAbort/Commit競合プロトコルを定義する必要がある。これはCanon採用前に明示的に判断する項目であり、理論の裏側へ隠すものではない。

### 9.6 干渉する複数patch

同じ停止境界へ二つのpatchを同時に入れない。参照実装は、予約済み範囲と重なる準備を拒否する。複数範囲を待ちながら取得する実装へ変更する場合は、共通の資源順序やwait-die等の別の進行規律が必要になる。

安全性は、他者が所有する停止境界を解放しないこと、各participantのYESがdurableな準備へ対応することに依存する。成功率や公平性は、それだけでは保証されない。独立した範囲の変更は局所的に進めてよいが、graph証拠や権限依存も本当に独立であることが必要である。

## 10. 保存・復元を三種類に分ける

### 10.1 現instanceのcrash recovery

現在のdurable prefixから、状態・要求決定・コード交換・epochを再生する。古い応答を受け取れなかったことを理由に、処理を再実行しない。requesterも、送信前のkeyと継続、消費後の位置をdurableに保持する必要がある。

参照actor machineでは、継続の保存を原子的な局所model transitionとして扱う。これを実際のOSプロセスへ持ち込む実装は未作成であり、局所journalの導入とその実現証明が必要である。

### 10.2 過去のチェックポイントの選択

F0.1の最小checkpoint completionとZ-path特徴づけをそのまま継承する。各保存点が既知の依存に閉じるよう、下限を伝播するアルゴリズムは、完全な有限入力について終了・健全・完全・最小である。

F0.2で追加した要求発行、durable決定、結果消費、patch準備と確定、コード交換も、選択する履歴の依存へ含める。未知の依存が後から追加される状態に対し、既知部分だけで完全なsnapshotを証明しない。

### 10.3 fresh instanceへのimport

別の操作として、停止済み構成のdataとcodeを、新しいinstanceへimportできる。これは過去を現在の世界へ上書きする操作ではない。

`capture`は、活動が完了し、ownerが起動中でfenceを持たないことを要求する。`restore_as_fresh`は、元sourceから再検査し、交換後の実際のcode、schema、型、局所不変条件、export証拠を確認してからfresh instanceを得る。

古いgrant、動作中のactivity、旧message、patch権限は持ち込まない。必要なら現在の権限側で新たにadmitする。旧履歴は歴史的記録であり、新しい使用許可ではない。

この関数は、今回のactor成分のquiescent snapshotである。別に存在するrelation subsystemや外部providerの未完了状態まで、すでに自動登録・保存したという意味ではない。完成したHostは、**登録済み全componentのcheckpoint契約を列挙し、不明な成分があれば完全snapshotとして拒否する**必要がある。

## 11. 宣言的な関係と観測

### 11.1 関係を値streamへ潰さない

関係を、anchor、定数、参照、加算、正確な有理数倍からなる有限DAGとして保持する。頂点のtopological orderで評価し、共有された部分は一回だけ計算する。

sampleにはanchorのincarnation、論理frontier、値、labelがある。選択したbindingの世代と一致し、必要なsampleが同じ契約上のfrontierを満たすときだけ評価する。sample不足は`PresentationGap`であり、ownerのbindingを勝手にfallbackさせない。

権限を伴うsemantic invalidationだけがfallback cursorを単調に進める。新しいlineageでのreacquireは別の認可された作用とする。

公開labelはanchorだけでなく、**関係定義そのもののlabel**も含めてjoinする。公開anchorに秘密offsetを加えた関係が、公開になってはいけない。完全な秘密情報flowのtheoremは、公開するbindingの存在・失敗・時刻も含む対象ごとに限定する。

### 11.2 逐次commandの低観測trace

別のIFC profileでは、式labelとpc labelを用い、低label状態への高labelの書込みと、低labelへの高pc下のemitを禁止する。

二実行がともに終了し、同じ低入力から始まるとき、低状態と低emit列が一致することを示す。高labelのloopは低state/emitを変更できないため、反復回数が違っても、終了する両実行では低観測列に影響しない。

ただし、片方だけが停止しない場合や、時刻・CPU使用量・message量からの漏洩は、この命題の外である。source actor prototypeはpublic-onlyであり、このIFC定理をそのまま秘密情報を含む全分散実行へ適用しない。

### 11.3 trace refinementだけから非干渉は得られない

抽象仕様が、秘密入力に関係なく低出力0と1の両方を許すとする。具体実装が秘密に従って0か1を選ぶ場合、各具体traceは抽象仕様で許される。しかし具体実装は秘密を漏らしている。

従って、一実行ずつのtrace包含と、抽象仕様の非干渉だけを合成して、具体実装の非干渉を結論してはいけない。二実行のscheduler・oracle・adversaryを対応づけるrelational refinement、または同じ低選択を保つ強い条件が必要である。

F0.2は、この反例を実行検査として含む。観測の保証を「全体の意味保存」という一語で過大にまとめない。

## 12. 統合定理と実装の責任

### 12.1 選択した安全性の統合

初期構成が型・owner-local invariant・現在の権限・世代・操作registry・journal・停止境界の整合性を満たし、次の遷移だけを使うとする。

- 検査済みactivityの局所計算・型付き呼出し・structured spawn/join。
- 現在使用条件を検査したowner操作と、原子的durable決定。
- 理想輸送の遅延・複製・破棄・真正な配送。
- 証拠を再検査した局所構成追加・交換、および第9節のpatch。
- 契約を維持するcrash recovery、または権限を持ち込まないfresh import。

これらの任意の有限prefixについて、基本WF、登録済みowner invariant、要求keyの重複実行禁止、登録済みexportの適合を保つ。具体な輸送・durability・patch内部stepを抽象化すると、同じsource/contractで許された意味のtraceへ対応する。

証明は、各stepの保存とsimulationを対応表に従って合成する。各stepに「正しいものとする」という未定義のoracleを置くのではなく、本書と`PROOFS.md`で示した具体的なguard、certificate規則、journal/patch遷移を使う。

**ただし、この統合定理は定義した抽象機械についての手証明であり、添付Python全体や既存Rust/QUIC全体を機械検証したものではない。** さらに、秘密trace非干渉や無条件のlivenessは、この安全性定理の結論へ含めない。

### 12.2 残るhost契約

| 境界 | 具体的に必要な約束 |
|---|---|
| transport/codec | 検査されたidentity・epoch・引数・結果を改変/曖昧化せず、真正性を確認してから意味上の操作へ渡す。 |
| local journal | state deltaとdecisionのcommitを分離せず、確認したdurable prefixをcrashで失わない。 |
| source/proof admission | 与えられた実行物と証明のsource/version/contractが一致し、未解決の義務を成功にしない。 |
| authority | logical grantの発行・失効・予約を、現在のissuer状態と使用点へ正しく結ぶ。 |
| patch coordination | YESより前にfence/shadowをdurable化し、一意の判断なしに復帰しない。他patchのfenceを外さない。 |
| observation | 値だけでなく存在・理由・因果参照も、そのobserverへ公開してよい範囲に制限する。 |
| resource/sandbox | sourceとproof producerの実行資源を制限し、未達を型付きの拒否/不明性として扱う。 |

これらは、全部が新しい数学上の未解決問題という意味ではない。一方、実装がその約束を満たす証拠なしに、「あとは実装するだけで保証される」とは言わない。

## 13. 実装までの道筋は、どこまで見えたか

**選択したfirst-order/public/finite-owner-operation/explicit-patchのprofileについては、基礎の構成と実行アルゴリズムを具体的に選べる状態になった。** 普通のコードを処理ごとの手書きharnessへ戻さずに動かし、証拠とauthorityを分離し、owner invariantを守り、状態移行を実行系へ接続する経路がある。

これは、最終Mir全体の基礎理論が確立したという意味ではない。特に次は別に残る。

- 高階closure、第一級の資源handle、一般のモジュールinstance化を含む表現力と境界。
- 任意の理論を導入するmeta言語と、その証明・意味の保守性。現在は小さな共通証拠言語へ翻訳できる具体例。
- 秘密を含むactivityと実ネットワークscheduler、Viewの全観測へ届くrelationalな情報流証明。
- nonblockingな分散更新、即時失効を持つ別の権限profile、Byzantineなownerの実行正しさ。
- 各host実装とモデルの対応、全文のproof-assistant機械化、独立した意味・証明レビュー。

これらをすべてI3-3へ追加すべきではない。既存の有限I3-3を区切った後、本成果のどの候補を採用し、どの保証をα版の必須条件にするかを対応づける。[R2–R3]

**理論上の新しい選択が必要な部分と、選んだ契約を実装・検証する部分を分けられるようになった**ことが、この版の「道筋」である。両者を、全体が完成したという言葉へ縮めない。

## 14. 新規性と既存成果

構造帰納法によるcompilerの対応、証明を提出して再検査する構造、Hoare規則、情報流のpc規律、write-ahead logging、2PCの安全性・blocking等は、既存の研究系統を利用する。新発見と主張しない。[S1–S6]

本成果の寄与は、Mirの目的に対し、通常の記述・検証境界・使用権限・障害・進化をつなぐ具体的なprofileを作り、接続が壊れる反例と、修正後の条件を同じ成果物へ記したことである。強い一般的新規性の主張は、この候補の機械化・独立レビュー・より広い比較を要する。

## 15. 読み方

全体の候補は本書、定義と手証明の詳細は`theory/PROOFS.md`、証明範囲と証拠の区分は`theory/THEOREM_LEDGER.md`を読む。再現は`python3 run_all.py`で行う。実行環境の制約、SMTや証拠kernelの信頼範囲、実行されなかったものは`VALIDATION.md`にある。

本書の命題はF0.2内の識別子であり、既存MirのTHM/OBLを追加・受理したものではない。F0.1から継承した命題も、その前提を省略して全体系の結論へ拡大しない。

## 参照

[S1] Xiaほか, *Interaction Trees: Representing Recursive and Impure Programs in Coq*, POPL 2020, arXiv:1906.00046. https://arxiv.org/abs/1906.00046

[S2] Gray・Lamport, *Consensus on Transaction Commit*, 2004. https://arxiv.org/abs/cs/0408036

[S3] Dalla Predaほか, *Dynamic Choreographies: Theory And Implementation*, LMCS 2017. https://arxiv.org/html/1611.09067v3

[S4] Gratzer, *Normalization for multimodal type theory*, LMCS 22(1), 2026. https://lmcs.episciences.org/17744

[S5] Pierceほか, *Software Foundations, Programming Language Foundations*, Hoare logic / typechecking chapters. https://softwarefoundations.cis.upenn.edu/plf-current/

[S6] Necula, *Proof-Carrying Code*, POPL 1997, DOI:10.1145/263699.263712. https://doi.org/10.1145/263699.263712 。一次project紹介: https://www.cs.cmu.edu/~fox/pcc.html

[R1] Mirrorea Canon, `NORTH-STAR.md`。 [R2] `theory/11-metatheory-ledger.md`。 [R3] `theory/08-patch-hotplug.md`。いずれもcommit `f6aae7ca277690ac558a08f124e414ae2d2d35ad`を照合基準とする。資料一覧と正確なpathは`provenance/sources.json`にある。

## 16. 最終実行の参照と証明状態

全体runnerの最終実行では、新規153テスト、継承69テスト、統合例、patchモデル、83 SMT入力がそれぞれ期待結果と一致した。詳細はVALIDATION.mdを参照する。これらの数を一般的な証明数や完成率へ読み替えない。

F2-01〜27は手証明である。source semanticsとCFGの対応では、再帰calleeの全実行を先に仮定せず、入口・stack・帰還に関する同時の一歩対応を用いる。Pythonコード自体と定理の対応、Lean/Rocq上の全文証明は未実施である。

fresh importはwell-formedな候補の再構成であり、元のarchiveの真正性を検証する署名機構ではない。public-only sourceモデルのsimulationから、private分散系の全情報流保証を結論しない。

実装する道筋があるという評価は、この選択profileの具体的な規則・protocol・証拠形式があるという意味である。任意の局所理論、高階handle/module、全componentのdurable session、広いprivacy保証までが確定済みという意味ではない。

## 補足：確定判断と利用側の保持する参照

分散patchのCommitは、影響する値の抽象的な新状態への切替点である。一方、各利用側が保持する操作の版参照や、既に発行した要求まで、瞬間的に新版へ書き換わるとはしない。ソースの抽象実行にも、利用側が保持した版付き参照と、使用時に不一致となる失敗を含める。Commit後、未Installのownerは停止境界内の旧値を新規作用へ公開せず、Install後には旧bindingの要求を拒否する。この区別なしに、世界全体のregistryを瞬間的に同時更新した通常の関数呼出しと同値だと主張してはいけない。

最終追加の二owner結合検査では、片方だけInstall済みの状態、もう片方のCrash/Recover、残りのInstall、部分Prepare後のAbortを、実際の型付きactorの値と停止境界へ接続して確認した。初回の検査用API名の取り違えは、`evidence/multi_owner_setup_error.txt`へ別保存し、理論的な反例として数えていない。
