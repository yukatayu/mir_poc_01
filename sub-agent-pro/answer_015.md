According to documents from 2026-04-13, 最も費用対効果が高い方針は、source sample corpus を「demo 集」ではなく、**fixed-subset semantics / lowering / runner / verification ladder を段階的に太らせる verifier-driving corpus** として増やすことです。つまり、実用っぽい題材を増やすより先に、**settled current L2 を source-backed に押し広げる二本組の contrast package** を連続で入れるのが自然です。current repo は `Macro 4` fixed-subset sample expansion が active、`Macro 5` theorem / model-check bridge が early-active ですが、shared-space、high-level ordering、type surface、host-facing finalization はまだ later gate 側です。したがって次の 10 本は、shared-space や app 風味へ寄せるより、Mir current L2 の checker floor / runtime contrast / artifact fan-out を厚くする配分に寄せるべきです。   

結論から言うと、**next 10 は 8:2 くらいで「理論 pressure」優先**がよいです。具体的には、8 本前後を static floor または runtime semantics の contrast row に使い、残り 2 本を「実用っぽいが current L2 の範囲を超えない」sample に使うのがよいです。ここでいう practical は shared-space room や host I/O ではなく、`perform` / `require` / `ensure` / via-chain が絡む通常実行らしい行為です。source sample corpus 自体が representative prose でも fixture corpus でもなく、**fixed subset の syntax-backed regression layer** として置かれている以上、まず厚くすべきなのは理論のエッジです。  

next family を選ぶ基準は、私は次の 4 条件で見ます。第一に、**current L2 settled か checker floor に直接圧をかけるか**。第二に、**現行 fixed subset だけで書けて grammar widening を要求しないか**。第三に、**1 family を 2 本の contrast pair で閉じられるか**。第四に、**1 本足すと lowering / runner / ladder / regression / artifact fan-out の複数層に同時に効くか**です。逆に落とすべき family は、shared-space、high-level ordering、type surface、new failure class、privileged I/O、final parser polish のように、1 本入れるだけで later gate を凍らせやすいものです。要するに、**“semantic pressure × existing-subset compatibility × contrast leverage × ladder leverage” が高く、“grammar pressure × public-surface pressure × host-boundary pressure” が低い family** を先に取るべきです。     

malformed / valid / proof-visible / host-I/O-visible の順番は、**同列の family と見ない**のが重要です。malformed と valid は semantics family です。proof/model-check visible は、その後段の **artifact promotion** です。host-I/O visible はさらに外側の **wrapper / adapter visibility** です。したがって自然な順番は、**(1) malformed static-stop で checker floor を固定 → (2) 同じ意味領域の valid runtime contrast を追加 → (3) その family の中から 1 本だけ theorem/model-check visible canary を昇格 → (4) host-I/O visible は新しい core sample を作らず既存 runtime row の wrapper smoke として扱う**、です。host-facing I/O は docs-first boundary として separate gate に切られており、language core に privileged `stdin/stdout` を入れない方針も固定済みなので、ここを source sample corpus 側から引っ張ってはいけません。   

私なら next 10 sample slot は、**5 つの二本組 package** としてこう切ります。

1. **capability strengthening pair**
   `e13` / `e20` 系を最優先で source-backed actualize します。これは current mainline そのもので、malformed capability second source-backed widening actualization に直結します。static-stop pair なので、lowering / runner / regression の widening に対して費用が低く、checker floor への効きが大きいです。proof/model-check visible canary を static 側から 1 本選ぶなら、この package から取るのが自然です。  

2. **underdeclared static-evidence pair**
   次は `e5` / `e12` 系の underdeclared floor を source-backed にするのがよいです。理由は、same-lineage / target / contract / capability の最小静的証拠が欠けたときに hidden acceptance へ流さない、という current L2 の重要境界を、grammar widening なしで押せるからです。これも static-stop なのでコストが低く、checker / lowering / regression の橋に効きます。もし omitted-target 系の malformed source が現 tranche でまだ parser-pressure-heavy なら、ここだけ contingency として `e17` 系を先に取ってもよいですが、基本線は underdeclared floor の source-backed 化です。  

3. **write-after-expiry contrast pair**
   その次に `e6` / `e7` 系を入れます。これは current L2 の guarded option chain 読み、`lease-expired` 後の later write-capable option 試行、write-after-expiry の `Reject` 境界を、runtime row として source-backed に押せるからです。理論 pressure は強いですが、見た目はかなり practical です。しかも new grammar も shared-space も不要です。  

4. **monotone degradation endgame pair**
   次に `e8` / `e9` 系を入れます。`admit-miss`、途中 failure、later success / final `Reject`、no re-promotion を source row で見せる pair で、canonical chain と monotone degradation を runtime 側から最も強く圧迫できます。proof/model-check visible canary を runtime 側から 1 本選ぶなら、この package の success row を使うのがよいです。理論的に最も「Mir らしい」family でありながら、現行 fixed subset のままで書けます。  

5. **request-contract practical pair**
   最後に `e10` / `e11` 系を入れます。これは `perform on` / `perform via` と `ensure` を絡めた、実行らしい sample です。 practical-looking を入れるならここまでで十分です。shared-space room や I/O scenario を practical の名目で source corpus に入れるより、まず current L2 request-contract family を source-backed にした方が、lowering / runner / theorem/model-check visibility に対する費用対効果が高いです。 

この 10 slot の意図は明確です。**1〜4 本目で static floor を厚くし、5〜8 本目で fallback / `lease` / monotone degradation の runtime contrast を厚くし、9〜10 本目で practical-looking だが still current L2 な request-contract family を入れる**。一方で、`e14/e15` duplicate declaration cluster は declaration hygiene への圧はありますが、現時点では checker / theorem / lowering への多層レバレッジがやや低いので、この 10 本の外に置くのが自然です。また shared-space room sample は current repo では docs-first boundary 側であり、source sample corpus の next 10 に入れるべきではありません。  

sample / lowering / runner / checker / theorem-model-check artifact の連動は、**row 単位ではなく package 単位**で回すのがよいです。1 本足すたびに全部更新するのではなく、2 本組が閉じたところで次の順に流します。
まず representative anchor を決め、対応 fixture を固定し、source `.txt` を fixture stem aligned で追加する。次に mapping matrix に最小 row を足す。ただし matrix には `ladder_order`、`sample_stem`、`representative_anchor_ref`、`fixture_ref`、`coverage_focus`、期待 verdict/outcome までを入れ、**reached stage や bless policy は入れない**。その後 lowering を fail-closed で通し、`run_current_l2_source_sample` の既存 report shape を変えずに runner を通し、verification ladder 側で `static gate` / `interpreter` / `formal hook` の reached stage だけを更新する。最後に regression helper の inventory/regression を package 単位で green にし、必要ならその package から 1 本だけ review-unit / model-check carrier へ fan-out する。source row 全部を artifact-visible にする必要はありません。   

proof/model-check visible sample の扱いは、**new family ではなく promoted view** に徹するべきです。current docs でも source-sample emitted verification artifact wiring、sample-facing theorem/model-check evidence summary、bless/review flowは、source sample row が先に存在し、その emitted route を review-unit / model-check carrier へ fan-out する順序で整理されています。したがって next 10 でやるべきことは、10 本すべてを theorem/model-check visible にすることではなく、**static canary を 1 本、runtime canary を 1 本だけ選んで artifact route を確認する**ことです。私なら static は capability pair から、runtime は monotone degradation pair から選びます。これで checker floor と runtime law の両方に artifact consumer をぶら下げられます。  

host-I/O visible sample は、**next 10 の source corpus には入れない**のが自然です。ここは docs-first I/O / host-facing port boundary の separate gate であり、language core に privileged `stdin/stdout` を入れない、capability-scoped port / adapter boundary に留める、という整理がすでにあります。したがって host-I/O visibility が必要なら、新しい sample family を起こすのではなく、既存 runtime row を `run_current_l2_source_sample` や current-L2 scoped CLI shell の wrapper smoke として再利用するべきです。source corpus を I/O scenario で太らせると、semantics ではなく host contract を先に凍らせやすいです。   

early-freeze を避ける設計 rule は、少なくとも 6 つ必要です。
第一に、**representative prose / fixture corpus / source sample corpus の三層分離を崩さない**こと。source sample は representative prose の昇格でも fixture JSON の逆生成でもありません。
第二に、**sample 追加と grammar widening を結び付けない**こと。`.txt` は final grammar ではなく、companion notation は companion のままです。
第三に、**matrix と ladder を分離する**こと。matrix は identity と対応表、ladder は reached stage、bless は review/regression 読みです。
第四に、**1 package = 1 semantic pressure axis + 1 contrast pair**を守ること。
第五に、**proof/model-check visible と host-I/O visible を derived promotion に留める**こと。
第六に、**named profile catalog や manifest を row 拡大のたびに増やさない**こと。cluster smoke は selection/profile helper で処理し、alias は最小のまま維持するのがよいです。    

要するに、next 10 の最適戦略はこうです。**capability → underdeclared → expiry → monotone → request-contract** の 5 package で source corpus を増やし、proof/model-check visibility はその中の canary row だけを昇格させ、host-I/O visibility は source corpus ではなく wrapper で扱う。これが、current mainline を壊さず、lowering / runner / checker / theorem bridge の全層に効き、しかも final grammar・shared-space・type surface を premature に凍らせない、一番安い増やし方です。    
