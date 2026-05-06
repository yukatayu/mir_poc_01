# MembershipChat

`MembershipChat` imports `WorldCore` and adds room membership/chat behavior.

- current executable input: `package.mir.json`
- representative source: `membership-chat.mir`
- current execution note: `run-local` executes one bounded `EchoText("Taro") -> "Hello, Taro!"` host boundary lane and records observer-safe host-I/O evidence; this is not a final text chat service or stdio builtin

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/membership-chat --format json
```
