# plan/04 — Mir current L2 核心

## この文書の役割

ここでは、current L2 で既に読みが揃っている Mir 核心をまとめる。
対象は parser grammar ではなく、意味論・failure・rollback・fallback・audit の境界である。

## Mir-0 / Mir-1 / Mirrorea の境界

### Mir-0 に残すもの

- event DAG と因果
- `place`
- effect request の最小操作（本文では `perform` と表記）
- contract
- 最小 failure space
- primitive fallback
- local rollback を伴う `try`
- place-local `atomic_cut`
- linear resource と monotone lifetime

### Mir-0 に入れないもの

- `barrier`
- `durable_cut`
- full fallback normalization の production semantics
- coroutine / emit / overlay detail
- distributed scheduler

### Mir-1 / 後段に送るもの

- `durable_cut`
- persistence / distributed finalization
- richer runtime
- static analysis / theorem proving と強く結びつく要素

### Mirrorea に残すもの

- logical name
- route rebinding
- overlay registration
- patch activation
- audit / path proof

Mir current L2 は、この Mir / Mirrorea 境界を維持したまま PoC を積んでいる。

## failure space

### current repo で意味論上あるもの

- `Reject`
- `Approximate`
- `Compensate`

### current parser-free PoC で machine-check しているもの

- `success`
- `explicit_failure`
- `Reject`
- static-only fixture の `not_evaluated`

`Approximate` / `Compensate` は wider failure space に残っているが、current parser-free PoC にはまだ織り込まれていない。

## `Reject` の位置づけ

- `Reject` は request-level outcome である。
- via-chain では、well-formed chain が success を返さずに尽きたときの最終 outcome として使う。
- option-local `admit` miss や `lease-expired` そのものを `Reject` と同一視しない。
- hidden acceptance も hidden repair も許さない。

## `try` / rollback

- rollback は local であり current `place` に閉じる。
- `try` body failure を fallback body に変換する。
- rollback は local state を巻き戻すが、degradation order を巻き戻さない。
- rollback 不可能な effect は rollback region から隔離するか、明示 compensation を要する。

## `atomic_cut`

- `atomic_cut` は place-local finalizing boundary である。
- current `place` の rollback frontier だけを確定する。
- process 全体の同期点でも durable commit でもない。
- current parser-free PoC では、active rollback frame がある場合に限り、その restore snapshot 更新として表す。

## fallback / preference chain の位置づけ

current L2 では次を採る。

- fallback は outer wrapper ではなく guarded option chain
- canonical chain は left-to-right の優先順
- same-lineage chain は monotone degradation
- earlier option への再昇格禁止
- write-after-expiry は later write-capable option を試し、尽きれば `Reject`

詳細は `plan/05-fallback-lease-and-chain-semantics.md` に切り出す。

## canonical normalization law

current L2 では、同じ logical access path / semantic lineage 上の nested fallback は canonical chain `A > B > C ...` へ flatten して読む。

重要なのは次である。

- nested outer / inner shape 自体は意味として保持しない
- canonical chain が表すのは優先順と guard / contract / capability
- 正規化できるのは same-lineage / monotone degradation が成り立つ branch だけ

## incompatible branch rejection phase

同じ canonical chain に載せられない branch は static rejection 寄りに扱う。

代表例:

- target 不一致
- documented lineage annotation の不一致
- monotone capability を壊す branch
- declared contract surface だけで successor 不適格と分かる branch

runtime `Reject` と static rejection を混同しない。

## static evidence floor

same-lineage として static に扱う最小 declared information は次である。

- `declared access target`
- edge-local な `documented lineage annotation`

どちらか一方だけでは足りない。
`target` だけでも lineage 証拠にならず、annotation だけでも anchor が足りない。

## underdeclared handling

次は current L2 では surface-level static error として止める。

- edge-local lineage annotation 欠如
- declared access target 欠如
- contract / capability surface 欠如で successor 判定ができない場合

underdeclared branch を hidden acceptance で evaluation へ流さない。

## lineage annotation

- edge-local が最小
- predecessor と successor の pair を明示する
- global lineage identifier、chain-level blanket annotation、option-local tag は current L2 の必須にしない
- explanation では `lineage(A -> B)` を使うが、final token ではない

## `admit` / non-admissible metadata

### `admit`

- option-local `admit` は admission-side metadata である
- `perform` の request-local `require` / `ensure` と混ぜない
- `admit` miss は option-local non-admissible skip

### non-admissible metadata

current L2 では、少なくとも次を formal carrier とする。

- `option_ref`
- `subreason`
  - `admit-miss`
  - `lease-expired`

dedicated skip event はまだ要求しない。

## capability mismatch の扱い

- capability mismatch は formal subreason に上げない
- request-local `require` と declared capability surface から読む narrative explanation に留める
- これにより event surface と metadata shape を最小のまま保つ

## current L2 settled / current PoC reading / OPEN

### current L2 settled

- guarded option chain
- canonical left-to-right degradation
- no re-promotion
- static rejection / underdeclared / runtime `Reject` の区別
- `admit-miss` / `lease-expired` の formal metadata
- capability mismatch narrative

### current parser-free PoC reading

- `OptionDecl` / `ChainDecl` は runtime no-op
- failure 伝播は meta-level control result
- `atomic_cut` frontier は rollback frame update として表す

### OPEN

- `Approximate` / `Compensate` を PoC へいつ入れるか
- detached trace / audit serialization
- direct target の request-level `Reject` を oracle carrier に入れる必要
- richer host interface
