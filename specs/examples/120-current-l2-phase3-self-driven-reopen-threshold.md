# 120 — current L2 Phase 3 self-driven reopen threshold

## 目的

この文書は、`specs/examples/119-current-l2-reconnect-freeze-threshold.md` と
Phase 3 closeout checkpoint を前提に、
**current checkpoint の後に、Phase 3 の self-driven 部分をまだ reopen すべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも public checker API でもない。
固定するのは、

- current Phase 3 private staged spike に、なお self-driven で積める前進があるか
- あるとしても、それが docs-only refinement に留まるか
- それとも current checkpoint では self-driven portion が一旦尽きたと読む方が自然か

という threshold judgment だけである。

## 前提

- current L2 の core semantics、parser-free PoC、detached validation loop は変更しない。
- stage 1 / stage 2 / stage 3 の private staged spike は source-backed actual evidence を持つ。
- reconnect subline は freeze 済みであり、`e19` は typed static reason family、`E21` / `E22` は runtime / proof boundary に残す。
- final parser grammar、public checker API、runtime/proof bridge の finalization はまだ Phase 3 の外に置く。

## 比較する 4 案

### 案 1. parser subset inventory / staged parser spike を docs-only でさらに磨く

`specs/examples/29-current-l2-first-parser-subset-inventory.md` と
`specs/examples/73-current-l2-first-parser-spike-staging.md` の wording や inventory を
さらに細かく整理する。

#### 利点

- final grammar を固定せず、parser boundary の認識を少しだけ明瞭にできる。
- docs-only なので helper contract を広げない。

#### 欠点

- current inventory と staging はすでに parser-led / checker-led sequencing の判断に十分使える。
- 新しい code anchor や new evidence を増やさない限り、前進量が薄い。

### 案 2. public checker API / generic family entry の前提を docs-only で比較する

family-specific helper を generic checker family や public checker API に寄せる前提条件だけを
docs-only で比較する。

#### 利点

- Phase 5 / 6 へ送る boundary inventory を少し明瞭にできる。
- 今後の public surface 比較の足場になる。

#### 欠点

- 実質的には Phase 5 / 6 の入口整理であり、Phase 3 current tranche の自然な続きではない。
- private helper actualization をこれ以上増やさない current line と噛み合いにくい。

### 案 3. `E21` / `E22` の runtime contrast を parser-side reconnect へ mirror する条件を再比較する

`specs/examples/119-current-l2-reconnect-freeze-threshold.md` の freeze line を reopen し、
stage 2 reconnect line を `E21` / `E22` contrast まで widening する可能性を再比較する。

#### 利点

- reconnect subline をさらに厚く見せられる。
- runtime representative との対応を一見強められる。

#### 欠点

- `E21` / `E22` の差の本体は nested `place`、`place_anchor == current_place`、
  restore scope にあり、current parser-side reconnect line より runtime / proof boundary に近い。
- current dedicated carrier は malformed pair と `no_findings` smoke に留まっており、
  narrow reconnect helper の責務を超えやすい。

### 案 4. current checkpoint では Phase 3 self-driven portion は一旦尽きたとみなす

- current Phase 3 private staged spike は closeout 済み
- reopen は later pressure が出たときだけ行う
- next self-driven 主線は Phase 2 maintenance tail / Phase 4 side line / Phase 5 inventory へ移す

#### 利点

- Phase 3 の責務を「private staged spike と freeze threshold の整理」までに留められる。
- final parser grammar や public checker API を勝手に既成事実化しない。
- 次の主線を detached loop friction reduction、shared-space boundary、small decidable core inventory に移しやすい。

#### 欠点

- Phase 3 を今すぐさらに広げたい場合は、別 pressure を積み直す必要がある。

## 比較

### 新しい evidence を増やせるか

- 案 1: ほぼ wording refinement に留まる。
- 案 2: public API pressure を先取りしやすく、current evidence line は増えにくい。
- 案 3: runtime/proof boundary に踏み込みやすい。
- 案 4: new evidence は増えないが、current evidence line を clean に閉じられる。

### current freeze line を壊さないか

- 案 1: 壊さない。
- 案 2: public surface 側へ滑りやすい。
- 案 3: reconnect freeze line を壊しやすい。
- 案 4: 最も壊さない。

### Phase 5 / 6 との境界が明瞭か

- 案 1: 境界は維持できるが、前進量が薄い。
- 案 2: 境界をまたぎやすい。
- 案 3: runtime / proof 側に寄りすぎる。
- 案 4: 境界を最も明瞭に保てる。

## current judgment

current checkpoint の読みとして最も自然なのは、
**案 4. current checkpoint では Phase 3 self-driven portion は一旦尽きたとみなす**
ことである。

理由は次の通りである。

1. stage 1 / stage 2 / stage 3 の private staged spike は、current L2 の parser boundary / first checker cut を考えるうえで十分な representative evidence を already 持つ。
2. これ以上 self-driven に広げる候補は、
   - wording refinement に留まるか
   - Phase 5 / 6 の public boundary comparison を先取りするか
   - runtime / proof boundary へ寄りすぎるか
   のいずれかになりやすい。
3. したがって、current checkpoint では Phase 3 を「reopen 候補を将来比較できる reserve path」として残しつつ、
   self-driven 主線は別 phase へ移す方が理論的に整っている。

## current meaning

- current Phase 3 self-driven portion は **一旦尽きた** と読んでよい。
- Phase 3 は未完了ではなく、**current checkpoint では reserve path** として残る。
- reopen する場合は、
  - Phase 5 inventory
  - actual parser/public checker pressure
  - runtime/proof mirror need
  のいずれかが明示的に増えたときだけでよい。

## next narrow step

current repo の next self-driven step として自然なのは、次のいずれかである。

1. detached validation loop の friction reduction
2. shared-space / membership side line
3. static analysis / type / theorem prover boundary の small decidable core inventory

Phase 3 自体を次に reopen する必要は、current checkpoint ではない。
