# Report 0239 — current L2 stage 1 parser guard slot handoff

## 1. Title and identifier

- Report 0239
- current L2 stage 1 parser guard slot handoff

## 2. Objective

`specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md` で固定した
stage 1 declaration-side guard slot の opaque attached slot judgment を前提に、
その slot を actual parser / checker handoff へ送るとき、
parser-side carrier と current parser-free AST fixture schema をどのように最小接続するのが自然かを docs-only で比較し、
current next narrow step を確定する。

## 3. Scope and assumptions

- current L2 の core semantics は変更しない。
- final parser grammar、final parser API、final AST schema は固定しない。
- current parser-free AST fixture schema は compatibility anchor として維持する。
- `plan/` は今回更新対象である。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `plan/00-index.md`
- `plan/06-surface-notation-status.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 5. Actions taken

1. current parser-free AST fixture schema の `OptionDecl` carrier と representative fixture を確認し、stage 1 で declaration-side guard slot を直接 `lease` field に同一視する pressure があるかを見直した。
2. 次の 3 案を比較した。
   - parser output を current fixture schema へ直接合わせる
   - parser-side opaque slot carrier を保持し、fixture schema へ thin lowering する
   - current fixture schema 自体を opaque slot carrier へ widen する
3. `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md` を追加し、current judgment を
   parser-side opaque slot carrier と fixture-side compatibility anchor を分ける cut として明文化した。
4. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` を mirror 更新した。
5. `plan/90-source-traceability.md` に今回の source addendum を追加した。

## 6. Evidence / outputs / test results

### Commands run

- `git status --short --branch`
- `date '+%Y-%m-%d %H:%M JST'`
- `python3 scripts/validate_docs.py`
- `git diff --check`

### 読み取り evidence

- current parser-free AST fixture schema では `OptionDecl` が `lease` と `admit` を分けて持っている。
- `e4-malformed-lineage` は `lease = "live"` / `admit = null` の代表であり、declaration structural floor と request / admissibility cluster が current fixture corpus 上でも分かれている。
- `e7-write-fallback-after-expiry` は `lease = "expired"` / `lease = "live"` の coexistence を示しており、current `lease` slot は companion carrier として十分 narrow である。
- `e3-option-admit-chain` は `admit` predicate node を持っており、stage 1 handoff に request / admissibility cluster を混ぜると先食いになることを再確認した。

### Command outputs

- `git status --short --branch`
  - `## main...origin/main`
  - modified / untracked files は今回 task の mirror 追加だけだった。
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-06 06:52 JST`

## 7. What changed in understanding

- stage 1 declaration-side guard slot の actual handoff は、
  parser-side opaque slot carrier と current parser-free AST fixture schema を同一物として扱わない方が自然である。
- current repo phase では、fixture-side `OptionDecl.lease` は final parser-side carrier ではなく、
  current smoke / checker bridge の compatibility anchor として読むのが最も安定する。
- したがって current next narrow step は、
  parser-side opaque slot carrier から `OptionDecl.lease` への thin lowering bridge を docs-only で正本化し、
  parser-side exact carrier 名と actual bridge API surface は次 task に残す、という cut になる。

## 8. Open questions

- parser-side opaque slot の exact carrier 名は何にするか。
- parser-side carrier が raw text / token slice / opaque leaf のどれを持つべきか。
- thin lowering bridge の actual private API surface をどこまで narrow に固定するか。
- later stage で `OptionDecl.lease` を widening する必要が本当に出るか。

## 9. Suggested next prompt

`specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md` を前提に、stage 1 parser-side opaque slot carrier の exact naming candidate と thin lowering bridge の private API surface を narrow に比較し、actual parser spike へ送る前にどこまで docs-only で固定するかを整理してください。

## 10. Files changed

- Added: `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`

## 11. plan/ update note

- `plan/` 更新あり: `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`
