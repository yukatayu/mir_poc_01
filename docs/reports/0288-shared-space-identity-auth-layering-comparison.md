# 0288 — shared-space identity / auth layering comparison

## Objective

shared-space line の次段として、participant carrier / authority / fairness と混線しやすい

- identity core
- auth stack
- admission policy
- display / projection identity

をどこで切るのが自然かを、authoritative room と append-friendly room をまたぐ boundary comparison として整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- raw auth protocol や concrete credential format は固定しない。
- current shared-space line の current first choices
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  - `opaque authority trust`
  を前提に、identity / auth をどこまで room semantics に持ち込まないかを比較する。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `https://blog.yukatayu.tech/blog/sync_language_01/`
- `https://blog.yukatayu.tech/blog/sync_language_02/`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0284-shared-space-fairness-trust-model-comparison.md`

## Actions taken

1. `plan/16` に identity / auth layering section を追加した。
2. membership carrier 一体化 / identity core 分離 / opaque actor handle 化の 3 案を比較した。
3. authoritative game room の join path と append-friendly room の contrast を擬似コード付きで追加した。
4. `plan/12` に identity / auth layering を独立 open problem として追加した。
5. `progress.md` に current first practical candidate を mirror した。
6. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0288-shared-space-identity-auth-layering-comparison.md`
- `docs/reports/0289-review-shared-space-identity-auth-layering-comparison.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:36 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 289 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Reviewer:

- reviewer completion は current interface から取得できなかったため、local review fallback を `0289` に記録した
- local review では substantive finding なし

## What changed in understanding

- participant carrier に principal identity と auth stack を全部埋め込むのは、implementation 初期には単純に見えても、activation / authority / fairness / audit と同じ carrier に漏れやすい。
- current authoritative room の最小 line では、`membership registry = identity core`、`auth stack / admission policy = separate carrier` の cut が最も自然である。
- append-friendly room では opaque actor handle model も強く見えるが、current authoritative room では principal continuity と audit connection を維持した方が説明しやすい。

## Open questions

- `principal_ref` を room audit artifact にどこまで mirror するか。
- `display_ref` を identity core に入れるか、projection layer に完全に外出しするか。
- compile-time over-approximation が room permission / visibility requirement のどこまでを触ってよいか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space admission policy と compile-time over-approximation の接点を、room capability / visibility requirement の line とどう切るか、current shared-space line の残課題として narrow に比較してください。`
