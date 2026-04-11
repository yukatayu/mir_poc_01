# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

current immediate execution order は `plan/17-research-phases-and-autonomy-gates.md` と `progress.md` の phase section を優先する。
この文書は snapshot として、**Phase 3 reconnect freeze fixed 後の Phase 1〜5 closeout と Phase 6 front-half compile-ready gate** を短く保つ。

## current reading

- Phase 1 は `specs/examples/291...292` により self-driven closeout fixed である。semantic core、invariant bridge、notation boundary は narrow に揃い、final grammar / type / schema は later に残している。
- Phase 2 は `specs/examples/293...294` により self-driven closeout fixed である。parser-free companion baseline の compile/test/smoke gate、helper boundary、detached loop compare-only policy は fixed し、reference update / bless、final retention/path policy、public exporter API は later に残している。
- Phase 3 は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。stage 1 / 2 structural floor と first checker reconnect bridge は entry criteria として整理され、stage 3 request/admit/predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残る。
- Phase 4 は `specs/examples/121...125` までで current package close である。self-driven current recommendation closeout と user-spec-required final catalog の切り分けをもう一段明示したい。
- Phase 5 は `specs/examples/126...286` までで current package close であり、verifier handoff gate は fixed 済みである。
- Phase 6 front-half の compile-ready minimal PoC は、docs / parser-free harness / test-only parser spike まではあるが、`mir-ast` / `mir-runtime` public crate の actual compile path はまだ薄い。

## いまから数 task の主眼

近い数 task の目的は、fixed 済みの Phase 1 / 2 / 3 entry criteria を前提に、Phase 4 / 5 の self-driven scope を phase-complete snapshot まで揃え、その後に Phase 6 front-half actualization へ入ることである。

## 次に自走で進める順番

### 1. Phase 4 / 5 closeout sweep

- shared-space self-driven closeout と proof / protocol / runtime-policy handoff closeout を順に片付ける
- rough weight: 中〜重
- rough 所要: 3〜6 task / 1〜3週

### 2. Phase 6 front-half actual parser / checker / runtime first tranche

- `mir-ast` minimal parser carrier
- `mir-semantics` / `mir-runtime` minimal compile path
- compile-ready local gate
- rough weight: 重い
- rough 所要: 4〜7 task / 1〜3週

## rough step estimate

| 目標 | rough step estimate | 注記 |
|---|---|---|
| Phase 4 / 5 closeout sweep | 3〜6 task | phase-complete snapshot を作る |
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

## 今の working assumption

- current L2 semantics と Phase 1 / 2 closeout bridge は大きく動かさない
- parser-free PoC は current baseline として維持する
- Phase 3 reconnect freeze は fixed 済みの entry criteria として扱う
- actual parser / checker / runtime first tranche は non-production minimal cut に留める
- shared-space final catalog は user-spec-required item として immediate gate から外す
- proof / model-check line は boundary freeze 後に narrow に actualize する
