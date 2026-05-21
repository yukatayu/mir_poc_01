use mir_semantics::computational_core::{
    Value, add_one_module, declared_module, eval_function, typecheck_module,
};

#[test]
fn pure_add_one_module_typechecks_and_evaluates() {
    let module = add_one_module();

    typecheck_module(&module).expect("add_one module should typecheck");
    let result =
        eval_function(&module, "add_one", vec![Value::Int64(41)]).expect("add_one should evaluate");

    assert_eq!(result, Value::Int64(42));
}

#[test]
fn pure_add_one_rejects_non_int_argument() {
    let module = add_one_module();

    let error = eval_function(
        &module,
        "add_one",
        vec![Value::Text("not-an-int".to_string())],
    )
    .expect_err("non-int argument should reject");

    assert!(error.to_string().contains("Int64"));
}

#[test]
fn declared_comp03_positive_modules_typecheck_and_evaluate() {
    let cases = [
        ("Computational.Scope.Positive", "clamp_zero", -5, 0),
        ("Computational.Arrays.Positive", "second", 5, 5),
        ("Computational.Vec3.Positive", "length_squared", 5, 110),
        ("Computational.ControlFlow.Positive", "sum_to", 5, 15),
        ("Computational.Compose.Positive", "add_two", 40, 42),
    ];

    for (module_id, function_id, input, expected) in cases {
        let module = declared_module(module_id).expect("module should exist");
        typecheck_module(&module).expect("positive module should typecheck");
        let output = eval_function(&module, function_id, vec![Value::Int64(input)])
            .expect("positive module should evaluate");
        assert_eq!(output, Value::Int64(expected), "module {module_id}");
    }
}

#[test]
fn declared_comp03_negative_modules_reject_with_stable_reason() {
    let cases = [
        (
            "Computational.Scope.NegativeUseBeforeDeclare",
            "clamp_zero",
            3,
            "unbound variable",
        ),
        (
            "Computational.Arrays.NegativeOutOfBounds",
            "second",
            5,
            "out of bounds",
        ),
        (
            "Computational.Vec3.NegativeField",
            "length_squared",
            5,
            "unknown field",
        ),
        (
            "Computational.ControlFlow.NegativeCondition",
            "sum_to",
            5,
            "condition must be Bool",
        ),
        (
            "Computational.Compose.NegativeMissingImport",
            "add_two",
            40,
            "add_one",
        ),
    ];

    for (module_id, function_id, input, expected_detail) in cases {
        let module = declared_module(module_id).expect("module should exist");
        let error = eval_function(&module, function_id, vec![Value::Int64(input)])
            .expect_err("negative module should reject");
        assert!(
            error.to_string().contains(expected_detail),
            "module {module_id} detail was {}",
            error
        );
    }
}
