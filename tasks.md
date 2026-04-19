# tasks

最終更新: 2026-04-19 10:23 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 規範判断の正本は `specs/`、detail は `plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/18` に寄せる。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current status at task level

- corrected runnable version floor
  - current authored sixteen と corrected prototype nonet は already runnable である
  - runner / CLI / regression / preview / actualization floor も machine-check で回る
  - したがって、「例をきちんと直した version が interpreter/runner で動く段階」に持っていくための substantive implementation package は、mapping 済み family ではもう残っていない
- current remaining work
  - remaining work は runtime enablement ではなく、later mixed gate と user-spec residual に移った
  - rough stimulus 自体は `samples/not_implemented/` preservation bucket に残し、corrected runnable version と混同しない
- current lanes
  - execution lane:
    `Macro 4 active on fixed authored/prototype floor`
  - theory-lab lane:
    `Macro 5 post-runnable mixed-gate actualization floor fixed + reopen package active`
  - reserve integration lane:
    `Macro 6 minimal working subset actual default / Macro 7 mixed + reserve reopen package active`

## current executable floor

- `samples/current-l2/`
  - current authored sixteen は inventory / runner / verification ladder / emitted artifact wiring / regression helper に乗っている
- `samples/prototype/`
  - corrected prototype nonet は explicit path + adjacent host-plan sidecar で runnable
  - helper-local `debug_outputs` / `verification_preview` / `artifact_preview` を出せる
- compare floor evidence
  - preview alignment pre-floor
  - model-check projection pre-floor
  - theorem discharge pre-floor
- actualization / narrowing floor evidence
  - Problem 1:
    theorem-first experimental pilot actualization (`specs/examples/470`)
    theorem-prover experimental binding preflight (`specs/examples/474`)
    theorem Lean-first non-production stub pilot actualization (`specs/examples/508`)
    theorem review-unit to Lean-stub repo-local artifact-conformance bridge (`specs/examples/509`)
    theorem Lean-stub representative trace-alignment bridge (`specs/examples/510`)
    theorem discharge actual-format probe (`specs/examples/479`)
    theorem discharge / public-theorem-contract threshold default (`specs/examples/481`)
    theorem contract shape threshold default (`specs/examples/485`)
    theorem transport/public-contract coupled later gate (`specs/examples/486`)
    theorem review-unit transport / notebook-contract actual adoption (`specs/examples/487`)
    theorem result-object preview / proof-object-schema reserve actualization (`specs/examples/491`)
    theorem result-object route actual adoption (`specs/examples/500`)
    theorem final public-contract reopen threshold (`specs/examples/506`)
    theorem result object / payload public-contract coupled later gate (`specs/examples/497`)
    theorem proof-object schema / prover-brand coupled later gate (`specs/examples/494`)
    model-check second-line concretization (`specs/examples/478`)
    model-check property/tool-seam probe (`specs/examples/480`)
    model-check property/tool-brand threshold default (`specs/examples/482`)
    model-check row-local property / checker-boundary actual adoption (`specs/examples/488`)
    model-check public-checker artifact preview / verifier-handoff reserve actualization (`specs/examples/492`)
    model-check tool-brand / verifier-handoff coupled later gate (`specs/examples/495`)
    model-check public checker artifact / migration coupled later gate (`specs/examples/498`)
    model-check checker-artifact route actual adoption (`specs/examples/501`)
    model-check final public-contract reopen threshold (`specs/examples/507`)
    witness/provider route actual adoption (`specs/examples/502`)
    order-handoff source wording route actual adoption (`specs/examples/503`)
  - Problem 2:
    authoritative-room vertical-slice actualization (`specs/examples/471`)
    witness/provider/artifact public-shape threshold default (`specs/examples/483`)
    witness/provider/artifact public-shape actual adoption (`specs/examples/489`)
    witness/provider public-contract / emitted-handoff coupled later gate (`specs/examples/493`)
    witness/provider public-schema coupled later gate (`specs/examples/499`)
    witness/provider schema route actual adoption (`specs/examples/504`)
    witness/provider final public-contract reopen threshold (`specs/examples/505`)
    order-handoff surface narrowing / stage-block secondary candidate (`specs/examples/473`)
    order-handoff surface/artifact threshold default (`specs/examples/484`)
    order-handoff surface actual adoption (`specs/examples/490`)
    order-handoff source wording / emitted-artifact coupled later gate (`specs/examples/496`)
    auditable-authority-witness strengthening actualization (`specs/examples/476`)
    delegated-rng-service practical actualization (`specs/examples/477`)
  - syntax / modality:
    minimal companion / experimental order-handoff surface (`specs/examples/472`)
- exact rough stimulus bucket
  - `samples/not_implemented/` は preservation bucket のまま維持する

## ordered self-driven packages

- Package 46 — order-handoff serial-scope sugar reserve surface
  - question:
    edge-row principal を保ったまま `serial on ...` / `stage` / `witness` family を helper-local reserve surface としてどこまで actualize するか
  - candidate family:
    edge-row principal、stage-block secondary、serial-scope sugar reserve
  - adequacy corpus:
    `p07 / p08 / p09`
  - promotion criteria:
    final source wordingやfinal grammarに上げず、current order-handoff wording line を補強する helper-local compare/actualization floor ができること
  - rough weight:
    medium (`Macro 5/6` reserve)
- Package 47 — witness/provider emitted-contract trace-alignment bridge
  - question:
    witness/provider route actual adoption 後に、public contract へ上げずに emitted-handoff / witness-provider trace alignment をどこまで repo-local に actualize するか
  - candidate family:
    witness/provider route first、public-schema/public-contract later keep、trace alignment helper first
  - adequacy corpus:
    `p07 / p08 / p09`
  - promotion criteria:
    final public schemaを採らずに representative alignment evidence を再現できること
  - rough weight:
    medium (`Macro 6/7` reserve)

## research-discovery items

| item | 何に影響するか | current recommendation |
|---|---|---|
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + structural marker family first を維持し、pressure が出るまで experimental adoption を待つ |
| final modal foundation / final source marker | syntax / modality / proof spine | partial basis + stronger family keep を維持し、final adoption は mixed gate に残す |

## remaining mixed gates

| topic | impact | current recommendation |
|---|---|---|
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + structural marker family first を維持し、experimental adoption は pressure が出るまで待つ |
| final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract | theorem-first pilot | review-unit transport first + notebook-consumer contract first + result-object route actual adoption + result/payload candidate only + proof-object-schema/prover-brand adjacent keep + theorem final public-contract reopen threshold + repo-local artifact-conformance bridge を維持し、final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract へは上げない |
| first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract | model-check line | row-local property route first と checker-boundary contract first と checker-artifact route actual adoption + verifier-handoff/tool-brand adjacent keep + migration candidate adjacent keep + model-check final public-contract reopen threshold を維持し、first settled property language / concrete tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract へは上げない |
| final source-surface handoff wording / final emitted-artifact schema | order-handoff public surface | edge-row / vertical continuation principal、stage-block secondary keep、thread/node same causal language keep、repo-local emitted artifact refs first、source-wording/emitted-artifact candidate only、source wording route actual adoption、serial sugar reserve を維持する |
| final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | shared-space stronger evidence | claim/payload split と witness/provider route non-collapse と repo-local emitted artifact refs first と witness/provider route actual adoption と witness/provider schema route actual adoption と witness/provider final public-contract reopen threshold と public-contract / emitted-handoff coupled-later gate と witness/provider public-schema coupled-later gate を維持し、final public contract 群は later に残す |
| final modal foundation / final source marker | syntax / modality | partial basis + stronger family keep を維持する |
| final public verifier contract / final parser-checker-runtime API | public surface | this line では採らない |

## true user-spec residuals

| item | impact | current recommendation |
|---|---|---|
| exhaustive final shared-space catalog beyond minimal subset | room profile / authority / replay scope | minimal working subset を current default に保ち、exhaustive finalization は後段へ |
| installed-binary / packaging / FFI / engine adapter / host integration target | backend / toolchain / operational target | repo-local CLI + tests + emitted artifacts + compare floor を near-end success に保つ |
| upper-layer application target beyond authoritative room scenario | domain-specific acceptance criteria | authoritative shared-space turn-based room を first target に保ち、broader target は later user choice に残す |

## self-driven macro phase reading

| macro phase | self-driven closeout 読み | 注記 |
|---|---|---|
| `Macro 0` | はい | maintenance closeout まで |
| `Macro 1` | はい | narrow semantic reopen の closeout まで |
| `Macro 2` | はい | current parser-free substrate closeout まで |
| `Macro 3` | はい | compile-ready minimal actualization closeout まで |
| `Macro 4` | はい | fixed-subset widening / adequacy closeout まで |
| `Macro 5` | はい | corrected runnable floor、threshold default、actual adoption、Lean stub pilot、repo-local artifact-conformance bridge、representative trace alignment bridge は close 済み。live package は Package 46 |
| `Macro 6` | はい | minimal working subset threshold default closeout まで |
| `Macro 7` | はい（repo-localまで） | repo-local CLI/tests/artifacts/compare floor まで。packaging/FFI は later |
| `Macro 8` | mixed | authoritative room default scenario までは narrow self-driven、broader app target は user-spec |

## anchor packages already closed

- compare-floor anchors:
  `specs/examples/458...465`
- actual-adoption anchors:
  `specs/examples/466...469`
- helper-local actualization / narrowing anchors:
  `specs/examples/470...474`
- deeper-theory / reserve / mixed-gate actualization anchors:
  `specs/examples/475...510`

## guard

- corrected runnable floor reached は final public language completion を意味しない
- live package nonzero は mixed gate / reserve line が still open であることを意味し、theory solved を意味しない
- `atomic_cut` を global consistent cut / durable commit / seq_cst fence と同一視しない
- low-level exact `memory_order` family を final source wordingにしない
- final parser grammar / final public API / final verifier contract / exhaustive final shared-space catalog をこの line で凍らせない
 - Package 48 — theorem actual Lean execution availability probe
  - question:
    representative trace alignment の次に、Lean tool が local に入った場合にだけ actual execution probe をどこまで narrow に試すか
  - candidate family:
    Lean-first keep / repo-local emitted stub bridge keep / actual execution optional probe
  - adequacy corpus:
    `e2 / e5 / p06`
  - promotion criteria:
    local tooling availability が満たされ、public theorem contract を採らずに actual tool invocation を reproducible に試せること
  - rough weight:
    medium (`Macro 5/7` reserve)
