# plan/17 — research phases and autonomy gates

## 目的

この文書は、repo 全体の長期研究を

- macro phase
- lane split
- autonomy gate
- stop condition

で見やすく整理する。

ここでいう phase は strict waterfall ではない。
複数 line の並走を前提にする。

## legacy checkpoint の扱い

- `specs/examples/...` や report に出てくる `Phase 1..7` は historical label として残す。
- repo 全体の current status には、それとは別に macro phase と lane split を使う。

## macro phase 一覧

`Macro 0〜8` は repo 全体の top-level macro axis として読む。
今のところ `Macro 9` 以降を足す前提はなく、後続課題は該当する macro へ戻して配置する。
したがって `Macro 8` は catch-all ではなく、application / domain realization 専用の phase として保つ。

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate | typical stop condition |
|---|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 低 | self-driven | snapshot/detail-side drift が増えたら更新 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven | L0/L1 invariant へ影響する変更が出る |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven | helper/public drift が出る |
| `Macro 3` | compile-ready minimal actualization | late | 中 | self-driven | support-only/public-candidate split が崩れる |
| `Macro 4` | executable fixed-subset sample expansion | active on fixed authored/prototype floor | 重 | self-driven | new sample family が core semantics を押し広げる |
| `Macro 5` | typed / theorem / model-check bridge | final-layer closeout packages active | 重 | self-driven up to narrow mixed gate | final public contract / full type calculus / concrete production binding が必要になる |
| `Macro 6` | fabric / shared-space / runtime evolution | minimal working subset actual default + public-seam compression closed | 重 | self-driven up to exhaustive-catalog gate | exhaustive catalog / stronger fairness profile が必要になる |
| `Macro 7` | toolchain / backend / host-facing integration | mixed with repo-local near-end success criteria | 重 | self-driven up to packaging / FFI gate | installed binary / packaging / external target が必要になる |
| `Macro 8` | domain / application realization | first authoritative-room scenario selected | とても重い | mixed beyond first scenario | broader application target が必要になる |

## current lane split

- execution lane:
  `Macro 4 active on fixed authored/prototype floor`
  （current-l2 authored sixteen と corrected prototype set `p01...p16` は fixed 済みだが、sample corpus 自体は theory-line adequacy corpus として active に保つ）
- theory-lab lane:
  `Macro 5 final-layer closeout packages active`
  （`specs/examples/458...519` で compare / actual-adoption / helper-local / deeper-theory floor が揃い、`520...605` で final-layer closeout defaults、Lean first slice、IFC / finite-index widening、parser-side tranche、theorem/model-check bridge reconnect、order-handoff source surface / artifact route tightening、authoritative-room first scenario helper summary tightening、authoritative-room reserve strengthening lane tightening、guided problem sample entrypoint / residual bundle matrix / problem bundle actualization / parser-side companion and bridge / parser-side inspector / parser-side representative mapping matrix / explained representative problem sample bundles / representative problem bundle smoke runner / representative problem bundle aggregate smoke summary / representative problem bundle failure-focused smoke diagnostics / representative problem bundle quickstart walkthrough hardening / representative problem quickstart CLI mirror / representative problem quickstart parity checks / representative problem mixed-gate reopen map refresh / representative problem split-package map refresh / Problem 1 typed source principal split helper actualization / Problem 1 theorem public-contract split helper actualization / Problem 1 model-check public-contract split helper actualization / Problem 2 source wording / emitted schema split helper actualization / Problem 2 witness-provider public-shape split helper actualization / representative problem reopen-map split closeout sync / remaining residual lane summary actualization / Problem 1 final-public-seam lane helper actualization / Problem 2 final-public-seam lane helper actualization / syntax-modality final-marker lane helper actualization / typed-checker first executable slice actualization / theorem-first emitted-artifact loop hardening / authoritative-room runnable scenario loop hardening / Problem 1 executable residual reopen sync / Problem 2 executable residual reopen sync / once-through closeout summary sync が actualize 済みである。current next queue は Package 133...135 と later mixed/user-spec residual にある）
- reserve integration lane:
  `Macro 6 minimal working subset actual default / Macro 7 mixed`
  （authoritative room minimal working subset と repo-local near-end success criteria は current default に上がったが、exhaustive final catalog、installed-binary / packaging / FFI、broader host integration は still later に残る）

## autonomy gate detail

### self-driven でよい line

- semantic kernel の narrow refinement
- parser-free regression / helper maintenance
- compile-ready minimal actualization の narrow widening
- fixed-subset source sample expansion
- typed attachment candidate inventory
- theorem pilot planning と wording hardening
- model-check projection / property-family reserve planning、bridge grain note、small-cluster projection keep/drop refresh
- order / handoff / syntax / modality の docs-first comparison と follow-up bridge note
- verifier-boundary matrix と adequacy corpus の docs-first hardening
- minimal companion / experimental order-handoff surface
- stage-block secondary order-handoff surface
- model-check second-line concretization（close 済み）
- theorem-prover experimental binding preflight
- verifier preview alignment pre-floor の helper-local compare hardening
- model-check projection pre-floor の helper-local compare hardening
- theorem discharge pre-floor の helper-local compare hardening
- theorem discharge actual-format probe（close 済み）
- model-check property/tool-seam probe（close 済み）
- principal theory spine / layered typing/proof architecture / Lean-first proof roadmap の docs-first integration
- layered strong typing / IFC first-fragment
- Lean formal skeleton / proof obligations
- `auditable_authority_witness` strengthening actualization
- `delegated_rng_service` practical actualization
- witness-provider-artifact public-shape threshold package (`M3`, close 済み)
- theorem discharge / public-theorem-contract threshold package (`M1`, close 済み)
- theorem contract shape threshold package (close 済み)
- theorem transport/public-contract coupled later-gate package (close 済み)
- model-check property-language / tool-brand threshold package (`M2`, close 済み)

### boundary までは self-driven でよい line

- shared-space docs-first boundary
- docs-first I/O / host-facing boundary
- ordering / `memory_order` reinterpretation の theory-first inventory
- small non-production compare helper / simulator / executable semantics

### mixed gate の line

- stronger typed-surface promotion の実昇格
- theorem discharge result / public-contract の具体 contract 化
- model-check settled property language / concrete tool seam の確定
- concrete theorem prover / model-check tool binding
- stronger fairness / replay / provider-attestation profile
- host binding artifact から concrete adapter target への移行
- public operational shell から installed-binary / packaging への移行
- comparison candidate を decision register に actual adoption judgment として昇格させるかどうか

### user specification が必要な line

- shared-space exhaustive final catalog beyond minimal working subset
- installed-binary / packaging / FFI / engine adapter / host integration target
- upper-layer application target beyond authoritative-room first scenario

## theory-lab line

### 位置づけ

- theory-lab line は execution lane と separable に回す。
- 主眼は
  - literature-backed comparison
  - adequacy corpus
  - falsification
  - tiny non-production prototype
  - verifier-boundary matrix
  - current first-line / stop-line integration
  である。

### promotion rule

- exploratory candidate は report first に留める。
- accepted comparison / wording / package order だけを `plan/` や `specs/examples/` に昇格させる。
- main docs / plan / spec を更新するのは integrator line だけにする。

### stop line

- final parser grammar
- final public parser / checker / runtime API
- full strong type system
- concrete theorem / model-check production contract
- shared-space final catalog
- backend / codegen public surface
- raw FFI / game engine direct binding actualization
- upper-layer application contract

## current judgments

- current repo は architecture-first だが、fixed-subset runnable path を already 持つ。
- type / proof / model-check line は「まだ無理」ではなく、`specs/examples/459` で current first line / stop line / later gate を 1 本に揃えた上で mixed gate を切り分けられる段階にある。
- order / handoff line も同様に、`specs/examples/460` で current first line / retained alternatives / mixed-gate boundary を 1 本に揃えた段階にある。
- syntax / modality line も同様に、`specs/examples/461` で current first line / retained alternatives / stop line / trigger を 1 本に揃えた段階にある。
- near-end closeout / mixed-gate-only reading も `specs/examples/462` で compare-floor 側の current queue reading に上がっている。
- verifier preview alignment pre-floor も `specs/examples/463` で current helper-local compare floor に上がっている。
- model-check projection pre-floor も `specs/examples/464` で current helper-local compare floor に上がっている。
- theorem discharge pre-floor も `specs/examples/465` で current helper-local compare floor に上がっている。
- Problem 1 actual adoption package と theorem-first default は `specs/examples/466` で current package judgment に上がっている。
- Problem 2 actual adoption package と authoritative-room default profile は `specs/examples/467` で current package judgment に上がっている。
- syntax / modality current recommendation は `specs/examples/468` で current package judgment に上がっている。
- near-end closeout after defaults は `specs/examples/469` で queue nonzero / residual mixed-user-spec gate reading に上がっている。
- theorem-first experimental pilot actualization は `specs/examples/470` で current helper-local actualization floor に上がっている。
- authoritative-room vertical-slice emitted-artifact ratchet は `specs/examples/471` で current helper-local actualization floor に上がっている。
- minimal companion / experimental order-handoff surface は `specs/examples/472` で current helper-local actualization floor に上がっている。
- order-handoff surface narrowing / stage-block secondary candidate は `specs/examples/473` で current helper-local actualization / narrowing floor に上がっている。
- theorem-prover experimental binding preflight は `specs/examples/474` で current helper-local actualization floor に上がっている。
- principal theory spine / Lean-first proof roadmap は `specs/examples/475` で current theory-lab direction に上がっている。
- auditable-authority-witness strengthening actualization は `specs/examples/476` で current reserve strengthening actualization floor に上がっている。
- delegated-rng-service practical actualization は `specs/examples/477` で current reserve practical actualization floor に上がっている。
- model-check second-line concretization は `specs/examples/478` で current helper-local second-line actualization floor に上がっている。
- theorem discharge actual-format probe は `specs/examples/479` で current helper-local theorem mixed-gate actualization floor に上がっている。
- model-check property/tool-seam probe は `specs/examples/480` で current helper-local model-check mixed-gate actualization floor に上がっている。
- model-check property/tool-brand threshold default は `specs/examples/482` で current helper-local model-check mixed-gate threshold floor に上がっている。
- witness/provider/artifact public-shape threshold default は `specs/examples/483` で current helper-local shared-space reserve threshold floor に上がっている。
- order-handoff surface / artifact threshold default は `specs/examples/484` で current helper-local order-handoff later mixed-gate threshold floor に上がっている。
- theorem contract shape threshold default は `specs/examples/485` で current helper-local theorem later mixed-gate shape threshold floor に上がっている。
- theorem transport/public-contract coupled later gate は `specs/examples/486` で current helper-local theorem actual-adoption-just-before floor に上がっている。
- theorem review-unit transport / notebook-contract actual adoption は `specs/examples/487` で current helper-local theorem actual adoption floor に上がっている。
- theorem result-object preview / proof-object-schema reserve actualization は `specs/examples/491` で current helper-local theorem mixed-gate actualization floor に上がっている。
- theorem result-object route actual adoption は `specs/examples/500` で current helper-local theorem actual-adoption floor に上がっている。
- theorem final public-contract reopen threshold は `specs/examples/506` で current helper-local theorem threshold floor に上がっている。
- theorem Lean-first non-production stub pilot actualization は `specs/examples/508` で current helper-local theorem external-pilot floor に上がっている。
- theorem review-unit to Lean-stub repo-local artifact-conformance bridge は `specs/examples/509` で current helper-local theorem second-stage conformance floor に上がっている。
- theorem Lean-stub representative trace-alignment bridge は `specs/examples/510` で current helper-local theorem representative bridge floor に上がっている。
- order-handoff serial-scope reserve surface は `specs/examples/511` で current helper-local order-handoff reserve-surface floor に上がっている。
- witness/provider emitted-contract representative trace-alignment bridge は `specs/examples/512` で current helper-local shared-space representative bridge floor に上がっている。
- theorem actual Lean execution availability probe は `specs/examples/513` で current theorem environment stop-line floor に上がっている。
- theorem public-seam compression は `specs/examples/514` で current helper-local theorem residual-compression floor に上がっている。
- order-handoff / witness-provider final public-seam compression は `specs/examples/515` で current helper-local Problem 2 / shared-space residual-compression floor に上がっている。
- theorem toolchain probe / reopen manifest は `specs/examples/516` で current helper-local theorem toolchain-ready floor に上がっている。
- model-check public-seam compression は `specs/examples/517` で current helper-local model-check residual-compression floor に上がっている。
- theorem actual Lean execution narrow probe は `specs/examples/518` で current helper-local theorem representative-static actual-execution floor に上がっている。
- theorem actual Lean execution representative prototype widening は `specs/examples/519` で current helper-local theorem representative-prototype actual-execution floor に上がっている。
- final-layer closeout defaults / reopened self-driven queue は `specs/examples/520` で current queue reconstruction floor に上がっている。
- theorem proof-object schema / prover-brand coupled later gate は `specs/examples/494` で current helper-local theorem mixed-gate actualization floor に上がっている。
- model-check row-local property / checker-boundary actual adoption は `specs/examples/488` で current helper-local model-check actual adoption floor に上がっている。
- model-check public-checker artifact preview / verifier-handoff reserve actualization は `specs/examples/492` で current helper-local model-check mixed-gate actualization floor に上がっている。
- model-check tool-brand / verifier-handoff coupled later gate は `specs/examples/495` で current helper-local model-check mixed-gate actualization floor に上がっている。
- model-check public checker artifact / migration coupled later gate は `specs/examples/498` で current helper-local model-check mixed-gate actualization floor に上がっている。
- model-check checker-artifact route actual adoption は `specs/examples/501` で current helper-local model-check actual-adoption floor に上がっている。
- model-check final public-contract reopen threshold は `specs/examples/507` で current helper-local model-check threshold floor に上がっている。
- witness/provider public-schema coupled later gate は `specs/examples/499` で current helper-local shared-space mixed-gate actualization floor に上がっている。
- witness/provider route actual adoption は `specs/examples/502` で current helper-local shared-space actual-adoption floor に上がっている。
- witness/provider schema route actual adoption は `specs/examples/504` で current helper-local shared-space actual-adoption floor に上がっている。
- witness/provider final public-contract reopen threshold は `specs/examples/505` で current helper-local shared-space threshold floor に上がっている。
- order-handoff source wording / emitted-artifact coupled later gate は `specs/examples/496` で current helper-local order-handoff mixed-gate actualization floor に上がっている。
- order-handoff source wording route actual adoption は `specs/examples/503` で current helper-local order-handoff actual-adoption floor に上がっている。
- theorem result object / payload public-contract coupled later gate は `specs/examples/497` で current helper-local theorem mixed-gate actualization floor に上がっている。
- witness/provider/artifact public-shape actual adoption は `specs/examples/489` で current helper-local shared-space actual adoption floor に上がっている。
- witness/provider public-contract / emitted-handoff coupled later gate は `specs/examples/493` で current helper-local shared-space/order-handoff mixed-gate actualization floor に上がっている。
- order-handoff surface actual adoption は `specs/examples/490` で current helper-local order-handoff actual adoption floor に上がっている。
- order / handoff / syntax / modality line も「未着手」ではなく、current first line / retained alternatives / mixed-gate boundary を docs-first に進められる段階にある。
- corrected prototype tranche close は、theory-lab solved や queue empty を意味しない。
- `p06` / `p07` / `p08` / `p09` は sample-visible corrected prototype であり、final typed calculus / final source wording / final shared-space profile / final public provider contract を意味しない。
- `p09` も sample-visible corrected prototype であり、final public provider receipt schema / delegated attestation を意味しない。
- theory-lab line は repo 全体から見ると advanced line であり、current reading では actual-adoption / helper-local actualization / narrowing / reserve strengthening / reserve practical actualization / model-check second-line actualization / theorem discharge actual-format probe / model-check property-tool-seam probe / theorem discharge-public-contract threshold default / theorem contract shape threshold default / theorem transport/public-contract coupled later gate / theorem review-unit actual adoption / model-check-property-tool threshold default / model-check row-local property actual adoption / witness-provider-artifact public-shape threshold / actual adoption / witness-provider public-contract/emitted-contract coupled later gate / order-handoff surface threshold / actual adoption / theorem public-seam compression / model-check public-seam compression / order-handoff-witness-provider public-seam compression / representative Lean sample set actual Lean execution / Package 58 helper-preview widening / Package 59 closeout sync / Package 93 Lean-first formal skeleton hardening / Package 94 theorem-model-check bridge reconnect まで close 済みだが、remaining work は post-runnable residual mixed gate / true user-spec residual に残る。
