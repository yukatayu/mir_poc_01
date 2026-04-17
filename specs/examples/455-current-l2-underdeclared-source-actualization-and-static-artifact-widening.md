# 455 — current L2 underdeclared source actualization and static artifact widening

## 目的

fixture-level で source of truth を already 持っていた underdeclared family のうち、

- lineage assertion omission
- declared target omission

を current source parser / lowerer convenience cut に引き上げ、
current authored sample corpus と helper-local verifier artifact route で比較可能にする。

## current judgment

1. current source parser / lowerer convenience cut は、次の omission form を受けてよい。
   - `fallback successor`
     - lineage assertion omission を source-authored underdeclared row として保持する。
   - `option name on capability read lease live`
     - declared target omission を source-authored underdeclared row として保持する。
2. 上の widening は convenience cut であり、final grammar adoption を意味しない。
3. underdeclared source row は `samples/current-l2/` に authored sample として昇格してよい。
   - `e5-underdeclared-lineage`
   - `e12-underdeclared-target-missing`
4. underdeclared source row でも static gate verdict が `underdeclared` なら、
   helper-local `verification_preview` / `artifact_preview` は `fixture_static_cluster` route を reached として出してよい。
5. current helper-local widening では、underdeclared static cluster の obligation family を malformed static cluster と同じ first cut に揃えてよい。
   - `canonical_normalization_law`
   - `no_re_promotion`
6. `underdeclared` は引き続き static floor の verdict であり、dynamic `Reject` や hidden runtime repair へ押し込まない。

## current non-goals

- underdeclared omission form を final public syntax として凍らせること
- `underdeclared` 専用の final artifact schema を決めること
- theorem discharge / model-check property language の final public contract を決めること
- missing lineage / missing target family を beyond-current fixed subset まで一般化すること
