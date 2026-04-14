According to a document from 2026-04-13, この repo の theorem-line はすでにかなり明確に切られています。current first package は `core_static_checker / theorem_prover_boundary / protocol_verifier_boundary / runtime_policy_boundary` の 4-way split で、theorem 側の current first cut は actual emitter ではなく docs-only projection bundle、minimum contract row core は `obligation_kind + evidence_refs`、first practical consumer class は `proof_notebook`、first lightweight attachment は `goal_text` です。さらに current `proof_notebook_review_unit` helper も、`ToolNeutralFormalHookArtifact -> ProofNotebookReviewUnitArtifact` を row-local review unit に narrow に落とす cut に留め、bridge sketch / compare-bless metadata / proof assistant adapter / public theorem verifier API には widen していません。したがって、first serious formalization は「syntax を読む証明」でも「public contract を作る作業」でもなく、**semantic core relation に対する narrow theorem library** から始めるのが自然です。     

## まず machine-checked proof に乗せるべき性質

私なら優先順はこう置きます。

1. **`canonical_normalization_law + no_re_promotion`**

   最初の中心はこれです。current L2 では guarded option chain、left-to-right monotone degradation、earlier option への再昇格禁止が settled で、`canonical_normalization_law / no_re_promotion` は checker floor の外に残した residual proof obligation として明示されています。したがって first serious formalization は、`fallback` の nested shape そのものではなく、**canonical chain への正規化 relation** と **その chain 上の evaluation が index を決して巻き戻さない**ことを証明するのが最も自然です。これは repo が既に theorem-side に残した “global law” そのもので、syntax ではなく semantic core relation を対象にできます。    

2. **`rollback_cut_non_interference / hidden_rollback_absence`**

   次点はこれです。Mir-0 / current L2 では `try` rollback は current `place` に局所で、`atomic_cut` は rollback frontier を確定するだけであり、degradation order を巻き戻しません。repo でもこれは canonical law と並ぶ residual proof obligation として扱われています。したがって second theorem package は、**place-local state + rollback frontier + cut boundary** に対する small-step または big-step relation を定義し、rollback が pre-cut prefix を壊さないことを証明するのが自然です。これなら `durable_cut` や higher-level async-control family へ踏み込まずに済みます。   

3. **checker floor の soundness bridge**

   same-lineage floor、underdeclared rejection、capability strengthening prohibition は first checker cut 側の local / structural / decidable floor であり、repo も theorem prover 側へ丸ごと移すのではなく checker 側に残す方針です。なので theorem-side でやるなら、「same-lineage floor 自体を first theorem object にする」のではなく、**checker floor が semantic well-formedness / admissibility に対して sound である**ことを示す bridge theorem にするのが正しいです。つまり first theorem line の 3 本目は、`WF_checker_floor(x) -> WF_semantic_core(x)` 型の証明です。   

4. **ownership / lifetime monotonicity**

   これは重要ですが、私は 4 番目に回します。理由は、ownership monotonicity 自体は L0/L1 の強い invariant ですが、first theorem line の current consumer は `proof_notebook` で、repo の first serious proof boundary は fallback/cut law の方が明示的かつ source-backed だからです。ownership を先にやると resource carrier、transfer、rollback 下の resource view まで一気に厚くなり、first narrow line のコストが上がります。chain/cut law を先に閉じ、その後に **resource duplication 不在 / transfer monotonicity** を semantic execution relation 上で足す方が収まりがよいです。   

5. **まだ first theorem line に入れないもの**

   `durable_cut` の cross-place `all_of` family、shared-space authority / fairness、higher-level async-control family、low-level memory-order family はまだ入れません。repo 自身が `atomic_cut` を local cut の最小核に留め、higher-level async-control や low-level memory-order は later workstream として切っています。first theorem line でここへ進むのは明らかに premature です。  

## proof object は syntax ではなく semantic core relation を対象にすべきか

これは **妥当です。むしろそれが repo の current cut に一致します。**

repo は theorem-side current first cut を docs-only projection bundle に留め、row core を `obligation_kind + evidence_refs` に固定し、`goal_text` だけを `proof_notebook` first bridge の lightweight attachment にしています。つまり theorem-side の正本は、source text でも parser AST でもなく、**semantic obligation row と、それが指す semantic evidence**です。proof object はそこに合わせるべきです。   

私なら theorem-facing core は次の 4 relation から始めます。

* `CanonicalizesTo(nested_fallback, chain)`
* `WellFormedChain(chain)` / `CompatibleSuccessor(...)`
* `Step(state, event, state')` あるいは `Run(state, outcome, state')`
* `RollbackRespectsCut(pre, post)` / `NoRePromotion(trace)`

ここで重要なのは、**syntax tree の proof ではなく semantic IR の proof**にすることです。syntax が必要なら later に「parser / lowering が theorem-facing semantic IR へ sound に写る」ことを bridge theorem として足せばよく、first serious formalization では不要です。いま syntax を theorem line に持ち込むと、repo が意図的に deferred にしている final parser grammar や companion notation が逆流します。   

実務的には、proof object と handoff object を分けるのがよいです。

* **proof object**

  * prover 内の theorem / lemma / relation proof
* **handoff object**

  * `subject_kind + subject_ref + obligation_kind + evidence_refs`
  * notebook first bridge では必要最小限として `goal_text` を追加

この分離を崩すと、proof assistant の都合が formal hook や checker/public surface に逆流します。repo は今まさにそれを避けています。  

## first concrete prover family の比較

結論から言うと、**default recommendation は Lean 4**、次点が **Isabelle/HOL**、その次が **Rocq (Coq)**、**F*** は first theorem-line ではなく later verified-component line 向きです。

### 1. Lean 4

Lean は official docs 上、dependent type theory ベースの interactive theorem prover で、minimal kernel が proof term だけを検査し、tactic language も Lean 自身で書け、Lean 4 は self-hosted かつ rich extension API を持つ、とされています。repo 側が欲しいのは、semantic core relation を first-class に定義し、current docs-only theorem projection を壊さず、later に repo-specific automation を足せる prover です。そこに最も素直に噛み合うのが Lean です。特にこの repo は theorem line を `proof_notebook` first consumer に置きつつも、将来的には `proof_assistant_adapter` や richer bridge を compare していくので、**relation-first で始めて、後から custom elaboration / automation を足せる** Lean の性格は相性がよいです。   ([Lean Language][1])

### 2. Isabelle/HOL

Isabelle は official site で generic proof assistant とされ、documentation では Isabelle/HOL を classical higher-order logic と明示しており、Isar は readable formal proof documents を志向する semi-automated reasoning layer です。したがって、**人間が読む semantic proof document** を first value に置くなら、Isabelle/HOL は非常に強い候補です。repo の `proof_notebook_review_unit` や human review checklist / walkthrough first という current threshold とも感触は近いです。逆に、将来 typed symbolic ref を足場に custom theorem-side automation や prover-local DSL を強く育てたいなら、Lean の方が一歩前に出ます。なので Isabelle は「review-oriented semantics proof を最優先するなら最有力代替」です。   ([isabelle.in.tum.de][2])

### 3. Rocq / Coq

Rocq Prover は official site で trustworthy, industrial-strength interactive theorem prover かつ dependently-typed programming language とされ、仕様から executable programs を OCaml / Haskell として抽出できると案内されています。したがって、**将来 goal が certified checker / extracted validator / proof-carrying checker** に寄るなら Rocq はかなり強いです。ただ、current repo の theorem-side は هنوز docs-only projection bundle、`proof_notebook` first consumer、`goal_text` first attachment、public checker migration deferred という cut なので、first serious formalization の最初の一歩としては Lean より少し重く感じます。私は Rocq を「first theorem line の別解」ではなく、「checker kernel / extracted validator を狙うなら強い second choice」と見ます。   ([Rocq][3])

### 4. F*

F* は official docs 上、dependent types と SMT-based automation、tactic-based interactive theorem proving を組み合わせた proof-oriented programming language で、pure/effectful programming を扱い、OCaml/F#/C/Wasm などへ出せます。これは **effectful implementation の proof** には非常に強いです。ただし current repo が theorem-side first line に求めているのは、program/proof co-design ではなく **semantic core relation の meta-theory** です。さらに repo は runtime coupling / invocation surface / adapter contract を still later に置いています。なので F* は first theorem-line ではなく、later の runtime-policy / adapter / verified host component 側で生きる family です。   ([F*][4])

### まとめると

repo の current semantics / theorem-line boundary との相性だけで順に並べるなら、

1. **Lean 4**
2. **Isabelle/HOL**
3. **Rocq / Coq**
4. **F***

です。
ただし、チームに既に Isabelle/HOL か Rocq の強い内部知見があるなら、その既存能力が Lean の理論上の優位を上回る可能性は十分あります。current theorem line は intentionally narrow なので、初期学習コストの影響が大きいからです。

## いま concrete binding を入れてよい範囲

ここは narrow に切るべきです。入れてよいのは次までです。

1. **`theorem_prover_boundary` に限った prover-specific semantic core package**

   `core_static_checker` や `runtime_policy_boundary` ではなく theorem 側だけに結びます。4-way split を壊さないことが前提です。  

2. **tool-neutral formal hook から prover-specific goal stub への one-way bridge**

   ここで持つ row core は current cut に合わせて `obligation_kind + evidence_refs` に留める。notebook first bridge でだけ `goal_text` を足す。actual path / URI / emitted artifact id はまだ持ち込まない。   

3. **row-local proof unit だけ**

   current `proof_notebook_review_unit` helper 自体が row-local review unit list に留め、bridge sketch を作らず、supported pair も narrow に絞っています。したがって concrete prover binding も、まずは row-local theorem file / theorem block / theory stub の生成に留めるのが自然です。multi-row merged artifact や retained bridge までは行かない方がよいです。 

4. **実際に machine-check する theorem は 2+1 本で十分**

   * canonicalization soundness
   * no re-promotion
   * rollback-cut non-interference

   これに checker floor soundness を side theorem として足す、という cut が first serious line として最小です。`proof_notebook_review_unit` current pilot とも整合します。  

## まだ docs-only に留める範囲

これはかなり明確です。次はまだ concrete binding に入れない方がいいです。

1. **public checker migration**

   repo は theorem-side projection bundle を current phase では docs-only bridge に留め、public checker migration は concrete theorem consumer pressure が出たときだけ reopen する cut を採っています。なので prover binding を checker API 化しない。  

2. **named notebook bridge artifact / retained bridge chain**

   compare basis refs、bless decision、review session metadata、retained path policy、emitted artifact、consumer adapter、invocation surface、transport protocol、failure body へ伸びる retained bridge family は全部 later reopen です。first serious formalization で触るべきではありません。   

3. **`proof_assistant_adapter` contract / `theorem_export_checker` contract**

   repo 自身が `proof_notebook` を first practical consumer、`proof_assistant_adapter` を second practical candidate、`theorem_export_checker` を later candidate に置いています。よって concrete binding を adapter contract まで広げるのは早いです。  

4. **runtime coupling / actual notebook exchange**

   current emitter も formal-hook JSON を読んで review-unit JSON を出す thin CLI emitter に留め、public theorem verifier API や retained artifact bless/update に widen していません。これが正しい cut です。 actual invocation protocol や exchange body はまだ docs-only でよいです。 

5. **low-level memory-order / higher-level async-control / shared-space theorem line**

   repo は theorem-line の current next pressureを low-level order familyへ進めることではなく、checker-cluster docs-first matrix に戻すことだと切っています。shared-space / authority / fairness も still later です。 first serious formalization をここへ伸ばすのは外れます。   

## 私ならこう進めます

最後に、実際の first theorem line の作業順を一行で言うとこうです。

**Lean 4 を第一候補に選び、semantic core IR 上で `canonicalizes_to`・`run_chain`・`rollback_respects_cut` を定義し、最初の machine-checked theorem を `canonicalization_soundness` と `no_re_promotion` に置き、その次に `rollback_cut_non_interference` を証明し、checker floor は theorem 本体ではなく soundness bridge に回す。handoff は current row core `obligation_kind + evidence_refs` を守り、`proof_notebook` 向けにだけ `goal_text` を足す。public checker migration、named bridge artifact、adapter contract、transport/exchange family はまだ docs-only に留める。**     ([Lean Language][1])

この方針なら、repo がすでに切っている theorem-side boundary を壊さずに、first serious formalization を本当に「serious」なものにできます。

[1]: https://lean-lang.org/doc/reference/latest/ "The Lean Language Reference"
[2]: https://isabelle.in.tum.de/documentation.html "Documentation"
[3]: https://rocq-prover.org/ "Welcome to a World of Rocq"
[4]: https://fstar-lang.org/ "F*: A Proof-Oriented Programming Language"
