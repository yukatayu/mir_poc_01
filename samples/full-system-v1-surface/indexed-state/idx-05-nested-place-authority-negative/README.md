# IDX-05 Nested Place Authority Negative

A nested `S { ... }` block inside a role instance is not an ambient authority
switch. It must elaborate to an owner-directed generated request before it can
write S-owned indexed state.
