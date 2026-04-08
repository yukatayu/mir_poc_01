# Report 0335 — current L2 phase3 resume side selection

- Date: 2026-04-08T09:12:00Z
- Author / agent: Codex
- Scope: request contract subset family freeze 後に、Phase 3 主線を parser boundary staging 側と first checker cut connection 側のどちらから再開するかの docs-only comparison
- Decision levels touched: L2

## 1. Objective

request contract subset family を freeze した後の Phase 3 主線を、parser boundary staging 側ではなく first checker cut connection 側へ戻すべきかを source-backed に整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/111-current-l2-stage3-request-contract-subset-freeze-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 3. Actions taken

1. request contract subset family freeze 後の戻り先を、parser boundary staging / first checker cut connection / simultaneous advance の 3 案で比較した。
2. `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md` の first checker cut readiness judgment と、
   `specs/examples/73-current-l2-first-parser-spike-staging.md` の checker-led staged spike judgment を読み直した。
3. syntax pressure と proof / checker boundary の整合を比較し、current next step は first checker cut connection 側だと `specs/examples/112...` に整理した。

## 4. Files changed

- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0335-current-l2-phase3-resume-side-selection.md`

## 5. Commands run and exact outputs

```bash
rg -n "mainline を first checker cut|parser boundary / first checker cut|first checker cut readiness" \
  specs/examples/45-current-l2-first-checker-cut-regression-baseline.md

rg -n "checker-led staged spike|first checker cut との handoff|current repo の next narrow step" \
  specs/examples/73-current-l2-first-parser-spike-staging.md

rg -n "Phase 3|first checker cut|accepted parse cluster" \
  plan/11-roadmap-near-term.md progress.md plan/07-parser-free-poc-stack.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md
```

主要出力:

```text
specs/examples/45-current-l2-first-checker-cut-regression-baseline.md:107:**案 1. mainline を first checker cut へ戻す**
specs/examples/45-current-l2-first-checker-cut-regression-baseline.md:115:4. parser boundary → first checker cut → theorem prover boundary という順序に戻す方が全体 roadmap と整合する
specs/examples/73-current-l2-first-parser-spike-staging.md:116:### 案 2. checker-led staged spike
specs/examples/73-current-l2-first-parser-spike-staging.md:197:current repo の next narrow step としては、**案 2. checker-led staged spike** が最も自然である。
plan/11-roadmap-near-term.md:10: current Phase 3 の next narrow step ...
plan/12-open-problems-and-risks.md:233:- current side-selection judgment としては、その戻り先でも parser-side accepted cluster widening を先に再開するより、first checker cut connection 側から existing parser evidence の reconnect family を比べる方が自然である。理由は、syntax pressure を still 抑えつつ local / structural / decidable floor の整理を前に出せるためである。
plan/90-source-traceability.md: 2026-04-08 phase 3 resume side selection addendum
```

## 6. Evidence / findings

- request contract subset family は freeze したので、parser-side widening をすぐ再開するより、
  `specs/examples/45...` の first checker cut readiness と
  `specs/examples/73...` の checker-led parser staging を接続し直す形で、
  checker-side reconnection を先に比べる方が筋がよい。
- current repo は parser boundary と checker boundary を混ぜない narrow progression を採っているため、simultaneous advance は不適切である。

## 7. Changes in understanding

- Phase 3 の次段は parser-side accepted cluster widening ではなく、existing parser evidence を first checker cut inventory へどう reconnect するかの比較だと読む方が自然である。

## 8. Open questions

- existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するか。

## 9. Suggested next prompt

`specs/examples/112-current-l2-phase3-resume-side-selection.md` を前提に、existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するかを narrow に比較してください。
