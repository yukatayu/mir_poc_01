import MirroreaProofFirstInvocationBoundary

namespace MirroreaProofFirst.InvocationContract
open InstancePrograms InstanceState WorldProjection InvocationBoundary

-- The delivered value, not merely some possible successful value, satisfies
-- the current definition and the instance's retained interface after replacement.
theorem delivered_contract (s : State d p n) (v : AuthorityView a) (t : Ticket) (result : Int)
    (valid : Valid s) (accepted : resultCheck s v t result = true) :
    Output t.definition.contract result ∧
    (∃ member key place, Current s v t member key place ∧ Output (s.instances key).interface result) ∧
    Machine.lo ≤ t.argument ∧ t.argument ≤ Machine.hi ∧
    Machine.lo ≤ result ∧ result ≤ Machine.hi := by
  have parts := accepted
  simp only [resultCheck,Bool.and_eq_true,decide_eq_true_eq,execute] at parts
  have checked : check s v t = true := parts.1
  have actual : Machine.run t.definition.code t.argument = some result := parts.2
  obtain ⟨value,execution,output⟩ := current_execution s v t valid checked
  have same : value = result := Option.some.inj ((Machine.run_exact _ _ _).mpr execution |>.symm.trans actual)
  subst value
  obtain ⟨member,key,place,current⟩ := check_parts s v t checked
  have cur := checkAt_sound s v t member key place current
  have definition := cur.2.2.2.2.2.1
  have narrowed := valid.interfaces key
  have out : Output (s.instances key).interface result := by
    have ofCurrent : Output (s.definitions (s.instances key).definition).contract result := definition ▸ output
    exact ⟨Int.le_trans narrowed.lower ofCurrent.1,Int.le_trans ofCurrent.2 narrowed.upper⟩
  have numerical := (ContractExport.CheckedArithmetic.denotes_math execution.2).2.2
  exact ⟨output,⟨member,key,place,cur,out⟩,execution.1.1,execution.1.2,numerical.1,numerical.2⟩

#print axioms delivered_contract
end MirroreaProofFirst.InvocationContract
