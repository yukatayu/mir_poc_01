# samples/generated

この directory は、**future non-Lean generated sample artifact** の reserve path です。

## current policy

- source sample は置かない
- heavy disposable artifact はまず external workdir を優先する
- committed generated artifact が必要な場合でも、source sample との混同を避けるため、path と README で generated だと明示する

## current exception

- `samples/lean/clean-near-end/` の theorem stub は、Lean bridge evidence として existing policy どおり `samples/lean/` 側に置く
