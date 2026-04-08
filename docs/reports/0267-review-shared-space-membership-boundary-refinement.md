# Report 0267 — review shared space membership boundary refinement

- Date: 2026-04-08T01:24:44.873319Z
- Author / agent: Codex
- Scope: `plan/16-shared-space-membership-and-example-boundary.md`、`plan/90-source-traceability.md`、`progress.md`、`docs/reports/0266-shared-space-membership-research-line-refresh.md` の recent docs-only changes を、shared-space membership boundary refinement の観点で review する。focus は semantic consistency と boundary hygiene に限定し、実装・文体・体裁は扱わない。
- Decision levels touched: review only。新しい規範判断は追加しない。

## 1. Objective

shared-space membership boundary refinement に関する recent docs-only changes が、次と整合しているかを確認する。

1. `specs/03-layer-model.md`
2. `specs/05-mirrorea-fabric.md`
3. `specs/10-open-questions.md`
4. 既存の `plan/16-shared-space-membership-and-example-boundary.md` boundary line

substantive finding があれば boundary-hygiene 観点で明示し、なければ no substantive findings を確認する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `plan/00-index.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0265-shared-space-open-questions-snapshot.md`
- `docs/reports/0266-shared-space-membership-research-line-refresh.md`

## 3. Actions taken

1. AGENTS の必読順に沿って repo guidance と normative docs を読み直した。
2. 対象 4 ファイルの diff を確認した。
3. tree-like participant view、session-scoped membership registry、activation visibility compile-time over-approximation、authority / consistency / RNG provider separation の各主張を `specs/03`、`specs/05`、`specs/10` と照合した。
4. 既存 boundary との連続性確認のため、`docs/reports/0265-shared-space-open-questions-snapshot.md` の activation-rule comparison も参照した。

## 4. Files changed

- `docs/reports/0267-review-shared-space-membership-boundary-refinement.md`
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 10:24 JST
```

```text
$ git status --short -- plan/16-shared-space-membership-and-example-boundary.md plan/90-source-traceability.md progress.md docs/reports/0266-shared-space-membership-research-line-refresh.md docs/reports/0267-review-shared-space-membership-boundary-refinement.md
 M plan/16-shared-space-membership-and-example-boundary.md
 M plan/90-source-traceability.md
 M progress.md
?? docs/reports/0266-shared-space-membership-research-line-refresh.md
?? docs/reports/0267-review-shared-space-membership-boundary-refinement.md
```

```text
$ rg -n "all_of|quorum-like|authority-ack|activation" docs/reports/0265-shared-space-open-questions-snapshot.md docs/reports/0264-shared-space-membership-boundary-and-example.md docs/reports/0111-faq-unresolved-issues-current-state.md plan/16-shared-space-membership-and-example-boundary.md
docs/reports/0264-shared-space-membership-boundary-and-example.md:173:   - `all_of` 的 activation
docs/reports/0264-shared-space-membership-boundary-and-example.md:174:   - quorum-like activation
docs/reports/0265-shared-space-open-questions-snapshot.md:97:1. authority-ack で active 化
docs/reports/0265-shared-space-open-questions-snapshot.md:98:2. `all_of` 的な全員観測を要求して active 化
docs/reports/0265-shared-space-open-questions-snapshot.md:99:3. quorum-like activation
docs/reports/0265-shared-space-open-questions-snapshot.md:118:`all_of` や quorum は Mir-1 / Mirrorea 的 aggregate rule と似るが、shared-space の activation まで直ちに同一視しない方が境界が綺麗。
plan/16-shared-space-membership-and-example-boundary.md:282:2. authority-ack / all_of / quorum-like の policy 候補の structural floor
plan/16-shared-space-membership-and-example-boundary.md:657:   - `all_of` 的 activation か
plan/16-shared-space-membership-and-example-boundary.md:658:   - quorum-like activation か
```

## 6. Evidence / findings

### Finding 1

`all_of` / `quorum-like` の再利用が、Mir-1 durable-cut scope rule と shared-space activation policy の境界を再び曖昧にしている。`specs/10-open-questions.md` では `all_of` は cross-place `durable_cut` の profile としてだけ意味づけられており、`quorum-like` は future extension 候補に留まる。一方、今回の追加文では [plan/16] の activation visibility section が `authority-ack / all_of / quorum-like` をそのまま activation-policy 候補として並べており、[docs/reports/0266] でも open question を同じ形で再掲している。既存の boundary line では [docs/reports/0265] が「Mir-1 / Mirrorea 的 aggregate rule と似るが、shared-space の activation まで直ちに同一視しない」と明示していたので、この caveat を落とした現在の書き方は boundary hygiene を弱める。

Relevant anchors:
- `specs/10-open-questions.md:173`-`195`
- `plan/16-shared-space-membership-and-example-boundary.md:281`-`282`
- `docs/reports/0266-shared-space-membership-research-line-refresh.md:131`
- `docs/reports/0265-shared-space-open-questions-snapshot.md:117`-`118`

### Finding 2

activation visibility の compile-time over-approximation section は、「compile-time に言えること」と「runtime にしか決まらないこと」を分ける意図自体は正しいが、例示がその分離を一部崩している。[plan/16] では declaration-fixed なものとして `current active members` を挙げ、`activation_visibility(room) = authority + current active watchers + explicit replicated observers` と書いているが、同じ section の後半では actual active member 集合は runtime に残ると明記している。`current active watchers` は compile-time carrier ではなく runtime membership state なので、この例のままだと static over-approximation が runtime fact を含むように読めてしまう。

Relevant anchors:
- `plan/16-shared-space-membership-and-example-boundary.md:253`-`267`
- `plan/16-shared-space-membership-and-example-boundary.md:285`-`295`

### No substantive finding

次の点は、今回の review scope では specs と整合している。

1. tree-like participant view を derived snapshot / UI / serialization view に留め、source of truth を session-scoped membership registry に置く整理
2. participant carrier と authority placement / consistency mode / RNG provider を別軸に保つ整理
3. `plan/90-source-traceability.md` と `progress.md` の mirror 更新そのもの

これらは `specs/03-layer-model.md` の L2/L3 分離、および `specs/05-mirrorea-fabric.md` の operational realization boundary と矛盾していない。

## 7. Changes in understanding

- tree-like participant view と session-scoped registry の併存整理自体は boundary-safe であることを確認した。
- 問題は tree-like / registry choice ではなく、activation policy comparison で Mir-1 用語を再利用するときの caveat が薄れた点にあると分かった。
- compile-time over-approximation の主張は方向として妥当だが、static carrier の例示は runtime active set を入れない書き方に直した方がよい。

## 8. Open questions

1. shared-space activation comparison で `all_of` / `quorum-like` を使い続けるなら、Mir-1 durable-cut profile との「類似であって同一語彙ではない」断りを `plan/16` 本文にも常設するか。
2. activation visibility の static floor を何で表すか。候補は watcher role、declared observer class、possible member superset などであり、runtime active set を含めない方がよい。

## 9. Suggested next prompt

`plan/16-shared-space-membership-and-example-boundary.md` の activation visibility section を boundary-hygiene 観点で修正してください。特に、Mir-1 の \`all_of\` / \`quorum-like\` を shared-space activation policy と直ちに同一視しない caveat を戻し、compile-time over-approximation の例から runtime-active set を外してください。
