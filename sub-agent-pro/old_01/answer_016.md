According to a document from 2026-04-13, 既存の risk register に書かれている項目を一段抽象化すると、いま本当に警戒すべき blind spot は次の 8 点です。ここでは「final parser grammar 未固定」「dynamic membership OPEN」のような明示済み論点そのものではなく、その背後にある“後で一気に高コスト化する二次リスク”を優先しています。  

## 1. 最重要: ratchet 各層のあいだに「意味保存契約」がまだない

**なぜ危ないか**
今の repo は、representative programs、fixture corpus、source-sample corpus、parser-to-`Program` lowering、runtime skeleton、formal hook を意図的に別 layer にしています。これは早期固定を避けるうえで正しいですが、その反面、「どの層からどの層へ何が意味保存されるのか」が named obligation として立っていません。結果として、sample が runnable でも、それが current L2 semantics を検証しているのか、単に 1 本の lowering / helper path を検証しているのかが曖昧なまま進む危険があります。これは sample-driven ratchet の根幹リスクです。  

**今から軽く打てる予防線**
`representative -> fixture`、`fixture -> source sample`、`source text -> Program`、`Program -> runtime report`、`runtime report -> formal hook` の各遷移ごとに、`must preserve / may erase / must explain mismatch` を 1 行で書いた **semantic preservation matrix** を置くべきです。sample や fixture を増やすたびに、そのどの edge を evidence 化しているかを明示するだけでも効果が大きいです。

**later でよいか**
よくありません。full proof は later でよいですが、**named preservation obligation の表だけは今**必要です。これがないと Macro 4/5 の成果が「層をまたいで積み上がった」というより「局所 helper が増えた」に見え始めます。  

## 2. 最重要: patch / overlay / rebinding を許すための refinement 関係が未定義

**なぜ危ないか**
core の原則として downstream addition、no shadowing、compatibility-preserving overlay はかなり強く置かれています。しかし現状は still prose-level で、「互換」とは何に対する refinement かがまだない。contract inclusion、failure-space preservation、time/resource budget の悪化禁止、audit/path-proof の保存が、それぞれ別々に解釈されうる状態です。Mirrorea runtime、public API versioning、backend 側の rewiring が入った時に、この曖昧さは一気に爆発します。  

**今から軽く打てる予防線**
docs-only で十分なので、`overlay / patch / rebinding admissibility relation` を 1 本立てるべきです。最小でも、`contract`, `failure`, `resource/timing`, `audit/path-proof` の 4 軸について、「何が preserved されれば admissible か」を checklist 化した方がよいです。

**later でよいか**
full formalization は later でよいですが、**refinement の観点名だけは今**必要です。これがないと safe evolution が slogan のまま残ります。  

## 3. 最重要: checker / proof / model-check / runtime-policy のあいだに単一の trusted semantic kernel が見えにくい

**なぜ危ないか**
今の整理は賢くて、checker floor と residual proof obligation を分け、public checker を later に残し、theorem bridge と model-check carrier を narrow に actualize しています。ただ、その分だけ「各 line が何を同じ property として見ているか」の meta-spec がないと、四つの別々の正しさが生まれます。checker は malformed floor を見て、theorem side は invariant 名だけを見て、model-check carrier は別 abstraction を見て、runtime policy はまた別の closure を持つ、というズレが起きやすいです。  

**今から軽く打てる予防線**
`obligation_kind` ごとに、`subject language / trusted assumptions / discharge authority / artifact semantics / appeal path` の 5 要素を固定した表を作るべきです。特に `same-lineage floor`, `canonical normalization`, `rollback-cut non-interference`, `authority-serial` のような主要 obligation については、どの層が最終 authority なのかを明示した方がよいです。

**later でよいか**
public checker API や concrete tool binding は later でよいです。ただし **“どの obligation を誰が discharge するのか” の台帳は今**必要です。  

## 4. 最重要: current runnable core が `Reject` 偏重で、failure algebra の拡張コストを過小評価しやすい

**なぜ危ないか**
Mir core 自体は `Reject / Approximate / Compensate` を持っていますが、current parser-free PoC が machine-check しているのは実質 `success / explicit_failure / Reject` です。これは今は合理的です。ただし host-facing I/O、durable_cut、external adapter、public shell が進むと、approximation と compensation は「あとで足せばよい装飾」ではなく、effect taxonomy の中心になります。そこで retrofitting すると、runtime outcome carrier、trace schema、formal hook row、proof obligation、public report 全部に波及します。  

**今から軽く打てる予防線**
実装は不要なので、`effect class × outcome class` の matrix を先に置くべきです。最低でも、`reversible / irreversible / compensable / approximable / durable-obligation-bearing` の分類と、rollback / cut / host I/O との関係を docs-only で切るべきです。

**later でよいか**
full carrier actualization は later でよいです。ただ、**taxonomy の宣言だけは今**やる価値があります。ここを先送りすると、後で `Reject` の使い方が広すぎて苦しくなります。  

## 5. 高優先: async-control / memory-order line で「どの invariant が scheduler-independent か」が未整理

**なぜ危ないか**
low-level memory-order を急いで core に入れない判断自体は正しいです。ただ、いま不足しているのは low-level 語彙ではなく、**どの invariant が scheduler や room policy に依存してはいけないか** の表です。これがないと、later shared-space / authority / fairness の policy assumption が、気づかないうちに core invariant や proof obligation に逆流します。  

**今から軽く打てる予防線**
`specs/09` の invariant ごとに、`core-local / checker-local / runtime-policy / shared-space-policy / backend-policy` の依存区分を 1 回付けるべきです。これだけで、将来の async-control line が core へ何を逆流させてはいけないかがかなり明確になります。

**later でよいか**
actual authority-serial contract や memory family の精密化は later でよいです。ですが **dependency labeling は今**入れた方がよいです。  

## 6. 高優先: shared-space は carrier split が進んでいるが、misbinding / replay / stale-authority threat model がまだ弱い

**なぜ危ないか**
membership registry、member incarnation、identity/auth layering、admission visibility、authority/resource ownership、delegated RNG provider、witness core まで整理は進んでいます。問題は、これらが **分割としては正しくても、何を防ぐための分割か** が adversarial / confusion case としてまだ弱いことです。たとえば stale join replay、authority handoff race、provider substitution、projection identity と principal identity の混同、old incarnation の late message などです。ここが弱いと、局所判断は全部もっともらしいのに、全体として unsafe になります。 

**今から軽く打てる予防線**
shared-space docs に protocol を入れる必要はありません。代わりに、6〜10 個の **forbidden confusion table** を作るべきです。各行に「何を取り違えてはいけないか」「それを防ぐ最小 field / ref は何か」を書く。`principal_ref`, `member_incarnation`, `control_epoch`, `authority owner ref`, `provider receipt ref`, `witness_kind` あたりを、その table にぶら下げるとよいです。

**later でよいか**
concrete auth protocol や consensus family は later でよいです。ただし **confusion / replay の threat table は今**作った方がよいです。runtime realization に入ってからでは遅い。  

## 7. 高優先: public API / tooling / backend line で、carrier の promotion / retirement policy がない

**なぜ危ないか**
今の repo には、AST fixture、run report、source-sample report、runtime skeleton report、detached artifact、review-unit、model-check carrier、shell concern など、thin だが distinct な carrier がかなりあります。これは narrow actualization としては正しいです。ただし、「どれが experimental で、どれが bridge-only で、どれが public candidate で、どれを later に捨てるのか」が policy 化されていないと、non-production JSON や helper-local shape が accidental ABI になります。これは public checker / CLI / backend に進むときの典型的な爆発点です。  

**今から軽く打てる予防線**
各 carrier / report / helper surface に、`experimental / non-production / bridge-only / public-candidate / public` の lifecycle label を付けるべきです。さらに promotion 候補には、`predecessor / successor / retirement trigger` を 1 行で要求する。これだけで accidental freezing をかなり防げます。

**later でよいか**
よくありません。特に CLI / public checker / runtime skeleton 周りは、**今が一番安く切れる時期**です。  

## 8. 監視対象: sample-driven ratchet が malformed/static 側に寄り、positive expressivity debt を溜める

**なぜ危ないか**
近接 mainline が malformed capability widening や static clusters に寄るのは理解できますし、現時点では合理的です。ただ、その結果として「よく reject できる」「よく explain できる」系の設計が先に厚くなり、将来本当に通したい positive path が十分に stress されないまま進む危険があります。とくに richer contract、複数 admissible option、cross-place への圧力、resource transfer、useful successful programs の expressive cliff が later surprise になりやすいです。  

**今から軽く打てる予防線**
docs-only でよいので、5〜8 本の **positive frontier set** を別枠で持つべきです。新しい syntax は不要で、今の companion / fixture / source-sample の範囲で、「将来も殺してはいけない成功系」を固定する。近接 task ごとに、その set を preserve しているかだけ確認すれば十分です。

**later でよいか**
runnable expansion は later でよいですが、**frontier set の選定だけは今**やっておいた方がよいです。Malformed bias は自覚が薄いまま積み上がりやすいからです。  

結論として、いちばん先に相談しておくべきなのは **1. 意味保存契約**, **3. trusted semantic kernel**, **4. failure algebra の先回り**, **6. shared-space の misbinding/replay threat model** です。これらは later workstream の「中身」ではなく、later workstream が core に逆流しないためのガードレールだからです。  
