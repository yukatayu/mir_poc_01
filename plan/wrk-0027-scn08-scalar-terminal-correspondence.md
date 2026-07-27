# WRK-0027 - SCN-08 scalar terminal correspondence

## 役割と境界

これは `working/WRK-0027` の登録後 source comparison を保持する **LAB evidence**
である。規範正本は `mirrorea_canon/` であり、この文書は Surface grammar、static
semantics、MirCore、SCN、fallback policy、OBL、Gate、Phase、runtime、public contract を
変更しない。

## 再現 cut

- Authority/input cut: `dfbe31d3d2b75ebaab6182240e80769ff6e95048`
- Working record: `mirrorea_canon/working/WRK-0027-scn08-scalar-terminal-correspondence.md`
- Result class: literal-transcription
- Registered checks: source existence, indexed Surface/Core declaration marker,
  SCN-08 `room_anchor`, SCN-08/P015 `default_pose`, and `git diff --check`

すべての登録 command は上記 registration push の後に exit 0 で通った。これは
source marker の再現であり、候補設計の compile、runtime 実行、conformance、proof
ではない。

## 観測された入力

| Source | 観測された literal fact | この fact 単独から言えないこと |
| --- | --- | --- |
| `spec/02-surface-grammar.md` | `StateDecl` は `state Ident [ Ident : Keyspace ] : Type` の indexed form | scalar declaration を拒否する最終 grammar decision |
| `spec/03-static-semantics.md` | indexed state は owner-owned `Active(K, epoch) ⇀ A` と説明される | scalar cell の storage/WF representation |
| `theory/01-mircore-v0.md` | MirCore state declaration と store explanation は indexed state form を示す | scalar Core constructor 又は elaboration policy |
| `theory/06-existence-fallback.md` | chain option は declared access target を必要とする | `default_pose` の declaration/resolution site |
| `scenarios/SCN-08-avatar-fallback.md` | `live_pose[p: Participant]` と `room_anchor: Pose`、`default_pose` の fallback option を例示する | この scenario notation 自体が既に static well-formed であること |
| `meta/proposals/PROPOSAL-015-surface-fallback-and-return-closure.md` | scalar terminal/default は明示的な Surface/Core correspondence を必要とし、hidden key/type default/unbound terminal を禁止すると記録する | correspondence の具体表現 |

## 限定結論

表示済みの source は、SCN-08 の scalar anchor と terminal default を既存の
indexed-state rule へ黙って落とす correspondence を供給しない。従って現在の
literal source だけから、`room_anchor` の owner、store shape、initialization、visibility、
well-formedness、又は `default_pose` の declaration/target resolution を復元しては
ならない。

これは「SCN-08 が invalid」「scalar state は不可能」「finite-domain elaboration が
不適切」という結論ではない。P015 と整合する未決 boundary を、後続の設計比較に残す
結果である。

## 後続比較の最小観点

後続 package は、少なくとも次の二候補を混同せずに比較する。

| 候補 | 必要な明示事項 | 失敗する shortcut |
| --- | --- | --- |
| distinct scalar Core declaration | owner、store slot、init、visibility、target resolution、WF | indexed keyspace を hidden singleton として導入する |
| already-declared finite domain への conservative elaboration | domain の既存宣言、source-to-Core mapping、init/default evidence、round-trip/diagnostic | scenario 側だけで domain/default を暗黙生成する |

いずれも chain static floor、THM-002 の monotone degradation、reacquire rule、failure
no-mutation と整合する trace を要する。これは候補選択ではなく比較条件である。

## ergonomics / inference への含意

source の省略を許すのは、規範入力から一意に決まり elaborated artifact から復元できる
場合だけである。この case では P015 が明示的に禁止する三つの補完（hidden membership
key、type-derived default、unbound terminal）があり、必要な correspondence も未選択
なので、scalar/terminal の省略を ergonomic inference として扱えない。C7 はこの結果を
input にするが、C7 自体は未実施である。

## 非効果と次の停止線

この evidence は Canon rule、scenario expectation、diagnostic、OBL、implementation
authorization を変更しない。shared operational model を受理する前に、scalar
representation と terminal target の候補を C1--C5/C7 の shared carrier 条件と一緒に
比較し、明示的な Canon process へ送る必要がある。
