import OwnerStatementPosition
namespace MirroreaProofFirst.OwnerStatementCompletion
open MixedOwnerContinuation MixedOwnerProgram

-- This witness describes one actual executor transition, including the inbox
-- reply and the complete saved request. It is not a supplied Boolean completion.
def Accepted (s next : Session p a) : Prop :=
 ∃ name ordinal control source held received rest after written,
 s.status = .waiting ∧ s.cursor.stopped = some (.write name ordinal control source) ∧
 MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting = some held ∧
 held.statement = source ∧
 held.saved.origin = ⟨source.site.document,source.site.byteOffset,s.activation,ordinal,control⟩ ∧
 s.state.inbox = received::rest ∧ received.pending = held.saved ∧
 MixedOwnerSourceIssue.acknowledge s.state.source received.pending received.evidence = some after ∧
 written ∈ s.state.owner.history ∧ written.pending = held.saved ∧
 next.state = {s.state with source := after,inbox := rest} ∧
 next.cursor.completed = s.cursor.completed ++ [.write name ordinal control source] ∧
 next.state.owner = s.state.owner

theorem owner_tick_accepted {control : Nat}
 (binding : OwnerStatementBinding.Matches s)
 (provenance : MixedOwnerSourceTrace.Provenance s.state)
 (phase : s.status = .waiting)
 (stopped : s.cursor.stopped = some (.write name ordinal control source))
 (ready : (complete s).status = .ready) : Accepted s (tick s member principal) := by
 obtain ⟨held,waiting,statement,origin⟩ := binding name ordinal control source phase stopped
 obtain ⟨next,receive,state⟩ := OwnerStatementAcceptance.owner_complete_parts waiting ready
 obtain ⟨received,rest,after,box,ack,equal⟩ := MixedOwnerSourceTrace.receive_parts receive
 obtain ⟨written,history,saved⟩ := (OwnerStatementAcceptance.owner_complete_committed provenance waiting ready).1
 refine ⟨name,ordinal,control,source,held,received,rest,after,written,
   phase,stopped,waiting,statement,origin,box,
   OwnerStatementAcceptance.acknowledged_saved_equal ack waiting,ack,history,saved,?_,?_,?_⟩
 · simpa only [tick,MixedSourceCursor.tick,phase] using state.trans equal
 · simp [tick,MixedSourceCursor.tick,phase,ready,stopped]
 · exact OwnerStatementHistory.tick_owner

theorem tick_delta (binding : OwnerStatementBinding.Matches s)
 (provenance : MixedOwnerSourceTrace.Provenance s.state) :
 OwnerStatementPosition.writes (tick s member principal).cursor.completed =
   OwnerStatementPosition.writes s.cursor.completed ∨ Accepted s (tick s member principal) := by
 cases phase : s.status with
 | failed reason => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase])
 | ready =>
   cases rest : s.cursor.remaining with
   | nil => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,rest])
   | cons entry tail =>
     by_cases ready : (advance s member principal entry).status = .ready
     · cases entry with
       | ordinary item => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,rest,ready,
           OwnerStatementPosition.writes,OwnerStatementPosition.entryOccurrence,List.filterMap_append])
       | install name control source => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,rest,ready,
           OwnerStatementPosition.writes,OwnerStatementPosition.entryOccurrence,List.filterMap_append])
       | write name ordinal control source => exact False.elim (OwnerStatementHistory.write_not_ready ready)
     · exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,rest,ready])
 | waiting =>
   by_cases ready : (complete s).status = .ready
   · cases stopped : s.cursor.stopped with
     | none => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,ready,stopped])
     | some entry =>
       cases entry with
       | ordinary item => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,ready,stopped,
           OwnerStatementPosition.writes,OwnerStatementPosition.entryOccurrence,List.filterMap_append])
       | install name control source => exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,ready,stopped,
           OwnerStatementPosition.writes,OwnerStatementPosition.entryOccurrence,List.filterMap_append])
       | write name ordinal control source => exact Or.inr (owner_tick_accepted binding provenance phase stopped ready)
   · exact Or.inl (by simp [tick,MixedSourceCursor.tick,phase,ready])

theorem accepted_projection (accepted : Accepted s next) :
 ∃ ordinal source, OwnerStatementPosition.writes next.cursor.completed =
   OwnerStatementPosition.writes s.cursor.completed ++ [(ordinal,source)] := by
 obtain ⟨_,ordinal,_,source,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,completed,_⟩ := accepted
 exact ⟨ordinal,source,by simp [completed,OwnerStatementPosition.writes,
   List.filterMap_append,OwnerStatementPosition.entryOccurrence]⟩

-- Metadata binding failure has a precise frame. The actual wrapper prepares a
-- pure candidate and publishes none of its tentative effects on this branch.
theorem refused_frame (s : OwnerMetadataSession.State p a) (candidate : Session p a) :
 (OwnerMetadataSession.refused s candidate).session.state = s.session.state ∧
 (OwnerMetadataSession.refused s candidate).session.cursor = s.session.cursor ∧
 (OwnerMetadataSession.refused s candidate).session.status = .failed .rejected ∧
 (OwnerMetadataSession.refused s candidate).session.control = candidate.control ∧
 (OwnerMetadataSession.refused s candidate).registry = s.registry ∧
 (OwnerMetadataSession.refused s candidate).packet = s.packet := by
 exact ⟨rfl,rfl,rfl,rfl,rfl,rfl⟩

theorem metadata_tick_route :
 (OwnerMetadataSession.tick s member principal).session = tick s.session member principal ∨
 OwnerMetadataSession.tick s member principal =
   OwnerMetadataSession.refused s (tick s.session member principal) := by
 unfold OwnerMetadataSession.tick
 dsimp only
 split
 · exact Or.inl rfl
 · split
   · split
     · split
       · exact Or.inl rfl
       · exact Or.inr rfl
     · exact Or.inr rfl
   · split
     · exact Or.inr rfl
     · exact Or.inl rfl

theorem admitted_tick_delta (admitted : OwnerStatementAdmission.Admitted realm view policy store s) :
 OwnerStatementPosition.writes (OwnerMetadataSession.tick s member principal).session.cursor.completed =
   OwnerStatementPosition.writes s.session.cursor.completed ∨
 Accepted s.session (OwnerMetadataSession.tick s member principal).session := by
 obtain ⟨_,_,binding,_,joint⟩ := OwnerStatementAdmission.admitted_facts admitted
 rcases metadata_tick_route (s:=s) (member:=member) (principal:=principal) with same | refused
 · rw [same]; exact tick_delta binding joint.1.2.2.2
 · exact Or.inl (by rw [refused]; rfl)

#print axioms owner_tick_accepted
#print axioms tick_delta
#print axioms accepted_projection
#print axioms refused_frame
#print axioms metadata_tick_route
#print axioms admitted_tick_delta
end MirroreaProofFirst.OwnerStatementCompletion
