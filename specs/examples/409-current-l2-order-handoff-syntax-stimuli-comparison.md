# 409 — current L2 order / handoff syntax stimuli comparison

## 目的

rough syntax sketch を adoption 候補ではなく、
structure extraction のための comparison material として整理する。

## syntax research principle

1. 自然に書けるものは自然な挙動をするべきである。
2. 理論的に破綻した動作は書けない / 書きにくいべきである。
3. 文法は compactness だけでなく semantic honesty を担う。
4. packed row / implicit precedence / outer-inner drift を誘発する surface は避ける。

## comparison axes

1. semantic honesty
2. checker legibility
3. modal adequacy
4. misreading resistance

## non-normative stimuli

### stimulus A

```text
player = A
release(context)[ roll_dice(player) ]
- with release(context)[ dice_owner ]
```

### stimulus B

```text
atomic[dice_owner]{
  roll_dice(dice_owner)
  ---fence---
}
```

### stimulus C

```text
atomic{
  ---fence<seq_cst>---
  change_owner(&dice_owner)
  ---fence<seq_cst>---
}
```

### stimulus D

```text
update_global_state{
  change_owner(&dice_owner)
}
```

## extracted reading

| stimulus | extracted structure | current reading |
|---|---|---|
| A | publication / handoff / witness separation | useful comparison material |
| B | room-local serial transition sugar | useful comparison material |
| C | low-level / backend fence family | useful comparison material |
| D | high-level room macro / transaction-like sugar | useful comparison material |

## current judgment

- these sketches are **not adopted surface forms**.
- final grammar remains OPEN.
- what matters now is which semantic cluster each sketch is trying to surface.

## what is not decided here

- final syntax
- final sugar / macro boundary
- whether any of A–D survives into parser-facing notation
