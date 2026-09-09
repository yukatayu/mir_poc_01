私の第一推奨は、Mirrorea をいきなり「完全な protocol / transport / consensus の仕様」に落とさず、先に **route snapshot と activation boundary を中心にした小さな docs-first artifact 群**だけを固定することです。Mirrorea の現在の責務は logical naming、routing、overlay insertion、patch activation、path proof、audit、reconfiguration であり、しかも no shadowing、compatibility-preserving overlay、downstream addition、explicit cut activation が強い制約です。さらに current repo では Mirrorea はまだ boundary / principle 整理が中心で、実装より docs-first boundary を先に切る段階です。したがって先に固定すべきなのは wire format や consensus 名ではなく、**何を証明し、何を明示し、どこで route/patch の世代が切り替わるか**です。   

この repo は、そうした docs-first 境界を `specs/examples/*` の companion 文書として先に切り、final parser や final protocol は後段に残す作法を既に持っています。`specs/00-document-map.md` でも、examples 群は final grammar や production API を固定するためではなく、current boundary を安定して読めるようにする補助正本として使われています。Mirrorea でも同じ作法を採るのが自然です。

## 先に固定すべき docs-first artifact

依存順で言うと、先に固定すべきものは 5 つです。

### 1. `RouteBindingSpec`

これは **logical name が、ある activation 世代で、どの route family と overlay obligation に解決されるか** を表す宣言的 artifact です。最小字段は、`logical_name`、`route_family_ref`、`route_epoch_ref`、`required_overlay_refs`、`optional_overlay_refs`、`contract_surface_ref`、`compatibility_guard_refs` くらいで十分です。ここで大事なのは「物理 hop を全部書く」ことではなく、**何が route resolution の結果として選ばれたと見なすのか** を固定することです。Mirrorea は logical name と route rebinding を責務に持ち、Workstream C でも logical naming / route rebinding / basic audit が最小目標ですから、まず binding artifact が要ります。  

### 2. `OverlayCompatibilityRow`

次に必要なのが **overlay が何を壊してよくて、何を壊してはいけないか** を明示する row です。最小でも、`overlay_ref`、`applies_to_route_family_ref`、`precondition_policy`、`guarantee_policy`、`failure_space_policy`、`budget_policy_ref`、`legacy_boundary_ref` を持たせるのがよいです。ここで固定すべきなのは、overlay が「互換性を保つ」ことの最小読みであって、静的に全部証明することではありません。既存 docs では compatibility-preserving overlay の方向として、precondition を強めない、guarantee を弱めない、明示的 contract change なしに time/resource budget を悪化させない、failure behavior を合意 failure space に留める、がすでに出ています。したがって overlay 互換性はまず **宣言的 envelope** として切り、細かい実証は audit と runtime policy に送るのが自然です。  

### 3. `PatchActivationRecord`

patch / route rebind / overlay insertion を **どの explicit cut / activation boundary で有効化したか** を表す artifact です。最小字段は、`activation_ref`、`activation_kind`、`target_logical_name_refs`、`from_route_epoch_ref`、`to_route_epoch_ref`、`cut_ref`、`compatibility_refs`、`in_flight_policy_ref` です。ここで重要なのは、Mirrorea が later cut vocabulary の意味そのものを定義しないことです。`durable_cut` や aggregate success/failure の意味づけは Mir-1 側に残り、Mirrorea は storage / distributed realization / audit representation を担います。したがって `PatchActivationRecord` は cut の意味を再定義するのではなく、**どの activation がどの cut/ref にぶら下がったか** を記録するものに留めるべきです。  

### 4. `RouteReceipt`

path proof / route proof の中心です。私の第一候補は、**完全 transcript ではなく、route snapshot に対する compact receipt** です。最小 shape は次です。

```text
RouteReceipt {
  request_ref,
  logical_name,
  route_epoch_ref,
  selected_route_ref,
  activation_ref,
  required_overlay_refs,
  observed_overlay_refs,
  outcome_ref,
  evidence_refs,
  handoff_ref?
}
```

この shape の要点は二つです。第一に、proof core には **request がどの logical name を、どの route 世代で、どの overlay obligation の下で通ったか** だけを残す。第二に、署名・ack・hop log・transport trace のような重いものは `evidence_refs` に外出しする。これは `durable_cut` / `all_of` で Mir-1 が event surface を最小化し、evidence payload の具体表現を Mirrorea 側裁量に残している設計と整合します。route proof も同様に、**core は ref-based、具象証拠は out-of-line** が自然です。   

### 5. `RouteHandoffReceipt` と `SuspensionBoundaryPolicy`

最後に、route change / patch activation と suspended task の相互作用を hidden にしないための小さな artifact と rule が要ります。私なら first cut では、`RouteHandoffReceipt { task_ref, from_route_epoch_ref, to_route_epoch_ref, handoff_reason_ref, evidence_refs }` を置き、別に `SuspensionBoundaryPolicy` で allowed behavior を定めます。ここで coroutine / suspended task は未決ですが、少なくとも repo は unrestricted model を受け入れていません。したがって first cut は **suspend-across-activation を fail-closed に寄せる** のが最も自然です。  

## 4つの問いへの直接回答

### 1. path proof / route proof の自然な shape

自然なのは、**宣言側と実行側を分ける二層形**です。
宣言側は `RouteBindingSpec`。実行側は `RouteReceipt`。
proof は「何を通るべきだったか」と「実際に何を通ったか」の組で読めるべきで、しかも no shadowing を壊さないため、rejection は元 contract space か明示 versioned contract space に属していなければなりません。したがって proof core には少なくとも、logical name、route epoch、selected route、required overlays、observed overlays、outcome、evidence refs が要ります。逆に、hop-by-hop packet transcript、署名 bundle 全文、transport retry の細部まで core に入れるのは早すぎます。そこは `evidence_refs` と audit non-core に残すべきです。   

ここで特に大事なのは、**proof は single-epoch を既定にする**ことです。1 本の `RouteReceipt` が複数の route epoch をまたぐのを first cut で許すと、partially observed route と rebind の境界が曖昧になります。したがって初期設計では、1 receipt = 1 route epoch / 1 activation snapshot を原則にし、跨ぐなら `RouteHandoffReceipt` を必須にするのがよいです。これは explicit cut activation と audit trace の整合を最も壊しません。 

### 2. overlay compatibility をどこまで static / docs / audit で表すか

私の推奨は、三分割です。

まず **static / docs** に置くのは、overlay の compatibility envelope です。
具体的には:

* どの route family / logical name に適用できるか
* precondition を preserve するか、明示 versioned change か
* guarantee を preserve するか、明示 versioned change か
* failure space を preserve するか、明示 extension か
* legacy wrap なのか、Mir semantics 内部の mechanism なのか

ここは docs-first で固定してよいです。なぜなら no shadowing / compatibility-preserving overlay / legacy boundary explicitness は既に principle としてかなり強いからです。  

次に **audit** に置くのは、どの compatibility row を根拠にその request が処理されたか、実際にどの overlay chain が active だったか、どの contract/versioned rejection surface が使われたか、です。time/resource budget についても、first cut では「静的に完全証明」ではなく、**宣言した envelope と実観測を照合できること** を目標にする方が現実的です。つまり budget は docs に threshold ref を置き、audit で actual measurements / breach / typed rejection を追えるようにする。 

最後に **runtime policy** に残すのは、rollout、canary、replica choice、timeout、load shedding、auth challenge、rate-limit、retry、provider placement です。これらは operations に強く依存し、しかも specs 自体が単一 consensus algorithm や単一 runtime への premature commitment を避けています。したがって docs-first で freeze するのは compatibility envelope まで、runtime は policy と evidence generation に残すのが自然です。 

### 3. patch activation を explicit cut に結びつけるときの in-flight / suspended / partially observed route

ここは rule を先に厳しく切った方がよいです。私の第一候補は次の 5 ルールです。

1. **activation は epoch を上げる。**
   route/overlay/patch の有効化は `activation_ref + route_epoch_ref` を必ず持つ。

2. **request は開始時に route snapshot を pin する。**
   non-suspended な in-flight request は、既定ではその snapshot で完走するか、明示 failure に落ちる。mid-flight silent re-resolve はしない。

3. **suspended task は epoch 変更をまたいで auto-migrate しない。**
   resume 時に old epoch と current epoch が違えば、既定では fail-closed。許す場合だけ `RouteHandoffReceipt` を伴う explicit rebind にする。

4. **partially observed route の splice を禁止する。**
   old route prefix と new route suffix を 1 本の logical request として hidden に継ぐことはしない。継ぐなら `handoff_ref` を立てる。

5. **resume 不可なら explicit failure / cancel / compensate にする。**
   captured resource / suspension assumption / overlay obligation が新 epoch と両立しないなら、hidden repair ではなく明示 outcome へ出す。

この 5 つで、no shadowing と audit trace がかなり守れます。特に 3 と 4 が重要で、route change / patch activation を「気づかないうちに task が新 path に移る」挙動にしないことが肝です。既存 docs でも in-flight coroutine / task state が route change と patch activation をまたぐ時の振る舞いは未決ですが、だからこそ first cut は conservative であるべきです。  

この設計は、shared-space 側で compile-time over-approximation と runtime control-plane を分け、`control_epoch` 相当の lightweight split を first cut にする current first choice とも相性がよいです。私はこれを Mirrorea route/patch 側にも借りるのが自然だと見ます。つまり full control-plane log をいきなり固定せず、まず epoch/ref split だけを入れる。これはあくまで類推ですが、repo 全体の boundary discipline とはよく一致しています。  

### 4. 単一 consensus algorithm に縛られない Mirrorea の最小 protocol surface

ここでは **algorithm nouns を surface に出さない** のが最重要です。
最小 protocol surface は、たとえば次の family で十分です。

```text
resolve_route(logical_name, requested_contract_ref?) 
  -> route_snapshot_ref, route_epoch_ref

propose_activation(change_set_ref, cut_ref)
  -> activation_ref

finalize_activation(activation_ref, outcome_ref, evidence_refs)
  -> activation_receipt_ref

begin_request(request_ref, route_snapshot_ref)
  -> route_observation_ref

resume_or_rebind(task_ref, expected_route_epoch_ref, resume_token_ref?)
  -> continue_pinned | explicit_rebind(new_route_snapshot_ref) | reject

emit_route_receipt(request_ref)
  -> route_receipt_ref
```

この surface のポイントは、`leader`、`term`、`ballot`、`raft_log_index`、`paxos_round` のような algorithm-specific field を持ち込まないことです。必要なのは activation と route resolution の **意味** だけで、合意の実現は evidence generation 側に押しやればよい。Mir-1 `durable_cut` でも、`all_of` の full coverage 条件だけを意味側に残し、evidence の具象形式や表現は Mirrorea 裁量に残しています。Mirrorea route/patch line でも同じ discipline を採るのが自然です。  

つまり first protocol cut は、**semantic-neutral な ref と outcome と evidence slot** までに留めるべきです。
後段に残すものは:

* evidence の具象形式（signature / receipt / log record / proof bundle）
* activation dissemination / retry / timeout / failed closure の protocol
* route snapshot の persistence / replication
* suspend token / handoff token の wire encoding
* concrete scaling orchestration
* consensus family との binding

これなら single-controller 実装でも replicated control plane 実装でも、同じ surface を維持したまま差し替え可能です。  

## 最小設計としての結論

私なら、今この repo で先に固定するのは次の順序です。

1. `RouteBindingSpec`
2. `OverlayCompatibilityRow`
3. `PatchActivationRecord`
4. `RouteReceipt`
5. `RouteHandoffReceipt` + `SuspensionBoundaryPolicy`

そして first rules は次の 6 本です。

* no shadowing
* compatibility row が無い overlay は production path に入れない
* activation は explicit cut + epoch bump でのみ有効
* request は single-epoch route snapshot を pin する
* suspend-across-activation は fail-closed を既定にする
* proof core は compact receipt、重い証拠は `evidence_refs` に外出しする

この cut なら、Mirrorea の docs-first boundary から operational line へ進んでも、まだ単一 consensus algorithm・最終 transport・最終 coroutine semantics・最終 auth protocol に縛られません。その一方で、**route proof が何を証明するのか、overlay が何を壊してはいけないのか、patch activation がどこで世代を切り替えるのか、suspended task をいつ fail-closed にするのか** は先に固定できます。これが no-shadowing と audit trace を壊さない最小設計だと思います。    

次段は、この 5 artifact を `specs/examples/` の companion 文書 2 本程度に切り、`docs-only minimal shape` と `retained-later protocol work` を明確に分けることです。
