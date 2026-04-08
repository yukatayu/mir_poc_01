# 0306 — review current L2 stage 3 multiline attachment first tranche

## Objective

`0305-current-l2-stage3-multiline-attachment-first-tranche.md` と対応する spec / code / mirror change が、
Phase 3 mainline の staged line と existing helper boundary を壊していないかを確認する。

## Scope and assumptions

- current L2 core semantics は変更しない。
- review は helper-local / test-only multiline attachment first tranche とその mirror に限定する。

## Documents consulted

- `docs/reports/0305-current-l2-stage3-multiline-attachment-first-tranche.md`
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. helper-local multiline attachment support が shared single attachment frame judgmentを越えて generic continuation search をしていないかを確認した。
2. blank line の扱いが docs-only cut と矛盾していないかを確認した。
3. mirror と report hygiene の factual drift を確認した。

## Evidence / outputs / test results

### Reviewer completion

- reviewer: `Godel`
- result: completion received

### Findings

- High: support helper が `perform` / `option` head の下を generic に探索して nested clause-like line を attachment header として拾っていたため、shared single attachment frame judgmentより広い over-acceptance があった。
- Medium: blank line を silent normalization で消していたため、current floor の `blank line は不可` と矛盾していた。
- Medium: review report が template stub のままだったため、completion ベースで埋める必要があった。

### Findings addressed in-task

- helper は immediate child attachment line だけを探索するように tightening した。
- blank line は helper-local で fail-closed に reject するようにした。
- red として nested header search / blank line normalization の 2 case を追加し、green まで確認した。

### No remaining substantive semantic finding

- mirror 側には factual drift がなく、shared single attachment frame を extraction bridge に留める judgment 自体は current stage line と整合している。

## What changed in understanding

- multiline attachment bridge の first tranche は、shared floor を守るために extraction 成功 path だけでなく over-acceptance guard まで含めて初めて最小になると確認できた。
- 次段で比較すべき論点は、request-local clause suite completion と malformed-source pair extension の sequencing だと明確になった。

## Open questions

- request-local clause suite ordering / multiplicity / dedent completion を、次段 docs-only comparison でどこまで shared / request-local 固有に切るか。
- malformed-source pair を増やすなら、`ensure:` missing block と request-local suite completion malformed family のどちらを先に扱うか。

## Suggested next prompt

`Phase 3 の次段として、request-local clause suite completion と multiline attachment malformed-source pair のどちらを先に扱うべきかを整理してください。`
