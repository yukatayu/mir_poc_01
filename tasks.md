# tasks

最終更新: 2026-04-16 23:43 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- detail は `plan/11`、`plan/12`、`plan/13`、`plan/18` に寄せ、ここでは current line / 順番 / blocker だけを書く。
- 規範判断の正本は `specs/` である。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current lanes

- execution lane:
  `Macro 4 / malformed duplicate-cluster source-sample widening comparison with try-rollback malformed-static kept-later inventory`
- theory-lab lane:
  `Macro 5` follow-up
  (`request/predicate/try cluster typed-surface reserve note`、`admissible evidence contraction note`、`tool-binding stop-line refresh`) と
  `Macro 5/6` follow-up
  (`order/handoff emitted-artifact schema reserve note`、`guarded-vs-MDTT/MTT reduction timing note`)
- reserve integration lane:
  `Macro 6/7 reserve integration checkpoint close`
  （public operational CLI packaging reserve note と shared-space fairness / replay strengthening reserve note は fixed 済み、next reopen は later mixed gate）
- immediate blocker: `0`
- current lane を止める user decision: `0`

## 自走可能な task package

| 順番 | lane | macro | task package | completion signal | rough estimate |
|---|---|---|---|---|---|
| 1 | theory-lab | `Macro 5` | request / predicate / `try` cluster typed-surface reserve note | handoff migration 後の next typed-surface reserve line が揃う | 1〜2 task |
| 2 | theory-lab | `Macro 5` | admissible evidence contraction note | proof artifact / bridge stop line fixed 後の next theorem reserve line が揃う | 1〜2 task |
| 3 | theory-lab | `Macro 5` | tool-binding stop-line refresh | sample-facing summary fixed 後の next model-check reserve line が揃う | 1〜2 task |
| 4 | theory-lab | `Macro 5/6` | order / handoff emitted-artifact schema reserve note | property-language bridge fixed 後の emitted-artifact schema reserve line が揃う | 1〜2 task |
| 5 | theory-lab | `Macro 5/6` | guarded-vs-MDTT/MTT reduction timing note | modal promotion threshold fixed 後の stronger-candidate reduction timing が揃う | 1〜2 task |
| 6 | execution | `Macro 4` | malformed duplicate-cluster source-sample widening comparison | duplicate cluster の next widening cut と non-promotion guard が揃う | 1〜2 task |

## 研究で詰める論点

- request / predicate / `try` cluster typed-surface reserve
  - detail / options / current recommendation は `plan/18` Track A を参照。
- admissible evidence contraction
  - detail / options / current recommendation は `plan/18` Track B を参照。
- model-check tool-binding stop line
  - detail / options / current recommendation は `plan/18` Track C を参照。
- order / handoff emitted-artifact schema reserve
  - detail / options / current recommendation は `plan/18` Track D を参照。
- guarded-vs-MDTT/MTT reduction timing
  - detail / options / current recommendation は `plan/18` Track F を参照。

## mixed gate / later reserve

- shared-space final fairness / replay operational profile
  - detail / options / current recommendation は `plan/16` と `plan/12` を参照。
- public operational CLI installed-binary / packaging success criteria
  - detail / options / current recommendation は `plan/09` と `plan/12` を参照。

## user が後で決めること

- shared-space final catalog
- first external integration target
- backend / tooling success criteria
- first application target
