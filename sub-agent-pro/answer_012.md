According to a document from 2026-04-13, 今の repo line に最も整合する答えは、「capability-scoped port / adapter boundary を先に docs-first で固定し、その後に bridge-only、narrow runtime adapter、最後に concrete adapter と raw FFI へ進む」という順序です。current docs はすでに、language core に privileged `stdin/stdout` を入れず、capability-scoped input/output port / adapter boundary を first docs-only cut に置き、visualizer / host substrate / host runtime は consumer/provider 側、FFI / game engine adapter と final naming は later gate に残す current cut を採っています。さらに near-term roadmap でも、host-facing integration は separate gate であり、raw FFI / game engine direct binding actualization は current stop line に置かれています。   

1. capability-scoped port / adapter boundary は妥当か

妥当です。Mir は effect / contract / failure / rollback の意味を定義する層であり、device privilege や `stdin/stdout` のような host-specific primitive を language core に固定する場所ではありません。Typed-Effects Wiring Platform はまさに、外部 effect boundary を inspectable・contract-aware・rewritable にする隣接 operational layer として置かれており、Mirrorea は routing / rebinding / audit を担うが、effect boundary そのものを language core に押し戻す役割ではありません。加えて、engine-specific concept は core language semantics に漏れ込ませてはいけない、という制約もあるので、port を capability-bearing な抽象境界として切り、host/visualizer/engine は adapter 側に残すのが自然です。    

あなたの「未接続ならただの port であり、attach されたときだけ effect が流れる」という直感も、この方針と整合します。ただし 1 点だけ補正が必要です。未接続 port を「no-op で黙って成功するもの」にしてはいけません。current docs 全体は、effect を first-class に保ち、failure を明示的に扱い、hidden repair や hidden promotion を避ける方向で揃っています。したがって未接続は「抽象 port ref が存在する」ことと「provider binding が成立している」ことを分け、未接続時の outcome は later host-facing contract で明示されるべきです。今の段階では、それを `Reject` にするか explicit failure にするかまでは固定しない方がよいですが、少なくとも hidden success にはしない、というのが自然です。これは docs の明示的 effect / structured failure / fail-closed harness 原則からの推奨です。   

2. recommended layering

私なら、次の 6 層に分けます。

まず最上段は **Mir core semantic boundary** です。ここには privileged I/O は置かず、置くとしても「capability-scoped port ref」「その port に対して許される effect / contract」「attach されうる consumer/provider が外側にいる」という抽象だけに留めます。ここで host 名、windowing、engine API、FFI symbol 名は出しません。  

その次が **docs-only host-facing boundary** です。ここが `specs/examples/385...386` 相当の層で、current boundary、consumer/provider refs、attach/detach の読み、host-facing port という working label、kept-later refs を固定する場所です。重要なのは、ここで fix するのは「概念境界」であって production host contract ではないことです。`host plan` はこの層にぶら下がる declarative binding artifact として扱うのがよく、runtime primitive や final public host API と同格にしない方がよいです。current `.host-plan.json` も production manifest ではなく harness 用 sidecar ですから、その読みをそのまま public host contract に昇格させない方がよいです。  

その次が **bridge-only runtime layer** です。ここでは narrow runtime adapter を library-side support cut として置きます。今の repo の語彙に寄せるなら、`run_current_l2_source_sample` を中心とする thin facade や、`run_current_l2_runtime_skeleton` のような support-only surface の上で、「どの port にどの provider が attach されているか」を explicit input として流す橋を切るのが自然です。ここで大事なのは library-before-CLI を守ることです。current docs は final public parser/checker/runtime first later gate を library-side に置き、CLI は second later gate として別に残しています。したがって host-facing integration の spine を CLI にしてはいけません。  

その下が **effect-base substrate** です。docs の既存語彙に寄せるなら、これは Typed-Effects Wiring Platform 相当の operational substrate です。責務は effect boundary の観測、routing / rewiring、state と event history の関係づけ、非 Mir system の統合支援です。Mirrorea はさらにその外側で logical naming、route rebinding、overlay、audit を担えますが、adapter binding の reusable substrate 自体は Typed-Effects 側に寄せる方が自然です。つまり effect-base substrate は Mir core ではなく、Mir adjacent / Mirrorea adjacent の operational layer です。   

そのさらに外側が **concrete adapters** です。ここに visualizer adapter、host runtime adapter、service adapter などを置きます。visualizer はここで二段に分けるのがよく、第一段では emitted artifact / trace / graph view を読む「受動 consumer」として入り、第二段で必要なら live output/input adapter になります。docs が visual tool に要求しているのは、ad-hoc hidden state ではなく同じ graph / contract model から説明可能であることなので、まず artifact/trace consumer として入れる方が筋がよいです。 

最後に **domain-specific engine adapter** と **raw FFI leaf** を分けます。game engine adapter は generic host I/O と同列ではなく、L4 domain engine / framework 寄りの存在です。PrismCascade も同じ理由で独立 kernel として分離されています。したがって engine adapter は generic host-facing contract を定義する層ではなく、その contractを消費する target-specific adapter に留めるべきです。raw FFI はそのさらに下の leaf implementation detail であり、adapter の背後に隠すべきです。FFI を generic port contract の顔に出すのは避けた方がよいです。   

3. host plan / visualizer / effect-base substrate / FFI / game engine adapter の位置づけ

この 5 つは同じ種類のものではありません。`host plan` は層ではなく **binding control artifact** です。位置としては docs-only boundary と bridge-only runtime のあいだに置くのが自然です。今の host plan sidecar と同様、何が attach され、どの provider が使われ、どの override/expectation があるかを記述する declarative object であり、final host protocol ではありません。 

`visualizer` は最初から live engine の顔にするより、まず **artifact/trace consumer** として導入するのが安全です。その後、必要なら port-bound output consumer、さらに必要なら input provider へ進めます。これにより visualizer が semantics を決める側に回るのを防げます。 

`effect-base substrate` は **Typed-Effects Wiring Platform 寄りの reusable operational substrate** です。adapter の attach、inspect、route、rewrite、trace linkage を支えます。`FFI` はそこには置かず、あくまで leaf binding です。`game engine adapter` は substrate の consumer/provider 実装であって、substate 自体ではありません。ここを混ぜると engine-specific API が generic host-facing contract を支配し始めます。  

4. docs-first を先に切り、raw FFI を遅らせる方針は妥当か

かなり妥当です。むしろ current docs が強く推しているのはその順序です。理由は三つあります。第一に、`stdin/stdout` や `host port` の terminology を早く固定すると Typed-Effects / host interface / adapter line が premature に凍りやすいこと。第二に、richer host interface を production host に誤昇格させやすいこと。第三に、backend / codegen / engine binding を早く入れると syntax / lowering / runtime boundary が早期固定しやすいことです。current roadmap はこのために Macro 7 を separate track にし、docs-first host-facing boundary を fixed しつつ、FFI や engine integration を later gate / stop line に残しています。    

ここで 1 つ補足すると、あなたの staged plan は正しいのですが、私は最後の「concrete engine / FFI」をさらに二つに割るのを勧めます。つまり、**concrete adapter** と **raw FFI** は同時に開けない方がよいです。まず engine/visualizer 側の adapter contract と target profile を narrow に決め、その adapter の内部実装として raw FFI を入れる。こうしておくと target profile の変更や engine 差し替えが、host-facing contract の再設計に直結しません。  

5. fixed-subset sample line の後で開くなら、どの順で package を切るか

おすすめは次の順です。

まず **boundary package** です。ここでは docs-only で、capability-scoped port inventory、consumer/provider split、attach/detach の読み、unattached / mismatched / unavailable binding の failure surface、working terminology を切ります。ここで final token や final public host contract を決めないのが肝です。 

次に **binding artifact package** です。ここで `host plan` を一般化した binding plan を切ります。ただし現行の `.host-plan.json` をそのまま production contract にしない。狙いは「どの port がどの provider に attach されるか」を declarative に言えることだけで、transport、UI event loop、vendor ABI、final naming はまだ kept-later に残します。 

その次が **library-side bridge package** です。ここでは `run_current_l2_runtime_skeleton` に近い support cut か、その隣に narrow adapter runner を置き、explicit attachment input を受ける path を作ります。ここではまだ CLI を主役にしません。current later ordering では library-side contract が first gate、CLI は second gate だからです。  

その次に **visualizer package** を入れるなら、第一弾は read-only artifact consumer にするのがよいです。trace / audit / graph view / report-linked visualization をまず作り、それで不足が明確になってから live adapter へ進む方が、境界が崩れにくいです。 

その後に **first concrete target profile package** を 1 つだけ選びます。ここは docs でも autonomy gate が mixed / user-spec-required 寄りです。つまり、visualizer / host substrate / FFI / game engine adapter の「first target profile」は、完全 self-driven ではなく user choice を入れた方がよい線です。したがって「generic host integration framework」を先に作るより、`artifact visualizer` か `console-like host adapter` か `one engine scene adapter` のどれか 1 つを first profile として選ぶ方がよいです。 

最後に **adapter package** と **FFI leaf package** を切ります。game engine を選んだなら engine adapter を先に作り、その内部に raw FFI を入れます。host runtime を選んだなら OS/window/input adapter を先に作り、その内部に FFI や platform API binding を入れます。いずれの場合も FFI は leaf であり、generic host-facing contract の一部にはしません。  

6. 最終的な recommendation

なので、あなたが挙げた「まず docs-only / bridge-only、次に narrow runtime adapter、最後に concrete engine / FFI」という staged plan は、現行 docs とかなりよく一致しています。私ならそれを次のように少しだけ sharpen します。

**docs-only boundary → declarative binding artifact → library-side bridge runtime → passive visualizer / artifact consumer → one chosen concrete adapter profile → raw FFI leaf**

この順にすると、Mir core を汚さず、Typed-Effects / Mirrorea / tooling / domain engine の境界も保ちやすいです。特に `host plan` を final host contract にしないこと、CLI を integration spine にしないこと、game engine adapter を generic host I/O と同列にしないこと、raw FFI を adapter の背後に閉じ込めること、この 4 点を守ると drift がかなり減ります。    

必要なら次の返答では、この layering をそのまま `specs/examples/385...386` 風の docs-only threshold 文体に落として、「entry criteria / current boundary / consumer-provider refs / guard / kept-later refs」の形で具体化します。
