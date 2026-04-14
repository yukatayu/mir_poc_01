# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 package 先までの near-term execution order を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## current reading

- current promoted line は `Macro 4 / stable malformed capability second source-backed widening actualization` である。
- current reserve line は `Macro 7 / public operational CLI concrete shell actualization` である。
- その後は、new sample family を無理に増やすより先に、typed / theorem / model-check detail-side plan を硬化するのが自然である。
- shared-space と host-I/O は docs-first boundary が already あるので、mainline の邪魔をしない compact reserve package として並走できる。

## ordered package list

| 順番 | macro | task package | なぜ今やるか | completion signal | rough estimate | status |
|---|---|---|---|---|---|---|
| 1 | `Macro 4` | stable malformed capability second source-backed widening actualization | current runnable line を 1 段太らせるため | `e13/e20` family の actual package cut が fixed され、runner / ladder / regression への widening 条件が閉じる | 1〜3 task | promoted |
| 2 | `Macro 7` | public operational CLI concrete shell actualization | thin facade と shell concern の境界を崩さないため | current-L2 scoped shell concern と excluded/support bucket が actual package として閉じる | 1〜2 task | reserve |
| 3 | `Macro 5` | typed-core attachment inventory and obligation allocation refresh | full type system へ飛ばず boundary plan を前進させるため | first attachment candidate、obligation owner、stop line が docs-first に固定される | 1〜2 task | adjacent |
| 4 | `Macro 5` | semantic-core theorem pilot planning | theorem line の next proof order を曖昧にしないため | first lemma order、carrier、tool-neutral stop line が fixed される | 1〜2 task | adjacent |
| 5 | `Macro 5` | model-check projection / property-family reserve inventory | concrete carrier の次段を明確にするため | projection grain、first property family、tool-binding stop line が整理される | 1〜2 task | adjacent |
| 6 | `Macro 6` | shared-space room-profile working subset and confusion/replay compact table | reopen 済み boundary の drift を抑えるため | small room-profile と confusion/replay table が docs-first にまとまる | 1〜2 task | reserve |
| 7 | `Macro 7` | host-I/O binding artifact / bridge-only note | I/O line の hidden promotion を防ぐため | bridge-only host binding note と excluded target line が固定される | 1 task | reserve |
| 8 | `Macro 5/6` | async-control / ordering boundary inventory | `memory_order` line を theory-first で整理するため | `atomic_cut` nucleus と higher-level ordering handoff boundary が整理される | 1〜2 task | reserve |

## autonomous batch recommendation

### Batch A. execution-facing closeout

1. package 1
2. package 2

### Batch B. reasoning-side hardening

3. package 3
4. package 4
5. package 5

### Batch C. boundary reserve hardening

6. package 6
7. package 7
8. package 8

## reopen triggers

- malformed line で runner / ladder mismatch が出たら、package 1 を優先する。
- public shell line で hidden promotion が見えたら、package 2 を優先する。
- typed / theorem / model-check line で sample-visible next step が曖昧になったら、packages 3〜5 を前倒しする。
- shared-space / host-I/O / ordering line で terminology drift が見えたら、packages 6〜8 を挟む。

## current recommendation

- 近接 mainline は packages 1〜2 のままでよい。
- ただし user が「大局計画を先に固めたい」と明示したので、packages 3〜8 の detail-side planning を `plan/18` と `plan/12` / `plan/13` へ先に明文化しておく。
- raw FFI、game engine direct binding、full strong type system、concrete theorem / model-check tool binding は、この文書の near-term line に入れない。
