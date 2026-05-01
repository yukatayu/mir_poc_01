# plan/10 — 全体ロードマップ

## 目的

この文書は、repo 全体の研究を

- どこまで executable mainline として進めるか
- どこから先を theory-lab / reserve integration line に分けるか
- 何を heavy future workstream に残すか

の観点で整理する。

近接の self-driven package queue は `plan/11-roadmap-near-term.md` と `tasks.md` を正とする。
この文書は macro-level の lane / gate / historical recovery 読みを保つための repository memory であり、
package 3 以降の Mirrorea future-axis 近接順序をここで上書きしない。
`progress.md` / `tasks.md` が current promoted workline を maintenance と読む場合、この文書の lane 記述は queue authority ではなく lane memory として読む。

## current overall reading

現在の repo は、次の 3 つを同時に持つ。

1. semantics / invariants / boundary の docs-first repo
2. parser-free current L2 の runnable validation substrate
3. compile-ready minimal actualization と fixed-subset source sample の narrow runnable path

したがって repository-memory reading は、
**architecture-first だが、fixed subset は理論と実行を ratchet で並走させている**
である。

## current lane split

### Lane A. execution

- 主眼:
  fixed-subset source sample と malformed family の narrow widening
- standing line in this memory:
  `Macro 4 active on fixed authored/prototype floor`
  （current-l2 authored sixteen、corrected prototype set `p01...p14`、runner / CLI / regression floor は fixed 済みであり、sample corpus 自体は adequacy corpus として active に保つ）
- goal:
  runnable corpus を広げつつ、runner / ladder / regression / docs mirror を崩さない

### Lane B. theory-lab

- 主眼:
  typed / theorem / model-check / order / memory / syntax / modality を
  docs-first comparison と boundary planning で進める
- standing line in this memory:
  `Macro 5 repo-local closeout reached / no promoted self-driven package`
  （`specs/examples/458...563` compare / adoption / helper-local actualization / actual Lean execution floor は repository memory として close 済みであり、current snapshot では theory-lab 側に新しい promoted self-driven package は置いていない。残るのは mixed gate、reserve interpretation、public-boundary readiness、docs freshness であり、queue authority は `progress.md` / `tasks.md` / `plan/11-roadmap-near-term.md` を正とする）
- goal:
  full implementation へ飛ばず、candidate family、adequacy corpus、verifier-boundary、stop line を固定した上で、post-runnable closeout package と mixed-gate package を narrow に閉じる

### Lane C. reserve integration

- 主眼:
  thin facade / public shell / shared-space / host-I/O boundary を hidden promotion なしで整理する
- standing line in this memory:
  `Macro 6-7 repo-local floor reached / mixed gate held open`
  （carrier/runtime floor、typed external preview、network canary、projection/codegen bridge evidence、viewer prototype inventory、hot-plug `P21` narrow floor、post-`P21` docs-first trilogy は current scope で close 済みである。`U1` actual commitment が未決の間、ここで自走する current line は new implementation ではなく maintenance / docs freshness / validation rerun である）
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
| `Macro 5` | typed / theorem / model-check bridge | repo-local closeout reached | mixed gate と reserve memory を drift させず、new promoted queue を `tasks.md` と矛盾させない |
| `Macro 6` | fabric / shared-space / runtime evolution | P21 narrow floor + docs-first trilogy closed | shared-space docs-first boundary、kept-later boundary、public-freeze hold line を maintenance 付きで保てる |
| `Macro 7` | toolchain / backend / host-facing integration | repo-side prototype / guardrail floor closed; public target unresolved | shell / facade / packaging boundary を final target profile から分離したまま `U1` actual commitment を待てる |
| `Macro 8` | domain / application realization | application-specific target not started | first target profile が user と合意される |

## historical sync_v3 recovery policy

- `旧資料_参考_ChatGPT_03_sync_v3/` は、current mainline の numbered reopen queue として扱わない。
- 旧資料から回収するものは、fixed surface / concrete stack / monolithic product roadmap ではなく、
  retained concern / stressor family / gate memory として owner 別に再配置する。
- solver / projection / artifact-trace scaling、historical requirement amnesia、monolithic reimport risk は `plan/12` で管理する。
- portal / multi-world / operational trust / audit / registry / observability / benchmark family は `plan/13` heavy line に置く。
- typed / theorem / model-check / ordering 側の proof-promotion memory、finite decidable first fragment、non-adoption boundary は `plan/18` で管理する。
- old fixed EBNF、old bundled `atomic_cut` / `durable_cut` / `barrier` surface、tool brand fixed、Stage `0...4` / `1.0` product roadmap は current repo にそのまま戻さない。

## standing recommendation

- Lane A は narrow actualization を維持する。
  - corrected runnable prototype と helper-local `debug_outputs` / `verification_preview` を使って usability / falsifier comparison を進めてよい。
- Lane B は mainline の「ついで」ではなく、separable な research program として前進させる。
- Lane B の narrow mixed-gate pre-floor では、preview summary、model-check projection helper、theorem discharge helper を final public contract や settled property language にせず compare floor を厚くしてよい。
- Lane C は thin facade と shell concern の boundary hardening、および mixed-gate boundary の明確化に留める。
- historical recovery は owner split を通して行い、old line を numbered queue や settled surface として再昇格させない。
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
