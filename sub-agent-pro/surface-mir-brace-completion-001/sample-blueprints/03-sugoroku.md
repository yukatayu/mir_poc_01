# Sample Blueprint 03 — Sugoroku

```mir
module Surface.Sugoroku

import Surface.MembershipChat

capability RollDice
capability HandoffTurn

record Player {
  position: Int64,
}

World {
  state player[p: Participant]: Player
    init Player { position: 0 }
    visible observer_safe fields { position }

  state dice_owner: Participant
}

fn add_position(pos: Int64, draw: Int64) -> Int64 {
  return pos + draw
}

BrowserClient[self] {
  when roll(draw: Int64) {
    World {
      require dice_owner == self
      player[self].position = add_position(player[self].position, draw)
      // Core elaboration may publish position_changed and require witness for handoff policy.
    }
  }
}
```

Expected:

- Mir-owned arithmetic.
- Indexed state access.
- Capability check for RollDice.
- Auto publish of visible position.
