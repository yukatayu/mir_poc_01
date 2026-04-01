# Report 0050 — review 0049 short rereview

- Date: 2026-04-01T01:52:42.509690Z
- Author / agent: Codex
- Scope: `docs/reports/0049-current-l2-step-semantics.md` と、その本文が参照する step semantics 関連仕様更新の短い再レビュー
- Decision levels touched: L2

## 1. Objective

`0049` と `specs/examples/03-current-l2-evaluation-state-schema.md` / `specs/examples/04-current-l2-step-semantics.md` を中心に、次だけを短く再確認する。

- 既存理論との不整合
- current L2 を越えた決め打ち
- state / schema / step semantics 間の矛盾

## 2. Inputs consulted

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
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `docs/reports/0049-current-l2-step-semantics.md`

## 3. Actions taken

1. `AGENTS.md` の読書順に従って基礎文書を再確認した。
2. `0049` が参照する step semantics 関連更新面を切り出した。
3. `Mir Core` の contract / fallback / `atomic_cut` 記述と、examples / state schema / step semantics を行番号付きで突き合わせた。
4. findings を既存理論不整合、L2 越え、state/schema/step 矛盾の 3 観点で絞った。

## 4. Files changed

- 新規: `docs/reports/0050-review-0049-short-rereview.md`

## 5. Commands run and exact outputs

```bash
sed -n '1,260p' docs/reports/0049-current-l2-step-semantics.md
sed -n '50,90p' specs/04-mir-core.md
sed -n '1,160p' specs/examples/00-representative-mir-programs.md
nl -ba specs/examples/03-current-l2-evaluation-state-schema.md | sed -n '190,240p'
nl -ba specs/examples/04-current-l2-step-semantics.md | sed -n '110,190p'
nl -ba specs/examples/04-current-l2-step-semantics.md | sed -n '240,275p'
```

```text
主要確認点:
- Mir Core は contract が admission と outcome space の両方を制約すると述べる。
- E1 は `ensure` 付き `PerformOn` を representative example に含む。
- evaluation state schema は `current_request.require` と `current_request.ensure` を保持する。
- step semantics の `PerformOn` / `PerformVia` 規則は `require` と option admissibility は扱うが、`ensure` の評価点を規定していない。
- evaluation state schema の E3 minimal state は `place_store` を含めない。
- step semantics の state 更新表は admissible `PerformVia` success で `place_store` を mutate すると述べる一方、`PerformVia` success 規則と E3 walkthrough 本文ではその mutation が落ちている。
```

## 6. Evidence / findings

1. High: `ensure` が state/schema には載っているのに step semantics から落ちています。`Mir Core` は contract が admission だけでなく outcome space も制約するとし、representative example E1 も `ensure` 付きです ([specs/04-mir-core.md](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L64), [specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L47), [specs/examples/03-current-l2-evaluation-state-schema.md](/home/yukatayu/dev/mir_poc_01/specs/examples/03-current-l2-evaluation-state-schema.md#L84), [specs/examples/04-current-l2-step-semantics.md](/home/yukatayu/dev/mir_poc_01/specs/examples/04-current-l2-step-semantics.md#L120)). ところが `PerformOn` / `PerformVia` の規則は `require` と option admissibility しか評価しておらず、`ensure` failure をどこでどう outcome 化するかが未記述です。これは existing theory の contract 読みを無言で弱めています。

2. Medium: `PerformVia` 成功時の `place_store` 更新について、state schema と step semantics 本文が食い違っています。E3 variant は `write_profile via profile_ref` ですが、evaluation state schema の E3 minimal state は `place_store` を不要とし ([specs/examples/00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L206), [specs/examples/03-current-l2-evaluation-state-schema.md](/home/yukatayu/dev/mir_poc_01/specs/examples/03-current-l2-evaluation-state-schema.md#L215))、一方で step semantics の state 更新表は admissible `PerformVia` success で `place_store` を mutate すると述べます ([specs/examples/04-current-l2-step-semantics.md](/home/yukatayu/dev/mir_poc_01/specs/examples/04-current-l2-step-semantics.md#L53))。さらに実際の `PerformVia` success 規則と E3 walkthrough からはその mutation が抜けています ([specs/examples/04-current-l2-step-semantics.md](/home/yukatayu/dev/mir_poc_01/specs/examples/04-current-l2-step-semantics.md#L153), [specs/examples/04-current-l2-step-semantics.md](/home/yukatayu/dev/mir_poc_01/specs/examples/04-current-l2-step-semantics.md#L252))。現在のままだと、「`PerformVia` 成功は stateful write を起こす」のか「E3/E6 では pure outcome だけ見る」のかが文書間で割れています。

## 7. Changes in understanding

- 今回の更新は L2 を大きく越えてはいないが、contract の outcome-side semantics と `PerformVia` の state mutation だけは companion 文書間の接続がまだ甘い。

## 8. Open questions

- `ensure` failure を current L2 の最小 step semantics に入れるなら、`explicit_failure` へ畳むのか、oracle 側 outcome として分けるのか。
- E3 variant の目的が「chain 選択だけの説明」なのか「write 成功まで含む説明」なのかを state schema 側で明記するか。

## 9. Suggested next prompt

`specs/examples/03-current-l2-evaluation-state-schema.md` と `specs/examples/04-current-l2-step-semantics.md` を最小修正し、1) `ensure` の評価点と failure reading、2) admissible `PerformVia` success 時の `place_store` mutation の要否を current L2 として揃えてください。
