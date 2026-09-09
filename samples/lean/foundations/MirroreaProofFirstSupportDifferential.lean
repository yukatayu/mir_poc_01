import MirroreaProofFirstSupport
open MirroreaProofFirst.Support

def choice : Nat → Formula (Fin 3)
  | 0 => .top
  | 1 => .bottom
  | 2 => .ref 0
  | 3 => .ref 1
  | 4 => .ref 2
  | 5 => allOf [.ref 0, .ref 1]
  | _ => anyOf [.ref 1, .ref 2]

def main : IO Unit := do
  let out ← IO.getStdout
  for code in List.range 343 do
    let forms := fun (k : Fin 3) => choice ((code / 7^k.val) % 7)
    for mask in List.range 8 do
      let eligible := fun (k : Fin 3) => (mask / 2^k.val) % 2 == 1
      let snapshot := Snapshot.mk forms eligible
      let live := snapshotLive snapshot
      let rank := snapshotRank snapshot
      let nodes := List.finRange 3
      if !checkSnapshot snapshot live rank then
        throw (IO.userError "Generated certificate rejected")
      let bits := nodes.foldl (fun total k => total + if live k then 2^k.val else 0) 0
      let ranks := nodes.map (fun k => if live k then toString (rank k) else "-1")
      out.putStrLn s!"{code},{mask},{bits},{String.intercalate "," ranks}"
