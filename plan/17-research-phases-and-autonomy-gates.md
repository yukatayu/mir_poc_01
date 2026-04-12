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
- 現在地: self-driven closeout fixed
- 重さ: とても重い
- autonomy gate:
  - docs-first inventory と theorem-line threshold comparison は **self-driven**
  - actual external contract finalization は **後段依存**

### Phase 6 — actual parser / checker / runtime commitment

- 主眼: actual parser subset、public checker boundary、runtime / host / artifact API
- 現在地: parser / checker-runtime / tool-neutral formal-hook / proof-notebook review-unit pilot fixed
- 重さ: 重い
- autonomy gate:
  - non-production minimal first tranche は **self-driven**
  - final parser / public checker / runtime host finalization は **後段依存**

### Phase 7 — higher-layer integration and domain realization

- 主眼: Mirrorea Fabric、Typed-Effect Wiring Platform、PrismCascade、shared-space upper layer、application design
- 現在地: 未着手
- 重さ: とても重い
- autonomy gate: **user spec required**

## 現在の主線

いま repo の主線は、次の 3 本である。

1. **Phase 6 proof-notebook bridge-sketch reopen ordering**
   - review-unit current cut の後で theorem-side bridge sketch をいつ reopen するかを later task として整理する
2. **Phase 0 / 6 drift suppression**
   - current line / next line / retained-later line の mirror drift を follow-up maintenance として抑える
3. **Phase 6 first widened authored row actualization**
   - current widen order `e1 -> e21 -> e3` の first slot `e1` を actual source row / runner / regression に反映する

Phase 3 は長く reserve path だったが、Phase 6 front-half へ入るための self-driven freeze は `specs/examples/287...290` で fixed 済みである。Phase 1 closeout も `specs/examples/291...292` で fixed 済みであり、semantic core / invariant bridge / notation boundary は current entry criteria と読んでよい。Phase 2 closeout も `specs/examples/293...294` で fixed 済みであり、parser-free baseline の compile/test/smoke gate と detached loop policy は current entry criteria と読んでよい。Phase 4 closeout も `specs/examples/295...296` で fixed 済みであり、`specs/examples/121...125` current package と user-spec-required final catalog の境界は current entry criteria と読んでよい。Phase 5 closeout も `specs/examples/297...298` で fixed 済みであり、verifier handoff surface、theorem retained bridge stop line、boundary inventory、retained-later line は current entry criteria と読んでよい。

## immediate execution order

1. **Phase 6 proof-notebook bridge-sketch reopen ordering** を immediate line として扱う
2. drift suppression は follow-up maintenance として継続する
3. その後に first widened authored row `e1` actualization を扱う

## いま止めるべき線

次は self-driven に比較を続けてよい。

- Phase 6 proof-notebook bridge-sketch reopen ordering
- Phase 6 first widened authored row actualization
- Phase 0 / 6 drift suppression

次は勝手に finalization しない。

- final parser grammar
- actual external proof / protocol verifier contract
- shared-space の final activation / authority / auth / identity / admission / consistency / fairness catalog
- higher-layer application contract

## 現在の判断

- **current authored-row widen sequencing は `e1 -> e21 -> e3` に fixed 済み** と読むのが自然である。
- **current immediate line は Phase 6 proof-notebook bridge-sketch reopen ordering** と読むのが自然である。
- **front-half compile-ready checkpoint close と syntax-backed sample verification readiness は別の progress axis** と読むのが自然である。
- **Phase 1 / 2 / 3 / 4 / 5 は Phase 6 front-half のための self-driven entry criteria を fixed 済み** と読むのが自然である。
- **`mir-ast` stage 1 / stage 2 carrier は Phase 6 parser first tranche として fixed 済み** と読むのが自然である。
- **`mir-semantics` program-level entry と `mir-runtime` current L2 thin skeleton は Phase 6 checker/runtime first tranche として fixed 済み** と読むのが自然である。
- **tool-neutral formal hook first tranche は Phase 6 compile-ready gate として fixed 済み** と読むのが自然である。
- **tool-neutral formal hook 後段の first theorem-side consumer は proof-notebook review unit まで fixed 済み** と読むのが自然である。
- **LLVM-family backend や higher-level async-control / low-level memory-order-like surface は source corpus / lowering / runner / ladder より前に mainline へ入れない** のが自然である。
- `progress.md` と `tasks.md` は、この phase 読みを rough snapshot として mirror する。
