open Lean
def check (name : String) (action : IO Unit) (error : Option String) : IO Unit := do
  let result ← try action; pure (none : Option String) catch caught => pure (some caught.toString)
  match result,error with
  | none,none => IO.println s!"CONTROL_EXPECTED_ACCEPT {name}"
  | some actual,some expected =>
    unless (actual.splitOn expected).length > 1 do throw (IO.userError s!"wrong rejection {name}: {actual}")
    IO.println s!"CONTROL_EXPECTED_REJECT {name}: {actual}"
  | _,_ => throw (IO.userError s!"unexpected result {name}: {result}")

