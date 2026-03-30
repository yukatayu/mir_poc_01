# 報告 0020 — fallback / preference chain incompatible branch rejection phase 整理

- 作成日時: 2026-03-31T00:42:06+09:00
- 作成者 / agent: Codex
- 対象範囲: canonical normalization law を前提にした incompatible branch の最小分類、static rejection と dynamic `Reject` の境界、未決定として残す点
- 触れた decision level: 既存の Mir-0 境界は維持しつつ、より広い Mir 側の L2 設計提案を 1 段具体化した

## 1. 目的

fallback / preference chain canonical normalization law はすでに L2 として置かれているため、今回はその law 自体を変えず、incompatible branch をどの段階で拒否するかだけを最小限で定める。
同じ logical access path / semantic lineage を共有しない nested fallback や、monotone degradation を壊す branch を、static rejection・dynamic `Reject`・未決定のどこへ置くかを切り分ける。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0018`、`docs/reports/0019` を指定順で読んだ。
- 作業開始時点の git dirty state は空だった。tracked 変更・untracked 変更ともになかった。
- `code_mapper` の read-only 要約により、今回の中心が `specs/04`、`specs/10`、`specs/12` であり、`specs/09` は invariant の強度を変えない補助参照に留めるべきことを確認した。
- 今回は canonical law 自体を変更しない。left-to-right flattening、monotone degradation、write-after-expiry、`try` / `atomic_cut` の既存整理を前提にする。
- `lease` を独立理論へ昇格させず、新しい failure class や巨大な capability lattice も導入しない。

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
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`

## 4. 実施内容

- `code_mapper` を最初に起動し、task-start dirty state が空であること、今回の主要対象が `specs/04`、`specs/10`、`specs/12` であること、`specs/09` は既存 invariant の参照に留めるべきことを確認した。
- `specs/04-mir-core.md` に `incompatible branch の rejection phase（L2）` 節を追加した。
  - incompatible branch の最小分類
  - static rejection を置く条件
  - dynamic `Reject` に残す条件
  - `try` / rollback / `atomic_cut` との非干渉
  - なお未決定な static 証拠要件
- `specs/10-open-questions.md` では、従来「static capability rule」とだけ曖昧に残っていた点を分解し、最小分類までは固定しつつ、static 証拠・mixed case・surface syntax などを未決定として残した。
- `specs/12-decision-register.md` に D-026 を追加し、canonical law を変えずに malformed branch と runtime failure を分離する L2 判断として記録した。
- `specs/09-invariants-and-constraints.md` は変更しなかった。今回は invariant を強化する仕事ではなく、canonical law の rejection phase を L2 で補う仕事だからである。
- `reviewer` は最後に起動し、結果は `no findings` だった。
- commit は複数本に分け、仕様本文と report を履歴上でも分離する。
  - 1 本目は仕様本文を記録する `fallback incompatible branch の rejection phase を整理する`（実際の commit は `76916f7`）
  - 2 本目はこの report を記録する `fallback incompatible branch rejection phase の作業報告を追加する`
  - ただし 2 本目の commit はこの report 自身を含むため、self-reference の制約上、本文内にその最終 hash を先回りして確定値で埋め込むことはできない。実 hash は final response と `git log` で補完する。

今回実際に変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`

## 5. コマンドと正確な出力

1. 作業開始時点の `git status --short`

```text
```

2. `python3 scripts/new_report.py --slug fallback-preference-chain-incompatible-branch-rejection-phase`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md
```

3. 仕様変更後の `git status --short`

```text
 M specs/04-mir-core.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md
```

4. `git diff -- specs/04-mir-core.md specs/10-open-questions.md specs/12-decision-register.md`

```diff
diff --git a/specs/04-mir-core.md b/specs/04-mir-core.md
index 658cddb..d9c051b 100644
--- a/specs/04-mir-core.md
+++ b/specs/04-mir-core.md
@@ -142,6 +142,20 @@ Mir-0 に残るのは従来どおり primitive fallback と monotone lifetime 
 - write-after-expiry も同じ law に従う。先頭の write-capable option が期限切れたとき、canonical form は earlier option へ戻らず、次の write-admissible option へ明示的に進むか、該当候補がなければ `Reject` になる。
 - `try` / rollback / `atomic_cut` は canonical form 自体を並べ替えない。これらは current `place` の state や rollback frontier を制約するが、canonical chain が表す優先順と degradation order は保存される。
 
+### incompatible branch の rejection phase（L2）
+
+- incompatible branch とは、canonical chain の同一列に載せる前提を満たさない nested fallback branch を指す最小補助語である。少なくとも次を含む。
+  - same logical access path / semantic lineage を共有しない branch
+  - 後段が前段より強い guarantee や capability を要求し、monotone degradation を壊す branch
+  - 後段 option が前段 option の contract-compatible fallback successor になれないことが、declared contract surface / capability surface だけで分かる branch
+- 上の incompatibility が declared access target、documented logical access path / semantic lineage 関係、declared contract surface、declared capability surface だけで判定できるなら、その nested fallback は evaluation に入る前に static rejection されるべきである。これは canonical law の失敗ではなく、「その branch は canonical form を持つ well-formed preference chain ではない」という判定である。
+- dynamic `Reject` は、canonical chain 自体は well-formed だが、current evaluation で admissible option が尽きた場合の明示的 outcome として残す。`lease` 期限切れ、`require` 不成立、explicit failure、write-after-expiry 後に後段の write-admissible option が存在しない場合はこの側に属する。
+- したがって static rejection は malformed fallback nest を弾くための前段判定であり、dynamic `Reject` は well-formed chain の runtime failure を表す。両者を混同して hidden repair や hidden backtracking を導入してはならない。
+- `try` / rollback / `atomic_cut` は incompatible branch の分類そのものを変えない。static rejection される branch は rollback や cut の前後を問わず canonical chain に入らず、dynamic `Reject` は従来どおり explicit failure outcome として既存の rollback / cut discipline に従う。
+- **未決定**: same logical access path / semantic lineage を static に証明する最小証拠を何にするか。
+- **未決定**: contract-compatible fallback successor を static に判定する最小規則をどこまで固定するか。特に current `require` failure のような runtime admission と、branch 自体の incompatibility をどこで分けるか。
+- **未決定**: capability surface の詳細を増やさずに判定できない mixed case を、将来 static rejection に寄せるか dynamic `Reject` に残すか。
```

5. `reviewer`

```text
no findings
```

6. `git diff --check`

```text
```

7. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 21 numbered report(s).
```

8. commit 記録

```text
[main 76916f7] fallback incompatible branch の rejection phase を整理する
 3 files changed, 20 insertions(+), 2 deletions(-)

2 本目の commit message:
fallback incompatible branch rejection phase の作業報告を追加する

注記:
この report 自身を含む最終 commit hash は self-reference を避けるため本文には固定値として埋め込めない。実 hash は final response と `git log` で確認する。
```

9. 1 本目 commit 後の `git status --short`

```text
?? docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md
```

## 6. 証拠 / 所見

今回の最小分類:

- incompatible branch は「canonical chain 自体を作れない malformed branch」としてだけ定義した。
- 具体的には、same logical access path / semantic lineage を共有しない場合、monotone degradation を壊す場合、contract-compatible fallback successor になれないことが static に分かる場合を含めた。
- ここでは capability lattice を拡張せず、既存の contract surface / capability surface という最小語彙だけで書ける範囲に留めた。

static rejection に寄せたもの:

- canonical chain の前提を static に満たさない nested fallback。
- same path / same lineage を共有しない branch。
- 前段より強い guarantee / capability を要求するため monotone degradation を壊す branch。
- 後段 option が前段 option の contract-compatible fallback successor になれないことが declared information だけで分かる branch。

dynamic `Reject` に残したもの:

- canonical chain 自体は well-formed で、実行時評価で admissible option が尽きたケース。
- `lease` 期限切れ。
- `require` 不成立。
- explicit failure。
- write-after-expiry 後に後段 write-admissible option が存在しないケース。

canonical law との整合:

- left-to-right flattening 自体は変更していない。
- 変えたのは「その law を適用できる branch と、そもそも canonical form を持たない branch の境界」だけである。
- static rejection を前段に置くことで、incompatible branch を runtime `Reject` でごまかす hidden repair を禁止した。
- dynamic `Reject` は引き続き explicit failure outcome であり、`try` / rollback / `atomic_cut` の既存意味論を壊さない。

AMBIGUOUS:

- same logical access path / semantic lineage を static に証明する最小証拠は、原文側にまだ存在しない。
- contract-compatible fallback successor の static 判定も、current `require` failure と branch incompatibility の境界がまだ粗い。
- capability surface は既存の write/read 差分を支える最小補助語として使っているが、どこまで static rejection に利用できるかは未決定である。

## 7. 今回整理して分かったこと

- canonical law の未決定部分をすべて runtime `Reject` に押し込むと、malformed branch と normal runtime failure が混ざり、hidden repair 禁止と相性が悪い。
- 一方で static rejection を広げすぎると、lineage 証拠や capability 証明まで事実化してしまう。したがって今回は、declared information だけで incompatibility が分かる範囲に限定するのが最小だった。
- write-after-expiry を dynamic `Reject` 側に残すことで、既存の `lease` / monotone degradation / `Reject` の整理を崩さずに rejection phase だけを明瞭化できた。

## 8. 未解決事項

- same logical access path / semantic lineage を static に証明する最小証拠を何にするか。
- contract-compatible fallback successor を static に判定する最小規則をどこまで固定するか。
- current `require` failure のような runtime admission と branch incompatibility の境界をどこで切るか。
- capability surface の詳細を増やさずに扱えない mixed case を static rejection と dynamic `Reject` のどちらへ寄せるか。
- `lease` / `GuardedRef` の最終 surface syntax。
- preference chain の完全代数、redundant option の collapse rule、dedicated observation surface。

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、fallback / preference chain canonical normalization law と incompatible branch rejection phase を前提に、same logical access path / semantic lineage を static に証明する最小証拠を整理してください。
目的は、incompatible branch を static rejection に寄せるために必要な最小の declared information が何かを定め、lineage 証拠不足を open question としてどこまで残すべきかを明確にすることです。

- Mir-0 を広げないでください。
- canonical law と rejection phase 自体は変更しないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0019-fallback-preference-chain-canonical-normalization.md`、`docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md` を読んでから始めてください。
- 新しい大きな capability theory は導入しないでください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
