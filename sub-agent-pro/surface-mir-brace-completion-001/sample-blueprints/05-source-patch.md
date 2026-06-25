# Sample Blueprint 05 — Source Patch Hot-Plug

```mir
module Patch.DebugLamp

import Surface.WorldCore

record DebugLamp {
  enabled: Bool,
}

World {
  state lamp[p: Participant]: DebugLamp
    init DebugLamp { enabled: true }
    visible observer_safe fields { enabled }
}
```

Command target:

```bash
mirrorea-alpha patch-source session#world Patch.DebugLamp.mir --format json
```

Expected:

- parse accepted.
- typecheck accepted.
- Core IR diff visible.
- HotPlugRequest accepted.
- activation_cut emitted.
- devtools shows new indexed state.
