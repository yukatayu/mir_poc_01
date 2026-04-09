# 0383 — Phase 5 handoff artifact threshold and checkpoint sweep

## Objective

Phase 5 current package の次段として、

- mixed handoff row bundle を current docs-only default に維持するか
- boundary-specific handoff artifact へ split するか
- actual handoff emitter をどの threshold で切るか

を narrow に比較し、同時に Phase 4 / 5 checkpoint 後の mirror drift を抑える。

## Scope and assumptions

- `specs/examples/126...` と `127...` を前提にする。
- final public checker API、final theorem prover / protocol verifier schema、actual emitter は固定しない。
- docs-first comparison と checkpoint mirror sweep に留める。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. Phase 5 current package の current reading を再確認し、mixed row sketch の利点と split / emitter の premature risk を整理した。
2. `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` を追加し、
   - mixed row bundle を current docs-only default に維持する
   - boundary-specific split は concrete consumer pressure が出たときだけ reopen する
   - actual handoff emitter はさらに後段へ deferred に残す
   current first choice を固定した。
3. practical example として、
   - fallback chain の theorem row
   - room action の protocol row + runtime row
   を使い、mixed row が current phase では自然である理由を明文化した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を更新し、Phase 5 を third inventory package close として読む snapshot に揃えた。
5. stale wording を sweep し、`second inventory package close` や旧 reopen phrasing が残っていないことを確認した。

## Files changed

- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z' && df -h . && free -h
sed -n '1,260p' specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md
sed -n '1,280p' specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md
sed -n '1,240p' tasks.md
sed -n '1,240p' progress.md
sed -n '1,220p' plan/11-roadmap-near-term.md
sed -n '1,240p' docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
rg -n "126-current|127-current|Phase 5|handoff artifact|mixed row|boundary-specific" Documentation.md specs/00-document-map.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/13-heavy-future-workstreams.md plan/17-research-phases-and-autonomy-gates.md progress.md tasks.md
rg -n "second inventory package close|threshold comparison を残す|mixed row bundle と boundary-specific handoff artifact" plan/17-research-phases-and-autonomy-gates.md progress.md tasks.md docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `specs/examples/128...` では、mixed row bundle を current docs-only default に維持する judgment を固定した。
- `plan/11`、`plan/17`、`progress.md`、`tasks.md` は Phase 5 を third inventory package close として読む snapshot に揃った。
- `docs/research_abstract/phase5...` は mixed row default / split threshold / emitter defer を簡潔に mirror した。
- reviewer は 1 回起動したが、current session では completion 回収口が利用できず、`0384` に local diff inspection fallback を記録した。
- `python3 scripts/validate_docs.py` は成功した。
- `git diff --check` は無出力で通った。

## What changed in understanding

- Phase 5 later reopen candidate は、単なる「mixed row vs split comparison」ではなく、
  **concrete external consumer pressure が出たときにだけ boundary-specific split や actual emitter を reopen する**
  threshold 問題だと整理できた。
- current phase では row split の精密化より、
  mixed row default を維持したまま source evidence family を安定させる方が自然である。

## Open questions

- first concrete external consumer は theorem prover / protocol verifier / runtime policy のどこから現れるか。
- `evidence_refs` family を actual artifact ref に寄せるなら、どの ref が first stable candidate になるか。
- actual handoff emitter を helper stack に入れるなら、retention / bless / compare policy をどこまで先に固定するか。

## Suggested next prompt

`Phase 5 の later reopen 候補として、first concrete external consumer pressure を theorem / protocol / runtime の 3 系統で narrow comparison し、mixed row default を崩さずにどの consumer が先に handoff actualization を要求するかを整理してください。`
