# Report 0044 — review non admissible reason audit metadata shape

- Date: 2026-03-31T09:29:45.595213Z
- Author / agent: Codex
- Scope: `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/examples/00-representative-mir-programs.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md` に対する maintainer / spec-editor 観点の review
- Decision levels touched: L2 review only

## 1. Objective

current L2 における non-admissible reason audit metadata の整理について、次の 3 点を review する。

1. 最小必須を `option ref` と `subreason` に置き、`reason kind` / explicit `request ref` field を未固定とした整理が既存理論と矛盾しないか。
2. event surface を増やさない方針と矛盾しないか。
3. capability mismatch を premature に taxonomy へ編入していないか。

spec 本文は編集せず、新しい review report だけを追加する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`

## 3. Actions taken

1. `README.md`、`Documentation.md`、`specs/00`〜`03`、`specs/09` を読み直し、review 観点に関係する L0 / L1 / L2 の不変条件を確認した。
2. 対象 5 ファイルについて、worktree 差分・index 差分・現行本文を確認した。
3. review 観点を `option ref` / `subreason` の最小性、event surface の非拡張、capability mismatch の taxonomy 非編入に絞って照合した。
4. findings の有無を maintainer / spec-editor 観点で整理した。

## 4. Files changed

- Changed:
  - `docs/reports/0044-review-non-admissible-reason-audit-metadata-shape.md`
- Checked but not changed:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
- Task-start dirty state:
  - review 開始時点の worktree では、対象 5 ファイルのうち未コミットだったのは本 review report の新規追加前提だけであり、spec 本文側の変更はすでに commit 済みだった。

## 5. Commands run and exact outputs

- `git status --short --branch`
  - `## main...origin/main [ahead 8]`
  - `A  docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
- `git diff -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
  - `[no output]`
- `git diff --cached -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
  - `[no output]`
- `git status --short --branch`
  - `## main...origin/main [ahead 10]`
  - `?? docs/reports/0044-review-non-admissible-reason-audit-metadata-shape.md`

## 6. Evidence / findings

## Findings

- findings なし。

### 6.1 `option ref` / `subreason` 最小必須の整理

- `specs/examples/01-current-l2-surface-syntax-candidates.md` は、metadata shape を「request に結び付いた option-local miss record」として説明し、その最小読解を `option ref` と `subreason` に絞っている。`reason kind` は metadata channel から読めればよく、独立 field を必須にしないとしているため、既存の「event surface を増やさず request-level outcome を優先する」方針と整合している。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` でも、`reason kind` / explicit `request ref` field を「未固定」「将来の detached serialization で必要になる可能性はあるが current L2 では要件化しない」と揃えており、`request-local attachment` を conceptual requirement に留める current L2 reading と矛盾しない。
- `specs/examples/00-representative-mir-programs.md` の E3 比較用 variant と E6 も、最小 metadata だけで説明できる形に留めており、例示文書側から追加要求を逆流させていない。

### 6.2 event surface 非拡張との整合

- `specs/examples/01-current-l2-surface-syntax-candidates.md` と `specs/10-open-questions.md` はいずれも、`admit` miss / `lease` expiry を dedicated skip event に上げないと明言している。
- その上で `specs/examples/00-representative-mir-programs.md` E3 比較用 variant は option miss を audit metadata に留め、E6 は request-level `Reject` を event としつつ `lease-expired` を metadata に留めている。よって individual option miss を event 化していない。
- `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md` も同じ整理を繰り返しており、review 観点 2) の event surface 非拡張と矛盾しない。

### 6.3 capability mismatch の taxonomy 非編入

- E6 では `readonly` 側の request / capability mismatch を narrative explanation に留め、最小 metadata shape に持ち上げていない。
- `specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` はいずれも capability mismatch の formal subreason 化を **未決定** として残している。
- したがって、今回の整理は `admit-miss` / `lease-expired` の最小分類だけを current L2 に残し、capability mismatch を premature に taxonomy へ編入していない。

### 6.4 review 上の注意

- 今回の依頼文では「最終未コミット差分」とされたが、review 実行時点で対象 spec 本文はすでに commit 済みだった。したがって本 review は uncommitted patch review というより、現行本文と主 report 0043 の整合 review として行った。
- この前提は scope のずれであり、spec 内容の defect ではない。

## 7. Changes in understanding

current L2 の non-admissible reason 整理は、field 名や serialization を固定するよりも先に、「request-level outcome を event surface に残し、option-level miss を request-local metadata に留める」という層分けが中核になっていることを再確認した。今回の最小 shape はその層分けを壊していない。

## 8. Open questions

- detached serialization や cross-trace correlation を扱う段階で、`request ref` 相当を explicit field に上げる必要があるか。
- capability mismatch を将来 formal subreason に入れる場合でも、event surface を増やさず metadata family の内部に留められるか。
- dedicated skip event を将来導入する場合、current L2 の request-level outcome 優先方針をどこまで保てるか。

## 9. Suggested next prompt

current L2 の representative examples を前提に、capability mismatch を `non-admissible reason` taxonomy の formal subreason 候補として扱う場合でも、event surface を増やさず audit metadata family に留められるかを比較整理してください。E6 の readability と current L2 の最小 metadata shape を壊さないことだけに絞って検討してください。
