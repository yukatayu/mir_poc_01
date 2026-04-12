# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

current immediate execution order は `plan/17-research-phases-and-autonomy-gates.md` と `progress.md` の phase section を優先する。
この文書は snapshot として、**Phase 6 front-half compile-ready gate fixed 後に、syntax-backed fixed-subset sample verification までどう進むか** を短く保つ。

## current reading

- Phase 1 は `specs/examples/291...292` により self-driven closeout fixed である。semantic core、invariant bridge、notation boundary は narrow に揃い、final grammar / type / schema は later に残している。
- Phase 2 は `specs/examples/293...294` により self-driven closeout fixed である。parser-free companion baseline の compile/test/smoke gate、helper boundary、detached loop compare-only policy は fixed し、reference update / bless、final retention/path policy、public exporter API は later に残している。
- Phase 3 は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。stage 1 / 2 structural floor と first checker reconnect bridge は entry criteria として整理され、stage 3 request/admit/predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残る。
- Phase 4 は `specs/examples/295...296` により self-driven closeout fixed である。current package は `specs/examples/121...125` を維持し、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required に、stronger control-plane split と distributed fairness は later に残している。
- Phase 5 は `specs/examples/297...298` により self-driven closeout fixed である。verifier handoff surface docs-only mixed-row bridge、theorem retained bridge stop line、proof / protocol / runtime-policy inventory、retained-later line を 1 本の closeout bundle にまとめ、actual artifact / tool binding / low-level memory-order family は later に残している。
- Phase 6 front-half compile-ready minimal PoC では、`mir-ast` stage 1 / stage 2 carrier、`mir-semantics` / `mir-runtime` checker/runtime first tranche、tool-neutral formal hook first tranche、checkpoint sweepは actualize / close 済みである。`specs/examples/305...306` により next reopen sequencing も fixed 済みであり、`specs/examples/307...308` により parser second tranche first package も actualize 済みである。`specs/examples/309...310` により reserve formal tool binding inventory も fixed 済みであり、`specs/examples/311...312` により parser-side follow-up package sequencing も fixed 済みであり、`specs/examples/313...314` により shared single attachment frame first package も actualize 済みであり、`specs/examples/315...316` により source-sample corpus scope / file layout も fixed 済みであり、`specs/examples/317...318` により representative / fixture / source mapping matrix も fixed 済みであり、`specs/examples/319...320` により lowering first cut、`specs/examples/321...322` により runner first cut、`specs/examples/323...324` により verification ladder wiring、`specs/examples/325...326` により source-sample authoring / bless / regression policy、`specs/examples/327...328` により theorem-first concrete tool pilot も fixed 済みである。
- ただし、source text sample を fixed subset で持ち、`static gate` / `interpreter` / `formal hook` を sample 単位で通す経路は policy と theorem-first first pilot まで fixed した一方、bridge sketch と wider authored row 側は still staged である。current authored quintet `e1` / `e2` / `e21` / `e4` / `e23` には reached-stage row と repo-local policy helper があり、`e3` guard comparison は fixed 済みで、remaining mainline は theorem-side bridge sketch reopen に置く。

## いまから数 task の主眼

近い数 task の目的は、fixed 済みの Phase 1 / 2 / 3 / 4 / 5 entry criteria と parser + checker/runtime + formal-hook checkpoint close を前提に、

1. theorem-side plain bridge sketch actualization を authored-row line の後に置く
2. compare-ready bridge sketch second reopen をその次段に置く
3. deferred `e3` actualization reopen timing をその後段に置く

ことである。

## 次に自走で進める順番

### 1. plain bridge sketch actualization

- review-unit current cut を維持したまま plain docs-only bridge sketch を narrow actual package に落とす
- rough weight: 中
- rough 所要: 1〜2 task / 数日

### 2. compare-ready bridge sketch second reopen

- plain bridge sketch actualization の後段として compare-ready bridge sketch threshold を narrow reopen する
- rough weight: 中
- rough 所要: 1〜2 task / 数日

### 3. deferred `e3` actualization reopen timing

- theorem-side bridge line の後で `e3-option-admit-chain` widening をどこへ戻すかを narrow に比較する
- rough weight: 中
- rough 所要: 1〜2 task / 数日

## rough step estimate

| 目標 | rough step estimate | 注記 |
|---|---|---|
| theorem-side bridge sketch reopen line | 1〜3 task | widen sequencing / bridge-sketch ordering / `e3` guard comparison は fixed 済みであり、remaining theorem-side next reopen timing を narrow に接続する |
| plain bridge-sketch reopen actualization | 1〜2 task | review-unit current cut を維持したまま plain docs-only bridge sketch を actual next theorem-side reopen に置く |

## いま見えている later blocker / open question

### 1. representative anchor unresolved row の扱い

- `e23` のように fixture-side `source_example_id` はあるが representative prose row が無い例をどう扱うか
- current recommendation は **unresolved representative anchor として row を残し、silent repair しない** である

### 2. concrete tool binding / backend timing

- theorem/model-check concrete tool や LLVM-family backend / external codegen をいつつなぐか
- current recommendation は **source corpus / lowering / runner / verification ladder / authoring policy の後** である

### 3. higher-level async-control / low-level memory-order family

- `atomic_cut` sample を増やすことと、高位制御 family や low-level memory-order-like surface を current executable core に入れることは分けて扱う
- current recommendation は **settled subset の sample 拡張を先に行い、async-control / memory-order family は heavy future workstream に残す** である

## 今の working assumption

- current L2 semantics と Phase 1 / 2 closeout bridge は大きく動かさない
- parser-free PoC は current baseline として維持する
- Phase 3 reconnect freeze は fixed 済みの entry criteria として扱う
- actual parser / checker / runtime first tranche は non-production minimal cut に留める
- reserve formal tool binding inventory、parser-side follow-up sequencing、shared single attachment frame actualization、source corpus scope / layout、theorem-first proof-notebook review-unit pilot は fixed 済み entry criteria として保持し、mapping / lowering / runner / verification ladder / source-sample policy を維持したまま widened authored-row order `e1 -> e21 -> e3` を前提に next line を進める
- LLVM-family backend や external codegen は current near-term mainline に置かない
