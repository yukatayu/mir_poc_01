import MirroreaProofFirstQualifiedReceipt

namespace MirroreaProofFirst.CustodyPublication

-- One private activation's coordinator; replica fields contain only the owner
-- projection. This does not serialize a Session or mandate a global publisher.
def image (s : QualifiedCustody.State p a) : OwnerImage.Image p a :=
  QualifiedPublication.image s.privateState

abbrev State (n p a : Nat) :=
  PublicationImage.State n (QualifiedCustody.State p a) (QualifiedPublication.Command p a) (OwnerImage.Image p a)

abbrev Reached (n revision : Nat) (initial : QualifiedCustody.State p a) (s : State n p a) :=
  PublicationImage.Reached image QualifiedCustody.evaluate n revision initial s

theorem payload_rooted
    (root : QualifiedCustody.Rooted realm view policy identity initial)
    (path : PublicationPayload.Reached QualifiedCustody.evaluate n revision initial s) :
    QualifiedCustody.Rooted realm view policy identity s.current := by
  induction path with
  | initial => exact root
  | step previous step ih =>
    cases step with
    | action allowed =>
      have refined := PublicationPayload.action_refines
        (fun before after => ∃ command, QualifiedCustody.evaluate before command = some after)
        (fun _ command _ accepted => ⟨command,accepted⟩)
        (PublicationPayload.reached_invariant previous) allowed
      rcases refined with unchanged | ⟨command,computed⟩
      · rw [unchanged]; exact ih
      · exact .step ih computed

theorem current_rooted
    (root : QualifiedCustody.Rooted realm view policy identity initial)
    (path : Reached n revision initial s) :
    QualifiedCustody.Rooted realm view policy identity s.current := by
  obtain ⟨abstract,reached,rfl⟩ := PublicationImage.reached_lifts path
  exact payload_rooted root (PublicationUse.reached_payload reached)

theorem guarded_entry_has_publication
    (nonempty : 0 < n) (path : Reached n revision initial s)
    (settled : s.barrier.announced = s.barrier.published) (free : ∀ i, s.held i = none)
    (entry : QualifiedCustody.Entry s.current command value) :
    ∃ next, PublicationImage.run image QualifiedCustody.evaluate s
        ((PublicationProgress.normal n (s.barrier.published+1)).map (PublicationProgress.lift command)) = some next ∧
      Reached n revision initial next ∧ next.current = value ∧
      next.barrier.published = s.barrier.published+1 ∧
      ∀ i, next.cached i = image value ∧ next.held i = none ∧
        PublicationImage.check QualifiedCustody.evaluate next (.enter i) = true :=
  PublicationImage.normal_succeeds nonempty path settled free (QualifiedCustody.evaluate_exact.mpr entry)

-- A matching typed envelope is first checked against the complete private
-- receiver. Its admitted entry has an actual normal publication path. Origin
-- and the authority to deliver this command remain transport obligations.
theorem receipt_has_publication
    (nonempty : 0 < n) (path : Reached n revision initial s)
    (settled : s.barrier.announced = s.barrier.published) (free : ∀ i, s.held i = none)
    (accepted : QualifiedReceipt.accept context s.current envelope = some value) :
    ∃ result next, envelope.result = .value result ∧
      PublicationImage.run image QualifiedCustody.evaluate s
        ((PublicationProgress.normal n (s.barrier.published+1)).map
          (PublicationProgress.lift (.receive envelope.ticket result))) = some next ∧
      Reached n revision initial next ∧ next.current = value := by
  obtain ⟨_,_,result,atResult,entry⟩ := QualifiedReceipt.accept_exact.mp accepted
  obtain ⟨next,ran,reached,current,_,_⟩ := guarded_entry_has_publication nonempty path settled free entry
  exact ⟨result,next,atResult,ran,reached,current⟩

theorem held_evaluation {n p a : Nat} {s : State n p a}
    {initial : QualifiedCustody.State p a} (path : Reached n revision initial s)
    (endpoint : Fin n) (pair : Nat × OwnerImage.Image p a) (held : s.held endpoint = some pair)
    (assigned : OwnerEvaluator.Assignment p) (ticket : InvocationBoundary.Ticket) (value : Int) :
    pair.1 = s.barrier.published ∧
      (OwnerEvaluator.run assigned pair.2 ticket = .value value ↔
        OwnerEvaluator.Admitted assigned (image s.current) ticket ∧
          InstancePrograms.Machine.Executes ticket.definition.code ticket.argument value) := by
  obtain ⟨revisionAt,current⟩ := PublicationImage.held_current path endpoint pair held
  rw [current]
  exact ⟨revisionAt,OwnerEvaluator.result_exact _ _ _ _⟩

-- Every replica install chooses retained data by the checked publication
-- revision. A separately supplied image is not an installation argument.
-- Native currentness still needs this publisher's authentic message/ack path.
theorem install_uses_published_image
    {s : State n p a} {next : State n p a}
    (accepted : PublicationImage.execute image QualifiedCustody.evaluate s
      (.administrative (.install endpoint r)) = some next) :
    ∃ saved, s.images r = some saved ∧ next.cached endpoint = saved := by
  unfold PublicationImage.execute at accepted
  split at accepted
  · rename_i checked
    have existsImage : (s.images r).isSome = true := by
      simp only [PublicationImage.check,PublicationImage.extra,Bool.and_eq_true] at checked
      exact checked.2.1
    obtain ⟨saved,atSaved⟩ := Option.isSome_iff_exists.mp existsImage
    cases accepted
    exact ⟨saved,atSaved,by simp [PublicationImage.apply,atSaved,PublicationPayload.put]⟩
  · cases accepted

#print axioms payload_rooted
#print axioms current_rooted
#print axioms guarded_entry_has_publication
#print axioms receipt_has_publication
#print axioms held_evaluation
#print axioms install_uses_published_image
end MirroreaProofFirst.CustodyPublication
