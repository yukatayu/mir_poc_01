# 451 — current L2 runnable prototype と not-implemented sample bucket

## 目的

過去の rough stimulus と、current L2 fixed subset で実際に parse / run できる corrected sample を、
同じ `samples/` 配下で混同しないための bucket policy を与える。

## current judgment

1. `samples/current-l2/`
   - source-backed authored fixed subset の corpus である。
   - current runner / ladder / regression inventory の正本として扱う。
   - current authored corpus sixteen 以外を hidden に数えない。
2. `samples/prototype/`
   - **feature family 自体は comparison / mixed gate に残るが、current lowerer / runner で corrected prototype を回したい** sample を置く。
   - current L2 lowerer が受ける subset の内側へ寄せる。
   - current operational shell では explicit path で受け、sample path の隣に adjacent `.host-plan.json` sidecar があれば `--host-plan` を省略してよい。
   - current authored `samples/current-l2/` inventory には含めない。
3. `samples/not_implemented/`
   - exact rough token / macro / syntax を preservation 目的で残す bucket である。
   - current parser / runner の対象にはしない。
   - current corrected runnable analogue があるなら `samples/prototype/` または `samples/current-l2/` から辿る。

## source comment policy

- sample 冒頭には、日本語で意図を短く説明する comment を置いてよい。
- current source lowerer の runnable floor では、leading contiguous `#` line block だけを blank/comment line として無視してよい。
- これは final grammar adoption ではなく、current sample authoring convenience の narrow cut である。

## first runnable prototype tranche

current first tranche は次である。

| sample | bucket | reading |
|---|---|---|
| `p01-dice-publication-handoff` | `samples/prototype/current-l2-order-handoff/` | rough stimulus A/B を current L2 `perform + atomic_cut + perform` へ落とし直した corrected prototype |
| `p02-dice-publication-fallback` | `samples/prototype/current-l2-order-handoff/` | publication failure と fallback recovery を current L2 `try { ... } fallback { ... }` で比較する corrected prototype |
| `p03-avatar-controller-attach-detach` | `samples/prototype/current-l2-dynamic-attach-detach/` | attach/detach 直観を current L2 `perform + atomic_cut + perform` で比較する corrected prototype |
| rough stimuli A–D | `samples/not_implemented/order-handoff/` | exact rough surface の preservation。current parser / runner に入れない |

## current non-goals

- rough stimuli A–D を adopted syntax と読むこと
- prototype sample を current authored sixteen に混ぜること
- `samples/not_implemented/` を parser widening queue と同一視すること
- final grammar / final public sample CLI / final packaging success criteria をここで固定すること
