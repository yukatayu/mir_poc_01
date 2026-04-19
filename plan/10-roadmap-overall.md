# plan/10 — 全体ロードマップ

## 目的

この文書は、repo 全体の研究を

- どこまで executable mainline として進めるか
- どこから先を theory-lab / reserve integration line に分けるか
- 何を heavy future workstream に残すか

の観点で整理する。

## current overall reading

現在の repo は、次の 3 つを同時に持つ。

1. semantics / invariants / boundary の docs-first repo
2. parser-free current L2 の runnable validation substrate
3. compile-ready minimal actualization と fixed-subset source sample の narrow runnable path

したがって current reading は、
**architecture-first だが、fixed subset は理論と実行を ratchet で並走させている**
である。

## current lane split

### Lane A. execution

- 主眼:
  fixed-subset source sample と malformed family の narrow widening
- current line:
  `Macro 4 active on fixed authored/prototype floor`
  （current-l2 authored sixteen、corrected prototype set `p01...p12`、runner / CLI / regression floor は fixed 済みであり、sample corpus 自体は adequacy corpus として active に保つ）
- goal:
  runnable corpus を広げつつ、runner / ladder / regression / docs mirror を崩さない

### Lane B. theory-lab

- 主眼:
  typed / theorem / model-check / order / memory / syntax / modality を
  docs-first comparison と boundary planning で進める
- current line:
  `Macro 5 final-layer closeout packages active`
  （boundary / pilot / framing floor は fixed 済みであり、`specs/examples/458...465` compare floor、`466...469` actual-adoption floor、`470...474` helper-local actualization / narrowing floor、`475...519` deeper-theory / reserve / mixed-gate / actual-execution actualization floor は close 済みである。corrected runnable floor と representative Lean sample set actual Lean execution は reached 済みであり、`specs/examples/520`、`521`、`522`、`523`、`524`、`525` により next active line は helper / CLI hardening and broader coverage と later mixed/user-spec residual と読み、Lean formal skeleton / proof obligations first slice、IFC secret valid/invalid concrete example、source-side authority pair、source-side label-flow negative、delegated RNG provider placement carry-over は committed `samples/lean/` corpus と source-side prototype corpus に actualize 済みと読む）
- goal:
  full implementation へ飛ばず、candidate family、adequacy corpus、verifier-boundary、stop line を固定した上で、post-runnable closeout package と mixed-gate package を narrow に閉じる

### Lane C. reserve integration

- 主眼:
  thin facade / public shell / shared-space / host-I/O boundary を hidden promotion なしで整理する
- current line:
  `Macro 6 minimal working subset actual default / Macro 7 mixed + reserve reopen package active`
- goal:
  support-only / shell / excluded bucket を混ぜずに保ちつつ、remaining topics を mixed gate に留める

## macro roadmap

この `Macro 0〜8` は repo 全体の top-level roadmap axis として置いている。
現時点では `Macro 9` 以降を追加する想定ではなく、今後の課題は該当する macro へ戻して配置する。
そのため `Macro 8` は「それ以降全部」の bucket ではなく、application / domain realization 専用の終端側 phase である。

| Macro phase | 主眼 | 現在位置 | 次の exit signal |
|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | snapshot と detail-side plan の drift suppression |
| `Macro 1` | semantic kernel / invariant stabilization | late | current L2 semantic reopen が narrow になる |
| `Macro 2` | parser-free validation substrate | late | detached loop / fixture corpus / compare helper の drift suppression |
| `Macro 3` | compile-ready minimal actualization | late | support-only / public-candidate split の安定化 |
| `Macro 4` | executable fixed-subset sample expansion | active on fixed authored/prototype floor | sample widening / prototype comparison を current semantics を崩さずに保てる |
| `Macro 5` | typed / theorem / model-check bridge | near-end closeout + pre-floor hardening | current first line / retained alternatives / stop line / later gate が source-backed に揃い、preview-alignment pre-floor、model-check projection pre-floor、theorem discharge pre-floor も helper-local compare として固定される |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary only | shared-space docs-first boundary と mixed-gate final profile の分離が保てる |
| `Macro 7` | toolchain / backend / host-facing integration | mixed | shell / facade / packaging boundary を final target profileから分離できる |
| `Macro 8` | domain / application realization | application-specific target not started | first target profile が user と合意される |

## current recommendation

- Lane A は narrow actualization を維持する。
  - corrected runnable prototype と helper-local `debug_outputs` / `verification_preview` を使って usability / falsifier comparison を進めてよい。
- Lane B は mainline の「ついで」ではなく、separable な research program として前進させる。
- Lane B の narrow mixed-gate pre-floor では、preview summary、model-check projection helper、theorem discharge helper を final public contract や settled property language にせず compare floor を厚くしてよい。
- Lane C は thin facade と shell concern の boundary hardening、および mixed-gate boundary の明確化に留める。
- old `Phase 7 = FutureWork` の巨大 bucket は再導入しない。

## まだ急がないもの

- final parser grammar
- final public parser / checker / runtime API
- concrete theorem prover / model-check tool binding
- LLVM-family backend / external codegen
- shared-space final catalog
- raw FFI / game engine direct binding actualization
- upper-layer application finalization

## autonomous research rhythm

current repo では、次の ratchet を基本にする。

1. docs-first comparison / threshold を切る
2. narrow pilot または helper-local actual evidence を足す
3. regression / smoke / detached loop を回す
4. `plan/` / `progress.md` / `tasks.md` / relevant docs を同期する

この rhythm を壊す broad actualization は避ける。
