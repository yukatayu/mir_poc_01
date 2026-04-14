結論から言うと、この repo の長期戦略は **「最終的に全部 Rust」ではなく、Rust-heavy core + mixed helper workflow を明示的に維持する** のが自然です。すでに current reading 自体が「Rust は parser / semantics / runtime / formal handoff 本体、Python は detached-loop orchestration・source-sample regression・docs validation・report scaffolding」という split を採っており、specs 側でも core implementation と tooling backend は Rust を優先する一方、implementation choice と public contract は早く凍らせない方針です。したがって目標は単一言語化ではなく、**どの責務が semantics-bearing / typed-contract-bearing か**で線を引くことです。  

この判断を強く支えているのが、current repo が helper surface を三つに分けて読んでいる点です。すでに stable public bucket として Rust の parser-free helper stack があり、別に crate-public but non-production compile-ready tranche があり、さらに repo-local helper / example emitter / Python orchestration helper は excluded bucket に残されています。しかも docs は繰り返し「`pub` visibility だけで final public contract と読まない」「repo-local Python helper を hidden promotion しない」と言っています。つまり混乱の原因は二言語であることそのものではなく、**bucket を跨いだ hidden promotion** です。  

## 1. どの層までを Rust へ寄せるべきか

Rust に寄せるべきなのは、単に「速そうなもの」ではなく、**意味論・型付き carrier・public/stable surface を持つもの**です。具体的には次です。

まず必須なのは、parser carrier、lowering、checker floor、evaluator/runtime、host harness の fail-closed behavior、`run_current_l2_source_sample` のような public entry 候補、`run_current_l2_runtime_skeleton` のような support cut です。これらは execution-critical であるだけでなく、`Program`、`FixtureHostPlan`、report carrier などの typed input/output を持ち、repo 全体の semantic source of truth に近いからです。  

次に、**pure transform だが external artifact や formal handoff の core を作るもの**も Rust に寄せるのが自然です。current docs でも detached bundle / aggregate / static-gate transform、formal hook transform、proof-notebook review-unit transform、model-check carrier transform は Rust の `examples/support/*` に置かれています。ここは「non-production だから Pythonでよい」ではなく、**Rust-native carrier から typed artifact へ落とす pure transform は Rust、 loop orchestration は Python**という split がすでに採られています。これが long-term にもいちばん drift を減らします。 

逆に、Rust へ寄せる必要がまだないのは、repo layout や sample policy に強く結びつくものです。`resolve_current_l2_source_sample_path` や accepted-set hard-coding が excluded bucket に置かれているのは、そうした coupling を final public contract に混ぜないためです。したがって「Rustで書けるか」ではなく、**repo-local policy coupling を public/stable layer に昇格させるべきか**で判断するべきです。 

## 2. どの helper は Python のまま残してよいか

Python のまま残して問題ないのは、**semantics を決めず、typed contract を確定せず、repo-local workflow を回すだけの helper** です。現行 docs でその代表とされているのは、`scripts/current_l2_detached_loop.py`、`scripts/current_l2_source_sample_regression.py`、各種 diff helper、assist helper、fixture scaffold helper です。これらは artifact 保存、compare、run-label convenience、single-fixture smoke、display-only suggestion、boilerplate 出力といった operational aid であり、public crate contract ではありません。  

この category には、docs validation、report scaffolding、review 用 checklist 生成、regression orchestration も入れてよいです。共通条件は一つで、**Python 側が verdict / outcome / reason family / artifact core schema を発明しないこと**です。Rust が出した typed result を読む、束ねる、保存する、比較する、見やすくする、という役割に留まるなら、Python は長期的にも十分正当です。逆に、Python が static verdict や runtime outcome を自前で再計算し始めたら、それは migration trigger です。 

family checker scripts も同様です。same-lineage / missing-option / capability family や `TryFallback` / `AtomicCut` structural checker は current docs で helper-local / non-production convenience に留められており、generic public checker API への昇格は still later とされています。したがって現段階では Python のままでよいですが、これを public checker contract や shared output contract に近づけるなら Rust 側へ寄せるのが自然です。 

## 3. public CLI / formal handoff / runtime shell / artifact emission を Rust へ寄せるタイミング

ここは一括移行ではなく、**4 本を別々に見る**のがよいです。

public CLI は、いちばん Rust へ寄せやすいです。current near-term reserve line 自体が「public operational CLI concrete shell actualization」で、しかもそれを `run_current_l2_source_sample` の上に載る **Rust-side operational wrapper** として narrow に actualize する読みが固定されています。つまり CLI は「final installed product」ではなく、「thin facade の上の narrow shell concern」として Rust に置くのが自然です。タイミングとしても、今の roadmap では near-term reserve にあります。   

runtime shell は、その一段内側です。`run_current_l2_runtime_skeleton` は current docs で later support cut に置かれており、explicit input surface も `Program` / `FixtureHostPlan` / optional `CurrentL2ParserBridgeInput` に narrow です。したがってこれは **いま Rust support として持つのが正しいが、まだ standalone final public entry に昇格させない** のが自然です。外部 consumer がこの typed support surface を直接必要とする時が、本格昇格のタイミングです。  

formal handoff は、core と wrapper を分けるべきです。core transform はすでに Rust に寄っていて正しいです。formal hook、proof-notebook review-unit、model-check carrier は Rust support modules で pure transform として置かれ、concrete tool binding は still later に残されています。したがって long-term 戦略としては、**formal handoff の core serialization / transform は Rust、consumer-specific binding と repo-local workflow は後段**が自然です。ここを Python で持ち続けると、typed row core と evidence ref family の drift が起きやすいです。 

artifact emission も同じで、payload-core transform は Rust、loop orchestration と compare convenience は Python でよいです。現在も bundle/aggregate/static-gate emitter の pure transform は Rust support にあり、`scripts/current_l2_detached_loop.py` や diff helpers は non-production wrapper に留まっています。さらに docs は storage/path policy と actual exporter API をまだ OPEN に残しています。したがって **artifact emission の「意味を持つ core」は Rust に寄せるべきだが、「どこへ保存し、どう compare を回すか」は Python のままでも問題ない** というのが最も自然です。  

## 4. mixed helper を残すなら、どういう boundary rule を置くべきか

いちばん効く rule は次の 4 本です。

第一に、**semantic truth is Rust** です。verdict、outcome、reason family、typed artifact core は Rust が決める。Python はそれを orchestration / compare / render するだけに留める。これで二言語 split があっても semantics drift はかなり防げます。 

第二に、**typed core と repo-local convenience を分離する**ことです。typed input/output carrier を持つ pure transform は Rust support module へ、run-label、artifact path、single-fixture smoke、diff wrapper、display-only assist は Python へ、とはっきり決める。現行 docs が examples/support と scripts を別 bucket にしているのは、この rule に沿っています。 

第三に、**public contract は Rust library or Rust CLI からしか出さない**ことです。Python script が widely used でも、それを public API と呼ばない。current docs でも repo-local Python orchestration helper は excluded bucket であり、hidden promotion を禁じています。ここを曖昧にすると、「実質 public だが non-versioned script」という最悪の状態になります。  

第四に、**移行は wrapper からではなく transform から行う**ことです。つまり「Python script を Rust CLI に書き換える」のではなく、先に Rust に pure transform / typed API を切り出し、その後必要なら Rust shell を載せ、最後に Python wrapper を薄く残すか削るか決める。この順序だと、public surface を不必要に太らせずに済みます。 current repo の detached export / formal handoff line は、すでにこの順序で進んでいます。 

## migration trigger と non-trigger

**migration trigger**

* helper が verdict / outcome / reason code / typed artifact core を決め始める。
* external contract、public CLI、installed binary、host-facing boundary に近づく。
* 複数 crate / test / support から共有される typed carrier が必要になる。
* exact-compare core や machine-facing schema を stable にしたくなる。
* Python 側に semantics の再実装が生まれ、Rust と drift し始める。
* repo-local convenience を超えて non-repo consumer pressure が出る。  

**non-trigger**

* その script が便利でよく使われる。
* Python の方が書きやすい。
* filesystem / path / run-label / compare orchestration を扱うだけ。
* report scaffolding、docs validation、display-only suggestion、boilerplate scaffold を行うだけ。
* sample policy や repo layout に強く coupling していて、まだ final contract に上げたくない。
* Rust の `pub` symbol が増えたので、ついでに全部 public 化したくなる。これは docs が明示的に禁じている非 trigger です。  

要するに、長期戦略としてはこう整理するのがよいです。**Rust は semantics-bearing core、typed transform、public thin facade、narrow shell を持つ。Python は repo-local workflow、compare、scaffold、docs/review orchestration を持つ。移すべきなのは “言語” ではなく “責務” であり、public pressure と typed contract pressure が migration trigger、便利さや美観は non-trigger** です。現在の repo の設計思想とも、near-term roadmap とも、この読みが最も整合しています。   
