# Plan 204 - WRK-0034 後の意味論合成 frontier provisional disposition

## 役割と権限

これは WRK-0034 の evidence cut に対する **LAB repository memory** である。
`mirrorea_canon/` が唯一の規範正本であり、本書は Core、Surface grammar、型・効果・失敗、
authority、契約、SCN、OBL、Gate/Phase、runtime、wire、公開 API を選択・変更しない。

ここでいう `no-candidate` は、WRK-0034 と同じ固定 finite-presentation line に新しい
`WRK-####` を pre-register できない、という局所的な disposition である。C7 の抽象的な
reconstructibility criterion は別 package で preflight、登録、実行、metadata link まで完了したため、
この Plan は fixed finite line の disposition と C7 への分岐を記録する historical LAB memory とする。
プロジェクト全体の研究、将来の既存 LAB lane、又は owner/canon による通常の設計を閉じるものではない。

## 対象 cut と確定している事実

対象は次の到達済み evidence chain である。

| 段階 | commit | 限定された内容 |
| --- | --- | --- |
| finite-sequence candidate selection | `1553bcc8fd140ad5ca98f5d7294fd802f776c7f1` | WRK-0033 の固定 model を変えない有限列 closure だけを候補化 |
| WRK-0034 pre-registration | `384a94bb3882da7acab393a38d663cf8994c59b4` | alternative、falsifier、non-effects、rollback を先に固定 |
| retained LAB evidence | `dc66f08237acd11e4de722cd67a42fae0b26e1eb` | `List.foldl` 上の fixed translation-preservation と final local observation equality |
| Canon metadata link | `c1af9c5007eb0a16ca6224d4742fd59883027321` | L3 `not-promoted` record の evidence pointer を link |

WRK-0034 は、opaque な有限 reply list に対し、WRK-0033 と byte-identical な
state/reply/transition/translation/observation/assumption を使う。従って retained したのは、
同じ list に対する fixed translation-preservation と、それから従う final local observation equality だけである。
これは Mir の trace、delivery、scheduler、history、pending carrier、request/receipt/occurrence
identity、payload/provenance、authority、failure semantics、persistence、source elaboration、
又は C3 proper を定義しない。

## 再審査の方法

Plan 199、Plan 200、Plan 203、WRK-0034、ADR-0014、P012 の current wording を照合した。
一時的な Oracle review はこの比較の補助入力として利用したが、以下の disposition は
Canon source と既存 LAB evidence に照らして記録するものであり、外部助言そのものを正本に
しない。

候補には、次を同時に要求した。

1. 既存 documented LAB lane の中で完結し、新しい helper、schema、CI/Make surface、evidence lane、
   production surface を増やさないこと。
2. 既存結果の言い換え、直接の系、又は単なる source audit ではないこと。
3. Core/authority/effect/failure/judgment、source/external contract、SCN、OBL、Gate/Phase、
   `theory/11` を選択又は再解釈しないこと。
4. 一意に再構成できる elaborated basis が既にある場合を除き、source omission/desugaring を
   仮定しないこと。

以上はこの evidence cut における LAB の candidate-selection discipline であり、ADR-0014 の
standing predicate に追加条件を設けるものではない。

## 再審査結果

| frontier | carrier-neutral に残る作業 | disposition と理由 |
| --- | --- | --- |
| C0-D totality shape | outcome existence と coherence の分離を再記述すること | **no-candidate**。P008 と既存 bounded evidence の重複であり、exact domain、equality/coherence、Diagnostic 又は OBL identity を選ぶと Canon statement design になる |
| C1 SCN-02 read/write composition | owner-serial write が stale read-dependent write を防がない例の追加 | **no-candidate**。WRK-0024 の再述であり、有意な比較には snapshot、evaluation locus、fusion 又は pending relation が必要 |
| C2-B identity anchor | identity 用語の追加整理 | **no-candidate**。C2-A/WRK-0030 の documentary boundary を超えるには equality、causal edge、replay、persistence の意味を選ぶ必要がある |
| C3 proper V1/R1 | arbitrary finite-list preservation theorem の list-prefix specialization / length-bound restriction | **no-candidate for the unchanged finite model**。corollary は新 evidence でなく、pending/receipt/correlation/held context/success-failure-resume/cut-save-load は Canon design の対象 |
| C4 SW1 | served-write の語を抽象化すること | **owner/Canon boundary**。request binding、facet projection、validation/mutation/failure relation を定義しなければ有意な検査にならない |
| C5 proper conditional A2 | ordinary-admission source span の追加 audit | **owner/Canon boundary**。composite occurrence、membership/grant/witness projection、lineage、rejection residue、load/rollback を選ぶ必要がある。issuance が独立に失敗・観測・schedule されるなら A1 successor decision が先行する |
| C6 scalar / terminal closure | source gap の再監査 | **no-candidate**。WRK-0027 の重複であり、scalar representation 又は finite-domain elaboration、ownership、initialization、visibility、persistence、terminal target を選ぶ必要がある |
| C7 source ergonomics | `erase : E -> S` と `observe : E -> O` を parameter にした fiber-constancy / unique-reconstruction criterion | **retained as WRK-0035 L3 evidence**。pointwise unique realized observation on `range erase` と explicit collision、full-codomain countermodel に限定する。concrete source omission rule、Mir carrier、authority、failure、identity、history は選ばない |

したがって、WRK-0034 と同じ fixed model に対する list-prefix specialization、別名の
final-observation theorem、`List.length xs <= n` に制限した同じ fold-preservation statement、
又は同じ有限 model の proof embellishment は、新しい L3 result として開始しない。C7 の
carrier-neutral factorization criterion は WRK-0035 で generic L3 conditional lemma として retained
したが、source rule、grounds、concrete elaborated artifact、又は reconstruction function は未解決である。trace、delivery、
history、persistence、liveness、infinite behavior を扱うために model を拡張することは、この
fixed-presentation line では新たな semantic selection となる。
WRK-0035 後の別 frontier では、Plan 206 が individually checked erasures の common coarsening を
直接検査しない危険だけを fixed finite countermodel として pre-register する候補に選別した。
この selection は C7 source rule 又は C3 proper の carrier 設計を開始しない。

## 次に必要な通常設計の最小境界

WRK-0034 と同じ fixed finite model に関する次の substantive action は、新規 successor WRK ではない。
carrier を選択する作業について現在記録されている ordinary design boundary は C3 proper であるが、
その着手順序と採否は owner/Canon process に属する。C7 の retained L3 result は C3 proper の
carrier 選択又は source inference を代替しない。C3 proper は少なくとも次を明示する必要がある。

1. typed pending / result / reply / receipt / correlation の carrier と equality/identity の範囲。
2. held `Gamma` / `Delta`、success と failure の transition、one resumption、evaluation order。
3. snapshot、cut、save/load、replay、authority/redaction/observability に関する non-effects 又は
   その後段への明示的な切り出し。
4. source 側で省略可能な事実は、上記の選択後に elaborated artifact から一意に再構成できるかを
   検査すること。構文上の sugar 候補も将来の design comparison として明示的に扱い、Canon/owner の
   選択前には現行 source rule 又は許可として扱わない。identity、authority、failure、history の
   ような観測可能な意味論事実を推測で補わない。

この package は最終仕様を先回りして固定するためのものではない。各 candidate と adverse case を
比較し、全体の簡潔さ、C0--C7 の整合、既存 scenario/contract への影響、将来の replaceable
adapter boundary を確認してから Canon process に載せる。

## 再開条件と非効果

この scoped frontier を再審査する代表的な契機は次のとおりである。この列挙は ADR-0014 の
standing predicate を限定するものではない。

- Canon/owner が C3、C4、C5、C6、又は C7 に必要な semantic carrier/contract を通常手続で
  決定し、既存 LAB lane で検査可能な狭い consequence が現れた場合。
- authority cut が変わり、既存結果と重複しない literal transcription、countermodel、
  conditional lemma、又は existing-lane experiment が ADR-0014 の standing predicate を満たす場合。
- 新たな candidate が、source inference を仮定せず、exact input、alternative/falsifier、
  non-effects、rollback を pre-register できる場合。

それまでは、この disposition を根拠とする同一 fixed-presentation line の successor WRK を作らない。
別 frontier の candidate、将来の fresh ADR-0014 preflight、又は WRK 番号を制約しない。この記録は
L2 promotion、proof/OBL status、conformance、Gate/Phase、implementation readiness、又は public completion を変更しない。

## 次の更新規則

- `plan/199` と `plan/200` は、この scoped disposition を current frontier の状態として参照する。
- `progress.md` と `tasks.md` は「C3 proper を始める前に再審査」の古い next step を、
  「固定 finite-presentation line は no-candidate、C7 factorization は retained L3 evidence、carrier 選択は
  ordinary Canon design boundary」へ更新する。
- 新しい L3 evidence を作る場合は、この Plan 204 を根拠にせず、未来の authority cut で
  ADR-0014 の standing predicate を最初から再評価する。
