# tasks

最終更新: 2026-04-17 09:04 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- detail は `plan/11`、`plan/12`、`plan/13`、`plan/18` に寄せ、ここでは current line / 順番 / blocker だけを書く。
- 規範判断の正本は `specs/` である。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current lanes

- execution lane:
  `Macro 0〜4 closeout fixed`
  （duplicate pair `e14/e15` actualized、broader try-rollback malformed-static family は kept-later inventory）
- theory-lab lane:
  `Macro 5 third-order follow-up active`
  （model-check small-cluster projection keep/drop と order/handoff source-surface wording reserve は fixed 済み、next reopen は modality internalization trigger note）
- reserve integration lane:
  `Macro 6/7 reserve integration checkpoint close`
  （public operational CLI packaging reserve note と shared-space fairness / replay strengthening reserve note は fixed 済み、next reopen は later mixed gate）
- immediate blocker: `0`
- current lane を止める user decision: `0`

## 自走して closeout まで持って行ける macro phase 読み

`Macro 0〜8` は repo 全体の top-level macro axis であり、現時点では `Macro 9` 以降を置く想定はない。
後続の仕事は「`Macro 8` の先に積む」のではなく、`Macro 0〜7` の深化として戻るか、application-specific realization として `Macro 8` に入る。
したがって `Macro 8` は catch-all bucket ではなく、application / domain target 専用の phase として読む。

| macro phase | self-driven closeout 読み | 注記 |
|---|---|---|
| `Macro 0` | はい | maintenance closeout まで |
| `Macro 1` | はい | narrow semantic reopen の closeout まで |
| `Macro 2` | はい | current parser-free substrate closeout まで |
| `Macro 3` | はい | compile-ready minimal current tranche closeout まで |
| `Macro 4` | はい | current fixed-subset widening closeout まで |
| `Macro 5` | はい | current boundary / pilot / framing closeout まで |
| `Macro 6` | いいえ | docs-first boundary / fairness-replay seam までは self-driven、full closeout は mixed gate |
| `Macro 7` | いいえ | thin facade / packaging boundary-prep までは self-driven、full closeout は mixed gate |
| `Macro 8` | いいえ | user specification 必須 |

## 自走可能な task package

### promoted immediate line

| 順番 | lane | macro | task package | completion signal | rough estimate |
|---|---|---|---|---|---|
| 1 | theory-lab | `Macro 5/6` | modality internalization trigger note | stronger-foundation trigger が current stop line と整合して揃う | 1〜2 task |

### boundary-prep まで自走可能な reserve queue

| 順番 | lane | macro | task package | completion signal | rough estimate |
|---|---|---|---|---|---|
| 2 | theory-lab | `Macro 5/6 reserve` | stronger typed-surface promotion threshold framing note | stronger typed surface を current stop line から越えずに reopen する条件と non-goal が揃う | 1 task |
| 3 | theory-lab | `Macro 5/6 reserve` | theorem discharge transport / public-contract later-gate framing note | abstract discharge-entry reserve の先にある transport / public contract の later seam が揃う | 1 task |
| 4 | theory-lab | `Macro 5/6 reserve` | model-check property-language / tool-binding later-gate framing note | first settled property language と concrete tool seam の stop line が揃う | 1 task |
| 5 | reserve integration | `Macro 6 reserve` | shared-space fairness / replay mixed-gate boundary note | final operational catalog へ送る前の fairness / replay boundary と user-spec-required seam が揃う | 1 task |
| 6 | reserve integration | `Macro 7 reserve` | public operational CLI installed-binary / packaging success-criteria mixed-gate boundary note | installed-binary promotion と packaging success criteria の mixed gate が揃う | 1 task |

## 研究で詰める論点

- modality internalization trigger
  - detail / options / current recommendation は `plan/18` Track F を参照。
- stronger typed-surface promotion threshold framing
  - detail / options / current recommendation は `plan/18` Track A と `plan/12` を参照。
- theorem discharge transport / public-contract framing
  - detail / options / current recommendation は `plan/18` Track B と `plan/12` を参照。
- model-check property-language / tool-binding framing
  - detail / options / current recommendation は `plan/18` Track C と `plan/12` を参照。
- shared-space fairness / replay mixed-gate boundary
  - detail / options / current recommendation は `plan/16` と `plan/12` を参照。
- public operational CLI installed-binary / packaging success-criteria boundary
  - detail / options / current recommendation は `plan/09` と `plan/12` を参照。

## mixed gate / later reserve

- 上の 2〜6 は boundary-prep までは self-driven に進めてよい。
- ただし、その先の
  - stronger typed surface の実昇格
  - theorem public contract の具体化
  - model-check concrete tool binding / property language の確定
  - shared-space fairness / replay operational profile の最終化
  - installed-binary promotion / packaging success criteria の確定
  は mixed gate のままに保つ。

## user が後で決めること

- shared-space final catalog
- first external integration target
- backend / tooling success criteria
- first application target
