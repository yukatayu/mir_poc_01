# 報告 0019 — fallback / preference chain canonical normalization law 整理

- 作成日時: 2026-03-31T00:10:00+09:00
- 作成者 / agent: Codex
- 対象範囲: nested fallback から preference chain canonical form への正規化 law、その一意性、単調劣化・再昇格禁止・write-after-expiry・`try` / `atomic_cut` との整合
- 触れた decision level: 既存の Mir-0 境界は維持しつつ、より広い Mir 側の L2 設計提案を 1 段具体化した

## 1. 目的

`fallback` / `preference chain` / `lease` の最小意味論を前提に、nested fallback を canonical preference chain へどう畳むかを最小限の law として明文化する。
今回は canonical form と正規化規則だけを固定し、完全代数、collapse rule、最終 surface syntax、dedicated observation surface は未決定のまま残す。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0017`、`docs/reports/0018` を指定順で読んだ。
- 作業開始時点の git dirty state は空だった。tracked 変更も untracked もなかった。
- `code_mapper` の read-only 要約により、今回の中心が `specs/04`、`specs/10`、`specs/12` であり、`specs/09` は既存 invariant を維持したまま参照すべきことを確認した。
- canonical normalization law は `lease` を独立理論へ昇格させない。`lease` は引き続き option ごとの lifetime guard に留める。
- 正規化は「同じ logical access path / semantic lineage 上の nested fallback を正規形へ畳む law」としてだけ扱い、incompatible branch を自動修復する規則は導入しない。

## 3. 参照文書

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
- `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`

## 4. 実施内容

- `code_mapper` を最初に再利用し、task-start dirty state が空であること、今回の主要対象が `specs/04`、`specs/10`、`specs/12` であること、`specs/09` は constraint 側に留めるべきことを確認した。
- `specs/04-mir-core.md` の L2 節に `canonical normalization law` を追加した。
  - canonical form は単一 access path / semantic lineage に属する有限列
  - 各 option が保持する最小情報
  - nested fallback を left-to-right flattening で `canon(x) ++ canon(y)` に畳む law
  - `A > B > C > D > E` への一意な落とし込み
  - write-after-expiry、`try` / rollback / `atomic_cut` との非干渉
- `specs/10-open-questions.md` では、最小 law までは固定しつつ、redundant option の collapse rule、incompatible branch の rejection phase、完全代数、surface syntax、観測面を未決定として残した。
- `specs/12-decision-register.md` に D-025 を追加し、canonical normalization law を L2 判断として記録した。
- `specs/09-invariants-and-constraints.md` は今回は変更しなかった。既存の「monotone form へ正規化されなければならない」という constraint で十分であり、law の詳細を invariant 側へ持ち上げないためである。
- `reviewer` を最後に起動し、canonical law が Mir-0 を広げていないか、flattening が既存意味論を壊していないかを点検した。
- `reviewer` の最終結果は `no findings` だった。
- commit は 2 本に分ける方針を採った。
  - 1 本目は仕様本文だけを記録する `fallback の canonical law を整理する`（実際の commit は `1731b68`）
  - 2 本目はこの report を記録する `fallback canonical law の作業報告を追加する`

今回実際に変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`

## 5. コマンドと正確な出力

1. `git status --short`

```text
```

2. `rg -n "canonical|normalization|normalize|nested fallback|preference chain|内側|外側|write-after-expiry|再昇格|monotone degradation|guarded option chain|GuardedRef|Reject" specs docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`

```text
specs/11-roadmap-and-workstreams.md:41:- graph normalization
specs/12-decision-register.md:17:| D-011 | Mir-0 boundary | Mir-0 を、event DAG、`place`、最小 effect request operation（ここでは `perform` と表記）、effect / contract、最小 failure space、primitive fallback、local `try` / rollback、place-local `atomic_cut`、linear resource から成る最小 kernel として固定し、`barrier`、`durable_cut`、完全な fallback normalization、`emit`、coroutine semantics、overlay alias detail は後段へ送る | L1 | Workstream A を安定化させつつ、より広い Mir / Mirrorea machinery は決めない |
specs/12-decision-register.md:30:| D-024 | fallback / preference chain / `lease` の最小意味 | Mir-0 は従来どおり primitive fallback と monotone lifetime だけを固定し、preference chain はより広い Mir で nested fallback の正規形として扱う。fallback 付き参照または guarded reference は、1 つの論理的 access path に対する有限の guarded option chain として読み、`lease` は各 option の lifetime guard を指す最小語彙に留める。`lease` 期限切れは monotone degradation の一種であり、同じ semantic lineage で earlier option への再昇格を許さない。`try` rollback や `atomic_cut` はこの degradation order を巻き戻さない。write は current option の contract が許す場合だけ成立し、write-capable option の `lease` が期限切れた後は、後段の write-capable option への explicit fallback がない限り `Reject` とする。surface syntax、完全代数、capability lattice はなお未決定である | L2 | 既存の primitive fallback と monotone lifetime から導かれる最小意味だけを明文化し、Mir-0 を広げずに guarded option chain の解釈を与える |
specs/10-open-questions.md:7:1. Mir-0 の primitive fallback を超えた、preference chain / fallback normalization の最終形式化。
specs/10-open-questions.md:8:   - 現時点の最小理解では、fallback 付き参照または guarded reference は、1 つの論理的 access path に対する有限の guarded option chain として読む。
specs/10-open-questions.md:9:   - preference chain は一次 primitive ではなく nested fallback の正規形であり、`lease` は各 option の lifetime guard を表す最小語彙に留める。
specs/10-open-questions.md:10:   - `lease` の期限切れは monotone degradation の一種であり、同じ semantic lineage で earlier option への再昇格を許さない。
specs/10-open-questions.md:11:   - write は current option の contract が許すときだけ成立し、write-capable option の `lease` が期限切れた後は、後段の write-capable option への explicit fallback がない限り `Reject` として扱う。
specs/03-layer-model.md:17:- ownership / lifetime / monotone degradation
specs/01-charter-and-decision-levels.md:51:- preference chain の正確な形
specs/04-mir-core.md:35:- preference chain と guarded reference は現在も活発な設計論点である。
specs/04-mir-core.md:84:- Mir-0 は、明示的 failure または monotone degradation の後に、次の contract-compatible option へ進む primitive として fallback を含む。
specs/04-mir-core.md:86:- nested fallback / preference chain の完全な normalization は Mir-0 の外にある。
specs/04-mir-core.md:122:- preference chain は一次 primitive ではなく、nested fallback を曖昧さなく書き下した正規形として扱う。`A > B > C` は、「`A` が explicit failure または monotone degradation で使えなければ `B`、さらに使えなければ `C`」という意味に読む。
specs/04-mir-core.md:123:- したがって fallback の推移律は、`A > B` と `B > C` が同じ論理的 access path 上の monotone degradation を表している限り、`A > B > C` への正規化が nested fallback と同じ outcome discipline を保つ、という**最小読解**でだけ扱う。
specs/04-mir-core.md:124:- chain の各段は、それより前の option から見て同等以下の guarantee しか持たない monotone degradation でなければならない。より強い capability への再昇格は許されない。
specs/04-mir-core.md:129:- write-capable option の `lease` が期限切れた場合、その option への write は成功してはならない。後段に write を許す option があり、かつ contract がその degrade を許すなら明示的 fallback でそちらへ進める。そうでなければ結果は explicit `Reject` である。
docs/reports/0018-fallback-preference-chain-and-lease-semantics.md:191:- write-after-expiry は hidden write を許さず、後段の write-capable option への explicit fallback がなければ `Reject` とした。これにより failure の明示性と contract discipline を崩さない。
```

3. `python3 scripts/new_report.py --slug fallback-preference-chain-canonical-normalization`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0019-fallback-preference-chain-canonical-normalization.md
```

4. 変更後の `git status --short`

```text
 M specs/04-mir-core.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0019-fallback-preference-chain-canonical-normalization.md
```

5. `git diff --check`

```text
```

6. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 20 numbered report(s).
```

7. `reviewer`

```text
no findings
```

8. `git commit --no-gpg-sign -m "fallback の canonical law を整理する"`

```text
[main 1731b68] fallback の canonical law を整理する
 3 files changed, 16 insertions(+), 1 deletion(-)
```

9. 1 本目 commit 後の `git status --short`

```text
?? docs/reports/0019-fallback-preference-chain-canonical-normalization.md
```

## 6. 証拠 / 所見

canonical form の位置づけ:

- preference chain は今回も一次 primitive ではなく正規形である。
- canonical form は「候補の優先順」と「各候補に付随する guard / contract / capability」を表し、元の nested syntax の内側 / 外側は保持しない。
- fallback successor は独立 field ではなく、chain 上の次要素として表す。これにより構造を増やさずに canonical form を定義できる。

正規化規則:

- law は left-to-right flattening に限定した。`canon(option) = [option]`、`canon(fallback(x, y)) = canon(x) ++ canon(y)` である。
- ただし適用条件を付け、`x` と `y` が同じ logical access path / semantic lineage を共有し、後段が monotone degradation を保つ場合に限るとした。
- したがって `fallback(fallback(A, B), fallback(C, fallback(D, E)))` と `fallback(A, fallback(B, fallback(C, fallback(D, E))))` は同じ `A > B > C > D > E` に落ちる。
- canonical law は “内側 / 外側” の syntactic nesting を評価意味から切り離し、leftmost viable option だけを観測可能意味に残す。

既存理論との整合:

- monotone degradation と再昇格禁止は law の適用条件に組み込み、後段の option が前段より強くならないことを前提にした。
- write-after-expiry は canonical chain の中で earlier option へ戻さず、後段の write-admissible option へ進むか `Reject` に落ちると整理した。
- `try` / rollback / `atomic_cut` は canonical form 自体を並べ替えず、current `place` の state / frontier だけを制約するものとして位置づけた。これにより既存の rollback / cut semantics と衝突しない。

今回 core に残したもの:

- canonical form は正規形であって一次 primitive ではないこと
- canonical form の最小情報
- left-to-right flattening
- leftmost viable option 読み
- monotone degradation / write-after-expiry / `try` / `atomic_cut` との整合

今回 open question に送ったもの:

- redundant option の collapse rule
- incompatible branch をどの段階で拒否するか
- 完全代数
- 最終 surface syntax
- dedicated observation surface

AMBIGUOUS:

- `capability surface` は今回、既存の write / read の違いを最小限で表す補助語として使った。これを独立した capability lattice に拡張するかは未決定である。
- `incompatible branch` は「同じ access path / lineage を共有しない」「monotone degradation を壊す」場合を総称しているが、静的エラーと実行時 `Reject` のどちらで扱うかはまだ決めていない。

## 7. 今回整理して分かったこと

- 前回 0018 で置いた「preference chain は正規形」という判断は、left-to-right flattening を与えるだけで実用上かなり明瞭になる。
- canonical form の一意性は「syntax tree の形」ではなく「観測可能な候補順序」を正本にすると得られる。
- 逆に、collapse rule や malformed branch の扱いまで今ここで入れると L2 の範囲を超えて理論が重くなる。

## 8. 未解決事項

- redundant option を collapse する law を入れるか。
- incompatible branch を static rejection にするか、dynamic `Reject` にするか。
- `lease` / `GuardedRef` の最終 surface syntax をどうするか。
- complete algebra をどこまで固定するか。
- canonical normalization の結果を event / audit surface へどこまで露出するか。

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、fallback / preference chain canonical normalization law を前提に、incompatible branch をどの段階で拒否するかを整理してください。
目的は、同じ logical access path / semantic lineage を共有しない fallback nest や、monotone degradation を壊す branch を、static rule・dynamic `Reject`・open question のどこへ置くかを最小限の意味論として定めることです。

- Mir-0 を広げないでください。
- `lease` を独立理論へ昇格させないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`、`docs/reports/0019-fallback-preference-chain-canonical-normalization.md` を読んでから始めてください。
- canonical law 自体は変えず、rejection phase だけを整理してください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
