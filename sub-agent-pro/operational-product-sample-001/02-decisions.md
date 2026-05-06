# 02 — decisions

## D-OPS-001: 新 root は `samples/product-alpha1/operational/`

理由:

- `samples/product-alpha1/demo/` を壊さない。
- `samples/practical-alpha1/` first-floor fixture と混同しない。
- 実運用プロセス再現 sample として明確に分ける。

## D-OPS-002: representative `.mir` と executable `package.mir.json` を両方置く

- `.mir` は将来 final textual grammar の representative source / design explanation。
- 現時点の executable alpha input は `package.mir.json`。
- product alpha CLI に direct `.mir` を渡して成功させない。

## D-OPS-003: import chain はまず package dependency として表す

`WorldCore <- MembershipChat <- SugorokuWorld` を source-ish import と package dependency の両方で示す。

- 現 CLI が package dependency を既に扱えるなら、それを使う。
- 足りなければ package dependency metadata を追加する。
- 実装できない dependency behavior は planned / manifest-only と明示する。

## D-OPS-004: server/client split は projection profile として示す

今回 direct server/client native binary generation はしない。

ただし、次を明記する。

- server target
- client/headless participant target
- packet boundary
- FFI boundary
- future backend / LLVM non-claim

## D-OPS-005: native output は native host launch bundle

現 product alpha の native output は:

```text
compiled Rust CLI + package + devtools + reports + manifest + run.sh
```

である。

これは direct Mir-to-machine-code ではない。

## D-OPS-006: portal は near-term future sample、spatial federation は later

- Portal / WorldLink は WWW hyperlink 相当であり、早期に skeleton を置く。
- continuous infinite shard federation は今回実装しない。
- first spatial future は hard authority boundary + gradient observation plan。

## D-OPS-007: vector clock を membership default にしない

- Membership freshness は epoch/incarnation。
- Hot-plug/shard config は config_epoch。
- Object replication は owner_epoch + sequence から始める。
- CRDT / dotted vector / dynamic clocks は optional future replication profile。

## D-OPS-008: devtools は optional polish ではなく completion condition

各 sample flow は、少なくとも JSON export で以下を見せる。

- import/dependency graph
- runtime Place graph
- message route graph
- event DAG
- hot-plug lifecycle
- save/load timeline
- contract/effect/failure/capability summary

## D-OPS-009: 実装できたものだけを runnable と書く

Portal / spatial / projection / LLVM は skeleton / future plan なら、そのように書く。
過大 claim は禁止。

## D-OPS-010: `P-OPS-01` は product alpha extension package であり final-public gate ではない

Final public grammar / ABI / WAN / distributed durable save-load / direct compiler backend は別 gate。
