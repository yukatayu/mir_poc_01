# 報告 0021 — fallback / preference chain static evidence floor 整理

- 作成日時: 2026-03-31T01:17:52+09:00
- 作成者 / agent: Codex
- 対象範囲: same logical access path / semantic lineage の最小 static 証拠、contract-compatible fallback successor の最小静的判定、underdeclared case の扱い
- 触れた decision level: 既存の Mir-0 境界と L2 の canonical law / rejection phase を維持したまま、その前提になる最小 static evidence だけを 1 段具体化した

## 1. 目的

fallback / preference chain canonical normalization law と incompatible branch rejection phase を前提に、same logical access path / semantic lineage を static に扱うための最小 declared information と、contract-compatible fallback successor を static に残すための最小比較対象を整理する。
新しい大きな capability theory や完全な型体系は導入せず、canonical law と rejection phase を支える最小証拠だけを定める。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0018`、`docs/reports/0019`、`docs/reports/0020` を指定順で読んだ。
- 作業開始時点の git dirty state は空だった。tracked 変更・untracked 変更ともになかった。
- `code_mapper` の read-only 要約により、今回の中心が `specs/04`、`specs/10`、`specs/12` であり、`specs/09` は invariant の強度を変えない補助参照に留めるべきことを確認した。
- 今回は canonical law 自体も incompatible branch rejection phase 自体も変更しない。left-to-right flattening、monotone degradation、static rejection と dynamic `Reject` の境界は既存整理を前提にする。
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
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`

## 4. 実施内容

- `code_mapper` を最初に起動し、task-start dirty state が空であること、今回の中心が `specs/04`、`specs/10`、`specs/12` であること、強めすぎると危ない箇所が `lease` 昇格・invariant 化・static rejection 拡張であることを確認した。
- `specs/04-mir-core.md` の `incompatible branch の rejection phase（L2）` の直後に、`static evidence floor（L2）` 節を追加した。
  - same logical access path / semantic lineage の最小 declared information
  - `declared access target` だけでは足りない理由
  - `documented lineage annotation` を要求する current L2 の読み
  - contract-compatible fallback successor を static に残すための最小比較対象
  - static rejection に寄せる explicit mismatch
  - underdeclared case を hidden acceptance せず未決定に残す扱い
- `specs/10-open-questions.md` では、今回固定した最小証拠と最小比較規則を bullet として追加し、なお未決定な surface form、contract 比較の粒度、mixed case の扱いを切り分けた。
- `specs/12-decision-register.md` に D-027 を追加し、canonical law と rejection phase を支える最小 static evidence floor を L2 判断として記録した。
- `specs/09-invariants-and-constraints.md` は変更しなかった。今回の整理は invariant ではなく L2 の補助整理に留めるべきだからである。
- `reviewer` は最後に起動し、初回は D-027 の要約が 04/10 より弱い点を指摘した。`underdeclared case` について「hidden に same-path 扱いして canonical chain へ入れない」「current L2 では static acceptance しない」を D-027 に追記した後、再確認では `no findings` だった。
- commit は仕様本文と report を分けて記録する。
  - 1 本目は仕様本文を記録する `fallback static evidence floor を整理する`（実際の commit は `6abd448`）
  - 2 本目はこの report を記録する `fallback static evidence floor の作業報告を追加する`
  - ただし 2 本目の commit はこの report 自身を含むため、self-reference の制約上、本文内にその最終 hash を先回りして確定値で埋め込むことはできない。実 hash は final response と `git log` で補完する。

今回実際に変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`

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
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`

## 5. コマンドと正確な出力

1. 作業開始時点の `git status --short`

```text
```

2. `python3 scripts/new_report.py --slug fallback-preference-chain-static-evidence-floor`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0021-fallback-preference-chain-static-evidence-floor.md
```

3. 仕様変更後の `git status --short`

```text
 M specs/04-mir-core.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0021-fallback-preference-chain-static-evidence-floor.md
```

4. `git diff -- specs/04-mir-core.md specs/10-open-questions.md specs/12-decision-register.md`

```diff
diff --git a/specs/04-mir-core.md b/specs/04-mir-core.md
index e0fc659..c945b2e 100644
--- a/specs/04-mir-core.md
+++ b/specs/04-mir-core.md
@@ -152,9 +152,22 @@ Mir-0 に残るのは従来どおり primitive fallback と monotone lifetime 
 - dynamic `Reject` は、canonical chain 自体は well-formed だが、current evaluation で admissible option が尽きた場合の明示的 outcome として残す。`lease` 期限切れ、`require` 不成立、explicit failure、write-after-expiry 後に後段の write-admissible option が存在しない場合はこの側に属する。
 - したがって static rejection は malformed fallback nest を弾くための前段判定であり、dynamic `Reject` は well-formed chain の runtime failure を表す。両者を混同して hidden repair や hidden backtracking を導入してはならない。
 - `try` / rollback / `atomic_cut` は incompatible branch の分類そのものを変えない。static rejection される branch は rollback や cut の前後を問わず canonical chain に入らず、dynamic `Reject` は従来どおり explicit failure outcome として既存の rollback / cut discipline に従う。
+### static evidence floor（L2）
+- same logical access path / semantic lineage を static に扱うための最小 declared information は、各 option の `declared access target` と、fallback successor が前段 option と同じ semantic lineage を継続することを示す `documented lineage annotation` である。
+- `declared access target` だけでは足りない。同じ target 風に見える option でも、same logical access path / semantic lineage を共有しているとは限らないためである。したがって current L2 では、static same-lineage 判定には `documented lineage annotation` を併せて要求する。
+- contract-compatible fallback successor を static に扱うための最小比較対象は、前段 option と後段 option の `declared contract surface` と `declared capability surface` である。current L2 の最小読解では、後段 option が前段 option より強い capability を要求せず、かつ declared contract surface に successor 使用を明示的に打ち消す矛盾がないときだけ、static な successor 候補として残してよい。
+- したがって current L2 で static rejection に寄せられるのは、少なくとも次である。
+  - `declared access target` が一致しない場合
+  - `documented lineage annotation` が same-lineage / same-path を否定する場合
+  - `declared capability surface` の比較だけで後段が前段より強い capability を要求すると分かる場合
+  - `declared contract surface` の比較だけで後段 option が fallback successor として明示的に矛盾すると分かる場合
+- 逆に、宣言情報が不足していて same-lineage も incompatibility も証明できない場合、その branch を hidden に same-path と見なして canonical chain へ入れてはならない。current L2 では、この underdeclared case を surface-level static error にするか、別の elaboration obligation にするかまでは固定しない。
+- dynamic `Reject` に残るのは、上の最小 declared information だけでは incompatibility が証明されず、canonical chain 自体は well-formed だが current evaluation で admissible option が尽きる場合である。`lease` 期限切れ、current `require` failure、explicit failure、write-after-expiry 後の後段不在は従来どおりこの側に属する。
```

5. `reviewer`

```text
初回:
1. 中: [specs/12-decision-register.md:33](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L33) の D-027 が、underdeclared case を「未決定に残す」とだけ要約しており、[specs/04-mir-core.md:166](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L166) と [specs/10-open-questions.md:20](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L20) にある「hidden に same-path 扱いして canonical chain へ入れてはならない」「current L2 では static acceptance しない」という制約を決定登録で落としています。これだと decision register だけを参照した読者に、underdeclared branch の canonicalization 可否まで未決定になったように読まれるため、rejection phase の境界が 04/10 と 12 でずれます。

修正後:
no findings
```

6. `git diff --check`

```text
```

7. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 22 numbered report(s).
```

8. commit 記録

```text
[main 6abd448] fallback static evidence floor を整理する
 3 files changed, 23 insertions(+), 5 deletions(-)

2 本目の commit message:
fallback static evidence floor の作業報告を追加する

注記:
この report 自身を含む最終 commit hash は self-reference を避けるため本文には固定値として埋め込めない。実 hash は final response と `git log` で確認する。
```

9. 1 本目 commit 後の `git status --short`

```text
?? docs/reports/0021-fallback-preference-chain-static-evidence-floor.md
```

## 6. 証拠 / 所見

今回固定した最小 static evidence:

- same logical access path / semantic lineage の static 証拠は、`declared access target` と `documented lineage annotation` の組に限定した。
- `declared access target` だけでは same-lineage を言えないと明記した。これにより target 同一視だけで hidden repair 的に canonical chain へ入れる余地を塞いだ。
- ただし `documented lineage annotation` 自体の具体 syntax や証拠形式は固定していない。ここは最小証拠の存在だけを定め、形式は未決定に残した。

contract-compatible fallback successor の最小静的判定:

- 前段 option と後段 option の `declared contract surface` と `declared capability surface` を比較対象にした。
- current L2 の static 判定は弱い one-way ルールであり、後段の capability 強化がなく、contract surface に明示矛盾がないときだけ「static に残せる候補」とした。
- 逆に、完全な contract implication や capability lattice は導入していない。

static rejection に寄せたもの:

- `declared access target` 不一致
- `documented lineage annotation` による same-lineage 否定
- `declared capability surface` の明示的強化
- `declared contract surface` の明示的矛盾

dynamic `Reject` に残したもの:

- canonical chain 自体は well-formed で、評価時に admissible option が尽きるケース
- `lease` 期限切れ
- current `require` failure
- explicit failure
- write-after-expiry 後の後段不在

open question に残したもの:

- `documented lineage annotation` の具体的 surface form
- `declared contract surface` の比較粒度
- `declared capability surface` を read / write 以上に広げずには判定できない mixed case
- declaration 不足の underdeclared case を surface-level static error と elaboration obligation のどちらへ寄せるか

AMBIGUOUS:

- `documented lineage annotation` は今回、same-lineage の存在を記録する最小補助語として使っている。これを型付けされた path proof へ広げるかどうかは未決定である。
- `declared contract surface` の「明示的矛盾」は、現時点では syntactic / directly declared な矛盾に留めている。含意・充足可能性・subsumption まで行くかは未決定である。
- `declared capability surface` も、現時点では write-after-expiry と同じく最小の非強化判定にしか使っていない。

## 7. 今回整理して分かったこと

- 0020 で分けた static rejection と dynamic `Reject` の境界は、最小 declared information を置くだけでかなり明瞭になる。
- 一番重要なのは `declared access target` だけでは同一 lineage 証拠にならないと明言することで、これがないと same-path の暗黙仮定が強すぎた。
- 一方で contract-compatible successor を強く定義しすぎると、ただちに capability lattice や contract calculus に踏み込むため、今回は「明示矛盾の不在」と「非強化」までに留めるのが最小だった。

## 8. 未解決事項

- `documented lineage annotation` の具体的な surface form と証拠形式
- same-place / cross-place 文脈で lineage 証拠をどこまで共通化するか
- `declared contract surface` の明示矛盾をどの粒度で比較するか
- `declared capability surface` の mixed case を static rejection、dynamic `Reject`、elaboration のどこへ送るか
- declaration 不足の underdeclared case を surface-level static error にするか elaboration obligation にするか
- `lease` / `GuardedRef` の最終 surface syntax
- preference chain の完全代数、redundant option の collapse rule、dedicated observation surface

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、fallback / preference chain canonical normalization law・incompatible branch rejection phase・static evidence floor を前提に、underdeclared fallback case を surface-level static error にするか elaboration obligation にするかを整理してください。
目的は、`declared access target` と `documented lineage annotation` が不足している branch を、hidden acceptance せずにどう扱うかを最小限の意味論として定めることです。

- Mir-0 を広げないでください。
- canonical law / rejection phase / static evidence floor 自体は変更しないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`、`docs/reports/0021-fallback-preference-chain-static-evidence-floor.md` を読んでから始めてください。
- 新しい大きな型理論や capability lattice は導入しないでください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
