# Report 0058 — review 0056 short re-review

- Date: 2026-04-01
- Author / agent: Codex
- Scope: `docs/reports/0056-current-l2-host-stub-harness.md` と、その mirror / 実装差分に対する short re-review
- Decision levels touched: L2

## 1. Objective

`0056` で整理した current L2 host harness が既存理論を壊していないかを短く再確認し、high-signal finding があれば今回の host plan sidecar work に入る前に最小修正へつなぐ。

## 2. Scope and assumptions

- review 対象は `0056` と、それに対応する host harness / interpreter / test mirror に限定する。
- current L2 semantics 自体は変えず、harness boundary の hidden strengthening や hidden precedence だけを検出対象にする。
- task 開始時点の worktree に既存 dirty state はなく、`git status --short --branch` は `## main...origin/main [ahead 2]` のみだった。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `docs/reports/0056-current-l2-host-stub-harness.md`
- `docs/reports/0057-final-review-current-l2-host-harness.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. reviewer に `0056` 対象の short re-review を依頼した。
2. reviewer から返った finding を確認した。
3. finding を今回の host plan sidecar / loader 作業へ直結する最小修正項目として整理した。
4. resulting fix を current task の follow-up commit に反映した。

## 5. Evidence / outputs / test results

### Files changed

- 新規: `docs/reports/0058-review-0056-short-rereview.md`

review finding を受けた follow-up fix は、仕様本文 / 実装本文 commit `5d5d172` に含めた。

### Commands run and exact outputs

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 2]
```

review result:

```text
1. `FixtureHostStub` is not just a neutral stub boundary right now; it silently supplies permissive host semantics when the plan is incomplete.
2. The rule matcher has silent shadowing semantics of its own: fields are wildcardable, and the first matching rule wins, but overlapping rules are neither specified nor rejected.
```

### Findings

- High: host harness が未被覆 oracle call を permissive default で進めており、verification-only boundary より強い hidden host semantics を持っていた。
- High: wildcard rule の first-match に対する precedence / overlap policy が未規定のまま実装に埋め込まれており、silent shadowing を起こし得た。
- これを受けて current task では、次を最小修正方針とした。
  - host plan rule は runtime で実際に発生する oracle call を明示的に被覆しなければならない
  - 未被覆 call は permissive success default ではなく invalid host plan として扱う
  - overlap する rule は loader / harness が reject する
- 上の follow-up fix を含む仕様本文 / 実装本文の commit hash は `5d5d172` である。report 自身の commit hash は self-reference の都合で本文に固定しない。

## 6. What changed in understanding

- current L2 host harness では、declarative plan を入れた時点で「何が host override で、何が interpreter の structural rule か」を machine-check できるようにする必要がある。そのため、未被覆 call の permissive default や overlap precedence を hidden policy のまま残せないことが明確になった。

## 7. Open questions

- uncovered oracle call を runtime 前の richer preflight coverage analysis で完全検出するかどうかは **未決定**。
- sidecar schema の field naming や detached serialization の固定は **未決定**。

## 8. Suggested next prompt

`0058-review-0056-short-rereview.md` の finding を踏まえ、current L2 host plan schema / sidecar loader を追加しつつ、未被覆 oracle call を invalid host plan として扱い、overlap rule を reject する方針を docs / code / tests で揃えてください。
