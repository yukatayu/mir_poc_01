use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use mir_ast::textual_alpha::{
    AstBinaryOp, AstBindValue, AstContractClause, AstExpr, AstExprKind, AstFunction, AstImport,
    AstPerformCall, AstRecord, AstRecordConstructField, AstStmt, AstTopLevel, AstTransition,
    AstType, AstUnaryOp, SourceSpan, TextualMirDiagnostic, TextualMirModuleResolution,
    parse_textual_mir_report_path, resolve_textual_mir_module_reference,
};

use super::typed_ir::{
    FullSystemV1CheckReport, FullSystemV1Obligation, TypedBinaryOp, TypedBindValue,
    TypedCapabilityDecl, TypedContractClause, TypedEffectDecl, TypedEffectOutput, TypedExpr,
    TypedExprKind, TypedFunction, TypedMirImport, TypedMirModule, TypedParam, TypedPerformCall,
    TypedRecordConstructField, TypedRecordField, TypedRecordType, TypedStmt, TypedTransition,
    TypedType, TypedUnaryOp,
};

#[derive(Clone)]
struct LoadedModule {
    path: PathBuf,
    module: mir_ast::textual_alpha::AstModule,
    import_paths: BTreeMap<String, PathBuf>,
}

struct DeclRef<'a, T> {
    module: &'a LoadedModule,
    value: &'a T,
}

impl<'a, T> Copy for DeclRef<'a, T> {}

impl<'a, T> Clone for DeclRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Clone)]
struct TypeBinding {
    ty: TypedType,
    mutable: bool,
}

#[derive(Clone)]
struct FunctionSignature {
    module_path: String,
    function_name: String,
    input_type: TypedType,
    output_type: TypedType,
}

struct ModuleAnalysis {
    module: TypedMirModule,
    diagnostics: Vec<TextualMirDiagnostic>,
}

pub fn check_textual_mir_module_path(path: impl AsRef<Path>) -> FullSystemV1CheckReport {
    let path = normalize_path(path.as_ref());
    let mut loaded = BTreeMap::new();
    let mut visiting = BTreeSet::new();
    let mut diagnostics = Vec::new();

    if !load_module_graph(&path, &mut loaded, &mut visiting, &mut diagnostics) {
        diagnostics.sort_by_key(|row| (row.span.line, row.span.column, row.code.clone()));
        return FullSystemV1CheckReport {
            accepted: false,
            module: None,
            accepted_obligations: Vec::new(),
            residual_obligations: Vec::new(),
            diagnostics,
            final_public_api_frozen: false,
        };
    }

    let Some(current) = loaded.get(&path) else {
        diagnostics.push(diagnostic(
            "module_not_loaded",
            format!("checker could not load `{}`", path.display()),
            zero_span(),
        ));
        return FullSystemV1CheckReport {
            accepted: false,
            module: None,
            accepted_obligations: Vec::new(),
            residual_obligations: Vec::new(),
            diagnostics,
            final_public_api_frozen: false,
        };
    };

    let mut visited = BTreeSet::new();
    let mut closure_diagnostics = Vec::new();
    let mut root_module = None;
    collect_module_closure(
        &path,
        current,
        &loaded,
        &mut visited,
        &mut closure_diagnostics,
        &mut root_module,
    );
    closure_diagnostics.sort_by_key(|row| (row.span.line, row.span.column, row.code.clone()));
    let accepted = closure_diagnostics.is_empty();

    FullSystemV1CheckReport {
        accepted,
        module: if accepted { root_module } else { None },
        accepted_obligations: if accepted {
            vec![
                obligation(
                    "imports_resolved",
                    "all direct textual Mir imports resolved to declared source modules",
                ),
                obligation(
                    "effect_failure_rows_explicit",
                    "every declared effect in the checked module carries an explicit failure row",
                ),
                obligation(
                    "typed_scope_closed",
                    "lexical scope, function signatures, and basic contract rows typecheck in the alpha checker floor",
                ),
            ]
        } else {
            Vec::new()
        },
        residual_obligations: if accepted {
            vec![
                obligation(
                    "ambient_effect_row_containment_not_modeled",
                    "ambient effect row containment remains a residual obligation at the alpha checker floor",
                ),
                obligation(
                    "ambient_failure_row_containment_not_modeled",
                    "ambient failure row containment remains a residual obligation at the alpha checker floor",
                ),
            ]
        } else {
            Vec::new()
        },
        diagnostics: closure_diagnostics,
        final_public_api_frozen: false,
    }
}

fn collect_module_closure(
    root_path: &Path,
    current: &LoadedModule,
    loaded: &BTreeMap<PathBuf, LoadedModule>,
    visited: &mut BTreeSet<PathBuf>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
    root_module: &mut Option<TypedMirModule>,
) {
    if !visited.insert(current.path.clone()) {
        return;
    }
    for import_path in current.import_paths.values() {
        if let Some(imported) = loaded.get(import_path) {
            collect_module_closure(
                root_path,
                imported,
                loaded,
                visited,
                diagnostics,
                root_module,
            );
        }
    }
    let analysis = analyze_loaded_module(current, loaded);
    if current.path == root_path {
        *root_module = Some(analysis.module);
        diagnostics.extend(analysis.diagnostics);
    } else {
        diagnostics.extend(
            analysis
                .diagnostics
                .into_iter()
                .map(|row| qualify_imported_diagnostic(current, row)),
        );
    }
}

fn analyze_loaded_module(
    current: &LoadedModule,
    loaded: &BTreeMap<PathBuf, LoadedModule>,
) -> ModuleAnalysis {
    let accessible_modules = accessible_modules(current, loaded);
    let local_capabilities = current_capability_set(current);
    let mut diagnostics = Vec::new();

    let typed_imports = current
        .module
        .imports
        .iter()
        .map(|import| TypedMirImport {
            module_path: import.module_path.clone(),
            resolved_path: current
                .import_paths
                .get(&import.module_path)
                .map(|path| path.display().to_string())
                .unwrap_or_default(),
            span: import.span.clone(),
        })
        .collect();

    let typed_capabilities = current
        .module
        .capabilities
        .iter()
        .map(|capability| TypedCapabilityDecl {
            capability_name: capability.capability_name.clone(),
            span: capability.span.clone(),
        })
        .collect();

    let typed_records = current
        .module
        .records
        .iter()
        .map(|record| lower_record(record, &accessible_modules, &mut diagnostics))
        .collect();

    let typed_effects = current
        .module
        .effects
        .iter()
        .map(|effect| {
            lower_effect_decl(
                current,
                effect,
                &accessible_modules,
                &local_capabilities,
                &mut diagnostics,
            )
        })
        .collect();

    let function_signatures = collect_function_signatures(&accessible_modules, &mut diagnostics);
    let typed_functions = current_functions(current)
        .into_iter()
        .map(|function| {
            lower_function(
                function,
                &accessible_modules,
                &function_signatures,
                &mut diagnostics,
            )
        })
        .collect();

    let typed_transitions = current
        .module
        .transitions
        .iter()
        .map(|transition| {
            lower_transition(
                current,
                transition,
                &accessible_modules,
                &function_signatures,
                &local_capabilities,
                &mut diagnostics,
            )
        })
        .collect();

    diagnostics.sort_by_key(|row| (row.span.line, row.span.column, row.code.clone()));
    ModuleAnalysis {
        module: TypedMirModule {
            module_path: current.module.module_path.clone(),
            imports: typed_imports,
            capabilities: typed_capabilities,
            records: typed_records,
            effects: typed_effects,
            functions: typed_functions,
            transitions: typed_transitions,
            span: current.module.span.clone(),
        },
        diagnostics,
    }
}

fn load_module_graph(
    path: &Path,
    loaded: &mut BTreeMap<PathBuf, LoadedModule>,
    visiting: &mut BTreeSet<PathBuf>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> bool {
    let path = normalize_path(path);
    if loaded.contains_key(&path) {
        return true;
    }
    if !visiting.insert(path.clone()) {
        diagnostics.push(diagnostic(
            "cyclic_import",
            format!("cyclic textual Mir import detected at `{}`", path.display()),
            zero_span(),
        ));
        return false;
    }

    let report = parse_textual_mir_report_path(&path);
    let module = match report.module {
        Some(module) => module,
        None => {
            diagnostics.extend(report.diagnostics);
            visiting.remove(&path);
            return false;
        }
    };
    if !report.accepted {
        diagnostics.extend(report.diagnostics);
        visiting.remove(&path);
        return false;
    }

    let mut import_paths = BTreeMap::new();
    for import in &module.imports {
        let import_path = match resolve_textual_mir_module_reference(&path, &import.module_path) {
            TextualMirModuleResolution::Unique(import_path) => normalize_path(&import_path),
            TextualMirModuleResolution::Missing => {
                diagnostics.push(unresolved_import_diagnostic(import));
                visiting.remove(&path);
                return false;
            }
            TextualMirModuleResolution::Ambiguous(paths) => {
                diagnostics.push(ambiguous_import_diagnostic(import, &paths));
                visiting.remove(&path);
                return false;
            }
        };
        import_paths.insert(import.module_path.clone(), import_path.clone());
        if !load_module_graph(&import_path, loaded, visiting, diagnostics) {
            visiting.remove(&path);
            return false;
        }
    }

    loaded.insert(
        path.clone(),
        LoadedModule {
            path: path.clone(),
            module,
            import_paths,
        },
    );
    visiting.remove(&path);
    true
}

fn accessible_modules<'a>(
    current: &'a LoadedModule,
    loaded: &'a BTreeMap<PathBuf, LoadedModule>,
) -> Vec<&'a LoadedModule> {
    let mut modules = vec![current];
    for import_path in current.import_paths.values() {
        if let Some(module) = loaded.get(import_path) {
            modules.push(module);
        }
    }
    modules
}

fn current_functions(module: &LoadedModule) -> Vec<&AstFunction> {
    module
        .module
        .items
        .iter()
        .filter_map(|item| match item {
            AstTopLevel::Function(function) => Some(function),
            _ => None,
        })
        .collect()
}

fn current_capability_set(module: &LoadedModule) -> BTreeSet<String> {
    module
        .module
        .capabilities
        .iter()
        .map(|capability| capability.capability_name.clone())
        .collect()
}

fn collect_function_signatures(
    modules: &[&LoadedModule],
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> BTreeMap<String, Vec<FunctionSignature>> {
    let mut signatures = BTreeMap::<String, Vec<FunctionSignature>>::new();
    for module in modules {
        for function in current_functions(module) {
            let input_type = lower_type(
                &function.input_type,
                modules,
                diagnostics,
                &function.span,
                "unknown_type",
            );
            let output_type = lower_type(
                &function.output_type,
                modules,
                diagnostics,
                &function.span,
                "unknown_type",
            );
            signatures
                .entry(function.function_name.clone())
                .or_default()
                .push(FunctionSignature {
                    module_path: module.module.module_path.clone(),
                    function_name: function.function_name.clone(),
                    input_type,
                    output_type,
                });
        }
    }
    signatures
}

fn lower_record(
    record: &AstRecord,
    modules: &[&LoadedModule],
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedRecordType {
    let fields = record
        .fields
        .iter()
        .map(|field| TypedRecordField {
            field_name: field.field_name.clone(),
            field_type: lower_type(
                &field.field_type,
                modules,
                diagnostics,
                &field.span,
                "unknown_type",
            ),
            span: field.span.clone(),
        })
        .collect();
    TypedRecordType {
        record_name: record.record_name.clone(),
        fields,
        span: record.span.clone(),
    }
}

fn lower_effect_decl(
    current: &LoadedModule,
    effect: &mir_ast::textual_alpha::AstEffectDecl,
    modules: &[&LoadedModule],
    local_capabilities: &BTreeSet<String>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedEffectDecl {
    if effect.failure_row.is_empty() {
        diagnostics.push(diagnostic(
            "effect_failure_row_missing",
            format!(
                "effect `{}` must declare an explicit failure row at the alpha checker floor",
                effect.effect_name
            ),
            effect.span.clone(),
        ));
    }
    for capability in &effect.required_capabilities {
        if !local_capabilities.contains(capability) {
            diagnostics.push(diagnostic(
                "capability_not_declared",
                format!(
                    "effect `{}` requires capability `{capability}` but the capability is not declared in module `{}`",
                    effect.effect_name, current.module.module_path
                ),
                effect.span.clone(),
            ));
        }
    }

    let parameters = effect
        .parameters
        .iter()
        .map(|parameter| TypedParam {
            name: parameter.name.clone(),
            param_type: lower_type(
                &parameter.param_type,
                modules,
                diagnostics,
                &parameter.span,
                "unknown_type",
            ),
            span: parameter.span.clone(),
        })
        .collect();
    let output = effect.output.as_ref().map(|output| TypedEffectOutput {
        name: output.name.clone(),
        output_type: lower_type(
            &output.output_type,
            modules,
            diagnostics,
            &output.span,
            "unknown_type",
        ),
        span: output.span.clone(),
    });

    TypedEffectDecl {
        effect_name: effect.effect_name.clone(),
        parameters,
        required_capabilities: effect.required_capabilities.clone(),
        output,
        failure_row: effect.failure_row.clone(),
        span: effect.span.clone(),
    }
}

fn lower_function(
    function: &AstFunction,
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedFunction {
    let parameter = TypedParam {
        name: function.parameter_name.clone(),
        param_type: lower_type(
            &function.input_type,
            modules,
            diagnostics,
            &function.span,
            "unknown_type",
        ),
        span: function.span.clone(),
    };
    let output_type = lower_type(
        &function.output_type,
        modules,
        diagnostics,
        &function.span,
        "unknown_type",
    );
    let mut env = BTreeMap::from([(
        function.parameter_name.clone(),
        TypeBinding {
            ty: parameter.param_type.clone(),
            mutable: false,
        },
    )]);
    let body = lower_block(
        &function.body,
        modules,
        function_signatures,
        &mut env,
        Some(&output_type),
        None,
        diagnostics,
    );
    TypedFunction {
        function_name: function.function_name.clone(),
        parameter,
        output_type,
        body,
        span: function.span.clone(),
    }
}

fn lower_transition(
    current: &LoadedModule,
    transition: &AstTransition,
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    local_capabilities: &BTreeSet<String>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedTransition {
    for capability in &transition.required_capabilities {
        if !local_capabilities.contains(capability) {
            diagnostics.push(diagnostic(
                "capability_not_declared",
                format!(
                    "transition `{}` requires capability `{capability}` but the capability is not declared in module `{}`",
                    transition.transition_name, current.module.module_path
                ),
                transition.span.clone(),
            ));
        }
    }
    let mut env = BTreeMap::new();
    let ambient_capabilities = transition
        .required_capabilities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let body = lower_block(
        &transition.body,
        modules,
        function_signatures,
        &mut env,
        None,
        Some(&ambient_capabilities),
        diagnostics,
    );
    TypedTransition {
        transition_name: transition.transition_name.clone(),
        place_ref: transition.place_ref.clone(),
        required_capabilities: transition.required_capabilities.clone(),
        body,
        span: transition.span.clone(),
    }
}

fn lower_block(
    statements: &[AstStmt],
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    env: &mut BTreeMap<String, TypeBinding>,
    expected_return: Option<&TypedType>,
    ambient_capabilities: Option<&BTreeSet<String>>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> Vec<TypedStmt> {
    statements
        .iter()
        .map(|statement| {
            lower_statement(
                statement,
                modules,
                function_signatures,
                env,
                expected_return,
                ambient_capabilities,
                diagnostics,
            )
        })
        .collect()
}

fn lower_statement(
    statement: &AstStmt,
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    env: &mut BTreeMap<String, TypeBinding>,
    expected_return: Option<&TypedType>,
    ambient_capabilities: Option<&BTreeSet<String>>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedStmt {
    match statement {
        AstStmt::Let {
            name,
            mutable,
            ty,
            value,
            span,
        } => {
            let expected_type = lower_type(ty, modules, diagnostics, span, "unknown_type");
            let typed_value = lower_expr(value, modules, function_signatures, env, diagnostics);
            if !types_compatible(&expected_type, &typed_value.ty) {
                diagnostics.push(diagnostic(
                    "let_type_mismatch",
                    format!(
                        "let `{name}` expects `{}`, found `{}`",
                        expected_type.display_name(),
                        typed_value.ty.display_name()
                    ),
                    span.clone(),
                ));
            }
            env.insert(
                name.clone(),
                TypeBinding {
                    ty: expected_type.clone(),
                    mutable: *mutable,
                },
            );
            TypedStmt::Let {
                name: name.clone(),
                mutable: *mutable,
                ty: expected_type,
                value: typed_value,
                span: span.clone(),
            }
        }
        AstStmt::Assign { name, value, span } => {
            let typed_value = lower_expr(value, modules, function_signatures, env, diagnostics);
            match env.get(name) {
                Some(binding) => {
                    if !binding.mutable {
                        diagnostics.push(diagnostic(
                            "immutable_assignment",
                            format!("cannot assign to immutable binding `{name}`"),
                            span.clone(),
                        ));
                    }
                    if !types_compatible(&binding.ty, &typed_value.ty) {
                        diagnostics.push(diagnostic(
                            "assignment_type_mismatch",
                            format!(
                                "assignment to `{name}` expects `{}`, found `{}`",
                                binding.ty.display_name(),
                                typed_value.ty.display_name()
                            ),
                            span.clone(),
                        ));
                    }
                }
                None => diagnostics.push(diagnostic(
                    "unbound_variable",
                    format!("assignment target `{name}` is not in scope"),
                    span.clone(),
                )),
            }
            TypedStmt::Assign {
                name: name.clone(),
                value: typed_value,
                span: span.clone(),
            }
        }
        AstStmt::If {
            condition,
            then_body,
            else_body,
            span,
        } => {
            let typed_condition =
                lower_expr(condition, modules, function_signatures, env, diagnostics);
            require_bool("if condition", &typed_condition, diagnostics);
            let mut then_env = env.clone();
            let mut else_env = env.clone();
            let then_body = lower_block(
                then_body,
                modules,
                function_signatures,
                &mut then_env,
                expected_return,
                ambient_capabilities,
                diagnostics,
            );
            let else_body = lower_block(
                else_body,
                modules,
                function_signatures,
                &mut else_env,
                expected_return,
                ambient_capabilities,
                diagnostics,
            );
            TypedStmt::If {
                condition: typed_condition,
                then_body,
                else_body,
                span: span.clone(),
            }
        }
        AstStmt::While {
            condition,
            body,
            span,
        } => {
            let typed_condition =
                lower_expr(condition, modules, function_signatures, env, diagnostics);
            require_bool("while condition", &typed_condition, diagnostics);
            let mut body_env = env.clone();
            let body = lower_block(
                body,
                modules,
                function_signatures,
                &mut body_env,
                expected_return,
                ambient_capabilities,
                diagnostics,
            );
            TypedStmt::While {
                condition: typed_condition,
                body,
                span: span.clone(),
            }
        }
        AstStmt::For {
            binding,
            start,
            end,
            body,
            span,
        } => {
            let typed_start = lower_expr(start, modules, function_signatures, env, diagnostics);
            let typed_end = lower_expr(end, modules, function_signatures, env, diagnostics);
            require_exact_type("for start", &typed_start, &TypedType::Int64, diagnostics);
            require_exact_type("for end", &typed_end, &TypedType::Int64, diagnostics);
            let mut body_env = env.clone();
            body_env.insert(
                binding.clone(),
                TypeBinding {
                    ty: TypedType::Int64,
                    mutable: false,
                },
            );
            let body = lower_block(
                body,
                modules,
                function_signatures,
                &mut body_env,
                expected_return,
                ambient_capabilities,
                diagnostics,
            );
            TypedStmt::For {
                binding: binding.clone(),
                start: typed_start,
                end: typed_end,
                body,
                span: span.clone(),
            }
        }
        AstStmt::Bind {
            name,
            value,
            contract_clauses,
            span,
        } => {
            let (typed_value, binding_type) = match value {
                AstBindValue::Expr(expr) => {
                    let typed_expr =
                        lower_expr(expr, modules, function_signatures, env, diagnostics);
                    (TypedBindValue::Expr(typed_expr.clone()), typed_expr.ty)
                }
                AstBindValue::Perform(call) => {
                    let typed_call = lower_perform_call(
                        call,
                        modules,
                        function_signatures,
                        env,
                        ambient_capabilities,
                        diagnostics,
                    );
                    let binding_type = typed_call.output_type.clone().unwrap_or_else(|| {
                        diagnostics.push(diagnostic(
                            "effect_output_missing_for_bind",
                            format!(
                                "bind target `{name}` expects effect `{}` to declare an output row",
                                typed_call.effect_name
                            ),
                            span.clone(),
                        ));
                        TypedType::Error
                    });
                    (TypedBindValue::Perform(typed_call), binding_type)
                }
            };
            let typed_clauses = lower_contract_clauses(
                contract_clauses,
                modules,
                function_signatures,
                env,
                diagnostics,
            );
            env.insert(
                name.clone(),
                TypeBinding {
                    ty: binding_type.clone(),
                    mutable: false,
                },
            );
            TypedStmt::Bind {
                name: name.clone(),
                binding_type,
                value: typed_value,
                contract_clauses: typed_clauses,
                span: span.clone(),
            }
        }
        AstStmt::Perform {
            call,
            contract_clauses,
            span,
        } => {
            let typed_call = lower_perform_call(
                call,
                modules,
                function_signatures,
                env,
                ambient_capabilities,
                diagnostics,
            );
            let typed_clauses = lower_contract_clauses(
                contract_clauses,
                modules,
                function_signatures,
                env,
                diagnostics,
            );
            TypedStmt::Perform {
                call: typed_call,
                contract_clauses: typed_clauses,
                span: span.clone(),
            }
        }
        AstStmt::Return { value, span } => {
            let typed_value = lower_expr(value, modules, function_signatures, env, diagnostics);
            match expected_return {
                Some(expected_type) => {
                    if !types_compatible(expected_type, &typed_value.ty) {
                        diagnostics.push(diagnostic(
                            "return_type_mismatch",
                            format!(
                                "return expects `{}`, found `{}`",
                                expected_type.display_name(),
                                typed_value.ty.display_name()
                            ),
                            span.clone(),
                        ));
                    }
                }
                None => diagnostics.push(diagnostic(
                    "return_not_allowed_in_transition",
                    "return is not admitted in the current transition checker floor".to_string(),
                    span.clone(),
                )),
            }
            TypedStmt::Return {
                value: typed_value,
                span: span.clone(),
            }
        }
    }
}

fn lower_contract_clauses(
    clauses: &[AstContractClause],
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    env: &BTreeMap<String, TypeBinding>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> Vec<TypedContractClause> {
    clauses
        .iter()
        .map(|clause| {
            let typed_condition = lower_expr(
                &clause.condition,
                modules,
                function_signatures,
                env,
                diagnostics,
            );
            require_bool("contract clause", &typed_condition, diagnostics);
            TypedContractClause {
                kind: clause.kind,
                condition: typed_condition,
                span: clause.span.clone(),
            }
        })
        .collect()
}

fn lower_perform_call(
    call: &AstPerformCall,
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    env: &BTreeMap<String, TypeBinding>,
    ambient_capabilities: Option<&BTreeSet<String>>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedPerformCall {
    let arguments = call
        .arguments
        .iter()
        .map(|argument| lower_expr(argument, modules, function_signatures, env, diagnostics))
        .collect::<Vec<_>>();

    let mut required_capabilities = Vec::new();
    let mut output_type = None;
    let mut failure_row = Vec::new();

    match resolve_effect(modules, &call.effect_name) {
        Resolution::Missing => diagnostics.push(diagnostic(
            "effect_not_declared",
            format!(
                "effect `{}` is not declared in the current import surface",
                call.effect_name
            ),
            call.span.clone(),
        )),
        Resolution::Ambiguous(matches) => diagnostics.push(diagnostic(
            "ambiguous_effect_reference",
            format!(
                "effect reference `{}` is ambiguous across {}",
                call.effect_name,
                matches.join(", ")
            ),
            call.span.clone(),
        )),
        Resolution::Found(effect) => {
            required_capabilities = effect.value.required_capabilities.clone();
            failure_row = effect.value.failure_row.clone();
            output_type = effect.value.output.as_ref().map(|output| {
                lower_type(
                    &output.output_type,
                    modules,
                    diagnostics,
                    &output.span,
                    "unknown_type",
                )
            });
            if effect.value.parameters.len() != arguments.len() {
                diagnostics.push(diagnostic(
                    "effect_arity_mismatch",
                    format!(
                        "effect `{}` expects {} argument(s), found {}",
                        effect.value.effect_name,
                        effect.value.parameters.len(),
                        arguments.len()
                    ),
                    call.span.clone(),
                ));
            }
            for (index, (parameter, argument)) in effect
                .value
                .parameters
                .iter()
                .zip(arguments.iter())
                .enumerate()
            {
                let parameter_type = lower_type(
                    &parameter.param_type,
                    modules,
                    diagnostics,
                    &parameter.span,
                    "unknown_type",
                );
                if !types_compatible(&parameter_type, &argument.ty) {
                    diagnostics.push(diagnostic(
                        "effect_argument_type_mismatch",
                        format!(
                            "effect `{}` argument {} expects `{}`, found `{}`",
                            effect.value.effect_name,
                            index,
                            parameter_type.display_name(),
                            argument.ty.display_name()
                        ),
                        argument.span.clone(),
                    ));
                }
            }
            if let Some(ambient_capabilities) = ambient_capabilities {
                let effect_module_capabilities = current_capability_set(effect.module);
                for capability in &effect.value.required_capabilities {
                    if effect_module_capabilities.contains(capability)
                        && !ambient_capabilities.contains(capability)
                    {
                        diagnostics.push(diagnostic(
                            "capability_requirement_missing",
                            format!(
                                "perform `{}` requires capability `{capability}` in the ambient transition row",
                                effect.value.effect_name
                            ),
                            call.span.clone(),
                        ));
                    }
                }
            }
        }
    }

    TypedPerformCall {
        effect_name: call.effect_name.clone(),
        arguments,
        boundary_ref: call.boundary_ref.clone(),
        required_capabilities,
        output_type,
        failure_row,
        span: call.span.clone(),
    }
}

fn lower_expr(
    expr: &AstExpr,
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    env: &BTreeMap<String, TypeBinding>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedExpr {
    match &expr.kind {
        AstExprKind::IntLiteral(value) => TypedExpr {
            kind: TypedExprKind::IntLiteral(*value),
            ty: TypedType::Int64,
            span: expr.span.clone(),
        },
        AstExprKind::FloatLiteral(value) => TypedExpr {
            kind: TypedExprKind::FloatLiteral(value.clone()),
            ty: TypedType::Float64,
            span: expr.span.clone(),
        },
        AstExprKind::BoolLiteral(value) => TypedExpr {
            kind: TypedExprKind::BoolLiteral(*value),
            ty: TypedType::Bool,
            span: expr.span.clone(),
        },
        AstExprKind::TextLiteral(value) => TypedExpr {
            kind: TypedExprKind::TextLiteral(value.clone()),
            ty: TypedType::Text,
            span: expr.span.clone(),
        },
        AstExprKind::Variable(name) => {
            if let Some(binding) = env.get(name) {
                TypedExpr {
                    kind: TypedExprKind::Variable(name.clone()),
                    ty: binding.ty.clone(),
                    span: expr.span.clone(),
                }
            } else if name.contains('.') {
                lower_dotted_variable(name, &expr.span, modules, env, diagnostics)
            } else {
                diagnostics.push(diagnostic(
                    "unbound_variable",
                    format!("variable `{name}` is not in scope"),
                    expr.span.clone(),
                ));
                TypedExpr {
                    kind: TypedExprKind::Variable(name.clone()),
                    ty: TypedType::Error,
                    span: expr.span.clone(),
                }
            }
        }
        AstExprKind::ArrayLiteral(elements) => {
            let typed_elements = elements
                .iter()
                .map(|element| lower_expr(element, modules, function_signatures, env, diagnostics))
                .collect::<Vec<_>>();
            let element_type = typed_elements
                .first()
                .map(|element| element.ty.clone())
                .unwrap_or_else(|| {
                    diagnostics.push(diagnostic(
                        "empty_array_literal_needs_context",
                        "empty array literal requires an explicit contextual element type"
                            .to_string(),
                        expr.span.clone(),
                    ));
                    TypedType::Error
                });
            for element in typed_elements.iter().skip(1) {
                if !types_compatible(&element_type, &element.ty) {
                    diagnostics.push(diagnostic(
                        "array_element_type_mismatch",
                        format!(
                            "array literal expects homogeneous elements, found `{}` and `{}`",
                            element_type.display_name(),
                            element.ty.display_name()
                        ),
                        element.span.clone(),
                    ));
                }
            }
            TypedExpr {
                kind: TypedExprKind::ArrayLiteral(typed_elements),
                ty: TypedType::FixedArray {
                    element: Box::new(element_type),
                    length: elements.len(),
                },
                span: expr.span.clone(),
            }
        }
        AstExprKind::RecordConstruct {
            record_name,
            fields,
        } => {
            let typed_fields = fields
                .iter()
                .map(|field| {
                    lower_record_construct_field(
                        field,
                        modules,
                        function_signatures,
                        env,
                        diagnostics,
                    )
                })
                .collect::<Vec<_>>();
            let ty = match resolve_record(modules, record_name) {
                Resolution::Missing => {
                    diagnostics.push(diagnostic(
                        "record_not_declared",
                        format!(
                            "record `{record_name}` is not declared in the current import surface"
                        ),
                        expr.span.clone(),
                    ));
                    TypedType::Error
                }
                Resolution::Ambiguous(matches) => {
                    diagnostics.push(diagnostic(
                        "ambiguous_record_reference",
                        format!(
                            "record reference `{record_name}` is ambiguous across {}",
                            matches.join(", ")
                        ),
                        expr.span.clone(),
                    ));
                    TypedType::Error
                }
                Resolution::Found(record) => {
                    let expected_fields = record
                        .value
                        .fields
                        .iter()
                        .map(|field| field.field_name.clone())
                        .collect::<BTreeSet<_>>();
                    let actual_fields = fields
                        .iter()
                        .map(|field| field.field_name.clone())
                        .collect::<BTreeSet<_>>();
                    if expected_fields != actual_fields {
                        diagnostics.push(diagnostic(
                            "record_field_set_mismatch",
                            format!(
                                "record `{record_name}` expects fields [{}], found [{}]",
                                join_set(&expected_fields),
                                join_set(&actual_fields)
                            ),
                            expr.span.clone(),
                        ));
                    }
                    for declared_field in &record.value.fields {
                        if let Some(actual_field) = typed_fields
                            .iter()
                            .find(|field| field.field_name == declared_field.field_name)
                        {
                            let expected_type = lower_type(
                                &declared_field.field_type,
                                modules,
                                diagnostics,
                                &declared_field.span,
                                "unknown_type",
                            );
                            if !types_compatible(&expected_type, &actual_field.value.ty) {
                                diagnostics.push(diagnostic(
                                    "record_field_type_mismatch",
                                    format!(
                                        "record `{record_name}` field `{}` expects `{}`, found `{}`",
                                        declared_field.field_name,
                                        expected_type.display_name(),
                                        actual_field.value.ty.display_name()
                                    ),
                                    actual_field.span.clone(),
                                ));
                            }
                        }
                    }
                    TypedType::Named(canonical_record_name(record.module, record.value))
                }
            };
            TypedExpr {
                kind: TypedExprKind::RecordConstruct {
                    record_name: record_name.clone(),
                    fields: typed_fields,
                },
                ty,
                span: expr.span.clone(),
            }
        }
        AstExprKind::Call { callee, arguments } => {
            let typed_arguments = arguments
                .iter()
                .map(|argument| {
                    lower_expr(argument, modules, function_signatures, env, diagnostics)
                })
                .collect::<Vec<_>>();
            let (function_name, module_path, ty) = match &callee.kind {
                AstExprKind::Variable(name) => {
                    match resolve_function(function_signatures, name) {
                        Resolution::Missing => {
                            diagnostics.push(diagnostic(
                            "function_not_declared",
                            format!("function `{name}` is not declared in the current import surface"),
                            callee.span.clone(),
                        ));
                            (name.clone(), String::new(), TypedType::Error)
                        }
                        Resolution::Ambiguous(matches) => {
                            diagnostics.push(diagnostic(
                                "ambiguous_function_reference",
                                format!(
                                    "function reference `{name}` is ambiguous across {}",
                                    matches.join(", ")
                                ),
                                callee.span.clone(),
                            ));
                            (name.clone(), String::new(), TypedType::Error)
                        }
                        Resolution::Found(signature) => {
                            if typed_arguments.len() != 1 {
                                diagnostics.push(diagnostic(
                                    "function_arity_mismatch",
                                    format!(
                                        "function `{}` expects 1 argument, found {}",
                                        signature.function_name,
                                        typed_arguments.len()
                                    ),
                                    expr.span.clone(),
                                ));
                            }
                            if let Some(argument) = typed_arguments.first() {
                                if !types_compatible(&signature.input_type, &argument.ty) {
                                    diagnostics.push(diagnostic(
                                        "function_argument_type_mismatch",
                                        format!(
                                            "function `{}` expects `{}`, found `{}`",
                                            signature.function_name,
                                            signature.input_type.display_name(),
                                            argument.ty.display_name()
                                        ),
                                        argument.span.clone(),
                                    ));
                                }
                            }
                            (
                                signature.function_name.clone(),
                                signature.module_path.clone(),
                                signature.output_type.clone(),
                            )
                        }
                    }
                }
                _ => {
                    diagnostics.push(diagnostic(
                        "unsupported_callee_shape",
                        "only direct function identifiers are admitted in the alpha checker floor"
                            .to_string(),
                        callee.span.clone(),
                    ));
                    (String::new(), String::new(), TypedType::Error)
                }
            };
            TypedExpr {
                kind: TypedExprKind::Call {
                    function_name,
                    module_path,
                    arguments: typed_arguments,
                },
                ty,
                span: expr.span.clone(),
            }
        }
        AstExprKind::Index { base, index } => {
            let typed_base = lower_expr(base, modules, function_signatures, env, diagnostics);
            let typed_index = lower_expr(index, modules, function_signatures, env, diagnostics);
            require_exact_type("array index", &typed_index, &TypedType::Int64, diagnostics);
            let ty = match &typed_base.ty {
                TypedType::FixedArray { element, length } => {
                    if let TypedExprKind::IntLiteral(value) = typed_index.kind {
                        if value < 0 || value as usize >= *length {
                            diagnostics.push(diagnostic(
                                "static_index_out_of_bounds",
                                format!(
                                    "array index {} is out of bounds for static length {}",
                                    value, length
                                ),
                                index.span.clone(),
                            ));
                        }
                    }
                    (**element).clone()
                }
                other => {
                    diagnostics.push(diagnostic(
                        "index_base_not_array",
                        format!(
                            "index base must be a fixed array, found `{}`",
                            other.display_name()
                        ),
                        base.span.clone(),
                    ));
                    TypedType::Error
                }
            };
            TypedExpr {
                kind: TypedExprKind::Index {
                    base: Box::new(typed_base),
                    index: Box::new(typed_index),
                },
                ty,
                span: expr.span.clone(),
            }
        }
        AstExprKind::FieldAccess { base, field_name } => {
            let typed_base = lower_expr(base, modules, function_signatures, env, diagnostics);
            let ty = match &typed_base.ty {
                TypedType::Named(record_name) => match resolve_record(modules, record_name) {
                    Resolution::Missing => {
                        diagnostics.push(diagnostic(
                            "record_not_declared",
                            format!("record `{record_name}` is not declared in the current import surface"),
                            base.span.clone(),
                        ));
                        TypedType::Error
                    }
                    Resolution::Ambiguous(matches) => {
                        diagnostics.push(diagnostic(
                            "ambiguous_record_reference",
                            format!(
                                "record reference `{record_name}` is ambiguous across {}",
                                matches.join(", ")
                            ),
                            base.span.clone(),
                        ));
                        TypedType::Error
                    }
                    Resolution::Found(record) => record
                        .value
                        .fields
                        .iter()
                        .find(|field| field.field_name == *field_name)
                        .map(|field| {
                            lower_type(
                                &field.field_type,
                                modules,
                                diagnostics,
                                &field.span,
                                "unknown_type",
                            )
                        })
                        .unwrap_or_else(|| {
                            diagnostics.push(diagnostic(
                                "record_field_not_declared",
                                format!(
                                    "record `{record_name}` does not declare field `{field_name}`"
                                ),
                                expr.span.clone(),
                            ));
                            TypedType::Error
                        }),
                },
                other => {
                    diagnostics.push(diagnostic(
                        "field_access_base_not_record",
                        format!(
                            "field access base must be a record value, found `{}`",
                            other.display_name()
                        ),
                        base.span.clone(),
                    ));
                    TypedType::Error
                }
            };
            TypedExpr {
                kind: TypedExprKind::FieldAccess {
                    base: Box::new(typed_base),
                    field_name: field_name.clone(),
                },
                ty,
                span: expr.span.clone(),
            }
        }
        AstExprKind::Unary { op, expr: inner } => {
            let typed_inner = lower_expr(inner, modules, function_signatures, env, diagnostics);
            let ty = match op {
                AstUnaryOp::Negate => {
                    if matches!(
                        typed_inner.ty,
                        TypedType::Int64 | TypedType::Float64 | TypedType::Error
                    ) {
                        typed_inner.ty.clone()
                    } else {
                        diagnostics.push(diagnostic(
                            "unary_type_mismatch",
                            format!(
                                "numeric negation expects Int64 or Float64, found `{}`",
                                typed_inner.ty.display_name()
                            ),
                            expr.span.clone(),
                        ));
                        TypedType::Error
                    }
                }
                AstUnaryOp::Not => {
                    require_exact_type("logical not", &typed_inner, &TypedType::Bool, diagnostics);
                    if typed_inner.ty.is_error() {
                        TypedType::Error
                    } else {
                        TypedType::Bool
                    }
                }
            };
            TypedExpr {
                kind: TypedExprKind::Unary {
                    op: lower_unary_op(*op),
                    expr: Box::new(typed_inner),
                },
                ty,
                span: expr.span.clone(),
            }
        }
        AstExprKind::Binary { op, left, right } => {
            let typed_left = lower_expr(left, modules, function_signatures, env, diagnostics);
            let typed_right = lower_expr(right, modules, function_signatures, env, diagnostics);
            let ty = lower_binary_result_type(op, &typed_left, &typed_right, diagnostics);
            TypedExpr {
                kind: TypedExprKind::Binary {
                    op: lower_binary_op(*op),
                    left: Box::new(typed_left),
                    right: Box::new(typed_right),
                },
                ty,
                span: expr.span.clone(),
            }
        }
    }
}

fn lower_record_construct_field(
    field: &AstRecordConstructField,
    modules: &[&LoadedModule],
    function_signatures: &BTreeMap<String, Vec<FunctionSignature>>,
    env: &BTreeMap<String, TypeBinding>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedRecordConstructField {
    TypedRecordConstructField {
        field_name: field.field_name.clone(),
        value: lower_expr(&field.value, modules, function_signatures, env, diagnostics),
        span: field.span.clone(),
    }
}

fn lower_binary_result_type(
    op: &AstBinaryOp,
    left: &TypedExpr,
    right: &TypedExpr,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedType {
    use AstBinaryOp as Op;
    match op {
        Op::Add | Op::Sub | Op::Mul | Op::Div => {
            if types_compatible(&left.ty, &right.ty)
                && matches!(
                    left.ty,
                    TypedType::Int64 | TypedType::Float64 | TypedType::Error
                )
            {
                left.ty.clone()
            } else {
                diagnostics.push(diagnostic(
                    "binary_type_mismatch",
                    format!(
                        "arithmetic operator expects matching Int64 or Float64 operands, found `{}` and `{}`",
                        left.ty.display_name(),
                        right.ty.display_name()
                    ),
                    left.span.clone(),
                ));
                TypedType::Error
            }
        }
        Op::Equal | Op::NotEqual => {
            if types_compatible(&left.ty, &right.ty) {
                TypedType::Bool
            } else {
                diagnostics.push(diagnostic(
                    "binary_type_mismatch",
                    format!(
                        "equality expects matching operand types, found `{}` and `{}`",
                        left.ty.display_name(),
                        right.ty.display_name()
                    ),
                    left.span.clone(),
                ));
                TypedType::Error
            }
        }
        Op::LessThan | Op::LessEqual | Op::GreaterThan | Op::GreaterEqual => {
            if types_compatible(&left.ty, &right.ty)
                && matches!(
                    left.ty,
                    TypedType::Int64 | TypedType::Float64 | TypedType::Error
                )
            {
                TypedType::Bool
            } else {
                diagnostics.push(diagnostic(
                    "binary_type_mismatch",
                    format!(
                        "comparison expects matching Int64 or Float64 operands, found `{}` and `{}`",
                        left.ty.display_name(),
                        right.ty.display_name()
                    ),
                    left.span.clone(),
                ));
                TypedType::Error
            }
        }
        Op::And | Op::Or => {
            if types_compatible(&left.ty, &TypedType::Bool)
                && types_compatible(&right.ty, &TypedType::Bool)
            {
                TypedType::Bool
            } else {
                diagnostics.push(diagnostic(
                    "binary_type_mismatch",
                    format!(
                        "logical operator expects Bool operands, found `{}` and `{}`",
                        left.ty.display_name(),
                        right.ty.display_name()
                    ),
                    left.span.clone(),
                ));
                TypedType::Error
            }
        }
    }
}

fn lower_dotted_variable(
    name: &str,
    span: &SourceSpan,
    modules: &[&LoadedModule],
    env: &BTreeMap<String, TypeBinding>,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) -> TypedExpr {
    let mut parts = name.split('.');
    let Some(base_name) = parts.next() else {
        diagnostics.push(diagnostic(
            "unbound_variable",
            format!("variable `{name}` is not in scope"),
            span.clone(),
        ));
        return TypedExpr {
            kind: TypedExprKind::Variable(name.to_string()),
            ty: TypedType::Error,
            span: span.clone(),
        };
    };
    let Some(binding) = env.get(base_name) else {
        diagnostics.push(diagnostic(
            "unbound_variable",
            format!("variable `{name}` is not in scope"),
            span.clone(),
        ));
        return TypedExpr {
            kind: TypedExprKind::Variable(name.to_string()),
            ty: TypedType::Error,
            span: span.clone(),
        };
    };

    let mut current = TypedExpr {
        kind: TypedExprKind::Variable(base_name.to_string()),
        ty: binding.ty.clone(),
        span: span.clone(),
    };
    for field_name in parts {
        let ty = match &current.ty {
            TypedType::Named(record_name) => match resolve_record(modules, record_name) {
                Resolution::Missing => {
                    diagnostics.push(diagnostic(
                        "record_not_declared",
                        format!(
                            "record `{record_name}` is not declared in the current import surface"
                        ),
                        span.clone(),
                    ));
                    TypedType::Error
                }
                Resolution::Ambiguous(matches) => {
                    diagnostics.push(diagnostic(
                        "ambiguous_record_reference",
                        format!(
                            "record reference `{record_name}` is ambiguous across {}",
                            matches.join(", ")
                        ),
                        span.clone(),
                    ));
                    TypedType::Error
                }
                Resolution::Found(record) => record
                    .value
                    .fields
                    .iter()
                    .find(|field| field.field_name == field_name)
                    .map(|field| {
                        lower_type(
                            &field.field_type,
                            modules,
                            diagnostics,
                            &field.span,
                            "unknown_type",
                        )
                    })
                    .unwrap_or_else(|| {
                        diagnostics.push(diagnostic(
                            "record_field_not_declared",
                            format!("record `{record_name}` does not declare field `{field_name}`"),
                            span.clone(),
                        ));
                        TypedType::Error
                    }),
            },
            other => {
                diagnostics.push(diagnostic(
                    "field_access_base_not_record",
                    format!(
                        "field access base must be a record value, found `{}`",
                        other.display_name()
                    ),
                    span.clone(),
                ));
                TypedType::Error
            }
        };
        current = TypedExpr {
            kind: TypedExprKind::FieldAccess {
                base: Box::new(current),
                field_name: field_name.to_string(),
            },
            ty,
            span: span.clone(),
        };
    }
    current
}

fn lower_type(
    ty: &AstType,
    modules: &[&LoadedModule],
    diagnostics: &mut Vec<TextualMirDiagnostic>,
    span: &SourceSpan,
    code: &str,
) -> TypedType {
    match ty {
        AstType::Bool => TypedType::Bool,
        AstType::Int64 => TypedType::Int64,
        AstType::UInt64 => TypedType::UInt64,
        AstType::Float64 => TypedType::Float64,
        AstType::Text => TypedType::Text,
        AstType::Unit => TypedType::Unit,
        AstType::Named(name) => match resolve_record(modules, name) {
            Resolution::Missing => {
                diagnostics.push(diagnostic(
                    code,
                    format!("named type `{name}` is not declared in the current import surface"),
                    span.clone(),
                ));
                TypedType::Error
            }
            Resolution::Ambiguous(matches) => {
                diagnostics.push(diagnostic(
                    "ambiguous_record_reference",
                    format!(
                        "named type `{name}` is ambiguous across {}",
                        matches.join(", ")
                    ),
                    span.clone(),
                ));
                TypedType::Error
            }
            Resolution::Found(record) => {
                TypedType::Named(canonical_record_name(record.module, record.value))
            }
        },
        AstType::FixedArray { element, length } => TypedType::FixedArray {
            element: Box::new(lower_type(element, modules, diagnostics, span, code)),
            length: *length,
        },
    }
}

fn lower_unary_op(op: AstUnaryOp) -> TypedUnaryOp {
    match op {
        AstUnaryOp::Negate => TypedUnaryOp::Negate,
        AstUnaryOp::Not => TypedUnaryOp::Not,
    }
}

fn lower_binary_op(op: AstBinaryOp) -> TypedBinaryOp {
    match op {
        AstBinaryOp::Add => TypedBinaryOp::Add,
        AstBinaryOp::Sub => TypedBinaryOp::Sub,
        AstBinaryOp::Mul => TypedBinaryOp::Mul,
        AstBinaryOp::Div => TypedBinaryOp::Div,
        AstBinaryOp::Equal => TypedBinaryOp::Equal,
        AstBinaryOp::NotEqual => TypedBinaryOp::NotEqual,
        AstBinaryOp::LessThan => TypedBinaryOp::LessThan,
        AstBinaryOp::LessEqual => TypedBinaryOp::LessEqual,
        AstBinaryOp::GreaterThan => TypedBinaryOp::GreaterThan,
        AstBinaryOp::GreaterEqual => TypedBinaryOp::GreaterEqual,
        AstBinaryOp::And => TypedBinaryOp::And,
        AstBinaryOp::Or => TypedBinaryOp::Or,
    }
}

fn resolve_function<'a>(
    signatures: &'a BTreeMap<String, Vec<FunctionSignature>>,
    query: &str,
) -> Resolution<&'a FunctionSignature> {
    let matches = signatures
        .values()
        .flat_map(|rows| rows.iter())
        .filter(|signature| matches_symbol(query, &signature.module_path, &signature.function_name))
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [] => Resolution::Missing,
        [signature] => Resolution::Found(*signature),
        _ => Resolution::Ambiguous(
            matches
                .iter()
                .map(|signature| format!("{}.{}", signature.module_path, signature.function_name))
                .collect(),
        ),
    }
}

fn resolve_effect<'a>(
    modules: &[&'a LoadedModule],
    query: &str,
) -> Resolution<DeclRef<'a, mir_ast::textual_alpha::AstEffectDecl>> {
    resolve_declaration(modules, query, |module| module.module.effects.iter())
}

fn resolve_record<'a>(
    modules: &[&'a LoadedModule],
    query: &str,
) -> Resolution<DeclRef<'a, AstRecord>> {
    resolve_declaration(modules, query, |module| module.module.records.iter())
}

fn resolve_declaration<'a, T, I, F>(
    modules: &[&'a LoadedModule],
    query: &str,
    mut iter_fn: F,
) -> Resolution<DeclRef<'a, T>>
where
    F: FnMut(&'a LoadedModule) -> I,
    I: Iterator<Item = &'a T>,
    T: NamedDeclaration,
{
    let mut matches = Vec::new();
    for module in modules {
        for value in iter_fn(module) {
            if matches_symbol(query, &module.module.module_path, value.declaration_name()) {
                matches.push(DeclRef { module, value });
            }
        }
    }
    match matches.as_slice() {
        [] => Resolution::Missing,
        [value] => Resolution::Found((*value).clone()),
        _ => Resolution::Ambiguous(
            matches
                .iter()
                .map(|row| {
                    format!(
                        "{}.{}",
                        row.module.module.module_path,
                        row.value.declaration_name()
                    )
                })
                .collect(),
        ),
    }
}

trait NamedDeclaration {
    fn declaration_name(&self) -> &str;
}

impl NamedDeclaration for AstRecord {
    fn declaration_name(&self) -> &str {
        &self.record_name
    }
}

impl NamedDeclaration for mir_ast::textual_alpha::AstEffectDecl {
    fn declaration_name(&self) -> &str {
        &self.effect_name
    }
}

enum Resolution<T> {
    Missing,
    Ambiguous(Vec<String>),
    Found(T),
}

fn require_bool(context: &str, expr: &TypedExpr, diagnostics: &mut Vec<TextualMirDiagnostic>) {
    require_exact_type(context, expr, &TypedType::Bool, diagnostics);
}

fn require_exact_type(
    context: &str,
    expr: &TypedExpr,
    expected: &TypedType,
    diagnostics: &mut Vec<TextualMirDiagnostic>,
) {
    if !types_compatible(expected, &expr.ty) {
        diagnostics.push(diagnostic(
            "type_mismatch",
            format!(
                "{context} expects `{}`, found `{}`",
                expected.display_name(),
                expr.ty.display_name()
            ),
            expr.span.clone(),
        ));
    }
}

fn types_compatible(expected: &TypedType, actual: &TypedType) -> bool {
    expected.is_error() || actual.is_error() || expected == actual
}

fn matches_symbol(query: &str, module_path: &str, declaration_name: &str) -> bool {
    if query == declaration_name {
        return true;
    }
    query == format!("{module_path}.{declaration_name}")
}

fn canonical_record_name(module: &LoadedModule, record: &AstRecord) -> String {
    format!("{}.{}", module.module.module_path, record.record_name)
}

fn obligation(code: &str, message: &str) -> FullSystemV1Obligation {
    FullSystemV1Obligation {
        code: code.to_string(),
        message: message.to_string(),
    }
}

fn diagnostic(code: &str, message: String, span: SourceSpan) -> TextualMirDiagnostic {
    TextualMirDiagnostic {
        code: code.to_string(),
        message,
        span,
    }
}

fn unresolved_import_diagnostic(import: &AstImport) -> TextualMirDiagnostic {
    diagnostic(
        "unresolved_import",
        format!(
            "import `{}` does not resolve to a declared textual Mir module",
            import.module_path
        ),
        import.span.clone(),
    )
}

fn ambiguous_import_diagnostic(import: &AstImport, paths: &[PathBuf]) -> TextualMirDiagnostic {
    diagnostic(
        "ambiguous_import_resolution",
        format!(
            "import `{}` resolves to multiple declared textual Mir modules: {}",
            import.module_path,
            paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        import.span.clone(),
    )
}

fn qualify_imported_diagnostic(
    module: &LoadedModule,
    mut diagnostic_row: TextualMirDiagnostic,
) -> TextualMirDiagnostic {
    diagnostic_row.message = format!(
        "in imported module `{}` ({}): {}",
        module.module.module_path,
        module.path.display(),
        diagnostic_row.message
    );
    diagnostic_row
}

fn normalize_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn zero_span() -> SourceSpan {
    SourceSpan {
        start: 0,
        end: 0,
        line: 0,
        column: 0,
    }
}

fn join_set(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(", ")
}
