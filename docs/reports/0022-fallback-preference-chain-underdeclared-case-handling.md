# 報告 0022 — fallback / preference chain underdeclared case の扱い整理

- 作成日時: 2026-03-31T02:12:00+09:00
- 作成者 / agent: Codex
- 対象範囲: canonical normalization law・incompatible branch rejection phase・static evidence floor を前提にした underdeclared fallback case の最小分類、surface-level static error と `elaboration obligation` の境界
- 触れた decision level: 既存の Mir-0 境界と L2 の canonical law / rejection phase / static evidence floor を維持したまま、underdeclared case の扱いだけを 1 段具体化した

## 1. 目的

`declared access target` と `documented lineage annotation`、および successor 判定に必要な contract / capability 情報が不足している fallback branch を、hidden acceptance せずにどう扱うかを最小限の意味論として定める。
今回は新しい capability theory や elaboration system を導入せず、current L2 で何を static error にし、`elaboration obligation` をどこまで未決定に残すかだけを整理する。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0018`、`docs/reports/0019`、`docs/reports/0020`、`docs/reports/0021` を指定順で読んだ。
- 作業開始時点の git dirty state は、branch が `main...origin/main [ahead 4]` で、tracked / untracked の作業中変更はなかった。
- 既存の canonical law、rejection phase、static evidence floor は変更しない。
- `code_mapper` を最初に使い、`specs/04`、`specs/10`、`specs/12` が中心であり、`specs/09` は invariant を強めない前提で参照に留めるべきことを確認した。
- `reviewer` を最後に使い、underdeclared case を surface-level static error に寄せたことで L2 を強めすぎていないかを点検した。

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
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`

## 4. 実施内容

- `code_mapper` を最初に起動し、task-start dirty state が空であること、中心ファイルが `specs/04`、`specs/10`、`specs/12` であること、L2 を invariant 側へ持ち上げないことを確認した。
- current L2 の underdeclared case を次の 3 類型に整理した。
  - `declared access target` はあるが `documented lineage annotation` がない場合
  - `declared access target` 自体が不足している場合
  - `declared contract surface` または `declared capability surface` が不足し、successor 判定ができない場合
- `specs/04-mir-core.md` では、これら 3 類型を underdeclared fallback case と定義し、current L2 ではいずれも surface-level static error とすること、dynamic `Reject` に押し込まないこと、`elaboration obligation` は current L2 の admitted path に含めないことを追加した。
- `specs/10-open-questions.md` では、上の current L2 判断を反映しつつ、future extension として `elaboration obligation` を本当に認める必要があるかを **未決定** として残した。
- `specs/12-decision-register.md` では、D-027 を D-028 参照へ整え、新たに D-028 を追加した。あわせて reviewer 指摘に従い、D-026 の stale な文言を「詳細比較粒度と mixed case が未決定」という現在の状態に狭めた。
- reviewer の初回指摘は 2 点だった。
  - D-026 が D-027 / D-028 と内部矛盾していたこと
  - `target anchor` という未定義語が新しく紛れ込んでいたこと
- 上の 2 点を修正した後、reviewer の再確認は `no findings` だった。
- reviewer は shared workspace に監査用 scratch report を 2 回生成したが、これは今回の成果物に含めず、内容だけ確認して作業木から除外した。最終成果物として残す report は本報告のみである。
- commit は 2 本に分ける。
  - 1 本目は仕様本文だけを記録する `fallback underdeclared case の扱いを整理する`（実際の commit は `d2c58e6`）
  - 2 本目は本 report を記録する `fallback underdeclared case の作業報告を追加する`
  - ただし 2 本目の commit はこの report 自身を含むため、self-reference の都合で本文中に最終 hash を固定値で埋め込めない。実 hash は final response と `git log` で補完する。

今回実際に変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`

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
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`

## 5. コマンドと正確な出力

1. 作業開始時点の `git status --short --branch`

```text
## main...origin/main [ahead 4]
```

2. `python3 scripts/new_report.py --slug fallback-preference-chain-underdeclared-case-handling`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md
```

3. reviewer 初回結果

```text
1. `D-026` is now stale and contradicts the new `D-027`/`D-028` split.
2. The new underdeclared-case text introduces `target anchor` as a deciding term without defining it.
```

4. reviewer 再確認

```text
no findings
```

5. 仕様本文 commit

```text
[main d2c58e6] fallback underdeclared case の扱いを整理する
 3 files changed, 15 insertions(+), 6 deletions(-)
```

## 6. 証拠 / 所見

今回の current L2 判断:

- underdeclared fallback case は、same-lineage 判定または successor 判定を static evidence floor まで持ち上げられない branch と定義した。
- current L2 では、この underdeclared case はすべて surface-level static error とする。
- したがって underdeclared branch は hidden に canonical chain へ入らず、dynamic `Reject` にも落ちない。
- `elaboration obligation` は current L2 の admitted path には入れない。将来これを導入するなら、canonicalization / evaluation 前に同じ static evidence floor を discharge しなければならない。

surface-level static error に寄せた理由:

- 既存の static evidence floor は「何が同じ lineage か」「何が successor として残れるか」を最小 declared information で判定するための floor であり、これが欠けた時点で well-formed chain として扱えない。
- ここで dynamic `Reject` に送ると、malformed branch と runtime exhaustion が再び混ざり、D-026 の rejection phase が曖昧になる。
- `elaboration obligation` を current L2 に admitted path として入れると、新しい elaboration system の仕様境界まで必要になり、今回の仕事の範囲を超える。

既存理論との整合:

- canonical normalization law 自体は変更していない。
- incompatible branch rejection phase も維持しており、変えたのは「underdeclared は malformed 側で止める」という current L2 の出口だけである。
- static evidence floor の中身も変更していない。変更したのは、その floor を満たせない場合の扱いだけである。
- `lease`、`try`、`atomic_cut`、dynamic `Reject` の既存意味はそのまま残る。

## 7. 今回整理して分かったこと

- D-027 までで「hidden acceptance はしない」は既に決まっており、今回の論点は「ではどこで止めるか」だけだった。
- 最小方針としては、underdeclared case を一律で surface-level static error に寄せるのが最も軽く、canonical law・rejection phase・static evidence floor の 3 つを壊さない。
- `elaboration obligation` を今すぐ admitted path にしないことで、将来の拡張余地は残しつつ、現在の意味論はかなり明瞭になった。

## 8. 未解決事項

- `documented lineage annotation` の具体的な surface form と証拠形式
- `declared contract surface` の「明示的矛盾」をどの粒度まで static に比較するか
- `declared capability surface` を read / write 以上に広げずに扱えない mixed case を、将来 static rejection に寄せるか dynamic `Reject` に残すか
- future extension として `elaboration obligation` を認める必要が本当にあるか
- `lease` / `GuardedRef` の最終 surface syntax
- preference chain の完全代数、redundant option の collapse rule、dedicated observation surface

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、fallback / preference chain canonical normalization law・incompatible branch rejection phase・static evidence floor・underdeclared case handling を前提に、
`documented lineage annotation` の最小 surface form を整理してください。
目的は、same logical access path / semantic lineage の static 証拠として何を最低限書けばよいかを、current L2 の範囲で最小限だけ明文化することです。

- Mir-0 を広げないでください。
- canonical law / rejection phase / static evidence floor / underdeclared handling 自体は変更しないでください。
- 新しい大きな型理論や capability lattice は導入しないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`、`docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md` を読んでから始めてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
