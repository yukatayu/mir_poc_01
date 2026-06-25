# Sample Blueprint 04 — Role Admission

```mir
module Surface.Admission

role BrowserClient {
  supports renderer.pose_v1
  supports devtools.observer_safe
}

place WorldAdmission
capability AdmitParticipant
capability ObserveWorld
capability SendRoomMessage

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }
}

WorldAdmission {
  when admit(req: JoinRequest) {
    require valid_principal(req.principal)
    require req.role == BrowserClient

    grant Member(World, req.principal)
    grant ObserveWorld(req.principal)
    grant SendRoomMessage(req.principal)

    publish participant_joined(req.principal)
      produces witness admission_witness
  }
}
```

Expected:

- Role claim not authority.
- WorldAdmission grants capabilities.
- spoofed server role rejected unless capability granted.
