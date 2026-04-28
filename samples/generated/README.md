# samples/generated

この directory は、**future non-Lean generated sample artifact** の reserve path です。

## current policy

- source sample は置かない
- heavy disposable artifact はまず external workdir を優先する
- committed generated artifact が必要な場合でも、source sample との混同を避けるため、path と README で generated だと明示する
- projection / placement 由来の emitted place-specific program family も、この reserve path を later package (`P15`) で使う
- current `P3` close では generated artifact policy だけを固定し、actual emitted program はまだ置かない

## current exception

- `samples/lean/clean-near-end/` の theorem stub は、Lean bridge evidence として existing policy どおり `samples/lean/` 側に置く
