# Report 0235 — current L2 first parser spike staging

## 1. Title and identifier

- Report 0235
- current L2 first parser spike staging

## 2. Objective

current L2 first parser cut inventory を前提に、
actual parser spike を切るならどの semantic cluster から staged に入れるべきかを
source-backed に比較し、current next narrow step を docs-only で固定する。

## 3. Scope and assumptions

- current L2 の core semantics、parser-free PoC、detached validation loop は変更しない。
- final parser grammar、exact lexical choice、reserved keyword の最終集合は固定しない。
- 今回は docs-only comparison に留め、parser 実装や checker API の actualization は行わない。
- `plan/` は relevant mirror を更新する。

## 4. Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/01-current-l2-surface-syntax-candidates.md`
10. `specs/examples/29-current-l2-first-parser-subset-inventory.md`
11. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
12. `plan/00-index.md`
13. `plan/06-surface-notation-status.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/13-heavy-future-workstreams.md`
17. `plan/90-source-traceability.md`
18. `progress.md`
19. `docs/reports/0114-third-remaining-problem-parser-boundary.md`
20. `docs/reports/0132-current-l2-first-parser-subset-inventory.md`
21. `docs/reports/0133-review-current-l2-first-parser-subset-inventory.md`
22. `docs/reports/0232-try-rollback-first-tranche-generic-public-recheck.md`
23. `docs/reports/0234-review-try-rollback-first-tranche-generic-public-recheck-followup.md`
24. `crates/mir-semantics/src/lib.rs`
25. `crates/mir-semantics/src/harness.rs`
26. `scripts/current_l2_same_lineage_checker.py`
27. `scripts/current_l2_missing_option_checker.py`
28. `scripts/current_l2_capability_checker.py`
29. `scripts/current_l2_try_rollback_structural_checker.py`
30. `scripts/current_l2_detached_loop.py`
31. `crates/mir-ast/tests/fixtures/current-l2/`

## 5. Actions taken

1. parser boundary / first parser cut inventory の既存 judgment を読み直し、full inventory 自体は変えないことを前提にした。
2. current actual checker / validation loop の code anchor と fixture corpus から、次の 3 cluster を再抽出した。
   - chain / declaration structural floor
   - `try` / rollback structural floor
   - request / admissibility cluster
3. actual parser spike の比較案を
   - monolithic first parser spike
   - checker-led staged spike
   - runtime-led staged spike
   の 3 つに分けた。
4. `specs/examples/73-current-l2-first-parser-spike-staging.md` を追加し、current next narrow step は checker-led staged spike だと整理した。
5. `specs/examples/29-current-l2-first-parser-subset-inventory.md`、`Documentation.md`、`specs/00-document-map.md`、`plan/06-surface-notation-status.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## 6. Evidence / outputs / test results

### code / fixture anchor

- `crates/mir-semantics/src/lib.rs`
  - `static_gate_detailed`
  - `collect_try_rollback_structural_reasons`
- `crates/mir-semantics/src/harness.rs`
  - `run_bundle`
  - `batch_summary_from_discovery`
- chain / declaration floor fixture:
  - `e4`, `e12`, `e13`, `e16`, `e17`, `e20`
- `try` / rollback floor fixture:
  - `e21`, `e22`, `e23`, `e24`
- request / admissibility fixture:
  - `e3`, `e10`, `e11`

### docs / diff validation

```text
python3 scripts/validate_docs.py
```

```text
git diff --check
```

## 7. What changed in understanding

- first parser cut inventory と actual parser spike の sequencing は別判断として切る方が自然である。
- current repo の actual evidence に最も近いのは、request-heavy surface よりも chain / declaration structural floor と `try` / rollback structural floor である。
- したがって current next narrow step では、actual parser spike を monolithic に始めるより checker-led staged spike として比較固定する方が手戻りが小さい。

## 8. Open questions

- chain / declaration structural floor の stage 1 parser spike で、minimal predicate fragment をどこまで parse 対象に含めるか。
- `try` / rollback structural floor を stage 2 へ送るとき、malformed source compare と runtime representative compare のどこまでを同じ parser spike に含めるか。
- request / admissibility cluster を stage 3 に送る前に、predicate fragment completion と clause attachment parser rule をどこまで inventory 化するか。

## 9. Files changed

- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/06-surface-notation-status.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 10. Commands run

- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,240p' specs/00-document-map.md`
- `sed -n '1,240p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,240p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `sed -n '1,260p' specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `sed -n '1,260p' plan/00-index.md`
- `sed -n '1,260p' plan/06-surface-notation-status.md`
- `sed -n '1,260p' plan/11-roadmap-near-term.md`
- `sed -n '1,260p' plan/12-open-problems-and-risks.md`
- `sed -n '1,260p' plan/13-heavy-future-workstreams.md`
- `sed -n '1,260p' progress.md`
- `rg -n "static_gate_detailed|collect_try_rollback_structural_reasons|run_bundle|batch_summary_from_discovery" crates/mir-semantics/src/lib.rs crates/mir-semantics/src/harness.rs`
- `rg -n "smoke-same-lineage-checker|smoke-missing-option-checker|smoke-capability-checker|smoke-try-rollback-structural-checker" scripts/current_l2_detached_loop.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## 11. plan/ updates

- `plan/06-surface-notation-status.md` 更新
- `plan/11-roadmap-near-term.md` 更新
- `plan/12-open-problems-and-risks.md` 更新
- `plan/90-source-traceability.md` 更新

## 12. Suggested next prompt

`current L2 parser-free PoC 基盤と first parser cut inventory を前提に、checker-led staged spike の stage 1 を actual parser spike として切るなら、option declaration core / explicit edge-row family / minimal guard fragment のどこまでを accepted parse cluster に含め、どこから先を elaboration helper や later stage に残すかを narrow に比較してください。`
