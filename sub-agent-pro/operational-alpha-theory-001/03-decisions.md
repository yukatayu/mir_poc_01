# 03 — decisions

## D1. practical α の completion は operational readiness とする

以後、`α-0.5 complete`、`α-0.8 complete` のような表現は、first-floor や evidence closeout ではなく、開発者が当該 layer を workflow として再現できる operational readiness を意味する。

## D2. current first floors は有用だが completion ではない

`package.mir.json` front-door、checker rows、local runtime report、hot-plug report、transport report、save/load report、devtools export、product preview bundle はすべて有用だが、それだけで operational completion ではない。

## D3. α-0.5 の核心は session-bound local observable runtime

α-0.5 の次 task は、single local runtime session を明示すること。

候補 carrier:

```text
PracticalAlphaSession = {
  session_id,
  package_id,
  checker_report,
  runtime_plan,
  runtime_state,
  membership_frontier,
  witness_refs,
  visible_history,
  event_dag,
  devtools_state,
  savepoints
}
```

この package では implementation しないが、spec / plan で completion condition として固定する。

## D4. α-0.8 の核心は same-session hot-plug

hot-plug report bundle ではなく、session に対する attach / reject / defer が必要。

```text
attach(session, package)
  -> checks
  -> verdict
  -> activation_cut or reject/defer
  -> event DAG update
  -> devtools update
```

## D5. debug / observability は optional ではない

debug / visualization は α-0.5 / α-0.8 の completion condition に含める。後付けの polish ではない。

## D6. typed external host boundary は α-0.5 の missing lower bound

FAQ 015 の指摘に従い、`EchoText` または `AddOne` のような host input -> typed effect -> output / receipt / view の最小 demo を α-0.5 requirement に入れる。

ただし stdio builtin にはしない。

## D7. theory freeze は bounded soundness boundary

今回作る theory specs は final public theorem completion ではない。bounded operational alpha に必要な soundness boundary を固定する。

## D8. no public/U1 decision

packaging / installed binary / host target / first shipped public surface / final catalog breadth は `U1` に残す。

## D9. no PrismCascade / Reversed Library implementation

PrismCascade は別 kernel。Reversed Library は上位 application。今回の α-0.5 / α-0.8 theory freeze に混ぜない。

