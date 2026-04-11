# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

current immediate execution order は `plan/17-research-phases-and-autonomy-gates.md` と `progress.md` の phase section を優先する。
この文書は snapshot として、**Phase 1〜5 closeout と Phase 6 front-half compile-ready gate** を短く保つ。

## current reading

- Phase 1 は closeout 前である。semantic core は安定しているが、invariants / notation / proof-obligation wording の最終 sweep はまだ残る。
- Phase 2 は closeout 前である。parser-free PoC、detached validation loop、fixture authoring baseline は成立済みで、残りは compile gate / retention policy / docs mirror closeout に寄る。
- Phase 3 は reopen 準備済みである。stage 1 / 2 / 3 parser spike の private evidence はあるが、`mir-ast` public crate は placeholder skeleton のままである。
- Phase 4 は `specs/examples/121...125` までで current package close である。self-driven current recommendation closeout と user-spec-required final catalog の切り分けをもう一段明示したい。
- Phase 5 は `specs/examples/126...286` までで current package close であり、verifier handoff gate は fixed 済みである。immediate next line は **`minimal-verifier-handoff-surface-ready minimal-parser-subset-freeze comparison`** である。
- Phase 6 front-half の compile-ready minimal PoC は、docs / parser-free harness / test-only parser spike まではあるが、`mir-ast` / `mir-runtime` public crate の actual compile path はまだ薄い。

## いまから数 task の主眼

近い数 task の目的は、Phase 5 の current package を Phase 6 front-half actualization に接続できるところまで narrow に閉じ、その前提として Phase 3 reserve path を reopen し、Phase 1 / 2 / 4 / 5 の self-driven scope を phase-complete snapshot まで揃えることである。

## 次に自走で進める順番

### 1. Phase 3 minimal parser subset freeze

- actual parser first tranche に上げる accepted cluster / reject cluster / retention floor を固定する
- `mir-ast/tests/support/current_l2_stage*` private helper と future public parser API の境界を明示する
- rough weight: 重め
- rough 所要: 1〜3 task / 2〜6日

### 2. Phase 3 -> 5 parser-to-checker reconnect freeze

- parser subset と first checker cut を reconnect し、Phase 6 checker line の minimal bridge を固定する
- rough weight: 中
- rough 所要: 1〜3 task / 2〜5日

### 3. Phase 1 / 2 / 4 / 5 closeout sweep

- Phase 1 semantics / invariants / notation closeout
- Phase 2 parser-free PoC / detached loop closeout
- Phase 4 shared-space self-driven closeout
- Phase 5 proof / protocol / runtime-policy handoff closeout
- rough weight: 中〜重
- rough 所要: 6〜12 task / 2〜4週

### 4. Phase 6 front-half actual parser / checker / runtime first tranche

- `mir-ast` minimal parser carrier
- `mir-semantics` / `mir-runtime` minimal compile path
- compile-ready local gate
- rough weight: 重い
- rough 所要: 4〜7 task / 1〜3週

## rough step estimate

| 目標 | rough step estimate | 注記 |
|---|---|---|
| minimal parser subset freeze | 2〜4 task | Phase 6 front-half parser gate |
| parser-to-checker reconnect freeze | 1〜3 task | checker line reconnect gate |
| Phase 1 / 2 / 4 / 5 closeout sweep | 6〜12 task | phase-complete snapshot を作る |
| Phase 6 front-half compile-ready minimal PoC | 4〜7 task | actual code path first tranche |

## いま見えている later blocker / open question

### 1. actual parser subset の public boundary

- stage 1 / 2 / 3 selected subset だけで十分か
- request head / wider clause suite / richer diagnostics まで広げるか
- current recommendation は **selected subset だけを先に actualize** である

### 2. theorem / model-check tool binding

- tool-neutral export で一旦 close するか
- theorem side / model-check side の concrete tool first cut をどこで選ぶか
- current recommendation は **boundary freeze の後で narrow に選ぶ** である

### 3. Phase 4 をどこまで完了扱いにするか

- self-driven current recommendation close で十分か
- final activation / authority / fairness catalog まで要るか
- current recommendation は **前者** である

## 今の working assumption

- current L2 semantics は大きく動かさない
- parser-free PoC は current baseline として維持する
- actual parser / checker / runtime first tranche は non-production minimal cut に留める
- shared-space final catalog は user-spec-required item として immediate gate から外す
- proof / model-check line は boundary freeze 後に narrow に actualize する
