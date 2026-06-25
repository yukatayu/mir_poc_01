# Sample Blueprint 02 — Membership Chat

```mir
module Surface.MembershipChat

import Surface.WorldCore

capability SendRoomMessage

record ChatLine {
  text: Text,
}

World {
  state last_message[p: Participant]: ChatLine
    init ChatLine { text: "" }
    visible observer_safe fields { text }
}

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }

  when send_chat(text: Text) {
    World {
      last_message[self].text = text
    }
  }
}
```

Expected elaboration:

- join becomes admission request.
- send_chat becomes generated message to World.
- update to visible field produces auto publish/observe.
