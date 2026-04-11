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
- Phase 4 は `specs/examples/295...296` により self-driven closeout fixed である。current package は `specs/examples/121...125` を維持し、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required に、stronger control-plane split と distributed fairness は later に残している。
- Phase 5 は `specs/examples/297...298` により self-driven closeout fixed である。verifier handoff surface docs-only mixed-row bridge、theorem retained bridge stop line、proof / protocol / runtime-policy inventory、retained-later line を 1 本の closeout bundle にまとめ、actual artifact / tool binding / low-level memory-order family は later に残している。
- Phase 6 front-half の compile-ready minimal PoC では、`mir-ast` stage 1 / stage 2 carrier は actualize 済みである。残る主線は `mir-semantics` / `mir-runtime` compile path と formal hook である。

## いまから数 task の主眼

近い数 task の目的は、fixed 済みの Phase 1 / 2 / 3 / 4 / 5 entry criteria と parser first tranche を前提に、Phase 6 front-half actualization を 2〜3 package で compile-ready checkpoint まで進めることである。

## 次に自走で進める順番

### 1. Phase 6 front-half actual checker / runtime skeleton first tranche

- `mir-semantics` / `mir-runtime` minimal compile path
- rough weight: 重い
- rough 所要: 1〜2 task / 4〜7日

### 2. compile-ready verification and formal hook

- compile / smoke gate と formal hook first tranche
- rough weight: 重い
- rough 所要: 1〜2 task / 3〜6日

### 3. Phase 6 compile-ready checkpoint drift suppression / mirror sweep

- checkpoint wording と mirror audit
- rough weight: 中
- rough 所要: 1 task / 1〜2日

## rough step estimate

| 目標 | rough step estimate | 注記 |
|---|---|---|
| Phase 6 front-half actual checker / runtime skeleton first tranche | 1〜2 task | parsed subset から checker floor / runtime skeleton まで compile path を通す |
| compile-ready verification and formal hook | 1〜2 task | cargo gate と tool-neutral or narrow tool first cut を揃える |
| Phase 6 compile-ready checkpoint drift suppression / mirror sweep | 1 task | specs / plan / snapshot / abstract の stale wording を掃除する |

## いま見えている later blocker / open question

### 1. actual parser subset の second-tranche widen boundary

- stage 1 / 2 carrier を compile-ready checkpoint minimum として固定するか
- selected stage 3 / perform head を second tranche にどう reopen するか
- current recommendation は **stage 1 / 2 carrier を entry criteria として維持** である

### 2. theorem / model-check tool binding

- tool-neutral export で一旦 close するか
- theorem side / model-check side の concrete tool first cut をどこで選ぶか
- current recommendation は **Task 1〜2 の後で narrow に選ぶ** である

## 今の working assumption

- current L2 semantics と Phase 1 / 2 closeout bridge は大きく動かさない
- parser-free PoC は current baseline として維持する
- Phase 3 reconnect freeze は fixed 済みの entry criteria として扱う
- Phase 4 self-driven closeout は fixed 済みとし、shared-space final catalog は user-spec-required item に留める
- actual parser / checker / runtime first tranche は non-production minimal cut に留める
- proof / model-check line は Phase 5 closeout fixed と parser first tranche を entry criteria に、Task 1〜2 の後で narrow に actualize する
