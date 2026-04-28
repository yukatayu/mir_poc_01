# samples/generated

この directory は、**non-Lean generated sample artifact** の reserve path です。

## current policy

- source sample は置かない
- heavy disposable artifact はまず external workdir を優先する
- committed generated artifact が必要な場合でも、source sample との混同を避けるため、path と README で generated だと明示する
- current committed generated bridge evidence は `samples/generated/projection-placement/manifest.json`
- `samples/generated/projection-placement/manifest.json` は generated bridge evidence であり、source sample でも final emitted executable program でもない
- `scripts/projection_codegen_samples.py` はこの manifest と live anchor を照合する helper-local / report-local validation floor を与える
- actual emitted executable family、optimizer、deployment planner、final public emitted-program ABI は kept-later gate に残す

## current exception

- `samples/lean/clean-near-end/` の theorem stub は、Lean bridge evidence として existing policy どおり `samples/lean/` 側に置く
