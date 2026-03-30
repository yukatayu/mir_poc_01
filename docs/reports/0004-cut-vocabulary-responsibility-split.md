# Report 0004 — cut vocabulary の責務分担整理

- 作成日時: 2026-03-27T05:17:54.138123Z
- 作成者 / agent: Codex
- 対象範囲: Mir-0 の外側に送った cut vocabulary のうち、`durable_cut` と `barrier` の責務分担を Mir-1 / Mirrorea 境界で整理する
- 触れた decision level: L1

## 1. 目的

Mir-0 を広げずに、`atomic_cut` の外側へ送った cut 関連語彙の置き場所を明文化することを目的とした。
今回は新しい大きな設計を追加せず、`durable_cut` と `barrier` の責務を最小限の語彙整理として固定する。

## 2. 参照した文書

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
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `docs/reports/0002-mir-0-review-findings.md`
- `docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md`
- `specs/05-mirrorea-fabric.md`

## 3. 実施内容

- `code_mapper` を使い、今回参照すべき文書と cut vocabulary の既存境界を確認した。
- `specs/04-mir-core.md` の Mir-0 外項目を更新し、`durable_cut` は Mir-1 側の cut vocabulary 候補、`barrier` は Mir-1 に残す場合でも ordering primitive 候補であることを明記した。
- 同じ箇所で、Mirrorea は `durable_cut` を意味づける側ではなく、後段で定義された stronger finalization requirement を distributed cut realization / routing / audit として実現する責務だけを持つと整理した。
- `specs/10-open-questions.md` を更新し、`durable_cut` と `barrier` の未決定点を Mir semantics 側に、Mirrorea 側の引受境界を Mirrorea / routing 側に分離して記録した。
- `specs/12-decision-register.md` に D-013 を追加し、Mir-0 / Mir-1 / Mirrorea の cut 境界に関する今回の判断を L1 として記録した。
- `specs/11-roadmap-and-workstreams.md` は確認したが、今回の責務整理だけでは新しい工程分解を増やさない方が境界を狭く保てるため、更新しなかった。

## 4. 変更したファイル

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0004-cut-vocabulary-responsibility-split.md`

## 5. 実行したコマンドと出力

1. `git status --short`

```text
 M specs/04-mir-core.md
 M specs/09-invariants-and-constraints.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0001-mir-0-semantic-core-alignment.md
?? docs/reports/0002-mir-0-review-findings.md
?? docs/reports/0003-mir-0-cut-and-vocabulary-clarification.md
?? docs/reports/0004-cut-vocabulary-responsibility-split.md
```

2. `sed -n '1,260p' specs/04-mir-core.md`

```text
- `durable_cut` は Mir-0 に含めない。`atomic_cut` に対し、persistence / durable commit / distributed finalization requirement を追加する Mir-1 側の cut vocabulary 候補として扱う。
- Mirrorea は `durable_cut` を意味づける場所ではない。Mir-1 で定められた stronger finalization requirement を、実ノード・routing・distributed cut realization 上で実現する責務を持つ。
- `barrier` は Mir-0 の cut vocabulary に含めない。Mir-1 に残すとすれば、commit-like primitive ではなく ordering primitive 候補として扱う。
- **未決定**: `barrier` が Mir-1 で独立語彙として本当に必要か、それとも他の ordering / cut 構成に吸収されるか。
- **未決定**: `durable_cut` が要求する persistence / durable commit / distributed finalization guarantees の最小集合。
```

3. `sed -n '1,220p' specs/10-open-questions.md`

```text
2. `durable_cut` は Mir-0 に含めない。Mir-1 側の cut vocabulary 候補として、`atomic_cut` に何を追加するのか
   - persistence,
   - durable commit,
   - distributed finalization,
   のどれを必須要件とするかが未決定である。
3. `barrier` は Mir-0 に含めない。Mir-1 に残すとすれば ordering primitive 候補であり、commit-like primitive として扱うべきではない。未決定なのは、独立語彙として残す価値が本当にあるかどうかである。
13. Mir-1 で定義された later cut vocabulary, especially `durable_cut`, を Mirrorea がどの operational boundary から引き受けるか。現在の最小理解では、意味づけではなく distributed cut realization / route change / audit の実現責務だけを負う。
```

4. `sed -n '1,220p' specs/12-decision-register.md`

```text
| D-013 | durable_cut / barrier の責務分担 | `durable_cut` は `atomic_cut` に persistence / durable commit / distributed finalization requirement を追加する Mir-1 側の cut vocabulary 候補とする。`barrier` は、Mir-1 に残すとしても commit-like primitive ではなく ordering primitive 候補として扱う。Mirrorea は later cut vocabulary の意味づけは行わず、その distributed cut realization / route / audit 側だけを引き受ける | L1 | Mir-0 / Mir-1 / Mirrorea の cut 境界を明確にする |
```

5. `sed -n '1,240p' docs/reports/0004-cut-vocabulary-responsibility-split.md`

```text
# Report 0004 — cut vocabulary responsibility split

- Date: 2026-03-27T05:17:54.138123Z
- Author / agent:
- Scope:
- Decision levels touched:
```

6. `git diff --check`

```text
<no output>
```

7. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 5 numbered report(s).
```

## 6. 根拠と所見

- `atomic_cut` との違いは、Mir-0 では rollback frontier だけを確定するのに対し、`durable_cut` はその先に persistence / durable commit / distributed finalization requirement を追加する語彙候補として扱う点にある。
- ただし、`durable_cut` がその三要素のうち何を最小必須要件とするかは、まだ **未決定** である。
- `barrier` は commit-like primitive として扱わず、残すとしても ordering primitive 候補として整理した。これにより `atomic_cut` と役割が衝突しない。
- `barrier` を Mir-1 に独立語彙として残す必要が本当にあるかは、まだ **未決定** である。
- Mir-0 / Mir-1 / Mirrorea の cut 境界は次のように整理された。
- Mir-0: `atomic_cut` による place-local rollback frontier の確定だけを担う。
- Mir-1: `durable_cut` と、必要なら `barrier` を後段語彙として定義する層である。
- Mirrorea: Mir-1 で定義された later cut vocabulary を、distributed cut realization / route change / audit の実装責務として引き受けるが、意味づけ自体は担わない。

## 7. 今回整理して変わった理解

- 以前は `durable_cut` が Mirrorea 側の語彙に近いようにも読めたが、今回の整理により、意味上は Mir-1 側の cut vocabulary 候補であり、Mirrorea はその operational realization を受け持つだけだと明確になった。
- `barrier` も cut の一種として曖昧に残すのではなく、commit-like primitive ではなく ordering primitive 候補として切り分けたことで、Mir-0 の `atomic_cut` と責務が重ならなくなった。
- その結果、Mir-0 は rollback/finalization の最小核として据え置かれ、後段語彙の責務だけを外側で整理する構図が以前より明確になった。

## 8. 未決定事項

- `durable_cut` にとって最小必須の guarantee が persistence / durable commit / distributed finalization のどこまでを含むかは **未決定**。
- `barrier` を Mir-1 の独立語彙として残すべきか、それとも他の ordering / cut 構成へ吸収すべきかは **未決定**。
- Mir-1 で later cut vocabulary を定義したあと、Mirrorea がどの operational boundary からそれを引き受けるかの詳細は **未決定**。
- `perform` の最終表面構文は今回も扱っておらず、引き続き **未決定** である。

## 9. Suggested next prompt

Mir-0 を広げずに、Mir-1 の `durable_cut` が要求する最小 guarantee set を整理してください。特に `persistence`、`durable commit`、`distributed finalization` のうち何を規範要件に含め、何を Mirrorea の operational realization に送るかを、日本語で `specs/10-open-questions.md` と必要最小限の関連文書に反映してください。
