# plan/17 — research phases and autonomy gates

## 目的

この文書は、repo 全体の長期研究を **phase** 単位で見直すための補助線である。
`plan/10` の全体ロードマップを置き換えるものではなく、

- いまどの phase にいるか
- 各 phase がどれくらい重いか
- どこまで self-driven に進めてよいか
- どこで user 仕様確認が要るか

を、作業順序と autonomy gate の観点で見やすく整理する。

## この文書の読み方

- phase は厳密なウォーターフォールではない。
- 複数 phase が並走することはある。
- ただし、**current promoted line** と **checkpoint maintenance / reserve path** は区別する。
- `self-driven` は、agent が narrow comparison / PoC / docs / regression を進めてよい範囲を指す。
- `user spec required` は、勝手に finalization すると手戻りが大きい範囲を指す。

## phase 一覧

### Phase 0 — repository memory / decision boundary

- 主眼: `specs/` / `plan/` / report / snapshot の役割分離、traceability、maintenance rule
- 現在地: maintenance
- 重さ: 低い
- autonomy gate: **self-driven**

### Phase 1 — current L2 semantics stabilization

- 主眼: fallback / `lease` / guarded option chain / `TryFallback` / `AtomicCut` の安定化
- 現在地: self-driven closeout fixed
- 重さ: 中
- autonomy gate: **self-driven**

### Phase 2 — parser-free PoC and detached validation loop

- 主眼: fixture / interpreter / host harness / bundle / batch / selection / detached loop
- 現在地: self-driven closeout fixed
- 重さ: 中
- autonomy gate: **self-driven**

### Phase 3 — parser boundary and first checker cut

- 主眼: staged parser spike、helper-local compare family、first checker reconnect
- 現在地: self-driven freeze fixed / later widen reserved
- 重さ: 中〜やや重い
- autonomy gate:
  - minimal parser subset / reconnect freeze は **self-driven**
  - stage 3 request reconnect / public parser-checker boundary widen は **後段依存**

### Phase 4 — shared-space / membership / practical example boundary

- 主眼: participant carrier、activation rule、authority placement、consistency mode、RNG / fairness、reconnect / leave / rejoin / causal metadata
- 現在地: self-driven closeout fixed
- 重さ: 重い
- autonomy gate:
  - docs-first boundary と practical example は **self-driven**
  - final activation / authority / auth / identity / admission / consistency / fairness catalog は **user spec required**

### Phase 5 — static analysis / type / theorem prover / async-control boundary

- 主眼: small decidable core、proof boundary、protocol verifier boundary、runtime policy boundary
- 現在地: verifier handoff gate は fixed 済みであり、proof-model-check handoff closeout は still 要る
- 重さ: とても重い
- autonomy gate:
  - docs-first inventory と theorem-line threshold comparison は **self-driven**
  - actual external contract finalization は **後段依存**

### Phase 6 — actual parser / checker / runtime commitment

- 主眼: actual parser subset、public checker boundary、runtime / host / artifact API
- 現在地: entry criteria visible / actual public crate path は未着手
- 重さ: 重い
- autonomy gate: **後段依存**

### Phase 7 — higher-layer integration and domain realization

- 主眼: Mirrorea Fabric、Typed-Effect Wiring Platform、PrismCascade、shared-space upper layer、application design
- 現在地: 未着手
- 重さ: とても重い
- autonomy gate: **user spec required**

## 現在の主線

いま repo の主線は、次の 3 本である。

1. **Phase 5 closeout sweep**
   - self-driven / current-recommendation scope を phase-complete snapshot へ揃える
2. **Phase 6 front-half actualization**
   - `mir-ast` / `mir-semantics` / `mir-runtime` をまたぐ compile-ready minimal PoC の first tranche

Phase 3 は長く reserve path だったが、Phase 6 front-half へ入るための self-driven freeze は `specs/examples/287...290` で fixed 済みである。Phase 1 closeout も `specs/examples/291...292` で fixed 済みであり、semantic core / invariant bridge / notation boundary は current entry criteria と読んでよい。Phase 2 closeout も `specs/examples/293...294` で fixed 済みであり、parser-free baseline の compile/test/smoke gate と detached loop policy は current entry criteria と読んでよい。Phase 4 closeout も `specs/examples/295...296` で fixed 済みであり、`specs/examples/121...125` current package と user-spec-required final catalog の境界は current entry criteria と読んでよい。

## immediate execution order

1. **Phase 5 proof / protocol / runtime-policy handoff closeout** を immediate line として扱う
2. **Phase 6 front-half actual parser / checker / runtime first tranche** を actualize する
3. 同じ task 群の中で checkpoint / mirror maintenance を継続する

## いま止めるべき線

次は self-driven に比較を続けてよい。

- Phase 5 closeout sweep
- checkpoint maintenance
- compile-ready first tranche に必要な crate-local surface inventory

次は勝手に finalization しない。

- final parser grammar
- actual external proof / protocol verifier contract
- shared-space の final activation / authority / auth / identity / admission / consistency / fairness catalog
- higher-layer application contract

## 現在の判断

- **current immediate line は Phase 5 proof / protocol / runtime-policy handoff closeout** と読むのが自然である。
- **Phase 1 / 2 / 3 / 4 は Phase 6 front-half のための self-driven entry criteria を fixed 済み、Phase 5 は self-driven closeout 前** と読むのが自然である。
- `progress.md` と `tasks.md` は、この phase 読みを rough snapshot として mirror する。
