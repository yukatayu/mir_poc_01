# samples/practical-alpha1

This root is the current practical alpha-1 first-floor front-door plus first checker-floor, first local-runtime, non-final practical hot-plug sample family, non-final practical transport sample family, first practical devtools export bundle family, first practical local save/load family, first practical product-preview family, and bounded practical integrated workflow carrier.

- It is separate from `samples/alpha/`, which remains the alpha-0 evidence root.
- It is also separate from the operational α-0.5 / α-0.8 / α-0.9 readiness line defined in `specs/19..24` and `plan/45..49`.
- It is not yet the active runnable root for the whole repo.
- `P-A1-01` introduces the first narrow front-door cut here:
  directory-or-file package loading for limited `package.mir.json` inputs.
- `P-A1-02` adds the first checker-floor cut here:
  package fixtures with `alpha_local_checker_input` plus expected checker reports.
- `P-A1-03` adds the first local-runtime cut here:
  checked package fixtures with `alpha_local_runtime_input` plus expected local-runtime reports.
- `P-A1-19` widens the local-runtime evidence and adds the first bounded α-0.5 session carrier here:
  `RUN-03/04` add local missing-capability and missing-witness rejects, and `scripts/practical_alpha05_session.py` / `crates/mir-runtime::practical_alpha05_session` bind checked runtime execution, observer-safe session export, and local save/load frontier into one same-session carrier without yet claiming typed host-I/O completion.
- `P-A1-20` widens the same bounded α-0.5 operational line here:
  `crates/mir-runtime::practical_alpha05_host_io`, example `mir_practical_alpha05_session -- host-io`, `samples/practical-alpha1/packages/oa05-07-add-one-host-io`, and `OA05-07` add one minimal typed external `AddOne` direct execution lane without claiming same-session hot-plug completion.
- `P-A1-21` widens the same session carrier into the bounded α-0.8 operational line here:
  `crates/mir-runtime::practical_alpha08_hotplug_session`, example `mir_practical_alpha05_session -- attach`, `scripts/practical_alpha08_session_hotplug.py`, and `OA08-01..10` add same-session debug / auth / rate-limit / object preview / deferred detach rows without claiming α-0.9 live devtools completion.
- `P-A1-22` widens the same session carrier into the bounded α-0.9 operational line here:
  `crates/mir-runtime::practical_alpha09_devtools`, example `mir_practical_alpha05_session -- export-devtools`, `scripts/practical_alpha09_devtools.py`, and `OA09-01..09` add session-bound event DAG / local route trace / membership timeline / witness relation / hot-plug lifecycle / fallback degradation / save-load timeline / observer-safe redacted view / retention-on-demand trace without claiming final public viewer/telemetry ABI or product-ready alpha-1.
- `P-A1-23` adds a bounded practical α-1 integrated workflow carrier over existing evidence:
  `scripts/practical_alpha1_integrated_workflow.py`, `scripts/tests/test_practical_alpha1_integrated_workflow.py`, and `PA1W-01..08` bind source front-door / checker / same-session runtime / typed host-I/O / hot-plug / save-load / session devtools / product-preview evidence into one developer workflow without claiming final public parser, viewer/telemetry ABI, distributed durable save/load, or product-ready alpha-1.
- `P-A1-04a` adds the first practical hot-plug cut here:
  layer package fixtures with `alpha_local_hotplug_input` plus exact expected hot-plug reports.
- `P-A1-04b` widens the same practical hot-plug cut here:
  attach-time stale-membership reject, attach-time missing-witness reject, and a narrow object package attach preview seam.
- `P-A1-04c` widens the same practical hot-plug cut here:
  explicit deferred detach minimal contract with `operation_kind = detach` and `detach_boundary_ref`.
- `P-A1-05` adds the first practical transport cut here:
  local TCP / Docker Compose TCP package transport with distinct transport-plan/report carriers and exact expected `TR-A1-01..07` reports.
- `P-A1-06` adds the first practical devtools export cut here:
  `scripts/practical_alpha1_export_devtools.py` emits distinct devtools bundles with exact expected `VIS-A1-01/02/06` JSON and a non-final static HTML viewer surface.
- `P-A1-09` widens the same practical devtools cut here:
  `VIS-A1-04` hot-plug lifecycle export over exact practical hotplug reports, limited to attach accepted boundary + membership snapshot + deferred detach boundary.
- `P-A1-12` widens the same practical devtools cut here:
  `VIS-A1-05` fallback degradation export over exact `AV-A1-03` avatar preview evidence, limited to rejected source lane + degraded roles + missing host capability visibility.
- `P-A1-13` widens the same practical devtools cut here:
  `VIS-A1-03` membership timeline export over exact `SL-A1-02` save-load evidence, limited to saved frontier + later live membership advance + restored frontier + stale-membership reject visibility.
- `P-A1-15` widens the same practical devtools cut here:
  `VIS-A1-07` retention-query export over exact `SL-A1-02` save-load evidence, limited to report-local retained-artifact catalog rows plus hit/miss on-demand query trace.
- `P-A1-07` adds the first practical local save/load cut here:
  `scripts/practical_alpha1_save_load.py` emits exact expected `SL-A1-01/02` reports over a distinct save-load plan, a saved local frontier carrier, and a non-final save-load report surface.
- `P-A1-16` widens the same practical local save/load cut here:
  `SL-A1-03` is an exact checker-backed save-load preflight reject row over invalid distributed-cut evidence, and does not build a saved local frontier or attempt runtime execution.
- `P-A1-17` realigns the same practical product-preview cut here:
  `PE2E-06` consumes the exact `SL-A1-03` save-load preflight reject report rather than direct checker evidence, and still does not claim runtime execution or distributed durability.
- `P-A1-10` adds the first practical avatar preview companion cut here:
  `scripts/practical_alpha1_avatar.py` emits exact expected `AV-A1-01/02/03` reports over checked package input, a distinct hotplug plan, and exact hot-plug source reports.
- `P-A1-08` adds the first practical product-preview cut here:
  `samples/practical-alpha1/previews/` plus `scripts/practical_alpha1_product_preview.py` emit exact expected `PE2E-01..07` bundles over preview manifests, exact practical reports, and exact practical devtools bundles.
- `P-A1-11` widens the same practical product-preview cut here:
  `PE2E-08/09` consume exact `AV-A1-02/03` avatar preview reports as thin companion bundles without claiming native execution or unsupported-runtime execution success.
- This front-door is non-final and does not freeze the final public grammar.

## Current package map

- `packages/src-01-minimal-world/`
  - `SRC-01`: minimal world parses
- `packages/src-02-fallback-chain/`
  - `SRC-02`: fallback chain source parses
- `packages/src-03-layer-debug/`
  - `SRC-03`: layer attach source parses
- `packages/src-04-layer-manifest/`
  - `SRC-04`: package manifest source parses
- `packages/src-05-invalid-syntax/`
  - `SRC-05`: invalid syntax rejected
- `packages/chk-*/`
  - `CHK-LIF-01..04`: first lifetime/fallback checker floor
  - `CHK-VAR-01..03`: first contract-variance checker floor
  - `CHK-CUT-01`: first cut-predicate checker floor
  - `CHK-PKG-01/02`: checker-only package-admission preview floor
- `packages/run-*/`
  - `RUN-01`: checked practical Sugoroku-style local dispatch with event DAG export
  - `RUN-02`: stale-membership rejection before local state mutation
  - `RUN-03`: missing-capability rejection before local state mutation
  - `RUN-04`: missing-witness rejection before local state mutation
- `packages/oa05-*/`
  - `OA05-07`: bounded α-0.5 same-session `AddOne` host-I/O direct execution lane with observer-safe receipt summary
- `scripts/practical_alpha09_devtools.py`
  - `OA09-01`: event DAG live/session export
  - `OA09-02`: local route trace over session dispatch records
  - `OA09-03`: membership timeline over current and saved session frontiers
  - `OA09-04`: witness relation export
  - `OA09-05`: hot-plug lifecycle over accepted / rejected / deferred session observations
  - `OA09-06`: fallback degradation without native execution
  - `OA09-07`: save/load timeline over the session savepoint
  - `OA09-08`: observer-safe redacted view with admin/debug kept-later marker
  - `OA09-09`: retention/on-demand trace plus non-final static HTML viewer
- `scripts/practical_alpha1_integrated_workflow.py`
  - `PA1W-01`: source front-door, checker, runtime plan, and local runtime evidence
  - `PA1W-02`: same-session runtime carrier plus typed `AddOne` host-I/O
  - `PA1W-03`: same-session hot-plug lifecycle accepted / rejected / deferred rows
  - `PA1W-04`: local save/load timeline tied to the same session
  - `PA1W-05`: session-bound devtools export and non-final viewer
  - `PA1W-06`: product-preview evidence consumed as exact first-floor evidence
  - `PA1W-07`: negative membership / capability / witness / hot-plug guards
  - `PA1W-08`: explicit non-final product/public boundaries
- `packages/hp-a1-*/`
  - `HP-A1-01`: debug layer attach accepted through manifest-driven hot-plug plan
  - `HP-A1-02`: non-admin debug attach rejected before activation cut
  - `HP-A1-03`: auth layer accepted only through explicit contract-update path
  - `HP-A1-04`: rate-limit layer accepted with explicit preview rejection evidence
  - `HP-A1-05`: incompatible patch rejected before activation cut
  - `HP-A1-04B1`: stale-membership attach rejected before activation cut
  - `HP-A1-04B2`: missing-witness attach rejected before activation cut
  - `HP-A1-06`: object package attach admitted only as a narrow preview seam
  - `HP-A1-07`: detach admitted only as an explicit deferred minimal contract boundary
- `packages/tr-a1-*/`
  - `TR-A1-01`: local TCP accepted through the practical transport carrier
  - `TR-A1-02`: Docker Compose TCP accepted through the practical transport carrier
  - `TR-A1-03`: stale membership rejected before transport delivery mutates world state
  - `TR-A1-04`: missing capability rejected at the practical transport admission boundary
  - `TR-A1-05`: missing witness rejected at the practical transport admission boundary
  - `TR-A1-06`: observer-safe route trace emitted with separated lanes
  - `TR-A1-07`: auth evidence preserved in a lane distinct from transport delivery
- `packages/sl-a1-*/`
  - `SL-A1-01`: one exact local-runtime frontier is saved and resumed through a distinct save-load carrier
  - `SL-A1-02`: one exact local-runtime frontier is saved, later membership freshness advances are injected, and resumed dispatch is rejected
  - `SL-A1-03`: invalid distributed cut is rejected at a distinct save-load preflight boundary by reusing the exact rejected `CHK-CUT-01` checker report
- `packages/av-a1-*/`
  - `AV-A1-01`: placeholder avatar preview over an accepted object-package hot-plug source report
  - `AV-A1-02`: non-final custom Mir avatar preview with `mir_humanoid_runtime_preview` selected and `native_execution_performed = false`
  - `AV-A1-03`: unsupported runtime request remains rejected at the source hot-plug report, while a visible monotone placeholder fallback preview is emitted as companion evidence
- `expected/vis-a1-*.expected.json`
  - `VIS-A1-01`: event DAG + publication / witness / handoff relation export
  - `VIS-A1-02`: observer-safe route trace export
  - `VIS-A1-03`: membership timeline export over exact save-load evidence
  - `VIS-A1-04`: hot-plug lifecycle export over attach accepted and deferred detach boundary reports
  - `VIS-A1-05`: fallback degradation export over rejected-source avatar preview evidence
  - `VIS-A1-06`: redacted observer view with auth-lane separation
  - `VIS-A1-07`: report-local retention query export over exact save-load retained-artifact evidence
- `expected/av-a1-*.expected.json`
  - exact expected non-final practical avatar preview reports over exact hot-plug source reports
- `expected/sl-a1-*.expected.json`
  - `SL-A1-01`: local-only roundtrip resume through a saved local frontier
  - `SL-A1-02`: stale-membership non-resurrection after restore
  - `SL-A1-03`: checker-backed invalid distributed-cut preflight reject before any saved local frontier is built
- `previews/`
  - `PE2E-01`: local full-toolchain preview
  - `PE2E-02`: Docker full-toolchain preview
  - `PE2E-03`: debug-layer companion preview
  - `PE2E-04`: placeholder object companion preview
  - `PE2E-05`: local save/load continue preview
  - `PE2E-06`: invalid distributed save rejected preview over exact `SL-A1-03` save-load preflight reject evidence
  - `PE2E-07`: devtools viewer preview
  - `PE2E-08`: custom-avatar companion preview over `AV-A1-02`
  - `PE2E-09`: unsupported-runtime visible fallback companion preview over `AV-A1-03`
- `expected/pe2e-a1-*.expected.json`
  - exact expected non-final product-preview bundles over the preview manifests

## Current boundary

- The current front-door reads only `package.mir.json`.
- Textual `.mir` source remains later work.
- The current checker floor is non-final and checker-only.
- The current local-runtime floor is also non-final and is limited to `RUN-01..04`.
- The current hot-plug floor is also non-final and is currently limited to `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, and `HP-A1-07`.
- The current transport floor is also non-final and is currently limited to `TR-A1-01..07`.
- The current devtools-export floor is also non-final and is currently limited to `VIS-A1-01/02/03/04/05/06/07`.
- The current local save/load floor is also non-final and is currently limited to `SL-A1-01/02/03`.
- The current avatar-preview floor is also non-final and is currently limited to `AV-A1-01/02/03`.
- The current product-preview floor is also non-final and is currently limited to `PE2E-01..09`.
- The current root is still the first-floor fixture family; `practical_alpha05_session`, `practical_alpha08_session_hotplug`, `practical_alpha09_devtools`, and `practical_alpha1_integrated_workflow` now consume it through same-session α-0.5 / α-0.8 / α-0.9 carriers and a bounded practical workflow, but final public viewer/telemetry ABI and product-ready alpha-1 remain later gates.
- Checked packages are lowered through a distinct runtime-plan carrier before local runtime execution.
- Checked layer packages are lowered through a distinct hotplug-plan carrier before hot-plug report assembly.
- Checked avatar packages are lowered through a distinct hotplug-plan boundary and exact hot-plug source reports before avatar preview report assembly.
- Checked world packages for transport are lowered through a distinct transport-plan carrier before transport report assembly.
- Exact practical reports are lowered through a distinct devtools export bundle before non-final viewer rendering.
- `VIS-A1-07` lowers only report-local retained-artifact catalog rows plus hit/miss query trace from exact `SL-A1-02` save-load evidence.
- Checked world packages for save/load are constrained by a distinct save-load plan and one exact practical local-runtime frontier before saved local frontier/report assembly.
- `SL-A1-03` is a distinct checker-backed save-load preflight reject row; it lowers an exact rejected checker report into a save-load preflight report and does not build a saved local frontier.
- Preview manifests are lowered through exact practical reports, exact practical devtools bundles, and exact avatar preview companion reports before non-final product-preview bundle assembly.
- The integrated workflow lowers existing exact evidence and same-session carrier exports into a non-final workflow report; it is not a public CLI, final viewer API, distributed durable save/load, or product-ready runtime.
- `PE2E-06` now uses exact `SL-A1-03` save-load preflight evidence as its source authority rather than direct checker consumption.
- Object package preview still goes through the distinct hotplug-plan carrier and keeps `object_attach_claimed = false`.
- `AV-A1-02` is a non-final custom Mir avatar preview report, not native execution.
- `AV-A1-03` keeps the source hot-plug report rejected for missing host capability and lowers only a visible monotone placeholder fallback preview.
- `VIS-A1-05` keeps that same rejected source lane and lowers only a visible fallback degradation export bundle; it does not reinterpret the row as native execution or unsupported-runtime execution success.
- `VIS-A1-07` keeps retention-query evidence report-local; it does not reinterpret the row as durable retained-artifact catalog service, cross-session/remote retrieval, or expiry lifecycle completion.
- `CHK-CUT-01` reuse in the save/load lane is limited to orphan-receive checker guard reuse only.
- `SL-A1-03` keeps invalid distributed-cut evidence in the save/load lane without claiming runtime checkpoint execution, saved-frontier build, or distributed durable save/load completion.
- `PE2E-04` is limited to `HP-A1-06` placeholder object preview companion evidence and does not actualize custom Mir avatar runtime or unsupported runtime fallback.
- `PE2E-08` is limited to a custom-avatar companion preview over `AV-A1-02` and does not actualize native execution or same-session runtime attachment.
- `PE2E-09` is limited to an unsupported-runtime visible fallback companion preview over rejected `AV-A1-03` and does not actualize execution success.
- It does not complete the full `specs/18` typed-checking list.
- It does not complete final object package attach, detach runtime lifecycle execution, WAN/federation, distributed durable save/load, stale witness/stale lease non-resurrection completion, native avatar execution, same-session product runtime, full devtools export, full product prototype, or final public runtime/devtools/transport/save-load/package-avatar ABI.
