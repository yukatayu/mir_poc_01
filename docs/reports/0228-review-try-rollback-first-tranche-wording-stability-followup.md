# 0228 — review followup for try-rollback first-tranche wording stability

## Objective

`specs/examples/70-current-l2-try-rollback-first-tranche-wording-stability.md` と、その mirror / report chain が
current first-tranche actual contract、helper boundary、report / traceability hygiene と矛盾していないかを review する。

## Scope and assumptions

- current task は review-only とし、target docs 自体の修正は行わない。
- current first-tranche actual contract の source of truth は `specs/examples/68`、`e23` / `e24` fixture、`scripts/current_l2_try_rollback_structural_checker.py`、`scripts/current_l2_detached_loop.py` とする。
- `plan/` 更新不要。
- `progress.md` 更新不要。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/69-current-l2-try-rollback-second-malformed-static-tranche-comparison.md`
- `specs/examples/70-current-l2-try-rollback-first-tranche-wording-stability.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `docs/reports/0226-try-rollback-first-tranche-wording-stability.md`
- `docs/reports/0227-review-try-rollback-first-tranche-wording-stability.md`
- `crates/mir-semantics/src/lib.rs`
- `scripts/current_l2_try_rollback_structural_checker.py`
- `scripts/current_l2_detached_loop.py`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`
- `crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json`

## Actions taken

1. AGENTS.md の読書順に従って基礎文書、progress、基礎 specs、relevant plan files を再読した。
2. `specs/examples/68` / actual fixture / helper script / smoke command から、current first-tranche contract を source-backed に再確認した。
3. target docs と reports を横断し、`e23` / `e24`、reason wording、helper-local row family、shared/generic/public boundary の mirror drift を点検した。
4. report / traceability hygiene として、pending review placeholder や stale mirror bullet の有無を確認した。

## Files changed

- `docs/reports/0228-review-try-rollback-first-tranche-wording-stability-followup.md`

## Commands run

```bash
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json --run-label review-wording-stability-e23 --overwrite
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json --run-label review-wording-stability-e24 --overwrite
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `e23` smoke

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
actual findings:
[
  {
    "subject_kind": "TryFallback",
    "finding_kind": "missing_fallback_body"
  }
]
```

- `e24` smoke

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
actual findings:
[
  {
    "subject_kind": "AtomicCut",
    "finding_kind": "disallowed_fallback_placement"
  }
]
```

- emitted static gate artifact reason wording

```text
e23: ["try fallback body must not be empty"]
e24: ["atomic cut may not appear inside fallback body"]
```

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 227 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

## What changed in understanding

- `specs/examples/70` 自体、`Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`progress.md`、`docs/reports/0226` は current exact wording / row family fixed-working-set judgment と整合している。
- ただし `plan/15-current-l2-fixture-authoring-template.md` には、first tranche actualization 前の mirror bullet が残っており、current actual command surface / malformed corpus state と衝突している。
- `docs/reports/0227-review-try-rollback-first-tranche-wording-stability.md` は review trail として参照されている一方、evidence がまだ `PENDING` の placeholder を含むため、task-close 済みの review artifact としては hygiene gap が残っている。

## Open questions

- `plan/15` の stale bullet を削るか、actualized first tranche state に書き換えるか。
- `docs/reports/0227` を completed review outcome で閉じるか、placeholder report であることを title / scope 上もっと明示するか。

## Suggested next prompt

`plan/15-current-l2-fixture-authoring-template.md` の stale first-tranche mirror bullet と `docs/reports/0227-review-try-rollback-first-tranche-wording-stability.md` の pending review placeholder を整理し、wording-stability chain の mirror hygiene を閉じてください。
