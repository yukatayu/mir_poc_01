According to a document from 2026-04-13, この repo の current line は、full 型システムを mainline に入れる前に、Mir の意味論安定化、parser-free runnable subset、small decidable core、static gate、tool-neutral formal hook、theorem/model-check bridge、sample-visible evidence lineを先に閉じる構えです。Mir-0 に固定している核も、event DAG、`place`、effect / contract、primitive fallback、local rollback、place-local `atomic_cut`、linear resource に絞られており、`durable_cut`、coroutine、higher-level ordering family は後段へ送っています。したがって最終理論は、最初から巨大な単一 calculus を狙うより、core / checker / proof / protocol / runtime policy を分けた layered theory として考えるのが自然です。     

まず結論を短く言うと、最終理論の第一候補は「**affine/linear な effect-capability core**」に「**place/world index**」と「**ordered degradation judgment**」を足し、shared-space / authority / higher-level ordering は別の protocol/session/policy layer に分ける形です。ラムダキューブは pure value layer の説明には補助的に使えますが、この言語全体の主説明軸には向きません。主説明軸は、(1) resource discipline、(2) effect / contract / failure、(3) place/world、(4) monotone degradation / admissibility、(5) rollback / cut、(6) protocol / authority の六つです。    

## 最終理論の候補

### 1. Affine CBPV 風 core ＋ graded effects ＋ place/world index

これは私の「term calculus としての第一候補」です。
中心は affine/linear resource discipline です。ownership / lifetime / duplication 禁止は、普通の simply typed λ-calculus より、affine あるいは linear な core の方が自然です。そこに computation judgment 側だけへ effect / contract / failure summary を付けると、`perform`、`try`、rollback、`atomic_cut` のような effectful construct を value layer と混ぜずに扱えます。さらに `place` は型そのものよりも **judgment index** として持つ方がよいです。つまり「どの型の値か」より「どの place で、どの effect/failure summary のもとで計算されるか」を computation judgment にぶら下げる形です。`atomic_cut` も純粋な type former ではなく、effect discipline / rollback discipline 上の境界として表す方が docs の読みと一致します。  

この候補で自然に組み合わさる既存理論群は、linear/affine type theory、graded effect systems、world- or region-indexed judgment、必要なら modal type theory です。separation logic は、私は source language の主型理論より **proof / meta-logic** 側に置くのがよいと思います。repo が強く求めているのは「明示的 effect」「local rollback」「no hidden repair」であって、heap entailment を表面型に直接載せることではないからです。 

### 2. Ordered capability / contract calculus ＋ proof-relevant admissibility

これは「static checker / semantics side の第一候補」です。
Mir の難しさは、ordinary term typing だけでは足りない点にあります。fallback chain は単なる union でも例外処理でもなく、same-lineage、declared target、capability strengthening prohibition、contract-compatible successor、underdeclared rejection を持つ **ordered admissibility system** です。ここでは `A > B > C` を型で丸めるより、

* option declaration の well-formedness
* successor admissibility
* capability weakening/preorder
* chain well-formedness
* underdeclared / malformed rejection
  を別 judgment にした方が自然です。つまり「型」より「証拠つき judgment」の比重が大きい。現行 docs が same-lineage floor、capability floor、underdeclared surface error、checker floor / residual proof obligation という形で整理しているので、repo の current line と最も整合するのはこの候補です。    

ここで効く既存理論群は、capability calculi、refinement/contract systems、proof-relevant subtyping、ordered/modal judgments です。重要なのは、fallback admissibility を「単なる subtyping」にしないことです。Mir では lineage evidence と declared target が要り、underdeclared case は hidden acceptance ではなく static error なので、proof-irrelevant な subtyping だけでは薄いです。 

### 3. 二層理論: affine core ＋ protocol/session/authority layer

これは「stack 全体の最終形」としての第一候補です。
shared-space、authority、higher-level ordering family まで入ると、Mir core だけで抱えるのは不自然です。docs でも `atomic_cut` は place-local cut に留め、higher-level async-control、authority-serial transition、witness-aware commit、room policy family は後段・別境界へ送る方針です。したがって full theory は、

* 下層: affine effect-capability core
* 上層: session / protocol / authority / room-policy theory
  の二層に分けるのがよいです。shared-space final catalog や fairness は protocol verifier / runtime policy 側に置き、Mir-0/1 の core typing に押し込まない方が、repo の layer model と research phase に合います。   

私の推奨を一言で言うと、**最終理論は 3 の形にして、下層 core は 1、static admissibility/checker 側は 2 で与える**、です。

## ラムダキューブは有益か

主説明軸としては、私は **有益ではあるが主役ではない** と見ます。
ラムダキューブが教えてくれるのは、term/type/kind の依存性と polymorphism の強さです。しかし Mir の難所はそこではありません。repo が前面に出しているのは、ownership / monotone lifetime、explicit effect / contract / failure、`place`、rollback / `atomic_cut`、ordered degradation、authority / shared-space boundary です。これらは substructural・effectful・modal・temporal・protocol 的な軸であって、ラムダキューブの座標だけでは見えません。   

なので説明の順番としては、

1. affine/linear か
2. effect/failure/contract を judgment に持つか
3. place/world index を持つか
4. degradation/admissibility を preorder/judgment に持つか
5. cut/rollback を local temporal discipline にするか
6. authority/shared-space を別 protocol layer に分けるか
   を先に言い、その後で「pure value fragment が将来どこまで polymorphic か」を補足として λ-cube で説明するのがよいです。かなり控えめに言えば、pure value fragment だけなら System F か Fω 近辺に落ちる可能性はありますが、そこを今決めても repo の主要論点はほとんど片付きません。

## 今の repo が最初に切るべき最小 typed core

私なら、最初の typed core は **「多相性の強い calculus」ではなく、「局所的で決定可能な judgment の束」** として切ります。

### 固定してよい sort / kind

最初に固定してよいのは、full dependent hierarchy ではなく次の sort です。

* `Type` — 値や計算結果の形
* `Place` — 計算の locus
* `Capability` — read/write などの surface
* `EffectSummary` — computation が起こしうる effect の要約
* `OutcomeKind` — `Reject` / `Approximate` / `Compensate` を含む outcome family
* `Pred` — `require` / `ensure` / `admit` に使う最小 predicate fragment

ここで `lease` は独立 universe にせず、**guard/predicate 側のもの**に留める方がよいです。`lineage` も first-class type constructor にせず、**evidence object / checker evidence** に留める方が current docs と一致します。authority/profile/protocol はこの core には入れません。   

### 固定してよい judgment

先に固定すべき judgment は次です。

1. 値の affine typing
   `Γ; Δ ⊢ v : A`
   ここで `Δ` は affine/linear resources。

2. place-indexed computation typing
   `Γ; Δ ⊢ c : A @ p ! ε ▷ ω`
   `p` は place、`ε` は effect summary、`ω` は outcome/failure summary。

3. option declaration well-formedness
   target / contract surface / capability surface / lease guard を持つ option の整合性。

4. successor admissibility / degradation judgment
   「後段は前段より capability を強めない」「declared contract surface に矛盾しない」。

5. chain well-formedness judgment
   declared target、一対一の edge-local lineage annotation、underdeclared rejection を含む。

6. rollback / cut structural safety judgment
   `try` / rollback locality と `atomic_cut` structural floor だけを扱う。

7. proof-obligation emission judgment
   canonical normalization / no re-promotion / rollback-cut non-interference は checker core に入れず、external handoff row として出す。

この切り方は、repo がすでに first checker cut 候補として認めている same-lineage floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、minimal predicate fragment、`try` / rollback locality structural floorと一致しています。  

### まだ固定しない方がよいもの

逆に今は固定しない方がよいのは、

* full fallback algebra / collapse rule
* general type inference strategy
* higher-kinded / dependent contract language
* full predicate grammar
* `Approximate` / `Compensate` の executable carrier
* coroutine / multi-shot continuation
* authority / fairness / shared-space final catalog
* low-level memory-order family

です。docs でもこれらは heavy future workstream、proof boundary、runtime policy boundary、shared-space line に残されています。 

## narrow runnable subset への接続方法

ここは repo の current methodology に合わせるのが大事です。手戻りを減らすには、型理論 line を **parser ではなく semantic carrier と checker boundary に接続** すべきです。

第一に、typed core の最初の consumer は final parser grammar ではなく、

* parser-free AST fixture
* `Program` carrier
* static gate
* `run_current_l2_runtime_skeleton`
* tool-neutral formal hook
  に置くのが自然です。surface notation は現時点で companion notation であり final grammar ではないので、型理論を syntax に結び付けるのが早すぎます。  

第二に、typed line は runtime replacement としてではなく **additive checker shadow** として入れるべきです。
つまり現行の runnable subset を壊さず、static gate 側に

* same-lineage
* capability no-strengthening
* missing-option structure
* `try` / `atomic_cut` structural
  の typed judgments を足し、global property は formal hook / theorem projection 側へ handoff する。repo が採っている `core_static_checker / theorem_prover_boundary / protocol_verifier_boundary / runtime_policy_boundary` の 4-way split を、そのまま型理論導入の split として使うのが最も手戻りが少ないです。 

第三に、導入順序は今の regression family に合わせるべきです。
いきなり “full typing” に行かず、

* same-lineage floor
* missing-option structure
* capability strengthening prohibition
* `try` / rollback structural floor
  の順で source-backed family を厚くし、その後に theorem-side obligation rows を細く actualize する。current repo はまさに heavy type workstream の前にこの checker-boundary 整理を mainline に戻してよい、という判断を置いています。 

第四に、authority / shared-space / higher-level ordering family は typed core の extension ではなく、**後で別 theory を載せる** と最初から決めておくべきです。`atomic_cut` に authority-serial transition や fairness を背負わせない、という docs の方針は型理論設計にもそのまま効きます。shared-space では participant carrier、authority placement、consistency mode、RNG/fairness source を別軸に保つべきだと明記されているので、ここを base typing に押し込まない方がよいです。 

## 最終提案

私の提案を一つにまとめるとこうです。

* **最終理論候補**
  「二層理論」が第一候補です。下層は affine CBPV 風の effect-capability core、上層は protocol/session/authority theory。fallback admissibility は ordinary typing の中に押し込まず、proof-relevant ordered judgment として持つ。

* **最初に切るべき最小 typed core**
  `Type / Place / Capability / EffectSummary / OutcomeKind / Pred` の sort と、
  値 typing、place-indexed computation typing、option/chain well-formedness、successor admissibility、rollback/cut structural safety、proof-obligation emission を固定する。

* **ラムダキューブの位置づけ**
  pure value fragment の polymorphism を later に説明する補助軸としては使える。だが主説明軸は resource / effect / place / degradation / cut / protocol の六軸である。

* **今の narrow subset への接続**
  parser grammar ではなく static checker と semantic carrier に接続し、current source-backed regression family を typed checker floor と theorem-side handoff row に変換していく。

この方向だと、repo がすでに守っている「Mir core を狭く保つ」「global law は proof boundary に残す」「authority/shared-space を別 layer に残す」という判断を壊さずに、将来の型理論をかなり強くできます。   
