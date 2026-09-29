import OwnerStatementAdmission
namespace MirroreaProofFirst.OwnerStatementPosition
open MixedOwnerProgram

abbrev Occurrence := Nat × MixedNamedOwnerSource.Assignment
def entryOccurrence : Entry → Option Occurrence
 | .write _ ordinal _ source => some (ordinal,source)
 | _ => none
def writes (entries : List Entry) : List Occurrence := entries.filterMap entryOccurrence
-- Enumerate ALL original items before filtering. Neither assignment rank nor
-- generated install/construction indices substitute for source position.
def indexed : Nat → List Item → List Occurrence
 | _,[] => []
 | start,.ordinary _::rest => indexed (start+1) rest
 | start,.assignment source::rest => (start,source)::indexed (start+1) rest

theorem lower_indexed {control : Nat}
 (typed : Lowers p ctx labels control start used env items entries final) :
 writes entries = indexed start items := by
 induction typed with
 | nil => rfl
 | ordinary _ _ ih => simpa [writes,indexed,entryOccurrence] using ih
 | assignment _ _ _ _ ih => simpa [writes,indexed,entryOccurrence,construction] using ih

theorem indexed_bound (present : entry ∈ indexed start items) :
 start ≤ entry.1 ∧ entry.1 < start + items.length := by
 induction items generalizing start with
 | nil => simp [indexed] at present
 | cons item rest ih =>
   cases item with
   | ordinary item =>
     obtain ⟨lower,upper⟩ := ih present
     simp only [List.length_cons]
     omega
   | assignment source =>
     rcases List.mem_cons.mp present with rfl | tail
     · simp
     · obtain ⟨lower,upper⟩ := ih tail
       simp only [List.length_cons]
       omega

theorem indexed_nodup (start : Nat) (items : List Item) :
 ((indexed start items).map Prod.fst).Nodup := by
 induction items generalizing start with
 | nil => simp [indexed]
 | cons item rest ih =>
   cases item with
   | ordinary item => exact ih _
   | assignment source =>
     simp only [indexed,List.map_cons,List.nodup_cons]
     refine ⟨?_,ih _⟩
     intro present
     obtain ⟨entry,member,same⟩ := List.mem_map.mp present
     have := (indexed_bound member).1
     omega

theorem indexed_source (present : (ordinal,source) ∈ indexed start items) :
 ∃ before after, items = before ++ .assignment source :: after ∧ ordinal = start + before.length := by
 induction items generalizing start with
 | nil => simp [indexed] at present
 | cons item rest ih =>
   cases item with
   | ordinary item =>
     obtain ⟨before,after,parts,position⟩ := ih present
     refine ⟨.ordinary item::before,after,by simp [parts],?_⟩
     simp only [List.length_cons]; omega
   | assignment head =>
     rcases List.mem_cons.mp present with same | tail
     · cases same
       exact ⟨[],rest,rfl,by simp⟩
     · obtain ⟨before,after,parts,position⟩ := ih tail
       refine ⟨.assignment head::before,after,by simp [parts],?_⟩
       simp only [List.length_cons]; omega

theorem stopped_source {control : Nat}
 (typed : Lowers p ctx labels control start used env items entries final)
 (partition : OwnerStatementCursor.Partition entries cursor)
 (stopped : cursor.stopped = some (.write name ordinal level source)) :
 ∃ before after, items = before ++ .assignment source :: after ∧ ordinal = start + before.length := by
 apply indexed_source
 rw [←lower_indexed typed,←partition]
 simp [writes,List.filterMap_append,stopped,entryOccurrence]

theorem indexed_prefix {control : Nat}
 (typed : Lowers p ctx labels control start used env items entries final)
 (partition : OwnerStatementCursor.Partition entries cursor) :
 ∃ suffix, indexed start items = writes cursor.completed ++ suffix := by
 refine ⟨writes (cursor.stopped.toList ++ cursor.remaining),?_⟩
 rw [←lower_indexed typed,←partition]
 simp [writes,List.filterMap_append,List.append_assoc]

theorem admitted_indexed_prefix
 (admitted : OwnerStatementAdmission.Admitted realm view policy store s) :
 (∃ suffix, indexed 0 s.session.program.items = writes s.session.cursor.completed ++ suffix) ∧
 ((writes s.session.entries).map Prod.fst).Nodup := by
 obtain ⟨compiled,cursor,_⟩ := OwnerStatementAdmission.admitted_facts admitted
 obtain ⟨_,_,typed⟩ := compiled
 exact ⟨indexed_prefix typed cursor.1,by rw [lower_indexed typed]; exact indexed_nodup _ _⟩

#print axioms lower_indexed
#print axioms indexed_bound
#print axioms indexed_nodup
#print axioms indexed_source
#print axioms stopped_source
#print axioms indexed_prefix
#print axioms admitted_indexed_prefix
end MirroreaProofFirst.OwnerStatementPosition
