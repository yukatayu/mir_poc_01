# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 package 先までの near-term execution order を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

execution lane、theory-lab lane、reserve integration lane は分けて書く。

## current reading

- current execution line は `Macro 4 / malformed duplicate-cluster source-sample widening comparison with try-rollback malformed-static kept-later inventory` である。
- current theory-lab line は
  - `Macro 5` follow-up
    (`request/predicate/try cluster typed-surface reserve note`、`admissible evidence contraction note`、`tool-binding stop-line refresh`)
  - `Macro 5/6` follow-up
    (`order/handoff emitted-artifact schema reserve note`、`guarded-vs-MDTT/MTT reduction timing note`)
  である。
- current reserve integration line は `Macro 6/7 reserve integration checkpoint close` であり、next reopen は later mixed gate に残る。
- mainline を急ぐ task と theory-lab package は混ぜず、同じ snapshot から parallel に進める。

## ordered package list

| 順番 | lane | macro | task package | completion signal | rough estimate |
|---|---|---|---|---|---|
| 1 | theory-lab | `Macro 5` | request / predicate / `try` cluster typed-surface reserve note | handoff migration 後の next typed-surface reserve line が揃う | 1〜2 task |
| 2 | theory-lab | `Macro 5` | admissible evidence contraction note | proof artifact / bridge stop line fixed 後の next theorem reserve line が揃う | 1〜2 task |
| 3 | theory-lab | `Macro 5` | tool-binding stop-line refresh | sample-facing summary fixed 後の next model-check reserve line が揃う | 1〜2 task |
| 4 | theory-lab | `Macro 5/6` | order / handoff emitted-artifact schema reserve note | property-language bridge fixed 後の emitted-artifact schema reserve line が揃う | 1〜2 task |
| 5 | theory-lab | `Macro 5/6` | guarded-vs-MDTT/MTT reduction timing note | modal promotion threshold fixed 後の stronger-candidate reduction timing が揃う | 1〜2 task |
| 6 | execution | `Macro 4` | malformed duplicate-cluster source-sample widening comparison | duplicate cluster の next widening cut と non-promotion guard が揃う | 1〜2 task |

## batch recommendation

### Batch A. theory-lab second-order follow-up

1. package 1
2. package 2
3. package 3
4. package 4
5. package 5

### Batch B. execution later reopen

6. package 6

## reopen triggers

- typed/theorem/model-check line で principal source / evidence contraction / tool-binding stop line の drift が見えたら、packages 1〜3 を前倒しする。
- order / handoff line で emitted-artifact schema や stronger modal reduction timing の ambiguity が増えたら、packages 4〜5 を前倒しする。
- malformed line で duplicate cluster widening と broader try-rollback malformed-static family の順序 drift が増えたら、package 6 を前倒しする。
- reserve integration line で installed packaging / shared-space final fairness profile の hidden promotion が見えたら、later mixed gate note を別 package で reopen する。

## current recommendation

- 近接 priority は theory-lab packages 1〜5 と execution package 6 でよい。
- raw FFI、game engine direct binding、full strong type system、concrete theorem / model-check tool binding は、この文書の near-term line に入れない。
