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
  `Macro 4 / malformed duplicate-cluster source-authored static-stop pair actualization comparison with try-rollback malformed-static kept-later inventory`
- goal:
  runnable corpus を広げつつ、runner / ladder / regression / docs mirror を崩さない

### Lane B. theory-lab

- 主眼:
  typed / theorem / model-check / order / memory / syntax / modality を
  docs-first comparison と boundary planning で進める
- current line:
  `Macro 5 third-order follow-up active`
  （model-check small-cluster projection keep/drop と order/handoff source-surface wording reserve は fixed 済み、next reopen は modality internalization trigger note）
- goal:
  full implementation へ飛ばず、candidate family、adequacy corpus、verifier-boundary、stop line を固定する

### Lane C. reserve integration

- 主眼:
  thin facade / public shell / shared-space / host-I/O boundary を hidden promotion なしで整理する
- current line:
  `Macro 6/7 reserve integration checkpoint close`
- goal:
  support-only / shell / excluded bucket を混ぜずに保ちつつ、next reopen を later mixed gate に留める

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
| `Macro 4` | executable fixed-subset sample expansion | active | widened row を current ladder に安全に追加できる |
| `Macro 5` | typed / theorem / model-check bridge | active at boundary | first planning cut と stop line が package 単位で固定される |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary only | room-profile / confusion-replay / authority-family の compact note が固定される |
| `Macro 7` | toolchain / backend / host-facing integration | thin facade plus reserve shell | shell actual package と bridge-only host note の境界が固定される |
| `Macro 8` | domain / application realization | application-specific target not started | first target profile が user と合意される |

## current recommendation

- Lane A は narrow actualization を維持する。
- Lane B は mainline の「ついで」ではなく、separable な research program として前進させる。
- Lane C は thin facade と shell concern の boundary hardening に留める。
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
