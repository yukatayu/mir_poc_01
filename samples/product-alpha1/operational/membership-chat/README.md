# MembershipChat

`MembershipChat` imports `WorldCore` and adds room membership/chat behavior.

- current executable input: `package.mir.json`
- representative source: `membership-chat.mir`
- current execution note: typed room text I/O is still a declared boundary; this package is runnable as a product alpha session carrier but does not claim final text chat execution

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/membership-chat --format json
```
