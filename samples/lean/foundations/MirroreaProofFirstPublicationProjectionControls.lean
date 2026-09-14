import ActualReference

namespace MirroreaProofFirst.PublicationProjectionControls

-- Deliberately unsafe comparison, not an admitted Publication transition:
-- a proposed control stores an entire post-Session, while a supposed "local"
-- source step is allowed to run before that stored Session is installed.
def main : IO Unit := do
  let before := ReferenceContinuation.drive 7 _root_.begun 0 7
  unless ReferenceSourceData.lookup before.state.values "count" == some (.plain (.integer 2 true)) do
    throw (IO.userError "source did not reach the ordinary assignment cut")
  let some (prepared,_) := ReferenceContinuation.controlInput before 0 0 7 (.leave 1) |
    throw (IO.userError "control preparation was not admitted")
  let afterAssignment := ReferenceContinuation.tick before 0 7
  unless ReferenceSourceData.lookup afterAssignment.state.values "count" == some (.plain (.integer 3 true)) &&
      afterAssignment.state.writes.length == before.state.writes.length+1 do
    throw (IO.userError "interleaved ordinary source assignment was not executed")
  let unsafePublished := prepared
  unless ReferenceSourceData.lookup unsafePublished.state.values "count" == some (.plain (.integer 2 true)) &&
      unsafePublished.state.writes.length+1 == afterAssignment.state.writes.length do
    throw (IO.userError "unsafe saved full-Session install did not exhibit the expected lost write")
  IO.println "PROJECTION_COUNTEREXAMPLE_OK prepared full-Session control plus interleaved ordinary count assignment loses count=3 and its real source write when the saved Session is installed."
  IO.println "Excluded from the current publication model; a split metadata/local implementation needs a proved projection/merge or protected preparation, not whole-state overwrite."

end MirroreaProofFirst.PublicationProjectionControls
