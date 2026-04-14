According to documents from 2026-04-13, いま shared-space line を本格化するなら、**最初から final exhaustive catalog を作りにいくのは不自然**です。repo 側の current judgment は、まず docs-first で **identity / auth layering → admission / compile-time visibility → authority / resource ownership** の順に切り、その上で authoritative room と append-friendly room をまたぐ **small working subset** を置き、そこから先の final activation / authority / auth / identity / admission / consistency / fairness catalog は **user-spec-required gate** に残す、という流れです。つまり、自然な順序は「最終カタログを決める」ではなく、「shared-space の最小核を分離し、比較可能な room-profile 軸を揃える」です。  

私なら、shared-space final catalog を次の 6 段に分けます。

| 段 | 先に formalize / docs-first actualize するもの                 | そこで固定する最小核                                                                                                                                                                    | autonomy                         |
| - | -------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| 1 | **membership identity core catalog**                     | `member_ref + principal_ref + member_incarnation + activation_state`、session-scoped membership registry、derived snapshot view、`membership_epoch` / `member_incarnation` split | self-driven                      |
| 2 | **admission / compile-time visibility boundary catalog** | role / capability / visibility / notify path の **over-approximation** だけを compile-time に残し、actual admission / activation / reconciliation は runtime control-plane に残す         | self-driven                      |
| 3 | **authority / resource ownership catalog**               | `owner_ref + delegation_set + handoff_epoch` のような resource authority slot、single owner slot と delegated capability の分離                                                        | self-driven                      |
| 4 | **room profile working-subset catalog**                  | activation rule family、authority placement family、consistency mode family、fairness source family の 4 軸 bundle                                                                 | self-driven だが最終固定は user goal 依存 |
| 5 | **fairness / replay / recovery strengthening catalog**   | `auditable_authority_witness` の最小 shape、rejoin=new incarnation、deactivation による uncommitted action invalidation、committed transition non-revival                              | 前半は self-driven、後半は user goal 依存 |
| 6 | **final operational / deployment catalog**               | exact auth protocol、exact activation profile、consensus/replication、distributed fairness protocol、reconnect/timeout/retry policy、app-specific guarantees                       | user goal required               |

この順序を勧める理由は単純で、**1〜3 が carrier / boundary の整理、4 が room-profile 軸の整理、5〜6 が claim と protocol の整理**だからです。carrier と boundary が未分離のまま fairness や recovery を詰めると、principal continuity、activation、authority handoff、provider placement、replay obligation が 1 つの carrier に潰れてしまいます。現在の docs はそこを明確に避けています。  

## どの catalog から始めるのが自然か

### 1. identity / principal / membership / incarnation / activation_state

ここは **最優先**です。理由は、これが admission、authority、fairness、replay の全ての土台だからです。current working judgment は、participant を plain array や tree source-of-truth にせず、**session-scoped membership registry** を source of truth に置き、array-like / tree-like なものは derived snapshot / UI view に留める、というものです。また churn と causal metadata を同じ carrier に押し込まず、**epoch / incarnation split** を first practical candidate、control-plane separated carrier を next stronger candidate に置いています。したがって最初に formalize すべきなのは、`MemberCore` 相当の identity core と、`membership_epoch` / `member_incarnation` / `activation_state` の最小語彙です。ここを曖昧にしたまま authority や replay を詰めると、leave 後の古い message と rejoin 後の新しい participant を区別できなくなります。   

この段階で **固定してよい最小カタログ**は、次までです。

* membership registry が source of truth
* `MemberCore = { member_ref, principal_ref, member_incarnation, activation_state }`
* `membership_epoch` は membership reconfiguration 側の世代
* array/list/tree は derived snapshot
* rejoin は new incarnation
* leave / rejoin を causal slot deletion と同一視しない

逆に、ここでまだ固定しない方がよいのは、final carrier 名、full control-plane log、membership epoch と route/patch epoch の共有、deactivation visibility の exact timing です。 

### 2. admission policy と compile-time visibility の境界

次に来るのがこれです。current judgment はかなり明確で、**compile-time には role / capability / visibility / notify path requirement の over-approximation だけを残す**、一方で **actual principal satisfaction / activation / active member set / reconciliation は runtime control-plane に残す**、という split を first practical cut にしています。つまり shared-space 側で固定すべきなのは、「この room/action が何を要求しうるか」の declaration surface までであり、「誰が実際に active になったか」までは固定しません。  

ここで formalize すべき最小カタログは、私は次の 2 枚に分けます。

* **declaration catalog**

  * room role family
  * room-local capability / permission ref family
  * visibility role / notify path family
* **runtime control-plane handoff catalog**

  * actual admission event
  * actual activation event
  * actual active-member set
  * reconciliation event

重要なのは、前者を shared-space docs-first の対象にして、後者は still-later の runtime realization に残すことです。raw auth protocol、exact principal set、closed-world exact admission/visibility はここで固定しないのが current repo の line です。 

### 3. authority placement、delegation、resource ownership

この段階で初めて authority / ownership を formalize するのが自然です。identity と admission の分離が済んでいない状態で authority を入れると、participant carrier、resource owner、delegated capability、fairness source が混線します。current working model は、Mir core の ownership/lifetime と shared-space の operational owner slot を同一視せず、**shared resource に対する authoritative write authority slot** として読む方が安全だ、としています。さらに authoritative room では **mutable shared resource ごとに current owner slot は 1 つ**という仮説を first practical candidate に置いています。

ここで formalize してよい最小カタログは、たとえば次です。

* `ResourceAuthority = { owner_ref, delegation_set, handoff_epoch }`
* `single room authority` を first candidate
* `replicated authority` を next candidate
* `relaxed projection authority` を future comparison
* resource owner slot と delegated capability を別 family にする

この cut の利点は、authoritative room の lock / commit / publish / handoff / audit を 1 本の transition sequence で説明できる一方、append-friendly room で delegated append capability を別に残せることです。つまり participant carrier と authority placement を直結しないで済みます。  

## その次に formalize すべきもの

### 4. room profile working subset

ここが shared-space line の本体です。ただし **final exhaustive catalog** にしてはいけません。current docs が推しているのは、room profile を少なくとも次の 4 軸 bundle として扱うことです。

1. activation rule family
2. authority placement family
3. consistency mode family
4. fairness source family

そして minimal authoritative room bundle は
`authority-ack + single room authority + authoritative serial transition + authority_rng`、
append-friendly room の current first choice は
`single room authority` を membership control-plane に使いつつ、data-plane は `append-friendly room` と delegated append capability に寄せる、という読みです。ここで大事なのは、**authoritative game room の bundle を shared-space 全体の既定と誤読しないこと**です。  

したがって、上位 application に寄せる前に shared-space 自体で固定すべき最小カタログは、私は次の **small working subset** だと思います。

* activation rule family

  * `authority-ack` を first practical candidate
  * `full-coverage-like activation` と `quorum-like activation` は comparison
* authority placement family

  * `single room authority` first
  * `replicated authority` next
  * `relaxed projection authority` future
* consistency mode family

  * `authoritative serial transition`
  * `append-friendly room`
  * `relaxed merge-friendly room` は later
* fairness source family

  * `authority_rng` first
  * `delegated_rng_service` next
  * `distributed_randomness_provider` future

この 4 軸 subset までは shared-space 自体で fixed してよい。逆に、actual consensus algorithm 名、deployment topology、raw auth stack、fairness proof protocol は still outside に残すべきです。  

### 5. fairness / consistency / replay / recovery obligation をどの層へ置くか

ここは一括で置かない方がよいです。分けるべきです。

**consistency mode** は L3 shared-space room profile の軸です。`authoritative serial transition` と `append-friendly room` を small catalog として先に持ち、merge-friendly branch は later に残すのが自然です。

**fairness source** も room profile の軸ですが、**fairness trust model** は別軸に分けるべきです。current docs は `authority_rng` と `delegated_rng_service` / `distributed_randomness_provider` の provider placement と、`opaque authority trust` / `auditable_authority_witness` / `delegated provider attestation` / `distributed fairness protocol` の trust claim を同じ軸に潰さないことを強調しています。つまり L3 room profile に置くのは「fairness source family」まで、audit / receipt / attestation / distributed fairness protocol は L2 Mirrorea / runtime-policy 側に寄せて later に残すのが自然です。 

**replay / recovery obligation** は二層に分けるべきです。room profile 側に残してよい最小核は、

* `member_incarnation`
* rejoin = new incarnation
* deactivation による uncommitted action invalidation
* committed transition non-revival

までです。これに対して、timeout / retry / resend / liveness probe / reconnect protocol の exact body は external policy / runtime control-plane に残すのが current working judgment です。ここを room profile に入れ始めると、shared-space line が network policy や UX grace policy を吸い込みすぎます。

## self-driven で詰められる部分と user goal が要る部分

### self-driven で詰めてよい部分

ここは docs-first でかなり進められます。

* membership registry を source of truth にすること
* `member_ref / principal_ref / member_incarnation / activation_state` を identity core に絞ること
* auth stack / admission policy / projection identity を side carriers に分けること
* compile-time を role / capability / visibility / notify path の over-approx に限定すること
* `ResourceAuthority` 的な owner slot / delegation / handoff epoch を整理すること
* room profile を 4 軸 bundle で書くこと
* consistency mode の first practical catalog を `authoritative serial transition` と `append-friendly room` に留めること
* fairness source を `authority_rng` first, `delegated_rng_service` next に置くこと
* `auditable_authority_witness` の minimal witness core を narrow strengthening として切ること
* replay/recovery の最小 room-profile rule を `member_incarnation + uncommitted action invalidation + committed transition non-revival` に留めること

要するに、**carrier split と policy family split までは self-driven** です。これは current docs の autonomy gate とも整合しています。   

### user goal が必要な部分

ここから先は shared-space final catalog になります。現行 docs でも明確に user-spec-required 側です。

* final activation rule

  * `authority-ack` を既定にするか
  * `full-coverage-like activation` を要求するか
  * `quorum-like activation` を入れるか
* exact identity / auth model

  * principal continuity をどこまで room semantics に見せるか
  * auth refresh / external subscription / avatar projection をどこまで room profile に出すか
* final fairness claim

  * authority trust でよいか
  * witness 必須か
  * provider attestation 必須か
  * distributed fairness まで要求するか
* consistency mode catalog の最終形

  * VR social を先に見るのか
  * collaborative editing を先に見るのか
  * Reversed Library / projection-heavy space を先に見るのか
* late leave / in-flight action / reconnect / resend / timeout の exact policy
* reset / notification / moderation / visibility の exact guarantee
* actual control-plane protocol、replication family、consensus family、deployment topology

これは plan/17 がそのまま「shared-space final activation / authority / auth / identity / admission / consistency / fairness catalog」は user specification が必要と書いている範囲です。ここは research だけで MECE な final catalog に閉じない方がよいです。 

## 結論

私の recommendation を一文で言うと、**shared-space line は final catalog から始めず、`membership core → admission boundary → resource authority → room-profile working subset → fairness/replay strengthening` の順で narrow に formalize し、その後の exact activation / fairness / consistency / recovery / protocol は user goal で閉じる**、です。
そして、上位 application に寄せる前に shared-space 自体で固定すべき最小カタログは、

* `MemberCore` を中核とする membership registry
* compile-time over-approx declaration surface
* `ResourceAuthority` 的な owner/delegation/handoff slot
* 4 軸 room profile small subset
* minimal replay/recovery rule

までで止めるのが自然です。これ以上を shared-space 単独で final にすると、VR social、collaborative、Reversed Library の要求差を先取りして固定しすぎます。   

この整理だと、次に actual task package を切るなら、順番は **「membership identity core catalog」→「admission / compile-time visibility catalog」→「authority / resource ownership catalog」→「authoritative room / append-friendly room の room-profile catalog」→「auditable witness / delegated RNG / minimal replay-recovery strengthening」** が最も後戻りが少ないです。  
