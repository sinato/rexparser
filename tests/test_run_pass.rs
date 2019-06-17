use std::process::Command;

fn run(filepath: &str, stdout: &str, status: &str) {
    // compile
    Command::new("sh")
        .arg("-c")
        .arg(format!("./target/debug/rexparser \"{}\"", filepath))
        .status()
        .expect("process failed to execute");

    // run generated IR and get returned status code
    let output = Command::new("sh")
        .arg("-c")
        .arg("llvm-link -S -o runnable.ll compiled.ll builtin.ll; llvm-as runnable.ll; lli runnable.bc")
        .output()
        .expect("failed to execute process");

    // assert status
    println!("{:?} => {:?}", output.status.to_string(), status);
    assert!(output.status.to_string() == String::from(format!("exit code: {}", status)));

    // assert stdout
    println!(
        "{:?} => {:?}",
        std::str::from_utf8(&output.stdout).unwrap(),
        stdout
    );
    assert!(std::str::from_utf8(&output.stdout).unwrap() == stdout);
}

#[test]
fn test_single_num() {
    let filepath = "./tests/resources/test_single_num.c";
    run(filepath, "", "1");
}

#[test]
fn test_add() {
    let filepath = "./tests/resources/test_add.c";
    run(filepath, "", "3");
}

#[test]
fn test_mul() {
    let filepath = "./tests/resources/test_mul.c";
    run(filepath, "", "6");
}

#[test]
fn test_multi_exp() {
    let filepath = "./tests/resources/test_multi_exp.c";
    run(filepath, "", "15");
}

#[test]
fn test_single_float() {
    let filepath = "./tests/resources/test_single_float.c";
    run(filepath, "", "7");
}

#[test]
fn test_float_exp() {
    let filepath = "./tests/resources/test_float_exp.c";
    run(filepath, "", "6");
}

#[test]
fn test_int_declare() {
    let filepath = "./tests/resources/test_int_declare.c";
    run(filepath, "", "42");
}

#[test]
fn test_variable() {
    let filepath = "./tests/resources/test_variable.c";
    run(filepath, "", "10");
}

#[test]
fn test_variable2() {
    let filepath = "./tests/resources/test_variable2.c";
    run(filepath, "", "16");
}

#[test]
fn test_float_variable() {
    let filepath = "./tests/resources/test_float_variable.c";
    run(filepath, "", "2");
}

#[test]
fn test_pointer() {
    let filepath = "./tests/resources/test_pointer.c";
    run(filepath, "", "24");
}

#[test]
fn test_array() {
    let filepath = "./tests/resources/test_array.c";
    run(filepath, "", "10");
}

#[test]
fn test_array2() {
    let filepath = "./tests/resources/test_array2.c";
    run(filepath, "", "22");
}

#[test]
fn test_multi_dim_array() {
    let filepath = "./tests/resources/test_multi_dim_array.c";
    run(filepath, "", "66");
}

#[test]
fn test_function() {
    let filepath = "./tests/resources/test_function.c";
    run(filepath, "", "55");
}

#[test]
fn test_function2() {
    let filepath = "./tests/resources/test_function2.c";
    run(filepath, "", "3");
}

#[test]
fn test_compound_statement() {
    let filepath = "./tests/resources/test_compound_statement.c";
    run(filepath, "", "17");
}

#[test]
fn test_compound_statement2() {
    let filepath = "./tests/resources/test_compound_statement2.c";
    run(filepath, "", "18");
}

#[test]
fn test_function_arr() {
    let filepath = "./tests/resources/test_function_arr.c";
    run(filepath, "", "35");
}

#[test]
fn test_function_arr2() {
    let filepath = "./tests/resources/test_function_arr2.c";
    run(filepath, "", "32");
}

#[test]
fn test_paren() {
    let filepath = "./tests/resources/test_paren.c";
    run(filepath, "", "20");
}

#[test]
fn test_compare() {
    let filepath = "./tests/resources/test_compare.c";
    run(filepath, "", "100");
}

#[test]
fn test_if() {
    let filepath = "./tests/resources/test_if.c";
    run(filepath, "", "103");
}

#[test]
fn test_if2() {
    let filepath = "./tests/resources/test_if2.c";
    run(filepath, "", "54");
}

#[test]
fn test_if3() {
    let filepath = "./tests/resources/test_if3.c";
    run(filepath, "", "103");
}

#[test]
fn test_while() {
    let filepath = "./tests/resources/test_while.c";
    run(filepath, "", "10");
}

#[test]
fn test_increment() {
    let filepath = "./tests/resources/test_increment.c";
    run(filepath, "", "34");
}

#[test]
fn test_increment_pre() {
    let filepath = "./tests/resources/test_increment_pre.c";
    run(filepath, "", "44");
}

#[test]
fn test_break() {
    let filepath = "./tests/resources/test_break.c";
    run(filepath, "", "100");
}

#[test]
fn test_assign_add() {
    let filepath = "./tests/resources/test_assign_add.c";
    run(filepath, "", "15");
}

#[test]
fn test_for() {
    let filepath = "./tests/resources/test_for.c";
    run(filepath, "", "45");
}

#[test]
fn test_for2() {
    let filepath = "./tests/resources/test_for2.c";
    run(filepath, "", "24");
}

#[test]
fn test_for3() {
    let filepath = "./tests/resources/test_for3.c";
    run(filepath, "", "60");
}

#[test]
fn test_for_break() {
    let filepath = "./tests/resources/test_for_break.c";
    run(filepath, "", "15");
}

#[test]
fn test_continue() {
    let filepath = "./tests/resources/test_continue.c";
    run(filepath, "", "108");
}

#[test]
fn test_continue2() {
    let filepath = "./tests/resources/test_continue2.c";
    run(filepath, "", "55");
}

#[test]
fn test_function_return() {
    let filepath = "./tests/resources/test_function_return.c";
    run(filepath, "", "99");
}

#[test]
fn test_putchar() {
    let filepath = "./tests/resources/test_putchar.c";
    run(filepath, "s", "33");
}

#[test]
fn test_without_curly() {
    let filepath = "./tests/resources/test_without_curly.c";
    run(filepath, "", "71");
}

#[test]
fn test_if_else() {
    let filepath = "./tests/resources/test_if_else.c";
    run(filepath, "", "91");
}

#[test]
fn test_if_else2() {
    let filepath = "./tests/resources/test_if_else2.c";
    run(filepath, "", "57");
}

#[test]
fn test_if_else3() {
    let filepath = "./tests/resources/test_if_else3.c";
    run(filepath, "", "100");
}

#[test]
fn test_and() {
    let filepath = "./tests/resources/test_and.c";
    run(filepath, "", "10");
}

#[test]
fn test_comp_int() {
    let filepath = "./tests/resources/builtin/test_comp_int.c";
    run(filepath, "", "101");
}

#[test]
fn test_sgt_int() {
    let filepath = "./tests/resources/builtin/test_sgt_int.c";
    run(filepath, "", "8");
}

#[test]
fn test_and_int() {
    let filepath = "./tests/resources/builtin/test_and_int.c";
    run(filepath, "", "39");
}
