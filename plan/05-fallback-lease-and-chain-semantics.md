# plan/05 — fallback / `lease` / chain semantics

## この文書の役割

この repo で最も drift を起こしやすいのが fallback / preference chain / `lease` まわりである。
ここでは current L2 の読みと、「寿命の長い外側 option へ fallback して使い続けたい」という直感の一致点 / 不一致点を切り分ける。

## current L2 の基本読み

### current L2 settled

- fallback は **guarded option chain**
- `lease` は option ごとの lifetime guard
- same-lineage chain は **left-to-right monotone degradation**
- **earlier option への再昇格禁止**
- `lease-expired` 後は later write-capable option を試してよい
- later candidate も尽きれば request-level `Reject`
- rollback / `atomic_cut` は degradation order を巻き戻さない

## canonical chain の向き

current L2 では `A > B > C` を次のように読む。

- `A` が最初の候補
- `A` が explicit failure または non-admissible なら `B`
- `B` も使えなければ `C`
- 途中で `A` へ戻らない

ここで残る意味は **候補列の優先順**であって、nested outer / inner の包み構造ではない。

## `lease` の位置づけ

- `lease` は option-local lifetime guard である
- 独立した新 failure class ではない
- `lease-expired` はその option が success-side choice になれないことを表す
- current L2 では、`lease-expired` は formal non-admissible subreason として audit metadata に残る

## monotone degradation

### settled

- degradation は monotone である
- same semantic lineage で stronger capability へ戻らない
- write-capable option から read-only option へ弱くなることはありうる
- write-after-expiry でも earlier option の resurrection は起こらない

### これが意味すること

- fallback は “包みを剥がして戻る” 操作ではない
- fallback は “同じ chain の後段候補へ進む” 操作である

## write-after-expiry の扱い

| 状況 | current L2 の読み |
|---|---|
| 先頭 write-capable option が `lease-expired` | その option は success-side candidate から外れる |
| 後段に write-capable option がある | later option を試行してよいが、それだけで success は確定しない |
| 後段に write-capable option が無い | request-level `Reject` |
| read-only option しか残らない | capability mismatch は narrative explanation、最終 outcome は `Reject` |

これは `e6`、`e7`、`e8`、`e9` の regression fixtures で machine-check されている。

## rollback / `atomic_cut` と degradation order

### current L2 settled

- `try` / rollback は local state を巻き戻す
- `atomic_cut` は rollback frontier を更新する
- どちらも chain の degradation order そのものは巻き戻さない

### 誤読しやすい点

rollback を見ると「earlier option へ戻れる」と感じやすいが、current L2 では戻るのは state だけである。
候補列の優先順は保存される。

## user intuition との一致点 / 不一致点

| 論点 | current L2 | 長寿命 outer option 直感 | 一致 / 不一致 |
|---|---|---|---|
| fallback の基本像 | guarded option chain | outer wrapper の寿命延長 | 不一致 |
| 後段候補で継続できるか | yes | yes | 一致 |
| write-after-expiry 後に later write-capable option を試せるか | yes | yes | 一致 |
| earlier option に戻れるか | no | 戻れそうに見える | 不一致 |
| rollback で degradation order も戻るか | no | 戻れそうに見える | 不一致 |

## nested outer/inner 読みが drift を生む点

current L2 では、nested outer / inner の見た目が次の誤読を生みやすい。

- outer-longer-lifetime wrapper 読み
- earlier option への再昇格読み
- fallback を control-flow 包みと読む誤読

この drift を抑えるために current L2 では次を採っている。

- canonical chain へ flatten する
- edge-local lineage annotation を要求する
- explicit edge-row form を暫定 companion notation にする

## regression fixture で machine-check している性質

| fixture | machine-check している点 |
|---|---|
| `e3-option-admit-chain` | option-local `admit` miss は non-admissible skip、later option success |
| `e6-write-after-expiry` | write-capable option expiry + later read-only only -> `Reject` |
| `e7-write-fallback-after-expiry` | `lease-expired` 後に later write-capable option で success |
| `e8-monotone-degradation-reject` | `admit-miss`、middle failure、final `Reject`、no re-promotion |
| `e9-monotone-degradation-success` | `admit-miss`、middle failure、later success、no re-promotion の success-side 補完 |

## settled と tension の切り分け

### settled current L2

- guarded option chain
- left-to-right monotone degradation
- no re-promotion
- write-after-expiry try-later-or-`Reject`
- rollback / cut non-interference

### tension として残るもの

- human intuition は outer wrapper 読みに引かれやすい
- notation の見た目次第で drift が再発しやすい
- final parser syntax をまだ決めていないため、surface での誤読防止は companion notation と prose に依存している

## この先の扱い

current L2 では semantics を outer-longer-lifetime へ戻さない。
必要なのは次である。

- prose で drift を明示する
- representative examples と regression fixtures で読みを固定する
- notation を outer/inner 誤読しにくい方向へ磨く
