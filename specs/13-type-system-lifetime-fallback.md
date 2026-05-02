# 13 — Lifetime / Fallback と Guarded Access Path（Alpha-0）

## role

この文書は、Mirrorea Spaces alpha line における
**lifetime / fallback / guarded reference の alpha-local 規範**を置く。

- ここで固定するのは final public parser grammar ではない
- ここで固定するのは alpha-local typed IR / checker / sample judgment の意味境界である
- final surface syntax、full dependent lifetime theory、public API は still later に残す

## decision level

- `L1`
  - fallback は target lifetime extension ではなく guarded logical access path の availability extension として読む
  - monotone degradation / no re-promotion を守る
  - write capability は fallback や subtyping で strengthen しない
- `L2`
  - explicit inherited chain / snapshot selected / static evidence floor / remote observed reference shape

## core distinction

fallback の principal meaning は次である。

> `c fallback a` は `c` の lifetime を `a` まで延ばさない。
> それは、guard が有効な限り `c` を試し、
> 失効または inadmissible 化したら contract-compatible な後段 option へ
> monotone に degrade する guarded logical access path を作る。

したがって、fallback は object identity preservation、
hidden resurrection、hidden write buffering を意味しない。

## guarded access path model

current alpha-local model では、
guarded reference は 1 つの logical access role / path に対する
有限 ordered option chain として読む。

```text
GuardedRef<T, Role, Cap, Label> = [Option1, Option2, ... , OptionN]

Option = {
  target,
  role,
  lease,
  contract,
  capability,
  label,
  lineage
}
```

各 option は少なくとも次を持つ。

- `target`
  - access target。same logical access path 判定の anchor。
- `role`
  - logical access role / path。
- `lease`
  - lifetime / freshness guard。
- `contract`
  - その option で許可される contract surface。
- `capability`
  - read / observe / write などの capability surface。
- `label`
  - observation / visibility / redaction に関わる label。
- `lineage`
  - inherited/spliced edge が same semantic lineage continuation であることを示す最小 evidence。

## raw ref / guarded ref / inherited chain / snapshot selected

これらは distinct な operation として扱う。

### plain reference

```text
c = ref(d) fallback a
```

- `c` は `d` object / cell を参照する
- `d` の内部 fallback chain は暗黙に継承しない
- `d` が失効したら `a` へ degrade できるが、
  `d` が持っていた別 option を hidden に splice しない

### inherited chain

```text
c = inherit_chain(d) fallback a
```

- `d` が guarded chain を指すとき、
  explicit same-lineage evidence がある場合だけ
  `d` の option chain を new chain に splice してよい
- canonical form は left-to-right flattening で読む

### snapshot selected

```text
c = snapshot_selected(d) fallback a
```

- current selected option だけを capture する
- `d` の full chain は引き継がない
- selection snapshot は object identity merge を意味しない

## canonical fallback chain

canonical form は、単一 logical access path / semantic lineage 上の
finite ordered option chain `o1 > o2 > ... > on` である。

```text
canon(option) = [option]
canon(fallback(x, y)) = canon(x) ++ canon(y)
```

この flattening を admissible として扱うのは次の場合に限る。

1. all options share the required logical access role / path
2. predecessor/successor edge-local lineage evidence がある
3. later option が contract-compatible successor である
4. later option が stronger capability / stronger guarantee を要求しない
5. successor label / redaction surface が monotone non-widening である
6. remote observed successor では membership epoch / observation frontier が stale 側へ戻らない
7. declared target / role / contract / capability / label surface が present である

canonical form が保持する principal information は次である。

- option priority order
- each option の target / role / guard / contract / capability / label / lineage
- remote observed option では membership epoch / observation frontier / freshness guard

canonical form は nested syntax の inside/outside position そのものを保持しない。

## static evidence floor

current alpha-local static same-lineage 判定に必要な最小 declared information は次である。

- `declared access target`
- `declared access role / path`
- edge-local `documented lineage annotation`
- declared contract surface
- declared capability surface
- declared label / redaction surface
- remote observed ref では declared membership / epoch freshness surface

current alpha-local line では、
underdeclared fallback case を dynamic `Reject` へ押し込まない。
minimum evidence floor を満たさない branch は static error とする。

underdeclared には少なくとも次を含む。

- declared target missing
- lineage annotation missing where inheritance/splice is claimed
- contract surface missing
- capability surface missing
- successor comparison に必要な minimum declared information 不足

## static rejection vs dynamic `Reject`

### static rejection

以下は static に reject / underdeclared / malformed へ寄せる。

- missing declared access target
- missing lineage annotation
- lineage annotation mismatch
- different logical access role / path
- successor capability strengthening
- successor label/frontier widening
- remote successor epoch rollback
- declared target mismatch
- declared role/path mismatch
- plain/raw reference をより長寿命な context へ格納して dangling 化させる宣言
- mutable variance violation
- write through declared read-only fallback
- declared contract incompatibility

### dynamic `Reject`

chain 自体は well-formed だが current evaluation で admissible option が尽きる場合だけ、
dynamic `Reject` を使う。

例:

- all options expired
- runtime membership / witness / freshness check failure
- runtime capability absence when static proof is intentionally postponed

## variance and capability rules

recommended current alpha-local variance は次である。

```text
Read<T>        covariant, subject to contract/label
Observe<T>     covariant, subject to contract/label
Write<T>       non-covariant
ReadWrite<T>   invariant
MutableRef<T>  invariant
Linear<T>      invariant
```

追加で次を固定する。

- write capability must not strengthen through fallback
- read-only fallback does not preserve write permission
- fallback successor は stronger capability を要求してはならない
- fallback successor は less-redacted observation surface を作ってはならない
- remote observed successor は older membership epoch / weaker freshness frontier へ戻ってはならない
- mutable/read-write remote reference を covariant にしない

## remote observed reference model

cross-place / cross-process / cross-machine value は raw pointer ではない。
current alpha-local remote observed ref は少なくとも次の carrier を要求する。

- `place identity`
- `target identity`
- `membership epoch / participant incarnation`
- `freshness guard`
- `visibility / observation frontier`
- `read / observe capability`
- optional explicit `write capability`
- `witness refs` when needed
- `fallback chain`
- `label / redaction policy` for debug / visualization

default expectation は read-only / observe-first である。
write-capable remote access は explicit capability と contract を必要とする。

## rollback / atomic_cut / load boundary

rollback、`atomic_cut`、load は option chain の degradation order を巻き戻さない。

- expired lease は rollback で resurrect しない
- degraded fallback position は same lineage で implicit に re-promote しない
- `atomic_cut` は place-local rollback frontier であり、
  lease や fallback order を reset しない
- load は stale lease / witness / membership epoch / fallback position を
  hidden に revive しない

explicit reacquire は future extension としてありうるが、
それは new event / new evidence / new epoch として扱う。
same-lineage implicit resurrection ではない。

## bird / sparkle anchor example

motivating example の correct reading は、
object lifetime inheritance ではなく access-path inheritance である。

```text
BirdAnchor = FriendHead > SelfShoulder
SparkleAnchor = inherit_chain(BirdAnchor) > WorldOrigin
```

ここで起こるのは次である。

- Bird disappearance は Bird object lifetime の終わりである
- Sparkle が FriendHead に留まれるかどうかは
  `BirdAnchor` chain を explicit に inherit したかで決まる
- Friend logout 後の degrade は `SelfShoulder` や `WorldOrigin` へ向かう
- Bird object lifetime を hidden に延長して説明しない

## proof / checker obligations

future checker / proof line が最低限 preserve すべき obligation は次である。

1. no successful dangling read
2. no successful write through expired or read-only option
3. no implicit chain propagation
4. inherited chain requires lineage evidence
5. fallback successor capability is monotone non-increasing
6. successor contract is contract-compatible
7. successor label / redaction surface is monotone non-widening
8. remote observed successor does not move to older epoch / weaker frontier
9. rollback / `atomic_cut` / load do not resurrect expired lease / freshness
10. canonicalization preserves target / role / observable choice for well-formed chains
11. underdeclared fallback is static error
12. reacquire is explicit, not hidden re-promotion

## required sample references

current alpha-local required sample family は少なくとも次を含む。

- `LIF-01` raw dangling reference rejected
- `LIF-02` fallback extends access path
- `LIF-03` nested inherit chain valid
- `LIF-04` plain ref does not inherit
- `LIF-05` underdeclared fallback static error
- `LIF-07` capability promotion rejected
- `LIF-08` write after read-only fallback rejected
- `LIF-10` `atomic_cut` no re-promotion
- `LIF-11` bird / sparkle anchor inheritance
- `LIF-14` remote observed ref stale membership

sample inventory の current repository memory は
`plan/39-type-system-freeze-roadmap.md` と `samples/alpha/lifetime-fallback/` を参照する。

## helper-local positive acceptance artifact boundary

current alpha-local line では、selected positive rows に限って
helper-local synthetic acceptance artifact を使ってよい。

- これは alpha-local / helper-local / non-public synthetic evidence である
- negative-static floor の `expected_static.checked_reason_codes` /
  `detached_noncore.reason_codes` とは別 carrier として扱う
- positive row の証拠は negative reason code の不在ではなく、
  explicit `expected_acceptance.checked_acceptance_rows` と
  `detached_noncore.acceptance_rows` の一致で判断する
- accepted scope は `alpha-acceptance-floor` に限る
- scope mismatch は reject する
- これは parser/runtime implementation、public checker verdict、
  final remote-reference API completion を証明しない

current helper-local acceptance floor で admissible なのは次だけである。

- `LIF-02` fallback extends access path
- `LIF-03` nested inherit chain valid
- `LIF-04` plain ref does not inherit

current helper-local positive rows は少なくとも次の kind family を持つ。

- `fallback_chain_canonicalized`
  canonical chain、capability、monotone degradation を明示する
- `inherited_chain_spliced_with_lineage`
  source chain、appended fallback、canonical chain、lineage edges、
  implicit propagation 不在を明示する
- `plain_ref_boundary_preserved`
  plain ref が inner chain を hidden に splice していないことを明示する

`LIF-15` は runtime / remote / richer lineage meaning を濃く含むため、
current helper-local acceptance floor には上げない。

## helper-local snapshot-selected artifact boundary

current alpha-local line では、
`snapshot_selected` positive row を dedicated helper-local carrier で扱ってよい。

- これは alpha-local / helper-local / non-public synthetic evidence である
- `snapshot_selected` は `inherit_chain` と `plain ref` とは別 semantics であり、
  selected option capture と excluded option 不在を explicit に示す必要がある
- negative-static floor の `reason_codes` や helper-local positive acceptance floor の
  `acceptance_rows` とは別 carrier として扱う
- fixture 側は `expected_snapshot.checked_snapshot_rows` /
  `expected_snapshot.checked_snapshot_scope` を持ち、
  artifact 側は `detached_noncore.snapshot_rows` /
  `detached_noncore.snapshot_scope` を持つ
- accepted scope は `alpha-snapshot-selected-floor` に限る
- scope mismatch は reject する
- positive row の証拠は selected option capture と non-selected option exclusion を
  explicit row で示すことであり、acceptance row や negative reason code の不在ではない
- `snapshot_freezes_selected_option` は snapshot 時点で selected option choice を
  row 内に固定するという意味に限る
  - temporal pinning、target lifetime extension、later degradation 不可 を意味しない
  - appended fallback への later degradation possibility は別に残る
- これは parser/runtime implementation、runtime object deletion behavior、
  public checker verdict、final remote-reference API completion を証明しない

current helper-local snapshot-selected floor で admissible なのは次だけである。

- `LIF-13` snapshot selected anchor

current helper-local snapshot-selected row は少なくとも次の kind family を持つ。

- `snapshot_selected_anchor`
  source chain、selected option、appended fallback、resulting chain、
  excluded options、full-chain inheritance 不在、snapshot freeze を明示する

## helper-local anchor-handoff artifact boundary

current alpha-local line では、
object deletion 後の inherited anchor-chain outcome を dedicated helper-local carrier で扱ってよい。

- これは alpha-local / helper-local / non-public synthetic evidence である
- `bird/sparkle` handoff は deleted object lifetime extension ではなく、
  inherited anchor-chain outcome と later degradation order を explicit に示す必要がある
- negative-static floor の `reason_codes`、helper-local positive acceptance floor の
  `acceptance_rows`、snapshot-selected floor の `snapshot_rows` とは別 carrier として扱う
- fixture 側は `expected_anchor_handoff.checked_anchor_handoff_rows` /
  `expected_anchor_handoff.checked_anchor_handoff_scope` を持ち、
  artifact 側は `detached_noncore.anchor_handoff_rows` /
  `detached_noncore.anchor_handoff_scope` を持つ
- accepted scope は `alpha-anchor-handoff-floor` に限る
- scope mismatch は reject する
- object lifetime extension は `false` でなければならず、
  hidden object reference retained は `false` でなければならない
- positive row の証拠は explicit inherited chain、appended fallback、
  post-delete chain、later degradation order を示すことであり、
  acceptance row、snapshot row、negative reason code の不在ではない
- これは parser/runtime implementation、runtime deletion behavior、
  public checker verdict、`FAIRY-05` reacquire-after-return completion、
  final remote-reference API completion を証明しない

current helper-local anchor-handoff floor で admissible なのは次だけである。

- `LIF-11` bird / sparkle anchor inheritance

current helper-local anchor-handoff row は少なくとも次の kind family を持つ。

- `bird_sparkle_anchor_chain_inherited_after_object_delete`
  deleted object、inheriting effect、source chain、appended fallback、
  post-delete chain、object lifetime extension 不在、hidden object reference 不在、
  later degradation order を明示する

## remaining positive-row carrier inventory boundary

current alpha-local line では、remaining positive rows を
docs-first / row-specific carrier inventory として固定してよい。

- これは actualization ではなく blocker inventory である
- current acceptance floor を widen しない
- parser/runtime bridge や public checker verdict を追加しない

current inventory で明示しておくべき row-specific blocker は次である。

- `LIF-15`
  requires dedicated remote-observed reference carrier
  `remote_observe_scope = alpha-remote-observe-floor`
  - place identity
  - target identity
  - membership epoch
  - participant incarnation
  - freshness frontier
  - visibility / observation frontier
  - read / observe capability
  - fallback chain
  - label / redaction policy
  - read-only covariance relation under contract / label
  - stale-frontier rejection line

inventory closeout は
`LIF-15` を helper-local acceptance row、snapshot row、anchor-handoff row、
remote-observe row、remote-reference runtime completion として actualize したことを意味しない。
`LIF-15` は上記 `remote_observe_scope` の minimum carrier が fixed されるまで
actualize しない。

## deferred

この spec で intentionally deferred に残すのは次である。

- final parser grammar / final reserved keyword set
- final surface syntax for `inherit_chain` / `snapshot_selected`
- full dependent lifetime / region polymorphism
- complete capability lattice
- full proof mechanization
- public remote-reference API surface
- distributed save/load protocol

## stop line

- fallback を target lifetime extension と書かない
- implicit chain propagation を default semantics と書かない
- read-only fallback から write capability を得たことにしない
- expired lease / stale witness / stale membership を rollback / load で revive しない
- this alpha-local spec を final public grammar / final public API freeze と混同しない
