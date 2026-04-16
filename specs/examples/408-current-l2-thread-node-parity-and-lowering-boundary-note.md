# 408 — current L2 thread / node parity and lowering boundary note

## 目的

thread と node を programming surface でどう読むかを、
雑な同一視に落とさず docs-first に整理する。

## source-backed floor

- `place` は server / process / executable ではなく semantic locus である。
- routing / transport / distributed realization は Mir-0 の外にある。
- shared-space / Mirrorea / operational profile は current docs-first boundary に留まる。

## current working line

**thread も node も place であり、差は lowering / evidence / failure / transport policy に出る**

という wording を working hypothesis として採る。

### parity side

- source-level causal language
- publication / observation / witness の構造
- local finalization と later handoff family の比較

### asymmetry side

- placement
- transport
- durability / replay
- witness burden
- failure policy
- fairness / authority

## current judgment

- `thread == node` と short-hand で書くべきではない。
- source-level parity と lower-level asymmetry を分けるのは有用である。
- ただしこの line はまだ **current working line** であり、final lowering contract ではない。

## practical contrast

### same-process analog

- one process
- two threads / two places
- publication edge and observation edge

### distributed room analog

- two nodes / two places
- transport and route boundary
- witness / replay / failure policy attachment

両者で causal language は比較できるが、
transport / evidence / durability policy は同じではない。
