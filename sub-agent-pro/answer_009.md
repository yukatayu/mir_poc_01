現時点の文書群に基づくと、自然な最終像は **`place = deployment unit` ではなく、`place` は deployment があとから実現する logical execution locus である**、という切り方です。`place` は Mir-0 では local state / rollback scope / ownership transfer を解釈する最小 locus であり、same-place と cross-place を区別できればよい一方、routing / naming / distributed placement policy は Mir-0 の外に置かれています。さらに `atomic_cut` も current `place` の rollback frontier だけを確定し、process-wide sync や distributed finalization を意味しません。したがって current spec の筋は、`place` を process / server / pod へ早く同一視するのではなく、**semantic locality をまず固定し、その後で operational realization が写像する**、です。 

この前提で、私は対応関係を次の 4 段で切るのが最も自然だと思います。

`place`
→ **logical place realization**（どの runtime locus として走るか）
→ **authority / room / host binding**（どの logical route・owner slot・port/adapter で外界に接続するか）
→ **deployment topology**（process / address space / container / pod / server / replica group）

重要なのは、ここを **1:1 にしない**ことです。1 つの process が複数の `place` を同居させてもよいし、逆に 1 つの logical authority が replica 群で実現されても、外からは 1 本の authoritative stream として見せる、という構成が取り得ます。L1 Mir Core、L2 Mirrorea Fabric、L3 shared-space、L0/external substrate の責務分離は、まさにこの late binding を可能にするためのものです。  

## 1. `place` と process / thread / address space / server / pod / actor / room authority の関係

私の推奨はこうです。

* **thread**

  * これは最下層の scheduler / implementation detail に留めるのがよいです。
  * `place` の semantic identity を thread に結びつけるべきではありません。
  * rollback frontier や ownership transfer の単位を thread にすると、scheduler choice が semantics に漏れやすいからです。`place` は L1、thread は L0/external です。 

* **process / address space**

  * これは **最初の practical な realization unit** です。
  * early phase では「1 process に複数 `place` を同居」させるのが自然で、逆に isolation / fault / security 要件が出たところだけを後で process 分離すればよいです。
  * つまり process は `place` の meaning ではなく、`place` 群の containment / isolation policy です。 

* **server / pod / node**

  * これは deployment topology 側です。
  * Mirrorea が logical name・routing・rebinding・audit を持ち、実ノード上の実現を引き受けるので、server / pod は Mir core ではなく Mirrorea / host / orchestration 側に置くのが整合的です。 

* **actor**

  * これは **runtime family / realization style** として扱うのがよいです。
  * もし actor が「local state を 1 箇所に持ち、明示的 message boundary を持ち、rollback/cut discipline を壊さない」なら、1 `place` または `place` group の実現候補になれます。
  * ただし actor model そのものを core meaning に上げるのは早いです。current spec でも coroutine / routing / suspension / scheduler は Mir-0 の外に置かれています。 

* **room authority**

  * これは generic な `place` 同義語ではなく、**shared-space / Mirrorea 側の operational role** です。
  * 文書群は、participant carrier、resource owner slot、delegated capability、authority placement、consistency mode、fairness source を別軸で持つべきだとしています。`single room authority` は room-level authoritative owner slot / write authority slot の最小候補であって、participant や process や pod へ単純還元していません。

要するに、`place` は **semantic locality**、process/address space は **realization containment**、server/pod は **deployment topology**、room authority は **policy/ownership role** です。この 4 つを混ぜないのが一番きれいです。  

## 2. compile / build / packaging はどう切るべきか

ここも、**「1 Mir program → 1 executable」でも「1 place → 1 executable」でもなく、二段構成**が自然です。

自然な流れはこうです。

1. **semantic compile**

   * source / syntax-backed sample から `Program` へ落とす
   * checker floor と runtime skeleton が読める semantic artifact を作る

2. **realization planning**

   * `place` 群をどの runtime locus に同居させるか
   * authority slot をどこに置くか
   * host-facing port / adapter / route binding をどう与えるか
   * runtime parameter を何にするか
     を別 artifact として与える

3. **packaging / deployment**

   * binary / image / container / pod / service / replica group を作る
   * 同じ binary image を複数 instance 起動してもよいし、役割ごとに別 executable を切ってもよい

current repo の compile-ready line でも、`run_current_l2_runtime_skeleton` は `Program` / `FixtureHostPlan` / optional parser bridge input を explicit input に取り、`run_current_l2_source_sample` は thin facade、CLI はさらに別 gate、host-facing boundary はさらに separate gate として切られています。これは、**semantic artifact、operational wrapper、installed binary/final host contract を分離する方向**を強く示しています。  

したがって最終像としては、「1 つの Mir program から複数 runtime instance へ落とす」方が自然です。ただし、それは **複数 executable を core が直接吐く**というより、**1 つの semantic program から、複数の realization / packaging choice が派生する**と読む方がよいです。binary の種類数と runtime instance 数は切り離すべきです。多くの場合は、`place` ごとに binary を増やすより、**runtime kind ごとに binary image を作り、instance ごとの差は binding / parameter / authority profile で与える**方が運用的にきれいです。 

## 3. route binding、host plan、runtime parameter、authority placement はどこに置くか

ここはかなり明確です。

**route binding / route rebinding / path proof / audit** は Mirrorea Fabric 側です。Mir core は `place` を持つが、どこでどう route されるかまでは決めません。Mirrorea が logical name、physical route、rebinding、overlay insertion、patch activation、audit を担います。 

**host plan** は current repo では、あくまで parser-free PoC / harness 側の artifact です。`.host-plan.json` は runtime fixture で必須の declarative stub asset ですが、production manifest ではありません。さらに helper stack 側でも、`FixtureHostPlan` coupling を final public contract として固定しないよう guard が入っています。したがって、host plan は language core ではなく、**current では test harness artifact、将来は host-facing operational layer の候補**です。ここは docs-first で境界を守るべきで、今の `FixtureHostPlan` を deployment plan の正本に昇格させるのは早いです。 

**runtime parameter** は host-facing port / adapter / typed-effect boundary 側です。現在も `stdin/stdout` を privileged primitive として language core に入れず、capability-scoped input/output port / adapter boundary を docs-first cut に置いています。visualizer / host substrate / FFI / engine adapter も separate gate です。だから env var、port、credential、sample selector、host input mode のようなものは core ではなく **operational wrapper / host adapter** に置くのが筋です。 

**authority placement** は shared-space / Mirrorea 側です。shared-space line は compile-time には role / capability / visibility / notify path の over-approximationだけを残し、actual activation / admission / reconciliation / active member set は runtime control-plane に残す first practical cutを既に採っています。`single room authority` / `replicated authority` / `relaxed projection authority` も authority placement family として比較しており、algorithm 名や deployment topology と直結させていません。したがって authority placement は core language ではなく、**room profile / runtime control-plane / operational realization** 側に置くのが自然です。 

## 4. `place` を保ったまま multi-node 実行へ進む中間段階

ここは、次の段階を踏むのがよいです。

### 段階A — single-process, multi-place semantics

まずは current line を素直に延長し、**1 つの runtime skeleton の中に複数の semantic `place` を持てる**ようにする段階です。重要なのは、同一 address space にいても `place` ごとに local state / rollback frontier / ownership transfer を分けることです。`place` を process と同一視しないまま、same-place / cross-place の区別だけを先に濃くできます。`atomic_cut` が process-wide sync ではなく place-local frontier であることとも整合します。 

### 段階B — single-host, multi-process / actorized realization

次に、logical `place` の一部を別 process / actor に切り出します。ただしこの段階でも、`durable_cut` や distributed finalization を language core に入れない。やるのは **logical place realization の分割** と **route binding の明示化** です。Mirrorea の logical naming / route rebinding / audit が効いてくるのはこの段階からです。 

### 段階C — room/profile based authority placement

shared-space の authoritative room では、participant carrier と別に、activation rule・authority placement・consistency mode・fairness source を room profile として束ねます。ここで `single room authority` を最小 operational candidate に置き、必要なら RNG だけ `delegated_rng_service` に差し替える、といった実運用寄りの比較ができます。この段階でも deployment topology 自体は still outside に残せます。

### 段階D — logical single authority, operationally replicated realization

その後で、`replicated authority` を operational next candidate として導入します。ここで大事なのは、**replica 群を複数 semantic place に見せない**ことです。shared-space docs は “logical authority role is one, internally authority_group may realize it” という読みを許しています。つまり semantics は single authority stream のまま、failover / replication は operational realization 側で吸収するのがきれいです。

### 段階E — only then `durable_cut` / aggregate evidence / multi-node finalization

`durable_cut`、aggregate attempt、`all_of` profile、per-place evidence correlation のようなものは Mir-1 / Mirrorea の later line です。これは `place` の core meaning を変えるのではなく、**複数 `place` にまたがる finalization の観測と audit を後段で足す**段階です。ここまで来て初めて multi-node durable execution の semantics を本格化させればよいです。 

## 今のうちに docs-first で切っておくべき boundary

ここは早めに明文化した方がよいです。

1. **`place` は process / pod / server ではない**

   * semantic locality であって deployment artifact ではない
   * process/address space は first practical realization unit に過ぎない

2. **`place` realization と authority placement は別**

   * generic `place`
   * room authority
   * resource owner slot
   * delegated capability
   * participant carrier
     を混ぜない

3. **`FixtureHostPlan` は current PoC harness artifact であって production deployment manifest ではない**

   * この coupling を final public contract に昇格させない

4. **library contract / CLI / host-facing adapter を 3 段で分ける**

   * library-side final public contract
   * operational CLI wrapper
   * host-facing port / adapter / FFI / engine integration
     を別 gate にする

5. **compile-time over-approximation と runtime control-plane を分ける**

   * role / capability / visibility / notify path requirement は compile-time
   * actual activation / admission / active set / reconciliation は runtime

6. **deployment topology は room profile の外に残す**

   * room profile に入れるのは activation rule family, authority placement family, consistency mode family, fairness source familyまで
   * actual consensus algorithm 名、reconnect protocol、deployment topology は outside

この 6 本を docs-first で切っておけば、将来 multi-node へ進んでも `place` の core meaning を汚さずに済みます。   

最後に、設計判断として一番強く勧めたい一文だけ書くと、**`place` は “where semantics localize”, deployment artifact は “how realization is packaged” と分けるべき**です。前者を Mir Core に、後者を Mirrorea / shared-space / host-facing adapter / orchestration に押し分ける。この切り方なら、1つの Mir program から単一 binary にも複数 runtime instance にも落とせますし、authoritative room を single room authority でも replicated authority でも実現できます。それでいて、`place` の rollback / ownership / cut semantics は崩れません。   
