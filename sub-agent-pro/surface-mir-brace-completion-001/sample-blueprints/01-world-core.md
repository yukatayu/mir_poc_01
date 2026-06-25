# Sample Blueprint 01 — World Core

```mir
module Surface.WorldCore

role Participant

role BrowserClient {
  supports renderer.pose_v1
  supports devtools.observer_safe
}

principal self

place World
place WorldAdmission

record ParticipantInfo {
  joined_epoch: UInt64,
  active: Bool,
}

World {
  state participant[p: Participant]: ParticipantInfo
    init ParticipantInfo { joined_epoch: 0, active: false }
    visible observer_safe fields { active }
}
```

Expected:

- parses.
- indexed state owner = World.
- keyspace = Participant.
- visible field = active.
