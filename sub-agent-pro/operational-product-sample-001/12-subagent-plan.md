# 12 — sub-agent review plan

Use available sub-agents. If specific names are not available, use equivalent reviewer roles.

## 1. Theory reviewer

Focus:

- Mir / Mirrorea separability
- stdio non-core
- fallback / lifetime non-overclaim
- auth/layer algebra
- atomic_cut / save/load / R2 quiescent-save
- portal/spatial future boundary

Questions:

- Does any text claim final grammar/API/ABI?
- Does any text treat native bundle as direct native codegen?
- Does portal/spatial skeleton claim implementation?

## 2. Sample architect reviewer

Focus:

- WorldCore -> MembershipChat -> Sugoroku import chain
- package dependency semantics
- representative `.mir` vs executable `package.mir.json`
- developer workflow clarity

Questions:

- Can an outside developer understand what to run?
- Are sample names stable and meaningful?
- Are planned-only future samples clearly marked?

## 3. Runtime/toolchain reviewer

Focus:

- `mirrorea-alpha` commands
- session id / session store
- attach/save/load/transport/devtools/bundle flow
- Docker behavior

Questions:

- Are commands executable?
- Are session names consistent?
- Does Docker skip avoid overclaim?

## 4. Devtools reviewer

Focus:

- diagrams and panels
- observer-safe output
- event DAG / route graph / membership timeline / hot-plug / save-load panels

Questions:

- Can a developer see what happened?
- Are raw witness/auth/capability secrets hidden?
- Are source/import and projection graphs represented?

## 5. Backend/native reviewer

Focus:

- native host bundle vs direct LLVM/backend
- projection profile
- packet / FFI boundary
- future backend wording

Questions:

- Is LLVM/codegen overclaimed?
- Are boundary schemas preserved?
- Are native execution and provenance/safety separated?

## 6. Security/auth reviewer

Focus:

- capabilities
- witness requirements
- auth policy
- rate-limit failure row
- native policy
- redaction

Questions:

- Is auth stack explicit?
- Are hidden privileges avoided?
- Are signatures described only as provenance?

## 7. Docs/source hierarchy reviewer

Focus:

- specs/plan/progress/tasks/samples_progress/docs/reports consistency
- source hierarchy validation
- report schema
- sample root taxonomy

Questions:

- Does `samples/product-alpha1/operational/` appear in samples docs?
- Does `samples/alpha/` remain alpha-0 evidence root?
- Are progress claims conservative?

## 8. Sub-agent failure handling

If an agent does not return:

- do not treat missing review as approval
- perform local focused review
- record missing agent / local review in report
