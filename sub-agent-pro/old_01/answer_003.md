According to a document from 2026-04-13, この段階で sample-visible に出してよいのは「full な型理論」ではなく、**current L2 で既に semantic floor と checker floor が見えている declared surface** です。つまり、source code に出すのは execution/admissibility に直接効く最小宣言だけに絞り、proof obligation や richer な typed evidence は formal hook / docs に寄せるのが自然です。current repo は既に fixed-subset sample と sample-visible theorem/model-check milestone まで進んでいますが、強い型システムそのものは heavy future workstream に残されています。  

私の結論を一行で言うと、**先に出すべきなのは「型」より「宣言された境界」**です。具体的には、`require` / `ensure`、`capability` / `lease` / `admit`、`declared access target`、edge-local `lineage`、そして source code ではなく formal/docs に出す `obligation_kind + evidence_refs` です。逆に、effect kind の宣言言語、authority annotation、global lineage ID、full capability lattice、public-looking checker contract はまだ早いです。  

## 1. sample-visible に出してよい minimal surface

| 優先 | 候補                                                                                        | user-facing syntax        | internal carrier        | formal artifact              | 最初に現す層                  |
| -- | ----------------------------------------------------------------------------------------- | ------------------------- | ----------------------- | ---------------------------- | ----------------------- |
| 1  | request-local contract clause (`perform` + `require` / `ensure`)                          | 出す                        | 持つ                      | 派生で十分                        | parser + checker        |
| 2  | option-local capability / admissibility (`option ... capability ... lease ...` + `admit`) | 出す                        | 持つ                      | 必要なら要約のみ                     | parser + checker        |
| 3  | fallback の static evidence (`declared access target` + edge-local `lineage`)              | chain-heavy sample に限って出す | 持つ                      | 持つ                           | checker first           |
| 4  | proof obligation row (`obligation_kind + evidence_refs`)                                  | source code には出さない        | formal hook carrier に持つ | 出す                           | formal hook             |
| 5  | proof notebook attachment (`goal_text`)                                                   | source code には出さない        | review-unit に持つ         | docs-only derived view に出す   | proof-notebook bridge   |
| 6  | typed malformed/static evidence (`checked_reason_codes`, structural findings)             | source code には出さない        | 持つ                      | detached/static artifact に限る | checker / detached loop |

この順序にする理由は明確です。`perform` と statement-local `require` / `ensure` は既に Mir-0 / current L2 の core semantics と companion notation に入っており、first parser cut 候補にもなっています。これは「要求」と「保証」を sample 上で見せる最小 contract surface で、full type theory を持ち込まずに verification-visible にできます。  

次に出してよいのが、option declaration 側の最小 capability/admission surface です。current L2 では `option ... capability ... lease ...` と option-local `admit` が companion notation に入り、`admit` は request-level outcome ではなく non-admissible skip として安定した読みを持っています。ここで出すべき「typed-ness」は **small closed capability surface** であって、一般 capability lattice ではありません。つまり `capability kind` は最小語彙として yes、`effect kind` 宣言言語はまだ no、です。  

3 番目に出してよいのが fallback の static evidence surface です。current L2 では same-lineage を static に扱う最小 declared information が、各 option の `declared access target` と edge-local な `documented lineage annotation` だと明記されています。これはまさに「full 型ではないが、sample-visible に出すべき最小 declared surface」です。逆に global lineage identifier、chain-level blanket annotation、elaboration obligation は current admitted path に入っていません。したがって chain-heavy sample では `lineage(A -> B)` を出してよいが、そこから先の generic lineage/type machinery はまだ出さない方がよいです。  

4 番目は source syntax ではなく formal/docs 側です。current theorem-line の minimum contract row core は `obligation_kind + evidence_refs` に留める、という判断が既に固定されています。これは sample-visible に「何が証明・検査対象か」を見せる最小 row であって、source code annotation ではありません。最初に現す場所は formal hook で、その後に sample-facing docs summary に出すのが正しい順序です。  

5 番目の `goal_text` も同じです。これは proof-notebook first bridge の lightweight attachment としては採ってよいが、consumer-specific attachment です。したがって source code に出すのではなく、proof-notebook review-unit や docs-only derived view にだけ出すのが自然です。 

6 番目の `checked_reason_codes` や `checked_try_rollback_structural_findings` は useful ですが、sample code に出すべきではありません。これらは malformed/static-stop の typed evidence であって、current でも detached/static artifact や helper-local compare に留める線が強いです。特に generic checker family や public checker API にまだ上げない、という判断が明示されています。 

## 2. parser / checker / runtime / formal hook のどこに最初に現すべきか

原則はかなり単純です。**source code に見せる surface は parser ではなく checker で意味を持てるものだけ**、**proof 側の label は formal hook first**、**runtime は新しい typed notion の導入場所にしない**、です。 

`require` / `ensure`、`option capability lease admit`、`declared access target`、edge-local `lineage` は、最初に parser+checker の組で現すのが自然です。parser 側では companion notation / fixed subset の structural acceptance に留め、checker 側で same-lineage floor、capability strengthening prohibition、underdeclared rejection、clause attachment floor を持たせる。runtime はその既存 surface を消費するだけで、新しい source-level typed construct を invent しない方がよいです。  

proof obligation label は formal hook first です。`obligation_kind + evidence_refs` は theorem-side projection bundle / formal hook carrier で出し、その後 sample-facing evidence summary に見せる。ここを parser や source syntax に先に混ぜると、「proof のための row」が「言語の型注釈」に見えてしまいます。current docs はむしろその premature 昇格を避ける方向です。  

runtime は request-level outcome と audit metadata の場所であって、新しい型語彙の表面化場所ではありません。current L2 でも `admit-miss` や `lease-expired` は dedicated event にせず non-admissible metadata に留め、capability mismatch も formal subreason ではなく narrative explanation に留めています。したがって runtime で「typed notion を増やして見せる」のは、いまは逆方向です。 

## 3. まだ sample code に現さない方がよい typed notion

まず `effect kind` の明示的 declaration language はまだ出さない方がよいです。Typed-Effects Wiring Platform 自体が Mir に近接した別 subsystem であり、effect declaration の exact contract language は open です。sample code 側では `perform op on ...` / `via ...` で十分で、effect kind は formal hook や later typed-effects line に残す方が自然です。 

次に authority / activation / fairness の annotation は、current core sample にはまだ出さない方がよいです。`place` 自体は Mir-0 primitive なので sample-visible でよいですが、`authority` は shared-space / runtime evolution 側の separate track で、final catalog も user specification required line にあります。authority subject ref や authority-serial rows は theorem-line retained bridge の later family としてはあり得ても、current fixed-subset source sample の core typed surface にはまだ早いです。  

さらに、global lineage ID、chain-level blanket annotation、elaboration obligation も出さない方がよいです。current L2 は edge-local annotation と declared target の組だけを floor にしており、それ以上の generic mechanism は deliberately 採っていません。ここを早く一般化すると、full type theory や elaboration system を先取りしてしまいます。 

また、capability mismatch の formal taxonomy や `reason kind` / `request ref` / dedicated skip event も source syntax に出さない方がよいです。current L2 は capability mismatch を formal subreason に上げず narrative explanation に留めていますし、non-admissible metadata も audit channel で十分としています。typed static reason codes は useful でも、source sample に見せる種類の surface ではありません。 

最後に、`Approximate` / `Compensate` を current fixed-subset sample の visible contract として前面化するのもまだ早いです。Mir-0 の wider failure space にはありますが、current parser-free PoC の machine-check surface は `success` / `explicit_failure` / `Reject` / `not_evaluated` が中心です。ここを sample-visible source surface に先に入れると、failure lattice の actualization が source syntax を先行拘束します。

## 4. 近接 3〜5 package の刻み方

自然なのは 4 package です。

### Package 1 — sample-visible declaration floor

ここでは source code と docs に出す最小 surface だけを固定します。対象は `place`、`perform`、statement-local `require` / `ensure`、`option ... capability ... lease ...`、option-local `admit`、explicit edge-row `lineage` です。更新先は representative examples / source sample / syntax candidate docs で、狙いは「sample が contract-aware で typed-ish に見える」ことだけに留めます。effect kind declaration、authority annotation、proof label は入れません。  

### Package 2 — checker floor actualization

ここで初めて “typed-ness” を enforce します。same-lineage floor、missing-option structure floor、capability strengthening prohibition、clause attachment / predicate fragment floor を checker-side に actualize し、malformed family には `checked_reason_codes` などの internal-only typed evidence を足します。ただし public checker API には上げず、helper-local / detached artifact に留めます。  

### Package 3 — formal hook minimum row

次に formal hook / theorem-side projection で `obligation_kind + evidence_refs` を actualize します。ここで evidence ref は typed symbolic ref family に留め、path / URI / emitted artifact id は later に残します。source sample から emitted verification artifact への wiring は既に近接 mainline に乗っているので、この package は current trajectory とも一致します。  

### Package 4 — sample-facing evidence docs

最後に sample-facing docs へ見せます。ここで出すのは sample surface 上の type annotation ではなく、evidence summary row と、proof_notebook first bridge に限った `goal_text` です。bless/review flow は repo-local docs-first に留め、named notebook artifact や public checker migration はまだ開きません。  

必要なら 5 番目に **docs-first host-facing boundary** を置けますが、これは source code の typed surface ではありません。capability-scoped I/O / adapter boundary は sample-visible theorem/model-check summary の**次段の docs-first gate**であり、source syntax に混ぜるものではないです。 

## 実務上の推奨まとめ

私なら、今の段階では次のルールで切ります。

* source code に出すのは、`require` / `ensure`、`capability` / `lease` / `admit`、`declared access target`、edge-local `lineage` まで。
* proof obligation は source code に出さず、formal hook / sample-facing docs に `obligation_kind + evidence_refs` として出す。
* `goal_text` は proof_notebook derived view に限る。
* `place` は visible のままでよいが、authority annotation へ広げない。
* effect kind declaration language、global lineage ID、reason taxonomy の source-level 昇格、public checker-looking contract はまだ出さない。

要するに、**sample-visible surface は “light declared surface” に留め、typed reasoning の本体は checker floor と formal hook row に押し込む**のが最も自然です。これなら sample code は十分に contract-aware に見えますが、full type theory を先に凍らせずに済みます。   

