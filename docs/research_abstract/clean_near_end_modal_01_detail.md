# clean near-end modal 01 detail

## この文書の役割

この文書は、clean near-end modal family の **実際に実行した sample と actual output** を確認するための detail です。
summary は `clean_near_end_modal_01.md` を参照してください。

## 実行した command

2026-04-23 に次を実行しました。

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-family modal --format json
```

## built-in と user-defined の境界

current built-in vocabulary のうち、この family で主に使うのは `transition`、`stage`、`perform`、`publish`、`witness` です。
一方で、`GameConfig`、`draw_pub`、`published(room)`、`witnessed(draw_pub)` は sample が使う user-defined / sample-defined 語です。

raw `◯` / `□` は current public sample syntax に出していません。

## 01_stage_stable_later_minimal

### 入力

```mir
module CleanNearEnd.StageStableLaterMinimal

stable config : GameConfig

transition staged_roll {
  stage prepare:
    cfg <- use stable config

  stage roll:
    draw <- perform roll_dice(cfg)
      produces value draw @ later

  stage publish:
    publish draw
      requires value draw @ later
      produces witness draw_pub
}
```

### actual output

```json
{
  "sample": "01_stage_stable_later_minimal",
  "family": "modal",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/modal/01_stage_stable_later_minimal.mir",
  "static_verdict": "valid",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": null,
  "constraints_solved": [
    "effect row { later, publish } <= { later, publish, witness }"
  ],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [],
  "mode_constraints": [
    "config : stable",
    "draw available at later stage",
    "publish requires later draw and produces witness"
  ],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "GameConfig",
    "draw_pub"
  ],
  "theorem_obligations": [
    "stable_values_are_reusable_across_stages"
  ],
  "witness_core_fields": [
    "draw_pub"
  ],
  "current_owner": null,
  "visible_history": []
}
```

## 02_published_witnessed_mode_bridge

### 入力

```mir
module CleanNearEnd.PublishedWitnessedModeBridge

transition published_to_witnessed_bridge {
  stage publish:
    publish draw
      scope room
      produces witness draw_pub
      produces value draw @ published(room)

  stage bridge:
    bridge draw
      from published(room)
      to witnessed(draw_pub)
      requires witness(draw_pub)
}
```

### actual output

```json
{
  "sample": "02_published_witnessed_mode_bridge",
  "family": "modal",
  "source_path": "/home/yukatayu/dev/mir_poc_01/crates/mir-runtime/../../samples/clean-near-end/modal/02_published_witnessed_mode_bridge.mir",
  "static_verdict": "valid",
  "entered_evaluation": false,
  "terminal_outcome": null,
  "reason_family": null,
  "constraints_solved": [
    "requires witness(draw_pub)",
    "publish(draw) must precede mode bridge"
  ],
  "constraints_failed": [],
  "residual_obligations": [],
  "relations": [
    [
      "publication_order",
      "publish",
      "bridge"
    ],
    [
      "witness_order",
      "publish",
      "bridge"
    ]
  ],
  "mode_constraints": [
    "value draw @ published(room)",
    "bridge consumes published draw",
    "bridge produces witnessed(draw_pub)"
  ],
  "model_check_result": null,
  "property": null,
  "checked_under": null,
  "counterexample_shape": [],
  "explanation": null,
  "built_in_terms": [
    "module",
    "index",
    "policy",
    "principal",
    "resource",
    "effect",
    "place",
    "option",
    "chain",
    "fallback",
    "lineage",
    "perform",
    "via",
    "require",
    "ensure",
    "atomic_cut",
    "transition",
    "stage",
    "publish",
    "observe",
    "handoff",
    "witness",
    "model",
    "property"
  ],
  "user_defined_terms": [
    "published(room)",
    "witnessed(draw_pub)"
  ],
  "theorem_obligations": [
    "published_mode_implies_witnessed_bridge_subject"
  ],
  "witness_core_fields": [
    "draw_pub"
  ],
  "current_owner": null,
  "visible_history": [
    "publish(draw)",
    "witness(draw_pub)"
  ]
}
```
