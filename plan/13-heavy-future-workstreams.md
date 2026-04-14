# plan/13 — 重い将来 workstream

## 目的

この文書は、current mainline や current detailed research program に即昇格させない heavy workstream を整理する。
detail-side で今まさに詰める線は `plan/18` に置き、ここでは **まだ重すぎるもの** をまとめる。

## heavy workstream 一覧

### 1. full strong type system

- 主題
  - ownership / lifetime / contract / effect / capability をどこまで型規則へ持ち上げるか。
- なぜ heavy か
  - parser / checker / theorem / runtime を同時に拘束するため。
- current recommendation
  - first attachment candidate inventory までは `plan/18` で進める。
  - full calculus と final syntax はここに残す。

### 2. production-grade theorem proving

- 主題
  - concrete prover binding、proof artifact contract、review workflow finalization。
- なぜ heavy か
  - tool choice と public contract が逆流しやすいため。
- current recommendation
  - semantic-core theorem pilot planning までは `plan/18` で進める。
  - concrete prover / proof object finalization はここに残す。

### 3. production-grade model checking / protocol verification

- 主題
  - concrete model-check tool binding、property language、protocol-family public contract。
- なぜ heavy か
  - runtime policy / shared-space / public checker / host integration と強く絡むため。
- current recommendation
  - projection / property-family reserve planning までは `plan/18` で進める。
  - production contract はここに残す。

### 4. full static analysis / decidability line

- 主題
  - static checker prototype、decidability matrix、complexity note。
- なぜ heavy か
  - surface、IR、proof boundary が揃わないと false commitment になりやすいため。

### 5. backend / codegen / editor surface

- 主題
  - LLVM-family backend、external codegen、editor / LSP / graph tooling。
- なぜ heavy か
  - syntax / lowering / runtime / public packaging を早期固定しやすいため。

### 6. operational shared-space / fabric runtime

- 主題
  - final activation / authority / auth / consistency / fairness catalog と concrete protocol profile。
- なぜ heavy か
  - user specification と operational policy の影響が大きいため。

### 7. raw external integration

- 主題
  - raw FFI、game engine direct binding、production visualizer / substrate integration。
- なぜ heavy か
  - host-facing boundaryを越えた concrete target choice が必要になるため。

### 8. domain / application realization

- 主題
  - synchronized shared-space、collaborative editing、virtual-world 系などの concrete app。
- なぜ heavy か
  - acceptance criteria と evaluation target が user 依存だから。

## current recommendation

- `plan/18` にある typed / theorem / model-check / ordering の detailed program は **今進める**。
- この文書の線は、そこで boundary/pilot plan が固まるまで mainline に混ぜない。
