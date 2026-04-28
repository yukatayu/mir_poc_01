# plan/20 — projection / placement current roadmap

## 目的

Mirrorea future-axis の phase 12 `Projection / placement` を、
repo-local current layer で手戻りを増やさずに進めるための roadmap を置く。

ここで固定するのは、system-wide source と place-specific program の区別、
place split、validity checklist、helper/report-local preview floor、emitted-program gate の close 条件、
generated-artifact reserve policy、stop line である。
final projection IR、generated place-specific program emitter、optimizer、cross-place equivalence checker は固定しない。

## current invariant

- Mirrorea は system-wide source を後で place-specific program に projection できる性質を保つ。
- source principal を server / participant / adapter / visualizer のどれかに早期固定しない。
- projection でも auth / membership / capability / witness / visualization / telemetry を潰さない。

## place split

current docs-first line では、少なくとも次を分けて読む。

- server / authoritative room path
- participant path
- domain runtime place path
- external adapter path
- visualizer / observer path

`Place` は participant や principal と同一ではなく、
queue / state / capability / visibility / observation frontier を持つ execution locus として扱う。

## current evidence anchors

- Sugoroku helper
  - `WorldServerPlace`
  - `ParticipantPlace[*]`
  - `SugorokuGamePlace#1`
  - `message_envelopes`
  - `visualization_views`
  - `projection_view`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
- clean near-end order-handoff
  - `05_delegated_rng_service`
  - `provider_boundary`
  - `provider_boundary_redacted_flow`
  - `cross_place_projection`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- docs-first phase 9 planned family
  - `samples/not_implemented/typed-external-boundary/`

## current executable preview floor

- helper-local preview:
  Sugoroku `03_roll_publish_handoff` の `projection_view` は、
  `SugorokuWorldSource#1` から `WorldServerPlace` / `SugorokuGamePlace#1` / `ParticipantPlace[*]`
  への place split、membership frontier、observer view refs、transport seam `local_queue` を
  evidence-oriented に見せる current preview である。
- report-local preview:
  clean near-end `05_delegated_rng_service` の `cross_place_projection` は、
  authority placement と provider placement を分離したまま provider-boundary lane を読む
  report-local inventory である。
- どちらも final emitted place program ではない。
  executable widening の current scope は projection validity を collapse せずに preview することであり、
  optimizer / scheduler / deployment planner を導入することではない。

## current emitted-program gate

`P3` current package が close するのは、
helper/report-local preview floor の上に actual emitted place program を実装したときではない。
current close 条件は、少なくとも次を docs-first に固定したときである。

1. current preview floor が helper-local / report-local evidence であり、
   final emitted place program ではないこと
2. projection validity report の minimum contents
3. generated place-specific program family の reserve policy
4. actual emitted executable family は `P3` ではなく `P15` family 以降へ残すこと。current `P15` first cut では committed generated bridge evidence only を actualize すること

したがって `P3 close` は emitted-program implementation closeout ではなく、
**preview floor と later emitted-program family の boundary fixation** である。

## projection validity checklist

projection / placement plan を current line に上げるときは、少なくとも次を確認する。

1. source principal を early server/client split に潰していないか
2. authority placement が explicit か
3. membership frontier が explicit か
4. witness path が preserved されるか
5. label / authority / redaction が visualization path まで preserved されるか
6. adapter path で transport / auth / witness が collapse していないか
7. failure / rejection / detach stop line が place-local に explicit か
8. helper-local preview を final public projection API と誤読させていないか
9. provider placement と authority placement を collapse していないか

## projection validity report minimum

current docs-first line では、projection validity report の exact field name は未固定でも、
少なくとも次の情報群を separate に説明できなければならない。

1. system-wide source ref
2. place split refs
3. authority placement refs
4. membership frontier refs
5. witness path refs
6. adapter transport seam refs
7. visualization / telemetry label-authority-redaction refs
8. place-local failure / rejection / detach stop-line refs
9. current artifact boundary:
   helper/report-local preview only であること
10. kept-later gates:
    emitted program emitter、optimizer、cross-place equivalence checker、deployment planner

これは final public schema ではない。
current `P3` close で固定するのは **minimum report categories** だけである。

## generated artifact reserve policy

- `samples/generated/` は generated place-specific program family を含む reserve path であり、current repo では committed generated bridge evidence もここに置く。
- source sample は `samples/generated/` に置かない。
- heavy disposable emitted artifact は repo root ではなく external workdir を優先する。
- committed generated artifact が必要な場合でも、
  generated であること、source sample ではないこと、どの source / report と結びつくかを明示する。
- `P15` current first cut では `samples/generated/projection-placement/manifest.json` と live-anchor alignment helper を committed generated bridge evidence として actualize する。
- actual emitted executable family、projection emitter、optimizer、deployment planner は kept-later gate に残す。

## current P15 first-cut actualization

- committed generated bridge evidence:
  `samples/generated/projection-placement/manifest.json`
- validation helper:
  `scripts/projection_codegen_samples.py`
- current artifact IDs:
  `P15-GEN-01..04`
- current closeout surface:
  `generated_bridge_artifact_inventory`
  `generated_reserve_inventory`
  `equivalence_review_categories`
  `validation_floor`
- current stop line:
  manifest bridge evidence only。final emitted executable program ではない。

## current command set

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug visualization --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-01 --format json
python3 scripts/projection_codegen_samples.py run P15-GEN-03 --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
find samples/generated -maxdepth 3 -type f | sort
```

## stop line

- final projection IR
- actual emitted executable family beyond manifest bridge
- generated place-specific program emitter
- placement optimizer
- cross-place equivalence checker
- production scheduler / deployment planner

## next relation

projection validity line は `plan/21-hotplug-attachpoint-roadmap.md` と
`plan/22-network-transport-roadmap.md` の両方を下支えする。
source-to-place split を transport widening や hot-plug activation と collapse しないことが current relation である。
