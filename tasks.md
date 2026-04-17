# tasks

最終更新: 2026-04-17 09:34 JST

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
  `Macro 5 boundary / pilot / framing closeout fixed`
  （modality internalization trigger、stronger typed-surface threshold framing、theorem discharge transport / public-contract later-gate framing、model-check property-language / tool-binding later-gate framing は fixed 済み、next work は mixed gate only）
- reserve integration lane:
  `Macro 6/7 mixed-gate boundary fixed`
  （shared-space fairness / replay mixed-gate boundary と public operational CLI installed-binary / packaging success-criteria mixed-gate boundaryは fixed 済み、next work は mixed gate / user-spec-required）
- current self-driven queue: `0 package`
- immediate blocker: `0`
- next gate: `mixed gate / user specification`

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
| `Macro 5` | はい | current boundary / pilot / framing closeout は fixed 済み |
| `Macro 6` | いいえ | docs-first boundary / mixed-gate boundary までは self-driven、full closeout は mixed gate |
| `Macro 7` | いいえ | thin facade / shell / mixed-gate boundary までは self-driven、full closeout は mixed gate |
| `Macro 8` | いいえ | user specification 必須 |

## current self-driven package reading

- current snapshot では、**promoted self-driven package は残っていない**。
- `Macro 0〜5` の current scoped self-driven closeout は fixed と読む。
- 次に残るのは、mixed gate topic の concretization / adoption / packaging / final catalog 側であり、current `tasks.md` では self-driven package としては数えない。

## mixed gate / later reserve

| topic | lane | macro | current reading | next gate |
|---|---|---|---|---|
| stronger typed surface promotion | theory-lab | `Macro 5/6` | threshold framing は fixed | mixed gate |
| theorem discharge transport / public-contract concretization | theory-lab | `Macro 5/6` | later-gate framing は fixed | mixed gate |
| model-check property-language / tool-binding concretization | theory-lab | `Macro 5/6` | later-gate framing は fixed | mixed gate |
| shared-space final fairness / replay operational profile | reserve integration | `Macro 6` | mixed-gate boundary は fixed | mixed gate / user spec |
| public operational CLI installed-binary promotion / packaging success criteria | reserve integration | `Macro 7` | mixed-gate boundary は fixed | mixed gate / user spec |

## 研究で詰める論点

- stronger typed surface を actual adoption する threshold と non-goal
  - detail / options / current recommendation は `plan/18` Track A と `plan/12` を参照。
- theorem discharge transport / public-contract concretization の gate
  - detail / options / current recommendation は `plan/18` Track B と `plan/12` を参照。
- model-check property-language / tool-binding concretization の gate
  - detail / options / current recommendation は `plan/18` Track C と `plan/12` を参照。
- modal foundation adoption の mixed-gate threshold
  - detail / options / current recommendation は `plan/18` Track F と `plan/12` を参照。
- shared-space fairness / replay final profile
  - detail / options / current recommendation は `plan/16` と `plan/12` を参照。
- public operational CLI installed-binary / packaging success criteria
  - detail / options / current recommendation は `plan/09` と `plan/12` を参照。

## user が後で決めること

- shared-space final catalog
- first external integration target
- backend / tooling success criteria
- first application target
