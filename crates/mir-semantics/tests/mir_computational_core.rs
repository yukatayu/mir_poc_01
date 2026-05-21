use mir_semantics::computational_core::{Value, add_one_module, eval_function, typecheck_module};

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
