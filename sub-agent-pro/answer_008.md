2026-04-13時点の文書に基づくと、自然なのは **final parser grammar を一括で freeze するのではなく、`semantic cluster → parser subset → parser-to-checker bridge → source-backed path → lexical surface` の順で段階的に freeze する** 進め方です。いまの repo は、companion notation を維持しつつも、`mir-ast` / source lowering / fixed-subset sample runner / verification ladder まで narrow syntax-backed path を持っていますが、文書群は一貫して「exact visual polish より semantic cluster を先に固定する」「accepted cluster と retained-later floor を分ける」「public parser API と final grammar を helper の成功だけで早く凍らせない」と読んでいます。   

## まず結論

いま freeze すべきなのは **「最終見た目」ではなく「受理してよい意味クラスタと橋の形」** です。
逆に、まだ freeze しない方がよいのは **A2/A1 の最終 lexical choice、reserved keyword 群、punctuation、richer predicate grammar、`contract` block sugar、option-local outcome metadata** です。explicit edge-row family 自体は first parser cut 候補に入れてよい一方、A2 hanging continuation を唯一の final lexical surface として固定するのはまだ早い、と文書が明示しています。 

## 1. final grammar freeze の entry criteria

entry criteria は次の 6 点で切るのが自然です。

第一に、**semantic cluster inventory が固定されていること**です。つまり first parser cut に入れる構文族が、`place`、`try { ... } fallback { ... }`、`atomic_cut`、`perform ... on ...`、`perform ... via ...`、statement-local `require` / `ensure`、`option ... on ... capability ... lease ...`、option-local `admit`、explicit edge-row family までで合意されていることです。ここが曖昧なまま lexical freeze に入ると、syntax が semantics を先に縛ります。 

第二に、**minimal parser subset freeze が済んでいること**です。文書群の current first choice は、Phase 6 front-half actual parser gate を stage 1 + stage 2 structural floor に留め、first-tranche malformed family を `missing edge-local lineage metadata`、`missing fallback body`、`atomic_cut` fallback placement に絞り、stage 3 `admit` / request clause / predicate line は retained-later floor に残すことです。つまり final grammar freeze の前に、まず accepted cluster と reject cluster を narrow に固定する必要があります。  

第三に、**parser-to-checker reconnect の最小橋が固定されていること**です。current docs は、parser subset freeze の次段として reconnect bridge を stage 1 summary + stage 2 try/rollback structural contract に留め、stage 3 request/predicate reconnect や runtime contrast は後段に残す方針です。つまり grammar freeze の entry は「parse できる」だけでなく、「checker floor にどう繋ぐか」が source-backed で見えていることを要求すべきです。 

第四に、**source-backed path が lowering / runner / ladder まで固定されていること**です。`specs/examples` の並びも、mapping matrix → parser-to-`Program` lowering first cut → syntax-backed sample runner first cut → verification ladder wiring → bless/regression policy、という ratchet を想定しています。ここが固まる前に final grammar freeze を掛けると、sample corpus と formal bridge の双方に同時に手戻りが出ます。  

第五に、**public surface が grammar を逆流的に拘束しないこと**です。現在の public-pressure candidate は `run_current_l2_source_sample` という runtime-led thin facade であり、`run_current_l2_runtime_skeleton` や `lower_current_l2_fixed_source_text` は support-only tranche に留められています。したがって final grammar freeze は、public parser/checker/runtime API そのものを先に決める作業ではなく、少なくとも現段階ではその前段の narrow internal freeze として扱うのが自然です。 

第六に、**fallback の見た目が semantics honesty を壊さないこと**です。explicit edge-row family を維持している理由は、guarded option chain、edge-local lineage、request-evaluation boundary を最も honest に保てるからです。したがって final freeze の条件には「outer/inner 誤読を増やさない lexical surface に落ち着いたこと」を入れるべきで、compactness だけでは足りません。 

## 2. grammar freeze 前に先に固定すべきもの

ここは lexical grammar より **carrier と compare surface** を先に凍らせるべきです。

まず stage 1 では、declaration-side guard slot を predicate parser の入口としてではなく、**opaque slot** として扱う cut を先に固定するのが自然です。文書は parser-side carrier 名の第一候補を `decl_guard_slot` に置き、current parser-free AST fixture schema へは直結させず、`OptionDecl.lease` への thin lowering bridge を compatibility anchor にするのが最小だとしています。ここで固定するのは token ではなく、**slot exists / attaches / transfers** の構造です。 

次に stage 3 first tranche では、`admit` も同じで、まず `decl_admit_slot.surface_text` retention smoke の line を固定し、fixture-side `OptionDecl.admit` predicate node への direct lower や canonicalization をまだ入れないのが自然です。ここで request-local `require` / `ensure` と一気に混ぜると lexical freeze pressure が急に上がる、と文書が警告しています。したがって **`decl_admit_slot` を持つ attached-slot extension** を request cluster 本体より先に固定するのがよいです。  

その次に固定すべきは、predicate grammar 全体ではなく、**shared single attachment frame** です。つまり `admit:` と request-local `require:` / `ensure:` に共通する `<clause-head>:` + 直下 1 段深い predicate block だけを structural floor として固定する。これは clause suite 全体でも generic continuation でもありません。ここを先に固定すると multiline attachment の drift を抑えつつ、predicate language 本体はまだ later に残せます。 

さらにその次に、`perform` owner の **fixed two-slot request clause suite** を固定するのが自然です。`require` / `ensure` は sibling attachment line、順序は `require` の後に `ensure`、各 at-most-one、suite termination は dedent / 次 statement head、blank line は fail-closed。ここは final parser grammar の前でも固定しやすく、sample/fixture/formal bridge の接点として非常に強いです。しかも helper 出力は `require_fragment_text` / `ensure_fragment_text` の two-slot carrier に留められるので、request head full parse を後段へ残せます。  

最後に、request cluster 側では full request node compare へ行く前に、**`Stage3RequestContractSubset` 相当の dedicated helper-local carrier** を fixed two-slot suite bridge と fixture-side `contract.require` / `contract.ensure` subset compare の接点として固定するのがよいです。request head kind / op / target / chain_ref は still later に残し、contract-only compare surface を 먼저 narrow に閉じる。この順序だと parser pressure を必要以上に上げずに checker-facing subset を安定化できます。  

## 3. companion notation を長く残してよい部分 / 早めに grammar に寄せるべき部分

長く companion に残してよいのは、**visual polish と sugar** です。
具体的には、A2 hanging continuation と A1 inline row の最終 lexical choice、line-leading `>` ladder の再比較、`contract { ... }` block sugar、richer predicate sublanguage、option-local outcome metadata は、いま freeze しない方がよいです。文書もこれらを OPEN に残しています。ここは semantics を変えない比較領域なので、companion に長く置いても damage が小さいです。 

逆に早めに parser grammar に寄せるべきなのは、**semantic cluster を支える structural floor** です。explicit edge-row family そのもの、`try { ... } fallback { ... }` の構文分離、statement-local `require` / `ensure` の attachment、option declaration core、option-local `admit` の owner 分離、single attachment frame、fixed two-slot suite、stage 1 + stage 2 accepted subset です。ここは final visual polish ではなく、「何が same construct で、何が別 construct か」を決める領域なので、companion に長く置くほど parser / checker / sample runner の境界がぶれます。  

要するに、**family は早く freeze、rendering は遅く freeze** が自然です。
explicit edge-row family 自体は freeze してよいが、A2 を唯一の最終 surface としてはまだ freeze しない。`require` / `ensure` の clause ownership と two-slot suite は freeze してよいが、full predicate grammar と block sugar はまだ freeze しない。これが current repo の staged line と最も整合します。 

## 4. sample corpus と formal bridge の両方に手戻りが少ない freeze strategy

手戻りを減らすには、**grammar freeze を source-sample row と formal hook row の「間」に直接置かず、mapping / lowering / runner / ladder / report shape の seam に置く** のがよいです。

具体的には、
representative prose → fixture corpus → fixed-subset source-sample corpus を明確に分けたまま、まず mapping matrix を固定し、その次に parser-to-`Program` lowering first cut を fail-closed で固定し、その次に syntax-backed sample runner first cut を固定し、その上で verification ladder と bless/regression policy を固定する。こうしておけば、grammar の修正は lowering 内部と accepted cluster の narrow change に局所化され、sample IDs・matrix rows・runner report shape・formal hook fan-out をなるべく維持できます。   

さらに public-pressure は `run_current_l2_source_sample` の thin facade に留め、`lower_current_l2_fixed_source_text`、`mir_ast::current_l2` parser carrier floor、`run_current_l2_runtime_skeleton` などは support-only tranche に残すのが重要です。これにより grammar widening がただちに public API freeze へ連鎖しません。formal bridge も `CurrentL2SourceSampleRunReport` や formal-hook artifact 経由で fan-out する current cutが採られているので、raw parser surface を formal bridge の contract にしない方が手戻りが少ないです。   

## staged grammar freeze の段階案

私なら次の 6 段で切ります。

1. **semantic inventory freeze**
   first parser cut に入れる semantic cluster と companion に残す cluster を固定する。
   ここで freeze するのは family まで。lexical polish は凍らせない。

2. **minimal parser subset freeze**
   accepted cluster を stage 1 + stage 2 structural floor に限定し、first-tranche malformed family も narrow に固定する。
   まだ private / test-only / non-production。public parser API にはしない。 

3. **parser carrier / bridge freeze**
   `decl_guard_slot`、`decl_admit_slot`、single attachment frame、fixed two-slot suite、`Stage3RequestContractSubset` までを helper-local carrier として固定する。
   ここでも request head full parse と predicate full grammar は後段。  

4. **parser-to-checker reconnect freeze**
   stage 1 summary + stage 2 structural contract を checker floor へ繋ぐ。
   same-lineage / missing-option / capability の local / structural floor と整合させる。  

5. **source-backed path freeze**
   mapping matrix、parser-to-`Program` lowering、sample runner、verification ladder、bless/regression policy、thin facade report shape を固定する。
   ここで sample corpus と formal bridge の seam を安定化する。  

6. **lexical/final grammar freeze for accepted subset**
   最後に A2/A1、token、punctuation、attachment rule の final choice を accepted subset に対してだけ freeze する。
   richer predicate grammar、option-local outcome metadata、broader request cluster は still later に残す。 

## 今の repo では freeze してよい / まだ freeze しない

**いま freeze してよい**

* first parser cut に入れる semantic cluster の棚卸し
* explicit edge-row family 自体
* accepted parser subset = stage 1 + stage 2 structural floor
* first-tranche malformed family
* `decl_guard_slot` の opaque carrier + thin lowering bridge
* `decl_admit_slot` の attached-slot first tranche
* shared single attachment frame
* fixed two-slot request clause suite
* `Stage3RequestContractSubset` までの contract-only compare surface
* mapping matrix / lowering first cut / sample runner first cut / verification ladder / bless-regression seam
* `run_current_l2_source_sample` を public-pressure candidate に据える thin facade cut    

**まだ freeze しない**

* final reserved keyword 集合
* `perform` / `on` / `via` / `admit` / `require` / `ensure` の最終 token
* A2 vs A1 の最終 lexical choice
* line-leading `>` ladder の最終 verdict
* `contract { ... }` block sugar
* richer predicate grammar（`or` / `not` / precedence / row-list widening）
* option-local outcome metadata
* request head full parse を含む broader request cluster
* typed parser error carrier / public diagnostics wording
* final public parser / checker / runtime API
* installed public CLI / final host contract    

要約すると、**いま freeze するべきなのは grammar の「見た目」ではなく、「どこまでを accepted cluster として source-backed に責任を持つか」という境界**です。
この repo では、final parser grammar freeze は stage 1/2 structural floor と source-backed lowering/runner/ladder が閉じた後、stage 3 request/admissibility cluster を helper-local carrier で分離してから、最後に accepted subset の lexical surface を凍らせる、という順が最も自然です。
