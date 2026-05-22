# role-split-positive

This sample proves the first bounded same-binary local role split floor.

- `world-server` owns `SugorokuPlace` and launches `main`.
- `world-client` owns `ClientView` and launches `render_preview`.
- `host-adapter` remains a passive endpoint for host read/write boundaries.
- The negative row uses the same accepted projection request but rejects `--target world-client --entry main`.
