According to a document from 2026-04-13, 現在の repo はすでに `ToolNeutralFormalHookArtifact -> ModelCheckConcreteCarrierArtifact` という row-local machine-facing sibling artifact までは actualize しており、その後段の source-sample emitted verification artifact wiring と sample-facing theorem/model-check evidence summary / bless-review flow も narrow に actualize 済みです。一方で、formal tool binding は still later で、current first concrete consumer は theorem-side の `proof_notebook_review_unit`、model-check は second reserve です。したがって、いま自然なのは「大きい verifier contract を先に固定する」ことではなく、「既存 row-local carrier を壊さない narrow non-production binding を second reserve として足す」ことです。   

私の推奨は、**first concrete tool family は TLA+ family、ただし first runner は TLC、second runner は Apalache**です。TLA+ の公式資料では TLC が主要な model checker とされ、TLA+ Wiki でも TLC は finite systems を主対象とする explicit-state model checker と説明されています。Apalache は公式 docs で TLA+ の symbolic model checker、かつ TLC の代替系として位置づけられています。repo 側の事情に照らすと、current carrier が `subject_kind + subject_ref + case(obligation_kind + evidence_refs)` の row-local case list であり、actual carrier first / wiring later / public-checker later の順序が強く守られているので、**まずは case ごとの小さな transition module を出せる family**が合っています。その意味で、generic な explicit-state class を抽象的に選ぶより、**TLA+ projection を共通 IR にして TLC と Apalache を同じ projection の 1st/2nd runner にする**方が repo の cut discipline と相性が良いです。   ([lamport.azurewebsites.net][1])

逆に、**Alloy を first main line に置くのは勧めません**。Alloy 6 は behavioral model と lasso trace を扱えますし、bounded search / relation-heavy な sanity check には有用です。ただ、この repo の first model-check line が欲しいのは「cut / rollback / degradation / authority transition の step relation に対する safety counterexample」であって、中心は relational shape ではありません。Alloy は shape-only の sidecar、たとえば malformed option graph や route/receipt schema の bounded counterexample 探索には向きますが、first home にはしない方が自然です。**Ivy** も first main line ではなく、後段の protocol-verifier boundary 寄りです。公式 docs でも Ivy は protocol の specification / verification と invariant checking / bounded model checking を decidable fragment に寄せる道具として説明されており、これは route proof / overlay path / authority protocol の line には合いますが、いまの semantic formal hook からの second reserve には重いです。  ([Alloy Tools][2])

したがって順序としては、次がいちばん自然です。

1. **theorem-first line は現状維持**
   `proof_notebook_review_unit` を first concrete consumer とする current order は崩さない。model-check は second reserve のまま進める。 

2. **self finite abstraction を internal layer として導入**
   これは external tool family ではなく、`ModelCheckConcreteCarrierArtifact` と concrete checker の間の私的 projection です。ここで bounded state / action / obligation を切る。 

3. **first concrete runner は TLC**
   最初は explicit-state で safety counterexample を取る。理由は、first line で必要なのが fairness 付き liveness ではなく、small bounded transition safety だからです。  ([docs.tlapl.us][3])

4. **同じ projection を Apalache にも流せるようにする**
   explicit-state で十分な間は TLC、state explosion や parametric bound の pressure が出たら Apalache を second runner にする。projection を共有すれば binder だけ増やせばよい。([Apalache][4])

5. **Alloy / Ivy は sidecar / later line**
   Alloy は bounded relation sanity、Ivy は route / overlay / authority の protocol-verifier boundary が立ってから。first main line にしない。  ([Alloy Tools][2])

### 2. carrier をどう state machine / temporal obligation に持ち上げるか

ここは **「universal semantic IR を作る」のではなく、「case ごとの micro-machine template へ持ち上げる」**のが自然です。current carrier 自体が row-local で、docs でも actual carrier first / wiring later / public-checker later を強く守っているので、ここで global machine を作ると bridge / bundle / runner / public-checker migration を premature に混ぜやすくなります。まずは obligation family ごとに小さい transition template を持ち、その template に `evidence_refs` を差し込む形がよいです。  

最小の internal projection は、たとえば次のような shape で十分です。

```text
AbstractMcCase {
  subject_kind,
  subject_ref,
  case_kind,          // rollback_cut | no_repromotion | authority_serial | route_overlay ...
  evidence_refs,
  bounds,             // chain_len<=3, place_count<=2, epoch_count<=2 ...
  state_vars,
  init,
  next,
  obligations,        // safety_invariant | bounded_reachability
  fairness = none     // first cut では固定で off
}
```

重要なのは 3 点です。
第一に、`evidence_refs` は theorem-side と同じ typed symbolic ref family のまま残し、tool binder が late resolve すること。第二に、`obligations` は first cut では invariant / bounded safety に限ること。第三に、source-sample emitted wiring や sample-facing summary は downstream fan-out であって、この projection 自体に混ぜないことです。  

この projection で切る micro-machine は、最初は 2 種で十分です。
ひとつは **rollback-cut machine**。state は `place-local rollback frontier`、`cut_seen`、`pre_cut_effect_committed`、`failure_seen` 程度で足ります。もうひとつは **chain-degradation machine**。state は `current_option_index`、`expired_or_rejected_prefix`、`write_capability_profile`、`terminal_outcome` 程度で足ります。authority / route 系は別 template に分け、first cut には入れません。  

そして first concrete runner の入力は **formal hook 直ではなく current row-local model-check carrier**にする方が良いです。docs でも `current_l2_emit_model_check_carrier.rs` は formal hook JSON から row-local carrier JSON を出す thin CLI emitter に留められており、source-sample runner wiring / public-checker migration / concrete tool binding は持ち込まない cut が既に固定されています。したがって next cut は、その JSON を読んで TLA+ module + config を出す non-production emitter を examples/support 側に追加するのが自然です。`lib.rs` / `harness.rs` の public API や `run_current_l2_source_sample` の report shape は触らない方がよいです。  

### 3. first model-check line で何を対象にすべきか

**first target は `rollback-cut non-interference` です。**
理由は 2 つあります。ひとつは、これは current docs で residual proof obligation として明示されつつ、semantic core 自体は settled judgment であること。もうひとつは、current authored source-sample / formal-hook reached runtime row が `e1` / `e2` / `e21` / `e22` に揃っており、特に `e21` / `e22` は try-rollback locality / `atomic_cut` frontier contrast を source-backed で持っていることです。つまり、**今すぐ source-sample emitted route に乗せやすい property** がこれです。 bounded local transition safety として TLC に落としやすく、counterexample trace も人間に説明しやすいです。  

**second target は `no re-promotion` です。**
ただしこれは **first ではなく second** が自然です。`no re-promotion` は current docs で `canonical_normalization_law` と並ぶ residual proof obligation に置かれており、primary home は theorem-side の方がきれいです。他方で、bounded chain machine に落とした counterexample search とは非常に相性がよいので、model-check は theorem-side を置き換えるのではなく、**finite witness / finite counterexample line** として併置するのが良いです。特に `e6/e7/e8/e9` family のような expiry/degradation runtime family が source-sample emitted route まで widen した時点で、TLC/Apalache sidecar が非常に有効になります。  

**`authority transition safety` は first line に入れない方がよいです。**
shared-space 側は अभी docs-first boundary で、authority / resource ownership / fairness / control-plane carrier は OPEN / FUTURE 寄りです。higher-level async-control family の current first cut も `authority_serial_transition_family` として theorem-line retained bridge 側に inventory されており、まだ model-check mainline に落とす段ではありません。authority transition safety 自体は将来 model-check / protocol-verifier に非常に向きますが、first cut に入れると shared-space final catalog を早く凍らせやすいです。  

**`route proof / overlay path` も first line ではありません。**
Mirrorea docs 自体が path proof の表現と検証を未解決事項に置いています。overlay/no-shadowing/path proof は重要ですが、現段階では runtime/audit contract の方が先で、model-check concrete tool の first cut に載せるには表現がまだ薄いです。これは later の protocol-verifier boundary か、runtime audit/property-check side の話として残す方が自然です。 

**`fairness / liveness` は first model-check line に送らない方がよいです。**
fairness source、authority trust model、delegated provider、distributed fairness protocol は shared-space 側でまだ comparison 中で、docs でも future comparison / OPEN / FUTURE として扱われています。first cut で fairness assumption を入れると、authority placement・provider placement・membership churn・timeout policy まで一緒に固定圧がかかります。したがって最初は safety-only、fairness/liveness は later が妥当です。   

### 4. property ごとの置き場所

* **same-lineage floor / malformed / underdeclared / capability strengthening / `try` / `atomic_cut` structural floor**
  **Primary: static checker**
  これは docs が明示的に first checker cut / checker floor に置いている local / structural / decidable family です。model-check へ送るより static に先に discharge すべきです。 

* **`rollback-cut non-interference`**
  **Primary: model-check first, secondary: theorem, runtime: trace evidence**
  理由は、residual proof obligation ではあるが、current source-backed runtime rows (`e21/e22`) に乗せやすく、small local transition machine で bounded safety として切りやすいからです。最終的な semantic statement は theorem-side にも残すべきですが、first concrete model-check cut の主対象として最も自然です。  

* **`no re-promotion`**
  **Primary: theorem, secondary: model-check, runtime: audit/trace**
  docs 上の位置づけは `canonical_normalization_law` と並ぶ residual proof obligation です。一般形は theorem-side に置くべきで、model-check は bounded chain instance での counterexample search / regression に使うのが自然です。 

* **authority transition safety**
  **Primary: later model-check / protocol-verifier, current now: theorem-side retained bridge + runtime audit**
  まだ authority-serial contract / owner slot / stage family / witness attachment を docs-first に切っている段階なので、first concrete model-check cutには早いです。shared-space profile がもう一段 narrow になってから、protocol-ish state machine として model-check へ送るのがよいです。 

* **route proof / overlay path**
  **Primary now: runtime/audit, later: protocol-verifier or model-check**
  path proof representation と verification 自体が unresolved なので、first concrete checker で抱えるより runtime/audit surface を先に保つべきです。 

* **fairness / liveness**
  **Primary now: runtime policy / docs-first boundary, later: model-check**
  first cut には入れない。authority trust, witness, delegated provider, distributed fairness protocol の比較がまだ future-facing だからです。 safety invariant が安定してから fairness assumption 付き temporal checking に進むべきです。  

要するに、**first concrete tool cut は「internal finite abstraction + TLA+ projection + TLC first runner」**です。
そして **first property は `rollback-cut non-interference`**、**second property は bounded `no re-promotion`**。
**authority / route / fairness は later**、**local structural floors は static checker に残す**、これが今の repo の sequencing と最も整合します。  

必要なら次に、この方針をそのまま `ModelCheckConcreteCarrierArtifact -> AbstractMcCase -> TLA+ module/TLC config` の最小 schema 案まで落として書けます。

[1]: https://lamport.azurewebsites.net/tla/tla.html?utm_source=chatgpt.com "My TLA+ Home Page - Leslie Lamport"
[2]: https://alloytools.org/?utm_source=chatgpt.com "Alloy Analyzer"
[3]: https://docs.tlapl.us/using%3Atlc%3Astart?utm_source=chatgpt.com "using:tlc:start - TLA+ Wiki"
[4]: https://apalache-mc.org/docs/apalache/index.html?utm_source=chatgpt.com "Getting Started - Apalache Documentation"
