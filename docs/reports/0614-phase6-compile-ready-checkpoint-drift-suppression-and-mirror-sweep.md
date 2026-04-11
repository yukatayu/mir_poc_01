# 0614 — phase6 compile-ready checkpoint drift suppression and mirror sweep

## Objective

Phase 6 front-half compile-ready checkpoint close 後の snapshot drift を suppress し、
`Documentation.md` / `progress.md` / `tasks.md` / research abstract / relevant `plan/`
の current line を repo-wide に揃える。

## Scope and assumptions

- `specs/examples/303...304` の compile-ready verification / formal hook cutは fixed 済みとし、今回 new normative spec は追加しない。
- current line は checkpoint sweep の先へ送り、**Phase 6 next reopen sequencing（parser second tranche widen vs concrete formal tool binding）** を snapshot 文書側の next mainline に置く。
- `specs/00-document-map.md` は new spec が無いため **更新不要** と判断した。
- `plan/09-helper-stack-and-responsibility-map.md` と `plan/12-open-problems-and-risks.md` は Task 3 で current state を吸えているため、この sweep では **更新不要** と判断した。
- `docs/research_abstract/README.md` は phase index 自体に変更が無いため **更新不要** と判断した。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0613-phase6-compile-ready-verification-and-formal-hook-package.md`
- `specs/examples/297-current-l2-phase4-shared-space-self-driven-closeout-ready-phase5-proof-protocol-runtime-policy-handoff-closeout-comparison.md`
- `specs/examples/298-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-minimal-phase5-proof-protocol-runtime-policy-handoff-closeout-threshold.md`
- `specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
- `specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
- `specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
- `specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-actual-checker-runtime-skeleton-first-tranche-threshold.md`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`

## Actions taken

1. repo-wide current line を再監査し、snapshot/core docs のうち compile-ready formal-hook close 後も古い mainline を指していた箇所を洗い出した。
2. `Documentation.md`、`progress.md`、`tasks.md` を checkpoint sweep close 後の current line へ更新し、current mainline を **Phase 6 next reopen sequencing** に切り替えた。
3. `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` と `docs/research_abstract/phase6-compile-ready-minimal-actualization.md` を更新し、adjacent abstract 間で old mainline が残らないようにした。
4. `plan/00`、`plan/01`、`plan/07`、`plan/10`、`plan/11`、`plan/17` を更新し、compile-ready checkpoint close 後の reading と next reopen sequencing を plan 側にも mirror した。
5. reviewer の drift 指摘に合わせて `plan/11` blocker wording と `plan/17` Phase 6 status line の矛盾を解消した。
6. `plan/90-source-traceability.md` に sweep addendum を追加し、今回の snapshot rewrites の source roots を記録した。

## Files changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0614-phase6-compile-ready-checkpoint-drift-suppression-and-mirror-sweep.md`

## Commands run and exact outputs

```bash
rg -n "Phase 6 compile-ready verification and formal hook|compile-ready verification / formal hook|compile-ready checkpoint drift suppression / mirror sweep|303\\.\\.304|tool-neutral formal hook|current promoted line|next promoted line" README.md Documentation.md progress.md tasks.md plan docs specs docs/research_abstract -g '!docs/reports/**'
python3 scripts/validate_docs.py
git diff --check
git status --short
```

- `rg -n ...`
  - stale snapshot wording と current-line mismatch を inventory した
- `python3 scripts/validate_docs.py`
  - docs scaffold と numbered report count の整合を確認した
- `git diff --check`
  - whitespace / marker drift が無いことを確認した
- `git status --short`
  - task close 直前は Task 4 docs-only edits だけが並ぶ状態だった

## Evidence / outputs / test results

- reviewer audit では、`phase5` abstract の current mainline lag、`plan/17` Phase 6 status line mismatch、`plan/11` blocker wording の task number drift を検出した
- それらを修正した後、snapshot/core docs で追加 finding は無い状態まで揃えた
- `python3 scripts/validate_docs.py` は current report count を正しく認識した
- `git diff --check` は無出力だった

## What changed in understanding

- compile-ready checkpoint close では、new spec を足さなくても snapshot 文書側の current line を次の sequencing question へ送るだけで repo memory を clean に保てる。
- adjacent abstract や phase gate 文書は、core snapshot より 1 package 遅れて drift しやすいので、checkpoint sweep では周辺文書まで含めて監査する必要があった。
- Phase 6 front-half は compile-ready close まで来たため、次の重要な問いは実装不足そのものではなく **どの reopen line を先に取るか** に移っている。

## Open questions

- parser second tranche widen と concrete formal tool binding のどちらを next line に置くべきか
- concrete formal tool binding を theorem side と model-check sideのどちらから narrow に開くべきか
- checkpoint close 後の reserve path をどこまで docs-first に圧縮してよいか

## Suggested next prompt

```text
Phase 6 next reopen sequencing を進め、parser second tranche widen と concrete formal tool binding のどちらを先に扱うべきかを comparison / threshold として整理してください。
```
